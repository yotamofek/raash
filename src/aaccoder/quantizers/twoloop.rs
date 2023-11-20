use std::mem::size_of;

use libc::{c_char, c_double, c_float, c_int, c_long, c_uchar, c_uint, c_ulong};

use crate::{
    aaccoder::{
        coef2minsf, cutoff_from_bitrate, ff_init_nextband_map, ff_pns_bits,
        ff_sfdelta_can_remove_band, find_form_factor, find_max_val, find_min_book,
        quantize_band_cost_cached,
    },
    aacenc::ff_quantize_band_cost_cache_init,
    aactab::ff_aac_scalefactor_bits,
    common::*,
    types::*,
};

pub(crate) unsafe extern "C" fn search(
    mut avctx: *mut AVCodecContext,
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
    lambda: c_float,
) {
    // TODO: is this safe?
    let mut s = &mut *s;
    let mut avctx = &mut *avctx;
    let mut sce = &mut *sce;

    let mut start: c_int = 0 as c_int;
    let mut i: c_int = 0;
    let mut w: c_int = 0;
    let mut w2: c_int = 0;
    let mut g: c_int = 0;
    let mut recomprd: c_int = 0;
    let mut destbits: c_int = (avctx.bit_rate as c_double * 1024.0f64
        / avctx.sample_rate as c_double
        / (if avctx.flags & (1 as c_int) << 1 as c_int != 0 {
            2.0f32
        } else {
            avctx.ch_layout.nb_channels as c_float
        }) as c_double
        * (lambda / 120.0f32) as c_double) as c_int;
    let mut refbits: c_int = destbits;
    let mut toomanybits: c_int = 0;
    let mut toofewbits: c_int = 0;
    let mut nzs: [c_char; 128] = [0; 128];
    let mut nextband: [c_uchar; 128] = [0; 128];
    let mut maxsf: [c_int; 128] = [0; 128];
    let mut minsf: [c_int; 128] = [0; 128];
    let mut dists: [c_float; 128] = [0.; 128];
    let mut qenergies: [c_float; 128] = [0.; 128];
    let mut uplims: [c_float; 128] = [0.; 128];
    let mut euplims: [c_float; 128] = [0.; 128];
    let mut energies: [c_float; 128] = [0.; 128];
    let mut maxvals: [c_float; 128] = [0.; 128];
    let mut spread_thr_r: [c_float; 128] = [0.; 128];
    let mut min_spread_thr_r: c_float = 0.;
    let mut max_spread_thr_r: c_float = 0.;
    let mut rdlambda: c_float = (2.0f32 * 120.0f32 / lambda).clamp(0.0625f32, 16.0f32);
    let nzslope: c_float = 1.5f32;
    let mut rdmin: c_float = 0.03125f32;
    let mut rdmax: c_float = 1.0f32;
    let mut sfoffs: c_float = ((120.0f32 / lambda).log2() * 4.0f32).clamp(-5., 10.);
    let mut fflag: c_int = 0;
    let mut minscaler: c_int = 0;
    let mut maxscaler: c_int = 0;
    let mut nminscaler: c_int = 0;
    let mut its: c_int = 0 as c_int;
    let mut maxits: c_int = 30 as c_int;
    let mut allz: c_int = 0 as c_int;
    let mut tbits: c_int = 0;
    let mut cutoff: c_int = 1024 as c_int;
    let mut pns_start_pos: c_int = 0;
    let mut prev: c_int = 0;
    let zeroscale = zeroscale(lambda);
    if s.psy.bitres.alloc >= 0 as c_int {
        destbits = (s.psy.bitres.alloc as c_float
            * (lambda
                / (if avctx.global_quality != 0 {
                    avctx.global_quality
                } else {
                    120 as c_int
                }) as c_float)) as c_int;
    }
    if avctx.flags & (1 as c_int) << 1 as c_int != 0 {
        if s.options.mid_side != 0 && s.cur_type == TYPE_CPE {
            destbits *= 2 as c_int;
        }
        toomanybits = 5800 as c_int;
        toofewbits = destbits / 16;
        sfoffs = (sce.ics.num_windows - 1 as c_int) as c_float;
        rdlambda = sqrtf(rdlambda);
        maxits *= 2 as c_int;
    } else {
        toomanybits = destbits + destbits / 8 as c_int;
        toofewbits = destbits - destbits / 8 as c_int;
        sfoffs = 0 as c_int as c_float;
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
    destbits = destbits.min(5800);
    toomanybits = toomanybits.min(5800);
    toofewbits = toofewbits.min(5800);
    min_spread_thr_r = -1.;
    max_spread_thr_r = -1.;
    w = 0 as c_int;
    while w < sce.ics.num_windows {
        start = 0 as c_int;
        g = start;
        while g < sce.ics.num_swb {
            let mut nz: c_int = 0 as c_int;
            let mut uplim: c_float = 0.0f32;
            let mut energy: c_float = 0.0f32;
            let mut spread: c_float = 0.0f32;
            w2 = 0 as c_int;
            while w2 < sce.ics.group_len[w as usize] as c_int {
                let mut band: *mut FFPsyBand =
                    &mut *((*(s.psy.ch).offset(s.cur_channel as isize)).psy_bands)
                        .as_mut_ptr()
                        .offset(((w + w2) * 16 + g) as isize) as *mut FFPsyBand;
                if start >= cutoff
                    || (*band).energy <= (*band).threshold * zeroscale
                    || (*band).threshold == 0.0f32
                {
                    sce.zeroes[((w + w2) * 16 + g) as usize] = 1;
                } else {
                    nz = 1;
                }
                w2 += 1;
                w2;
            }
            if nz == 0 {
                uplim = 0.0f32;
            } else {
                nz = 0;
                w2 = 0;
                while w2 < sce.ics.group_len[w as usize] as c_int {
                    let mut band_0: *mut FFPsyBand =
                        &mut *((*(s.psy.ch).offset(s.cur_channel as isize)).psy_bands)
                            .as_mut_ptr()
                            .offset(((w + w2) * 16 + g) as isize)
                            as *mut FFPsyBand;
                    if !((*band_0).energy <= (*band_0).threshold * zeroscale
                        || (*band_0).threshold == 0.0f32)
                    {
                        uplim += (*band_0).threshold;
                        energy += (*band_0).energy;
                        spread += (*band_0).spread;
                        nz += 1;
                        nz;
                    }
                    w2 += 1;
                    w2;
                }
            }
            uplims[(w * 16 + g) as usize] = uplim;
            energies[(w * 16 + g) as usize] = energy;
            nzs[(w * 16 + g) as usize] = nz as c_char;
            sce.zeroes[(w * 16 + g) as usize] = (nz == 0) as c_int as c_uchar;
            allz |= nz;
            if nz != 0 && sce.can_pns[(w * 16 + g) as usize] as c_int != 0 {
                spread_thr_r[(w * 16 + g) as usize] = energy * nz as c_float / (uplim * spread);
                if min_spread_thr_r < 0 as c_int as c_float {
                    max_spread_thr_r = spread_thr_r[(w * 16 + g) as usize];
                    min_spread_thr_r = max_spread_thr_r;
                } else {
                    min_spread_thr_r = if min_spread_thr_r > spread_thr_r[(w * 16 + g) as usize] {
                        spread_thr_r[(w * 16 + g) as usize]
                    } else {
                        min_spread_thr_r
                    };
                    max_spread_thr_r = if max_spread_thr_r > spread_thr_r[(w * 16 + g) as usize] {
                        max_spread_thr_r
                    } else {
                        spread_thr_r[(w * 16 + g) as usize]
                    };
                }
            }
            let fresh1 = g;
            g += 1;
            start += *(sce.ics.swb_sizes).offset(fresh1 as isize) as c_int;
        }
        w += sce.ics.group_len[w as usize] as c_int;
    }
    minscaler = 65535;
    w = 0;
    while w < sce.ics.num_windows {
        g = 0;
        while g < sce.ics.num_swb {
            if sce.zeroes[(w * 16 + g) as usize] != 0 {
                sce.sf_idx[(w * 16 + g) as usize] = 140;
            } else {
                sce.sf_idx[(w * 16 + g) as usize] = ((140.
                    + 1.75f64
                        * (uplims[(w * 16 + g) as usize].max(0.00125)
                            / *(sce.ics.swb_sizes).offset(g as isize) as c_int as c_float)
                            .log2() as c_double
                    + sfoffs as c_double)
                    as c_int)
                    .clamp(60, 255);
                minscaler = if minscaler > sce.sf_idx[(w * 16 + g) as usize] {
                    sce.sf_idx[(w * 16 + g) as usize]
                } else {
                    minscaler
                };
            }
            g += 1;
            g;
        }
        w += sce.ics.group_len[w as usize] as c_int;
    }
    minscaler = minscaler.clamp(140, 255);
    w = 0 as c_int;
    while w < sce.ics.num_windows {
        g = 0 as c_int;
        while g < sce.ics.num_swb {
            if sce.zeroes[(w * 16 + g) as usize] == 0 {
                sce.sf_idx[(w * 16 + g) as usize] =
                    sce.sf_idx[(w * 16 + g) as usize].clamp(minscaler, minscaler + 60 - 1);
            }
            g += 1;
            g;
        }
        w += sce.ics.group_len[w as usize] as c_int;
    }
    if allz == 0 {
        return;
    }
    (s.abs_pow34).expect("non-null function pointer")(
        (s.scoefs).as_mut_ptr(),
        (sce.coeffs).as_mut_ptr(),
        1024,
    );
    ff_quantize_band_cost_cache_init(s);
    i = 0 as c_int;
    while (i as c_ulong)
        < (size_of::<[c_int; 128]>() as c_ulong).wrapping_div(size_of::<c_int>() as c_ulong)
    {
        minsf[i as usize] = 0 as c_int;
        i += 1;
        i;
    }
    w = 0 as c_int;
    while w < sce.ics.num_windows {
        start = w * 128 as c_int;
        g = 0 as c_int;
        while g < sce.ics.num_swb {
            let mut scaled: *const c_float = (s.scoefs).as_mut_ptr().offset(start as isize);
            let mut minsfidx: c_int = 0;
            maxvals[(w * 16 + g) as usize] = find_max_val(
                sce.ics.group_len[w as usize] as c_int,
                *(sce.ics.swb_sizes).offset(g as isize) as c_int,
                scaled,
            );
            if maxvals[(w * 16 + g) as usize] > 0 as c_int as c_float {
                minsfidx = coef2minsf(maxvals[(w * 16 + g) as usize]) as c_int;
                w2 = 0 as c_int;
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
    euplims = uplims;
    w = 0 as c_int;
    while w < sce.ics.num_windows {
        let mut de_psy_factor: c_float = if sce.ics.num_windows > 1 as c_int {
            8.0f32 / sce.ics.group_len[w as usize] as c_int as c_float
        } else {
            1.0f32
        };
        start = w * 128 as c_int;
        g = 0 as c_int;
        while g < sce.ics.num_swb {
            if nzs[g as usize] as c_int > 0 as c_int {
                let mut cleanup_factor: c_float =
                    ((start as c_float / (cutoff as c_float * 0.75f32)).clamp(1., 2.)).powi(2);
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
                if avctx.flags & (1 as c_int) << 1 as c_int == 0 {
                    energy2uplim = sqrtf(energy2uplim);
                }
                energy2uplim = if 0.015625f32
                    > (if 1.0f32 > energy2uplim {
                        energy2uplim
                    } else {
                        1.0f32
                    }) {
                    0.015625f32
                } else if 1.0f32 > energy2uplim {
                    energy2uplim
                } else {
                    1.0f32
                };
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
                    2.0f32,
                );
                energy2uplim *= de_psy_factor;
                if avctx.flags & (1 as c_int) << 1 as c_int == 0 {
                    energy2uplim = sqrtf(energy2uplim);
                }
                energy2uplim = if 0.015625f32
                    > (if 1.0f32 > energy2uplim {
                        energy2uplim
                    } else {
                        1.0f32
                    }) {
                    0.015625f32
                } else if 1.0f32 > energy2uplim {
                    energy2uplim
                } else {
                    1.0f32
                };
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
    i = 0 as c_int;
    while (i as c_ulong)
        < (size_of::<[c_int; 128]>() as c_ulong).wrapping_div(size_of::<c_int>() as c_ulong)
    {
        maxsf[i as usize] = 255 as c_int;
        i += 1;
        i;
    }
    loop {
        let mut overdist: c_int = 0;
        let mut qstep: c_int = if its != 0 { 1 as c_int } else { 32 as c_int };
        loop {
            let mut changed: c_int = 0 as c_int;
            prev = -(1 as c_int);
            recomprd = 0 as c_int;
            tbits = 0 as c_int;
            w = 0 as c_int;
            while w < sce.ics.num_windows {
                start = w * 128 as c_int;
                g = 0 as c_int;
                while g < sce.ics.num_swb {
                    let mut coefs: *const c_float =
                        &mut *(sce.coeffs).as_mut_ptr().offset(start as isize) as *mut c_float;
                    let mut scaled_0: *const c_float =
                        &mut *(s.scoefs).as_mut_ptr().offset(start as isize) as *mut c_float;
                    let mut bits: c_int = 0 as c_int;
                    let mut cb: c_int = 0;
                    let mut dist: c_float = 0.0f32;
                    let mut qenergy: c_float = 0.0f32;
                    if sce.zeroes[(w * 16 + g) as usize] as c_int != 0
                        || sce.sf_idx[(w * 16 + g) as usize] >= 218 as c_int
                    {
                        start += *(sce.ics.swb_sizes).offset(g as isize) as c_int;
                        if sce.can_pns[(w * 16 + g) as usize] != 0 {
                            tbits += ff_pns_bits(sce, w, g);
                        }
                    } else {
                        cb = find_min_book(
                            maxvals[(w * 16 + g) as usize],
                            sce.sf_idx[(w * 16 + g) as usize],
                        );
                        w2 = 0 as c_int;
                        while w2 < sce.ics.group_len[w as usize] as c_int {
                            let mut b: c_int = 0;
                            let mut sqenergy: c_float = 0.;
                            dist += quantize_band_cost_cached(
                                s,
                                w + w2,
                                g,
                                coefs.offset((w2 * 128 as c_int) as isize),
                                scaled_0.offset((w2 * 128 as c_int) as isize),
                                *(sce.ics.swb_sizes).offset(g as isize) as c_int,
                                sce.sf_idx[(w * 16 + g) as usize],
                                cb,
                                1.0f32,
                                ::core::f32::INFINITY,
                                &mut b,
                                &mut sqenergy,
                                0 as c_int,
                            );
                            bits += b;
                            qenergy += sqenergy;
                            w2 += 1;
                            w2;
                        }
                        dists[(w * 16 + g) as usize] = dist - bits as c_float;
                        qenergies[(w * 16 + g) as usize] = qenergy;
                        if prev != -(1 as c_int) {
                            let mut sfdiff: c_int = (sce.sf_idx[(w * 16 + g) as usize] - prev
                                + 60 as c_int)
                                .clamp(0, 2 * 60);
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
            if tbits > toomanybits {
                recomprd = 1 as c_int;
                i = 0 as c_int;
                while i < 128 as c_int {
                    if sce.sf_idx[i as usize] < 255 as c_int - 36 as c_int {
                        let mut maxsf_i: c_int = if tbits > 5800 as c_int {
                            255 as c_int
                        } else {
                            maxsf[i as usize]
                        };
                        let mut new_sf: c_int = if maxsf_i > sce.sf_idx[i as usize] + qstep {
                            sce.sf_idx[i as usize] + qstep
                        } else {
                            maxsf_i
                        };
                        if new_sf != sce.sf_idx[i as usize] {
                            sce.sf_idx[i as usize] = new_sf;
                            changed = 1 as c_int;
                        }
                    }
                    i += 1;
                    i;
                }
            } else if tbits < toofewbits {
                recomprd = 1 as c_int;
                i = 0 as c_int;
                while i < 128 as c_int {
                    if sce.sf_idx[i as usize] > 140 as c_int {
                        let mut new_sf_0: c_int = if (if minsf[i as usize] > 140 as c_int {
                            minsf[i as usize]
                        } else {
                            140 as c_int
                        }) > sce.sf_idx[i as usize] - qstep
                        {
                            if minsf[i as usize] > 140 as c_int {
                                minsf[i as usize]
                            } else {
                                140 as c_int
                            }
                        } else {
                            sce.sf_idx[i as usize] - qstep
                        };
                        if new_sf_0 != sce.sf_idx[i as usize] {
                            sce.sf_idx[i as usize] = new_sf_0;
                            changed = 1 as c_int;
                        }
                    }
                    i += 1;
                    i;
                }
            }
            qstep >>= 1 as c_int;
            if qstep == 0
                && tbits > toomanybits
                && sce.sf_idx[0 as c_int as usize] < 217 as c_int
                && changed != 0
            {
                qstep = 1 as c_int;
            }
            if qstep == 0 {
                break;
            }
        }
        overdist = 1 as c_int;
        fflag = (tbits < toofewbits) as c_int;
        i = 0 as c_int;
        while i < 2 as c_int && (overdist != 0 || recomprd != 0) {
            if recomprd != 0 {
                prev = -(1 as c_int);
                tbits = 0 as c_int;
                w = 0 as c_int;
                while w < sce.ics.num_windows {
                    start = w * 128 as c_int;
                    g = 0 as c_int;
                    while g < sce.ics.num_swb {
                        let mut coefs_0: *const c_float =
                            (sce.coeffs).as_mut_ptr().offset(start as isize);
                        let mut scaled_1: *const c_float =
                            (s.scoefs).as_mut_ptr().offset(start as isize);
                        let mut bits_0: c_int = 0 as c_int;
                        let mut cb_0: c_int = 0;
                        let mut dist_0: c_float = 0.0f32;
                        let mut qenergy_0: c_float = 0.0f32;
                        if sce.zeroes[(w * 16 + g) as usize] as c_int != 0
                            || sce.sf_idx[(w * 16 + g) as usize] >= 218 as c_int
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
                            w2 = 0 as c_int;
                            while w2 < sce.ics.group_len[w as usize] as c_int {
                                let mut b_0: c_int = 0;
                                let mut sqenergy_0: c_float = 0.;
                                dist_0 += quantize_band_cost_cached(
                                    s,
                                    w + w2,
                                    g,
                                    coefs_0.offset((w2 * 128 as c_int) as isize),
                                    scaled_1.offset((w2 * 128 as c_int) as isize),
                                    *(sce.ics.swb_sizes).offset(g as isize) as c_int,
                                    sce.sf_idx[(w * 16 + g) as usize],
                                    cb_0,
                                    1.0f32,
                                    ::core::f32::INFINITY,
                                    &mut b_0,
                                    &mut sqenergy_0,
                                    0 as c_int,
                                );
                                bits_0 += b_0;
                                qenergy_0 += sqenergy_0;
                                w2 += 1;
                                w2;
                            }
                            dists[(w * 16 + g) as usize] = dist_0 - bits_0 as c_float;
                            qenergies[(w * 16 + g) as usize] = qenergy_0;
                            if prev != -(1 as c_int) {
                                let mut sfdiff_0: c_int = av_clip_c(
                                    sce.sf_idx[(w * 16 + g) as usize] - prev + 60 as c_int,
                                    0 as c_int,
                                    2 as c_int * 60 as c_int,
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
            if i == 0 && s.options.pns != 0 && its > maxits / 2 as c_int && tbits > toofewbits {
                let mut maxoverdist: c_float = 0.0f32;
                let mut ovrfactor: c_float =
                    1.0f32 + (maxits - its) as c_float * 16.0f32 / maxits as c_float;
                recomprd = 0 as c_int;
                overdist = recomprd;
                w = 0 as c_int;
                while w < sce.ics.num_windows {
                    start = 0 as c_int;
                    g = start;
                    while g < sce.ics.num_swb {
                        if sce.zeroes[(w * 16 + g) as usize] == 0
                            && sce.sf_idx[(w * 16 + g) as usize] > 140 as c_int
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
                    let mut zeroable: c_int = 0 as c_int;
                    let mut zeroed: c_int = 0 as c_int;
                    let mut maxzeroed: c_int = 0;
                    let mut zloop: c_int = 0;
                    w = 0 as c_int;
                    while w < sce.ics.num_windows {
                        start = 0 as c_int;
                        g = start;
                        while g < sce.ics.num_swb {
                            if start >= pns_start_pos
                                && sce.zeroes[(w * 16 + g) as usize] == 0
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
                    zspread = (maxspread - minspread) * 0.0125f32 + minspread;
                    zspread = if (if min_spread_thr_r * 8.0f32 > zspread {
                        zspread
                    } else {
                        min_spread_thr_r * 8.0f32
                    }) > ((toomanybits - tbits) as c_float * min_spread_thr_r
                        + (tbits - toofewbits) as c_float * max_spread_thr_r)
                        / (toomanybits - toofewbits + 1 as c_int) as c_float
                    {
                        ((toomanybits - tbits) as c_float * min_spread_thr_r
                            + (tbits - toofewbits) as c_float * max_spread_thr_r)
                            / (toomanybits - toofewbits + 1 as c_int) as c_float
                    } else if min_spread_thr_r * 8.0f32 > zspread {
                        zspread
                    } else {
                        min_spread_thr_r * 8.0f32
                    };
                    maxzeroed = if zeroable
                        > (if 1 as c_int
                            > (zeroable * its + maxits - 1 as c_int) / (2 as c_int * maxits)
                        {
                            1 as c_int
                        } else {
                            (zeroable * its + maxits - 1 as c_int) / (2 as c_int * maxits)
                        }) {
                        if 1 as c_int
                            > (zeroable * its + maxits - 1 as c_int) / (2 as c_int * maxits)
                        {
                            1 as c_int
                        } else {
                            (zeroable * its + maxits - 1 as c_int) / (2 as c_int * maxits)
                        }
                    } else {
                        zeroable
                    };
                    zloop = 0 as c_int;
                    while zloop < 2 as c_int {
                        let mut loopovrfactor: c_float =
                            if zloop != 0 { 1.0f32 } else { ovrfactor };
                        let mut loopminsf: c_int = if zloop != 0 {
                            140 as c_int - 36 as c_int
                        } else {
                            140 as c_int
                        };
                        let mut mcb: c_int = 0;
                        g = sce.ics.num_swb - 1 as c_int;
                        while g > 0 as c_int && zeroed < maxzeroed {
                            if (*(sce.ics.swb_offset).offset(g as isize) as c_int) >= pns_start_pos
                            {
                                w = 0 as c_int;
                                while w < sce.ics.num_windows {
                                    if sce.zeroes[(w * 16 + g) as usize] == 0
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
                                            || mcb <= 1 as c_int
                                                && dists[(w * 16 + g) as usize]
                                                    > (if uplims[(w * 16 + g) as usize]
                                                        > euplims[(w * 16 + g) as usize]
                                                    {
                                                        euplims[(w * 16 + g) as usize]
                                                    } else {
                                                        uplims[(w * 16 + g) as usize]
                                                    }))
                                    {
                                        sce.zeroes[(w * 16 + g) as usize] = 1 as c_int as c_uchar;
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
                        fflag = 1 as c_int;
                        recomprd = fflag;
                    }
                } else {
                    overdist = 0 as c_int;
                }
            }
            i += 1;
            i;
        }
        minscaler = 255 as c_int;
        maxscaler = 0 as c_int;
        w = 0 as c_int;
        while w < sce.ics.num_windows {
            g = 0 as c_int;
            while g < sce.ics.num_swb {
                if sce.zeroes[(w * 16 + g) as usize] == 0 {
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
        nminscaler = av_clip_c(
            minscaler,
            140 as c_int - 36 as c_int,
            255 as c_int - 36 as c_int,
        );
        minscaler = nminscaler;
        prev = -(1 as c_int);
        w = 0 as c_int;
        while w < sce.ics.num_windows {
            let mut depth: c_int = if its > maxits / 2 as c_int {
                if its > maxits * 2 as c_int / 3 as c_int {
                    1 as c_int
                } else {
                    3 as c_int
                }
            } else {
                10 as c_int
            };
            let mut edepth: c_int = depth + 2 as c_int;
            let mut uplmax: c_float = its as c_float / (maxits as c_float * 0.25f32) + 1.0f32;
            uplmax *= if tbits > destbits {
                if 2.0f32
                    > tbits as c_float
                        / (if 1 as c_int > destbits {
                            1 as c_int
                        } else {
                            destbits
                        }) as c_float
                {
                    tbits as c_float
                        / (if 1 as c_int > destbits {
                            1 as c_int
                        } else {
                            destbits
                        }) as c_float
                } else {
                    2.0f32
                }
            } else {
                1.0f32
            };
            start = w * 128 as c_int;
            g = 0 as c_int;
            while g < sce.ics.num_swb {
                let mut prevsc: c_int = sce.sf_idx[(w * 16 + g) as usize];
                if prev < 0 as c_int && sce.zeroes[(w * 16 + g) as usize] == 0 {
                    prev = sce.sf_idx[0 as c_int as usize];
                }
                if sce.zeroes[(w * 16 + g) as usize] == 0 {
                    let mut coefs_1: *const c_float =
                        (sce.coeffs).as_mut_ptr().offset(start as isize);
                    let mut scaled_2: *const c_float =
                        (s.scoefs).as_mut_ptr().offset(start as isize);
                    let mut cmb: c_int = find_min_book(
                        maxvals[(w * 16 + g) as usize],
                        sce.sf_idx[(w * 16 + g) as usize],
                    );
                    let mut mindeltasf: c_int = if 0 as c_int > prev - 60 as c_int {
                        0 as c_int
                    } else {
                        prev - 60 as c_int
                    };
                    let mut maxdeltasf: c_int = if 255 as c_int - 36 as c_int > prev + 60 as c_int {
                        prev + 60 as c_int
                    } else {
                        255 as c_int - 36 as c_int
                    };
                    if (cmb == 0 || dists[(w * 16 + g) as usize] > uplims[(w * 16 + g) as usize])
                        && sce.sf_idx[(w * 16 + g) as usize]
                            > (if mindeltasf > minsf[(w * 16 + g) as usize] {
                                mindeltasf
                            } else {
                                minsf[(w * 16 + g) as usize]
                            })
                    {
                        i = 0 as c_int;
                        while i < edepth && sce.sf_idx[(w * 16 + g) as usize] > mindeltasf {
                            let mut cb_1: c_int = 0;
                            let mut bits_1: c_int = 0;
                            let mut dist_1: c_float = 0.;
                            let mut qenergy_1: c_float = 0.;
                            let mut mb: c_int = find_min_book(
                                maxvals[(w * 16 + g) as usize],
                                sce.sf_idx[(w * 16 + g) as usize] - 1 as c_int,
                            );
                            cb_1 = find_min_book(
                                maxvals[(w * 16 + g) as usize],
                                sce.sf_idx[(w * 16 + g) as usize],
                            );
                            qenergy_1 = 0.0f32;
                            dist_1 = qenergy_1;
                            bits_1 = 0 as c_int;
                            if cb_1 == 0 {
                                maxsf[(w * 16 + g) as usize] = if sce.sf_idx[(w * 16 + g) as usize]
                                    - 1 as c_int
                                    > maxsf[(w * 16 + g) as usize]
                                {
                                    maxsf[(w * 16 + g) as usize]
                                } else {
                                    sce.sf_idx[(w * 16 + g) as usize] - 1 as c_int
                                };
                            } else if i >= depth
                                && dists[(w * 16 + g) as usize] < euplims[(w * 16 + g) as usize]
                            {
                                break;
                            }
                            if g == 0
                                && sce.ics.num_windows > 1 as c_int
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
                            w2 = 0 as c_int;
                            while w2 < sce.ics.group_len[w as usize] as c_int {
                                let mut b_1: c_int = 0;
                                let mut sqenergy_1: c_float = 0.;
                                dist_1 += quantize_band_cost_cached(
                                    s,
                                    w + w2,
                                    g,
                                    coefs_1.offset((w2 * 128 as c_int) as isize),
                                    scaled_2.offset((w2 * 128 as c_int) as isize),
                                    *(sce.ics.swb_sizes).offset(g as isize) as c_int,
                                    sce.sf_idx[(w * 16 + g) as usize] - 1 as c_int,
                                    cb_1,
                                    1.0f32,
                                    ::core::f32::INFINITY,
                                    &mut b_1,
                                    &mut sqenergy_1,
                                    0 as c_int,
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
                    } else if tbits > toofewbits
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
                        && fabsf(qenergies[(w * 16 + g) as usize] - energies[(w * 16 + g) as usize])
                            < euplims[(w * 16 + g) as usize]
                    {
                        i = 0 as c_int;
                        while i < depth && sce.sf_idx[(w * 16 + g) as usize] < maxdeltasf {
                            let mut cb_2: c_int = 0;
                            let mut bits_2: c_int = 0;
                            let mut dist_2: c_float = 0.;
                            let mut qenergy_2: c_float = 0.;
                            cb_2 = find_min_book(
                                maxvals[(w * 16 + g) as usize],
                                sce.sf_idx[(w * 16 + g) as usize] + 1 as c_int,
                            );
                            if cb_2 > 0 as c_int {
                                qenergy_2 = 0.0f32;
                                dist_2 = qenergy_2;
                                bits_2 = 0 as c_int;
                                w2 = 0 as c_int;
                                while w2 < sce.ics.group_len[w as usize] as c_int {
                                    let mut b_2: c_int = 0;
                                    let mut sqenergy_2: c_float = 0.;
                                    dist_2 += quantize_band_cost_cached(
                                        s,
                                        w + w2,
                                        g,
                                        coefs_1.offset((w2 * 128 as c_int) as isize),
                                        scaled_2.offset((w2 * 128 as c_int) as isize),
                                        *(sce.ics.swb_sizes).offset(g as isize) as c_int,
                                        sce.sf_idx[(w * 16 + g) as usize] + 1 as c_int,
                                        cb_2,
                                        1.0f32,
                                        ::core::f32::INFINITY,
                                        &mut b_2,
                                        &mut sqenergy_2,
                                        0 as c_int,
                                    );
                                    bits_2 += b_2;
                                    qenergy_2 += sqenergy_2;
                                    w2 += 1;
                                    w2;
                                }
                                dist_2 -= bits_2 as c_float;
                                if !(dist_2
                                    < (if euplims[(w * 16 + g) as usize]
                                        > uplims[(w * 16 + g) as usize]
                                    {
                                        uplims[(w * 16 + g) as usize]
                                    } else {
                                        euplims[(w * 16 + g) as usize]
                                    }))
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
                                maxsf[(w * 16 + g) as usize] = if sce.sf_idx[(w * 16 + g) as usize]
                                    > maxsf[(w * 16 + g) as usize]
                                {
                                    maxsf[(w * 16 + g) as usize]
                                } else {
                                    sce.sf_idx[(w * 16 + g) as usize]
                                };
                                break;
                            }
                        }
                    }
                    sce.sf_idx[(w * 16 + g) as usize] =
                        av_clip_c(sce.sf_idx[(w * 16 + g) as usize], mindeltasf, maxdeltasf);
                    prev = sce.sf_idx[(w * 16 + g) as usize];
                    if sce.sf_idx[(w * 16 + g) as usize] != prevsc {
                        fflag = 1 as c_int;
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
        prev = -(1 as c_int);
        w = 0 as c_int;
        while w < sce.ics.num_windows {
            g = 0 as c_int;
            while g < sce.ics.num_swb {
                if sce.zeroes[(w * 16 + g) as usize] == 0 {
                    let mut prevsf: c_int = sce.sf_idx[(w * 16 + g) as usize];
                    if prev < 0 as c_int {
                        prev = prevsf;
                    }
                    sce.sf_idx[(w * 16 + g) as usize] = av_clip_c(
                        sce.sf_idx[(w * 16 + g) as usize],
                        prev - 60 as c_int,
                        prev + 60 as c_int,
                    );
                    sce.band_type[(w * 16 + g) as usize] = find_min_book(
                        maxvals[(w * 16 + g) as usize],
                        sce.sf_idx[(w * 16 + g) as usize],
                    ) as BandType;
                    prev = sce.sf_idx[(w * 16 + g) as usize];
                    if fflag == 0 && prevsf != sce.sf_idx[(w * 16 + g) as usize] {
                        fflag = 1 as c_int;
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
    prev = -(1 as c_int);
    w = 0 as c_int;
    while w < sce.ics.num_windows {
        g = 0 as c_int;
        while g < sce.ics.num_swb {
            if sce.zeroes[(w * 16 + g) as usize] == 0 {
                sce.band_type[(w * 16 + g) as usize] = find_min_book(
                    maxvals[(w * 16 + g) as usize],
                    sce.sf_idx[(w * 16 + g) as usize],
                ) as BandType;
                if sce.band_type[(w * 16 + g) as usize] as c_uint <= 0 as c_int as c_uint {
                    if ff_sfdelta_can_remove_band(sce, nextband.as_mut_ptr(), prev, w * 16 + g) == 0
                    {
                        sce.band_type[(w * 16 + g) as usize] = 1 as BandType;
                    } else {
                        sce.zeroes[(w * 16 + g) as usize] = 1 as c_int as c_uchar;
                        sce.band_type[(w * 16 + g) as usize] = ZERO_BT;
                    }
                }
            } else {
                sce.band_type[(w * 16 + g) as usize] = ZERO_BT;
            }
            if sce.zeroes[(w * 16 + g) as usize] == 0 {
                if prev != -(1 as c_int) {
                    let mut _sfdiff_1: c_int =
                        sce.sf_idx[(w * 16 + g) as usize] - prev + 60 as c_int;
                } else if sce.zeroes[0 as c_int as usize] != 0 {
                    sce.sf_idx[0 as c_int as usize] = sce.sf_idx[(w * 16 + g) as usize];
                }
                prev = sce.sf_idx[(w * 16 + g) as usize];
            }
            g += 1;
            g;
        }
        w += sce.ics.group_len[w as usize] as c_int;
    }
}

fn zeroscale(lambda: f32) -> f32 {
    if lambda > 120.0f32 {
        (120.0f32 / lambda).powf(0.25f32).clamp(0.0625f32, 1.0f32)
    } else {
        1.0f32
    }
}

fn frame_bit_rate(
    avctx: &mut AVCodecContext,
    s: &mut AACEncContext,
    refbits: i32,
    rate_bandwidth_multiplier: f32,
) -> i32 {
    let mut frame_bit_rate: c_int = (if avctx.flags & (1 as c_int) << 1 as c_int != 0 {
        refbits as c_float * rate_bandwidth_multiplier * avctx.sample_rate as c_float
            / 1024 as c_int as c_float
    } else {
        (avctx.bit_rate / avctx.ch_layout.nb_channels as c_long) as c_float
    }) as c_int;
    if s.options.pns != 0 || s.options.intensity_stereo != 0 {
        frame_bit_rate = (frame_bit_rate as c_float * 1.15f32) as c_int;
    }
    frame_bit_rate
}
