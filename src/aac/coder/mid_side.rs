use std::{iter, ptr};

use array_util::W;
use ffmpeg_src_macro::ffmpeg_src;
use libc::{c_double, c_float, c_int};

use super::{find_min_book, math::bval2bmax, quantize_band_cost, sfdelta_can_replace};
use crate::{
    aac::{
        encoder::{ctx::AACEncContext, pow::Pow34},
        IndividualChannelStream, WindowedIteration, SCALE_DIV_512, SCALE_MAX_POS,
    },
    types::{BandType, ChannelElement, NOISE_BT, RESERVED_BT},
};

#[ffmpeg_src(file = "libavcodec/aaccoder.c", lines = 978..=1117, name = "search_for_ms")]
pub(crate) unsafe fn search(mut s: *mut AACEncContext, mut cpe: *mut ChannelElement) {
    let mut start: c_int = 0;

    let ([M, S, L34, R34, M34, S34, ..], []) = (*s).scaled_coeffs.as_chunks_mut::<128>() else {
        unreachable!();
    };

    let lambda = (*s).lambda;
    let mslambda = f32::min(1., lambda / 120.);

    let [sce0, sce1] = &mut (*cpe).ch;

    if (*cpe).common_window == 0 {
        return;
    }

    // Scout out next nonzero bands
    let mut nextband0 = ptr::from_mut(sce0).init_nextband_map();
    let mut nextband1 = ptr::from_mut(sce1).init_nextband_map();

    let mut prev_mid = sce0.sf_idx[W(0)][0];
    let mut prev_side = sce1.sf_idx[W(0)][0];
    for WindowedIteration { w, group_len } in sce0.ics.iter_windows() {
        start = 0;
        for g in 0..sce0.ics.num_swb {
            let swb_size = sce0.ics.swb_sizes[g as usize];
            let bmax = bval2bmax(g as c_float * 17. / sce0.ics.num_swb as c_float) / 0.0045;
            if !(*cpe).is_mask[W(w)][g as usize] {
                (*cpe).ms_mask[W(w)][g as usize] = false;
            }
            if !sce0.zeroes[W(w)][g as usize]
                && !sce1.zeroes[W(w)][g as usize]
                && !(*cpe).is_mask[W(w)][g as usize]
            {
                let mut Mmax: c_float = 0.;
                let mut Smax: c_float = 0.;
                for w2 in 0..c_int::from(group_len) {
                    for i in 0..c_int::from(swb_size) {
                        M[i as usize] = ((sce0.coeffs[(start + (w + w2) * 128 + i) as usize]
                            + sce1.coeffs[(start + (w + w2) * 128 + i) as usize])
                            as c_double
                            * 0.5) as c_float;
                        S[i as usize] =
                            M[i as usize] - sce1.coeffs[(start + (w + w2) * 128 + i) as usize];
                    }
                    for (M34, M) in iter::zip(
                        &mut M34[..usize::from(swb_size)],
                        &M[..usize::from(swb_size)],
                    ) {
                        *M34 = M.abs_pow34();
                    }
                    for (S34, S) in iter::zip(
                        &mut S34[..usize::from(swb_size)],
                        &S[..usize::from(swb_size)],
                    ) {
                        *S34 = S.abs_pow34();
                    }
                    for i in 0..c_int::from(swb_size) {
                        Mmax = f32::max(Mmax, M34[i as usize]);
                        Smax = f32::max(Smax, S34[i as usize]);
                    }
                }
                for sid_sf_boost in 0..4 {
                    let mut dist1: c_float = 0.;
                    let mut dist2: c_float = 0.;
                    let mut B0: c_int = 0;
                    let mut B1: c_int = 0;
                    let minidx =
                        c_int::min(sce0.sf_idx[W(w)][g as usize], sce1.sf_idx[W(w)][g as usize]);
                    let mididx = minidx.clamp(0, (SCALE_MAX_POS - SCALE_DIV_512).into());
                    let sididx = (minidx - sid_sf_boost * 3)
                        .clamp(0, (SCALE_MAX_POS - SCALE_DIV_512).into());
                    if sce0.band_type[W(w)][g as usize] != NOISE_BT
                        && sce1.band_type[W(w)][g as usize] != NOISE_BT
                        && (!sfdelta_can_replace(sce0, &nextband0, prev_mid, mididx, w * 16 + g)
                            || !sfdelta_can_replace(
                                sce1,
                                &nextband1,
                                prev_side,
                                sididx,
                                w * 16 + g,
                            ))
                    {
                        continue;
                    }
                    let midcb = find_min_book(Mmax, mididx);
                    let sidcb = find_min_book(Smax, sididx);

                    // No CB can be zero
                    let midcb = midcb.max(1);
                    let sidcb = sidcb.max(1);

                    for w2 in 0..c_int::from(group_len) {
                        let band0 = &(*s).psy.ch[((*s).cur_channel) as usize].psy_bands
                            [((w + w2) * 16 + g) as usize];
                        let band1 = &(*s).psy.ch[((*s).cur_channel + 1) as usize].psy_bands
                            [((w + w2) * 16 + g) as usize];
                        let minthr = c_float::min(band0.threshold, band1.threshold);
                        let mut b1: c_int = 0;
                        let mut b2: c_int = 0;
                        let mut b3: c_int = 0;
                        let mut b4: c_int = 0;
                        for i in 0..c_int::from(swb_size) {
                            M[i as usize] = ((sce0.coeffs[(start + (w + w2) * 128 + i) as usize]
                                + sce1.coeffs[(start + (w + w2) * 128 + i) as usize])
                                as c_double
                                * 0.5) as c_float;
                            S[i as usize] =
                                M[i as usize] - sce1.coeffs[(start + (w + w2) * 128 + i) as usize];
                        }
                        for (L34, coeff) in iter::zip(
                            &mut L34[..usize::from(swb_size)],
                            &sce0.coeffs[(start + (w + w2) * 128) as usize..]
                                [..usize::from(swb_size)],
                        ) {
                            *L34 = coeff.abs_pow34();
                        }
                        for (R34, coeff) in iter::zip(
                            &mut R34[..usize::from(swb_size)],
                            &sce1.coeffs[(start + (w + w2) * 128) as usize..]
                                [..usize::from(swb_size)],
                        ) {
                            *R34 = coeff.abs_pow34();
                        }
                        for (M34, M) in iter::zip(
                            &mut M34[..usize::from(swb_size)],
                            &M[..usize::from(swb_size)],
                        ) {
                            *M34 = M.abs_pow34();
                        }
                        for (S34, S) in iter::zip(
                            &mut S34[..usize::from(swb_size)],
                            &S[..usize::from(swb_size)],
                        ) {
                            *S34 = S.abs_pow34();
                        }
                        dist1 += quantize_band_cost(
                            &sce0.coeffs[(start + (w + w2) * 128) as usize..][..swb_size.into()],
                            &L34[..swb_size.into()],
                            sce0.sf_idx[W(w)][g as usize],
                            sce0.band_type[W(w)][g as usize] as c_int,
                            lambda / (band0.threshold + 1.175_494_4e-38),
                            f32::INFINITY,
                            Some(&mut b1),
                            None,
                        );
                        dist1 += quantize_band_cost(
                            &sce1.coeffs[(start + (w + w2) * 128) as usize..]
                                [..sce1.ics.swb_sizes[g as usize].into()],
                            &R34[..sce1.ics.swb_sizes[g as usize].into()],
                            sce1.sf_idx[W(w)][g as usize],
                            sce1.band_type[W(w)][g as usize] as c_int,
                            lambda / (band1.threshold + 1.175_494_4e-38),
                            f32::INFINITY,
                            Some(&mut b2),
                            None,
                        );
                        dist2 += quantize_band_cost(
                            &M[..swb_size.into()],
                            &M34[..swb_size.into()],
                            mididx,
                            midcb,
                            lambda / (minthr + 1.175_494_4e-38),
                            f32::INFINITY,
                            Some(&mut b3),
                            None,
                        );
                        dist2 += quantize_band_cost(
                            &S[..sce1.ics.swb_sizes[g as usize].into()],
                            &S34[..sce1.ics.swb_sizes[g as usize].into()],
                            sididx,
                            sidcb,
                            mslambda / (minthr * bmax + 1.175_494_4e-38),
                            f32::INFINITY,
                            Some(&mut b4),
                            None,
                        );
                        B0 += b1 + b2;
                        B1 += b3 + b4;
                        dist1 -= (b1 + b2) as c_float;
                        dist2 -= (b3 + b4) as c_float;
                    }
                    (*cpe).ms_mask[W(w)][g as usize] = dist2 <= dist1 && B1 < B0;
                    if (*cpe).ms_mask[W(w)][g as usize] {
                        if sce0.band_type[W(w)][g as usize] != NOISE_BT
                            && sce1.band_type[W(w)][g as usize] != NOISE_BT
                        {
                            sce0.sf_idx[W(w)][g as usize] = mididx;
                            sce1.sf_idx[W(w)][g as usize] = sididx;
                            sce0.band_type[W(w)][g as usize] = midcb as BandType;
                            sce1.band_type[W(w)][g as usize] = sidcb as BandType;
                        } else if (sce0.band_type[W(w)][g as usize] != NOISE_BT)
                            ^ (sce1.band_type[W(w)][g as usize] != NOISE_BT)
                        {
                            // ms_mask unneeded, and it confuses some decoders
                            (*cpe).ms_mask[W(w)][g as usize] = false;
                        }
                        break;
                    } else if B1 > B0 {
                        // More boost won't fix this
                        break;
                    }
                }
            }
            if !sce0.zeroes[W(w)][g as usize] && sce0.band_type[W(w)][g as usize] < RESERVED_BT {
                prev_mid = sce0.sf_idx[W(w)][g as usize];
            }
            if !sce1.zeroes[W(w)][g as usize]
                && !(*cpe).is_mask[W(w)][g as usize]
                && sce1.band_type[W(w)][g as usize] < RESERVED_BT
            {
                prev_side = sce1.sf_idx[W(w)][g as usize];
            }
            start += c_int::from(swb_size);
        }
    }
}

#[ffmpeg_src(file = "libavcodec/aacenc.c", lines = 609..=639, name = "apply_mid_side_stereo")]
pub(crate) unsafe fn apply(mut cpe: *mut ChannelElement) {
    let ref ics @ IndividualChannelStream {
        num_swb, swb_sizes, ..
    } = (*cpe).ch[0].ics;

    if (*cpe).common_window == 0 {
        return;
    }

    for WindowedIteration { w, group_len } in ics.iter_windows() {
        for w2 in 0..c_int::from(group_len) {
            let mut start: c_int = (w + w2) * 128;
            for g in 0..num_swb {
                // ms_mask can be used for other purposes in PNS and I/S,
                // so must not apply M/S if any band uses either, even if
                // ms_mask is set.
                let swb_size = swb_sizes[g as usize];
                if !(*cpe).ms_mask[W(w)][g as usize]
                    || (*cpe).is_mask[W(w)][g as usize]
                    || (*cpe).ch[0].band_type[W(w)][g as usize] >= NOISE_BT
                    || (*cpe).ch[1].band_type[W(w)][g as usize] >= NOISE_BT
                {
                    start += c_int::from(swb_size);
                    continue;
                }

                let [L_coeffs, R_coeffs] = (*cpe)
                    .ch
                    .each_mut()
                    .map(|ch| &mut ch.coeffs[start as usize..][..swb_size.into()]);
                for (L, R) in iter::zip(L_coeffs, R_coeffs) {
                    *L = (*L + *R) * 0.5;
                    *R = *L - *R;
                }
                start += c_int::from(swb_size);
            }
        }
    }
}
