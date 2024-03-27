use std::{iter, ptr};

use ffi::codec::{channel::AVChannelLayout, AVCodecContext};
use ffmpeg_src_macro::ffmpeg_src;
use libc::{c_double, c_float, c_int, c_long, c_uint};
use reductor::{MinF, MinMaxF, Reduce, Reductors, Sum};

use super::{math::lcg_random, quantize_band_cost, sfdelta_can_remove_band};
use crate::{
    aac::{
        encoder::{ctx::AACEncContext, pow::Pow34},
        psy_model::cutoff_from_bitrate,
        tables::POW_SF_TABLES,
        WindowedIteration,
    },
    common::*,
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

unsafe fn refbits(avctx: *const AVCodecContext, lambda: c_float) -> c_int {
    let AVCodecContext {
        bit_rate,
        sample_rate,
        flags,
        ch_layout: AVChannelLayout { nb_channels, .. },
        ..
    } = *avctx;

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

unsafe fn frame_bit_rate(avctx: *const AVCodecContext, lambda: c_float) -> c_int {
    // Keep this in sync with twoloop's cutoff selection
    let rate_bandwidth_multiplier = 1.5;

    let AVCodecContext {
        bit_rate,
        sample_rate,
        flags,
        ch_layout: AVChannelLayout { nb_channels, .. },
        ..
    } = *avctx;

    let mut frame_bit_rate: c_int = (if flags.qscale() {
        refbits(avctx, lambda) as c_float * rate_bandwidth_multiplier * sample_rate as c_float
            / 1024.
    } else {
        (bit_rate / nb_channels as c_long) as c_float
    }) as c_int;
    frame_bit_rate = (frame_bit_rate as c_float * 1.15) as c_int;
    frame_bit_rate
}

unsafe fn bandwidth(avctx: *const AVCodecContext, lambda: c_float) -> c_int {
    if (*avctx).cutoff > 0 {
        (*avctx).cutoff
    } else {
        3000.max(cutoff_from_bitrate(
            frame_bit_rate(avctx, lambda),
            1,
            (*avctx).sample_rate,
        ))
    }
}

fn freq_mult(sample_rate: c_int, wlen: c_int) -> c_float {
    sample_rate as c_float * 0.5 / wlen as c_float
}

unsafe fn cutoff(avctx: *const AVCodecContext, lambda: c_float, wlen: c_int) -> c_int {
    bandwidth(avctx, lambda) * 2 * wlen / (*avctx).sample_rate
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

fn reduce_bands(psy_bands: &[FFPsyBand], group_len: u8) -> ReducedBands {
    let (
        Reductors::<(_, Option<MinMaxF<_>>)>((Sum(sfb_energy), energy)),
        MinF::<Option<_>>(spread),
        Sum::<f32>(threshold),
    ) = psy_bands
        .iter()
        .step_by(16)
        .take(group_len.into())
        .map(
            |&FFPsyBand {
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
pub(crate) unsafe fn search(
    mut s: *mut AACEncContext,
    mut avctx: *const AVCodecContext,
    mut sce: *mut SingleChannelElement,
) {
    let mut wlen: c_int = 1024 / (*sce).ics.num_windows;

    let AVCodecContext { sample_rate, .. } = *avctx;

    let ([PNS, PNS34, _, NOR34, ..], _) = (*s).scoefs.as_chunks_mut::<128>() else {
        panic!();
    };

    let lambda: c_float = (*s).lambda;
    let freq_mult = freq_mult(sample_rate, wlen);
    let thr_mult: c_float = NOISE_LAMBDA_REPLACE * (100. / lambda);
    let spread_threshold = spread_threshold(lambda);
    let dist_bias: c_float = (4. * 120. / lambda).clamp(0.25, 4.);
    let pns_transient_energy_r = pns_transient_energy_r(lambda);
    let mut prev: c_int = -1000;
    let mut prev_sf: c_int = -1;
    let cutoff = cutoff(avctx, lambda, wlen);
    (*sce).band_alt = (*sce).band_type;
    let mut nextband = sce.init_nextband_map();
    for WindowedIteration { w, group_len } in (*sce).ics.iter_windows() {
        let mut wstart: c_int = w * 128;
        let num_swb = (*sce).ics.num_swb as usize;
        for (g, (&swb_offset, &swb_size)) in iter::zip(
            &(*sce).ics.swb_offset[..num_swb],
            &(*sce).ics.swb_sizes[..num_swb],
        )
        .enumerate()
        {
            let g = g as c_int;
            let mut dist1: c_float = 0.;
            let mut dist2: c_float = 0.;
            let mut pns_energy: c_float = 0.;
            let start: c_int = wstart + swb_offset as c_int;
            let freq: c_float = (start - wstart) as c_float * freq_mult;
            let freq_boost = freq_boost(freq);
            if freq < NOISE_LOW_LIMIT || start - wstart >= cutoff {
                if !(*sce).zeroes[(w * 16 + g) as usize] {
                    prev_sf = (*sce).sf_idx[(w * 16 + g) as usize];
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
            } = reduce_bands(
                &(*s).psy.ch[(*s).cur_channel as usize].psy_bands[(w * 16 + g) as usize..],
                group_len,
            );

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
            if (!(*sce).zeroes[(w * 16 + g) as usize]
                && !sfdelta_can_remove_band(sce, nextband.as_mut_ptr(), prev_sf, w * 16 + g))
                || (((*sce).zeroes[(w * 16 + g) as usize]
                    || (*sce).band_alt[(w * 16 + g) as usize] as u64 == 0)
                    && sfb_energy < threshold * sqrtf(1. / freq_boost))
                || spread < spread_threshold
                || (!(*sce).zeroes[(w * 16 + g) as usize]
                    && (*sce).band_alt[(w * 16 + g) as usize] as c_uint != 0
                    && sfb_energy > threshold * thr_mult * freq_boost)
                || min_energy < pns_transient_energy_r * max_energy
            {
                (*sce).pns_ener[(w * 16 + g) as usize] = sfb_energy;
                if !(*sce).zeroes[(w * 16 + g) as usize] {
                    prev_sf = (*sce).sf_idx[(w * 16 + g) as usize];
                }
                continue;
            }

            let pns_tgt_energy = sfb_energy * c_float::min(1., spread * spread);
            let noise_sfi = av_clip_c(roundf(log2f(pns_tgt_energy) * 2.) as c_int, -100, 155);
            let noise_amp = -POW_SF_TABLES.pow2[(noise_sfi + 200) as usize];
            if prev != -1000 {
                let mut noise_sfdiff: c_int = noise_sfi - prev + 60;
                if !(0..=2 * 60).contains(&noise_sfdiff) {
                    if !(*sce).zeroes[(w * 16 + g) as usize] {
                        prev_sf = (*sce).sf_idx[(w * 16 + g) as usize];
                    }
                    continue;
                }
            }

            for w2 in 0..c_int::from(group_len) {
                let start_c: c_int = (w + w2) * 128 + swb_offset as c_int;
                let band = &mut (*s).psy.ch[(*s).cur_channel as usize].psy_bands
                    [((w + w2) * 16 + g) as usize];

                let [PNS, PNS34, NOR34] =
                    [&mut *PNS, PNS34, NOR34].map(|arr| &mut arr[..usize::from(swb_size)]);

                PNS.fill_with(|| {
                    (*s).random_state = lcg_random((*s).random_state as c_uint);
                    (*s).random_state as c_float
                });

                // (yotam): scalarproduct_float
                let band_energy = PNS.iter().map(|PNS| PNS.powi(2)).sum();

                let scale = noise_amp / sqrtf(band_energy);

                // (yotam): vector_fmac_scalar
                PNS.iter_mut().for_each(|PNS| {
                    *PNS *= scale;
                });
                // (yotam): scalarproduct_float
                let pns_senergy: c_float = PNS.iter().map(|PNS| PNS.powi(2)).sum();

                pns_energy += pns_senergy;

                for (NOR34, coeff) in NOR34
                    .iter_mut()
                    .zip(&(*sce).coeffs[start_c as usize..][..usize::from(swb_size)])
                {
                    *NOR34 = coeff.abs_pow34();
                }
                for (PNS34, PNS) in PNS34.iter_mut().zip(&*PNS) {
                    *PNS34 = PNS.abs_pow34();
                }

                dist1 += quantize_band_cost(
                    s,
                    &mut *((*sce).coeffs).as_mut_ptr().offset(start_c as isize),
                    NOR34.as_mut_ptr(),
                    swb_size.into(),
                    (*sce).sf_idx[((w + w2) * 16 + g) as usize],
                    (*sce).band_alt[((w + w2) * 16 + g) as usize] as c_int,
                    lambda / band.threshold,
                    f32::INFINITY,
                    ptr::null_mut(),
                    ptr::null_mut(),
                );
                // Estimate rd on average as 5 bits for SF, 4 for the CB, plus spread energy *
                // lambda/thr
                dist2 += band.energy / (band.spread * band.spread) * lambda * dist_thresh
                    / band.threshold;
            }
            dist2 += if g != 0 && (*sce).band_type[(w * 16 + g - 1) as usize] == NOISE_BT {
                5.
            } else {
                9.
            };
            let energy_ratio = pns_tgt_energy / pns_energy; // Compensates for quantization error
            (*sce).pns_ener[(w * 16 + g) as usize] = energy_ratio * pns_tgt_energy;
            if (*sce).zeroes[(w * 16 + g) as usize]
                || (*sce).band_alt[(w * 16 + g) as usize] as u64 == 0
                || energy_ratio > 0.85 && energy_ratio < 1.25 && dist2 < dist1
            {
                (*sce).band_type[(w * 16 + g) as usize] = NOISE_BT;
                (*sce).zeroes[(w * 16 + g) as usize] = false;
                prev = noise_sfi;
            } else if !(*sce).zeroes[(w * 16 + g) as usize] {
                prev_sf = (*sce).sf_idx[(w * 16 + g) as usize];
            }
        }
    }
}

#[ffmpeg_src(file = "libavcodec/aaccoder.c", lines = 907..=976, name = "mark_pns")]
pub(crate) unsafe fn mark(
    mut s: *mut AACEncContext,
    mut avctx: *mut AVCodecContext,
    mut sce: *mut SingleChannelElement,
) {
    let mut wlen: c_int = 1024 / (*sce).ics.num_windows;
    let lambda: c_float = (*s).lambda;
    let freq_mult = freq_mult((*avctx).sample_rate, wlen);
    let spread_threshold = spread_threshold(lambda);
    let pns_transient_energy_r = pns_transient_energy_r(lambda);
    let cutoff = cutoff(avctx, lambda, wlen);
    (*sce).band_alt = (*sce).band_type;
    for WindowedIteration { w, group_len } in (*sce).ics.iter_windows() {
        for (g, &swb_offset) in (*sce).ics.swb_offset[..(*sce).ics.num_swb as usize]
            .iter()
            .enumerate()
        {
            let g = g as c_int;
            let start = c_int::from(swb_offset);
            let freq: c_float = start as c_float * freq_mult;
            let freq_boost = freq_boost(freq);

            if freq < NOISE_LOW_LIMIT || start >= cutoff {
                (*sce).can_pns[(w * 16 + g) as usize] = false;
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
            } = reduce_bands(
                &(*s).psy.ch[(*s).cur_channel as usize].psy_bands[(w * 16 + g) as usize..],
                group_len,
            );

            // PNS is acceptable when all of these are true:
            // 1. high spread energy (noise-like band)
            // 2. near-threshold energy (high PE means the random nature of PNS content will
            //    be noticed)
            // 3. on short window groups, all windows have similar energy (variations in
            //    energy would be destroyed by PNS)
            (*sce).pns_ener[(w * 16 + g) as usize] = sfb_energy;
            (*sce).can_pns[(w * 16 + g) as usize] = !(sfb_energy
                < threshold * sqrtf(1.5 / freq_boost)
                || spread < spread_threshold
                || min_energy < pns_transient_energy_r * max_energy);
        }
    }
}
