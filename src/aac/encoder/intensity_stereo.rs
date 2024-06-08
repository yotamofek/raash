use std::{iter::zip, ops::Mul};

use array_util::{WindowedArray, W, W2};
use encoder::CodecContext;
use ffmpeg_src_macro::ffmpeg_src;
use izip::izip;
use libc::{c_double, c_float, c_int, c_uchar};
use reductor::{Reduce, Sum};

use super::pow::Pow34;
use crate::{
    aac::{
        coder::{quantization::QuantizationCost, quantize_band_cost, sfdelta_encoding_range},
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

    let q34: c_float = POW_SF_TABLES.pow34()[(200 - sf + 140 - 36) as usize];
    let qmaxval = (maxval * q34 + 0.405_4_f32) as usize;
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
        ref mut scaled_coeffs,
        lambda,
        psy: PsyContext {
            ch: ref mut psy_channels,
            ..
        },
        cur_channel,
        ..
    } = *s;
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
        common_window,
        ref mut is_mode,
        ref mut ms_mask,
        ref mut is_mask,
        ch: [ref mut sce0, ref mut sce1],
        ..
    } = *cpe;

    if common_window == 0 {
        return;
    }

    let mut prev_sf1 = None;
    let mut prev_bt = None;
    let mut prev_is = false;

    let nextband1 = WindowedArray::<_, 16>(sce1.init_next_band_map());

    let [&mut SingleChannelElement {
        ics: ref ics0 @ IndividualChannelStream { num_windows, .. },
        coeffs: ref coeffs0,
        zeroes: ref zeroes0,
        band_type: ref mut band_types0,
        is_ener: ref mut is_ener0,
        sf_idx: ref sf_indices0,
        ..
    }, &mut SingleChannelElement {
        ics: IndividualChannelStream {
            swb_sizes: swb_sizes1,
            ..
        },
        coeffs: ref coeffs1,
        zeroes: ref zeroes1,
        band_type: ref mut band_types1,
        is_ener: ref mut is_ener1,
        sf_idx: ref sf_indices1,
        ..
    }] = [sce0, sce1];

    let freq_mult =
        sample_rate as c_float / (1024. / c_float::from(c_uchar::from(num_windows))) / 2.;

    for WindowedIteration { w, group_len } in ics0.iter_windows() {
        let [coeffs0, coeffs1] = [coeffs0, coeffs1].map(|coeffs| &coeffs[W2(w)]);
        let [psy_bands0, psy_bands1] = [psy_bands0, psy_bands1].map(|bands| &bands[W2(w)]);

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
                        (**coeffs)[start as usize..][..usize::from(group_len) * 128]
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

                    let [left, right] = [coeffs0, coeffs1];

                    let ([left34, right34, is, i34, ..], []) = scaled_coeffs.as_chunks_mut::<256>()
                    else {
                        unreachable!();
                    };

                    let (Sum::<c_float>(dist1), Sum::<c_float>(dist2)) = (0..group_len)
                        .map(|w| {
                            (
                                [left, right].map(|coeffs| &coeffs[W(w)][start as usize..]),
                                [psy_bands0, psy_bands1].map(|bands| &bands[W(w)][g]),
                            )
                        })
                        .map(|([left, right], [band0, band1])| {
                            let is_sf_idx: c_int = 1.max(sf_idx0 - 4);
                            let minthr = c_float::min(band0.threshold, band1.threshold);

                            for (is, &left, &right) in
                                izip!(&mut *is, left, right).take(swb_size0.into())
                            {
                                *is = ((left + phase * right) as c_double
                                    * ((ener0 / ener01) as c_double).sqrt())
                                    as c_float;
                            }
                            for (coeff34, coeff) in
                                [(&mut *left34, left), (right34, right), (i34, is)]
                                    .into_iter()
                                    .flat_map(|(coeff34, coeff)| {
                                        zip(&mut *coeff34, coeff).take(swb_size0.into())
                                    })
                            {
                                *coeff34 = coeff.abs_pow34();
                            }

                            let maxval = i34
                                .iter()
                                .take(swb_size0.into())
                                .copied()
                                .max_by(c_float::total_cmp)
                                .unwrap();
                            let is_band_type = find_min_book(maxval, is_sf_idx);

                            let QuantizationCost {
                                distortion: dist1, ..
                            } = quantize_band_cost(
                                &left[..swb_size0.into()],
                                &left34[..swb_size0.into()],
                                sf_idx0,
                                *band_type0 as c_int,
                                lambda / band0.threshold,
                                f32::INFINITY,
                            ) + quantize_band_cost(
                                &right[..swb_size1.into()],
                                &right34[..swb_size1.into()],
                                sf_idx1,
                                *band_type1 as c_int,
                                lambda / band1.threshold,
                                f32::INFINITY,
                            );

                            let dist2 = quantize_band_cost(
                                &is[..swb_size0.into()],
                                &i34[..swb_size0.into()],
                                is_sf_idx,
                                is_band_type,
                                lambda / minthr,
                                f32::INFINITY,
                            )
                            .distortion
                                + {
                                    let e01_34: c_float = phase * pos_pow34(ener1 / ener0);
                                    let dist_spec_err = izip!(&*left34, &*right34, &*i34)
                                        .take(swb_size0.into())
                                        .map(|(&left34, &right34, &i34)| {
                                            (left34 - i34).powi(2)
                                                + (right34 - i34 * e01_34).powi(2)
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
        common_window,
        ref ms_mask,
        ref is_mask,
        ch:
            [SingleChannelElement {
                ref ics,
                ref is_ener,
                coeffs: ref mut coeffs0,
                ..
            }, SingleChannelElement {
                ref band_type,
                coeffs: ref mut coeffs1,
                ..
            }],
        ..
    } = *cpe;

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
