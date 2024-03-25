use std::{iter::zip, slice};

use ffi::codec::AVCodecContext;
use ffmpeg_src_macro::ffmpeg_src;
use itertools::izip;
use libc::{c_char, c_double, c_float, c_int, c_long, c_uchar, c_uint};

use crate::{
    aac::{
        coder::{
            ff_init_nextband_map, ff_pns_bits, find_form_factor, find_max_val, find_min_book,
            math::coef2minsf, quantize_band_cost_cached, sfdelta_can_remove_band,
        },
        encoder::{ctx::AACEncContext, ff_quantize_band_cost_cache_init, pow::Pow34},
        psy_model::cutoff_from_bitrate,
        tables::ff_aac_scalefactor_bits,
        SyntaxElementType, SCALE_DIFF_ZERO, SCALE_DIV_512, SCALE_MAX_DIFF, SCALE_MAX_POS,
        SCALE_ONE_POS,
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
    // TODO: is this safe?
    let mut s = &mut *s;
    let mut avctx = &mut *avctx;
    let mut sce = &mut *sce;

    let mut start: c_int = 0;
    let mut i: c_int = 0;
    let mut w: c_int = 0;
    let mut w2: c_int = 0;
    let mut g: c_int = 0;
    let mut recomprd: c_int = 0;
    let mut dest_bits: c_int = (avctx.bit_rate as c_double * 1024.
        / avctx.sample_rate as c_double
        / (if avctx.flags & AV_CODEC_FLAG_QSCALE != 0 {
            2.
        } else {
            avctx.ch_layout.nb_channels as c_float
        }) as c_double
        * (lambda / 120.) as c_double) as c_int;
    let mut refbits: c_int = dest_bits;
    let mut too_many_bits: c_int = 0;
    let mut too_few_bits: c_int = 0;
    let mut nextband: [c_uchar; 128] = [0; 128];
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
    if s.psy.bitres.alloc >= 0 {
        dest_bits = (s.psy.bitres.alloc as c_float
            * (lambda
                / (if avctx.global_quality != 0 {
                    avctx.global_quality
                } else {
                    120
                }) as c_float)) as c_int;
    }

    if avctx.flags & AV_CODEC_FLAG_QSCALE != 0 {
        if s.options.mid_side != 0 && s.cur_type == SyntaxElementType::ChannelPairElement {
            dest_bits *= 2;
        }
        too_many_bits = 5800;
        too_few_bits = dest_bits / 16;
        sfoffs = (sce.ics.num_windows - 1) as c_float;
        rdlambda = sqrtf(rdlambda);
        maxits *= 2;
    } else {
        too_many_bits = dest_bits + dest_bits / 8;
        too_few_bits = dest_bits - dest_bits / 8;
        sfoffs = 0.;
        rdlambda = sqrtf(rdlambda);
    }
    let wlen: c_int = 1024 / sce.ics.num_windows;

    let frame_bit_rate = frame_bit_rate(avctx, s, refbits, 1.5);
    let bandwidth = if avctx.cutoff > 0 {
        avctx.cutoff
    } else {
        s.psy.cutoff = cutoff_from_bitrate(frame_bit_rate, 1, avctx.sample_rate).max(3000);
        s.psy.cutoff
    };
    s.psy.cutoff = bandwidth;
    cutoff = bandwidth * 2 * wlen / avctx.sample_rate;
    pns_start_pos = 4000 * 2 * wlen / avctx.sample_rate;

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
    } = loop1(sce, s, cutoff, zeroscale);

    let minscaler = find_min_scaler(sce, &uplims, sfoffs);

    clip_non_zeros(sce, minscaler);

    if let AllZ::False = allz {
        return;
    }

    s.scoefs = sce.coeffs.map(Pow34::abs_pow34);

    ff_quantize_band_cost_cache_init(s);
    minsf.fill(0);

    w = 0;
    while w < sce.ics.num_windows {
        start = w * 128;
        g = 0;
        while g < sce.ics.num_swb {
            let mut scaled = s.scoefs[start as usize..].as_ptr();

            maxvals[(w * 16 + g) as usize] = find_max_val(
                sce.ics.group_len[w as usize] as c_int,
                *(sce.ics.swb_sizes).offset(g as isize) as c_int,
                scaled,
            );

            if maxvals[(w * 16 + g) as usize] > 0. {
                let minsfidx = coef2minsf(maxvals[(w * 16 + g) as usize]) as c_int;
                w2 = 0;
                while w2 < sce.ics.group_len[w as usize] as c_int {
                    minsf[((w + w2) * 16 + g) as usize] = minsfidx;
                    w2 += 1;
                    w2;
                }
            }
            start += *(sce.ics.swb_sizes).offset(g as isize) as c_int;
            g += 1;
            g;
        }
        w += sce.ics.group_len[w as usize] as c_int;
    }

    // Scale uplims to match rate distortion to quality
    // bu applying noisy band depriorization and tonal band priorization.
    // Maxval-energy ratio gives us an idea of how noisy/tonal the band is.
    // If maxval^2 ~ energy, then that band is mostly noise, and we can
    // relax rate distortion requirements.
    euplims = uplims;
    w = 0;
    while w < sce.ics.num_windows {
        let mut de_psy_factor: c_float = if sce.ics.num_windows > 1 {
            8. / sce.ics.group_len[w as usize] as c_int as c_float
        } else {
            1.
        };
        start = w * 128;
        g = 0;
        while g < sce.ics.num_swb {
            if nzs[g as usize] as c_int > 0 {
                let mut cleanup_factor: c_float =
                    ((start as c_float / (cutoff as c_float * 0.75)).clamp(1., 2.)).powi(2);
                let mut energy2uplim: c_float = find_form_factor(
                    sce.ics.group_len[w as usize] as c_int,
                    *(sce.ics.swb_sizes).offset(g as isize) as c_int,
                    uplims[(w * 16 + g) as usize]
                        / (nzs[g as usize] as c_int
                            * *(sce.ics.swb_sizes).offset(w as isize) as c_int)
                            as c_float,
                    (sce.coeffs).as_mut_ptr().offset(start as isize),
                    nzslope * cleanup_factor,
                );
                energy2uplim *= de_psy_factor;
                if avctx.flags & AV_CODEC_FLAG_QSCALE == 0 {
                    energy2uplim = sqrtf(energy2uplim);
                }
                energy2uplim = energy2uplim.clamp(0.015625, 1.);
                uplims[(w * 16 + g) as usize] *= (rdlambda * energy2uplim).clamp(rdmin, rdmax)
                    * sce.ics.group_len[w as usize] as c_int as c_float;
                energy2uplim = find_form_factor(
                    sce.ics.group_len[w as usize] as c_int,
                    *(sce.ics.swb_sizes).offset(g as isize) as c_int,
                    uplims[(w * 16 + g) as usize]
                        / (nzs[g as usize] as c_int
                            * *(sce.ics.swb_sizes).offset(w as isize) as c_int)
                            as c_float,
                    (sce.coeffs).as_mut_ptr().offset(start as isize),
                    2.,
                );
                energy2uplim *= de_psy_factor;
                if avctx.flags & AV_CODEC_FLAG_QSCALE == 0 {
                    energy2uplim = energy2uplim.sqrt();
                }
                energy2uplim = energy2uplim.clamp(0.015625, 1.);
                euplims[(w * 16 + g) as usize] *=
                    (rdlambda * energy2uplim * sce.ics.group_len[w as usize] as c_int as c_float)
                        .clamp(0.5, 1.);
            }
            start += *(sce.ics.swb_sizes).offset(g as isize) as c_int;
            g += 1;
            g;
        }
        w += sce.ics.group_len[w as usize] as c_int;
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
            w = 0;
            while w < sce.ics.num_windows {
                start = w * 128;
                g = 0;
                while g < sce.ics.num_swb {
                    let mut coefs: *const c_float =
                        &mut *(sce.coeffs).as_mut_ptr().offset(start as isize) as *mut c_float;
                    let mut scaled_0: *const c_float =
                        &mut *(s.scoefs).as_mut_ptr().offset(start as isize) as *mut c_float;
                    let mut bits: c_int = 0;
                    let mut cb: c_int = 0;
                    let mut dist: c_float = 0.;
                    let mut qenergy: c_float = 0.;
                    if sce.zeroes[(w * 16 + g) as usize] as c_int != 0
                        || sce.sf_idx[(w * 16 + g) as usize] >= 218
                    {
                        start += *(sce.ics.swb_sizes).offset(g as isize) as c_int;
                        if sce.can_pns[(w * 16 + g) as usize] != 0 {
                            // PNS isn't free
                            tbits += ff_pns_bits(sce, w, g);
                        }
                    } else {
                        cb = find_min_book(
                            maxvals[(w * 16 + g) as usize],
                            sce.sf_idx[(w * 16 + g) as usize],
                        );
                        w2 = 0;
                        while w2 < sce.ics.group_len[w as usize] as c_int {
                            let mut b: c_int = 0;
                            let mut sqenergy: c_float = 0.;
                            dist += quantize_band_cost_cached(
                                s,
                                w + w2,
                                g,
                                coefs.offset((w2 * 128) as isize),
                                scaled_0.offset((w2 * 128) as isize),
                                *(sce.ics.swb_sizes).offset(g as isize) as c_int,
                                sce.sf_idx[(w * 16 + g) as usize],
                                cb,
                                1.,
                                ::core::f32::INFINITY,
                                &mut b,
                                &mut sqenergy,
                                0,
                            );
                            bits += b;
                            qenergy += sqenergy;
                            w2 += 1;
                            w2;
                        }
                        dists[(w * 16 + g) as usize] = dist - bits as c_float;
                        qenergies[(w * 16 + g) as usize] = qenergy;
                        if prev != -1 {
                            let mut sfdiff = (sce.sf_idx[(w * 16 + g) as usize] - prev
                                + c_int::from(SCALE_DIFF_ZERO))
                            .clamp(0, 2 * c_int::from(SCALE_MAX_DIFF));
                            bits += ff_aac_scalefactor_bits[sfdiff as usize] as c_int;
                        }
                        tbits += bits;
                        start += *(sce.ics.swb_sizes).offset(g as isize) as c_int;
                        prev = sce.sf_idx[(w * 16 + g) as usize];
                    }
                    g += 1;
                    g;
                }
                w += sce.ics.group_len[w as usize] as c_int;
            }
            if tbits > too_many_bits {
                recomprd = 1;
                i = 0;
                while i < 128 {
                    if sce.sf_idx[i as usize] < 255 - 36 {
                        let mut maxsf_i: c_int = if tbits > 5800 { 255 } else { maxsf[i as usize] };
                        let mut new_sf: c_int = if maxsf_i > sce.sf_idx[i as usize] + qstep {
                            sce.sf_idx[i as usize] + qstep
                        } else {
                            maxsf_i
                        };
                        if new_sf != sce.sf_idx[i as usize] {
                            sce.sf_idx[i as usize] = new_sf;
                            changed = 1;
                        }
                    }
                    i += 1;
                    i;
                }
            } else if tbits < too_few_bits {
                recomprd = 1;
                i = 0;
                while i < 128 {
                    if sce.sf_idx[i as usize] > 140 {
                        let mut new_sf_0: c_int = if (if minsf[i as usize] > 140 {
                            minsf[i as usize]
                        } else {
                            140
                        }) > sce.sf_idx[i as usize] - qstep
                        {
                            if minsf[i as usize] > 140 {
                                minsf[i as usize]
                            } else {
                                140
                            }
                        } else {
                            sce.sf_idx[i as usize] - qstep
                        };
                        if new_sf_0 != sce.sf_idx[i as usize] {
                            sce.sf_idx[i as usize] = new_sf_0;
                            changed = 1;
                        }
                    }
                    i += 1;
                    i;
                }
            }
            qstep >>= 1;
            if qstep == 0 && tbits > too_many_bits && sce.sf_idx[0] < 217 && changed != 0 {
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
                w = 0;
                while w < sce.ics.num_windows {
                    start = w * 128;
                    g = 0;
                    while g < sce.ics.num_swb {
                        let mut coefs_0: *const c_float =
                            (sce.coeffs).as_mut_ptr().offset(start as isize);
                        let mut scaled_1: *const c_float =
                            (s.scoefs).as_mut_ptr().offset(start as isize);
                        let mut bits_0: c_int = 0;
                        let mut cb_0: c_int = 0;
                        let mut dist_0: c_float = 0.;
                        let mut qenergy_0: c_float = 0.;
                        if sce.zeroes[(w * 16 + g) as usize] as c_int != 0
                            || sce.sf_idx[(w * 16 + g) as usize] >= 218
                        {
                            start += *(sce.ics.swb_sizes).offset(g as isize) as c_int;
                            if sce.can_pns[(w * 16 + g) as usize] != 0 {
                                tbits += ff_pns_bits(sce, w, g);
                            }
                        } else {
                            cb_0 = find_min_book(
                                maxvals[(w * 16 + g) as usize],
                                sce.sf_idx[(w * 16 + g) as usize],
                            );
                            w2 = 0;
                            while w2 < sce.ics.group_len[w as usize] as c_int {
                                let mut b_0: c_int = 0;
                                let mut sqenergy_0: c_float = 0.;
                                dist_0 += quantize_band_cost_cached(
                                    s,
                                    w + w2,
                                    g,
                                    coefs_0.offset((w2 * 128) as isize),
                                    scaled_1.offset((w2 * 128) as isize),
                                    *(sce.ics.swb_sizes).offset(g as isize) as c_int,
                                    sce.sf_idx[(w * 16 + g) as usize],
                                    cb_0,
                                    1.,
                                    ::core::f32::INFINITY,
                                    &mut b_0,
                                    &mut sqenergy_0,
                                    0,
                                );
                                bits_0 += b_0;
                                qenergy_0 += sqenergy_0;
                                w2 += 1;
                                w2;
                            }
                            dists[(w * 16 + g) as usize] = dist_0 - bits_0 as c_float;
                            qenergies[(w * 16 + g) as usize] = qenergy_0;
                            if prev != -1 {
                                let mut sfdiff_0: c_int = av_clip_c(
                                    sce.sf_idx[(w * 16 + g) as usize] - prev + 60,
                                    0,
                                    2 * 60,
                                );
                                bits_0 += ff_aac_scalefactor_bits[sfdiff_0 as usize] as c_int;
                            }
                            tbits += bits_0;
                            start += *(sce.ics.swb_sizes).offset(g as isize) as c_int;
                            prev = sce.sf_idx[(w * 16 + g) as usize];
                        }
                        g += 1;
                        g;
                    }
                    w += sce.ics.group_len[w as usize] as c_int;
                }
            }
            if i == 0 && s.options.pns != 0 && its > maxits / 2 && tbits > too_few_bits {
                let mut maxoverdist: c_float = 0.;
                let mut ovrfactor: c_float =
                    1. + (maxits - its) as c_float * 16. / maxits as c_float;
                recomprd = 0;
                overdist = recomprd;
                w = 0;
                while w < sce.ics.num_windows {
                    start = 0;
                    g = start;
                    while g < sce.ics.num_swb {
                        if !sce.zeroes[(w * 16 + g) as usize]
                            && sce.sf_idx[(w * 16 + g) as usize] > 140
                            && dists[(w * 16 + g) as usize]
                                > uplims[(w * 16 + g) as usize] * ovrfactor
                        {
                            let mut ovrdist: c_float = dists[(w * 16 + g) as usize]
                                / (if uplims[(w * 16 + g) as usize] > euplims[(w * 16 + g) as usize]
                                {
                                    uplims[(w * 16 + g) as usize]
                                } else {
                                    euplims[(w * 16 + g) as usize]
                                });
                            maxoverdist = if maxoverdist > ovrdist {
                                maxoverdist
                            } else {
                                ovrdist
                            };
                            overdist += 1;
                            overdist;
                        }
                        let fresh2 = g;
                        g += 1;
                        start += *(sce.ics.swb_sizes).offset(fresh2 as isize) as c_int;
                    }
                    w += sce.ics.group_len[w as usize] as c_int;
                }
                if overdist != 0 {
                    let mut minspread: c_float = max_spread_thr_r;
                    let mut maxspread: c_float = min_spread_thr_r;
                    let mut zspread: c_float = 0.;
                    let mut zeroable: c_int = 0;
                    let mut zeroed: c_int = 0;
                    let mut maxzeroed: c_int = 0;
                    let mut zloop: c_int = 0;
                    w = 0;
                    while w < sce.ics.num_windows {
                        start = 0;
                        g = start;
                        while g < sce.ics.num_swb {
                            if start >= pns_start_pos
                                && !sce.zeroes[(w * 16 + g) as usize]
                                && sce.can_pns[(w * 16 + g) as usize] as c_int != 0
                            {
                                minspread = if minspread > spread_thr_r[(w * 16 + g) as usize] {
                                    spread_thr_r[(w * 16 + g) as usize]
                                } else {
                                    minspread
                                };
                                maxspread = if maxspread > spread_thr_r[(w * 16 + g) as usize] {
                                    maxspread
                                } else {
                                    spread_thr_r[(w * 16 + g) as usize]
                                };
                                zeroable += 1;
                                zeroable;
                            }
                            let fresh3 = g;
                            g += 1;
                            start += *(sce.ics.swb_sizes).offset(fresh3 as isize) as c_int;
                        }
                        w += sce.ics.group_len[w as usize] as c_int;
                    }
                    zspread = (maxspread - minspread) * 0.0125 + minspread;
                    zspread = if (if min_spread_thr_r * 8. > zspread {
                        zspread
                    } else {
                        min_spread_thr_r * 8.
                    }) > ((too_many_bits - tbits) as c_float * min_spread_thr_r
                        + (tbits - too_few_bits) as c_float * max_spread_thr_r)
                        / (too_many_bits - too_few_bits + 1) as c_float
                    {
                        ((too_many_bits - tbits) as c_float * min_spread_thr_r
                            + (tbits - too_few_bits) as c_float * max_spread_thr_r)
                            / (too_many_bits - too_few_bits + 1) as c_float
                    } else if min_spread_thr_r * 8. > zspread {
                        zspread
                    } else {
                        min_spread_thr_r * 8.
                    };
                    maxzeroed = if zeroable
                        > (if 1 > (zeroable * its + maxits - 1) / (2 * maxits) {
                            1
                        } else {
                            (zeroable * its + maxits - 1) / (2 * maxits)
                        }) {
                        if 1 > (zeroable * its + maxits - 1) / (2 * maxits) {
                            1
                        } else {
                            (zeroable * its + maxits - 1) / (2 * maxits)
                        }
                    } else {
                        zeroable
                    };
                    zloop = 0;
                    while zloop < 2 {
                        let mut loopovrfactor: c_float = if zloop != 0 { 1. } else { ovrfactor };
                        let mut loopminsf: c_int = if zloop != 0 { 140 - 36 } else { 140 };
                        let mut mcb: c_int = 0;
                        g = sce.ics.num_swb - 1;
                        while g > 0 && zeroed < maxzeroed {
                            if (*(sce.ics.swb_offset).offset(g as isize) as c_int) >= pns_start_pos
                            {
                                w = 0;
                                while w < sce.ics.num_windows {
                                    if !sce.zeroes[(w * 16 + g) as usize]
                                        && sce.can_pns[(w * 16 + g) as usize] as c_int != 0
                                        && spread_thr_r[(w * 16 + g) as usize] <= zspread
                                        && sce.sf_idx[(w * 16 + g) as usize] > loopminsf
                                        && (dists[(w * 16 + g) as usize]
                                            > loopovrfactor * uplims[(w * 16 + g) as usize]
                                            || {
                                                mcb = find_min_book(
                                                    maxvals[(w * 16 + g) as usize],
                                                    sce.sf_idx[(w * 16 + g) as usize],
                                                );
                                                mcb == 0
                                            }
                                            || mcb <= 1
                                                && dists[(w * 16 + g) as usize]
                                                    > (if uplims[(w * 16 + g) as usize]
                                                        > euplims[(w * 16 + g) as usize]
                                                    {
                                                        euplims[(w * 16 + g) as usize]
                                                    } else {
                                                        uplims[(w * 16 + g) as usize]
                                                    }))
                                    {
                                        sce.zeroes[(w * 16 + g) as usize] = true;
                                        sce.band_type[(w * 16 + g) as usize] = ZERO_BT;
                                        zeroed += 1;
                                        zeroed;
                                    }
                                    w += sce.ics.group_len[w as usize] as c_int;
                                }
                            }
                            g -= 1;
                            g;
                        }
                        zloop += 1;
                        zloop;
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
        w = 0;
        while w < sce.ics.num_windows {
            g = 0;
            while g < sce.ics.num_swb {
                if !sce.zeroes[(w * 16 + g) as usize] {
                    minscaler = if minscaler > sce.sf_idx[(w * 16 + g) as usize] {
                        sce.sf_idx[(w * 16 + g) as usize]
                    } else {
                        minscaler
                    };
                    maxscaler = if maxscaler > sce.sf_idx[(w * 16 + g) as usize] {
                        maxscaler
                    } else {
                        sce.sf_idx[(w * 16 + g) as usize]
                    };
                }
                g += 1;
                g;
            }
            w += sce.ics.group_len[w as usize] as c_int;
        }
        nminscaler = av_clip_c(minscaler, 140 - 36, 255 - 36);
        minscaler = nminscaler;
        prev = -1;
        w = 0;
        while w < sce.ics.num_windows {
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
            uplmax *= if tbits > dest_bits {
                if 2. > tbits as c_float / (if 1 > dest_bits { 1 } else { dest_bits }) as c_float {
                    tbits as c_float / (if 1 > dest_bits { 1 } else { dest_bits }) as c_float
                } else {
                    2.
                }
            } else {
                1.
            };
            start = w * 128;
            g = 0;
            while g < sce.ics.num_swb {
                let mut prevsc: c_int = sce.sf_idx[(w * 16 + g) as usize];
                if prev < 0 && !sce.zeroes[(w * 16 + g) as usize] {
                    prev = sce.sf_idx[0];
                }
                if !sce.zeroes[(w * 16 + g) as usize] {
                    let mut coefs_1: *const c_float =
                        (sce.coeffs).as_mut_ptr().offset(start as isize);
                    let mut scaled_2: *const c_float =
                        (s.scoefs).as_mut_ptr().offset(start as isize);
                    let mut cmb: c_int = find_min_book(
                        maxvals[(w * 16 + g) as usize],
                        sce.sf_idx[(w * 16 + g) as usize],
                    );
                    let mut mindeltasf: c_int = if 0 > prev - 60 { 0 } else { prev - 60 };
                    let mut maxdeltasf: c_int = if 255 - 36 > prev + 60 {
                        prev + 60
                    } else {
                        255 - 36
                    };
                    if (cmb == 0 || dists[(w * 16 + g) as usize] > uplims[(w * 16 + g) as usize])
                        && sce.sf_idx[(w * 16 + g) as usize]
                            > (if mindeltasf > minsf[(w * 16 + g) as usize] {
                                mindeltasf
                            } else {
                                minsf[(w * 16 + g) as usize]
                            })
                    {
                        i = 0;
                        while i < edepth && sce.sf_idx[(w * 16 + g) as usize] > mindeltasf {
                            let mut cb_1: c_int = 0;
                            let mut bits_1: c_int = 0;
                            let mut dist_1: c_float = 0.;
                            let mut qenergy_1: c_float = 0.;
                            let mut mb: c_int = find_min_book(
                                maxvals[(w * 16 + g) as usize],
                                sce.sf_idx[(w * 16 + g) as usize] - 1,
                            );
                            cb_1 = find_min_book(
                                maxvals[(w * 16 + g) as usize],
                                sce.sf_idx[(w * 16 + g) as usize],
                            );
                            qenergy_1 = 0.;
                            dist_1 = qenergy_1;
                            bits_1 = 0;
                            if cb_1 == 0 {
                                maxsf[(w * 16 + g) as usize] = if sce.sf_idx[(w * 16 + g) as usize]
                                    - 1
                                    > maxsf[(w * 16 + g) as usize]
                                {
                                    maxsf[(w * 16 + g) as usize]
                                } else {
                                    sce.sf_idx[(w * 16 + g) as usize] - 1
                                };
                            } else if i >= depth
                                && dists[(w * 16 + g) as usize] < euplims[(w * 16 + g) as usize]
                            {
                                break;
                            }
                            if g == 0
                                && sce.ics.num_windows > 1
                                && dists[(w * 16 + g) as usize] >= euplims[(w * 16 + g) as usize]
                            {
                                maxsf[(w * 16 + g) as usize] = if sce.sf_idx[(w * 16 + g) as usize]
                                    > maxsf[(w * 16 + g) as usize]
                                {
                                    maxsf[(w * 16 + g) as usize]
                                } else {
                                    sce.sf_idx[(w * 16 + g) as usize]
                                };
                            }
                            w2 = 0;
                            while w2 < sce.ics.group_len[w as usize] as c_int {
                                let mut b_1: c_int = 0;
                                let mut sqenergy_1: c_float = 0.;
                                dist_1 += quantize_band_cost_cached(
                                    s,
                                    w + w2,
                                    g,
                                    coefs_1.offset((w2 * 128) as isize),
                                    scaled_2.offset((w2 * 128) as isize),
                                    *(sce.ics.swb_sizes).offset(g as isize) as c_int,
                                    sce.sf_idx[(w * 16 + g) as usize] - 1,
                                    cb_1,
                                    1.,
                                    ::core::f32::INFINITY,
                                    &mut b_1,
                                    &mut sqenergy_1,
                                    0,
                                );
                                bits_1 += b_1;
                                qenergy_1 += sqenergy_1;
                                w2 += 1;
                                w2;
                            }
                            sce.sf_idx[(w * 16 + g) as usize] -= 1;
                            sce.sf_idx[(w * 16 + g) as usize];
                            dists[(w * 16 + g) as usize] = dist_1 - bits_1 as c_float;
                            qenergies[(w * 16 + g) as usize] = qenergy_1;
                            if mb != 0
                                && (sce.sf_idx[(w * 16 + g) as usize] < mindeltasf
                                    || dists[(w * 16 + g) as usize]
                                        < (if uplmax * uplims[(w * 16 + g) as usize]
                                            > euplims[(w * 16 + g) as usize]
                                        {
                                            euplims[(w * 16 + g) as usize]
                                        } else {
                                            uplmax * uplims[(w * 16 + g) as usize]
                                        })
                                        && fabsf(
                                            qenergies[(w * 16 + g) as usize]
                                                - energies[(w * 16 + g) as usize],
                                        ) < euplims[(w * 16 + g) as usize])
                            {
                                break;
                            }
                            i += 1;
                            i;
                        }
                    } else if tbits > too_few_bits
                        && sce.sf_idx[(w * 16 + g) as usize]
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
                        while i < depth && sce.sf_idx[(w * 16 + g) as usize] < maxdeltasf {
                            let mut cb_2: c_int = 0;
                            let mut bits_2: c_int = 0;
                            let mut dist_2: c_float = 0.;
                            let mut qenergy_2: c_float = 0.;
                            cb_2 = find_min_book(
                                maxvals[(w * 16 + g) as usize],
                                sce.sf_idx[(w * 16 + g) as usize] + 1,
                            );
                            if cb_2 > 0 {
                                qenergy_2 = 0.;
                                dist_2 = qenergy_2;
                                bits_2 = 0;
                                w2 = 0;
                                while w2 < sce.ics.group_len[w as usize] as c_int {
                                    let mut b_2: c_int = 0;
                                    let mut sqenergy_2: c_float = 0.;
                                    dist_2 += quantize_band_cost_cached(
                                        s,
                                        w + w2,
                                        g,
                                        coefs_1.offset((w2 * 128) as isize),
                                        scaled_2.offset((w2 * 128) as isize),
                                        *(sce.ics.swb_sizes).offset(g as isize) as c_int,
                                        sce.sf_idx[(w * 16 + g) as usize] + 1,
                                        cb_2,
                                        1.,
                                        ::core::f32::INFINITY,
                                        &mut b_2,
                                        &mut sqenergy_2,
                                        0,
                                    );
                                    bits_2 += b_2;
                                    qenergy_2 += sqenergy_2;
                                    w2 += 1;
                                    w2;
                                }
                                dist_2 -= bits_2 as c_float;
                                if !(dist_2
                                    < uplims[(w * 16 + g) as usize]
                                        .min(euplims[(w * 16 + g) as usize]))
                                {
                                    break;
                                }
                                sce.sf_idx[(w * 16 + g) as usize] += 1;
                                sce.sf_idx[(w * 16 + g) as usize];
                                dists[(w * 16 + g) as usize] = dist_2;
                                qenergies[(w * 16 + g) as usize] = qenergy_2;
                                i += 1;
                                i;
                            } else {
                                maxsf[(w * 16 + g) as usize] = sce.sf_idx[(w * 16 + g) as usize]
                                    .min(maxsf[(w * 16 + g) as usize]);
                                break;
                            }
                        }
                    }
                    sce.sf_idx[(w * 16 + g) as usize] =
                        av_clip_c(sce.sf_idx[(w * 16 + g) as usize], mindeltasf, maxdeltasf);
                    prev = sce.sf_idx[(w * 16 + g) as usize];
                    if sce.sf_idx[(w * 16 + g) as usize] != prevsc {
                        fflag = 1;
                    }
                    nminscaler = if nminscaler > sce.sf_idx[(w * 16 + g) as usize] {
                        sce.sf_idx[(w * 16 + g) as usize]
                    } else {
                        nminscaler
                    };
                    sce.band_type[(w * 16 + g) as usize] = find_min_book(
                        maxvals[(w * 16 + g) as usize],
                        sce.sf_idx[(w * 16 + g) as usize],
                    ) as BandType;
                }
                start += *(sce.ics.swb_sizes).offset(g as isize) as c_int;
                g += 1;
                g;
            }
            w += sce.ics.group_len[w as usize] as c_int;
        }
        prev = -1;
        w = 0;
        while w < sce.ics.num_windows {
            g = 0;
            while g < sce.ics.num_swb {
                if !sce.zeroes[(w * 16 + g) as usize] {
                    let mut prevsf: c_int = sce.sf_idx[(w * 16 + g) as usize];
                    if prev < 0 {
                        prev = prevsf;
                    }
                    sce.sf_idx[(w * 16 + g) as usize] =
                        av_clip_c(sce.sf_idx[(w * 16 + g) as usize], prev - 60, prev + 60);
                    sce.band_type[(w * 16 + g) as usize] = find_min_book(
                        maxvals[(w * 16 + g) as usize],
                        sce.sf_idx[(w * 16 + g) as usize],
                    ) as BandType;
                    prev = sce.sf_idx[(w * 16 + g) as usize];
                    if fflag == 0 && prevsf != sce.sf_idx[(w * 16 + g) as usize] {
                        fflag = 1;
                    }
                }
                g += 1;
                g;
            }
            w += sce.ics.group_len[w as usize] as c_int;
        }
        its += 1;
        its;
        if !(fflag != 0 && its < maxits) {
            break;
        }
    }
    ff_init_nextband_map(sce, nextband.as_mut_ptr());
    prev = -1;
    w = 0;
    while w < sce.ics.num_windows {
        g = 0;
        while g < sce.ics.num_swb {
            if !sce.zeroes[(w * 16 + g) as usize] {
                sce.band_type[(w * 16 + g) as usize] = find_min_book(
                    maxvals[(w * 16 + g) as usize],
                    sce.sf_idx[(w * 16 + g) as usize],
                ) as BandType;
                if sce.band_type[(w * 16 + g) as usize] as c_uint <= 0 as c_uint {
                    if !sfdelta_can_remove_band(sce, nextband.as_mut_ptr(), prev, w * 16 + g) {
                        sce.band_type[(w * 16 + g) as usize] = 1 as BandType;
                    } else {
                        sce.zeroes[(w * 16 + g) as usize] = true;
                        sce.band_type[(w * 16 + g) as usize] = ZERO_BT;
                    }
                }
            } else {
                sce.band_type[(w * 16 + g) as usize] = ZERO_BT;
            }
            if !sce.zeroes[(w * 16 + g) as usize] {
                if prev != -1 {
                    let mut _sfdiff_1: c_int = sce.sf_idx[(w * 16 + g) as usize] - prev + 60;
                } else if sce.zeroes[0] {
                    sce.sf_idx[0] = sce.sf_idx[(w * 16 + g) as usize];
                }
                prev = sce.sf_idx[(w * 16 + g) as usize];
            }
            g += 1;
            g;
        }
        w += sce.ics.group_len[w as usize] as c_int;
    }
}

#[ffmpeg_src(file = "libavcodec/aaccoder_twoloop.h", lines = 285..=290)]
fn clip_non_zeros(sce: &mut SingleChannelElement, minscaler: i32) {
    let minscaler = minscaler.clamp(
        (SCALE_ONE_POS - SCALE_DIV_512).into(),
        (SCALE_MAX_POS - SCALE_DIV_512).into(),
    );

    let mut w = 0;
    while w < sce.ics.num_windows as usize {
        let num_swb = sce.ics.num_swb as usize;

        for (_, sf_idx) in zip(
            &sce.zeroes[w * 16..][..num_swb],
            &mut sce.sf_idx[w * 16..][..num_swb],
        )
        .filter(|(&zero, _)| !zero)
        {
            *sf_idx = (*sf_idx).clamp(minscaler, minscaler + i32::from(SCALE_MAX_DIFF) - 1);
        }

        w += sce.ics.group_len[w] as usize;
    }
}

/// Compute initial scalers
#[ffmpeg_src(file = "libavcodec/aaccoder_twoloop.h", lines = 262..=283)]
fn find_min_scaler(
    sce: &mut SingleChannelElement,
    uplims: &[c_float; 128],
    sfoffs: c_float,
) -> c_int {
    let mut minscaler = 65535;
    let mut w = 0;

    // TODO: is this safe?
    let swb_sizes = unsafe { slice::from_raw_parts(sce.ics.swb_sizes, sce.ics.num_swb as usize) };

    while w < sce.ics.num_windows as usize {
        for (&zero, sf_idx, &uplim, &swb_size) in izip!(
            &sce.zeroes[w * 16..],
            &mut sce.sf_idx[w * 16..],
            &uplims[w * 16..],
            // (yotam): this one actually limits the rest to num_swb
            swb_sizes
        ) {
            if !zero {
                *sf_idx = SCALE_ONE_POS.into();
            } else {
                *sf_idx = ((c_double::from(SCALE_ONE_POS)
                    + 1.75 * (uplim.max(0.00125) / swb_size as c_float).log2() as c_double
                    + sfoffs as c_double) as c_int)
                    .clamp(60, SCALE_MAX_POS.into());
                minscaler = minscaler.min(*sf_idx as c_int);
            }
        }
        w += sce.ics.group_len[w] as usize;
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
    let swb_sizes = unsafe { slice::from_raw_parts(sce.ics.swb_sizes, sce.ics.num_swb as usize) };
    let ch = &s.psy.ch[s.cur_channel as usize];

    let mut w = 0;
    let mut start;
    let mut g;
    while w < sce.ics.num_windows {
        start = 0;
        g = start;
        while g < sce.ics.num_swb {
            let mut nz: c_int = 0;
            let mut uplim: c_float = 0.;
            let mut energy: c_float = 0.;
            let mut spread: c_float = 0.;
            let mut w2 = 0;
            while w2 < sce.ics.group_len[w as usize] as c_int {
                let band = ch.psy_bands[((w + w2) * 16 + g) as usize];
                if start >= cutoff
                    || band.energy <= band.threshold * zeroscale
                    || band.threshold == 0.
                {
                    sce.zeroes[((w + w2) * 16 + g) as usize] = true;
                } else {
                    nz = 1;
                }
                w2 += 1;
                w2;
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
        w += sce.ics.group_len[w as usize] as c_int;
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
fn frame_bit_rate(
    avctx: &AVCodecContext,
    s: &AACEncContext,
    refbits: i32,
    rate_bandwidth_multiplier: f32,
) -> i32 {
    let mut frame_bit_rate = if avctx.flags & AV_CODEC_FLAG_QSCALE != 0 {
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
