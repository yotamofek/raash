use std::{
    cell::Cell,
    iter::{successors, zip},
    ops::BitOrAssign,
};

use array_util::{WindowedArray, W, W2};
use encoder::CodecContext;
use ffmpeg_src_macro::ffmpeg_src;
use izip::izip;
use libc::{c_char, c_double, c_float, c_int, c_long, c_uchar, c_ushort};

use crate::{
    aac::{
        coder::{
            find_form_factor, find_min_book, math::Float as _, quantization::QuantizationCost,
            sfdelta_encoding_range,
        },
        encoder::{
            ctx::{AACEncContext, QuantizeBandCostCache},
            pow::Pow34,
        },
        psy_model::cutoff_from_bitrate,
        tables::SCALEFACTOR_BITS,
        IndividualChannelStream, SyntaxElementType, WindowedIteration, SCALE_DIFF_ZERO,
        SCALE_DIV_512, SCALE_MAX_DIFF, SCALE_MAX_POS, SCALE_ONE_POS,
    },
    types::*,
};

impl WindowedIteration {
    fn calc_quantization_cost_sum(
        &self,
        quantize_band_cost_cache: &mut QuantizeBandCostCache,
        g: c_int,
        coeffs: &WindowedArray<[c_float], 128>,
        scaled_coeffs: &WindowedArray<[c_float], 128>,
        swb_size: c_uchar,
        sf_idx: c_int,
        cb: c_uchar,
    ) -> QuantizationCost {
        izip!(self.w.., coeffs, scaled_coeffs)
            .take(self.group_len.into())
            .map(|(w, coeffs, scaled)| {
                quantize_band_cost_cache.quantize_band_cost_cached(
                    w,
                    g as c_int,
                    &coeffs[..swb_size.into()],
                    &scaled[..swb_size.into()],
                    sf_idx,
                    cb.into(),
                    1.,
                    f32::INFINITY,
                    0,
                )
            })
            .sum()
    }
}

#[ffmpeg_src(file = "libavcodec/aaccoder_twoloop.h", lines = 67..=761, name = "search_for_quantizers_twoloop")]
pub(crate) fn search(
    avctx: &CodecContext,
    s: &mut AACEncContext,
    sce: &mut SingleChannelElement,
    lambda: c_float,
) {
    let mut dest_bits: c_int = (avctx.bit_rate().get() as c_double * 1024.
        / avctx.sample_rate().get() as c_double
        / (if avctx.flags().get().qscale() {
            2.
        } else {
            avctx.ch_layout().get().nb_channels as c_float
        }) as c_double
        * (lambda / 120.) as c_double) as c_int;
    let refbits = dest_bits;
    let too_many_bits;
    let too_few_bits;
    let mut dists = WindowedArray::<_, 16>([0.; 128]);
    let mut qenergies = WindowedArray::<_, 16>([0.; 128]);
    let mut rdlambda = (2. * 120. / lambda).clamp(0.0625, 16.);
    let sfoffs;
    let mut nminscaler;
    let mut its: c_int = 0;
    let mut maxits: c_int = 30;
    let zeroscale = zeroscale(lambda);

    if s.psy.bitres.alloc >= 0 {
        // Psy granted us extra bits to use, from the reservoire
        // adjust for lambda except what psy already did
        dest_bits = (s.psy.bitres.alloc as c_float
            * (lambda
                / (if avctx.global_quality().get() != 0 {
                    avctx.global_quality().get()
                } else {
                    120
                }) as c_float)) as c_int;
    }

    if avctx.flags().get().qscale() {
        // Constant Q-scale doesn't compensate MS coding on its own
        // No need to be overly precise, this only controls RD
        // adjustment CB limits when going overboard
        if s.options.mid_side != 0 && s.cur_type == SyntaxElementType::ChannelPairElement {
            dest_bits *= 2;
        }

        // When using a constant Q-scale, don't adjust bits, just use RD
        // Don't let it go overboard, though... 8x psy target is enough
        too_many_bits = 5800;
        too_few_bits = dest_bits / 16;

        // Don't offset scalers, just RD
        sfoffs = c_float::from(c_uchar::from(sce.ics.num_windows) - 1);
        rdlambda = rdlambda.sqrt();

        // search further
        maxits *= 2;
    } else {
        // When using ABR, be strict, but a reasonable leeway is
        // critical to allow RC to smoothly track desired bitrate
        // without sudden quality drops that cause audible artifacts.
        // Symmetry is also desirable, to avoid systematic bias.
        too_many_bits = dest_bits + dest_bits / 8;
        too_few_bits = dest_bits - dest_bits / 8;

        sfoffs = 0.;
        rdlambda = rdlambda.sqrt();
    }
    let wlen = 1024 / c_int::from(c_uchar::from(sce.ics.num_windows));

    let frame_bit_rate = frame_bit_rate(avctx, &*s, refbits, 1.5);
    let bandwidth = if avctx.cutoff().get() > 0 {
        avctx.cutoff().get()
    } else {
        cutoff_from_bitrate(frame_bit_rate, 1, avctx.sample_rate().get()).max(3000)
    };
    s.psy.cutoff = bandwidth;

    let cutoff = bandwidth * 2 * wlen / avctx.sample_rate().get();
    let pns_start_pos = 4000 * 2 * wlen / avctx.sample_rate().get();

    // for values above this the decoder might end up in an endless loop
    // due to always having more bits than what can be encoded.
    let dest_bits = dest_bits.min(5800);
    let too_many_bits = too_many_bits.min(5800);
    let too_few_bits = too_few_bits.min(5800);

    let InitialQuantizers {
        mut uplims,
        energies,
        nzs,
        spread_thr_r,
        min_spread_thr_r,
        max_spread_thr_r,
        allz,
    } = calc_initial_quantizers(&mut *sce, &*s, cutoff, zeroscale);

    let energies = WindowedArray::<_, 16>(energies);
    let spread_thr_r = WindowedArray::<_, 16>(spread_thr_r);

    let minscaler = find_min_scaler(&mut *sce, &uplims, sfoffs);

    clip_non_zeros(&mut *sce, minscaler);

    if let AllZ::False = allz {
        return;
    }

    **s.scaled_coeffs = sce.coeffs.map(Pow34::abs_pow34);

    s.quantize_band_cost_cache.init();

    let (minsf, maxvals) = calc_minsf_maxvals(&sce.ics, WindowedArray::from_ref(&s.scaled_coeffs));
    let euplims = WindowedArray::<_, 16>(scale_uplims(
        &sce.ics,
        &mut uplims,
        &sce.coeffs,
        &nzs,
        cutoff,
        rdlambda,
        avctx.flags().get().qscale(),
    ));

    let uplims = WindowedArray::<_, 16>::from_ref(&uplims);
    let mut maxsf = WindowedArray::<_, 16>([c_int::from(SCALE_MAX_POS); 128]);

    // perform two-loop search
    // outer loop - improve quality
    loop {
        let InnerLoopResult {
            mut tbits,
            mut recomprd,
        } = quantize_spectrum(
            sce,
            s,
            its,
            &maxvals,
            &maxsf,
            &minsf,
            too_many_bits,
            too_few_bits,
            &mut dists,
            &mut qenergies,
        );

        let mut overdist = true;
        let mut fflag = tbits < too_few_bits;

        let mut i = 0..2;
        while let Some(i) = i.next()
            && (overdist || recomprd)
        {
            if recomprd {
                let mut prev = None::<c_int>;
                tbits = 0;
                for wi @ WindowedIteration { w, .. } in sce.ics.iter_windows() {
                    let coeffs = [&sce.coeffs, &s.scaled_coeffs].map(|arr| &arr[W(w)]);
                    for (
                        g,
                        (&zero, &sf_idx, &can_pns, &maxval, dist, qenergy, (swb_size, offset)),
                    ) in izip!(
                        &sce.zeroes[W(w)],
                        &sce.sf_idx[W(w)],
                        &sce.can_pns[W(w)],
                        &maxvals[W(w)],
                        &mut dists[W(w)],
                        &mut qenergies[W(w)],
                        sce.ics.iter_swb_sizes_sum(),
                    )
                    .enumerate()
                    {
                        let [coeffs, scaled] =
                            coeffs.map(|arr| WindowedArray::from_ref(&arr[offset.into()..]));
                        if zero || sf_idx >= 218 {
                            if can_pns {
                                tbits += if let Some(g) = g.checked_sub(1)
                                    && sce.zeroes[W(w)][g]
                                    && sce.can_pns[W(w)][g]
                                {
                                    5
                                } else {
                                    9
                                };
                            }
                            continue;
                        }

                        let cb = find_min_book(maxval, sf_idx);
                        let cost = wi.calc_quantization_cost_sum(
                            &mut s.quantize_band_cost_cache,
                            g as c_int,
                            coeffs,
                            scaled,
                            swb_size,
                            sf_idx,
                            cb,
                        );
                        *dist = cost.distortion - cost.bits as c_float;
                        *qenergy = cost.energy;
                        if let Some(prev) = prev {
                            let sf_diff = (sf_idx - prev + 60).clamp(0, 2 * 60);
                            tbits += c_int::from(SCALEFACTOR_BITS[sf_diff as usize]);
                        }
                        tbits += cost.bits;
                        prev = Some(sf_idx);
                    }
                }
            }

            if !(i == 0 && s.options.pns != 0 && its > maxits / 2 && tbits > too_few_bits) {
                continue;
            }

            let ovrfactor = 1. + (maxits - its) as c_float * 16. / maxits as c_float;
            recomprd = false;
            overdist = sce
                .ics
                .iter_windows()
                .flat_map(|WindowedIteration { w, .. }| {
                    izip!(
                        &sce.zeroes[W(w)],
                        &sce.sf_idx[W(w)],
                        &dists[W(w)],
                        &uplims[W(w)],
                    )
                    .take(sce.ics.num_swb as usize)
                })
                .any(|(&zero, &sf_idx, &dist, &uplim)| {
                    !zero && sf_idx > 140 && dist > uplim * ovrfactor
                });

            if !overdist {
                continue;
            }

            let mut zeroed = 0;
            let (maxspread, minspread, zeroable) = sce
                .ics
                .iter_windows()
                .flat_map(|WindowedIteration { w, .. }| {
                    izip!(
                        sce.ics.iter_swb_sizes_sum(),
                        &sce.zeroes[W(w)],
                        &sce.can_pns[W(w)],
                        &spread_thr_r[W(w)],
                    )
                })
                .filter(|((_, start), &zero, &can_pns, ..)| {
                    c_int::from(*start) >= pns_start_pos && !zero && can_pns
                })
                .fold(
                    (min_spread_thr_r, max_spread_thr_r, 0),
                    |(max, min, count), (.., &spread_thr_r)| {
                        (max.max(spread_thr_r), min.min(spread_thr_r), count + 1)
                    },
                );
            let zspread = (maxspread - minspread) * 0.0125 + minspread;
            // Don't PNS everything even if allowed. It suppresses bit starvation signals
            // from RC, and forced the hand of the later search_for_pns
            // step. Instead, PNS a fraction of the spread_thr_r range
            // depending on how starved for bits we are,
            // and leave further PNSing to search_for_pns if worthwhile.
            let zspread = (min_spread_thr_r * 8.).min(zspread).min(
                ((too_many_bits - tbits) as c_float * min_spread_thr_r
                    + (tbits - too_few_bits) as c_float * max_spread_thr_r)
                    / (too_many_bits - too_few_bits + 1) as c_float,
            );
            let maxzeroed = (zeroable as c_int)
                .min(1.max((zeroable as c_int * its + maxits - 1) / (2 * maxits)));
            for zloop in 0..2 {
                // Two passes: first distorted stuff - two birds in one shot and all that,
                // then anything viable. Viable means not zero, but either CB=zero-able
                // (too high SF), not SF <= 1 (that means we'd be operating at very high
                // quality, we don't want PNS when doing VHQ), PNS allowed, and within
                // the lowest ranking percentile.
                let loopovrfactor: c_float = if zloop != 0 { 1. } else { ovrfactor };
                let loopminsf = c_int::from(if zloop != 0 {
                    SCALE_ONE_POS - SCALE_DIV_512
                } else {
                    SCALE_ONE_POS
                });

                let mut g = (1..sce.ics.num_swb).rev();
                while let Some(g) = g.next()
                    && zeroed < maxzeroed
                {
                    if c_int::from(sce.ics.swb_offset[g as usize]) < pns_start_pos {
                        continue;
                    }

                    let SingleChannelElement {
                        zeroes,
                        ref can_pns,
                        ref sf_idx,
                        band_type,
                        ref ics,
                        ..
                    } = sce;
                    let zeroes = zeroes.as_array_of_cells_deref();

                    zeroed += ics
                        .iter_windows()
                        .filter(|&WindowedIteration { w, .. }| {
                            !zeroes[W(w)][g as usize].get()
                                && can_pns[W(w)][g as usize]
                                && spread_thr_r[W(w)][g as usize] <= zspread
                                && sf_idx[W(w)][g as usize] > loopminsf
                                && {
                                    let dist = dists[W(w)][g as usize];
                                    dist > loopovrfactor * uplims[W(w)][g as usize]
                                        || (dist
                                            > uplims[W(w)][g as usize]
                                                .min(euplims[W(w)][g as usize])
                                            && find_min_book(
                                                maxvals[W(w)][g as usize],
                                                sf_idx[W(w)][g as usize],
                                            ) <= 1)
                                }
                        })
                        .map(|WindowedIteration { w, .. }| {
                            zeroes[W(w)][g as usize].set(true);
                            band_type[W(w)][g as usize] = ZERO_BT;
                        })
                        .count() as c_int;
                }
            }
            if zeroed != 0 {
                fflag = true;
                recomprd = true;
            }
        }

        nminscaler = sce
            .ics
            .iter_windows()
            .flat_map(|WindowedIteration { w, .. }| {
                zip(&sce.zeroes[W(w)], &sce.sf_idx[W(w)]).take(sce.ics.num_swb as usize)
            })
            .filter_map(|(&zero, &sf_idx)| (!zero).then_some(sf_idx))
            .min()
            .map(|min| {
                min.clamp(
                    c_int::from(SCALE_ONE_POS - SCALE_DIV_512),
                    c_int::from(SCALE_MAX_POS - SCALE_DIV_512),
                )
            })
            .unwrap_or((SCALE_MAX_POS - SCALE_DIV_512).into());

        let mut prev = None;
        let sf_indices = sce.sf_idx.as_array_of_cells_deref();
        for wi @ WindowedIteration { w, .. } in sce.ics.iter_windows() {
            // Start with big steps, end up fine-tunning
            let depth = if its > maxits / 2 {
                if its > maxits * 2 / 3 {
                    1
                } else {
                    3
                }
            } else {
                10
            };
            let edepth = depth + 2;
            let mut uplmax = its as c_float / (maxits as c_float * 0.25) + 1.;
            if tbits > dest_bits {
                uplmax *= c_float::min(2., tbits as c_float / dest_bits.max(1) as c_float);
            };

            for (
                g,
                (
                    (swb_size, start),
                    sf_idx,
                    _,
                    &maxval,
                    dist,
                    &uplim,
                    &euplim,
                    &minsf,
                    maxsf,
                    &energy,
                    qenergy,
                    band_type,
                ),
            ) in izip!(
                sce.ics.iter_swb_sizes_sum(),
                &sf_indices[W(w)],
                &sce.zeroes[W(w)],
                &maxvals[W(w)],
                &mut dists[W(w)],
                &uplims[W(w)],
                &euplims[W(w)],
                &minsf[W(w)],
                &mut maxsf[W(w)],
                &energies[W(w)],
                &mut qenergies[W(w)],
                &mut sce.band_type[W(w)],
            )
            .enumerate()
            .filter(|(_, (_, _, &zero, ..))| !zero)
            {
                let prevsc = sf_idx.get();
                let prev = prev.get_or_insert_with(|| sf_indices[W(0)][0].get());
                let coeffs = WindowedArray::<_, 128>::from_ref(&sce.coeffs[W(w)][start.into()..]);
                let scaled =
                    WindowedArray::<_, 128>::from_ref(&s.scaled_coeffs[W(w)][start.into()..]);
                let cmb = find_min_book(maxval, sf_idx.get());
                let mindeltasf = c_int::max(0, *prev - c_int::from(SCALE_MAX_DIFF));
                let maxdeltasf = c_int::min(
                    (SCALE_MAX_POS - SCALE_DIV_512).into(),
                    *prev + c_int::from(SCALE_MAX_DIFF),
                );
                if (cmb == 0 || *dist > uplim) && sf_idx.get() > mindeltasf.max(minsf) {
                    // Try to make sure there is some energy in every nonzero band
                    // NOTE: This algorithm must be forcibly imbalanced, pushing harder
                    //  on holes or more distorted bands at first, otherwise there's
                    //  no net gain (since the next iteration will offset all bands
                    //  on the opposite direction to compensate for extra bits)
                    let mut i = 0..edepth;
                    while let Some(i) = i.next()
                        && sf_idx.get() > mindeltasf
                    {
                        let mb = find_min_book(maxval, sf_idx.get() - 1);
                        let cb = find_min_book(maxval, sf_idx.get());
                        if cb == 0 {
                            *maxsf = c_int::min(sf_idx.get() - 1, *maxsf);
                        } else if i >= depth && *dist < euplim {
                            break;
                        }
                        if g == 0 && sce.ics.num_windows == WindowCount::Eight && *dist >= euplim {
                            *maxsf = c_int::min(*maxsf, sf_idx.get());
                        }
                        let cost = wi.calc_quantization_cost_sum(
                            &mut s.quantize_band_cost_cache,
                            g as c_int,
                            coeffs,
                            scaled,
                            swb_size,
                            sf_idx.get() - 1,
                            cb,
                        );
                        sf_idx.update(|idx| idx - 1);
                        *dist = cost.distortion - cost.bits as c_float;
                        *qenergy = cost.energy;
                        if mb != 0
                            && (sf_idx.get() < mindeltasf
                                || *dist < (uplmax * uplim).min(euplim)
                                    && (*qenergy - energy).abs() < euplim)
                        {
                            break;
                        }
                    }
                } else if tbits > too_few_bits
                    && sf_idx.get() < maxdeltasf.min(*maxsf)
                    && *dist < uplim.min(euplim)
                    && (*qenergy - energy).abs() < euplim
                {
                    // Um... over target. Save bits for more important stuff.
                    let mut i = 0..depth;
                    while let Some(_) = i.next()
                        && sf_idx.get() < maxdeltasf
                    {
                        let cb = find_min_book(maxval, sf_idx.get() + 1);
                        if cb == 0 {
                            *maxsf = sf_idx.get().min(*maxsf);
                            break;
                        }

                        let mut cost = wi.calc_quantization_cost_sum(
                            &mut s.quantize_band_cost_cache,
                            g as c_int,
                            coeffs,
                            scaled,
                            swb_size,
                            sf_idx.get() + 1,
                            cb,
                        );
                        cost.distortion -= cost.bits as c_float;
                        if cost.distortion >= uplim.min(euplim) {
                            break;
                        }
                        sf_idx.update(|idx| idx + 1);
                        *dist = cost.distortion;
                        *qenergy = cost.energy;
                    }
                }
                *prev = sf_idx.update(|idx| idx.clamp(mindeltasf, maxdeltasf));
                if *prev != prevsc {
                    fflag = true;
                }
                nminscaler = nminscaler.min(*prev);
                *band_type = find_min_book(maxval, *prev) as BandType;
            }
        }

        // SF difference limit violation risk. Must re-clamp.
        let mut prev = None;
        for WindowedIteration { w, .. } in sce.ics.iter_windows() {
            for (.., &maxval, sf_idx, band_type) in izip!(
                &sce.zeroes[W(w)],
                &maxvals[W(w)],
                &mut sce.sf_idx[W(w)],
                &mut sce.band_type[W(w)],
            )
            .take(sce.ics.num_swb as usize)
            .filter(|(&zero, ..)| !zero)
            {
                let prevsf = *sf_idx;
                let prev = prev.get_or_insert(prevsf);
                *sf_idx = (*sf_idx).clamp(*prev - 60, *prev + 60);
                *band_type = find_min_book(maxval, *sf_idx) as BandType;
                *prev = *sf_idx;
                if !fflag && prevsf != *sf_idx {
                    fflag = true;
                }
            }
        }

        its += 1;

        if !(fflag && its < maxits) {
            break;
        }
    }

    find_next_bands(sce, *maxvals);
}

#[ffmpeg_src(file = "libavcodec/aaccoder_twoloop.h", lines = 297..=312)]
fn calc_minsf_maxvals(
    ics: &IndividualChannelStream,
    scaled_coeffs: &WindowedArray<[c_float; 1024], 128>,
) -> (
    WindowedArray<[c_int; 128], 16>,
    WindowedArray<[c_float; 128], 16>,
) {
    let mut minsf = WindowedArray([0; 128]);
    let mut maxvals = WindowedArray([0.; 128]);
    for WindowedIteration { w, group_len } in ics.iter_windows() {
        let minsf = &minsf.as_array_of_cells()[W(w)];
        for ((swb_size, offset), maxval, minsf) in izip!(
            ics.iter_swb_sizes_sum(),
            &mut maxvals[W(w)],
            successors(Some(minsf), |minsf| minsf.get(1..)),
        ) {
            let scaled = &scaled_coeffs[W2(w)];

            *maxval = scaled
                .into_iter()
                .take(group_len.into())
                .flat_map(|scaled| scaled.iter().skip(offset.into()).take(swb_size.into()))
                .copied()
                .max_by(c_float::total_cmp)
                .unwrap_or_default();
            if *maxval <= 0. {
                continue;
            }

            let minsfidx = maxval.coef2minsf().into();
            for minsf in minsf.iter().step_by(16).take(group_len.into()) {
                minsf.set(minsfidx);
            }
        }
    }
    (minsf, maxvals)
}

#[ffmpeg_src(file = "libavcodec/aaccoder_twoloop.h", lines = 285..=290)]
fn clip_non_zeros(sce: &mut SingleChannelElement, minscaler: i32) {
    let minscaler = minscaler.clamp(
        (SCALE_ONE_POS - SCALE_DIV_512).into(),
        (SCALE_MAX_POS - SCALE_DIV_512).into(),
    );

    for WindowedIteration { w, .. } in sce.ics.iter_windows() {
        let num_swb = sce.ics.num_swb as usize;

        for (_, sf_idx) in zip(
            &sce.zeroes[W(w)][..num_swb],
            &mut sce.sf_idx[W(w)][..num_swb],
        )
        .filter(|(&zero, _)| !zero)
        {
            *sf_idx = (*sf_idx).clamp(minscaler, minscaler + i32::from(SCALE_MAX_DIFF) - 1);
        }
    }
}

/// Compute initial scalers
#[ffmpeg_src(file = "libavcodec/aaccoder_twoloop.h", lines = 262..=283)]
fn find_min_scaler(
    sce: &mut SingleChannelElement,
    uplims: &[c_float; 128],
    sfoffs: c_float,
) -> c_int {
    let swb_sizes = &sce.ics.swb_sizes[..sce.ics.num_swb as usize];

    let sf_idx = Cell::as_array_of_cells(Cell::from_mut(&mut *sce.sf_idx));

    sce.ics
        .iter_windows()
        .flat_map(|WindowedIteration { w, .. }| {
            izip!(
                &sf_idx[(w * 16) as usize..],
                &sce.zeroes[W(w)],
                &uplims[(w * 16) as usize..],
                swb_sizes
            )
        })
        .filter_map(|(sf_idx, &zero, &uplim, &swb_size)| {
            if zero {
                sf_idx.set(SCALE_ONE_POS.into());
                return None;
            }

            sf_idx.set(
                // log2f-to-distortion ratio is, technically, 2 (1.5db = 4, but it's power vs
                // level so it's 2). But, as offsets are applied, low-frequency signals are too
                // sensitive to the induced distortion, so we make scaling more conservative by
                // choosing a lower log2f-to-distortion ratio, and thus more robust.
                c_double::clamp(
                    1.75 * c_double::from((uplim.max(0.00125) / c_float::from(swb_size)).log2())
                        + c_double::from(SCALE_ONE_POS)
                        + c_double::from(sfoffs),
                    60.,
                    SCALE_MAX_POS.into(),
                ) as c_int,
            );
            Some(sf_idx.get())
        })
        .min()
        .unwrap_or(c_int::from(c_ushort::MAX))
}

/// Scale uplims to match rate distortion to quality
/// bu applying noisy band depriorization and tonal band priorization.
/// Maxval-energy ratio gives us an idea of how noisy/tonal the band is.
/// If maxval^2 ~ energy, then that band is mostly noise, and we can
/// relax rate distortion requirements.
#[ffmpeg_src(file = "libavcodec/aaccoder_twoloop.h", lines = 314..=359)]
fn scale_uplims(
    ics: &IndividualChannelStream,
    uplims: &mut [c_float; 128],
    coeffs: &[c_float; 1024],
    nzs: &[c_char; 128],
    cutoff: c_int,
    rdlambda: c_float,
    qscale: bool,
) -> [c_float; 128] {
    const NZ_SLOPE: c_float = 1.5;
    const RDMIN: c_float = 0.03125;
    const RDMAX: c_float = 1.;

    let coeffs = WindowedArray::<_, 128>::from_ref(coeffs);
    let uplims = WindowedArray::<_, 16>::from_mut(uplims);
    let mut euplims = *uplims;
    for WindowedIteration { w, group_len } in ics.iter_windows() {
        // psy already priorizes transients to some extent
        let de_psy_factor = if ics.num_windows == WindowCount::Eight {
            8. / c_float::from(group_len)
        } else {
            1.
        };

        let coeffs = &coeffs[W(w)];

        for ((swb_size, offset), &nz, uplim, euplim) in izip!(
            ics.iter_swb_sizes_sum(),
            nzs,
            &mut uplims[W(w)],
            &mut euplims[W(w)],
        )
        .filter(|(_, &nz, ..)| nz > 0)
        {
            let start = w * 128 + c_int::from(offset);
            let coeffs = &coeffs[offset.into()..];
            let cleanup_factor = (start as c_float / (cutoff as c_float * 0.75))
                .clamp(1., 2.)
                .powi(2);
            let mut energy2uplim = find_form_factor(
                group_len,
                swb_size,
                *uplim / (c_int::from(nz) * c_int::from(ics.swb_sizes[w as usize])) as c_float,
                coeffs,
                NZ_SLOPE * cleanup_factor,
            );
            energy2uplim *= de_psy_factor;
            if !qscale {
                // In ABR, we need to priorize less and let rate control do its thing
                energy2uplim = energy2uplim.sqrt();
            }
            energy2uplim = energy2uplim.clamp(0.015625, 1.);
            *uplim *= (rdlambda * energy2uplim).clamp(RDMIN, RDMAX) * c_float::from(group_len);

            let mut energy2uplim = find_form_factor(
                group_len,
                swb_size,
                *uplim / (c_int::from(nz) * c_int::from(ics.swb_sizes[w as usize])) as c_float,
                coeffs,
                2.,
            );
            energy2uplim *= de_psy_factor;
            if !qscale {
                // In ABR, we need to priorize less and let rate control do its thing
                energy2uplim = energy2uplim.sqrt();
            }
            energy2uplim = energy2uplim.clamp(0.015625, 1.);
            *euplim *=
                (rdlambda * energy2uplim * c_float::from(ics.group_len[w as usize])).clamp(0.5, 1.);
        }
    }

    *euplims
}

#[derive(Default, PartialEq, Eq)]
enum AllZ {
    #[default]
    False,
    True,
}

impl BitOrAssign<bool> for AllZ {
    fn bitor_assign(&mut self, rhs: bool) {
        if rhs {
            *self = Self::True;
        }
    }
}

struct InitialQuantizers {
    uplims: [f32; 128],
    energies: [f32; 128],
    nzs: [i8; 128],
    spread_thr_r: [f32; 128],
    min_spread_thr_r: f32,
    max_spread_thr_r: f32,
    allz: AllZ,
}

impl Default for InitialQuantizers {
    fn default() -> Self {
        Self {
            uplims: [0.; 128],
            energies: [0.; 128],
            nzs: [0; 128],
            spread_thr_r: [0.; 128],
            min_spread_thr_r: -1.,
            max_spread_thr_r: -1.,
            allz: Default::default(),
        }
    }
}

/// XXX: some heuristic to determine initial quantizers will reduce search time
/// determine zero bands and upper distortion limits
#[ffmpeg_src(file = "libavcodec/aaccoder_twoloop.h", lines = 217..=260)]
fn calc_initial_quantizers(
    sce: &mut SingleChannelElement,
    s: &AACEncContext,
    cutoff: i32,
    zeroscale: f32,
) -> InitialQuantizers {
    let mut res = InitialQuantizers::default();
    let InitialQuantizers {
        uplims,
        energies,
        nzs,
        spread_thr_r,
        min_spread_thr_r,
        max_spread_thr_r,
        allz,
    } = &mut res;

    let uplims = WindowedArray::<_, 16>::from_mut(uplims);
    let energies = WindowedArray::<_, 16>::from_mut(energies);
    let nzs = WindowedArray::<_, 16>::from_mut(nzs);
    let spread_thr_r = WindowedArray::<_, 16>::from_mut(spread_thr_r);

    let SingleChannelElement {
        ref ics,
        ref can_pns,
        ref mut zeroes,
        ..
    } = *sce;

    let FFPsyChannel { psy_bands, .. } = &s.psy.ch[s.cur_channel as usize];
    let zeroes = zeroes.as_array_of_cells_deref();

    for WindowedIteration { w, group_len } in ics.iter_windows() {
        for ((_, start), uplim, energy, nz, spread_thr_r, &can_pns, psy_bands, zeroes) in izip!(
            ics.iter_swb_sizes_sum(),
            &mut uplims[W(w)],
            &mut energies[W(w)],
            &mut nzs[W(w)],
            &mut spread_thr_r[W(w)],
            &can_pns[W(w)],
            successors(Some(&psy_bands[W(w)]), |psy_bands| psy_bands.get(1..)),
            successors(Some(&zeroes[W(w)]), |zeroes| zeroes.get(1..)),
        ) {
            let mut spread: c_float = 0.;
            *uplim = 0.;
            *energy = 0.;
            *nz = zip(psy_bands, zeroes)
                .step_by(16)
                .take(group_len.into())
                .filter(|(band, zero)| {
                    if c_int::from(start) >= cutoff
                        || band.energy <= band.threshold * zeroscale
                        || band.threshold == 0.
                    {
                        zero.set(true);
                        false
                    } else {
                        *uplim += band.threshold;
                        *energy += band.energy;
                        spread += band.spread;
                        true
                    }
                })
                .count() as c_char;
            zeroes[0].set(*nz == 0);
            *allz |= *nz > 0;
            if *nz > 0 && can_pns {
                *spread_thr_r = *energy * *nz as c_float / (*uplim * spread);
                (*min_spread_thr_r, *max_spread_thr_r) = if *min_spread_thr_r < 0. {
                    (*spread_thr_r, *spread_thr_r)
                } else {
                    (
                        min_spread_thr_r.min(*spread_thr_r),
                        max_spread_thr_r.max(*spread_thr_r),
                    )
                }
            }
        }
    }

    res
}

/// zeroscale controls a multiplier of the threshold, if band energy
/// is below this, a zero is forced. Keep it lower than 1, unless
/// low lambda is used, because energy < threshold doesn't mean there's
/// no audible signal outright, it's just energy. Also make it rise
/// slower than rdlambda, as rdscale has due compensation with
/// noisy band depriorization below, whereas zeroing logic is rather dumb
#[ffmpeg_src(file = "libavcodec/aaccoder_twoloop.h", lines = 123..=128)]
fn zeroscale(lambda: f32) -> f32 {
    if lambda > 120. {
        (120. / lambda).powf(0.25).clamp(0.0625, 1.)
    } else {
        1.
    }
}

/// Scale, psy gives us constant quality, this LP only scales
/// bitrate by lambda, so we save bits on subjectively unimportant HF
/// rather than increase quantization noise. Adjust nominal bitrate
/// to effective bitrate according to encoding parameters,
/// AAC_CUTOFF_FROM_BITRATE is calibrated for effective bitrate.
#[ffmpeg_src(file = "libavcodec/aaccoder_twoloop.h", lines = 187..=193)]
fn frame_bit_rate(
    avctx: &CodecContext,
    s: &AACEncContext,
    refbits: i32,
    rate_bandwidth_multiplier: f32,
) -> i32 {
    let mut frame_bit_rate = if avctx.flags().get().qscale() {
        refbits as c_float * rate_bandwidth_multiplier * avctx.sample_rate().get() as c_float
            / 1024.
    } else {
        (avctx.bit_rate().get() / c_long::from(avctx.ch_layout().get().nb_channels)) as c_float
    };

    // Compensate for extensions that increase efficiency
    if s.options.pns != 0 || s.options.intensity_stereo != 0 {
        frame_bit_rate *= 1.15;
    }

    frame_bit_rate as c_int
}

/// Scout out next nonzero bands
#[ffmpeg_src(file = "libavcodec/aaccoder_twoloop.h", lines = 727..=760)]
fn find_next_bands(sce: &mut SingleChannelElement, maxvals: [c_float; 128]) {
    let nextband = sce.init_next_band_map();

    let SingleChannelElement {
        ics: ref ics @ IndividualChannelStream { num_swb, .. },
        ref mut zeroes,
        ref mut band_type,
        sf_idx: ref mut sf_indices,
        ..
    } = *sce;
    let zeroes = Cell::from_mut(&mut ***zeroes).as_array_of_cells();
    let sf_indices = Cell::from_mut(&mut ***sf_indices).as_array_of_cells();

    let mut prev = None;
    for WindowedIteration { w, .. } in ics.iter_windows() {
        let i = (w * 16) as usize;
        // Make sure proper codebooks are set
        for (zero, band_type, maxval, sf_idx, g) in izip!(
            &zeroes[i..],
            &mut band_type[W(w)],
            &maxvals[i..],
            &sf_indices[i..],
            0..num_swb,
        ) {
            if !zero.get() {
                *band_type = find_min_book(*maxval, sf_idx.get()) as BandType;
                if *band_type == 0 {
                    if !prev.is_some_and(|sf| {
                        sfdelta_encoding_range(sf)
                            .contains(&sf_indices[usize::from(nextband[i + g as usize])].get())
                    }) {
                        // Cannot zero out, make sure it's not attempted
                        *band_type = 1 as BandType;
                    } else {
                        zero.set(true);
                        *band_type = ZERO_BT;
                    }
                }
            } else {
                *band_type = ZERO_BT;
            }
            if !zero.get() {
                if let Some(prev) = prev {
                    let sfdiff: c_int = sf_idx.get() - prev + c_int::from(SCALE_DIFF_ZERO);
                    assert!((0..=2 * c_int::from(SCALE_MAX_DIFF)).contains(&sfdiff));
                } else if zeroes[0].get() {
                    sf_indices[0].set(sf_idx.get());
                }
                prev = Some(sf_idx.get());
            }
        }
    }
}

struct InnerLoopResult {
    tbits: c_int,
    recomprd: bool,
}

/// inner loop - quantize spectrum to fit into given number of bits
#[ffmpeg_src(file = "libavcodec/aaccoder_twoloop.h", lines = 367..=447)]
fn quantize_spectrum(
    sce: &mut SingleChannelElement,
    s: &mut AACEncContext,
    its: c_int,
    maxvals: &WindowedArray<[c_float; 128], 16>,
    maxsf: &[c_int; 128],
    minsf: &[c_int; 128],
    too_many_bits: c_int,
    too_few_bits: c_int,
    dists: &mut WindowedArray<[c_float; 128], 16>,
    qenergies: &mut WindowedArray<[c_float; 128], 16>,
) -> InnerLoopResult {
    let mut qstep: c_int = if its != 0 { 1 } else { 32 };
    loop {
        let mut prev: Option<c_int> = None;
        let mut tbits = 0;
        for wi @ WindowedIteration { w, .. } in sce.ics.iter_windows() {
            let [coeffs, scaled] = [&sce.coeffs, &s.scaled_coeffs].map(|c| &c[W(w)]);
            for (g, (&zero, &sf_idx, &can_pns, &maxval, dist, qenergy, (swb_size, offset))) in
                izip!(
                    &sce.zeroes[W(w)],
                    &sce.sf_idx[W(w)],
                    &sce.can_pns[W(w)],
                    &maxvals[W(w)],
                    &mut dists[W(w)],
                    &mut qenergies[W(w)],
                    sce.ics.iter_swb_sizes_sum(),
                )
                .enumerate()
            {
                let [coeffs, scaled] = [coeffs, scaled]
                    .map(|coeffs| WindowedArray::from_ref(&coeffs[offset.into()..]));
                if zero || sf_idx >= 218 {
                    if can_pns {
                        // PNS isn't free
                        tbits += if let Some(g) = g.checked_sub(1)
                            && sce.zeroes[W(w)][g]
                            && sce.can_pns[W(w)][g]
                        {
                            5
                        } else {
                            9
                        };
                    }
                    continue;
                }

                let cb = find_min_book(maxval, sf_idx);
                let cost = wi.calc_quantization_cost_sum(
                    &mut s.quantize_band_cost_cache,
                    g as c_int,
                    coeffs,
                    scaled,
                    swb_size,
                    sf_idx,
                    cb,
                );
                *dist = cost.distortion - cost.bits as c_float;
                *qenergy = cost.energy;
                tbits += cost.bits;
                if let Some(prev) = prev {
                    let sfdiff = (sf_idx - prev + c_int::from(SCALE_DIFF_ZERO))
                        .clamp(0, 2 * c_int::from(SCALE_MAX_DIFF));
                    tbits += c_int::from(SCALEFACTOR_BITS[sfdiff as usize]);
                }
                prev = Some(sf_idx);
            }
        }
        let mut changed = false;
        let mut recomprd = false;
        if tbits > too_many_bits {
            recomprd = true;
            for (sf_idx, &maxsf) in zip(&mut *sce.sf_idx, maxsf)
                .filter(|(&mut sf_idx, _)| sf_idx < c_int::from(SCALE_MAX_POS - SCALE_DIV_512))
            {
                let maxsf = if tbits <= 5800 {
                    maxsf
                } else {
                    SCALE_MAX_POS.into()
                };
                let new_sf = maxsf.min(*sf_idx + qstep);
                if new_sf != *sf_idx {
                    *sf_idx = new_sf;
                    changed = true;
                }
            }
        } else if tbits < too_few_bits {
            recomprd = true;
            for (sf_idx, &minsf) in zip(&mut *sce.sf_idx, minsf)
                .filter(|(&mut sf_idx, _)| sf_idx > c_int::from(SCALE_ONE_POS))
            {
                let new_sf = minsf.max(SCALE_ONE_POS.into()).max(*sf_idx - qstep);
                if new_sf != *sf_idx {
                    *sf_idx = new_sf;
                    changed = true;
                }
            }
        }
        qstep = match qstep >> 1 {
            0 if tbits > too_many_bits && sce.sf_idx[W(0)][0] < 217 && changed => 1,
            0 => {
                break InnerLoopResult { tbits, recomprd };
            }
            qstep => qstep,
        };
    }
}
