#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use std::{iter::zip, ops::Mul, ptr};

use ffi::codec::AVCodecContext;
use ffmpeg_src_macro::ffmpeg_src;
use itertools::izip;
use libc::{c_double, c_float, c_int, c_uint};
use reductor::{Reduce, Sum};

use super::pow::Pow34;
use crate::{
    aac::{
        coder::{quantize_and_encode_band::quantize_and_encode_band_cost, sfdelta_can_remove_band},
        encoder::ctx::AACEncContext,
        tables::POW_SF_TABLES,
        WindowedIteration,
    },
    common::*,
    types::*,
};

#[inline]
fn pos_pow34(mut a: c_float) -> c_float {
    sqrtf(a * sqrtf(a))
}

#[inline]
fn find_min_book(mut maxval: c_float, mut sf: c_int) -> c_int {
    let mut Q34: c_float = POW_SF_TABLES.pow34[(200 - sf + 140 - 36) as usize];
    let qmaxval = (maxval * Q34 + 0.405_4_f32) as usize;
    aac_maxval_cb.get(qmaxval).copied().unwrap_or(11) as c_int
}

#[inline]
unsafe fn quantize_band_cost(
    mut s: *mut AACEncContext,
    mut in_0: *const c_float,
    mut scaled: *const c_float,
    mut size: c_int,
    mut scale_idx: c_int,
    mut cb: c_int,
    lambda: c_float,
    uplim: c_float,
    mut bits: *mut c_int,
    mut energy: *mut c_float,
) -> c_float {
    quantize_and_encode_band_cost(
        s,
        ptr::null_mut::<PutBitContext>(),
        in_0,
        ptr::null_mut::<c_float>(),
        scaled,
        size,
        scale_idx,
        cb,
        lambda,
        uplim,
        bits,
        energy,
    )
}

#[derive(Clone, Copy)]
enum Phase {
    Positive,
    Negative,
}

impl Mul<c_float> for Phase {
    type Output = c_float;

    fn mul(self, rhs: c_float) -> Self::Output {
        rhs * match self {
            Phase::Positive => 1.,
            Phase::Negative => -1.,
        }
    }
}

#[derive(Copy, Clone)]
struct AACISError {
    phase: Phase,
    error: c_float,
    ener01: c_float,
}

#[ffmpeg_src(file = "libavcodec/aacenc_is.c", lines = 33..=96, name = "ff_aac_is_encoding_err")]
unsafe fn encoding_err(
    mut s: *mut AACEncContext,
    mut cpe: *mut ChannelElement,
    start: c_int,
    mut w: c_int,
    mut g: c_int,
    mut ener0: c_float,
    mut ener1: c_float,
    mut ener01: c_float,
    use_pcoeffs: bool,
    mut phase: Phase,
) -> Option<AACISError> {
    let [sce0, sce1] = &mut ((*cpe).ch);
    let swb_size = usize::from(sce0.ics.swb_sizes[g as usize]);

    let mut L = if use_pcoeffs {
        &*sce0.pcoeffs
    } else {
        &sce0.coeffs
    };
    let mut R = if use_pcoeffs {
        &*sce1.pcoeffs
    } else {
        &sce1.coeffs
    };

    let ([L34, R34, IS, I34, ..], []) = (*s).scoefs.as_chunks_mut::<256>() else {
        unreachable!();
    };

    let mut dist1: c_float = 0.;
    let mut dist2: c_float = 0.;

    if ener01 <= 0. || ener0 <= 0. {
        return None;
    }

    for w2 in 0..c_int::from(sce0.ics.group_len[w as usize]) {
        let band0 = &(*s).psy.ch[(*s).cur_channel as usize].psy_bands[((w + w2) * 16 + g) as usize];
        let band1 =
            &(*s).psy.ch[((*s).cur_channel + 1) as usize].psy_bands[((w + w2) * 16 + g) as usize];
        let mut is_band_type: c_int = 0;
        let mut is_sf_idx: c_int = 1.max(sce0.sf_idx[(w * 16 + g) as usize] - 4);
        let mut e01_34: c_float = phase * pos_pow34(ener1 / ener0);
        let mut minthr = c_float::min(band0.threshold, band1.threshold);
        let wstart = (start + (w + w2) * 128) as usize;
        for (IS, &L, &R) in izip!(
            &mut IS[..swb_size],
            &L[wstart..][..swb_size],
            &R[wstart..][..swb_size],
        ) {
            *IS = ((L + phase * R) as c_double * sqrt((ener0 / ener01) as c_double)) as c_float;
        }
        for (C34, C) in [(&mut *L34, &L[wstart..]), (R34, &R[wstart..]), (I34, IS)] {
            for (C34, C) in zip(&mut C34[..swb_size], &C[..swb_size]) {
                *C34 = C.abs_pow34();
            }
        }
        let maxval = I34[..swb_size]
            .iter()
            .copied()
            .max_by(c_float::total_cmp)
            .unwrap();
        is_band_type = find_min_book(maxval, is_sf_idx);
        dist1 += quantize_band_cost(
            s,
            L[wstart..].as_ptr(),
            L34.as_ptr(),
            swb_size as c_int,
            sce0.sf_idx[(w * 16 + g) as usize],
            sce0.band_type[(w * 16 + g) as usize] as c_int,
            (*s).lambda / band0.threshold,
            f32::INFINITY,
            ptr::null_mut(),
            ptr::null_mut(),
        );
        dist1 += quantize_band_cost(
            s,
            R[wstart..].as_ptr(),
            R34.as_ptr(),
            sce1.ics.swb_sizes[g as usize] as c_int,
            sce1.sf_idx[(w * 16 + g) as usize],
            sce1.band_type[(w * 16 + g) as usize] as c_int,
            (*s).lambda / band1.threshold,
            f32::INFINITY,
            ptr::null_mut(),
            ptr::null_mut(),
        );
        dist2 += quantize_band_cost(
            s,
            IS.as_ptr(),
            I34.as_ptr(),
            swb_size as c_int,
            is_sf_idx,
            is_band_type,
            (*s).lambda / minthr,
            f32::INFINITY,
            ptr::null_mut(),
            ptr::null_mut(),
        );

        let dist_spec_err = izip!(&L34[..swb_size], &R34[..swb_size], &I34[..swb_size])
            .map(|(&L34, &R34, &I34)| {
                (L34 - I34) * (L34 - I34) + (R34 - I34 * e01_34) * (R34 - I34 * e01_34)
            })
            .sum::<c_float>();

        dist2 += dist_spec_err * ((*s).lambda / minthr);
    }

    (dist2 <= dist1).then_some(AACISError {
        phase,
        error: dist2 - dist1,
        ener01,
    })
}

#[ffmpeg_src(file = "libavcodec/aacenc_is.c", lines = 98..=158, name = "ff_aac_search_for_is")]
pub(crate) unsafe fn search(
    mut s: *mut AACEncContext,
    mut avctx: *mut AVCodecContext,
    mut cpe: *mut ChannelElement,
) {
    let [sce0, sce1] = &mut (*cpe).ch;

    let mut start: c_int = 0;
    let mut count: c_int = 0;
    let mut prev_sf1: c_int = -1;
    let mut prev_bt: c_int = -1;
    let mut prev_is = false;
    let freq_mult =
        (*avctx).sample_rate as c_float / (1024. / sce0.ics.num_windows as c_float) / 2.;
    if (*cpe).common_window == 0 {
        return;
    }
    let mut nextband1 = ptr::from_mut(sce1).init_nextband_map();
    for WindowedIteration { w, group_len } in sce0.ics.iter_windows() {
        start = 0;
        for g in 0..sce0.ics.num_swb {
            let wstart = (w * 16 + g) as usize;
            if start as c_float * freq_mult > 6100. * ((*s).lambda / 170.)
                && (*cpe)
                    .ch
                    .iter()
                    .all(|ch| ch.band_type[wstart] != NOISE_BT && !ch.zeroes[wstart])
                && sfdelta_can_remove_band(sce1, &nextband1, prev_sf1, wstart as c_int)
            {
                let [coeffs0, coeffs1] = [&sce0.coeffs, &sce1.coeffs].map(|coeffs| {
                    coeffs[(start + w * 128) as usize..][..usize::from(group_len) * 128]
                        .array_chunks::<128>()
                        .flat_map(|chunk| &chunk[..sce0.ics.swb_sizes[g as usize].into()])
                });
                let (Sum(ener0), Sum(ener1), Sum(ener01), Sum(ener01p)) = zip(coeffs0, coeffs1)
                    .map(|(&coef0, &coef1)| {
                        (
                            coef0.powi(2),
                            coef1.powi(2),
                            (coef0 + coef1).powi(2),
                            (coef0 - coef1).powi(2),
                        )
                    })
                    .reduce_with();

                let ph_err1 = encoding_err(
                    s,
                    cpe,
                    start,
                    w,
                    g,
                    ener0,
                    ener1,
                    ener01p,
                    false,
                    Phase::Negative,
                );
                let ph_err2 = encoding_err(
                    s,
                    cpe,
                    start,
                    w,
                    g,
                    ener0,
                    ener1,
                    ener01,
                    false,
                    Phase::Positive,
                );
                let best = match (&ph_err1, &ph_err2) {
                    (Some(err1), Some(err2)) if err1.error < err2.error => Some(err1),
                    (_, Some(err2)) => Some(err2),
                    (Some(err), None) => Some(err),
                    (None, None) => None,
                };
                if let Some(best) = best {
                    (*cpe).is_mask[wstart] = true;
                    (*cpe).ms_mask[wstart] = false;
                    (*cpe).ch[0].is_ener[wstart] = (ener0 / best.ener01).sqrt();
                    (*cpe).ch[1].is_ener[wstart] = ener0 / ener1;
                    (*cpe).ch[1].band_type[wstart] = if let Phase::Positive = best.phase {
                        INTENSITY_BT
                    } else {
                        INTENSITY_BT2
                    };
                    if prev_is && prev_bt as c_uint != (*cpe).ch[1].band_type[wstart] as c_uint {
                        // Flip M/S mask and pick the other CB, since it encodes more efficiently
                        (*cpe).ms_mask[wstart] = true;
                        (*cpe).ch[1].band_type[wstart] = if let Phase::Positive = best.phase {
                            INTENSITY_BT2
                        } else {
                            INTENSITY_BT
                        };
                    }
                    prev_bt = (*cpe).ch[1].band_type[wstart] as c_int;
                    count += 1;
                }
            }
            if !sce1.zeroes[wstart] && (sce1.band_type[wstart]) < RESERVED_BT {
                prev_sf1 = sce1.sf_idx[wstart];
            }
            prev_is = (*cpe).is_mask[wstart];
            start += sce0.ics.swb_sizes[g as usize] as c_int;
        }
    }
    (*cpe).is_mode = count != 0;
}
