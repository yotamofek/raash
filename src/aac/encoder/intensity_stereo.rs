#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use std::{iter::zip, ops::Mul};

use array_util::{WindowedArray, W};
use encoder::CodecContext;
use ffmpeg_src_macro::ffmpeg_src;
use izip::izip;
use libc::{c_double, c_float, c_int, c_uchar};
use reductor::{Reduce, Sum};

use super::pow::Pow34;
use crate::{
    aac::{
        coder::{quantize_band_cost, sfdelta_encoding_range},
        encoder::ctx::AACEncContext,
        tables::POW_SF_TABLES,
        IndividualChannelStream, WindowedIteration,
    },
    types::*,
};

#[inline]
fn pos_pow34(a: c_float) -> c_float {
    (a * a.sqrt()).sqrt()
}

#[inline]
fn find_min_book(maxval: c_float, sf: c_int) -> c_int {
    const MAXVAL_CB: [c_uchar; 14] = [0, 1, 3, 5, 5, 7, 7, 7, 9, 9, 9, 9, 9, 11];

    let mut Q34: c_float = POW_SF_TABLES.pow34()[(200 - sf + 140 - 36) as usize];
    let qmaxval = (maxval * Q34 + 0.405_4_f32) as usize;
    MAXVAL_CB.get(qmaxval).copied().unwrap_or(11) as c_int
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
struct ISError {
    phase: Phase,
    error: c_float,
    ener01: c_float,
}

#[ffmpeg_src(file = "libavcodec/aacenc_is.c", lines = 98..=158, name = "ff_aac_search_for_is")]
pub(crate) fn search(s: &mut AACEncContext, avctx: &CodecContext, cpe: &mut ChannelElement) {
    let AACEncContext {
        scaled_coeffs,
        mut lambda,
        psy: FFPsyContext {
            ch: psy_channels, ..
        },
        mut cur_channel,
        ..
    } = s;
    let sample_rate = avctx.sample_rate().get();

    let [FFPsyChannel {
        psy_bands: psy_bands0,
        ..
    }, FFPsyChannel {
        psy_bands: psy_bands1,
        ..
    }] = &psy_channels[cur_channel as usize..][..2]
    else {
        unreachable!()
    };

    let ChannelElement {
        mut common_window,
        is_mode,
        ms_mask,
        is_mask,
        ch: [sce0, sce1],
        ..
    } = cpe;

    if common_window == 0 {
        return;
    }

    let mut prev_sf1 = None;
    let mut prev_bt = None;
    let mut prev_is = false;

    let nextband1 = WindowedArray::<_, 16>(sce1.init_next_band_map());

    let [SingleChannelElement {
        ics: ref ics0 @ IndividualChannelStream {
            mut num_windows, ..
        },
        coeffs: ref coeffs0,
        zeroes: ref zeroes0,
        band_type: band_types0,
        is_ener: is_ener0,
        sf_idx: ref sf_indices0,
        ..
    }, SingleChannelElement {
        ics:
            IndividualChannelStream {
                swb_sizes: mut swb_sizes1,
                ..
            },
        coeffs: ref coeffs1,
        zeroes: ref zeroes1,
        band_type: band_types1,
        is_ener: is_ener1,
        sf_idx: ref sf_indices1,
        ..
    }] = [sce0, sce1];

    let freq_mult = sample_rate as c_float / (1024. / num_windows as c_float) / 2.;

    for WindowedIteration { w, group_len } in ics0.iter_windows() {
        let [coeffs0, coeffs1] = [coeffs0, coeffs1].map(|coeffs| &coeffs[W(w)]);
        let [psy_bands0, psy_bands1] = [psy_bands0, psy_bands1]
            .map(|bands| &bands[W(w)])
            .map(WindowedArray::<_, 16>::from_ref);

        for (
            g,
            (swb_size0, start),
            &swb_size1,
            band_type0,
            band_type1,
            &zero0,
            &zero1,
            is_mask,
            ms_mask,
            is_ener0,
            is_ener1,
            &sf_idx0,
            &sf_idx1,
            &nextband1,
        ) in izip!(
            0..,
            ics0.iter_swb_sizes_sum(),
            swb_sizes1,
            &mut band_types0[W(w)],
            &mut band_types1[W(w)],
            &zeroes0[W(w)],
            &zeroes1[W(w)],
            &mut is_mask[W(w)],
            &mut ms_mask[W(w)],
            &mut is_ener0[W(w)],
            &mut is_ener1[W(w)],
            &sf_indices0[W(w)],
            &sf_indices1[W(w)],
            &nextband1[W(w)],
        ) {
            if c_float::from(start) * freq_mult > 6100. * (lambda / 170.)
                && [(*band_type0, zero0), (*band_type1, zero1)]
                    .iter()
                    .all(|&(band_type, zero)| band_type != NOISE_BT && !zero)
                && let Some(sf) = prev_sf1
                // (yotam): inlined from sfdelta_can_remove_band
                && sfdelta_encoding_range(sf).contains(&(**sf_indices1)[usize::from(nextband1)])
            {
                let (Sum::<c_float>(ener0), Sum::<c_float>(ener1), Sum(ener01), Sum(ener01p)) = {
                    let [coeffs0, coeffs1] = [coeffs0, coeffs1].map(|coeffs| {
                        coeffs[start as usize..][..usize::from(group_len) * 128]
                            .array_chunks::<128>()
                            .flat_map(|chunk| &chunk[..swb_size0.into()])
                    });
                    zip(coeffs0, coeffs1)
                        .map(|(&coef0, &coef1)| {
                            (
                                coef0.powi(2),
                                coef1.powi(2),
                                (coef0 + coef1).powi(2),
                                (coef0 - coef1).powi(2),
                            )
                        })
                        .reduce_with()
                };

                // #[ffmpeg_src(file = "libavcodec/aacenc_is.c", lines = 33..=96, name =
                // "ff_aac_is_encoding_err")]
                let mut encoding_err = |ener01, phase| {
                    if ener01 <= 0. || ener0 <= 0. {
                        return None;
                    }

                    let [L, R] = [coeffs0, coeffs1].map(WindowedArray::<_, 128>::from_ref);

                    let ([L34, R34, IS, I34, ..], []) = scaled_coeffs.as_chunks_mut::<256>() else {
                        unreachable!();
                    };

                    let (Sum::<c_float>(dist1), Sum::<c_float>(dist2)) = (0..group_len)
                        .map(|w| {
                            (
                                [L, R].map(|C| &C[W(w)][start as usize..]),
                                [psy_bands0, psy_bands1].map(|bands| &bands[W(w)][g]),
                            )
                        })
                        .map(|([L, R], [band0, band1])| {
                            let is_sf_idx: c_int = 1.max(sf_idx0 - 4);
                            let minthr = c_float::min(band0.threshold, band1.threshold);

                            for (IS, &L, &R) in izip!(&mut *IS, L, R).take(swb_size0.into()) {
                                *IS = ((L + phase * R) as c_double
                                    * ((ener0 / ener01) as c_double).sqrt())
                                    as c_float;
                            }
                            for (C34, C) in [(&mut *L34, L), (R34, R), (I34, IS)]
                                .into_iter()
                                .flat_map(|(C34, C)| zip(&mut *C34, C).take(swb_size0.into()))
                            {
                                *C34 = C.abs_pow34();
                            }

                            let maxval = I34
                                .iter()
                                .take(swb_size0.into())
                                .copied()
                                .max_by(c_float::total_cmp)
                                .unwrap();
                            let is_band_type = find_min_book(maxval, is_sf_idx);

                            let dist1 = quantize_band_cost(
                                &L[..swb_size0.into()],
                                &L34[..swb_size0.into()],
                                sf_idx0,
                                *band_type0 as c_int,
                                lambda / band0.threshold,
                                f32::INFINITY,
                                None,
                                None,
                            ) + quantize_band_cost(
                                &R[..swb_size1.into()],
                                &R34[..swb_size1.into()],
                                sf_idx1,
                                *band_type1 as c_int,
                                lambda / band1.threshold,
                                f32::INFINITY,
                                None,
                                None,
                            );

                            let dist2 = quantize_band_cost(
                                &IS[..swb_size0.into()],
                                &I34[..swb_size0.into()],
                                is_sf_idx,
                                is_band_type,
                                lambda / minthr,
                                f32::INFINITY,
                                None,
                                None,
                            ) + {
                                let e01_34: c_float = phase * pos_pow34(ener1 / ener0);
                                let dist_spec_err = izip!(&*L34, &*R34, &*I34)
                                    .take(swb_size0.into())
                                    .map(|(&L34, &R34, &I34)| {
                                        (L34 - I34) * (L34 - I34)
                                            + (R34 - I34 * e01_34) * (R34 - I34 * e01_34)
                                    })
                                    .sum::<c_float>();

                                dist_spec_err * (lambda / minthr)
                            };

                            (dist1, dist2)
                        })
                        .reduce_with();

                    (dist2 <= dist1).then_some(ISError {
                        phase,
                        error: dist2 - dist1,
                        ener01,
                    })
                };

                let [ph_err1, ph_err2] = [(ener01p, Phase::Negative), (ener01, Phase::Positive)]
                    .map(|(ener01, phase)| encoding_err(ener01, phase));

                let best = match (ph_err1, ph_err2) {
                    (Some(err1), Some(err2)) if err1.error < err2.error => Some(err1),
                    (_, Some(err2)) => Some(err2),
                    (Some(err), None) => Some(err),
                    (None, None) => None,
                };
                if let Some(best) = best {
                    *is_mask = true;
                    *ms_mask = false;
                    (*is_ener0, *is_ener1) = ((ener0 / best.ener01).sqrt(), ener0 / ener1);
                    *band_type1 = if let Phase::Positive = best.phase {
                        INTENSITY_BT
                    } else {
                        INTENSITY_BT2
                    };
                    if prev_is && prev_bt != Some(*band_type1) {
                        // Flip M/S mask and pick the other CB, since it encodes more efficiently
                        *ms_mask = true;
                        *band_type1 = if let Phase::Positive = best.phase {
                            INTENSITY_BT2
                        } else {
                            INTENSITY_BT
                        };
                    }
                    prev_bt = Some(*band_type1);
                    *is_mode = true;
                }
            }
            if !zero1 && *band_type1 < RESERVED_BT {
                prev_sf1 = Some(sf_idx1);
            }
            prev_is = *is_mask;
        }
    }
}

#[ffmpeg_src(file = "libavcodec/aacenc.c", lines = 580..=607, name = "apply_intensity_stereo")]
pub(super) fn apply(cpe: &mut ChannelElement) {
    let ChannelElement {
        mut common_window,
        ref ms_mask,
        ref is_mask,
        ch:
            [SingleChannelElement {
                ref ics,
                ref is_ener,
                coeffs: coeffs0,
                ..
            }, SingleChannelElement {
                ref band_type,
                coeffs: coeffs1,
                ..
            }],
        ..
    } = cpe;

    if common_window == 0 {
        return;
    }

    let [coeffs0, coeffs1] = [coeffs0, coeffs1].map(WindowedArray::as_array_of_cells_deref);

    for WindowedIteration { w, group_len } in ics.iter_windows() {
        for (coeffs0, coeffs1) in zip(coeffs0, coeffs1).take(group_len.into()) {
            for ((swb_size, offset), &band_type, &scale, _, &ms_mask) in izip!(
                ics.iter_swb_sizes_sum(),
                &band_type[W(w)],
                &is_ener[W(w)],
                &is_mask[W(w)],
                &ms_mask[W(w)],
            )
            .filter(|(_, _, _, &is_mask, _)| is_mask)
            {
                let p = {
                    let p = -1 + 2 * (band_type as c_int - 14);

                    if ms_mask {
                        -p
                    } else {
                        p
                    }
                } as c_float;

                for (coeff0, coeff1) in zip(coeffs0, coeffs1)
                    .skip(offset.into())
                    .take(swb_size.into())
                {
                    let sum = (coeff0.get() + coeff1.get() * p) * scale;
                    coeff0.set(sum);
                    coeff1.set(0.);
                }
            }
        }
    }
}
