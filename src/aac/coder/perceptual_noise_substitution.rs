use std::{
    cell::Cell,
    iter::{once, successors, zip},
};

use array_util::{WindowedArray, W, W2};
use encoder::CodecContext;
use ffmpeg_src_macro::ffmpeg_src;
use izip::izip;
use libc::{c_double, c_float, c_int, c_long, c_uchar, c_uint};
use reductor::{MinF, MinMaxF, Reduce, Reductors, Sum};

use super::{
    math::lcg_random, quantization::QuantizationCost, quantize_band_cost, sfdelta_can_remove_band,
};
use crate::{
    aac::{
        encoder::{ctx::AACEncContext, pow::Pow34},
        psy_model::cutoff_from_bitrate,
        tables::POW_SF_TABLES,
        WindowedIteration,
    },
    types::*,
};

/// Frequency in Hz for lower limit of noise substitution
#[ffmpeg_src(file = "libavcodec/aaccoder_twoloop.h", lines = 54)]
const NOISE_LOW_LIMIT: c_float = 4000.;

/// Parameter of f(x) = a*(lambda/100), defines the maximum fourier spread
/// beyond which no PNS is used (since the SFBs contain tone rather than noise)
#[ffmpeg_src(file = "libavcodec/aaccoder.c", lines = 57)]
const NOISE_SPREAD_THRESHOLD: c_float = 0.9;

/// Parameter of f(x) = a*(100/lambda), defines how much PNS is allowed to
/// replace low energy non zero bands
#[ffmpeg_src(file = "libavcodec/aaccoder.c", lines = 61)]
const NOISE_LAMBDA_REPLACE: c_float = 1.948;

fn pns_transient_energy_r(lambda: c_float) -> c_float {
    0.7_f32.min(lambda / 140.)
}

fn refbits(avctx: &CodecContext, lambda: c_float) -> c_int {
    let bit_rate = avctx.bit_rate().get();
    let sample_rate = avctx.sample_rate().get();
    let flags = avctx.flags().get();
    let nb_channels = avctx.ch_layout().get().nb_channels;

    (bit_rate as c_double * 1024.
        / sample_rate as c_double
        / if flags.qscale() {
            2.
        } else {
            nb_channels as c_float
        } as c_double
        * (lambda / 120.) as c_double) as c_int
}

fn spread_threshold(lambda: c_float) -> c_float {
    0.75_f32.min(NOISE_SPREAD_THRESHOLD * 0.5_f32.max(lambda / 100.))
}

fn frame_bit_rate(avctx: &CodecContext, lambda: c_float) -> c_int {
    // Keep this in sync with twoloop's cutoff selection
    let rate_bandwidth_multiplier = 1.5;

    let bit_rate = avctx.bit_rate().get();
    let sample_rate = avctx.sample_rate().get();
    let flags = avctx.flags().get();
    let nb_channels = avctx.ch_layout().get().nb_channels;

    let mut frame_bit_rate: c_int = (if flags.qscale() {
        refbits(avctx, lambda) as c_float * rate_bandwidth_multiplier * sample_rate as c_float
            / 1024.
    } else {
        (bit_rate / nb_channels as c_long) as c_float
    }) as c_int;
    frame_bit_rate = (frame_bit_rate as c_float * 1.15) as c_int;
    frame_bit_rate
}

fn bandwidth(avctx: &CodecContext, lambda: c_float) -> c_int {
    if avctx.cutoff().get() > 0 {
        avctx.cutoff().get()
    } else {
        3000.max(cutoff_from_bitrate(
            frame_bit_rate(avctx, lambda),
            1,
            avctx.sample_rate().get(),
        ))
    }
}

fn freq_mult(sample_rate: c_int, wlen: c_int) -> c_float {
    sample_rate as c_float * 0.5 / wlen as c_float
}

fn cutoff(avctx: &CodecContext, lambda: c_float, wlen: c_int) -> c_int {
    bandwidth(avctx, lambda) * 2 * wlen / avctx.sample_rate().get()
}

fn freq_boost(freq: c_float) -> c_float {
    (0.88 * freq / NOISE_LOW_LIMIT).max(1.)
}

struct ReducedBands {
    sfb_energy: f32,
    spread: f32,
    threshold: f32,
    energy: MinMaxF<f32>,
}

fn reduce_bands(psy_bands: &[Cell<FFPsyBand>], group_len: u8) -> ReducedBands {
    let (
        Reductors::<(_, Option<MinMaxF<_>>)>((Sum(sfb_energy), energy)),
        MinF::<Option<_>>(spread),
        Sum::<f32>(threshold),
    ) = psy_bands
        .iter()
        .step_by(16)
        .take(group_len.into())
        .map(Cell::get)
        .map(
            |FFPsyBand {
                 energy,
                 threshold,
                 spread,
                 ..
             }| (energy, spread, threshold),
        )
        .reduce_with();

    ReducedBands {
        sfb_energy,
        spread: spread.unwrap_or(2.),
        threshold,
        energy: energy.unwrap_or(MinMaxF { min: -1., max: 0. }),
    }
}

#[ffmpeg_src(file = "libavcodec/aaccoder.c", lines = 765..=905, name = "search_for_pns")]
pub(crate) fn search(s: &mut AACEncContext, avctx: &CodecContext, sce: &mut SingleChannelElement) {
    let wlen: c_int = 1024 / c_int::from(c_uchar::from(sce.ics.num_windows));

    let sample_rate = avctx.sample_rate().get();

    let ([pns, pns34, _, nor34, ..], []) = s.scaled_coeffs.as_chunks_mut::<128>() else {
        unreachable!();
    };

    let lambda = s.lambda;
    let freq_mult = freq_mult(sample_rate, wlen);
    let thr_mult = NOISE_LAMBDA_REPLACE * (100. / lambda);
    let spread_threshold = spread_threshold(lambda);
    let dist_bias = (4. * 120. / lambda).clamp(0.25, 4.);
    let pns_transient_energy_r = pns_transient_energy_r(lambda);
    let mut prev = None;
    let mut prev_sf = None;
    let cutoff = cutoff(avctx, lambda, wlen);
    sce.band_alt = sce.band_type;
    let nextband = WindowedArray::<_, 16>(sce.init_next_band_map());
    let psy_bands = s.psy.ch[s.cur_channel as usize]
        .psy_bands
        .as_array_of_cells_deref();
    let band_types = sce.band_type.as_array_of_cells_deref();
    let sf_indices = sce.sf_idx.as_array_of_cells_deref();
    for WindowedIteration { w, group_len } in sce.ics.iter_windows() {
        let wstart: c_int = w * 128;
        let num_swb = sce.ics.num_swb as usize;
        for (
            g,
            (
                &swb_offset,
                &swb_size,
                zero,
                sf_idx,
                &band_alt,
                pns_ener,
                &nextband,
                band_type,
                prev_band_type,
                cur_psy_bands,
            ),
        ) in izip!(
            sce.ics.swb_offset,
            sce.ics.swb_sizes,
            &mut sce.zeroes[W(w)],
            &sf_indices[W(w)],
            &sce.band_alt[W(w)],
            &mut sce.pns_ener[W(w)],
            &nextband[W(w)],
            &band_types[W(w)],
            once(None).chain(band_types[W(w)].iter().map(Some)),
            successors(Some(&psy_bands[W(w)]), |bands| bands.get(1..)),
        )
        .take(num_swb)
        .enumerate()
        {
            let start = wstart + swb_offset as c_int;
            let freq = (start - wstart) as c_float * freq_mult;
            let freq_boost = freq_boost(freq);
            if freq < NOISE_LOW_LIMIT || start - wstart >= cutoff {
                if !*zero {
                    prev_sf = Some(sf_idx);
                }
                continue;
            }

            let ReducedBands {
                sfb_energy,
                spread,
                threshold,
                energy:
                    MinMaxF {
                        min: min_energy,
                        max: max_energy,
                    },
            } = reduce_bands(cur_psy_bands, group_len);

            // Ramps down at ~8000Hz and loosens the dist threshold
            let dist_thresh = (2.5 * NOISE_LOW_LIMIT / freq).clamp(0.5, 2.5) * dist_bias;

            // PNS is acceptable when all of these are true:
            // 1. high spread energy (noise-like band)
            // 2. near-threshold energy (high PE means the random nature of PNS content
            // will be noticed)
            // 3. on short window groups, all windows have similar energy (variations in
            // energy would be destroyed by PNS)
            //
            // At this stage, point 2 is relaxed for zeroed bands near
            // the noise threshold (hole avoidance is more important)
            if (!*zero
                && !prev_sf.is_some_and(|prev_sf| {
                    sfdelta_can_remove_band(sf_indices, prev_sf.get(), nextband)
                }))
                || ((*zero || band_alt == 0) && sfb_energy < threshold * freq_boost.recip().sqrt())
                || spread < spread_threshold
                || (!*zero && band_alt != 0 && sfb_energy > threshold * thr_mult * freq_boost)
                || min_energy < pns_transient_energy_r * max_energy
            {
                *pns_ener = sfb_energy;
                if !*zero {
                    prev_sf = Some(sf_idx);
                }
                continue;
            }

            let pns_tgt_energy = sfb_energy * c_float::min(1., spread * spread);
            let noise_sfi = ((pns_tgt_energy.log2() * 2.).round() as c_int).clamp(-100, 155);
            let noise_amp = -POW_SF_TABLES.pow2()[(noise_sfi + 200) as usize];
            if let Some(prev) = prev {
                let noise_sfdiff = noise_sfi - prev + 60;
                if !(0..=2 * 60).contains(&noise_sfdiff) {
                    if !*zero {
                        prev_sf = Some(sf_idx);
                    }
                    continue;
                }
            }

            let [pns, pns34, nor34] =
                [&mut *pns, pns34, nor34].map(|arr| &mut arr[..usize::from(swb_size)]);

            let (Sum::<c_float>(dist1), Sum::<c_float>(mut dist2), Sum::<c_float>(pns_energy)) =
                izip!(
                    &sce.coeffs[W2(w)],
                    &psy_bands[W2(w)],
                    &sf_indices[W2(w)],
                    &sce.band_alt[W2(w)],
                )
                .take(group_len.into())
                .map(|(coeffs, psy_bands, sf_indices, band_alts)| {
                    (
                        &coeffs[swb_offset.into()..][..swb_size.into()],
                        psy_bands[g].get(),
                        &sf_indices[g],
                        band_alts[g],
                    )
                })
                .map(|(coeffs, band, sf_idx, band_alt)| {
                    pns.fill_with(|| {
                        s.random_state = lcg_random(s.random_state as c_uint);
                        s.random_state as c_float
                    });

                    // (yotam): scalarproduct_float
                    let band_energy: c_float = pns.iter().map(|pns| pns.powi(2)).sum();

                    let scale = noise_amp / band_energy.sqrt();

                    // (yotam): vector_fmac_scalar
                    pns.iter_mut().for_each(|pns| {
                        *pns *= scale;
                    });
                    // (yotam): scalarproduct_float
                    let pns_energy: c_float = pns.iter().map(|pns| pns.powi(2)).sum();

                    for (nor34, coeff) in zip(&mut *nor34, coeffs) {
                        *nor34 = coeff.abs_pow34();
                    }
                    for (pns34, pns) in zip(&mut *pns34, &*pns) {
                        *pns34 = pns.abs_pow34();
                    }

                    let QuantizationCost {
                        distortion: dist1, ..
                    } = quantize_band_cost(
                        coeffs,
                        nor34,
                        sf_idx.get(),
                        band_alt as c_int,
                        lambda / band.threshold,
                        f32::INFINITY,
                    );
                    // Estimate rd on average as 5 bits for SF, 4 for the CB, plus spread energy *
                    // lambda/thr
                    let dist2 =
                        band.energy / band.spread.powi(2) * lambda * dist_thresh / band.threshold;

                    (dist1, dist2, pns_energy)
                })
                .reduce_with();
            dist2 += if prev_band_type.is_some_and(|band_type| band_type.get() == NOISE_BT) {
                5.
            } else {
                9.
            };
            let energy_ratio = pns_tgt_energy / pns_energy; // Compensates for quantization error
            *pns_ener = energy_ratio * pns_tgt_energy;
            if *zero || band_alt == 0 || energy_ratio > 0.85 && energy_ratio < 1.25 && dist2 < dist1
            {
                band_type.set(NOISE_BT);
                *zero = false;
                prev = Some(noise_sfi);
            } else if !*zero {
                prev_sf = Some(sf_idx);
            }
        }
    }
}

#[ffmpeg_src(file = "libavcodec/aaccoder.c", lines = 907..=976, name = "mark_pns")]
pub(crate) fn mark(s: &mut AACEncContext, avctx: &CodecContext, sce: &mut SingleChannelElement) {
    let wlen = 1024 / c_int::from(c_uchar::from(sce.ics.num_windows));
    let lambda = s.lambda;
    let freq_mult = freq_mult(avctx.sample_rate().get(), wlen);
    let spread_threshold = spread_threshold(lambda);
    let pns_transient_energy_r = pns_transient_energy_r(lambda);
    let cutoff = cutoff(avctx, lambda, wlen);
    sce.band_alt = sce.band_type;
    let psy_bands = s.psy.ch[s.cur_channel as usize]
        .psy_bands
        .as_array_of_cells_deref();
    for WindowedIteration { w, group_len } in sce.ics.iter_windows() {
        for (&swb_offset, can_pns, pns_ener, psy_bands) in izip!(
            sce.ics.swb_offset,
            &mut sce.can_pns[W(w)],
            &mut sce.pns_ener[W(w)],
            successors(Some(&psy_bands[W(w)]), |bands| bands.get(1..)),
        )
        .take(sce.ics.num_swb as usize)
        {
            let start = c_int::from(swb_offset);
            let freq: c_float = start as c_float * freq_mult;
            let freq_boost = freq_boost(freq);

            if freq < NOISE_LOW_LIMIT || start >= cutoff {
                *can_pns = false;
                continue;
            }

            let ReducedBands {
                sfb_energy,
                spread,
                threshold,
                energy:
                    MinMaxF {
                        min: min_energy,
                        max: max_energy,
                    },
            } = reduce_bands(psy_bands, group_len);

            // PNS is acceptable when all of these are true:
            // 1. high spread energy (noise-like band)
            // 2. near-threshold energy (high PE means the random nature of PNS content will
            //    be noticed)
            // 3. on short window groups, all windows have similar energy (variations in
            //    energy would be destroyed by PNS)
            *pns_ener = sfb_energy;
            *can_pns = !(sfb_energy < threshold * (1.5 / freq_boost).sqrt()
                || spread < spread_threshold
                || min_energy < pns_transient_energy_r * max_energy);
        }
    }
}
