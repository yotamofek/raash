#![allow(
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use std::alloc::{alloc_zeroed, Layout};

use array_util::{WindowedArray, W};
use ffi::codec::AVCodecContext;
use ffmpeg_src_macro::ffmpeg_src;
use libc::{c_double, c_float, c_int, c_long, c_uchar, c_uint};

use super::{
    WindowSequence, EIGHT_SHORT_SEQUENCE, LONG_START_SEQUENCE, LONG_STOP_SEQUENCE,
    ONLY_LONG_SEQUENCE,
};
use crate::{common::*, psy_model::find_group, types::*};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
// TODO: remove explicit repr and discriminants when `AacPsyBand` has a default impl
#[repr(u32)]
enum AvoidHoles {
    Active = 2,
    Inactive = 1,
    #[default]
    None = 0,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct AacPsyBand {
    energy: c_float,
    thr: c_float,
    thr_quiet: c_float,
    nz_lines: c_float,
    active_lines: c_float,
    pe: c_float,
    pe_const: c_float,
    norm_fac: c_float,
    avoid_holes: AvoidHoles,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct AacPsyChannel {
    band: WindowedArray<[AacPsyBand; 128], 16>,
    prev_band: [AacPsyBand; 128],
    win_energy: c_float,
    iir_state: [c_float; 2],
    next_grouping: c_uchar,
    next_window_seq: WindowSequence,
    attack_threshold: c_float,
    prev_energy_subshort: [c_float; 24],
    prev_attack: c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct AacPsyCoeffs {
    ath: c_float,
    barks: c_float,
    spread_low: [c_float; 2],
    spread_hi: [c_float; 2],
    min_snr: c_float,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct AacPsyContext {
    chan_bitrate: c_int,
    frame_bits: c_int,
    fill_level: c_int,
    pe: C2RustUnnamed_2,
    psy_coef: [[AacPsyCoeffs; 64]; 2],
    ch: *mut AacPsyChannel,
    global_quality: c_float,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct C2RustUnnamed_2 {
    min: c_float,
    max: c_float,
    previous: c_float,
    correction: c_float,
}

#[derive(Copy, Clone)]
pub(crate) struct PsyLamePreset {
    quality: c_int,
    st_lrm: c_float,
}

impl PsyLamePreset {
    pub(crate) const fn new(quality: c_int, st_lrm: c_float) -> Self {
        Self { quality, st_lrm }
    }
}

static ABR_MAP: [PsyLamePreset; 13] = [
    PsyLamePreset::new(8, 6.60),
    PsyLamePreset::new(16, 6.60),
    PsyLamePreset::new(24, 6.60),
    PsyLamePreset::new(32, 6.60),
    PsyLamePreset::new(40, 6.60),
    PsyLamePreset::new(48, 6.60),
    PsyLamePreset::new(56, 6.60),
    PsyLamePreset::new(64, 6.40),
    PsyLamePreset::new(80, 6.00),
    PsyLamePreset::new(96, 5.60),
    PsyLamePreset::new(112, 5.20),
    PsyLamePreset::new(128, 5.20),
    PsyLamePreset::new(160, 5.20),
];

static VBR_MAP: [PsyLamePreset; 11] = [
    PsyLamePreset::new(0, 4.20),
    PsyLamePreset::new(1, 4.20),
    PsyLamePreset::new(2, 4.20),
    PsyLamePreset::new(3, 4.20),
    PsyLamePreset::new(4, 4.20),
    PsyLamePreset::new(5, 4.20),
    PsyLamePreset::new(6, 4.20),
    PsyLamePreset::new(7, 4.20),
    PsyLamePreset::new(8, 4.20),
    PsyLamePreset::new(9, 4.20),
    PsyLamePreset::new(10, 4.20),
];

static FIR_COEFFS: [c_float; 10] = [
    -8.65163e-18 * 2.,
    -0.00851586 * 2.,
    -6.74764e-18 * 2.,
    0.0209036 * 2.,
    -3.36639e-17 * 2.,
    -0.0438162 * 2.,
    -1.54175e-17 * 2.,
    0.0931738 * 2.,
    -5.52212e-17 * 2.,
    -0.313819 * 2.,
];

#[ffmpeg_src(file = "libavcodec/psymodel.h", lines = 41..=45, name = "AAC_CUTOFF")]
unsafe fn cutoff(ctx: *const AVCodecContext) -> c_int {
    if (*ctx).flags.qscale() {
        (*ctx).sample_rate / 2
    } else {
        cutoff_from_bitrate(
            (*ctx).bit_rate.try_into().unwrap(),
            (*ctx).ch_layout.nb_channels,
            (*ctx).sample_rate,
        )
    }
}

/// cutoff for VBR is purposely increased, since LP filtering actually
/// hinders VBR performance rather than the opposite
#[ffmpeg_src(file = "libavcodec/psymodel.h", lines = 35..=40, name = "AAC_CUTOFF_FROM_BITRATE")]
pub(crate) fn cutoff_from_bitrate(bit_rate: c_int, channels: c_int, sample_rate: c_int) -> c_int {
    if bit_rate == 0 {
        return sample_rate / 2;
    }

    (bit_rate / channels / 5)
        .max(bit_rate / channels * 15 / 32 - 5500)
        .min(3000 + bit_rate / channels / 4)
        .min(12000 + bit_rate / channels / 16)
        .min(22000)
        .min(sample_rate / 2)
}

fn lame_calc_attack_threshold(bitrate: c_int) -> c_float {
    let mut lower_range: c_int = 12;
    let mut upper_range: c_int = 12;
    let mut lower_range_kbps: c_int = ABR_MAP[12].quality;
    let mut upper_range_kbps: c_int = ABR_MAP[12].quality;
    let mut i: c_int = 0;
    i = 1;
    while i < 13 {
        if (if bitrate > ABR_MAP[i as usize].quality {
            bitrate
        } else {
            ABR_MAP[i as usize].quality
        }) != bitrate
        {
            upper_range = i;
            upper_range_kbps = ABR_MAP[i as usize].quality;
            lower_range = i - 1;
            lower_range_kbps = ABR_MAP[(i - 1) as usize].quality;
            break;
        } else {
            i += 1;
            i;
        }
    }
    if upper_range_kbps - bitrate > bitrate - lower_range_kbps {
        return ABR_MAP[lower_range as usize].st_lrm;
    }
    ABR_MAP[upper_range as usize].st_lrm
}

#[cold]
unsafe fn lame_window_init(mut ctx: *mut AacPsyContext, mut avctx: *mut AVCodecContext) {
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    i = 0;
    while i < (*avctx).ch_layout.nb_channels {
        let mut pch: *mut AacPsyChannel =
            &mut *((*ctx).ch).offset(i as isize) as *mut AacPsyChannel;
        if (*avctx).flags.qscale() {
            (*pch).attack_threshold =
                VBR_MAP[av_clip_c((*avctx).global_quality / 118, 0, 10) as usize].st_lrm;
        } else {
            (*pch).attack_threshold = lame_calc_attack_threshold(
                ((*avctx).bit_rate / (*avctx).ch_layout.nb_channels as c_long / 1000 as c_long)
                    as c_int,
            );
        }
        j = 0;
        while j < 8 * 3 {
            (*pch).prev_energy_subshort[j as usize] = 10.;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}

#[cold]
fn calc_bark(mut f: c_float) -> c_float {
    13.3 * (0.00076 * f).atan() + 3.5 * (f / 7500. * (f / 7500.)).atan()
}

#[cold]
fn ath(mut f: c_float, mut add: c_float) -> c_float {
    f /= 1000.;
    (3.64 * pow(f as c_double, -0.8)
        - 6.8 * exp(-0.6 * (f as c_double - 3.4) * (f as c_double - 3.4))
        + 6. * exp(-0.15 * (f as c_double - 8.7) * (f as c_double - 8.7))
        + (0.6 + 0.04 * add as c_double)
            * 0.001
            * f as c_double
            * f as c_double
            * f as c_double
            * f as c_double) as c_float
}

#[cold]
pub(crate) unsafe extern "C" fn psy_3gpp_init(mut ctx: *mut FFPsyContext) -> c_int {
    let mut pctx: *mut AacPsyContext = std::ptr::null_mut::<AacPsyContext>();
    let mut bark: c_float = 0.;
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut g: c_int = 0;
    let mut start: c_int = 0;
    let mut prev: c_float = 0.;
    let mut minscale: c_float = 0.;
    let mut minath: c_float = 0.;
    let mut minsnr: c_float = 0.;
    let mut pe_min: c_float = 0.;
    let mut chan_bitrate: c_int = ((*(*ctx).avctx).bit_rate as c_float
        / (if (*(*ctx).avctx).flags.qscale() {
            2.
        } else {
            (*(*ctx).avctx).ch_layout.nb_channels as c_float
        })) as c_int;
    let bandwidth: c_int = (if (*ctx).cutoff != 0 {
        (*ctx).cutoff
    } else {
        cutoff((*ctx).avctx)
    }) as c_int;
    let num_bark: c_float = calc_bark(bandwidth as c_float);
    if bandwidth <= 0 {
        return -22;
    }
    (*ctx).model_priv_data = alloc_zeroed(Layout::new::<AacPsyContext>()).cast();
    if ((*ctx).model_priv_data).is_null() {
        return -12;
    }
    pctx = (*ctx).model_priv_data as *mut AacPsyContext;
    (*pctx).global_quality = (if (*(*ctx).avctx).global_quality != 0 {
        (*(*ctx).avctx).global_quality
    } else {
        120
    }) as c_float
        * 0.01;
    if (*(*ctx).avctx).flags.qscale() {
        chan_bitrate = (chan_bitrate as c_double / 120.
            * (if (*(*ctx).avctx).global_quality != 0 {
                (*(*ctx).avctx).global_quality
            } else {
                120
            }) as c_double) as c_int;
    }
    (*pctx).chan_bitrate = chan_bitrate;
    (*pctx).frame_bits = if 2560 > chan_bitrate * 1024 / (*(*ctx).avctx).sample_rate {
        chan_bitrate * 1024 / (*(*ctx).avctx).sample_rate
    } else {
        2560
    };
    (*pctx).pe.min =
        8. * 1024. * bandwidth as c_float / ((*(*ctx).avctx).sample_rate as c_float * 2.);
    (*pctx).pe.max =
        12. * 1024. * bandwidth as c_float / ((*(*ctx).avctx).sample_rate as c_float * 2.);
    (*ctx).bitres.size = 6144 - (*pctx).frame_bits;
    (*ctx).bitres.size -= (*ctx).bitres.size % 8;
    (*pctx).fill_level = (*ctx).bitres.size;
    minath = ath((3410. - 0.733 * 4.) as c_float, 4.);
    j = 0;
    while j < 2 {
        let mut coeffs: *mut AacPsyCoeffs = ((*pctx).psy_coef[j as usize]).as_mut_ptr();
        let mut band_sizes: *const c_uchar = ((*ctx).bands)[j as usize].as_ptr();
        let mut line_to_frequency: c_float =
            (*(*ctx).avctx).sample_rate as c_float / (if j != 0 { 256. } else { 2048. });
        let mut avg_chan_bits: c_float = chan_bitrate as c_float
            * (if j != 0 { 128. } else { 1024. })
            / (*(*ctx).avctx).sample_rate as c_float;
        let mut bark_pe: c_float = 0.024 * (avg_chan_bits * 1.18) / num_bark;
        let mut en_spread_low: c_float = if j != 0 { 2. } else { 3. };
        let mut en_spread_hi: c_float = if j != 0 || chan_bitrate as c_float <= 22. {
            1.5
        } else {
            2.
        };
        i = 0;
        prev = 0.;
        g = 0;
        while g < ((*ctx).num_bands)[j as usize] {
            i += *band_sizes.offset(g as isize) as c_int;
            bark = calc_bark((i - 1) as c_float * line_to_frequency);
            (*coeffs.offset(g as isize)).barks = ((bark + prev) as c_double / 2.) as c_float;
            prev = bark;
            g += 1;
            g;
        }
        g = 0;
        while g < ((*ctx).num_bands)[j as usize] - 1 {
            let mut coeff: *mut AacPsyCoeffs = &mut *coeffs.offset(g as isize) as *mut AacPsyCoeffs;
            let mut bark_width: c_float =
                (*coeffs.offset((g + 1) as isize)).barks - (*coeffs).barks;
            (*coeff).spread_low[0] = ff_exp10((-bark_width * 3.) as c_double) as c_float;
            (*coeff).spread_hi[0] = ff_exp10((-bark_width * 1.5) as c_double) as c_float;
            (*coeff).spread_low[1] = ff_exp10((-bark_width * en_spread_low) as c_double) as c_float;
            (*coeff).spread_hi[1] = ff_exp10((-bark_width * en_spread_hi) as c_double) as c_float;
            pe_min = bark_pe * bark_width;
            minsnr =
                (exp2((pe_min / *band_sizes.offset(g as isize) as c_int as c_float) as c_double)
                    - 1.5) as c_float;
            (*coeff).min_snr = av_clipf_c(1. / minsnr, 3.1622776e-3f32, 7.943_282e-1_f32);
            g += 1;
            g;
        }
        start = 0;
        g = 0;
        while g < ((*ctx).num_bands)[j as usize] {
            minscale = ath(start as c_float * line_to_frequency, 4.);
            i = 1;
            while i < *band_sizes.offset(g as isize) as c_int {
                minscale = if minscale > ath((start + i) as c_float * line_to_frequency, 4.) {
                    ath((start + i) as c_float * line_to_frequency, 4.)
                } else {
                    minscale
                };
                i += 1;
                i;
            }
            (*coeffs.offset(g as isize)).ath = minscale - minath;
            start += *band_sizes.offset(g as isize) as c_int;
            g += 1;
            g;
        }
        j += 1;
        j;
    }
    (*pctx).ch = alloc_zeroed(
        Layout::array::<AacPsyChannel>((*(*ctx).avctx).ch_layout.nb_channels as usize).unwrap(),
    )
    .cast();
    if ((*pctx).ch).is_null() {
        // TODO: leaks 🚿
        // av_freep(&mut (*ctx).model_priv_data as *mut *mut c_void as *mut c_void);
        return -12;
    }
    lame_window_init(pctx, (*ctx).avctx);
    0
}

const WINDOW_GROUPING: [c_uchar; 9] = [0xb6, 0x6c, 0xd8, 0xb2, 0x66, 0xc6, 0x96, 0x36, 0x36];

unsafe fn calc_bit_demand(
    mut ctx: *mut AacPsyContext,
    mut pe: c_float,
    mut bits: c_int,
    mut size: c_int,
    mut short_window: bool,
) -> c_int {
    #[derive(Default)]
    struct SlopeAdd {
        slope: c_float,
        add: c_float,
    }

    #[derive(Default)]
    struct Info {
        bit_save: SlopeAdd,
        bit_spend: SlopeAdd,
        clip_low: c_float,
        clip_high: c_float,
    }

    fn get_info(short_window: bool) -> Info {
        Info {
            clip_low: 0.2,
            ..if short_window {
                Info {
                    bit_save: SlopeAdd {
                        slope: -0.36363637,
                        add: -0.75,
                    },
                    bit_spend: SlopeAdd {
                        slope: 0.818_181_8,
                        add: -0.261_111_1,
                    },
                    clip_high: 0.75,
                    ..Default::default()
                }
            } else {
                Info {
                    bit_save: SlopeAdd {
                        slope: -0.46666667,
                        add: -0.842_857_1,
                    },
                    bit_spend: SlopeAdd {
                        slope: 0.666_666_7,
                        add: -0.35,
                    },
                    clip_high: 0.95,
                    ..Default::default()
                }
            }
        }
    }

    let Info {
        bit_save:
            SlopeAdd {
                slope: bitsave_slope,
                add: bitsave_add,
            },
        bit_spend:
            SlopeAdd {
                slope: bitspend_slope,
                add: bitspend_add,
            },
        clip_low,
        clip_high,
    } = get_info(short_window);

    let mut clipped_pe: c_float = 0.;
    let mut bit_save: c_float = 0.;
    let mut bit_spend: c_float = 0.;
    let mut bit_factor: c_float = 0.;
    let mut fill_level: c_float = 0.;
    let mut forgetful_min_pe: c_float = 0.;
    (*ctx).fill_level += (*ctx).frame_bits - bits;
    (*ctx).fill_level = av_clip_c((*ctx).fill_level, 0, size);
    fill_level = av_clipf_c(
        (*ctx).fill_level as c_float / size as c_float,
        clip_low,
        clip_high,
    );
    clipped_pe = av_clipf_c(pe, (*ctx).pe.min, (*ctx).pe.max);
    bit_save = (fill_level + bitsave_add) * bitsave_slope;
    bit_spend = (fill_level + bitspend_add) * bitspend_slope;
    bit_factor = 1. - bit_save
        + (bit_spend - bit_save) / ((*ctx).pe.max - (*ctx).pe.min) * (clipped_pe - (*ctx).pe.min);
    (*ctx).pe.max = if pe > (*ctx).pe.max {
        pe
    } else {
        (*ctx).pe.max
    };
    forgetful_min_pe = ((*ctx).pe.min * 511.
        + (if (*ctx).pe.min > pe * (pe / (*ctx).pe.max) {
            (*ctx).pe.min
        } else {
            pe * (pe / (*ctx).pe.max)
        }))
        / (511 + 1) as c_float;
    (*ctx).pe.min = if pe > forgetful_min_pe {
        forgetful_min_pe
    } else {
        pe
    };
    (if (*ctx).frame_bits as c_float * bit_factor
        > (if (*ctx).frame_bits + size - bits > (*ctx).frame_bits / 8 {
            (*ctx).frame_bits + size - bits
        } else {
            (*ctx).frame_bits / 8
        }) as c_float
    {
        (if (*ctx).frame_bits + size - bits > (*ctx).frame_bits / 8 {
            (*ctx).frame_bits + size - bits
        } else {
            (*ctx).frame_bits / 8
        }) as c_float
    } else {
        (*ctx).frame_bits as c_float * bit_factor
    }) as c_int
}

unsafe fn calc_pe_3gpp(mut band: *mut AacPsyBand) -> c_float {
    let mut pe: c_float = 0.;
    let mut a: c_float = 0.;
    (*band).pe = 0.;
    (*band).pe_const = 0.;
    (*band).active_lines = 0.;
    if (*band).energy > (*band).thr {
        a = log2f((*band).energy);
        pe = a - log2f((*band).thr);
        (*band).active_lines = (*band).nz_lines;
        if pe < 3. {
            pe = pe * 0.559_357_3_f32 + 1.3219281;
            a = a * 0.559_357_3_f32 + 1.3219281;
            (*band).active_lines *= 0.559_357_3_f32;
        }
        (*band).pe = pe * (*band).nz_lines;
        (*band).pe_const = a * (*band).nz_lines;
    }
    (*band).pe
}
unsafe fn calc_reduction_3gpp(
    mut a: c_float,
    mut desired_pe: c_float,
    mut pe: c_float,
    mut active_lines: c_float,
) -> c_float {
    let mut thr_avg: c_float = 0.;
    let mut reduction: c_float = 0.;
    if active_lines as c_double == 0. {
        return 0.;
    }
    thr_avg = exp2f((a - pe) / (4. * active_lines));
    reduction = exp2f((a - desired_pe) / (4. * active_lines)) - thr_avg;
    if reduction > 0. {
        reduction
    } else {
        0.
    }
}

unsafe fn calc_reduced_thr_3gpp(
    mut band: *mut AacPsyBand,
    mut min_snr: c_float,
    mut reduction: c_float,
) -> c_float {
    let mut thr: c_float = (*band).thr;
    if (*band).energy > thr {
        thr = sqrtf(thr);
        thr = sqrtf(thr) + reduction;
        thr *= thr;
        thr *= thr;
        if thr > (*band).energy * min_snr && (*band).avoid_holes != AvoidHoles::None {
            thr = if (*band).thr > (*band).energy * min_snr {
                (*band).thr
            } else {
                (*band).energy * min_snr
            };
            (*band).avoid_holes = AvoidHoles::Active;
        }
    }
    thr
}

unsafe fn calc_thr_3gpp(
    mut wi: *const FFPsyWindowInfo,
    num_bands: c_int,
    mut pch: *mut AacPsyChannel,
    mut band_sizes: *const c_uchar,
    mut coefs: *const c_float,
    cutoff: c_int,
) {
    let mut i: c_int = 0;
    let mut w: c_int = 0;
    let mut g: c_int = 0;
    let mut start: c_int = 0;
    let mut wstart: c_int = 0;
    w = 0;
    while w < (*wi).num_windows * 16 {
        wstart = 0;
        g = 0;
        while g < num_bands {
            let mut band: *mut AacPsyBand =
                &mut *((*pch).band).as_mut_ptr().offset((w + g) as isize) as *mut AacPsyBand;
            let mut form_factor: c_float = 0.;
            let mut Temp: c_float = 0.;
            (*band).energy = 0.;
            if wstart < cutoff {
                i = 0;
                while i < *band_sizes.offset(g as isize) as c_int {
                    (*band).energy +=
                        *coefs.offset((start + i) as isize) * *coefs.offset((start + i) as isize);
                    form_factor +=
                        sqrtf(fabs(*coefs.offset((start + i) as isize) as c_double) as c_float);
                    i += 1;
                    i;
                }
            }
            Temp = if (*band).energy > 0. {
                sqrtf(*band_sizes.offset(g as isize) as c_float / (*band).energy)
            } else {
                0.
            };
            (*band).thr = (*band).energy * 0.001258925;
            (*band).nz_lines = form_factor * sqrtf(Temp);
            start += *band_sizes.offset(g as isize) as c_int;
            wstart += *band_sizes.offset(g as isize) as c_int;
            g += 1;
            g;
        }
        w += 16;
    }
}

unsafe fn psy_hp_filter(
    mut firbuf: *const c_float,
    mut hpfsmpl: *mut c_float,
    mut psy_fir_coeffs_0: *const c_float,
) {
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    i = 0;
    while i < 1024 {
        let mut sum1: c_float = 0.;
        let mut sum2: c_float = 0.;
        sum1 = *firbuf.offset((i + (21 - 1) / 2) as isize);
        sum2 = 0.;
        j = 0;
        while j < (21 - 1) / 2 - 1 {
            sum1 += *psy_fir_coeffs_0.offset(j as isize)
                * (*firbuf.offset((i + j) as isize) + *firbuf.offset((i + 21 - j) as isize));
            sum2 += *psy_fir_coeffs_0.offset((j + 1) as isize)
                * (*firbuf.offset((i + j + 1) as isize)
                    + *firbuf.offset((i + 21 - j - 1) as isize));
            j += 2;
        }
        *hpfsmpl.offset(i as isize) = (sum1 + sum2) * 32768.;
        i += 1;
        i;
    }
}
unsafe fn psy_3gpp_analyze_channel(
    mut ctx: *mut FFPsyContext,
    mut channel: c_int,
    mut coefs: *const c_float,
    mut wi: *const FFPsyWindowInfo,
) {
    let mut pctx: *mut AacPsyContext = (*ctx).model_priv_data as *mut AacPsyContext;
    let mut pch: *mut AacPsyChannel =
        &mut *((*pctx).ch).offset(channel as isize) as *mut AacPsyChannel;
    let mut i: c_int = 0;
    let mut w: c_int = 0;
    let mut g: c_int = 0;
    let mut desired_bits: c_float = 0.;
    let mut desired_pe: c_float = 0.;
    let mut delta_pe: c_float = 0.;
    let mut reduction: c_float = f32::NAN;
    let mut spread_en: [c_float; 128] = [0.; 128];
    let mut a: c_float = 0.;
    let mut active_lines: c_float = 0.;
    let mut norm_fac: c_float = 0.;
    let mut pe: c_float = if (*pctx).chan_bitrate > 32000 {
        0.
    } else if 50. > 100. - (*pctx).chan_bitrate as c_float * 100. / 32000. {
        50.
    } else {
        100. - (*pctx).chan_bitrate as c_float * 100. / 32000.
    };
    let num_bands = ((*ctx).num_bands)[((*wi).num_windows == 8) as usize];
    let mut band_sizes = ((*ctx).bands)[((*wi).num_windows == 8) as usize];
    let mut coeffs: *mut AacPsyCoeffs =
        ((*pctx).psy_coef[((*wi).num_windows == 8) as c_int as usize]).as_mut_ptr();
    let avoid_hole_thr: c_float = if (*wi).num_windows == 8 { 0.63 } else { 0.5 };
    let bandwidth: c_int = (if (*ctx).cutoff != 0 {
        (*ctx).cutoff
    } else {
        cutoff((*ctx).avctx)
    }) as c_int;
    let cutoff: c_int = bandwidth * 2048 / (*wi).num_windows / (*(*ctx).avctx).sample_rate;
    calc_thr_3gpp(wi, num_bands, pch, band_sizes.as_ptr(), coefs, cutoff);
    w = 0;
    while w < (*wi).num_windows * 16 {
        let mut bands: *mut AacPsyBand =
            &mut *((*pch).band).as_mut_ptr().offset(w as isize) as *mut AacPsyBand;
        spread_en[0] = (*bands.offset(0)).energy;
        g = 1;
        while g < num_bands {
            (*bands.offset(g as isize)).thr = if (*bands.offset(g as isize)).thr
                > (*bands.offset((g - 1) as isize)).thr * (*coeffs.offset(g as isize)).spread_hi[0]
            {
                (*bands.offset(g as isize)).thr
            } else {
                (*bands.offset((g - 1) as isize)).thr * (*coeffs.offset(g as isize)).spread_hi[0]
            };
            spread_en[(w + g) as usize] = if (*bands.offset(g as isize)).energy
                > spread_en[(w + g - 1) as usize] * (*coeffs.offset(g as isize)).spread_hi[1]
            {
                (*bands.offset(g as isize)).energy
            } else {
                spread_en[(w + g - 1) as usize] * (*coeffs.offset(g as isize)).spread_hi[1]
            };
            g += 1;
            g;
        }
        g = num_bands - 2;
        while g >= 0 {
            (*bands.offset(g as isize)).thr = if (*bands.offset(g as isize)).thr
                > (*bands.offset((g + 1) as isize)).thr * (*coeffs.offset(g as isize)).spread_low[0]
            {
                (*bands.offset(g as isize)).thr
            } else {
                (*bands.offset((g + 1) as isize)).thr * (*coeffs.offset(g as isize)).spread_low[0]
            };
            spread_en[(w + g) as usize] = if spread_en[(w + g) as usize]
                > spread_en[(w + g + 1) as usize] * (*coeffs.offset(g as isize)).spread_low[1]
            {
                spread_en[(w + g) as usize]
            } else {
                spread_en[(w + g + 1) as usize] * (*coeffs.offset(g as isize)).spread_low[1]
            };
            g -= 1;
            g;
        }
        g = 0;
        while g < num_bands {
            let mut band: *mut AacPsyBand = &mut *bands.offset(g as isize) as *mut AacPsyBand;
            (*band).thr = if (*band).thr > (*coeffs.offset(g as isize)).ath {
                (*band).thr
            } else {
                (*coeffs.offset(g as isize)).ath
            };
            (*band).thr_quiet = (*band).thr;
            if !((*wi).window_type[0] == LONG_STOP_SEQUENCE as c_int
                || w == 0 && (*wi).window_type[1] == LONG_START_SEQUENCE as c_int)
            {
                (*band).thr = if 0.01 * (*band).thr
                    > (if (*band).thr > 2. * (*pch).prev_band[(w + g) as usize].thr_quiet {
                        2. * (*pch).prev_band[(w + g) as usize].thr_quiet
                    } else {
                        (*band).thr
                    }) {
                    0.01 * (*band).thr
                } else if (*band).thr > 2. * (*pch).prev_band[(w + g) as usize].thr_quiet {
                    2. * (*pch).prev_band[(w + g) as usize].thr_quiet
                } else {
                    (*band).thr
                };
            }
            pe += calc_pe_3gpp(band);
            a += (*band).pe_const;
            active_lines += (*band).active_lines;
            if spread_en[(w + g) as usize] * avoid_hole_thr > (*band).energy
                || (*coeffs.offset(g as isize)).min_snr > 1.
            {
                (*band).avoid_holes = AvoidHoles::None;
            } else {
                (*band).avoid_holes = AvoidHoles::Inactive;
            }
            g += 1;
            g;
        }
        w += 16;
    }
    (*ctx).ch[channel as usize].entropy = pe;
    if (*(*ctx).avctx).flags.qscale() {
        desired_pe = pe
            * (if (*(*ctx).avctx).global_quality != 0 {
                (*(*ctx).avctx).global_quality
            } else {
                120
            }) as c_float
            / (2. * 2.5 * 120.);
        desired_bits = if 2560. > desired_pe / 1.18 {
            desired_pe / 1.18
        } else {
            2560.
        };
        desired_pe = desired_bits * 1.18;
        if (*ctx).bitres.bits > 0 {
            desired_bits = if 2560. > desired_pe / 1.18 {
                desired_pe / 1.18
            } else {
                2560.
            };
            desired_pe = desired_bits * 1.18;
        }
        (*pctx).pe.max = if pe > (*pctx).pe.max {
            pe
        } else {
            (*pctx).pe.max
        };
        (*pctx).pe.min = if pe > (*pctx).pe.min {
            (*pctx).pe.min
        } else {
            pe
        };
    } else {
        desired_bits = calc_bit_demand(
            pctx,
            pe,
            (*ctx).bitres.bits,
            (*ctx).bitres.size,
            (*wi).num_windows == 8,
        ) as c_float;
        desired_pe = desired_bits * 1.18;
        if (*ctx).bitres.bits > 0 {
            desired_pe *= av_clipf_c(
                (*pctx).pe.previous / ((*ctx).bitres.bits as c_float * 1.18),
                0.85,
                1.15,
            );
        }
    }
    (*pctx).pe.previous = desired_bits * 1.18;
    (*ctx).bitres.alloc = desired_bits as c_int;
    if desired_pe < pe {
        w = 0;
        while w < (*wi).num_windows * 16 {
            reduction = calc_reduction_3gpp(a, desired_pe, pe, active_lines);
            pe = 0.;
            a = 0.;
            active_lines = 0.;
            g = 0;
            while g < num_bands {
                let mut band_0: *mut AacPsyBand =
                    &mut *((*pch).band).as_mut_ptr().offset((w + g) as isize) as *mut AacPsyBand;
                (*band_0).thr =
                    calc_reduced_thr_3gpp(band_0, (*coeffs.offset(g as isize)).min_snr, reduction);
                pe += calc_pe_3gpp(band_0);
                a += (*band_0).pe_const;
                active_lines += (*band_0).active_lines;
                g += 1;
                g;
            }
            w += 16;
        }
        i = 0;
        while i < 2 {
            let mut pe_no_ah: c_float = 0.;
            let mut desired_pe_no_ah: c_float = 0.;
            a = 0.;
            active_lines = a;
            w = 0;
            while w < (*wi).num_windows * 16 {
                g = 0;
                while g < num_bands {
                    let mut band_1: *mut AacPsyBand =
                        &mut *((*pch).band).as_mut_ptr().offset((w + g) as isize)
                            as *mut AacPsyBand;
                    if (*band_1).avoid_holes != AvoidHoles::Active {
                        pe_no_ah += (*band_1).pe;
                        a += (*band_1).pe_const;
                        active_lines += (*band_1).active_lines;
                    }
                    g += 1;
                    g;
                }
                w += 16;
            }
            desired_pe_no_ah = if desired_pe - (pe - pe_no_ah) > 0. {
                desired_pe - (pe - pe_no_ah)
            } else {
                0.
            };
            if active_lines > 0. {
                reduction = calc_reduction_3gpp(a, desired_pe_no_ah, pe_no_ah, active_lines);
            }
            pe = 0.;
            w = 0;
            while w < (*wi).num_windows * 16 {
                g = 0;
                while g < num_bands {
                    let mut band_2: *mut AacPsyBand =
                        &mut *((*pch).band).as_mut_ptr().offset((w + g) as isize)
                            as *mut AacPsyBand;
                    if active_lines > 0. {
                        (*band_2).thr = calc_reduced_thr_3gpp(
                            band_2,
                            (*coeffs.offset(g as isize)).min_snr,
                            reduction,
                        );
                    }
                    pe += calc_pe_3gpp(band_2);
                    if (*band_2).thr > 0. {
                        (*band_2).norm_fac = (*band_2).active_lines / (*band_2).thr;
                    } else {
                        (*band_2).norm_fac = 0.;
                    }
                    norm_fac += (*band_2).norm_fac;
                    g += 1;
                    g;
                }
                w += 16;
            }
            delta_pe = desired_pe - pe;
            if fabs(delta_pe as c_double) > (0.05 * desired_pe) as c_double {
                break;
            }
            i += 1;
            i;
        }
        if pe < 1.15 * desired_pe {
            norm_fac = if norm_fac != 0. { 1. / norm_fac } else { 0. };
            w = 0;
            while w < (*wi).num_windows * 16 {
                g = 0;
                while g < num_bands {
                    let mut band_3: *mut AacPsyBand =
                        &mut *((*pch).band).as_mut_ptr().offset((w + g) as isize)
                            as *mut AacPsyBand;
                    if (*band_3).active_lines > 0.5 {
                        let mut delta_sfb_pe: c_float = (*band_3).norm_fac * norm_fac * delta_pe;
                        let mut thr: c_float = (*band_3).thr;
                        thr *= exp2f(delta_sfb_pe / (*band_3).active_lines);
                        if thr > (*coeffs.offset(g as isize)).min_snr * (*band_3).energy
                            && (*band_3).avoid_holes == AvoidHoles::Inactive
                        {
                            thr = if (*band_3).thr
                                > (*coeffs.offset(g as isize)).min_snr * (*band_3).energy
                            {
                                (*band_3).thr
                            } else {
                                (*coeffs.offset(g as isize)).min_snr * (*band_3).energy
                            };
                        }
                        (*band_3).thr = thr;
                    }
                    g += 1;
                    g;
                }
                w += 16;
            }
        } else {
            g = num_bands;
            while pe > desired_pe && {
                let fresh0 = g;
                g -= 1;
                fresh0 != 0
            } {
                w = 0;
                while w < (*wi).num_windows * 16 {
                    let mut band_4: *mut AacPsyBand =
                        &mut *((*pch).band).as_mut_ptr().offset((w + g) as isize)
                            as *mut AacPsyBand;
                    if (*band_4).avoid_holes != AvoidHoles::None
                        && (*coeffs.offset(g as isize)).min_snr < 7.943_282e-1_f32
                    {
                        (*coeffs.offset(g as isize)).min_snr = 7.943_282e-1_f32;
                        (*band_4).thr = (*band_4).energy * 7.943_282e-1_f32;
                        pe += (*band_4).active_lines * 1.5 - (*band_4).pe;
                    }
                    w += 16;
                }
            }
        }
    }
    w = 0;
    while w < (*wi).num_windows {
        g = 0;
        while g < num_bands {
            let band_5 = &(*pch).band[W(w)][g as usize];
            let psy_band = &mut (*ctx).ch[channel as usize].psy_bands[W(w)][g as usize];
            psy_band.threshold = band_5.thr;
            psy_band.energy = band_5.energy;
            psy_band.spread = band_5.active_lines * 2. / band_sizes[g as usize] as c_int as c_float;
            psy_band.bits = (band_5.pe / 1.18) as c_int;
            g += 1;
            g;
        }
        w += 1;
    }
    (*pch).prev_band = *(*pch).band;
}

pub(super) unsafe extern "C" fn psy_3gpp_analyze(
    mut ctx: *mut FFPsyContext,
    mut channel: c_int,
    mut coeffs: *mut *const c_float,
    mut wi: *const FFPsyWindowInfo,
) {
    let mut ch: c_int = 0;
    let mut group: *mut FFPsyChannelGroup = find_group(ctx, channel);
    ch = 0;
    while ch < (*group).num_ch as c_int {
        psy_3gpp_analyze_channel(
            ctx,
            channel + ch,
            *coeffs.offset(ch as isize),
            &*wi.offset(ch as isize),
        );
        ch += 1;
        ch;
    }
}

#[cold]
pub(crate) unsafe extern "C" fn psy_3gpp_end(mut apc: *mut FFPsyContext) {
    let mut pctx: *mut AacPsyContext = (*apc).model_priv_data as *mut AacPsyContext;
    // TODO: leaks 🚿
    if !pctx.is_null() {
        // av_freep(&mut (*pctx).ch as *mut *mut AacPsyChannel as *mut c_void);
    }
    // av_freep(&mut (*apc).model_priv_data as *mut *mut c_void as *mut c_void);
}

unsafe fn lame_apply_block_type(
    mut ctx: *mut AacPsyChannel,
    mut wi: *mut FFPsyWindowInfo,
    mut uselongblock: c_int,
) {
    let mut blocktype: c_int = ONLY_LONG_SEQUENCE as c_int;
    if uselongblock != 0 {
        if (*ctx).next_window_seq as c_uint == EIGHT_SHORT_SEQUENCE as c_int as c_uint {
            blocktype = LONG_STOP_SEQUENCE as c_int;
        }
    } else {
        blocktype = EIGHT_SHORT_SEQUENCE as c_int;
        if (*ctx).next_window_seq as c_uint == ONLY_LONG_SEQUENCE as c_int as c_uint {
            (*ctx).next_window_seq = LONG_START_SEQUENCE;
        }
        if (*ctx).next_window_seq as c_uint == LONG_STOP_SEQUENCE as c_int as c_uint {
            (*ctx).next_window_seq = EIGHT_SHORT_SEQUENCE;
        }
    }
    (*wi).window_type[0] = (*ctx).next_window_seq as c_int;
    (*ctx).next_window_seq = blocktype as WindowSequence;
}

pub(super) unsafe fn psy_lame_window(
    mut ctx: &mut FFPsyContext,
    mut la: Option<&[c_float]>,
    mut channel: c_int,
    mut prev_type: c_int,
) -> FFPsyWindowInfo {
    let mut pctx: *mut AacPsyContext = ctx.model_priv_data as *mut AacPsyContext;
    let mut pch: *mut AacPsyChannel =
        &mut *((*pctx).ch).offset(channel as isize) as *mut AacPsyChannel;
    let mut grouping: c_int = 0;
    let mut uselongblock: c_int = 1;
    let mut attacks: [c_int; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut i: c_int = 0;
    let mut wi = FFPsyWindowInfo::default();
    if let Some(la) = la {
        let mut hpfsmpl: [c_float; 1024] = [0.; 1024];
        let mut pf: *const c_float = hpfsmpl.as_mut_ptr();
        let mut attack_intensity: [c_float; 27] = [0.; 27];
        let mut energy_subshort: [c_float; 27] = [0.; 27];
        let mut energy_short: [c_float; 9] = [0., 0., 0., 0., 0., 0., 0., 0., 0.];
        let mut firbuf: *const c_float = la.as_ptr().offset((128 / 4 - 21) as isize);
        let mut att_sum: c_int = 0;
        psy_hp_filter(firbuf, hpfsmpl.as_mut_ptr(), FIR_COEFFS.as_ptr());
        i = 0;
        while i < 3 {
            energy_subshort[i as usize] = (*pch).prev_energy_subshort[(i + (8 - 1) * 3) as usize];
            attack_intensity[i as usize] = energy_subshort[i as usize]
                / (*pch).prev_energy_subshort[(i + ((8 - 2) * 3 + 1)) as usize];
            energy_short[0] += energy_subshort[i as usize];
            i += 1;
            i;
        }
        i = 0;
        while i < 8 * 3 {
            let pfe: *const c_float = pf.offset((1024 / (8 * 3)) as isize);
            let mut p: c_float = 1.;
            while pf < pfe {
                p = if p > fabsf(*pf) { p } else { fabsf(*pf) };
                pf = pf.offset(1);
                pf;
            }
            energy_subshort[(i + 3) as usize] = p;
            (*pch).prev_energy_subshort[i as usize] = energy_subshort[(i + 3) as usize];
            energy_short[(1 + i / 3) as usize] += p;
            if p > energy_subshort[(i + 1) as usize] {
                p /= energy_subshort[(i + 1) as usize];
            } else if energy_subshort[(i + 1) as usize] > p * 10. {
                p = energy_subshort[(i + 1) as usize] / (p * 10.);
            } else {
                p = 0.;
            }
            attack_intensity[(i + 3) as usize] = p;
            i += 1;
            i;
        }
        i = 0;
        while i < (8 + 1) * 3 {
            if attacks[(i / 3) as usize] == 0
                && attack_intensity[i as usize] > (*pch).attack_threshold
            {
                attacks[(i / 3) as usize] = i % 3 + 1;
            }
            i += 1;
            i;
        }
        i = 1;
        while i < 8 + 1 {
            let u: c_float = energy_short[(i - 1) as usize];
            let v: c_float = energy_short[i as usize];
            let m: c_float = if u > v { u } else { v };
            if m < 40000. && u < 1.7 * v && v < 1.7 * u {
                if i == 1 && attacks[0] < attacks[i as usize] {
                    attacks[0] = 0;
                }
                attacks[i as usize] = 0;
            }
            att_sum += attacks[i as usize];
            i += 1;
            i;
        }
        if attacks[0] <= (*pch).prev_attack {
            attacks[0] = 0;
        }
        att_sum += attacks[0];
        if (*pch).prev_attack == 3 || att_sum != 0 {
            uselongblock = 0;
            i = 1;
            while i < 8 + 1 {
                if attacks[i as usize] != 0 && attacks[(i - 1) as usize] != 0 {
                    attacks[i as usize] = 0;
                }
                i += 1;
                i;
            }
        }
    } else {
        uselongblock = !(prev_type == EIGHT_SHORT_SEQUENCE as c_int) as c_int;
    }
    lame_apply_block_type(pch, &mut wi, uselongblock);
    wi.window_type[1] = prev_type;
    if wi.window_type[0] != EIGHT_SHORT_SEQUENCE as c_int {
        wi.num_windows = 1;
        wi.grouping[0] = 1;
        if wi.window_type[0] == LONG_START_SEQUENCE as c_int {
            wi.window_shape = 0;
        } else {
            wi.window_shape = 1;
        }
    } else {
        let mut lastgrp: c_int = 0;
        wi.num_windows = 8;
        wi.window_shape = 0;
        i = 0;
        while i < 8 {
            if (*pch).next_grouping as c_int >> i & 1 == 0 {
                lastgrp = i;
            }
            wi.grouping[lastgrp as usize] += 1;
            wi.grouping[lastgrp as usize];
            i += 1;
            i;
        }
    }
    i = 0;
    while i < 9 {
        if attacks[i as usize] != 0 {
            grouping = i;
            break;
        } else {
            i += 1;
            i;
        }
    }
    (*pch).next_grouping = WINDOW_GROUPING[grouping as usize];
    (*pch).prev_attack = attacks[8];
    wi
}
