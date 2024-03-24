use std::ptr;

use ffi::codec::AVCodecContext;
use ffmpeg_src_macro::ffmpeg_src;
use libc::{c_double, c_float, c_int, c_long, c_uchar, c_uint};

use super::{ff_init_nextband_map, math::lcg_random, quantize_band_cost, sfdelta_can_remove_band};
use crate::{
    aac::{
        encoder::{abs_pow34_v, ctx::AACEncContext},
        psy_model::cutoff_from_bitrate,
        tables::POW_SF_TABLES,
    },
    common::*,
    types::*,
};

/// Frequency in Hz for lower limit of noise substitution
#[ffmpeg_src(file = "libavcodec/aaccoder_twoloop.h", lines = 54)]
const NOISE_LOW_LIMIT: c_float = 4000.;

#[ffmpeg_src(file = "libavcodec/aaccoder.c", lines = 765..=905, name = "search_for_pns")]
pub(crate) unsafe fn search(
    mut s: *mut AACEncContext,
    mut avctx: *mut AVCodecContext,
    mut sce: *mut SingleChannelElement,
) {
    let mut band: *mut FFPsyBand = ptr::null_mut::<FFPsyBand>();
    let mut w: c_int = 0;
    let mut g: c_int = 0;
    let mut w2: c_int = 0;
    let mut i: c_int = 0;
    let mut wlen: c_int = 1024 / (*sce).ics.num_windows;
    let mut cutoff: c_int = 0;

    let [PNS, PNS34, NOR34] = [0, 1, 3].map(|i| (*s).scoefs[128 * i..].as_mut_ptr());

    let mut nextband: [c_uchar; 128] = [0; 128];
    let lambda: c_float = (*s).lambda;
    let freq_mult: c_float = (*avctx).sample_rate as c_float * 0.5 / wlen as c_float;
    let thr_mult: c_float = 1.948 * (100. / lambda);
    let spread_threshold: c_float = if 0.75
        > 0.9
            * (if 0.5 > lambda / 100. {
                0.5
            } else {
                lambda / 100.
            }) {
        0.9 * (if 0.5 > lambda / 100. {
            0.5
        } else {
            lambda / 100.
        })
    } else {
        0.75
    };
    let dist_bias: c_float = (4. * 120. / lambda).clamp(0.25, 4.);
    let pns_transient_energy_r: c_float = if 0.7 > lambda / 140. {
        lambda / 140.
    } else {
        0.7
    };
    let mut refbits: c_int = ((*avctx).bit_rate as c_double * 1024.
        / (*avctx).sample_rate as c_double
        / (if (*avctx).flags & AV_CODEC_FLAG_QSCALE != 0 {
            2.
        } else {
            (*avctx).ch_layout.nb_channels as c_float
        }) as c_double
        * (lambda / 120.) as c_double) as c_int;
    let mut rate_bandwidth_multiplier: c_float = 1.5;
    let mut prev: c_int = -1000;
    let mut prev_sf: c_int = -1;
    let mut frame_bit_rate: c_int = (if (*avctx).flags & AV_CODEC_FLAG_QSCALE != 0 {
        refbits as c_float * rate_bandwidth_multiplier * (*avctx).sample_rate as c_float / 1024.
    } else {
        ((*avctx).bit_rate / (*avctx).ch_layout.nb_channels as c_long) as c_float
    }) as c_int;
    frame_bit_rate = (frame_bit_rate as c_float * 1.15) as c_int;
    let bandwidth = if (*avctx).cutoff > 0 {
        (*avctx).cutoff
    } else {
        3000.max(cutoff_from_bitrate(frame_bit_rate, 1, (*avctx).sample_rate))
    };
    cutoff = bandwidth * 2 * wlen / (*avctx).sample_rate;
    (*sce).band_alt = (*sce).band_type;
    ff_init_nextband_map(sce, nextband.as_mut_ptr());
    w = 0;
    while w < (*sce).ics.num_windows {
        let mut wstart: c_int = w * 128;
        let mut current_block_67: u64;
        g = 0;
        while g < (*sce).ics.num_swb {
            let mut noise_sfi: c_int = 0;
            let mut dist1: c_float = 0.;
            let mut dist2: c_float = 0.;
            let mut noise_amp: c_float = 0.;
            let mut pns_energy: c_float = 0.;
            let mut pns_tgt_energy: c_float = 0.;
            let mut energy_ratio: c_float = 0.;
            let mut dist_thresh: c_float = 0.;
            let mut sfb_energy: c_float = 0.;
            let mut threshold: c_float = 0.;
            let mut spread: c_float = 2.;
            let mut min_energy: c_float = -1.;
            let mut max_energy: c_float = 0.;
            let start: c_int = wstart + *((*sce).ics.swb_offset).offset(g as isize) as c_int;
            let freq: c_float = (start - wstart) as c_float * freq_mult;
            let freq_boost = (0.88 * freq / NOISE_LOW_LIMIT).max(1.);
            if freq < NOISE_LOW_LIMIT || start - wstart >= cutoff {
                if (*sce).zeroes[(w * 16 + g) as usize] == 0 {
                    prev_sf = (*sce).sf_idx[(w * 16 + g) as usize];
                }
            } else {
                w2 = 0;
                while w2 < (*sce).ics.group_len[w as usize] as c_int {
                    band = &mut (*s).psy.ch[(*s).cur_channel as usize].psy_bands
                        [((w + w2) * 16 + g) as usize] as *mut FFPsyBand;
                    sfb_energy += (*band).energy;
                    spread = spread.min((*band).spread);
                    threshold += (*band).threshold;
                    if w2 == 0 {
                        max_energy = (*band).energy;
                        min_energy = max_energy;
                    } else {
                        min_energy = min_energy.min((*band).energy);
                        max_energy = max_energy.max((*band).energy);
                    }
                    w2 += 1;
                    w2;
                }

                // Ramps down at ~8000Hz and loosens the dist threshold
                dist_thresh = (2.5 * NOISE_LOW_LIMIT / freq).clamp(0.5, 2.5) * dist_bias;

                // PNS is acceptable when all of these are true:
                // 1. high spread energy (noise-like band)
                // 2. near-threshold energy (high PE means the random nature of PNS content
                // will be noticed)
                // 3. on short window groups, all windows have similar energy (variations in
                // energy would be destroyed by PNS)
                //
                // At this stage, point 2 is relaxed for zeroed bands near
                // the noise threshold (hole avoidance is more important)
                if (*sce).zeroes[(w * 16 + g) as usize] == 0
                    && !sfdelta_can_remove_band(sce, nextband.as_mut_ptr(), prev_sf, w * 16 + g)
                    || ((*sce).zeroes[(w * 16 + g) as usize] as c_int != 0
                        || (*sce).band_alt[(w * 16 + g) as usize] as u64 == 0)
                        && sfb_energy < threshold * sqrtf(1. / freq_boost)
                    || spread < spread_threshold
                    || (*sce).zeroes[(w * 16 + g) as usize] == 0
                        && (*sce).band_alt[(w * 16 + g) as usize] as c_uint != 0
                        && sfb_energy > threshold * thr_mult * freq_boost
                    || min_energy < pns_transient_energy_r * max_energy
                {
                    (*sce).pns_ener[(w * 16 + g) as usize] = sfb_energy;
                    if (*sce).zeroes[(w * 16 + g) as usize] == 0 {
                        prev_sf = (*sce).sf_idx[(w * 16 + g) as usize];
                    }
                } else {
                    pns_tgt_energy = sfb_energy
                        * (if 1. > spread * spread {
                            spread * spread
                        } else {
                            1.
                        });
                    noise_sfi = av_clip_c(roundf(log2f(pns_tgt_energy) * 2.) as c_int, -100, 155);
                    noise_amp = -POW_SF_TABLES.pow2[(noise_sfi + 200) as usize];
                    if prev != -1000 {
                        let mut noise_sfdiff: c_int = noise_sfi - prev + 60;
                        if !(0..=2 * 60).contains(&noise_sfdiff) {
                            if (*sce).zeroes[(w * 16 + g) as usize] == 0 {
                                prev_sf = (*sce).sf_idx[(w * 16 + g) as usize];
                            }
                            current_block_67 = 1054647088692577877;
                        } else {
                            current_block_67 = 1847472278776910194;
                        }
                    } else {
                        current_block_67 = 1847472278776910194;
                    }
                    match current_block_67 {
                        1054647088692577877 => {}
                        _ => {
                            w2 = 0;
                            while w2 < (*sce).ics.group_len[w as usize] as c_int {
                                let mut band_energy: c_float = 0.;
                                let mut scale: c_float = 0.;
                                let mut pns_senergy: c_float = 0.;
                                let start_c: c_int = (w + w2) * 128
                                    + *((*sce).ics.swb_offset).offset(g as isize) as c_int;
                                band = &mut (*s).psy.ch[(*s).cur_channel as usize].psy_bands
                                    [((w + w2) * 16 + g) as usize]
                                    as *mut FFPsyBand;
                                i = 0;
                                while i < *((*sce).ics.swb_sizes).offset(g as isize) as c_int {
                                    (*s).random_state = lcg_random((*s).random_state as c_uint);
                                    *PNS.offset(i as isize) = (*s).random_state as c_float;
                                    i += 1;
                                    i;
                                }
                                band_energy = ((*(*s).fdsp).scalarproduct_float)
                                    .expect("non-null function pointer")(
                                    PNS,
                                    PNS,
                                    *((*sce).ics.swb_sizes).offset(g as isize) as c_int,
                                );
                                scale = noise_amp / sqrtf(band_energy);
                                ((*(*s).fdsp).vector_fmul_scalar)
                                    .expect("non-null function pointer")(
                                    PNS,
                                    PNS,
                                    scale,
                                    *((*sce).ics.swb_sizes).offset(g as isize) as c_int,
                                );
                                pns_senergy = ((*(*s).fdsp).scalarproduct_float)
                                    .expect("non-null function pointer")(
                                    PNS,
                                    PNS,
                                    *((*sce).ics.swb_sizes).offset(g as isize) as c_int,
                                );
                                pns_energy += pns_senergy;
                                abs_pow34_v(
                                    NOR34,
                                    &mut *((*sce).coeffs).as_mut_ptr().offset(start_c as isize),
                                    *((*sce).ics.swb_sizes).offset(g as isize) as c_int,
                                );
                                abs_pow34_v(
                                    PNS34,
                                    PNS,
                                    *((*sce).ics.swb_sizes).offset(g as isize) as c_int,
                                );
                                dist1 += quantize_band_cost(
                                    s,
                                    &mut *((*sce).coeffs).as_mut_ptr().offset(start_c as isize),
                                    NOR34,
                                    *((*sce).ics.swb_sizes).offset(g as isize) as c_int,
                                    (*sce).sf_idx[((w + w2) * 16 + g) as usize],
                                    (*sce).band_alt[((w + w2) * 16 + g) as usize] as c_int,
                                    lambda / (*band).threshold,
                                    ::core::f32::INFINITY,
                                    ptr::null_mut::<c_int>(),
                                    ptr::null_mut::<c_float>(),
                                );
                                dist2 += (*band).energy / ((*band).spread * (*band).spread)
                                    * lambda
                                    * dist_thresh
                                    / (*band).threshold;
                                w2 += 1;
                                w2;
                            }
                            if g != 0
                                && (*sce).band_type[(w * 16 + g - 1) as usize] as c_uint
                                    == NOISE_BT as c_int as c_uint
                            {
                                dist2 += 5.;
                            } else {
                                dist2 += 9.;
                            }
                            energy_ratio = pns_tgt_energy / pns_energy;
                            (*sce).pns_ener[(w * 16 + g) as usize] = energy_ratio * pns_tgt_energy;
                            if (*sce).zeroes[(w * 16 + g) as usize] as c_int != 0
                                || (*sce).band_alt[(w * 16 + g) as usize] as u64 == 0
                                || energy_ratio > 0.85 && energy_ratio < 1.25 && dist2 < dist1
                            {
                                (*sce).band_type[(w * 16 + g) as usize] = NOISE_BT;
                                (*sce).zeroes[(w * 16 + g) as usize] = 0;
                                prev = noise_sfi;
                            } else if (*sce).zeroes[(w * 16 + g) as usize] == 0 {
                                prev_sf = (*sce).sf_idx[(w * 16 + g) as usize];
                            }
                        }
                    }
                }
            }
            g += 1;
            g;
        }
        w += (*sce).ics.group_len[w as usize] as c_int;
    }
}

#[ffmpeg_src(file = "libavcodec/aaccoder.c", lines = 907..=976, name = "mark_pns")]
pub(crate) unsafe fn mark(
    mut s: *mut AACEncContext,
    mut avctx: *mut AVCodecContext,
    mut sce: *mut SingleChannelElement,
) {
    let mut band: *mut FFPsyBand = ptr::null_mut::<FFPsyBand>();
    let mut w: c_int = 0;
    let mut g: c_int = 0;
    let mut w2: c_int = 0;
    let mut wlen: c_int = 1024 / (*sce).ics.num_windows;
    let mut cutoff: c_int = 0;
    let lambda: c_float = (*s).lambda;
    let freq_mult: c_float = (*avctx).sample_rate as c_float * 0.5 / wlen as c_float;
    let spread_threshold: c_float = if 0.75
        > 0.9
            * (if 0.5 > lambda / 100. {
                0.5
            } else {
                lambda / 100.
            }) {
        0.9 * (if 0.5 > lambda / 100. {
            0.5
        } else {
            lambda / 100.
        })
    } else {
        0.75
    };
    let pns_transient_energy_r: c_float = if 0.7 > lambda / 140. {
        lambda / 140.
    } else {
        0.7
    };
    let mut refbits: c_int = ((*avctx).bit_rate as c_double * 1024.
        / (*avctx).sample_rate as c_double
        / (if (*avctx).flags & AV_CODEC_FLAG_QSCALE != 0 {
            2.
        } else {
            (*avctx).ch_layout.nb_channels as c_float
        }) as c_double
        * (lambda / 120.) as c_double) as c_int;
    let mut rate_bandwidth_multiplier: c_float = 1.5;
    let mut frame_bit_rate: c_int = (if (*avctx).flags & AV_CODEC_FLAG_QSCALE != 0 {
        refbits as c_float * rate_bandwidth_multiplier * (*avctx).sample_rate as c_float / 1024.
    } else {
        ((*avctx).bit_rate / (*avctx).ch_layout.nb_channels as c_long) as c_float
    }) as c_int;
    frame_bit_rate = (frame_bit_rate as c_float * 1.15) as c_int;
    let bandwidth = if (*avctx).cutoff > 0 {
        (*avctx).cutoff
    } else {
        3000.max(cutoff_from_bitrate(frame_bit_rate, 1, (*avctx).sample_rate))
    };
    cutoff = bandwidth * 2 * wlen / (*avctx).sample_rate;
    (*sce).band_alt = (*sce).band_type;
    w = 0;
    while w < (*sce).ics.num_windows {
        g = 0;
        while g < (*sce).ics.num_swb {
            let mut sfb_energy: c_float = 0.;
            let mut threshold: c_float = 0.;
            let mut spread: c_float = 2.;
            let mut min_energy: c_float = -1.;
            let mut max_energy: c_float = 0.;
            let start: c_int = *((*sce).ics.swb_offset).offset(g as isize) as c_int;
            let freq: c_float = start as c_float * freq_mult;
            let freq_boost: c_float = if 0.88 * freq / 4000. > 1. {
                0.88 * freq / 4000.
            } else {
                1.
            };
            if freq < 4000. || start >= cutoff {
                (*sce).can_pns[(w * 16 + g) as usize] = 0;
            } else {
                w2 = 0;
                while w2 < (*sce).ics.group_len[w as usize] as c_int {
                    band = &mut (*s).psy.ch[(*s).cur_channel as usize].psy_bands
                        [((w + w2) * 16 + g) as usize] as *mut FFPsyBand;
                    sfb_energy += (*band).energy;
                    spread = if spread > (*band).spread {
                        (*band).spread
                    } else {
                        spread
                    };
                    threshold += (*band).threshold;
                    if w2 == 0 {
                        max_energy = (*band).energy;
                        min_energy = max_energy;
                    } else {
                        min_energy = if min_energy > (*band).energy {
                            (*band).energy
                        } else {
                            min_energy
                        };
                        max_energy = if max_energy > (*band).energy {
                            max_energy
                        } else {
                            (*band).energy
                        };
                    }
                    w2 += 1;
                    w2;
                }
                (*sce).pns_ener[(w * 16 + g) as usize] = sfb_energy;
                if sfb_energy < threshold * sqrtf(1.5 / freq_boost)
                    || spread < spread_threshold
                    || min_energy < pns_transient_energy_r * max_energy
                {
                    (*sce).can_pns[(w * 16 + g) as usize] = 0;
                } else {
                    (*sce).can_pns[(w * 16 + g) as usize] = 1;
                }
            }
            g += 1;
            g;
        }
        w += (*sce).ics.group_len[w as usize] as c_int;
    }
}
