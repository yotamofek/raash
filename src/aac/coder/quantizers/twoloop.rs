use std::{cell::Cell, iter::zip, ops::BitOrAssign};

use array_util::W;
use encoder::CodecContext;
use ffmpeg_src_macro::ffmpeg_src;
use izip::izip;
use libc::{c_char, c_double, c_float, c_int, c_long, c_uchar, c_ushort};
use reductor::{Reduce as _, Sum};

use crate::{
    aac::{
        coder::{
            find_form_factor, find_min_book, math::Float as _, quantization::QuantizationCost,
            sfdelta_encoding_range,
        },
        encoder::{ctx::AACEncContext, pow::Pow34},
        psy_model::cutoff_from_bitrate,
        tables::SCALEFACTOR_BITS,
        IndividualChannelStream, SyntaxElementType, WindowedIteration, SCALE_DIFF_ZERO,
        SCALE_DIV_512, SCALE_MAX_DIFF, SCALE_MAX_POS, SCALE_ONE_POS,
    },
    types::*,
};

#[ffmpeg_src(file = "libavcodec/aaccoder_twoloop.h", lines = 67..=761, name = "search_for_quantizers_twoloop")]
pub(crate) fn search(
    avctx: &CodecContext,
    s: &mut AACEncContext,
    sce: &mut SingleChannelElement,
    lambda: c_float,
) {
    let mut start: c_int = 0;
    let mut g: c_int = 0;
    let mut dest_bits: c_int = (avctx.bit_rate().get() as c_double * 1024.
        / avctx.sample_rate().get() as c_double
        / (if avctx.flags().get().qscale() {
            2.
        } else {
            avctx.ch_layout().get().nb_channels as c_float
        }) as c_double
        * (lambda / 120.) as c_double) as c_int;
    let mut refbits: c_int = dest_bits;
    let mut too_many_bits: c_int = 0;
    let mut too_few_bits: c_int = 0;
    let mut dists: [c_float; 128] = [0.; 128];
    let mut qenergies: [c_float; 128] = [0.; 128];
    let mut rdlambda: c_float = (2. * 120. / lambda).clamp(0.0625, 16.);
    let mut sfoffs: c_float = ((120. / lambda).log2() * 4.).clamp(-5., 10.);
    let mut maxscaler: c_int = 0;
    let mut nminscaler: c_int = 0;
    let mut its: c_int = 0;
    let mut maxits: c_int = 30;
    let mut prev: c_int = 0;
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
    let wlen: c_int = 1024 / c_int::from(c_uchar::from(sce.ics.num_windows));

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

    let Loop1Result {
        mut uplims,
        energies,
        nzs,
        spread_thr_r,
        min_spread_thr_r,
        max_spread_thr_r,
        allz,
    } = loop1(&mut *sce, &*s, cutoff, zeroscale);

    let minscaler = find_min_scaler(&mut *sce, &uplims, sfoffs);

    clip_non_zeros(&mut *sce, minscaler);

    if let AllZ::False = allz {
        return;
    }

    **s.scaled_coeffs = sce.coeffs.map(Pow34::abs_pow34);

    s.quantize_band_cost_cache.init();

    let (minsf, maxvals) = calc_minsf_maxvals(&sce.ics, &s.scaled_coeffs);
    let euplims = scale_uplims(
        &sce.ics,
        &mut uplims,
        &sce.coeffs,
        &nzs,
        cutoff,
        rdlambda,
        avctx.flags().get().qscale(),
    );
    let uplims = uplims;

    let mut maxsf = [c_int::from(SCALE_MAX_POS); 128];

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
                prev = -1;
                tbits = 0;
                for WindowedIteration { w, group_len } in sce.ics.iter_windows() {
                    let coeffs = [&sce.coeffs, &s.scaled_coeffs].map(|arr| &arr[W(w)]);
                    let wstart = w as usize * 16;
                    for (
                        g,
                        (&zero, &sf_idx, &can_pns, &maxval, dist, qenergy, (swb_size, offset)),
                    ) in izip!(
                        &sce.zeroes[W(w)],
                        &sce.sf_idx[W(w)],
                        &sce.can_pns[W(w)],
                        &maxvals[wstart..],
                        &mut dists[wstart..],
                        &mut qenergies[wstart..],
                        sce.ics.iter_swb_sizes_sum(),
                    )
                    .enumerate()
                    {
                        let [coeffs, scaled] = coeffs.map(|arr| &arr[offset.into()..]);
                        let mut cost_0 = QuantizationCost::default();
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
                        for w2 in 0..group_len {
                            let wstart = usize::from(w2) * 128;
                            cost_0 += s.quantize_band_cost_cache.quantize_band_cost_cached(
                                w + c_int::from(w2),
                                g as c_int,
                                &coeffs[wstart..][..swb_size.into()],
                                &scaled[wstart..][..swb_size.into()],
                                sf_idx,
                                cb,
                                1.,
                                f32::INFINITY,
                                0,
                            );
                        }
                        *dist = cost_0.distortion - cost_0.bits as c_float;
                        *qenergy = cost_0.energy;
                        if prev != -1 {
                            let sf_diff = (sf_idx - prev + 60).clamp(0, 2 * 60);
                            cost_0.bits += c_int::from(SCALEFACTOR_BITS[sf_diff as usize]);
                        }
                        tbits += cost_0.bits;
                        prev = sf_idx;
                    }
                }
            }
            if i == 0 && s.options.pns != 0 && its > maxits / 2 && tbits > too_few_bits {
                let mut maxoverdist: c_float = 0.;
                let mut ovrfactor: c_float =
                    1. + (maxits - its) as c_float * 16. / maxits as c_float;
                recomprd = false;
                overdist = false;
                for WindowedIteration { w, .. } in sce.ics.iter_windows() {
                    start = 0;
                    g = start;
                    while g < sce.ics.num_swb {
                        if !sce.zeroes[W(w)][g as usize]
                            && sce.sf_idx[W(w)][g as usize] > 140
                            && dists[(w * 16 + g) as usize]
                                > uplims[(w * 16 + g) as usize] * ovrfactor
                        {
                            let mut ovrdist: c_float = dists[(w * 16 + g) as usize]
                                / c_float::max(
                                    uplims[(w * 16 + g) as usize],
                                    euplims[(w * 16 + g) as usize],
                                );
                            maxoverdist = c_float::max(maxoverdist, ovrdist);
                            overdist = true;
                        }
                        start += sce.ics.swb_sizes[g as usize] as c_int;
                        g += 1;
                    }
                }
                if overdist {
                    let mut minspread: c_float = max_spread_thr_r;
                    let mut maxspread: c_float = min_spread_thr_r;
                    let mut zspread: c_float = 0.;
                    let mut zeroable: c_int = 0;
                    let mut zeroed: c_int = 0;
                    let mut maxzeroed: c_int = 0;
                    for WindowedIteration { w, .. } in sce.ics.iter_windows() {
                        start = 0;
                        g = start;
                        while g < sce.ics.num_swb {
                            if start >= pns_start_pos
                                && !sce.zeroes[W(w)][g as usize]
                                && sce.can_pns[W(w)][g as usize]
                            {
                                minspread = minspread.min(spread_thr_r[(w * 16 + g) as usize]);
                                maxspread = maxspread.max(spread_thr_r[(w * 16 + g) as usize]);
                                zeroable += 1;
                            }
                            start += sce.ics.swb_sizes[g as usize] as c_int;
                            g += 1;
                        }
                    }
                    zspread = (maxspread - minspread) * 0.0125 + minspread;
                    // Don't PNS everything even if allowed. It suppresses bit starvation signals
                    // from RC, and forced the hand of the later search_for_pns
                    // step. Instead, PNS a fraction of the spread_thr_r range
                    // depending on how starved for bits we are,
                    // and leave further PNSing to search_for_pns if worthwhile.
                    zspread = (min_spread_thr_r * 8.).min(zspread).min(
                        ((too_many_bits - tbits) as c_float * min_spread_thr_r
                            + (tbits - too_few_bits) as c_float * max_spread_thr_r)
                            / (too_many_bits - too_few_bits + 1) as c_float,
                    );
                    maxzeroed = zeroable.min(1.max((zeroable * its + maxits - 1) / (2 * maxits)));
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
                            let zeroes = Cell::from_mut(&mut ***zeroes).as_array_of_cells();

                            zeroed += ics
                                .iter_windows()
                                .filter(|&WindowedIteration { w, .. }| {
                                    let i = (w * 16 + g) as usize;
                                    !zeroes[i].get()
                                        && can_pns[W(w)][g as usize]
                                        && spread_thr_r[i] <= zspread
                                        && sf_idx[W(w)][g as usize] > loopminsf
                                        && (dists[i] > loopovrfactor * uplims[i] || {
                                            let mcb =
                                                find_min_book(maxvals[i], sf_idx[W(w)][g as usize]);
                                            mcb == 0
                                                || mcb <= 1
                                                    && dists[i]
                                                        > c_float::min(uplims[i], euplims[i])
                                        })
                                })
                                .map(|WindowedIteration { w, .. }| {
                                    let i = (w * 16 + g) as usize;
                                    zeroes[i].set(true);
                                    band_type[W(w)][g as usize] = ZERO_BT;
                                })
                                .count() as c_int;
                        }
                    }
                    if zeroed != 0 {
                        fflag = true;
                        recomprd = true;
                    }
                } else {
                    overdist = false;
                }
            }
        }
        let mut minscaler = 255;
        maxscaler = 0;
        for WindowedIteration { w, .. } in sce.ics.iter_windows() {
            for g in 0..sce.ics.num_swb {
                if !sce.zeroes[W(w)][g as usize] {
                    minscaler = minscaler.min(sce.sf_idx[W(w)][g as usize]);
                    maxscaler = maxscaler.max(sce.sf_idx[W(w)][g as usize]);
                }
            }
        }
        nminscaler = minscaler.clamp(140 - 36, 255 - 36);
        minscaler = nminscaler;
        prev = -1;
        for WindowedIteration { w, group_len } in sce.ics.iter_windows() {
            let mut depth: c_int = if its > maxits / 2 {
                if its > maxits * 2 / 3 {
                    1
                } else {
                    3
                }
            } else {
                10
            };
            let mut edepth: c_int = depth + 2;
            let mut uplmax: c_float = its as c_float / (maxits as c_float * 0.25) + 1.;
            if tbits > dest_bits {
                uplmax *= c_float::min(2., tbits as c_float / dest_bits.max(1) as c_float);
            };
            start = 0;
            for g in 0..sce.ics.num_swb {
                let swb_size = sce.ics.swb_sizes[g as usize];
                let mut prevsc: c_int = sce.sf_idx[W(w)][g as usize];
                if prev < 0 && !sce.zeroes[W(w)][g as usize] {
                    prev = sce.sf_idx[W(0)][0];
                }
                if !sce.zeroes[W(w)][g as usize] {
                    let coefs_1 = &sce.coeffs[W(w)][start as usize..];
                    let scaled_2 = &s.scaled_coeffs[W(w)][start as usize..];
                    let mut cmb: c_int =
                        find_min_book(maxvals[(w * 16 + g) as usize], sce.sf_idx[W(w)][g as usize]);
                    let mut mindeltasf = c_int::max(0, prev - c_int::from(SCALE_MAX_DIFF));
                    let mut maxdeltasf = c_int::min(
                        (SCALE_MAX_POS - SCALE_DIV_512).into(),
                        prev + c_int::from(SCALE_MAX_DIFF),
                    );
                    if (cmb == 0 || dists[(w * 16 + g) as usize] > uplims[(w * 16 + g) as usize])
                        && sce.sf_idx[W(w)][g as usize]
                            > mindeltasf.max(minsf[(w * 16 + g) as usize])
                    {
                        // Try to make sure there is some energy in every nonzero band
                        // NOTE: This algorithm must be forcibly imbalanced, pushing harder
                        //  on holes or more distorted bands at first, otherwise there's
                        //  no net gain (since the next iteration will offset all bands
                        //  on the opposite direction to compensate for extra bits)
                        let mut i = 0;
                        while i < edepth && sce.sf_idx[W(w)][g as usize] > mindeltasf {
                            let mut cb_1: c_int = 0;
                            let mut cost_1 = QuantizationCost::default();
                            let mut mb: c_int = find_min_book(
                                maxvals[(w * 16 + g) as usize],
                                sce.sf_idx[W(w)][g as usize] - 1,
                            );
                            cb_1 = find_min_book(
                                maxvals[(w * 16 + g) as usize],
                                sce.sf_idx[W(w)][g as usize],
                            );
                            if cb_1 == 0 {
                                maxsf[(w * 16 + g) as usize] = c_int::min(
                                    sce.sf_idx[W(w)][g as usize] - 1,
                                    maxsf[(w * 16 + g) as usize],
                                );
                            } else if i >= depth
                                && dists[(w * 16 + g) as usize] < euplims[(w * 16 + g) as usize]
                            {
                                break;
                            }
                            if g == 0
                                && sce.ics.num_windows == WindowCount::Eight
                                && dists[(w * 16 + g) as usize] >= euplims[(w * 16 + g) as usize]
                            {
                                maxsf[(w * 16 + g) as usize] = c_int::min(
                                    maxsf[(w * 16 + g) as usize],
                                    sce.sf_idx[W(w)][g as usize],
                                );
                            }
                            for w2 in 0..group_len {
                                let wstart = usize::from(w2) * 128;
                                cost_1 += s.quantize_band_cost_cache.quantize_band_cost_cached(
                                    w + c_int::from(w2),
                                    g,
                                    &coefs_1[wstart..][..swb_size.into()],
                                    &scaled_2[wstart..][..swb_size.into()],
                                    sce.sf_idx[W(w)][g as usize] - 1,
                                    cb_1,
                                    1.,
                                    f32::INFINITY,
                                    0,
                                );
                            }
                            sce.sf_idx[W(w)][g as usize] -= 1;
                            dists[(w * 16 + g) as usize] =
                                cost_1.distortion - cost_1.bits as c_float;
                            qenergies[(w * 16 + g) as usize] = cost_1.energy;
                            if mb != 0
                                && (sce.sf_idx[W(w)][g as usize] < mindeltasf
                                    || dists[(w * 16 + g) as usize]
                                        < c_float::min(
                                            uplmax * uplims[(w * 16 + g) as usize],
                                            euplims[(w * 16 + g) as usize],
                                        )
                                        && (qenergies[(w * 16 + g) as usize]
                                            - energies[(w * 16 + g) as usize])
                                            .abs()
                                            < euplims[(w * 16 + g) as usize])
                            {
                                break;
                            }
                            i += 1;
                        }
                    } else if tbits > too_few_bits
                        && sce.sf_idx[W(w)][g as usize]
                            < (if maxdeltasf > maxsf[(w * 16 + g) as usize] {
                                maxsf[(w * 16 + g) as usize]
                            } else {
                                maxdeltasf
                            })
                        && dists[(w * 16 + g) as usize]
                            < (if euplims[(w * 16 + g) as usize] > uplims[(w * 16 + g) as usize] {
                                uplims[(w * 16 + g) as usize]
                            } else {
                                euplims[(w * 16 + g) as usize]
                            })
                        && (qenergies[(w * 16 + g) as usize] - energies[(w * 16 + g) as usize])
                            .abs()
                            < euplims[(w * 16 + g) as usize]
                    {
                        let mut i = 0;
                        while i < depth && sce.sf_idx[W(w)][g as usize] < maxdeltasf {
                            let mut cb_2: c_int = 0;
                            let mut cost_2 = QuantizationCost::default();
                            cb_2 = find_min_book(
                                maxvals[(w * 16 + g) as usize],
                                sce.sf_idx[W(w)][g as usize] + 1,
                            );
                            if cb_2 > 0 {
                                for w2 in 0..group_len {
                                    let wstart = usize::from(w2) * 128;
                                    cost_2 += s.quantize_band_cost_cache.quantize_band_cost_cached(
                                        w + c_int::from(w2),
                                        g,
                                        &coefs_1[wstart..][..swb_size.into()],
                                        &scaled_2[wstart..][..swb_size.into()],
                                        sce.sf_idx[W(w)][g as usize] + 1,
                                        cb_2,
                                        1.,
                                        f32::INFINITY,
                                        0,
                                    );
                                }
                                cost_2.distortion -= cost_2.bits as c_float;
                                if !(cost_2.distortion
                                    < uplims[(w * 16 + g) as usize]
                                        .min(euplims[(w * 16 + g) as usize]))
                                {
                                    break;
                                }
                                sce.sf_idx[W(w)][g as usize] += 1;
                                dists[(w * 16 + g) as usize] = cost_2.distortion;
                                qenergies[(w * 16 + g) as usize] = cost_2.energy;
                                i += 1;
                            } else {
                                maxsf[(w * 16 + g) as usize] =
                                    sce.sf_idx[W(w)][g as usize].min(maxsf[(w * 16 + g) as usize]);
                                break;
                            }
                        }
                    }
                    sce.sf_idx[W(w)][g as usize] =
                        sce.sf_idx[W(w)][g as usize].clamp(mindeltasf, maxdeltasf);
                    prev = sce.sf_idx[W(w)][g as usize];
                    if sce.sf_idx[W(w)][g as usize] != prevsc {
                        fflag = true;
                    }
                    nminscaler = if nminscaler > sce.sf_idx[W(w)][g as usize] {
                        sce.sf_idx[W(w)][g as usize]
                    } else {
                        nminscaler
                    };
                    sce.band_type[W(w)][g as usize] =
                        find_min_book(maxvals[(w * 16 + g) as usize], sce.sf_idx[W(w)][g as usize])
                            as BandType;
                }
                start += sce.ics.swb_sizes[g as usize] as c_int;
            }
        }
        prev = -1;
        for WindowedIteration { w, .. } in sce.ics.iter_windows() {
            for g in 0..sce.ics.num_swb {
                if !sce.zeroes[W(w)][g as usize] {
                    let mut prevsf: c_int = sce.sf_idx[W(w)][g as usize];
                    if prev < 0 {
                        prev = prevsf;
                    }
                    sce.sf_idx[W(w)][g as usize] =
                        sce.sf_idx[W(w)][g as usize].clamp(prev - 60, prev + 60);
                    sce.band_type[W(w)][g as usize] =
                        find_min_book(maxvals[(w * 16 + g) as usize], sce.sf_idx[W(w)][g as usize])
                            as BandType;
                    prev = sce.sf_idx[W(w)][g as usize];
                    if !fflag && prevsf != sce.sf_idx[W(w)][g as usize] {
                        fflag = true;
                    }
                }
            }
        }
        its += 1;
        if !(fflag && its < maxits) {
            break;
        }
    }

    find_next_bands(sce, maxvals);
}

#[ffmpeg_src(file = "libavcodec/aaccoder_twoloop.h", lines = 297..=312)]
fn calc_minsf_maxvals(
    ics: &IndividualChannelStream,
    scaled_coeffs: &[c_float; 1024], // TODO(yotam): make windowed
) -> ([c_int; 128], [c_float; 128]) {
    let mut minsf = [0; 128];
    let mut maxvals = [0.; 128];
    for WindowedIteration { w, group_len } in ics.iter_windows() {
        let start = w * 128;
        for (g, (swb_size, offset)) in ics.iter_swb_sizes_sum().enumerate() {
            let wstart = (w * 16) as usize + g;
            let maxval = &mut maxvals[wstart];
            let scaled = &scaled_coeffs[(start + c_int::from(offset)) as usize..];

            *maxval = (0..usize::from(group_len))
                .flat_map(|w2| &scaled[(w2 * 128)..][..swb_size.into()])
                .copied()
                .max_by(c_float::total_cmp)
                .unwrap_or_default();
            if *maxval <= 0. {
                continue;
            }

            let minsfidx = maxval.coef2minsf().into();
            for minsf in minsf[wstart..]
                .iter_mut()
                .step_by(16)
                .take(group_len.into())
            {
                *minsf = minsfidx;
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

    let mut euplims = *uplims;
    for WindowedIteration { w, group_len } in ics.iter_windows() {
        // psy already priorizes transients to some extent
        let de_psy_factor = if ics.num_windows == WindowCount::Eight {
            8. / c_float::from(group_len)
        } else {
            1.
        };

        for ((swb_size, offset), &nz, uplim, euplim) in izip!(
            ics.iter_swb_sizes_sum(),
            nzs,
            &mut uplims[(w * 16) as usize..],
            &mut euplims[(w * 16) as usize..],
        )
        .filter(|(_, &nz, ..)| nz > 0)
        {
            let start = w * 128 + c_int::from(offset);
            let coeffs = &coeffs[start as usize..];
            let cleanup_factor = (start as c_float / (cutoff as c_float * 0.75))
                .clamp(1., 2.)
                .powi(2);
            let mut energy2uplim = find_form_factor(
                group_len,
                swb_size,
                *uplim / (nz as c_int * ics.swb_sizes[w as usize] as c_int) as c_float,
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
                *uplim / (nz as c_int * ics.swb_sizes[w as usize] as c_int) as c_float,
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
    euplims
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

struct Loop1Result {
    uplims: [f32; 128],
    energies: [f32; 128],
    nzs: [i8; 128],
    spread_thr_r: [f32; 128],
    min_spread_thr_r: f32,
    max_spread_thr_r: f32,
    allz: AllZ,
}

impl Default for Loop1Result {
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
fn loop1(
    sce: &mut SingleChannelElement,
    s: &AACEncContext,
    cutoff: i32,
    zeroscale: f32,
) -> Loop1Result {
    let mut res = Loop1Result::default();
    let Loop1Result {
        uplims,
        energies,
        nzs,
        spread_thr_r,
        min_spread_thr_r,
        max_spread_thr_r,
        allz,
    } = &mut res;

    let SingleChannelElement {
        ics:
            ref ics @ IndividualChannelStream {
                mut num_swb,
                mut swb_sizes,
                ..
            },
        ref can_pns,
        zeroes,
        ..
    } = sce;

    let swb_sizes = &swb_sizes[..num_swb as usize];
    let FFPsyChannel { psy_bands, .. } = &s.psy.ch[s.cur_channel as usize];
    let zeroes = Cell::from_mut(&mut ***zeroes).as_array_of_cells();

    for WindowedIteration { w, group_len } in ics.iter_windows() {
        let mut start = 0;
        let mut g = start;
        while g < ics.num_swb {
            let wstart = (w * 16 + g) as usize;
            let mut uplim: c_float = 0.;
            let mut energy: c_float = 0.;
            let mut spread: c_float = 0.;
            let nz = zip(&psy_bands[W(w)][g as usize..], &zeroes[wstart..])
                .step_by(16)
                .take(group_len.into())
                .filter(|(band, zero)| {
                    if start >= cutoff
                        || band.energy <= band.threshold * zeroscale
                        || band.threshold == 0.
                    {
                        zero.set(true);
                        false
                    } else {
                        uplim += band.threshold;
                        energy += band.energy;
                        spread += band.spread;
                        true
                    }
                })
                .count();
            uplims[wstart] = uplim;
            energies[wstart] = energy;
            nzs[wstart] = nz as c_char;
            zeroes[wstart].set(nz == 0);
            *allz |= nz > 0;
            if nz > 0 && can_pns[W(w)][g as usize] {
                let spread_thr_r = &mut spread_thr_r[wstart];
                *spread_thr_r = energy * nz as c_float / (uplim * spread);
                (*min_spread_thr_r, *max_spread_thr_r) = if *min_spread_thr_r < 0. {
                    (*spread_thr_r, *spread_thr_r)
                } else {
                    (
                        min_spread_thr_r.min(*spread_thr_r),
                        max_spread_thr_r.max(*spread_thr_r),
                    )
                }
            }
            start += swb_sizes[g as usize] as c_int;
            g += 1;
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
    let mut nextband = sce.init_next_band_map();

    let SingleChannelElement {
        ics: ref ics @ IndividualChannelStream { mut num_swb, .. },
        zeroes,
        band_type,
        sf_idx: sf_indices,
        ..
    } = sce;
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
    maxvals: &[c_float; 128],
    maxsf: &[c_int; 128],
    minsf: &[c_int; 128],
    too_many_bits: c_int,
    too_few_bits: c_int,
    dists: &mut [c_float; 128],
    qenergies: &mut [c_float; 128],
) -> InnerLoopResult {
    let mut qstep: c_int = if its != 0 { 1 } else { 32 };
    loop {
        let mut prev: Option<c_int> = None;
        let mut tbits = 0;
        for WindowedIteration { w, group_len } in sce.ics.iter_windows() {
            let [coeffs, scaled] = [&sce.coeffs, &s.scaled_coeffs].map(|c| &c[W(w)]);
            let wstart = w as usize * 16;
            for (g, (&zero, &sf_idx, &can_pns, &maxval, dist, qenergy, (swb_size, offset))) in
                izip!(
                    &sce.zeroes[W(w)],
                    &sce.sf_idx[W(w)],
                    &sce.can_pns[W(w)],
                    &maxvals[wstart..],
                    &mut dists[wstart..],
                    &mut qenergies[wstart..],
                    sce.ics.iter_swb_sizes_sum(),
                )
                .enumerate()
            {
                let [coeffs, scaled] =
                    [coeffs, scaled].map(|coeffs| &coeffs[usize::from(offset)..]);
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
                let Sum::<QuantizationCost>(mut cost) = (0..group_len)
                    .map(|w2| {
                        let wstart = usize::from(w2) * 128;
                        s.quantize_band_cost_cache.quantize_band_cost_cached(
                            w + c_int::from(w2),
                            g as c_int,
                            &coeffs[wstart..][..swb_size.into()],
                            &scaled[wstart..][..swb_size.into()],
                            sf_idx,
                            cb,
                            1.,
                            f32::INFINITY,
                            0,
                        )
                    })
                    .reduce_with();
                *dist = cost.distortion - cost.bits as c_float;
                *qenergy = cost.energy;
                if let Some(prev) = prev {
                    let sfdiff = (sf_idx - prev + c_int::from(SCALE_DIFF_ZERO))
                        .clamp(0, 2 * c_int::from(SCALE_MAX_DIFF));
                    cost.bits += c_int::from(SCALEFACTOR_BITS[sfdiff as usize]);
                }
                tbits += cost.bits;
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
