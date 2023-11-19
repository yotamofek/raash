#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use std::alloc::{alloc_zeroed, Layout};

use crate::{common::*, psymodel::ff_psy_find_group, types::*};

pub type C2RustUnnamed_1 = libc::c_uint;
pub const PSY_3GPP_AH_ACTIVE: C2RustUnnamed_1 = 2;
pub const PSY_3GPP_AH_INACTIVE: C2RustUnnamed_1 = 1;
pub const PSY_3GPP_AH_NONE: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AacPsyBand {
    pub energy: libc::c_float,
    pub thr: libc::c_float,
    pub thr_quiet: libc::c_float,
    pub nz_lines: libc::c_float,
    pub active_lines: libc::c_float,
    pub pe: libc::c_float,
    pub pe_const: libc::c_float,
    pub norm_fac: libc::c_float,
    pub avoid_holes: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AacPsyChannel {
    pub band: [AacPsyBand; 128],
    pub prev_band: [AacPsyBand; 128],
    pub win_energy: libc::c_float,
    pub iir_state: [libc::c_float; 2],
    pub next_grouping: uint8_t,
    pub next_window_seq: WindowSequence,
    pub attack_threshold: libc::c_float,
    pub prev_energy_subshort: [libc::c_float; 24],
    pub prev_attack: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AacPsyCoeffs {
    pub ath: libc::c_float,
    pub barks: libc::c_float,
    pub spread_low: [libc::c_float; 2],
    pub spread_hi: [libc::c_float; 2],
    pub min_snr: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AacPsyContext {
    pub chan_bitrate: libc::c_int,
    pub frame_bits: libc::c_int,
    pub fill_level: libc::c_int,
    pub pe: C2RustUnnamed_2,
    pub psy_coef: [[AacPsyCoeffs; 64]; 2],
    pub ch: *mut AacPsyChannel,
    pub global_quality: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub min: libc::c_float,
    pub max: libc::c_float,
    pub previous: libc::c_float,
    pub correction: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PsyLamePreset {
    pub quality: libc::c_int,
    pub st_lrm: libc::c_float,
}
#[inline(always)]
unsafe extern "C" fn ff_exp10(mut x: libc::c_double) -> libc::c_double {
    exp2(3.321_928_094_887_362_f64 * x)
}
#[inline(always)]
unsafe extern "C" fn av_clip_c(
    mut a: libc::c_int,
    mut amin: libc::c_int,
    mut amax: libc::c_int,
) -> libc::c_int {
    if a < amin {
        amin
    } else if a > amax {
        return amax;
    } else {
        return a;
    }
}
#[inline(always)]
unsafe extern "C" fn av_clipf_c(
    mut a: libc::c_float,
    mut amin: libc::c_float,
    mut amax: libc::c_float,
) -> libc::c_float {
    if (if a > amin { a } else { amin }) > amax {
        amax
    } else if a > amin {
        a
    } else {
        amin
    }
}
static mut psy_abr_map: [PsyLamePreset; 13] = [
    {
        PsyLamePreset {
            quality: 8 as libc::c_int,
            st_lrm: 6.60f64 as libc::c_float,
        }
    },
    {
        PsyLamePreset {
            quality: 16 as libc::c_int,
            st_lrm: 6.60f64 as libc::c_float,
        }
    },
    {
        PsyLamePreset {
            quality: 24 as libc::c_int,
            st_lrm: 6.60f64 as libc::c_float,
        }
    },
    {
        PsyLamePreset {
            quality: 32 as libc::c_int,
            st_lrm: 6.60f64 as libc::c_float,
        }
    },
    {
        PsyLamePreset {
            quality: 40 as libc::c_int,
            st_lrm: 6.60f64 as libc::c_float,
        }
    },
    {
        PsyLamePreset {
            quality: 48 as libc::c_int,
            st_lrm: 6.60f64 as libc::c_float,
        }
    },
    {
        PsyLamePreset {
            quality: 56 as libc::c_int,
            st_lrm: 6.60f64 as libc::c_float,
        }
    },
    {
        PsyLamePreset {
            quality: 64 as libc::c_int,
            st_lrm: 6.40f64 as libc::c_float,
        }
    },
    {
        PsyLamePreset {
            quality: 80 as libc::c_int,
            st_lrm: 6.00f64 as libc::c_float,
        }
    },
    {
        PsyLamePreset {
            quality: 96 as libc::c_int,
            st_lrm: 5.60f64 as libc::c_float,
        }
    },
    {
        PsyLamePreset {
            quality: 112 as libc::c_int,
            st_lrm: 5.20f64 as libc::c_float,
        }
    },
    {
        PsyLamePreset {
            quality: 128 as libc::c_int,
            st_lrm: 5.20f64 as libc::c_float,
        }
    },
    {
        PsyLamePreset {
            quality: 160 as libc::c_int,
            st_lrm: 5.20f64 as libc::c_float,
        }
    },
];
static mut psy_vbr_map: [PsyLamePreset; 11] = [
    {
        PsyLamePreset {
            quality: 0 as libc::c_int,
            st_lrm: 4.20f64 as libc::c_float,
        }
    },
    {
        PsyLamePreset {
            quality: 1 as libc::c_int,
            st_lrm: 4.20f64 as libc::c_float,
        }
    },
    {
        PsyLamePreset {
            quality: 2 as libc::c_int,
            st_lrm: 4.20f64 as libc::c_float,
        }
    },
    {
        PsyLamePreset {
            quality: 3 as libc::c_int,
            st_lrm: 4.20f64 as libc::c_float,
        }
    },
    {
        PsyLamePreset {
            quality: 4 as libc::c_int,
            st_lrm: 4.20f64 as libc::c_float,
        }
    },
    {
        PsyLamePreset {
            quality: 5 as libc::c_int,
            st_lrm: 4.20f64 as libc::c_float,
        }
    },
    {
        PsyLamePreset {
            quality: 6 as libc::c_int,
            st_lrm: 4.20f64 as libc::c_float,
        }
    },
    {
        PsyLamePreset {
            quality: 7 as libc::c_int,
            st_lrm: 4.20f64 as libc::c_float,
        }
    },
    {
        PsyLamePreset {
            quality: 8 as libc::c_int,
            st_lrm: 4.20f64 as libc::c_float,
        }
    },
    {
        PsyLamePreset {
            quality: 9 as libc::c_int,
            st_lrm: 4.20f64 as libc::c_float,
        }
    },
    {
        PsyLamePreset {
            quality: 10 as libc::c_int,
            st_lrm: 4.20f64 as libc::c_float,
        }
    },
];
static mut psy_fir_coeffs: [libc::c_float; 10] = [
    (-8.65163e-18f64 * 2 as libc::c_int as libc::c_double) as libc::c_float,
    (-0.00851586f64 * 2 as libc::c_int as libc::c_double) as libc::c_float,
    (-6.74764e-18f64 * 2 as libc::c_int as libc::c_double) as libc::c_float,
    (0.0209036f64 * 2 as libc::c_int as libc::c_double) as libc::c_float,
    (-3.36639e-17f64 * 2 as libc::c_int as libc::c_double) as libc::c_float,
    (-0.0438162f64 * 2 as libc::c_int as libc::c_double) as libc::c_float,
    (-1.54175e-17f64 * 2 as libc::c_int as libc::c_double) as libc::c_float,
    (0.0931738f64 * 2 as libc::c_int as libc::c_double) as libc::c_float,
    (-5.52212e-17f64 * 2 as libc::c_int as libc::c_double) as libc::c_float,
    (-0.313819f64 * 2 as libc::c_int as libc::c_double) as libc::c_float,
];
unsafe extern "C" fn lame_calc_attack_threshold(mut bitrate: libc::c_int) -> libc::c_float {
    let mut lower_range: libc::c_int = 12 as libc::c_int;
    let mut upper_range: libc::c_int = 12 as libc::c_int;
    let mut lower_range_kbps: libc::c_int = psy_abr_map[12 as libc::c_int as usize].quality;
    let mut upper_range_kbps: libc::c_int = psy_abr_map[12 as libc::c_int as usize].quality;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < 13 as libc::c_int {
        if (if bitrate > psy_abr_map[i as usize].quality {
            bitrate
        } else {
            psy_abr_map[i as usize].quality
        }) != bitrate
        {
            upper_range = i;
            upper_range_kbps = psy_abr_map[i as usize].quality;
            lower_range = i - 1 as libc::c_int;
            lower_range_kbps = psy_abr_map[(i - 1 as libc::c_int) as usize].quality;
            break;
        } else {
            i += 1;
            i;
        }
    }
    if upper_range_kbps - bitrate > bitrate - lower_range_kbps {
        return psy_abr_map[lower_range as usize].st_lrm;
    }
    psy_abr_map[upper_range as usize].st_lrm
}
#[cold]
unsafe extern "C" fn lame_window_init(mut ctx: *mut AacPsyContext, mut avctx: *mut AVCodecContext) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*avctx).ch_layout.nb_channels {
        let mut pch: *mut AacPsyChannel =
            &mut *((*ctx).ch).offset(i as isize) as *mut AacPsyChannel;
        if (*avctx).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
            (*pch).attack_threshold = psy_vbr_map[av_clip_c(
                (*avctx).global_quality / 118 as libc::c_int,
                0 as libc::c_int,
                10 as libc::c_int,
            ) as usize]
                .st_lrm;
        } else {
            (*pch).attack_threshold = lame_calc_attack_threshold(
                ((*avctx).bit_rate
                    / (*avctx).ch_layout.nb_channels as libc::c_long
                    / 1000 as libc::c_int as libc::c_long) as libc::c_int,
            );
        }
        j = 0 as libc::c_int;
        while j < 8 as libc::c_int * 3 as libc::c_int {
            (*pch).prev_energy_subshort[j as usize] = 10.0f32;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[cold]
unsafe extern "C" fn calc_bark(mut f: libc::c_float) -> libc::c_float {
    13.3f32 * atanf(0.00076f32 * f) + 3.5f32 * atanf(f / 7500.0f32 * (f / 7500.0f32))
}
#[cold]
unsafe extern "C" fn ath(mut f: libc::c_float, mut add: libc::c_float) -> libc::c_float {
    f /= 1000.0f32;
    (3.64f64 * pow(f as libc::c_double, -0.8f64)
        - 6.8f64 * exp(-0.6f64 * (f as libc::c_double - 3.4f64) * (f as libc::c_double - 3.4f64))
        + 6.0f64 * exp(-0.15f64 * (f as libc::c_double - 8.7f64) * (f as libc::c_double - 8.7f64))
        + (0.6f64 + 0.04f64 * add as libc::c_double)
            * 0.001f64
            * f as libc::c_double
            * f as libc::c_double
            * f as libc::c_double
            * f as libc::c_double) as libc::c_float
}
#[cold]
unsafe extern "C" fn psy_3gpp_init(mut ctx: *mut FFPsyContext) -> libc::c_int {
    let mut pctx: *mut AacPsyContext = std::ptr::null_mut::<AacPsyContext>();
    let mut bark: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut prev: libc::c_float = 0.;
    let mut minscale: libc::c_float = 0.;
    let mut minath: libc::c_float = 0.;
    let mut minsnr: libc::c_float = 0.;
    let mut pe_min: libc::c_float = 0.;
    let mut chan_bitrate: libc::c_int = ((*(*ctx).avctx).bit_rate as libc::c_float
        / (if (*(*ctx).avctx).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
            2.0f32
        } else {
            (*(*ctx).avctx).ch_layout.nb_channels as libc::c_float
        })) as libc::c_int;
    let bandwidth: libc::c_int = (if (*ctx).cutoff != 0 {
        (*ctx).cutoff as libc::c_long
    } else if (*(*ctx).avctx).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        ((*(*ctx).avctx).sample_rate / 2 as libc::c_int) as libc::c_long
    } else if (*(*ctx).avctx).bit_rate != 0 {
        if (if (if (if (if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        }) > 3000 as libc::c_int as libc::c_long
            + (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 4 as libc::c_int as libc::c_long
        {
            3000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 4 as libc::c_int as libc::c_long
        } else if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        }) > 12000 as libc::c_int as libc::c_long
            + (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 16 as libc::c_int as libc::c_long
        {
            12000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 16 as libc::c_int as libc::c_long
        } else if (if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        }) > 3000 as libc::c_int as libc::c_long
            + (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 4 as libc::c_int as libc::c_long
        {
            3000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 4 as libc::c_int as libc::c_long
        } else if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        }) > 22000 as libc::c_int as libc::c_long
        {
            22000 as libc::c_int as libc::c_long
        } else if (if (if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        }) > 3000 as libc::c_int as libc::c_long
            + (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 4 as libc::c_int as libc::c_long
        {
            3000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 4 as libc::c_int as libc::c_long
        } else if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        }) > 12000 as libc::c_int as libc::c_long
            + (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 16 as libc::c_int as libc::c_long
        {
            12000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 16 as libc::c_int as libc::c_long
        } else if (if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        }) > 3000 as libc::c_int as libc::c_long
            + (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 4 as libc::c_int as libc::c_long
        {
            3000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 4 as libc::c_int as libc::c_long
        } else if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        }) > ((*(*ctx).avctx).sample_rate / 2 as libc::c_int) as libc::c_long
        {
            ((*(*ctx).avctx).sample_rate / 2 as libc::c_int) as libc::c_long
        } else if (if (if (if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        }) > 3000 as libc::c_int as libc::c_long
            + (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 4 as libc::c_int as libc::c_long
        {
            3000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 4 as libc::c_int as libc::c_long
        } else if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        }) > 12000 as libc::c_int as libc::c_long
            + (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 16 as libc::c_int as libc::c_long
        {
            12000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 16 as libc::c_int as libc::c_long
        } else if (if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        }) > 3000 as libc::c_int as libc::c_long
            + (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 4 as libc::c_int as libc::c_long
        {
            3000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 4 as libc::c_int as libc::c_long
        } else if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        }) > 22000 as libc::c_int as libc::c_long
        {
            22000 as libc::c_int as libc::c_long
        } else if (if (if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        }) > 3000 as libc::c_int as libc::c_long
            + (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 4 as libc::c_int as libc::c_long
        {
            3000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 4 as libc::c_int as libc::c_long
        } else if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        }) > 12000 as libc::c_int as libc::c_long
            + (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 16 as libc::c_int as libc::c_long
        {
            12000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 16 as libc::c_int as libc::c_long
        } else if (if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        }) > 3000 as libc::c_int as libc::c_long
            + (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 4 as libc::c_int as libc::c_long
        {
            3000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 4 as libc::c_int as libc::c_long
        } else if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        }
    } else {
        ((*(*ctx).avctx).sample_rate / 2 as libc::c_int) as libc::c_long
    }) as libc::c_int;
    let num_bark: libc::c_float = calc_bark(bandwidth as libc::c_float);
    if bandwidth <= 0 as libc::c_int {
        return -(22 as libc::c_int);
    }
    (*ctx).model_priv_data = alloc_zeroed(Layout::new::<AacPsyContext>()).cast();
    if ((*ctx).model_priv_data).is_null() {
        return -(12 as libc::c_int);
    }
    pctx = (*ctx).model_priv_data as *mut AacPsyContext;
    (*pctx).global_quality = (if (*(*ctx).avctx).global_quality != 0 {
        (*(*ctx).avctx).global_quality
    } else {
        120 as libc::c_int
    }) as libc::c_float
        * 0.01f32;
    if (*(*ctx).avctx).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        chan_bitrate = (chan_bitrate as libc::c_double / 120.0f64
            * (if (*(*ctx).avctx).global_quality != 0 {
                (*(*ctx).avctx).global_quality
            } else {
                120 as libc::c_int
            }) as libc::c_double) as libc::c_int;
    }
    (*pctx).chan_bitrate = chan_bitrate;
    (*pctx).frame_bits =
        if 2560 as libc::c_int > chan_bitrate * 1024 as libc::c_int / (*(*ctx).avctx).sample_rate {
            chan_bitrate * 1024 as libc::c_int / (*(*ctx).avctx).sample_rate
        } else {
            2560 as libc::c_int
        };
    (*pctx).pe.min = 8.0f32 * 1024 as libc::c_int as libc::c_float * bandwidth as libc::c_float
        / ((*(*ctx).avctx).sample_rate as libc::c_float * 2.0f32);
    (*pctx).pe.max = 12.0f32 * 1024 as libc::c_int as libc::c_float * bandwidth as libc::c_float
        / ((*(*ctx).avctx).sample_rate as libc::c_float * 2.0f32);
    (*ctx).bitres.size = 6144 as libc::c_int - (*pctx).frame_bits;
    (*ctx).bitres.size -= (*ctx).bitres.size % 8 as libc::c_int;
    (*pctx).fill_level = (*ctx).bitres.size;
    minath = ath(
        (3410 as libc::c_int as libc::c_double - 0.733f64 * 4 as libc::c_int as libc::c_double)
            as libc::c_float,
        4 as libc::c_int as libc::c_float,
    );
    j = 0 as libc::c_int;
    while j < 2 as libc::c_int {
        let mut coeffs: *mut AacPsyCoeffs = ((*pctx).psy_coef[j as usize]).as_mut_ptr();
        let mut band_sizes: *const uint8_t = *((*ctx).bands).offset(j as isize);
        let mut line_to_frequency: libc::c_float = (*(*ctx).avctx).sample_rate as libc::c_float
            / (if j != 0 { 256.0f32 } else { 2048.0f32 });
        let mut avg_chan_bits: libc::c_float = chan_bitrate as libc::c_float
            * (if j != 0 { 128.0f32 } else { 1024.0f32 })
            / (*(*ctx).avctx).sample_rate as libc::c_float;
        let mut bark_pe: libc::c_float = 0.024f32 * (avg_chan_bits * 1.18f32) / num_bark;
        let mut en_spread_low: libc::c_float = if j != 0 { 2.0f32 } else { 3.0f32 };
        let mut en_spread_hi: libc::c_float = if j != 0 || chan_bitrate as libc::c_float <= 22.0f32
        {
            1.5f32
        } else {
            2.0f32
        };
        i = 0 as libc::c_int;
        prev = 0.0f64 as libc::c_float;
        g = 0 as libc::c_int;
        while g < *((*ctx).num_bands).offset(j as isize) {
            i += *band_sizes.offset(g as isize) as libc::c_int;
            bark = calc_bark((i - 1 as libc::c_int) as libc::c_float * line_to_frequency);
            (*coeffs.offset(g as isize)).barks =
                ((bark + prev) as libc::c_double / 2.0f64) as libc::c_float;
            prev = bark;
            g += 1;
            g;
        }
        g = 0 as libc::c_int;
        while g < *((*ctx).num_bands).offset(j as isize) - 1 as libc::c_int {
            let mut coeff: *mut AacPsyCoeffs = &mut *coeffs.offset(g as isize) as *mut AacPsyCoeffs;
            let mut bark_width: libc::c_float =
                (*coeffs.offset((g + 1 as libc::c_int) as isize)).barks - (*coeffs).barks;
            (*coeff).spread_low[0 as libc::c_int as usize] =
                ff_exp10((-bark_width * 3.0f32) as libc::c_double) as libc::c_float;
            (*coeff).spread_hi[0 as libc::c_int as usize] =
                ff_exp10((-bark_width * 1.5f32) as libc::c_double) as libc::c_float;
            (*coeff).spread_low[1 as libc::c_int as usize] =
                ff_exp10((-bark_width * en_spread_low) as libc::c_double) as libc::c_float;
            (*coeff).spread_hi[1 as libc::c_int as usize] =
                ff_exp10((-bark_width * en_spread_hi) as libc::c_double) as libc::c_float;
            pe_min = bark_pe * bark_width;
            minsnr = (exp2(
                (pe_min / *band_sizes.offset(g as isize) as libc::c_int as libc::c_float)
                    as libc::c_double,
            ) - 1.5f32 as libc::c_double) as libc::c_float;
            (*coeff).min_snr = av_clipf_c(1.0f32 / minsnr, 3.1622776e-3f32, 7.943_282e-1_f32);
            g += 1;
            g;
        }
        start = 0 as libc::c_int;
        g = 0 as libc::c_int;
        while g < *((*ctx).num_bands).offset(j as isize) {
            minscale = ath(
                start as libc::c_float * line_to_frequency,
                4 as libc::c_int as libc::c_float,
            );
            i = 1 as libc::c_int;
            while i < *band_sizes.offset(g as isize) as libc::c_int {
                minscale = if minscale
                    > ath(
                        (start + i) as libc::c_float * line_to_frequency,
                        4 as libc::c_int as libc::c_float,
                    ) {
                    ath(
                        (start + i) as libc::c_float * line_to_frequency,
                        4 as libc::c_int as libc::c_float,
                    )
                } else {
                    minscale
                };
                i += 1;
                i;
            }
            (*coeffs.offset(g as isize)).ath = minscale - minath;
            start += *band_sizes.offset(g as isize) as libc::c_int;
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
        // av_freep(&mut (*ctx).model_priv_data as *mut *mut libc::c_void as *mut libc::c_void);
        return -(12 as libc::c_int);
    }
    lame_window_init(pctx, (*ctx).avctx);
    0 as libc::c_int
}
static mut window_grouping: [uint8_t; 9] = [
    0xb6 as libc::c_int as uint8_t,
    0x6c as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xb2 as libc::c_int as uint8_t,
    0x66 as libc::c_int as uint8_t,
    0xc6 as libc::c_int as uint8_t,
    0x96 as libc::c_int as uint8_t,
    0x36 as libc::c_int as uint8_t,
    0x36 as libc::c_int as uint8_t,
];
unsafe extern "C" fn calc_bit_demand(
    mut ctx: *mut AacPsyContext,
    mut pe: libc::c_float,
    mut bits: libc::c_int,
    mut size: libc::c_int,
    mut short_window: libc::c_int,
) -> libc::c_int {
    let bitsave_slope: libc::c_float = if short_window != 0 {
        -0.36363637f32
    } else {
        -0.46666667f32
    };
    let bitsave_add: libc::c_float = if short_window != 0 {
        -0.75f32
    } else {
        -0.842_857_1_f32
    };
    let bitspend_slope: libc::c_float = if short_window != 0 {
        0.818_181_8_f32
    } else {
        0.666_666_7_f32
    };
    let bitspend_add: libc::c_float = if short_window != 0 {
        -0.261_111_1_f32
    } else {
        -0.35f32
    };
    let clip_low: libc::c_float = if short_window != 0 { 0.2f32 } else { 0.2f32 };
    let clip_high: libc::c_float = if short_window != 0 { 0.75f32 } else { 0.95f32 };
    let mut clipped_pe: libc::c_float = 0.;
    let mut bit_save: libc::c_float = 0.;
    let mut bit_spend: libc::c_float = 0.;
    let mut bit_factor: libc::c_float = 0.;
    let mut fill_level: libc::c_float = 0.;
    let mut forgetful_min_pe: libc::c_float = 0.;
    (*ctx).fill_level += (*ctx).frame_bits - bits;
    (*ctx).fill_level = av_clip_c((*ctx).fill_level, 0 as libc::c_int, size);
    fill_level = av_clipf_c(
        (*ctx).fill_level as libc::c_float / size as libc::c_float,
        clip_low,
        clip_high,
    );
    clipped_pe = av_clipf_c(pe, (*ctx).pe.min, (*ctx).pe.max);
    bit_save = (fill_level + bitsave_add) * bitsave_slope;
    bit_spend = (fill_level + bitspend_add) * bitspend_slope;
    bit_factor = 1.0f32 - bit_save
        + (bit_spend - bit_save) / ((*ctx).pe.max - (*ctx).pe.min) * (clipped_pe - (*ctx).pe.min);
    (*ctx).pe.max = if pe > (*ctx).pe.max {
        pe
    } else {
        (*ctx).pe.max
    };
    forgetful_min_pe = ((*ctx).pe.min * 511 as libc::c_int as libc::c_float
        + (if (*ctx).pe.min > pe * (pe / (*ctx).pe.max) {
            (*ctx).pe.min
        } else {
            pe * (pe / (*ctx).pe.max)
        }))
        / (511 as libc::c_int + 1 as libc::c_int) as libc::c_float;
    (*ctx).pe.min = if pe > forgetful_min_pe {
        forgetful_min_pe
    } else {
        pe
    };
    (if (*ctx).frame_bits as libc::c_float * bit_factor
        > (if (*ctx).frame_bits + size - bits > (*ctx).frame_bits / 8 as libc::c_int {
            (*ctx).frame_bits + size - bits
        } else {
            (*ctx).frame_bits / 8 as libc::c_int
        }) as libc::c_float
    {
        (if (*ctx).frame_bits + size - bits > (*ctx).frame_bits / 8 as libc::c_int {
            (*ctx).frame_bits + size - bits
        } else {
            (*ctx).frame_bits / 8 as libc::c_int
        }) as libc::c_float
    } else {
        (*ctx).frame_bits as libc::c_float * bit_factor
    }) as libc::c_int
}
unsafe extern "C" fn calc_pe_3gpp(mut band: *mut AacPsyBand) -> libc::c_float {
    let mut pe: libc::c_float = 0.;
    let mut a: libc::c_float = 0.;
    (*band).pe = 0.0f32;
    (*band).pe_const = 0.0f32;
    (*band).active_lines = 0.0f32;
    if (*band).energy > (*band).thr {
        a = log2f((*band).energy);
        pe = a - log2f((*band).thr);
        (*band).active_lines = (*band).nz_lines;
        if pe < 3.0f32 {
            pe = pe * 0.559_357_3_f32 + 1.3219281f32;
            a = a * 0.559_357_3_f32 + 1.3219281f32;
            (*band).active_lines *= 0.559_357_3_f32;
        }
        (*band).pe = pe * (*band).nz_lines;
        (*band).pe_const = a * (*band).nz_lines;
    }
    (*band).pe
}
unsafe extern "C" fn calc_reduction_3gpp(
    mut a: libc::c_float,
    mut desired_pe: libc::c_float,
    mut pe: libc::c_float,
    mut active_lines: libc::c_float,
) -> libc::c_float {
    let mut thr_avg: libc::c_float = 0.;
    let mut reduction: libc::c_float = 0.;
    if active_lines as libc::c_double == 0.0f64 {
        return 0 as libc::c_int as libc::c_float;
    }
    thr_avg = exp2f((a - pe) / (4.0f32 * active_lines));
    reduction = exp2f((a - desired_pe) / (4.0f32 * active_lines)) - thr_avg;
    if reduction > 0.0f32 {
        reduction
    } else {
        0.0f32
    }
}
unsafe extern "C" fn calc_reduced_thr_3gpp(
    mut band: *mut AacPsyBand,
    mut min_snr: libc::c_float,
    mut reduction: libc::c_float,
) -> libc::c_float {
    let mut thr: libc::c_float = (*band).thr;
    if (*band).energy > thr {
        thr = sqrtf(thr);
        thr = sqrtf(thr) + reduction;
        thr *= thr;
        thr *= thr;
        if thr > (*band).energy * min_snr && (*band).avoid_holes != PSY_3GPP_AH_NONE as libc::c_int
        {
            thr = if (*band).thr > (*band).energy * min_snr {
                (*band).thr
            } else {
                (*band).energy * min_snr
            };
            (*band).avoid_holes = PSY_3GPP_AH_ACTIVE as libc::c_int;
        }
    }
    thr
}
unsafe extern "C" fn calc_thr_3gpp(
    mut wi: *const FFPsyWindowInfo,
    num_bands: libc::c_int,
    mut pch: *mut AacPsyChannel,
    mut band_sizes: *const uint8_t,
    mut coefs: *const libc::c_float,
    cutoff: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut start: libc::c_int = 0 as libc::c_int;
    let mut wstart: libc::c_int = 0 as libc::c_int;
    w = 0 as libc::c_int;
    while w < (*wi).num_windows * 16 as libc::c_int {
        wstart = 0 as libc::c_int;
        g = 0 as libc::c_int;
        while g < num_bands {
            let mut band: *mut AacPsyBand =
                &mut *((*pch).band).as_mut_ptr().offset((w + g) as isize) as *mut AacPsyBand;
            let mut form_factor: libc::c_float = 0.0f32;
            let mut Temp: libc::c_float = 0.;
            (*band).energy = 0.0f32;
            if wstart < cutoff {
                i = 0 as libc::c_int;
                while i < *band_sizes.offset(g as isize) as libc::c_int {
                    (*band).energy +=
                        *coefs.offset((start + i) as isize) * *coefs.offset((start + i) as isize);
                    form_factor +=
                        sqrtf(fabs(*coefs.offset((start + i) as isize) as libc::c_double)
                            as libc::c_float);
                    i += 1;
                    i;
                }
            }
            Temp = if (*band).energy > 0 as libc::c_int as libc::c_float {
                sqrtf(*band_sizes.offset(g as isize) as libc::c_float / (*band).energy)
            } else {
                0 as libc::c_int as libc::c_float
            };
            (*band).thr = (*band).energy * 0.001258925f32;
            (*band).nz_lines = form_factor * sqrtf(Temp);
            start += *band_sizes.offset(g as isize) as libc::c_int;
            wstart += *band_sizes.offset(g as isize) as libc::c_int;
            g += 1;
            g;
        }
        w += 16 as libc::c_int;
    }
}
unsafe extern "C" fn psy_hp_filter(
    mut firbuf: *const libc::c_float,
    mut hpfsmpl: *mut libc::c_float,
    mut psy_fir_coeffs_0: *const libc::c_float,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 1024 as libc::c_int {
        let mut sum1: libc::c_float = 0.;
        let mut sum2: libc::c_float = 0.;
        sum1 = *firbuf
            .offset((i + (21 as libc::c_int - 1 as libc::c_int) / 2 as libc::c_int) as isize);
        sum2 = 0.0f64 as libc::c_float;
        j = 0 as libc::c_int;
        while j < (21 as libc::c_int - 1 as libc::c_int) / 2 as libc::c_int - 1 as libc::c_int {
            sum1 += *psy_fir_coeffs_0.offset(j as isize)
                * (*firbuf.offset((i + j) as isize)
                    + *firbuf.offset((i + 21 as libc::c_int - j) as isize));
            sum2 += *psy_fir_coeffs_0.offset((j + 1 as libc::c_int) as isize)
                * (*firbuf.offset((i + j + 1 as libc::c_int) as isize)
                    + *firbuf.offset((i + 21 as libc::c_int - j - 1 as libc::c_int) as isize));
            j += 2 as libc::c_int;
        }
        *hpfsmpl.offset(i as isize) = (sum1 + sum2) * 32768.0f32;
        i += 1;
        i;
    }
}
unsafe extern "C" fn psy_3gpp_analyze_channel(
    mut ctx: *mut FFPsyContext,
    mut channel: libc::c_int,
    mut coefs: *const libc::c_float,
    mut wi: *const FFPsyWindowInfo,
) {
    let mut pctx: *mut AacPsyContext = (*ctx).model_priv_data as *mut AacPsyContext;
    let mut pch: *mut AacPsyChannel =
        &mut *((*pctx).ch).offset(channel as isize) as *mut AacPsyChannel;
    let mut i: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut desired_bits: libc::c_float = 0.;
    let mut desired_pe: libc::c_float = 0.;
    let mut delta_pe: libc::c_float = 0.;
    let mut reduction: libc::c_float = ::core::f32::NAN;
    let mut spread_en: [libc::c_float; 128] = [
        0 as libc::c_int as libc::c_float,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ];
    let mut a: libc::c_float = 0.0f32;
    let mut active_lines: libc::c_float = 0.0f32;
    let mut norm_fac: libc::c_float = 0.0f32;
    let mut pe: libc::c_float = if (*pctx).chan_bitrate > 32000 as libc::c_int {
        0.0f32
    } else if 50.0f32 > 100.0f32 - (*pctx).chan_bitrate as libc::c_float * 100.0f32 / 32000.0f32 {
        50.0f32
    } else {
        100.0f32 - (*pctx).chan_bitrate as libc::c_float * 100.0f32 / 32000.0f32
    };
    let num_bands: libc::c_int =
        *((*ctx).num_bands).offset(((*wi).num_windows == 8 as libc::c_int) as libc::c_int as isize);
    let mut band_sizes: *const uint8_t =
        *((*ctx).bands).offset(((*wi).num_windows == 8 as libc::c_int) as libc::c_int as isize);
    let mut coeffs: *mut AacPsyCoeffs = ((*pctx).psy_coef
        [((*wi).num_windows == 8 as libc::c_int) as libc::c_int as usize])
        .as_mut_ptr();
    let avoid_hole_thr: libc::c_float = if (*wi).num_windows == 8 as libc::c_int {
        0.63f32
    } else {
        0.5f32
    };
    let bandwidth: libc::c_int = (if (*ctx).cutoff != 0 {
        (*ctx).cutoff as libc::c_long
    } else if (*(*ctx).avctx).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        ((*(*ctx).avctx).sample_rate / 2 as libc::c_int) as libc::c_long
    } else if (*(*ctx).avctx).bit_rate != 0 {
        if (if (if (if (if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        }) > 3000 as libc::c_int as libc::c_long
            + (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 4 as libc::c_int as libc::c_long
        {
            3000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 4 as libc::c_int as libc::c_long
        } else if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        }) > 12000 as libc::c_int as libc::c_long
            + (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 16 as libc::c_int as libc::c_long
        {
            12000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 16 as libc::c_int as libc::c_long
        } else if (if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        }) > 3000 as libc::c_int as libc::c_long
            + (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 4 as libc::c_int as libc::c_long
        {
            3000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 4 as libc::c_int as libc::c_long
        } else if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        }) > 22000 as libc::c_int as libc::c_long
        {
            22000 as libc::c_int as libc::c_long
        } else if (if (if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        }) > 3000 as libc::c_int as libc::c_long
            + (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 4 as libc::c_int as libc::c_long
        {
            3000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 4 as libc::c_int as libc::c_long
        } else if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        }) > 12000 as libc::c_int as libc::c_long
            + (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 16 as libc::c_int as libc::c_long
        {
            12000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 16 as libc::c_int as libc::c_long
        } else if (if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        }) > 3000 as libc::c_int as libc::c_long
            + (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 4 as libc::c_int as libc::c_long
        {
            3000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 4 as libc::c_int as libc::c_long
        } else if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        }) > ((*(*ctx).avctx).sample_rate / 2 as libc::c_int) as libc::c_long
        {
            ((*(*ctx).avctx).sample_rate / 2 as libc::c_int) as libc::c_long
        } else if (if (if (if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        }) > 3000 as libc::c_int as libc::c_long
            + (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 4 as libc::c_int as libc::c_long
        {
            3000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 4 as libc::c_int as libc::c_long
        } else if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        }) > 12000 as libc::c_int as libc::c_long
            + (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 16 as libc::c_int as libc::c_long
        {
            12000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 16 as libc::c_int as libc::c_long
        } else if (if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        }) > 3000 as libc::c_int as libc::c_long
            + (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 4 as libc::c_int as libc::c_long
        {
            3000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 4 as libc::c_int as libc::c_long
        } else if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        }) > 22000 as libc::c_int as libc::c_long
        {
            22000 as libc::c_int as libc::c_long
        } else if (if (if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        }) > 3000 as libc::c_int as libc::c_long
            + (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 4 as libc::c_int as libc::c_long
        {
            3000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 4 as libc::c_int as libc::c_long
        } else if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        }) > 12000 as libc::c_int as libc::c_long
            + (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 16 as libc::c_int as libc::c_long
        {
            12000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 16 as libc::c_int as libc::c_long
        } else if (if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        }) > 3000 as libc::c_int as libc::c_long
            + (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 4 as libc::c_int as libc::c_long
        {
            3000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 4 as libc::c_int as libc::c_long
        } else if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long
                / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        }
    } else {
        ((*(*ctx).avctx).sample_rate / 2 as libc::c_int) as libc::c_long
    }) as libc::c_int;
    let cutoff: libc::c_int =
        bandwidth * 2048 as libc::c_int / (*wi).num_windows / (*(*ctx).avctx).sample_rate;
    calc_thr_3gpp(wi, num_bands, pch, band_sizes, coefs, cutoff);
    w = 0 as libc::c_int;
    while w < (*wi).num_windows * 16 as libc::c_int {
        let mut bands: *mut AacPsyBand =
            &mut *((*pch).band).as_mut_ptr().offset(w as isize) as *mut AacPsyBand;
        spread_en[0 as libc::c_int as usize] = (*bands.offset(0 as libc::c_int as isize)).energy;
        g = 1 as libc::c_int;
        while g < num_bands {
            (*bands.offset(g as isize)).thr = if (*bands.offset(g as isize)).thr
                > (*bands.offset((g - 1 as libc::c_int) as isize)).thr
                    * (*coeffs.offset(g as isize)).spread_hi[0 as libc::c_int as usize]
            {
                (*bands.offset(g as isize)).thr
            } else {
                (*bands.offset((g - 1 as libc::c_int) as isize)).thr
                    * (*coeffs.offset(g as isize)).spread_hi[0 as libc::c_int as usize]
            };
            spread_en[(w + g) as usize] = if (*bands.offset(g as isize)).energy
                > spread_en[(w + g - 1 as libc::c_int) as usize]
                    * (*coeffs.offset(g as isize)).spread_hi[1 as libc::c_int as usize]
            {
                (*bands.offset(g as isize)).energy
            } else {
                spread_en[(w + g - 1 as libc::c_int) as usize]
                    * (*coeffs.offset(g as isize)).spread_hi[1 as libc::c_int as usize]
            };
            g += 1;
            g;
        }
        g = num_bands - 2 as libc::c_int;
        while g >= 0 as libc::c_int {
            (*bands.offset(g as isize)).thr = if (*bands.offset(g as isize)).thr
                > (*bands.offset((g + 1 as libc::c_int) as isize)).thr
                    * (*coeffs.offset(g as isize)).spread_low[0 as libc::c_int as usize]
            {
                (*bands.offset(g as isize)).thr
            } else {
                (*bands.offset((g + 1 as libc::c_int) as isize)).thr
                    * (*coeffs.offset(g as isize)).spread_low[0 as libc::c_int as usize]
            };
            spread_en[(w + g) as usize] = if spread_en[(w + g) as usize]
                > spread_en[(w + g + 1 as libc::c_int) as usize]
                    * (*coeffs.offset(g as isize)).spread_low[1 as libc::c_int as usize]
            {
                spread_en[(w + g) as usize]
            } else {
                spread_en[(w + g + 1 as libc::c_int) as usize]
                    * (*coeffs.offset(g as isize)).spread_low[1 as libc::c_int as usize]
            };
            g -= 1;
            g;
        }
        g = 0 as libc::c_int;
        while g < num_bands {
            let mut band: *mut AacPsyBand = &mut *bands.offset(g as isize) as *mut AacPsyBand;
            (*band).thr = if (*band).thr > (*coeffs.offset(g as isize)).ath {
                (*band).thr
            } else {
                (*coeffs.offset(g as isize)).ath
            };
            (*band).thr_quiet = (*band).thr;
            if !((*wi).window_type[0 as libc::c_int as usize] == LONG_STOP_SEQUENCE as libc::c_int
                || w == 0
                    && (*wi).window_type[1 as libc::c_int as usize]
                        == LONG_START_SEQUENCE as libc::c_int)
            {
                (*band).thr = if 0.01f32 * (*band).thr
                    > (if (*band).thr > 2.0f32 * (*pch).prev_band[(w + g) as usize].thr_quiet {
                        2.0f32 * (*pch).prev_band[(w + g) as usize].thr_quiet
                    } else {
                        (*band).thr
                    }) {
                    0.01f32 * (*band).thr
                } else if (*band).thr > 2.0f32 * (*pch).prev_band[(w + g) as usize].thr_quiet {
                    2.0f32 * (*pch).prev_band[(w + g) as usize].thr_quiet
                } else {
                    (*band).thr
                };
            }
            pe += calc_pe_3gpp(band);
            a += (*band).pe_const;
            active_lines += (*band).active_lines;
            if spread_en[(w + g) as usize] * avoid_hole_thr > (*band).energy
                || (*coeffs.offset(g as isize)).min_snr > 1.0f32
            {
                (*band).avoid_holes = PSY_3GPP_AH_NONE as libc::c_int;
            } else {
                (*band).avoid_holes = PSY_3GPP_AH_INACTIVE as libc::c_int;
            }
            g += 1;
            g;
        }
        w += 16 as libc::c_int;
    }
    (*((*ctx).ch).offset(channel as isize)).entropy = pe;
    if (*(*ctx).avctx).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        desired_pe = pe
            * (if (*(*ctx).avctx).global_quality != 0 {
                (*(*ctx).avctx).global_quality
            } else {
                120 as libc::c_int
            }) as libc::c_float
            / (2 as libc::c_int as libc::c_float * 2.5f32 * 120.0f32);
        desired_bits = if 2560 as libc::c_int as libc::c_float > desired_pe / 1.18f32 {
            desired_pe / 1.18f32
        } else {
            2560 as libc::c_int as libc::c_float
        };
        desired_pe = desired_bits * 1.18f32;
        if (*ctx).bitres.bits > 0 as libc::c_int {
            desired_bits = if 2560 as libc::c_int as libc::c_float > desired_pe / 1.18f32 {
                desired_pe / 1.18f32
            } else {
                2560 as libc::c_int as libc::c_float
            };
            desired_pe = desired_bits * 1.18f32;
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
            ((*wi).num_windows == 8 as libc::c_int) as libc::c_int,
        ) as libc::c_float;
        desired_pe = desired_bits * 1.18f32;
        if (*ctx).bitres.bits > 0 as libc::c_int {
            desired_pe *= av_clipf_c(
                (*pctx).pe.previous / ((*ctx).bitres.bits as libc::c_float * 1.18f32),
                0.85f32,
                1.15f32,
            );
        }
    }
    (*pctx).pe.previous = desired_bits * 1.18f32;
    (*ctx).bitres.alloc = desired_bits as libc::c_int;
    if desired_pe < pe {
        w = 0 as libc::c_int;
        while w < (*wi).num_windows * 16 as libc::c_int {
            reduction = calc_reduction_3gpp(a, desired_pe, pe, active_lines);
            pe = 0.0f32;
            a = 0.0f32;
            active_lines = 0.0f32;
            g = 0 as libc::c_int;
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
            w += 16 as libc::c_int;
        }
        i = 0 as libc::c_int;
        while i < 2 as libc::c_int {
            let mut pe_no_ah: libc::c_float = 0.0f32;
            let mut desired_pe_no_ah: libc::c_float = 0.;
            a = 0.0f32;
            active_lines = a;
            w = 0 as libc::c_int;
            while w < (*wi).num_windows * 16 as libc::c_int {
                g = 0 as libc::c_int;
                while g < num_bands {
                    let mut band_1: *mut AacPsyBand =
                        &mut *((*pch).band).as_mut_ptr().offset((w + g) as isize)
                            as *mut AacPsyBand;
                    if (*band_1).avoid_holes != PSY_3GPP_AH_ACTIVE as libc::c_int {
                        pe_no_ah += (*band_1).pe;
                        a += (*band_1).pe_const;
                        active_lines += (*band_1).active_lines;
                    }
                    g += 1;
                    g;
                }
                w += 16 as libc::c_int;
            }
            desired_pe_no_ah = if desired_pe - (pe - pe_no_ah) > 0.0f32 {
                desired_pe - (pe - pe_no_ah)
            } else {
                0.0f32
            };
            if active_lines > 0.0f32 {
                reduction = calc_reduction_3gpp(a, desired_pe_no_ah, pe_no_ah, active_lines);
            }
            pe = 0.0f32;
            w = 0 as libc::c_int;
            while w < (*wi).num_windows * 16 as libc::c_int {
                g = 0 as libc::c_int;
                while g < num_bands {
                    let mut band_2: *mut AacPsyBand =
                        &mut *((*pch).band).as_mut_ptr().offset((w + g) as isize)
                            as *mut AacPsyBand;
                    if active_lines > 0.0f32 {
                        (*band_2).thr = calc_reduced_thr_3gpp(
                            band_2,
                            (*coeffs.offset(g as isize)).min_snr,
                            reduction,
                        );
                    }
                    pe += calc_pe_3gpp(band_2);
                    if (*band_2).thr > 0.0f32 {
                        (*band_2).norm_fac = (*band_2).active_lines / (*band_2).thr;
                    } else {
                        (*band_2).norm_fac = 0.0f32;
                    }
                    norm_fac += (*band_2).norm_fac;
                    g += 1;
                    g;
                }
                w += 16 as libc::c_int;
            }
            delta_pe = desired_pe - pe;
            if fabs(delta_pe as libc::c_double) > (0.05f32 * desired_pe) as libc::c_double {
                break;
            }
            i += 1;
            i;
        }
        if pe < 1.15f32 * desired_pe {
            norm_fac = if norm_fac != 0. {
                1.0f32 / norm_fac
            } else {
                0 as libc::c_int as libc::c_float
            };
            w = 0 as libc::c_int;
            while w < (*wi).num_windows * 16 as libc::c_int {
                g = 0 as libc::c_int;
                while g < num_bands {
                    let mut band_3: *mut AacPsyBand =
                        &mut *((*pch).band).as_mut_ptr().offset((w + g) as isize)
                            as *mut AacPsyBand;
                    if (*band_3).active_lines > 0.5f32 {
                        let mut delta_sfb_pe: libc::c_float =
                            (*band_3).norm_fac * norm_fac * delta_pe;
                        let mut thr: libc::c_float = (*band_3).thr;
                        thr *= exp2f(delta_sfb_pe / (*band_3).active_lines);
                        if thr > (*coeffs.offset(g as isize)).min_snr * (*band_3).energy
                            && (*band_3).avoid_holes == PSY_3GPP_AH_INACTIVE as libc::c_int
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
                w += 16 as libc::c_int;
            }
        } else {
            g = num_bands;
            while pe > desired_pe && {
                let fresh0 = g;
                g -= 1;
                fresh0 != 0
            } {
                w = 0 as libc::c_int;
                while w < (*wi).num_windows * 16 as libc::c_int {
                    let mut band_4: *mut AacPsyBand =
                        &mut *((*pch).band).as_mut_ptr().offset((w + g) as isize)
                            as *mut AacPsyBand;
                    if (*band_4).avoid_holes != PSY_3GPP_AH_NONE as libc::c_int
                        && (*coeffs.offset(g as isize)).min_snr < 7.943_282e-1_f32
                    {
                        (*coeffs.offset(g as isize)).min_snr = 7.943_282e-1_f32;
                        (*band_4).thr = (*band_4).energy * 7.943_282e-1_f32;
                        pe += (*band_4).active_lines * 1.5f32 - (*band_4).pe;
                    }
                    w += 16 as libc::c_int;
                }
            }
        }
    }
    w = 0 as libc::c_int;
    while w < (*wi).num_windows * 16 as libc::c_int {
        g = 0 as libc::c_int;
        while g < num_bands {
            let mut band_5: *mut AacPsyBand =
                &mut *((*pch).band).as_mut_ptr().offset((w + g) as isize) as *mut AacPsyBand;
            let mut psy_band: *mut FFPsyBand =
                &mut *((*((*ctx).ch).offset(channel as isize)).psy_bands)
                    .as_mut_ptr()
                    .offset((w + g) as isize) as *mut FFPsyBand;
            (*psy_band).threshold = (*band_5).thr;
            (*psy_band).energy = (*band_5).energy;
            (*psy_band).spread = (*band_5).active_lines * 2.0f32
                / *band_sizes.offset(g as isize) as libc::c_int as libc::c_float;
            (*psy_band).bits = ((*band_5).pe / 1.18f32) as libc::c_int;
            g += 1;
            g;
        }
        w += 16 as libc::c_int;
    }
    (*pch).prev_band = (*pch).band;
}
unsafe extern "C" fn psy_3gpp_analyze(
    mut ctx: *mut FFPsyContext,
    mut channel: libc::c_int,
    mut coeffs: *mut *const libc::c_float,
    mut wi: *const FFPsyWindowInfo,
) {
    let mut ch: libc::c_int = 0;
    let mut group: *mut FFPsyChannelGroup = ff_psy_find_group(ctx, channel);
    ch = 0 as libc::c_int;
    while ch < (*group).num_ch as libc::c_int {
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
unsafe extern "C" fn psy_3gpp_end(mut apc: *mut FFPsyContext) {
    let mut pctx: *mut AacPsyContext = (*apc).model_priv_data as *mut AacPsyContext;
    // TODO: leaks 🚿
    if !pctx.is_null() {
        // av_freep(&mut (*pctx).ch as *mut *mut AacPsyChannel as *mut libc::c_void);
    }
    // av_freep(&mut (*apc).model_priv_data as *mut *mut libc::c_void as *mut libc::c_void);
}
unsafe extern "C" fn lame_apply_block_type(
    mut ctx: *mut AacPsyChannel,
    mut wi: *mut FFPsyWindowInfo,
    mut uselongblock: libc::c_int,
) {
    let mut blocktype: libc::c_int = ONLY_LONG_SEQUENCE as libc::c_int;
    if uselongblock != 0 {
        if (*ctx).next_window_seq as libc::c_uint
            == EIGHT_SHORT_SEQUENCE as libc::c_int as libc::c_uint
        {
            blocktype = LONG_STOP_SEQUENCE as libc::c_int;
        }
    } else {
        blocktype = EIGHT_SHORT_SEQUENCE as libc::c_int;
        if (*ctx).next_window_seq as libc::c_uint
            == ONLY_LONG_SEQUENCE as libc::c_int as libc::c_uint
        {
            (*ctx).next_window_seq = LONG_START_SEQUENCE;
        }
        if (*ctx).next_window_seq as libc::c_uint
            == LONG_STOP_SEQUENCE as libc::c_int as libc::c_uint
        {
            (*ctx).next_window_seq = EIGHT_SHORT_SEQUENCE;
        }
    }
    (*wi).window_type[0 as libc::c_int as usize] = (*ctx).next_window_seq as libc::c_int;
    (*ctx).next_window_seq = blocktype as WindowSequence;
}
unsafe extern "C" fn psy_lame_window(
    mut ctx: *mut FFPsyContext,
    mut _audio: *const libc::c_float,
    mut la: *const libc::c_float,
    mut channel: libc::c_int,
    mut prev_type: libc::c_int,
) -> FFPsyWindowInfo {
    let mut pctx: *mut AacPsyContext = (*ctx).model_priv_data as *mut AacPsyContext;
    let mut pch: *mut AacPsyChannel =
        &mut *((*pctx).ch).offset(channel as isize) as *mut AacPsyChannel;
    let mut grouping: libc::c_int = 0 as libc::c_int;
    let mut uselongblock: libc::c_int = 1 as libc::c_int;
    let mut attacks: [libc::c_int; 9] = [0 as libc::c_int, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut i: libc::c_int = 0;
    let mut wi: FFPsyWindowInfo = {
        FFPsyWindowInfo {
            window_type: [0 as libc::c_int, 0, 0],
            window_shape: 0,
            num_windows: 0,
            grouping: [0; 8],
            clipping: [0.; 8],
            window_sizes: std::ptr::null_mut::<libc::c_int>(),
        }
    };
    if !la.is_null() {
        let mut hpfsmpl: [libc::c_float; 1024] = [0.; 1024];
        let mut pf: *const libc::c_float = hpfsmpl.as_mut_ptr();
        let mut attack_intensity: [libc::c_float; 27] = [0.; 27];
        let mut energy_subshort: [libc::c_float; 27] = [0.; 27];
        let mut energy_short: [libc::c_float; 9] = [
            0 as libc::c_int as libc::c_float,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
        ];
        let mut firbuf: *const libc::c_float =
            la.offset((128 as libc::c_int / 4 as libc::c_int - 21 as libc::c_int) as isize);
        let mut att_sum: libc::c_int = 0 as libc::c_int;
        psy_hp_filter(firbuf, hpfsmpl.as_mut_ptr(), psy_fir_coeffs.as_ptr());
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            energy_subshort[i as usize] = (*pch).prev_energy_subshort
                [(i + (8 as libc::c_int - 1 as libc::c_int) * 3 as libc::c_int) as usize];
            attack_intensity[i as usize] = energy_subshort[i as usize]
                / (*pch).prev_energy_subshort[(i
                    + ((8 as libc::c_int - 2 as libc::c_int) * 3 as libc::c_int + 1 as libc::c_int))
                    as usize];
            energy_short[0 as libc::c_int as usize] += energy_subshort[i as usize];
            i += 1;
            i;
        }
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int * 3 as libc::c_int {
            let pfe: *const libc::c_float =
                pf.offset((1024 as libc::c_int / (8 as libc::c_int * 3 as libc::c_int)) as isize);
            let mut p: libc::c_float = 1.0f32;
            while pf < pfe {
                p = if p > fabsf(*pf) { p } else { fabsf(*pf) };
                pf = pf.offset(1);
                pf;
            }
            energy_subshort[(i + 3 as libc::c_int) as usize] = p;
            (*pch).prev_energy_subshort[i as usize] =
                energy_subshort[(i + 3 as libc::c_int) as usize];
            energy_short[(1 as libc::c_int + i / 3 as libc::c_int) as usize] += p;
            if p > energy_subshort[(i + 1 as libc::c_int) as usize] {
                p /= energy_subshort[(i + 1 as libc::c_int) as usize];
            } else if energy_subshort[(i + 1 as libc::c_int) as usize] > p * 10.0f32 {
                p = energy_subshort[(i + 1 as libc::c_int) as usize] / (p * 10.0f32);
            } else {
                p = 0.0f64 as libc::c_float;
            }
            attack_intensity[(i + 3 as libc::c_int) as usize] = p;
            i += 1;
            i;
        }
        i = 0 as libc::c_int;
        while i < (8 as libc::c_int + 1 as libc::c_int) * 3 as libc::c_int {
            if attacks[(i / 3 as libc::c_int) as usize] == 0
                && attack_intensity[i as usize] > (*pch).attack_threshold
            {
                attacks[(i / 3 as libc::c_int) as usize] = i % 3 as libc::c_int + 1 as libc::c_int;
            }
            i += 1;
            i;
        }
        i = 1 as libc::c_int;
        while i < 8 as libc::c_int + 1 as libc::c_int {
            let u: libc::c_float = energy_short[(i - 1 as libc::c_int) as usize];
            let v: libc::c_float = energy_short[i as usize];
            let m: libc::c_float = if u > v { u } else { v };
            if m < 40000 as libc::c_int as libc::c_float && u < 1.7f32 * v && v < 1.7f32 * u {
                if i == 1 as libc::c_int && attacks[0 as libc::c_int as usize] < attacks[i as usize]
                {
                    attacks[0 as libc::c_int as usize] = 0 as libc::c_int;
                }
                attacks[i as usize] = 0 as libc::c_int;
            }
            att_sum += attacks[i as usize];
            i += 1;
            i;
        }
        if attacks[0 as libc::c_int as usize] <= (*pch).prev_attack {
            attacks[0 as libc::c_int as usize] = 0 as libc::c_int;
        }
        att_sum += attacks[0 as libc::c_int as usize];
        if (*pch).prev_attack == 3 as libc::c_int || att_sum != 0 {
            uselongblock = 0 as libc::c_int;
            i = 1 as libc::c_int;
            while i < 8 as libc::c_int + 1 as libc::c_int {
                if attacks[i as usize] != 0 && attacks[(i - 1 as libc::c_int) as usize] != 0 {
                    attacks[i as usize] = 0 as libc::c_int;
                }
                i += 1;
                i;
            }
        }
    } else {
        uselongblock = !(prev_type == EIGHT_SHORT_SEQUENCE as libc::c_int) as libc::c_int;
    }
    lame_apply_block_type(pch, &mut wi, uselongblock);
    wi.window_type[1 as libc::c_int as usize] = prev_type;
    if wi.window_type[0 as libc::c_int as usize] != EIGHT_SHORT_SEQUENCE as libc::c_int {
        wi.num_windows = 1 as libc::c_int;
        wi.grouping[0 as libc::c_int as usize] = 1 as libc::c_int;
        if wi.window_type[0 as libc::c_int as usize] == LONG_START_SEQUENCE as libc::c_int {
            wi.window_shape = 0 as libc::c_int;
        } else {
            wi.window_shape = 1 as libc::c_int;
        }
    } else {
        let mut lastgrp: libc::c_int = 0 as libc::c_int;
        wi.num_windows = 8 as libc::c_int;
        wi.window_shape = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            if (*pch).next_grouping as libc::c_int >> i & 1 as libc::c_int == 0 {
                lastgrp = i;
            }
            wi.grouping[lastgrp as usize] += 1;
            wi.grouping[lastgrp as usize];
            i += 1;
            i;
        }
    }
    i = 0 as libc::c_int;
    while i < 9 as libc::c_int {
        if attacks[i as usize] != 0 {
            grouping = i;
            break;
        } else {
            i += 1;
            i;
        }
    }
    (*pch).next_grouping = window_grouping[grouping as usize];
    (*pch).prev_attack = attacks[8 as libc::c_int as usize];
    wi
}
#[no_mangle]
pub static mut ff_aac_psy_model: FFPsyModel = unsafe {
    {
        FFPsyModel {
            name: b"3GPP TS 26.403-inspired model\0" as *const u8 as *const libc::c_char,
            init: Some(psy_3gpp_init as unsafe extern "C" fn(*mut FFPsyContext) -> libc::c_int),
            window: Some(
                psy_lame_window
                    as unsafe extern "C" fn(
                        *mut FFPsyContext,
                        *const libc::c_float,
                        *const libc::c_float,
                        libc::c_int,
                        libc::c_int,
                    ) -> FFPsyWindowInfo,
            ),
            analyze: Some(
                psy_3gpp_analyze
                    as unsafe extern "C" fn(
                        *mut FFPsyContext,
                        libc::c_int,
                        *mut *const libc::c_float,
                        *const FFPsyWindowInfo,
                    ) -> (),
            ),
            end: Some(psy_3gpp_end as unsafe extern "C" fn(*mut FFPsyContext) -> ()),
        }
    }
};
