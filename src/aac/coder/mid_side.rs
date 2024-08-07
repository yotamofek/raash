use std::iter::zip;

use array_util::{WindowedArray, W, W2};
use ffmpeg_src_macro::ffmpeg_src;
use izip::izip;
use libc::{c_float, c_int};
use reductor::{Reduce as _, Sum};

use super::{
    find_min_book, math::Float as _, quantization::QuantizationCost, quantize_band_cost,
    sfdelta_can_replace, SingleChannelElement,
};
use crate::{
    aac::{
        encoder::{ctx::AACEncContext, pow::Pow34},
        WindowedIteration, SCALE_DIV_512, SCALE_MAX_POS,
    },
    types::{BandType, ChannelElement, NOISE_BT, RESERVED_BT},
};

#[ffmpeg_src(file = "libavcodec/aaccoder.c", lines = 978..=1117, name = "search_for_ms")]
pub(crate) fn search(s: &mut AACEncContext, cpe: &mut ChannelElement) {
    let ([mid, side, left34, right34, mid34, side34, ..], []) =
        s.scaled_coeffs.as_chunks_mut::<128>()
    else {
        unreachable!();
    };

    let lambda = s.lambda;
    let mslambda = f32::min(1., lambda / 120.);

    let [sce0, sce1] = &mut cpe.ch;

    if cpe.common_window == 0 {
        return;
    }

    // Scout out next nonzero bands
    let nextband0 = WindowedArray::<_, 16>(sce0.init_next_band_map());
    let nextband1 = WindowedArray::<_, 16>(sce1.init_next_band_map());

    let mut prev_mid = sce0.sf_idx[W(0)][0];
    let mut prev_side = sce1.sf_idx[W(0)][0];
    let [sf_indices0, sf_indices1] =
        [&mut sce0.sf_idx, &mut sce1.sf_idx].map(WindowedArray::as_array_of_cells_deref);
    for WindowedIteration { w, group_len } in sce0.ics.iter_windows() {
        let [coeffs0, coeffs1] = [&sce0.coeffs, &sce1.coeffs].map(|coeffs| &coeffs[W2(w)]);
        let [psy_chans0, psy_chans1, ..] = &s.psy.ch[s.cur_channel as usize..] else {
            unreachable!();
        };
        let [psy_bands0, psy_bands1] = [psy_chans0, psy_chans1].map(|ch| &ch.psy_bands[W2(w)]);
        for (
            g,
            (
                (swb_size0, offset),
                &swb_size1,
                &is_mask,
                ms_mask,
                &zeroes0,
                &zeroes1,
                sf_idx0,
                sf_idx1,
                band_type0,
                band_type1,
                &nextband0,
                &nextband1,
            ),
        ) in izip!(
            sce0.ics.iter_swb_sizes_sum(),
            sce1.ics.swb_sizes,
            &cpe.is_mask[W(w)],
            &mut cpe.ms_mask[W(w)],
            &sce0.zeroes[W(w)],
            &sce1.zeroes[W(w)],
            &sf_indices0[W(w)],
            &sf_indices1[W(w)],
            &mut sce0.band_type[W(w)],
            &mut sce1.band_type[W(w)],
            &nextband0[W(w)],
            &nextband1[W(w)],
        )
        .enumerate()
        {
            let bmax = (g as c_float * 17. / sce0.ics.num_swb as c_float).bval2bmax() / 0.0045;
            if !is_mask {
                *ms_mask = false;
            }
            if !zeroes0 && !zeroes1 && !is_mask {
                let mut mid_max: c_float = 0.;
                let mut side_max: c_float = 0.;
                for (coeffs0, coeffs1) in zip(coeffs0, coeffs1).take(group_len.into()) {
                    for (mid, side, (&coeff0, &coeff1)) in izip!(
                        &mut *mid,
                        &mut *side,
                        zip(coeffs0, coeffs1).skip(offset.into()),
                    ) {
                        *mid = (coeff0 + coeff1) * 0.5;
                        *side = *mid - coeff1;
                    }
                    for (mid34, mid) in zip(&mut *mid34, &*mid).take(swb_size0.into()) {
                        *mid34 = mid.abs_pow34();
                    }
                    for (side34, side) in zip(&mut *side34, &*side).take(swb_size0.into()) {
                        *side34 = side.abs_pow34();
                    }
                    for (&mid, &side) in zip(&*mid34, &*side34).take(swb_size0.into()) {
                        mid_max = mid_max.max(mid);
                        side_max = side_max.max(side);
                    }
                }
                for sid_sf_boost in 0..4 {
                    let minidx = c_int::min(sf_idx0.get(), sf_idx1.get());
                    let mididx = minidx.clamp(0, (SCALE_MAX_POS - SCALE_DIV_512).into());
                    let sididx = (minidx - sid_sf_boost * 3)
                        .clamp(0, (SCALE_MAX_POS - SCALE_DIV_512).into());

                    if ![*band_type0, *band_type1].contains(&NOISE_BT)
                        && (!sfdelta_can_replace(sf_indices0, prev_mid, mididx, nextband0)
                            || !sfdelta_can_replace(sf_indices1, prev_side, sididx, nextband1))
                    {
                        continue;
                    }

                    let [midcb, sidcb] = [
                        find_min_book(mid_max, mididx),
                        find_min_book(side_max, sididx),
                    ]
                    .map(|cb| {
                        // No CB can be zero
                        cb.max(1)
                    });

                    let (
                        Sum::<c_float>(dist1),
                        Sum::<c_float>(dist2),
                        Sum::<c_int>(bits0),
                        Sum::<c_int>(bits1),
                    ) = izip!(psy_bands0, psy_bands1, coeffs0, coeffs1)
                        .take(group_len.into())
                        .map(|(bands0, bands1, coeffs0, coeffs1)| {
                            let [band0, band1] = [bands0, bands1].map(|band| &band[g]);
                            let minthr = c_float::min(band0.threshold, band1.threshold);
                            for (mid, side, (&coeff0, &coeff1)) in izip!(
                                &mut *mid,
                                &mut *side,
                                zip(coeffs0, coeffs1).skip(offset.into())
                            )
                            .take(swb_size0.into())
                            {
                                *mid = (coeff0 + coeff1) * 0.5;
                                *side = *mid - coeff1;
                            }
                            for (left34, coeff) in
                                zip(&mut *left34, coeffs0.iter().skip(offset.into()))
                                    .take(swb_size0.into())
                            {
                                *left34 = coeff.abs_pow34();
                            }
                            for (right34, coeff) in
                                zip(&mut *right34, coeffs1.iter().skip(offset.into()))
                                    .take(swb_size0.into())
                            {
                                *right34 = coeff.abs_pow34();
                            }
                            for (mid34, mid) in zip(&mut *mid34, &*mid).take(swb_size0.into()) {
                                *mid34 = mid.abs_pow34();
                            }
                            for (side34, side) in zip(&mut *side34, &*side).take(swb_size0.into()) {
                                *side34 = side.abs_pow34();
                            }
                            let QuantizationCost {
                                distortion: dist1,
                                bits: bits0,
                                ..
                            } = quantize_band_cost(
                                &coeffs0[offset.into()..][..swb_size0.into()],
                                &left34[..swb_size0.into()],
                                sf_idx0.get(),
                                *band_type0 as c_int,
                                lambda / (band0.threshold + c_float::MIN_POSITIVE),
                                f32::INFINITY,
                            ) + quantize_band_cost(
                                &coeffs1[offset.into()..][..swb_size1.into()],
                                &right34[..swb_size1.into()],
                                sf_idx1.get(),
                                *band_type1 as c_int,
                                lambda / (band1.threshold + c_float::MIN_POSITIVE),
                                f32::INFINITY,
                            );
                            let QuantizationCost {
                                distortion: dist2,
                                bits: bits1,
                                ..
                            } = quantize_band_cost(
                                &mid[..swb_size0.into()],
                                &mid34[..swb_size0.into()],
                                mididx,
                                midcb.into(),
                                lambda / (minthr + c_float::MIN_POSITIVE),
                                f32::INFINITY,
                            ) + quantize_band_cost(
                                &side[..swb_size1.into()],
                                &side34[..swb_size1.into()],
                                sididx,
                                sidcb.into(),
                                mslambda / (minthr * bmax + c_float::MIN_POSITIVE),
                                f32::INFINITY,
                            );
                            (dist1, dist2, bits0, bits1)
                        })
                        .map(|(dist1, dist2, bits0, bits1)| {
                            (
                                dist1 - bits0 as c_float,
                                dist2 - bits1 as c_float,
                                bits0,
                                bits1,
                            )
                        })
                        .reduce_with();
                    *ms_mask = dist2 <= dist1 && bits1 < bits0;
                    if *ms_mask {
                        if ![*band_type0, *band_type1].contains(&NOISE_BT) {
                            sf_idx0.set(mididx);
                            sf_idx1.set(sididx);
                            *band_type0 = midcb as BandType;
                            *band_type1 = sidcb as BandType;
                        } else if (*band_type0 != NOISE_BT) ^ (*band_type1 != NOISE_BT) {
                            // ms_mask unneeded, and it confuses some decoders
                            *ms_mask = false;
                        }
                        break;
                    } else if bits1 > bits0 {
                        // More boost won't fix this
                        break;
                    }
                }
            }
            if !zeroes0 && *band_type0 < RESERVED_BT {
                prev_mid = sf_idx0.get();
            }
            if !zeroes1 && !is_mask && *band_type1 < RESERVED_BT {
                prev_side = sf_idx1.get();
            }
        }
    }
}

#[ffmpeg_src(file = "libavcodec/aacenc.c", lines = 609..=639, name = "apply_mid_side_stereo")]
pub(crate) fn apply(cpe: &mut ChannelElement) {
    let ChannelElement {
        ch:
            [SingleChannelElement {
                ref ics,
                band_type: ref band_types0,
                coeffs: ref mut coeffs0,
                ..
            }, SingleChannelElement {
                band_type: ref band_types1,
                coeffs: ref mut coeffs1,
                ..
            }],
        ref ms_mask,
        ref is_mask,
        common_window,
        ..
    } = *cpe;

    if common_window == 0 {
        return;
    }

    for WindowedIteration { w, group_len } in ics.iter_windows() {
        let [coeffs0, coeffs1] = [&mut *coeffs0, &mut *coeffs1]
            .map(WindowedArray::as_array_of_cells_deref)
            .map(|coeffs| &coeffs[W2(w)]);
        for (left_coeffs, right_coeffs) in zip(coeffs0, coeffs1).take(group_len.into()) {
            for ((swb_size, offset), ..) in izip!(
                ics.iter_swb_sizes_sum(),
                &ms_mask[W(w)],
                &is_mask[W(w)],
                &band_types0[W(w)],
                &band_types1[W(w)],
            )
            .filter(|(_, &ms_mask, &is_mask, &band_type0, &band_type1)| {
                // ms_mask can be used for other purposes in PNS and I/S,
                // so must not apply M/S if any band uses either, even if
                // ms_mask is set.
                ms_mask && !is_mask && band_type0 < NOISE_BT && band_type1 < NOISE_BT
            }) {
                for (left, right) in zip(left_coeffs, right_coeffs)
                    .skip(offset.into())
                    .take(swb_size.into())
                {
                    left.update(|left| (left + right.get()) * 0.5);
                    right.update(|right| left.get() - right);
                }
            }
        }
    }
}
