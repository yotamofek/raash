use std::iter::zip;

use ffi::codec::AVCodecContext;
use ffmpeg_src_macro::ffmpeg_src;
use itertools::izip;
use libc::{c_char, c_double, c_float, c_int, c_long, c_uint, c_ushort};

use crate::{
    aac::{
        coder::{
            ff_pns_bits, find_form_factor, find_max_val, find_min_book, math::coef2minsf,
            quantize_band_cost_cached, sfdelta_can_remove_band,
        },
        encoder::{ctx::AACEncContext, pow::Pow34},
        psy_model::cutoff_from_bitrate,
        tables::ff_aac_scalefactor_bits,
        SyntaxElementType, WindowedIteration, SCALE_DIFF_ZERO, SCALE_DIV_512, SCALE_MAX_DIFF,
        SCALE_MAX_POS, SCALE_ONE_POS,
    },
    common::*,
    types::*,
};

#[ffmpeg_src(file = "libavcodec/aaccoder_twoloop.h", lines = 67..=761, name = "search_for_quantizers_twoloop")]
pub(crate) unsafe fn search(
    mut avctx: *mut AVCodecContext,
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
    lambda: c_float,
) {
    let mut start: c_int = 0;
    let mut i: c_int = 0;
    let mut g: c_int = 0;
    let mut recomprd: c_int = 0;
    let mut dest_bits: c_int = ((*avctx).bit_rate as c_double * 1024.
        / (*avctx).sample_rate as c_double
        / (if (*avctx).flags.qscale() {
            2.
        } else {
            (*avctx).ch_layout.nb_channels as c_float
        }) as c_double
        * (lambda / 120.) as c_double) as c_int;
    let mut refbits: c_int = dest_bits;
    let mut too_many_bits: c_int = 0;
    let mut too_few_bits: c_int = 0;
    let mut maxsf: [c_int; 128] = [0; 128];
    let mut minsf: [c_int; 128] = [0; 128];
    let mut dists: [c_float; 128] = [0.; 128];
    let mut qenergies: [c_float; 128] = [0.; 128];
    let mut euplims: [c_float; 128] = [0.; 128];
    let mut maxvals: [c_float; 128] = [0.; 128];
    let mut rdlambda: c_float = (2. * 120. / lambda).clamp(0.0625, 16.);
    let nzslope: c_float = 1.5;
    let mut rdmin: c_float = 0.03125;
    let mut rdmax: c_float = 1.;
    let mut sfoffs: c_float = ((120. / lambda).log2() * 4.).clamp(-5., 10.);
    let mut fflag: c_int = 0;
    let mut maxscaler: c_int = 0;
    let mut nminscaler: c_int = 0;
    let mut its: c_int = 0;
    let mut maxits: c_int = 30;
    let mut tbits: c_int = 0;
    let mut cutoff: c_int = 1024;
    let mut pns_start_pos: c_int = 0;
    let mut prev: c_int = 0;
    let zeroscale = zeroscale(lambda);

    if (*s).psy.bitres.alloc >= 0 {
        // Psy granted us extra bits to use, from the reservoire
        // adjust for lambda except what psy already did
        dest_bits = ((*s).psy.bitres.alloc as c_float
            * (lambda
                / (if (*avctx).global_quality != 0 {
                    (*avctx).global_quality
                } else {
                    120
                }) as c_float)) as c_int;
    }

    if (*avctx).flags.qscale() {
        // Constant Q-scale doesn't compensate MS coding on its own
        // No need to be overly precise, this only controls RD
        // adjustment CB limits when going overboard
        if (*s).options.mid_side != 0 && (*s).cur_type == SyntaxElementType::ChannelPairElement {
            dest_bits *= 2;
        }

        // When using a constant Q-scale, don't adjust bits, just use RD
        // Don't let it go overboard, though... 8x psy target is enough
        too_many_bits = 5800;
        too_few_bits = dest_bits / 16;

        // Don't offset scalers, just RD
        sfoffs = ((*sce).ics.num_windows - 1) as c_float;
        rdlambda = sqrtf(rdlambda);

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
        rdlambda = sqrtf(rdlambda);
    }
    let wlen: c_int = 1024 / (*sce).ics.num_windows;

    let frame_bit_rate = frame_bit_rate(&*avctx, &*s, refbits, 1.5);
    let bandwidth = if (*avctx).cutoff > 0 {
        (*avctx).cutoff
    } else {
        (*s).psy.cutoff = cutoff_from_bitrate(frame_bit_rate, 1, (*avctx).sample_rate).max(3000);
        (*s).psy.cutoff
    };
    (*s).psy.cutoff = bandwidth;

    cutoff = bandwidth * 2 * wlen / (*avctx).sample_rate;
    pns_start_pos = 4000 * 2 * wlen / (*avctx).sample_rate;

    // for values above this the decoder might end up in an endless loop
    // due to always having more bits than what can be encoded.
    dest_bits = dest_bits.min(5800);
    too_many_bits = too_many_bits.min(5800);
    too_few_bits = too_few_bits.min(5800);

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

    *(*s).scaled_coeffs = (*sce).coeffs.map(Pow34::abs_pow34);

    (*s).quantize_band_cost_cache.init();
    minsf.fill(0);

    for WindowedIteration { w, group_len } in (*sce).ics.iter_windows() {
        start = w * 128;
        for g in 0..(*sce).ics.num_swb {
            let mut scaled = (*s).scaled_coeffs[start as usize..].as_ptr();

            maxvals[(w * 16 + g) as usize] = find_max_val(
                c_int::from(group_len),
                (*sce).ics.swb_sizes[g as usize] as c_int,
                scaled,
            );

            if maxvals[(w * 16 + g) as usize] > 0. {
                let minsfidx = coef2minsf(maxvals[(w * 16 + g) as usize]) as c_int;
                for w2 in 0..c_int::from(group_len) {
                    minsf[((w + w2) * 16 + g) as usize] = minsfidx;
                }
            }
            start += (*sce).ics.swb_sizes[g as usize] as c_int;
        }
    }

    // Scale uplims to match rate distortion to quality
    // bu applying noisy band depriorization and tonal band priorization.
    // Maxval-energy ratio gives us an idea of how noisy/tonal the band is.
    // If maxval^2 ~ energy, then that band is mostly noise, and we can
    // relax rate distortion requirements.
    euplims = uplims;
    for WindowedIteration { w, group_len } in (*sce).ics.iter_windows() {
        // psy already priorizes transients to some extent
        let mut de_psy_factor: c_float = if (*sce).ics.num_windows > 1 {
            8. / c_float::from(group_len)
        } else {
            1.
        };
        start = w * 128;
        for g in 0..(*sce).ics.num_swb {
            if nzs[g as usize] as c_int > 0 {
                let mut cleanup_factor: c_float =
                    ((start as c_float / (cutoff as c_float * 0.75)).clamp(1., 2.)).powi(2);
                let mut energy2uplim: c_float = find_form_factor(
                    group_len,
                    (*sce).ics.swb_sizes[g as usize],
                    uplims[(w * 16 + g) as usize]
                        / (nzs[g as usize] as c_int * (*sce).ics.swb_sizes[w as usize] as c_int)
                            as c_float,
                    &(*sce).coeffs[start as usize..],
                    nzslope * cleanup_factor,
                );
                energy2uplim *= de_psy_factor;
                if !(*avctx).flags.qscale() {
                    // In ABR, we need to priorize less and let rate control do its thing
                    energy2uplim = sqrtf(energy2uplim);
                }
                energy2uplim = energy2uplim.clamp(0.015625, 1.);
                uplims[(w * 16 + g) as usize] *=
                    (rdlambda * energy2uplim).clamp(rdmin, rdmax) * c_float::from(group_len);

                energy2uplim = find_form_factor(
                    group_len,
                    (*sce).ics.swb_sizes[g as usize],
                    uplims[(w * 16 + g) as usize]
                        / (nzs[g as usize] as c_int * (*sce).ics.swb_sizes[w as usize] as c_int)
                            as c_float,
                    &(*sce).coeffs[start as usize..],
                    2.,
                );
                energy2uplim *= de_psy_factor;
                if !(*avctx).flags.qscale() {
                    // In ABR, we need to priorize less and let rate control do its thing
                    energy2uplim = energy2uplim.sqrt();
                }
                energy2uplim = energy2uplim.clamp(0.015625, 1.);
                euplims[(w * 16 + g) as usize] *=
                    (rdlambda * energy2uplim * c_float::from((*sce).ics.group_len[w as usize]))
                        .clamp(0.5, 1.);
            }
            start += (*sce).ics.swb_sizes[g as usize] as c_int;
        }
    }

    maxsf.fill(SCALE_MAX_POS.into());

    // perform two-loop search
    // outer loop - improve quality
    loop {
        //inner loop - quantize spectrum to fit into given number of bits
        let mut overdist: c_int = 0;
        let mut qstep: c_int = if its != 0 { 1 } else { 32 };
        loop {
            let mut changed: c_int = 0;
            prev = -1;
            recomprd = 0;
            tbits = 0;
            for WindowedIteration { w, group_len } in (*sce).ics.iter_windows() {
                start = w * 128;
                for g in 0..(*sce).ics.num_swb {
                    let mut coefs = &(*sce).coeffs[start as usize..];
                    let mut scaled_0 = &(*s).scaled_coeffs[start as usize..];
                    let mut bits: c_int = 0;
                    let mut cb: c_int = 0;
                    let mut dist: c_float = 0.;
                    let mut qenergy: c_float = 0.;
                    if (*sce).zeroes[(w * 16 + g) as usize] as c_int != 0
                        || (*sce).sf_idx[(w * 16 + g) as usize] >= 218
                    {
                        start += (*sce).ics.swb_sizes[g as usize] as c_int;
                        if (*sce).can_pns[(w * 16 + g) as usize] {
                            // PNS isn't free
                            tbits += ff_pns_bits(sce, w, g);
                        }
                    } else {
                        cb = find_min_book(
                            maxvals[(w * 16 + g) as usize],
                            (*sce).sf_idx[(w * 16 + g) as usize],
                        );
                        for w2 in 0..group_len {
                            let wstart = usize::from(w2) * 128;
                            let mut b: c_int = 0;
                            let mut sqenergy: c_float = 0.;
                            dist += quantize_band_cost_cached(
                                &mut (*s).quantize_band_cost_cache,
                                w + c_int::from(w2),
                                g,
                                &coefs[wstart..][..(*sce).ics.swb_sizes[g as usize].into()],
                                &scaled_0[wstart..][..(*sce).ics.swb_sizes[g as usize].into()],
                                (*sce).sf_idx[(w * 16 + g) as usize],
                                cb,
                                1.,
                                f32::INFINITY,
                                &mut b,
                                &mut sqenergy,
                                0,
                            );
                            bits += b;
                            qenergy += sqenergy;
                        }
                        dists[(w * 16 + g) as usize] = dist - bits as c_float;
                        qenergies[(w * 16 + g) as usize] = qenergy;
                        if prev != -1 {
                            let mut sfdiff = ((*sce).sf_idx[(w * 16 + g) as usize] - prev
                                + c_int::from(SCALE_DIFF_ZERO))
                            .clamp(0, 2 * c_int::from(SCALE_MAX_DIFF));
                            bits += ff_aac_scalefactor_bits[sfdiff as usize] as c_int;
                        }
                        tbits += bits;
                        start += (*sce).ics.swb_sizes[g as usize] as c_int;
                        prev = (*sce).sf_idx[(w * 16 + g) as usize];
                    }
                }
            }
            if tbits > too_many_bits {
                recomprd = 1;
                for (sf_idx, &maxsf) in zip(&mut (*sce).sf_idx, &maxsf)
                    .filter(|(&mut sf_idx, _)| sf_idx < c_int::from(SCALE_MAX_POS - SCALE_DIV_512))
                {
                    let mut maxsf_i = if tbits <= 5800 {
                        maxsf
                    } else {
                        SCALE_MAX_POS.into()
                    };
                    let mut new_sf = maxsf_i.min(*sf_idx + qstep);
                    if new_sf != *sf_idx {
                        *sf_idx = new_sf;
                        changed = 1;
                    }
                }
            } else if tbits < too_few_bits {
                recomprd = 1;
                for (sf_idx, &minsf) in zip(&mut (*sce).sf_idx, &minsf)
                    .filter(|(&mut sf_idx, _)| sf_idx > c_int::from(SCALE_ONE_POS))
                {
                    let new_sf = minsf.max(SCALE_ONE_POS.into()).max(*sf_idx - qstep);
                    if new_sf != *sf_idx {
                        *sf_idx = new_sf;
                        changed = 1;
                    }
                }
            }
            qstep >>= 1;
            if qstep == 0 && tbits > too_many_bits && (*sce).sf_idx[0] < 217 && changed != 0 {
                qstep = 1;
            }
            if qstep == 0 {
                break;
            }
        }

        overdist = 1;
        fflag = (tbits < too_few_bits) as c_int;
        i = 0;
        while i < 2 && (overdist != 0 || recomprd != 0) {
            if recomprd != 0 {
                prev = -1;
                tbits = 0;
                for WindowedIteration { w, group_len } in (*sce).ics.iter_windows() {
                    start = w * 128;
                    for g in 0..(*sce).ics.num_swb {
                        let coefs_0 = &(*sce).coeffs[start as usize..];
                        let scaled_1 = &(*s).scaled_coeffs[start as usize..];
                        let mut bits_0: c_int = 0;
                        let mut cb_0: c_int = 0;
                        let mut dist_0: c_float = 0.;
                        let mut qenergy_0: c_float = 0.;
                        if (*sce).zeroes[(w * 16 + g) as usize] as c_int != 0
                            || (*sce).sf_idx[(w * 16 + g) as usize] >= 218
                        {
                            start += (*sce).ics.swb_sizes[g as usize] as c_int;
                            if (*sce).can_pns[(w * 16 + g) as usize] {
                                tbits += ff_pns_bits(sce, w, g);
                            }
                        } else {
                            cb_0 = find_min_book(
                                maxvals[(w * 16 + g) as usize],
                                (*sce).sf_idx[(w * 16 + g) as usize],
                            );
                            for w2 in 0..group_len {
                                let wstart = usize::from(w2) * 128;
                                let mut b = 0;
                                let mut sqenergy = 0.;
                                dist_0 += quantize_band_cost_cached(
                                    &mut (*s).quantize_band_cost_cache,
                                    w + c_int::from(w2),
                                    g,
                                    &coefs_0[wstart..][..(*sce).ics.swb_sizes[g as usize].into()],
                                    &scaled_1[wstart..][..(*sce).ics.swb_sizes[g as usize].into()],
                                    (*sce).sf_idx[(w * 16 + g) as usize],
                                    cb_0,
                                    1.,
                                    f32::INFINITY,
                                    &mut b,
                                    &mut sqenergy,
                                    0,
                                );
                                bits_0 += b;
                                qenergy_0 += sqenergy;
                            }
                            dists[(w * 16 + g) as usize] = dist_0 - bits_0 as c_float;
                            qenergies[(w * 16 + g) as usize] = qenergy_0;
                            if prev != -1 {
                                let mut sfdiff_0: c_int = av_clip_c(
                                    (*sce).sf_idx[(w * 16 + g) as usize] - prev + 60,
                                    0,
                                    2 * 60,
                                );
                                bits_0 += ff_aac_scalefactor_bits[sfdiff_0 as usize] as c_int;
                            }
                            tbits += bits_0;
                            start += (*sce).ics.swb_sizes[g as usize] as c_int;
                            prev = (*sce).sf_idx[(w * 16 + g) as usize];
                        }
                    }
                }
            }
            if i == 0 && (*s).options.pns != 0 && its > maxits / 2 && tbits > too_few_bits {
                let mut maxoverdist: c_float = 0.;
                let mut ovrfactor: c_float =
                    1. + (maxits - its) as c_float * 16. / maxits as c_float;
                recomprd = 0;
                overdist = recomprd;
                for WindowedIteration { w, .. } in (*sce).ics.iter_windows() {
                    start = 0;
                    g = start;
                    while g < (*sce).ics.num_swb {
                        if !(*sce).zeroes[(w * 16 + g) as usize]
                            && (*sce).sf_idx[(w * 16 + g) as usize] > 140
                            && dists[(w * 16 + g) as usize]
                                > uplims[(w * 16 + g) as usize] * ovrfactor
                        {
                            let mut ovrdist: c_float = dists[(w * 16 + g) as usize]
                                / c_float::max(
                                    uplims[(w * 16 + g) as usize],
                                    euplims[(w * 16 + g) as usize],
                                );
                            maxoverdist = c_float::max(maxoverdist, ovrdist);
                            overdist += 1;
                        }
                        let fresh2 = g;
                        g += 1;
                        start += (*sce).ics.swb_sizes[fresh2 as usize] as c_int;
                    }
                }
                if overdist != 0 {
                    let mut minspread: c_float = max_spread_thr_r;
                    let mut maxspread: c_float = min_spread_thr_r;
                    let mut zspread: c_float = 0.;
                    let mut zeroable: c_int = 0;
                    let mut zeroed: c_int = 0;
                    let mut maxzeroed: c_int = 0;
                    for WindowedIteration { w, .. } in (*sce).ics.iter_windows() {
                        start = 0;
                        g = start;
                        while g < (*sce).ics.num_swb {
                            if start >= pns_start_pos
                                && !(*sce).zeroes[(w * 16 + g) as usize]
                                && (*sce).can_pns[(w * 16 + g) as usize] as c_int != 0
                            {
                                minspread = minspread.min(spread_thr_r[(w * 16 + g) as usize]);
                                maxspread = maxspread.max(spread_thr_r[(w * 16 + g) as usize]);
                                zeroable += 1;
                            }
                            let fresh3 = g;
                            g += 1;
                            start += (*sce).ics.swb_sizes[fresh3 as usize] as c_int;
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
                        let mut loopovrfactor: c_float = if zloop != 0 { 1. } else { ovrfactor };
                        let mut loopminsf = c_int::from(if zloop != 0 {
                            SCALE_ONE_POS - SCALE_DIV_512
                        } else {
                            SCALE_ONE_POS
                        });
                        let mut mcb: c_int = 0;
                        g = (*sce).ics.num_swb - 1;
                        while g > 0 && zeroed < maxzeroed {
                            if ((*sce).ics.swb_offset[g as usize] as c_int) >= pns_start_pos {
                                for WindowedIteration { w, .. } in (*sce).ics.iter_windows() {
                                    if !(*sce).zeroes[(w * 16 + g) as usize]
                                        && (*sce).can_pns[(w * 16 + g) as usize] as c_int != 0
                                        && spread_thr_r[(w * 16 + g) as usize] <= zspread
                                        && (*sce).sf_idx[(w * 16 + g) as usize] > loopminsf
                                        && (dists[(w * 16 + g) as usize]
                                            > loopovrfactor * uplims[(w * 16 + g) as usize]
                                            || {
                                                mcb = find_min_book(
                                                    maxvals[(w * 16 + g) as usize],
                                                    (*sce).sf_idx[(w * 16 + g) as usize],
                                                );
                                                mcb == 0
                                            }
                                            || mcb <= 1
                                                && dists[(w * 16 + g) as usize]
                                                    > c_float::min(
                                                        uplims[(w * 16 + g) as usize],
                                                        euplims[(w * 16 + g) as usize],
                                                    ))
                                    {
                                        (*sce).zeroes[(w * 16 + g) as usize] = true;
                                        (*sce).band_type[(w * 16 + g) as usize] = ZERO_BT;
                                        zeroed += 1;
                                    }
                                }
                            }
                            g -= 1;
                            g;
                        }
                    }
                    if zeroed != 0 {
                        fflag = 1;
                        recomprd = fflag;
                    }
                } else {
                    overdist = 0;
                }
            }
            i += 1;
            i;
        }
        let mut minscaler = 255;
        maxscaler = 0;
        for WindowedIteration { w, .. } in (*sce).ics.iter_windows() {
            for g in 0..(*sce).ics.num_swb {
                if !(*sce).zeroes[(w * 16 + g) as usize] {
                    minscaler = minscaler.min((*sce).sf_idx[(w * 16 + g) as usize]);
                    maxscaler = maxscaler.max((*sce).sf_idx[(w * 16 + g) as usize]);
                }
            }
        }
        nminscaler = av_clip_c(minscaler, 140 - 36, 255 - 36);
        minscaler = nminscaler;
        prev = -1;
        for WindowedIteration { w, group_len } in (*sce).ics.iter_windows() {
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
            start = w * 128;
            for g in 0..(*sce).ics.num_swb {
                let swb_size = (*sce).ics.swb_sizes[g as usize];
                let mut prevsc: c_int = (*sce).sf_idx[(w * 16 + g) as usize];
                if prev < 0 && !(*sce).zeroes[(w * 16 + g) as usize] {
                    prev = (*sce).sf_idx[0];
                }
                if !(*sce).zeroes[(w * 16 + g) as usize] {
                    let coefs_1 = &(*sce).coeffs[start as usize..];
                    let scaled_2 = &(*s).scaled_coeffs[start as usize..];
                    let mut cmb: c_int = find_min_book(
                        maxvals[(w * 16 + g) as usize],
                        (*sce).sf_idx[(w * 16 + g) as usize],
                    );
                    let mut mindeltasf = c_int::max(0, prev - c_int::from(SCALE_MAX_DIFF));
                    let mut maxdeltasf = c_int::min(
                        (SCALE_MAX_POS - SCALE_DIV_512).into(),
                        prev + c_int::from(SCALE_MAX_DIFF),
                    );
                    if (cmb == 0 || dists[(w * 16 + g) as usize] > uplims[(w * 16 + g) as usize])
                        && (*sce).sf_idx[(w * 16 + g) as usize]
                            > mindeltasf.max(minsf[(w * 16 + g) as usize])
                    {
                        // Try to make sure there is some energy in every nonzero band
                        // NOTE: This algorithm must be forcibly imbalanced, pushing harder
                        //  on holes or more distorted bands at first, otherwise there's
                        //  no net gain (since the next iteration will offset all bands
                        //  on the opposite direction to compensate for extra bits)
                        i = 0;
                        while i < edepth && (*sce).sf_idx[(w * 16 + g) as usize] > mindeltasf {
                            let mut cb_1: c_int = 0;
                            let mut bits_1: c_int = 0;
                            let mut dist_1: c_float = 0.;
                            let mut qenergy_1: c_float = 0.;
                            let mut mb: c_int = find_min_book(
                                maxvals[(w * 16 + g) as usize],
                                (*sce).sf_idx[(w * 16 + g) as usize] - 1,
                            );
                            cb_1 = find_min_book(
                                maxvals[(w * 16 + g) as usize],
                                (*sce).sf_idx[(w * 16 + g) as usize],
                            );
                            qenergy_1 = 0.;
                            dist_1 = qenergy_1;
                            bits_1 = 0;
                            if cb_1 == 0 {
                                maxsf[(w * 16 + g) as usize] = c_int::min(
                                    (*sce).sf_idx[(w * 16 + g) as usize] - 1,
                                    maxsf[(w * 16 + g) as usize],
                                );
                            } else if i >= depth
                                && dists[(w * 16 + g) as usize] < euplims[(w * 16 + g) as usize]
                            {
                                break;
                            }
                            if g == 0
                                && (*sce).ics.num_windows > 1
                                && dists[(w * 16 + g) as usize] >= euplims[(w * 16 + g) as usize]
                            {
                                maxsf[(w * 16 + g) as usize] = c_int::min(
                                    maxsf[(w * 16 + g) as usize],
                                    (*sce).sf_idx[(w * 16 + g) as usize],
                                );
                            }
                            for w2 in 0..group_len {
                                let wstart = usize::from(w2) * 128;
                                let mut b = 0;
                                let mut sqenergy = 0.;
                                dist_1 += quantize_band_cost_cached(
                                    &mut (*s).quantize_band_cost_cache,
                                    w + c_int::from(w2),
                                    g,
                                    &coefs_1[wstart..][..swb_size.into()],
                                    &scaled_2[wstart..][..swb_size.into()],
                                    (*sce).sf_idx[(w * 16 + g) as usize] - 1,
                                    cb_1,
                                    1.,
                                    f32::INFINITY,
                                    &mut b,
                                    &mut sqenergy,
                                    0,
                                );
                                bits_1 += b;
                                qenergy_1 += sqenergy;
                            }
                            (*sce).sf_idx[(w * 16 + g) as usize] -= 1;
                            (*sce).sf_idx[(w * 16 + g) as usize];
                            dists[(w * 16 + g) as usize] = dist_1 - bits_1 as c_float;
                            qenergies[(w * 16 + g) as usize] = qenergy_1;
                            if mb != 0
                                && ((*sce).sf_idx[(w * 16 + g) as usize] < mindeltasf
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
                            i;
                        }
                    } else if tbits > too_few_bits
                        && (*sce).sf_idx[(w * 16 + g) as usize]
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
                        i = 0;
                        while i < depth && (*sce).sf_idx[(w * 16 + g) as usize] < maxdeltasf {
                            let mut cb_2: c_int = 0;
                            let mut bits_2: c_int = 0;
                            let mut dist_2: c_float = 0.;
                            let mut qenergy_2: c_float = 0.;
                            cb_2 = find_min_book(
                                maxvals[(w * 16 + g) as usize],
                                (*sce).sf_idx[(w * 16 + g) as usize] + 1,
                            );
                            if cb_2 > 0 {
                                qenergy_2 = 0.;
                                dist_2 = qenergy_2;
                                bits_2 = 0;
                                for w2 in 0..group_len {
                                    let wstart = usize::from(w2) * 128;
                                    let mut b = 0;
                                    let mut sqenergy = 0.;
                                    dist_2 += quantize_band_cost_cached(
                                        &mut (*s).quantize_band_cost_cache,
                                        w + c_int::from(w2),
                                        g,
                                        &coefs_1[wstart..][..swb_size.into()],
                                        &scaled_2[wstart..][..swb_size.into()],
                                        (*sce).sf_idx[(w * 16 + g) as usize] + 1,
                                        cb_2,
                                        1.,
                                        f32::INFINITY,
                                        &mut b,
                                        &mut sqenergy,
                                        0,
                                    );
                                    bits_2 += b;
                                    qenergy_2 += sqenergy;
                                }
                                dist_2 -= bits_2 as c_float;
                                if !(dist_2
                                    < uplims[(w * 16 + g) as usize]
                                        .min(euplims[(w * 16 + g) as usize]))
                                {
                                    break;
                                }
                                (*sce).sf_idx[(w * 16 + g) as usize] += 1;
                                dists[(w * 16 + g) as usize] = dist_2;
                                qenergies[(w * 16 + g) as usize] = qenergy_2;
                                i += 1;
                                i;
                            } else {
                                maxsf[(w * 16 + g) as usize] = (*sce).sf_idx[(w * 16 + g) as usize]
                                    .min(maxsf[(w * 16 + g) as usize]);
                                break;
                            }
                        }
                    }
                    (*sce).sf_idx[(w * 16 + g) as usize] =
                        av_clip_c((*sce).sf_idx[(w * 16 + g) as usize], mindeltasf, maxdeltasf);
                    prev = (*sce).sf_idx[(w * 16 + g) as usize];
                    if (*sce).sf_idx[(w * 16 + g) as usize] != prevsc {
                        fflag = 1;
                    }
                    nminscaler = if nminscaler > (*sce).sf_idx[(w * 16 + g) as usize] {
                        (*sce).sf_idx[(w * 16 + g) as usize]
                    } else {
                        nminscaler
                    };
                    (*sce).band_type[(w * 16 + g) as usize] = find_min_book(
                        maxvals[(w * 16 + g) as usize],
                        (*sce).sf_idx[(w * 16 + g) as usize],
                    ) as BandType;
                }
                start += (*sce).ics.swb_sizes[g as usize] as c_int;
            }
        }
        prev = -1;
        for WindowedIteration { w, .. } in (*sce).ics.iter_windows() {
            for g in 0..(*sce).ics.num_swb {
                if !(*sce).zeroes[(w * 16 + g) as usize] {
                    let mut prevsf: c_int = (*sce).sf_idx[(w * 16 + g) as usize];
                    if prev < 0 {
                        prev = prevsf;
                    }
                    (*sce).sf_idx[(w * 16 + g) as usize] =
                        av_clip_c((*sce).sf_idx[(w * 16 + g) as usize], prev - 60, prev + 60);
                    (*sce).band_type[(w * 16 + g) as usize] = find_min_book(
                        maxvals[(w * 16 + g) as usize],
                        (*sce).sf_idx[(w * 16 + g) as usize],
                    ) as BandType;
                    prev = (*sce).sf_idx[(w * 16 + g) as usize];
                    if fflag == 0 && prevsf != (*sce).sf_idx[(w * 16 + g) as usize] {
                        fflag = 1;
                    }
                }
            }
        }
        its += 1;
        its;
        if !(fflag != 0 && its < maxits) {
            break;
        }
    }
    let mut nextband = sce.init_nextband_map();
    prev = -1;
    for WindowedIteration { w, .. } in (*sce).ics.iter_windows() {
        for g in 0..(*sce).ics.num_swb {
            if !(*sce).zeroes[(w * 16 + g) as usize] {
                (*sce).band_type[(w * 16 + g) as usize] = find_min_book(
                    maxvals[(w * 16 + g) as usize],
                    (*sce).sf_idx[(w * 16 + g) as usize],
                ) as BandType;
                if (*sce).band_type[(w * 16 + g) as usize] as c_uint <= 0 as c_uint {
                    if !sfdelta_can_remove_band(sce, &nextband, prev, w * 16 + g) {
                        (*sce).band_type[(w * 16 + g) as usize] = 1 as BandType;
                    } else {
                        (*sce).zeroes[(w * 16 + g) as usize] = true;
                        (*sce).band_type[(w * 16 + g) as usize] = ZERO_BT;
                    }
                }
            } else {
                (*sce).band_type[(w * 16 + g) as usize] = ZERO_BT;
            }
            if !(*sce).zeroes[(w * 16 + g) as usize] {
                if prev != -1 {
                    let mut _sfdiff_1: c_int = (*sce).sf_idx[(w * 16 + g) as usize] - prev + 60;
                } else if (*sce).zeroes[0] {
                    (*sce).sf_idx[0] = (*sce).sf_idx[(w * 16 + g) as usize];
                }
                prev = (*sce).sf_idx[(w * 16 + g) as usize];
            }
        }
    }
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
            &sce.zeroes[w as usize * 16..][..num_swb],
            &mut sce.sf_idx[w as usize * 16..][..num_swb],
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
    let mut minscaler = c_int::from(c_ushort::MAX);

    let swb_sizes = &sce.ics.swb_sizes[..sce.ics.num_swb as usize];

    for WindowedIteration { w, .. } in sce.ics.iter_windows() {
        for (&zero, sf_idx, &uplim, &swb_size) in izip!(
            &sce.zeroes[w as usize * 16..],
            &mut sce.sf_idx[w as usize * 16..],
            &uplims[w as usize * 16..],
            // (yotam): this one actually limits the rest to num_swb
            swb_sizes
        ) {
            if zero {
                *sf_idx = SCALE_ONE_POS.into();
                continue;
            }

            // log2f-to-distortion ratio is, technically, 2 (1.5db = 4, but
            // it's power vs level so it's 2). But, as offsets
            // are applied, low-frequency signals are too sensitive to the
            // induced distortion, so we make scaling more
            // conservative by choosing a lower log2f-to-distortion ratio,
            // and thus more robust.
            *sf_idx = ((c_double::from(SCALE_ONE_POS)
                + 1.75 * (uplim.max(0.00125) / c_float::from(swb_size)).log2() as c_double
                + sfoffs as c_double) as c_int)
                .clamp(60, SCALE_MAX_POS.into());
            minscaler = minscaler.min(*sf_idx as c_int);
        }
    }

    minscaler
}

#[derive(Default, PartialEq, Eq)]
enum AllZ {
    #[default]
    False,
    True,
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

    // TODO: is this safe?
    let swb_sizes = &sce.ics.swb_sizes[..sce.ics.num_swb as usize];
    let ch = &s.psy.ch[s.cur_channel as usize];

    let mut start;
    let mut g;
    for WindowedIteration { w, group_len } in sce.ics.iter_windows() {
        start = 0;
        g = start;
        while g < sce.ics.num_swb {
            let mut nz: c_int = 0;
            let mut uplim: c_float = 0.;
            let mut energy: c_float = 0.;
            let mut spread: c_float = 0.;
            for w2 in 0..c_int::from(group_len) {
                let band = ch.psy_bands[((w + w2) * 16 + g) as usize];
                if start >= cutoff
                    || band.energy <= band.threshold * zeroscale
                    || band.threshold == 0.
                {
                    sce.zeroes[((w + w2) * 16 + g) as usize] = true;
                } else {
                    nz = 1;
                }
            }
            if nz == 0 {
                uplim = 0.;
            } else {
                nz = ch.psy_bands[(w * 16 + g) as usize..]
                    .iter()
                    .step_by(16)
                    .take(sce.ics.group_len[w as usize] as usize)
                    .filter(|band| {
                        !(band.energy <= band.threshold * zeroscale || band.threshold == 0.)
                    })
                    .inspect(|band| {
                        uplim += band.threshold;
                        energy += band.energy;
                        spread += band.spread;
                    })
                    .count() as i32;
            }
            uplims[(w * 16 + g) as usize] = uplim;
            energies[(w * 16 + g) as usize] = energy;
            nzs[(w * 16 + g) as usize] = nz as c_char;
            sce.zeroes[(w * 16 + g) as usize] = nz == 0;
            if nz > 0 {
                *allz = AllZ::True
            }
            if nz != 0 && sce.can_pns[(w * 16 + g) as usize] as c_int != 0 {
                spread_thr_r[(w * 16 + g) as usize] = energy * nz as c_float / (uplim * spread);
                if *min_spread_thr_r < 0. {
                    *max_spread_thr_r = spread_thr_r[(w * 16 + g) as usize];
                    *min_spread_thr_r = *max_spread_thr_r;
                } else {
                    *min_spread_thr_r = min_spread_thr_r.min(spread_thr_r[(w * 16 + g) as usize]);
                    *max_spread_thr_r = max_spread_thr_r.max(spread_thr_r[(w * 16 + g) as usize]);
                }
            }
            let fresh1 = g;
            g += 1;
            start += swb_sizes[fresh1 as usize] as c_int;
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

#[ffmpeg_src(file = "libavcodec/aaccoder_twoloop.h", lines = 187..=193)]
/// Scale, psy gives us constant quality, this LP only scales
/// bitrate by lambda, so we save bits on subjectively unimportant HF
/// rather than increase quantization noise. Adjust nominal bitrate
/// to effective bitrate according to encoding parameters,
/// AAC_CUTOFF_FROM_BITRATE is calibrated for effective bitrate.
fn frame_bit_rate(
    avctx: &AVCodecContext,
    s: &AACEncContext,
    refbits: i32,
    rate_bandwidth_multiplier: f32,
) -> i32 {
    let mut frame_bit_rate = if avctx.flags.qscale() {
        refbits as c_float * rate_bandwidth_multiplier * avctx.sample_rate as c_float / 1024.
    } else {
        (avctx.bit_rate / avctx.ch_layout.nb_channels as c_long) as c_float
    };

    // Compensate for extensions that increase efficiency
    if s.options.pns != 0 || s.options.intensity_stereo != 0 {
        frame_bit_rate *= 1.15;
    }

    frame_bit_rate as c_int
}
