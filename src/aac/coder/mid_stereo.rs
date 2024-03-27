use std::ptr;

use libc::{c_double, c_float, c_int, c_uchar, c_uint};

use super::{find_min_book, math::bval2bmax, quantize_band_cost, sfdelta_can_replace};
use crate::{
    aac::{
        encoder::{abs_pow34_v, ctx::AACEncContext},
        WindowedIteration,
    },
    common::av_clip_c,
    types::{BandType, ChannelElement, FFPsyBand, NOISE_BT, RESERVED_BT},
};

pub(crate) unsafe fn search_for_ms(mut s: *mut AACEncContext, mut cpe: *mut ChannelElement) {
    let mut start: c_int = 0;
    let mut i: c_int = 0;
    let mut w2: c_int = 0;
    let mut g: c_int = 0;
    let mut sid_sf_boost: c_int = 0;
    let mut prev_mid: c_int = 0;
    let mut prev_side: c_int = 0;

    let [M, S, L34, R34, M34, S34] =
        [0, 1, 2, 3, 4, 5].map(|i| ((*s).scoefs)[128 * i..].as_mut_ptr());

    let lambda = (*s).lambda;
    let mslambda = if 1. > lambda / 120. {
        lambda / 120.
    } else {
        1.
    };

    let [sce0, sce1] = &mut (*cpe).ch;

    if (*cpe).common_window == 0 {
        return;
    }
    let mut nextband0 = ptr::from_mut(sce0).init_nextband_map();
    let mut nextband1 = ptr::from_mut(sce1).init_nextband_map();
    prev_mid = sce0.sf_idx[0];
    prev_side = sce1.sf_idx[0];
    for WindowedIteration { w, group_len } in sce0.ics.iter_windows() {
        start = 0;
        g = 0;
        while g < sce0.ics.num_swb {
            let mut bmax: c_float =
                bval2bmax(g as c_float * 17. / sce0.ics.num_swb as c_float) / 0.0045;
            if (*cpe).is_mask[(w * 16 + g) as usize] == 0 {
                (*cpe).ms_mask[(w * 16 + g) as usize] = 0;
            }
            if !sce0.zeroes[(w * 16 + g) as usize]
                && !sce1.zeroes[(w * 16 + g) as usize]
                && (*cpe).is_mask[(w * 16 + g) as usize] == 0
            {
                let mut Mmax: c_float = 0.;
                let mut Smax: c_float = 0.;
                w2 = 0;
                while w2 < c_int::from(group_len) {
                    i = 0;
                    while i < sce0.ics.swb_sizes[g as usize] as c_int {
                        *M.offset(i as isize) = ((sce0.coeffs
                            [(start + (w + w2) * 128 + i) as usize]
                            + sce1.coeffs[(start + (w + w2) * 128 + i) as usize])
                            as c_double
                            * 0.5) as c_float;
                        *S.offset(i as isize) = *M.offset(i as isize)
                            - sce1.coeffs[(start + (w + w2) * 128 + i) as usize];
                        i += 1;
                        i;
                    }
                    abs_pow34_v(M34, M, sce0.ics.swb_sizes[g as usize] as c_int);
                    abs_pow34_v(S34, S, sce0.ics.swb_sizes[g as usize] as c_int);
                    i = 0;
                    while i < sce0.ics.swb_sizes[g as usize] as c_int {
                        Mmax = if Mmax > *M34.offset(i as isize) {
                            Mmax
                        } else {
                            *M34.offset(i as isize)
                        };
                        Smax = if Smax > *S34.offset(i as isize) {
                            Smax
                        } else {
                            *S34.offset(i as isize)
                        };
                        i += 1;
                        i;
                    }
                    w2 += 1;
                    w2;
                }
                sid_sf_boost = 0;
                while sid_sf_boost < 4 {
                    let mut dist1: c_float = 0.;
                    let mut dist2: c_float = 0.;
                    let mut B0: c_int = 0;
                    let mut B1: c_int = 0;
                    let mut minidx: c_int = 0;
                    let mut mididx: c_int = 0;
                    let mut sididx: c_int = 0;
                    let mut midcb: c_int = 0;
                    let mut sidcb: c_int = 0;
                    minidx = if sce0.sf_idx[(w * 16 + g) as usize]
                        > sce1.sf_idx[(w * 16 + g) as usize]
                    {
                        sce1.sf_idx[(w * 16 + g) as usize]
                    } else {
                        sce0.sf_idx[(w * 16 + g) as usize]
                    };
                    mididx = av_clip_c(minidx, 0, 255 - 36);
                    sididx = av_clip_c(minidx - sid_sf_boost * 3, 0, 255 - 36);
                    if !(sce0.band_type[(w * 16 + g) as usize] != NOISE_BT
                        && sce1.band_type[(w * 16 + g) as usize] != NOISE_BT
                        && (!sfdelta_can_replace(sce0, &nextband0, prev_mid, mididx, w * 16 + g)
                            || !sfdelta_can_replace(
                                sce1,
                                &nextband1,
                                prev_side,
                                sididx,
                                w * 16 + g,
                            )))
                    {
                        midcb = find_min_book(Mmax, mididx);
                        sidcb = find_min_book(Smax, sididx);
                        midcb = if 1 > midcb { 1 } else { midcb };
                        sidcb = if 1 > sidcb { 1 } else { sidcb };
                        w2 = 0;
                        while w2 < c_int::from(group_len) {
                            let mut band0: *mut FFPsyBand = &mut (*s).psy.ch
                                [((*s).cur_channel) as usize]
                                .psy_bands[((w + w2) * 16 + g) as usize]
                                as *mut FFPsyBand;
                            let mut band1: *mut FFPsyBand = &mut (*s).psy.ch
                                [((*s).cur_channel + 1) as usize]
                                .psy_bands[((w + w2) * 16 + g) as usize]
                                as *mut FFPsyBand;
                            let mut minthr: c_float = if (*band0).threshold > (*band1).threshold {
                                (*band1).threshold
                            } else {
                                (*band0).threshold
                            };
                            let mut b1: c_int = 0;
                            let mut b2: c_int = 0;
                            let mut b3: c_int = 0;
                            let mut b4: c_int = 0;
                            i = 0;
                            while i < sce0.ics.swb_sizes[g as usize] as c_int {
                                *M.offset(i as isize) =
                                    ((sce0.coeffs[(start + (w + w2) * 128 + i) as usize]
                                        + sce1.coeffs[(start + (w + w2) * 128 + i) as usize])
                                        as c_double
                                        * 0.5) as c_float;
                                *S.offset(i as isize) = *M.offset(i as isize)
                                    - sce1.coeffs[(start + (w + w2) * 128 + i) as usize];
                                i += 1;
                                i;
                            }
                            abs_pow34_v(
                                L34,
                                (sce0.coeffs)
                                    .as_mut_ptr()
                                    .offset(start as isize)
                                    .offset(((w + w2) * 128) as isize),
                                sce0.ics.swb_sizes[g as usize] as c_int,
                            );
                            abs_pow34_v(
                                R34,
                                (sce1.coeffs)
                                    .as_mut_ptr()
                                    .offset(start as isize)
                                    .offset(((w + w2) * 128) as isize),
                                sce0.ics.swb_sizes[g as usize] as c_int,
                            );
                            abs_pow34_v(M34, M, sce0.ics.swb_sizes[g as usize] as c_int);
                            abs_pow34_v(S34, S, sce0.ics.swb_sizes[g as usize] as c_int);
                            dist1 += quantize_band_cost(
                                s,
                                &mut *(sce0.coeffs)
                                    .as_mut_ptr()
                                    .offset((start + (w + w2) * 128) as isize),
                                L34,
                                sce0.ics.swb_sizes[g as usize] as c_int,
                                sce0.sf_idx[(w * 16 + g) as usize],
                                sce0.band_type[(w * 16 + g) as usize] as c_int,
                                lambda / ((*band0).threshold + 1.175_494_4e-38),
                                ::core::f32::INFINITY,
                                &mut b1,
                                ptr::null_mut::<c_float>(),
                            );
                            dist1 += quantize_band_cost(
                                s,
                                &mut *(sce1.coeffs)
                                    .as_mut_ptr()
                                    .offset((start + (w + w2) * 128) as isize),
                                R34,
                                sce1.ics.swb_sizes[g as usize] as c_int,
                                sce1.sf_idx[(w * 16 + g) as usize],
                                sce1.band_type[(w * 16 + g) as usize] as c_int,
                                lambda / ((*band1).threshold + 1.175_494_4e-38),
                                ::core::f32::INFINITY,
                                &mut b2,
                                ptr::null_mut::<c_float>(),
                            );
                            dist2 += quantize_band_cost(
                                s,
                                M,
                                M34,
                                sce0.ics.swb_sizes[g as usize] as c_int,
                                mididx,
                                midcb,
                                lambda / (minthr + 1.175_494_4e-38),
                                ::core::f32::INFINITY,
                                &mut b3,
                                ptr::null_mut::<c_float>(),
                            );
                            dist2 += quantize_band_cost(
                                s,
                                S,
                                S34,
                                sce1.ics.swb_sizes[g as usize] as c_int,
                                sididx,
                                sidcb,
                                mslambda / (minthr * bmax + 1.175_494_4e-38),
                                ::core::f32::INFINITY,
                                &mut b4,
                                ptr::null_mut::<c_float>(),
                            );
                            B0 += b1 + b2;
                            B1 += b3 + b4;
                            dist1 -= (b1 + b2) as c_float;
                            dist2 -= (b3 + b4) as c_float;
                            w2 += 1;
                            w2;
                        }
                        (*cpe).ms_mask[(w * 16 + g) as usize] =
                            (dist2 <= dist1 && B1 < B0) as c_int as c_uchar;
                        if (*cpe).ms_mask[(w * 16 + g) as usize] != 0 {
                            if sce0.band_type[(w * 16 + g) as usize] as c_uint
                                != NOISE_BT as c_int as c_uint
                                && sce1.band_type[(w * 16 + g) as usize] as c_uint
                                    != NOISE_BT as c_int as c_uint
                            {
                                sce0.sf_idx[(w * 16 + g) as usize] = mididx;
                                sce1.sf_idx[(w * 16 + g) as usize] = sididx;
                                sce0.band_type[(w * 16 + g) as usize] = midcb as BandType;
                                sce1.band_type[(w * 16 + g) as usize] = sidcb as BandType;
                            } else if (sce0.band_type[(w * 16 + g) as usize] as c_uint
                                != NOISE_BT as c_int as c_uint)
                                as c_int
                                ^ (sce1.band_type[(w * 16 + g) as usize] as c_uint
                                    != NOISE_BT as c_int as c_uint)
                                    as c_int
                                != 0
                            {
                                (*cpe).ms_mask[(w * 16 + g) as usize] = 0;
                            }
                            break;
                        } else if B1 > B0 {
                            break;
                        }
                    }
                    sid_sf_boost += 1;
                    sid_sf_boost;
                }
            }
            if !sce0.zeroes[(w * 16 + g) as usize]
                && (sce0.band_type[(w * 16 + g) as usize] as c_uint)
                    < RESERVED_BT as c_int as c_uint
            {
                prev_mid = sce0.sf_idx[(w * 16 + g) as usize];
            }
            if !sce1.zeroes[(w * 16 + g) as usize]
                && (*cpe).is_mask[(w * 16 + g) as usize] == 0
                && (sce1.band_type[(w * 16 + g) as usize] as c_uint)
                    < RESERVED_BT as c_int as c_uint
            {
                prev_side = sce1.sf_idx[(w * 16 + g) as usize];
            }
            start += sce0.ics.swb_sizes[g as usize] as c_int;
            g += 1;
            g;
        }
    }
}
