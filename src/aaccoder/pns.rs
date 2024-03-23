use std::ptr;

use ffi::codec::AVCodecContext;
use libc::{c_double, c_float, c_int, c_long, c_uchar, c_uint};

use super::{
    ff_init_nextband_map, ff_sfdelta_can_remove_band, math::lcg_random, quantize_band_cost,
};
use crate::{
    aacenc::{abs_pow34_v, ctx::AACEncContext},
    aactab::POW_SF_TABLES,
    common::*,
    types::*,
};

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
    let mut wlen: c_int = 1024 as c_int / (*sce).ics.num_windows;
    let mut bandwidth: c_int = 0;
    let mut cutoff: c_int = 0;
    let mut PNS: *mut c_float = &mut *((*s).scoefs)
        .as_mut_ptr()
        .offset((0 as c_int * 128 as c_int) as isize)
        as *mut c_float;
    let mut PNS34: *mut c_float = &mut *((*s).scoefs)
        .as_mut_ptr()
        .offset((1 as c_int * 128 as c_int) as isize)
        as *mut c_float;
    let mut NOR34: *mut c_float = &mut *((*s).scoefs)
        .as_mut_ptr()
        .offset((3 as c_int * 128 as c_int) as isize)
        as *mut c_float;
    let mut nextband: [c_uchar; 128] = [0; 128];
    let lambda: c_float = (*s).lambda;
    let freq_mult: c_float = (*avctx).sample_rate as c_float * 0.5f32 / wlen as c_float;
    let thr_mult: c_float = 1.948f32 * (100.0f32 / lambda);
    let spread_threshold: c_float = if 0.75f32
        > 0.9f32
            * (if 0.5f32 > lambda / 100.0f32 {
                0.5f32
            } else {
                lambda / 100.0f32
            }) {
        0.9f32
            * (if 0.5f32 > lambda / 100.0f32 {
                0.5f32
            } else {
                lambda / 100.0f32
            })
    } else {
        0.75f32
    };
    let dist_bias: c_float = (4.0f32 * 120 as c_int as c_float / lambda).clamp(0.25, 4.);
    let pns_transient_energy_r: c_float = if 0.7f32 > lambda / 140.0f32 {
        lambda / 140.0f32
    } else {
        0.7f32
    };
    let mut refbits: c_int = ((*avctx).bit_rate as c_double * 1024.0f64
        / (*avctx).sample_rate as c_double
        / (if (*avctx).flags & AV_CODEC_FLAG_QSCALE != 0 {
            2.0f32
        } else {
            (*avctx).ch_layout.nb_channels as c_float
        }) as c_double
        * (lambda / 120.0f32) as c_double) as c_int;
    let mut rate_bandwidth_multiplier: c_float = 1.5f32;
    let mut prev: c_int = -(1000 as c_int);
    let mut prev_sf: c_int = -(1 as c_int);
    let mut frame_bit_rate: c_int = (if (*avctx).flags & AV_CODEC_FLAG_QSCALE != 0 {
        refbits as c_float * rate_bandwidth_multiplier * (*avctx).sample_rate as c_float
            / 1024 as c_int as c_float
    } else {
        ((*avctx).bit_rate / (*avctx).ch_layout.nb_channels as c_long) as c_float
    }) as c_int;
    frame_bit_rate = (frame_bit_rate as c_float * 1.15f32) as c_int;
    if (*avctx).cutoff > 0 as c_int {
        bandwidth = (*avctx).cutoff;
    } else {
        bandwidth = if 3000 as c_int
            > (if frame_bit_rate != 0 {
                if (if (if (if (if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                {
                    3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                } else if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
                {
                    12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
                } else if (if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                {
                    3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                } else if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 22000 as c_int
                {
                    22000 as c_int
                } else if (if (if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                {
                    3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                } else if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
                {
                    12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
                } else if (if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                {
                    3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                } else if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > (*avctx).sample_rate / 2 as c_int
                {
                    (*avctx).sample_rate / 2 as c_int
                } else if (if (if (if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                {
                    3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                } else if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
                {
                    12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
                } else if (if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                {
                    3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                } else if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 22000 as c_int
                {
                    22000 as c_int
                } else if (if (if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                {
                    3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                } else if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
                {
                    12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
                } else if (if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                {
                    3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                } else if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }
            } else {
                (*avctx).sample_rate / 2 as c_int
            }) {
            3000 as c_int
        } else if frame_bit_rate != 0 {
            if (if (if (if (if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            {
                3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            } else if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
            {
                12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
            } else if (if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            {
                3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            } else if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 22000 as c_int
            {
                22000 as c_int
            } else if (if (if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            {
                3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            } else if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
            {
                12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
            } else if (if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            {
                3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            } else if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > (*avctx).sample_rate / 2 as c_int
            {
                (*avctx).sample_rate / 2 as c_int
            } else if (if (if (if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            {
                3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            } else if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
            {
                12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
            } else if (if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            {
                3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            } else if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 22000 as c_int
            {
                22000 as c_int
            } else if (if (if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            {
                3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            } else if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
            {
                12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
            } else if (if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            {
                3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            } else if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }
        } else {
            (*avctx).sample_rate / 2 as c_int
        };
    }
    cutoff = bandwidth * 2 as c_int * wlen / (*avctx).sample_rate;
    (*sce).band_alt = (*sce).band_type;
    ff_init_nextband_map(sce, nextband.as_mut_ptr());
    w = 0 as c_int;
    while w < (*sce).ics.num_windows {
        let mut wstart: c_int = w * 128 as c_int;
        let mut current_block_67: u64;
        g = 0 as c_int;
        while g < (*sce).ics.num_swb {
            let mut noise_sfi: c_int = 0;
            let mut dist1: c_float = 0.0f32;
            let mut dist2: c_float = 0.0f32;
            let mut noise_amp: c_float = 0.;
            let mut pns_energy: c_float = 0.0f32;
            let mut pns_tgt_energy: c_float = 0.;
            let mut energy_ratio: c_float = 0.;
            let mut dist_thresh: c_float = 0.;
            let mut sfb_energy: c_float = 0.0f32;
            let mut threshold: c_float = 0.0f32;
            let mut spread: c_float = 2.0f32;
            let mut min_energy: c_float = -1.0f32;
            let mut max_energy: c_float = 0.0f32;
            let start: c_int = wstart + *((*sce).ics.swb_offset).offset(g as isize) as c_int;
            let freq: c_float = (start - wstart) as c_float * freq_mult;
            let freq_boost: c_float = if 0.88f32 * freq / 4000 as c_int as c_float > 1.0f32 {
                0.88f32 * freq / 4000 as c_int as c_float
            } else {
                1.0f32
            };
            if freq < 4000 as c_int as c_float || start - wstart >= cutoff {
                if (*sce).zeroes[(w * 16 as c_int + g) as usize] == 0 {
                    prev_sf = (*sce).sf_idx[(w * 16 as c_int + g) as usize];
                }
            } else {
                w2 = 0 as c_int;
                while w2 < (*sce).ics.group_len[w as usize] as c_int {
                    band = &mut (*s).psy.ch[(*s).cur_channel as usize].psy_bands
                        [((w + w2) * 16 as c_int + g) as usize]
                        as *mut FFPsyBand;
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
                dist_thresh =
                    (2.5f32 * 4000 as c_int as c_float / freq).clamp(0.5, 2.5) * dist_bias;
                if (*sce).zeroes[(w * 16 as c_int + g) as usize] == 0
                    && ff_sfdelta_can_remove_band(
                        sce,
                        nextband.as_mut_ptr(),
                        prev_sf,
                        w * 16 as c_int + g,
                    ) == 0
                    || ((*sce).zeroes[(w * 16 as c_int + g) as usize] as c_int != 0
                        || (*sce).band_alt[(w * 16 as c_int + g) as usize] as u64 == 0)
                        && sfb_energy < threshold * sqrtf(1.0f32 / freq_boost)
                    || spread < spread_threshold
                    || (*sce).zeroes[(w * 16 as c_int + g) as usize] == 0
                        && (*sce).band_alt[(w * 16 as c_int + g) as usize] as c_uint != 0
                        && sfb_energy > threshold * thr_mult * freq_boost
                    || min_energy < pns_transient_energy_r * max_energy
                {
                    (*sce).pns_ener[(w * 16 as c_int + g) as usize] = sfb_energy;
                    if (*sce).zeroes[(w * 16 as c_int + g) as usize] == 0 {
                        prev_sf = (*sce).sf_idx[(w * 16 as c_int + g) as usize];
                    }
                } else {
                    pns_tgt_energy = sfb_energy
                        * (if 1.0f32 > spread * spread {
                            spread * spread
                        } else {
                            1.0f32
                        });
                    noise_sfi = av_clip_c(
                        roundf(log2f(pns_tgt_energy) * 2 as c_int as c_float) as c_int,
                        -(100 as c_int),
                        155 as c_int,
                    );
                    noise_amp = -POW_SF_TABLES.pow2[(noise_sfi + 200 as c_int) as usize];
                    if prev != -(1000 as c_int) {
                        let mut noise_sfdiff: c_int = noise_sfi - prev + 60 as c_int;
                        if noise_sfdiff < 0 as c_int || noise_sfdiff > 2 as c_int * 60 as c_int {
                            if (*sce).zeroes[(w * 16 as c_int + g) as usize] == 0 {
                                prev_sf = (*sce).sf_idx[(w * 16 as c_int + g) as usize];
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
                            w2 = 0 as c_int;
                            while w2 < (*sce).ics.group_len[w as usize] as c_int {
                                let mut band_energy: c_float = 0.;
                                let mut scale: c_float = 0.;
                                let mut pns_senergy: c_float = 0.;
                                let start_c: c_int = (w + w2) * 128 as c_int
                                    + *((*sce).ics.swb_offset).offset(g as isize) as c_int;
                                band = &mut (*s).psy.ch[(*s).cur_channel as usize].psy_bands
                                    [((w + w2) * 16 as c_int + g) as usize]
                                    as *mut FFPsyBand;
                                i = 0 as c_int;
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
                                    (*sce).sf_idx[((w + w2) * 16 as c_int + g) as usize],
                                    (*sce).band_alt[((w + w2) * 16 as c_int + g) as usize] as c_int,
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
                                && (*sce).band_type[(w * 16 as c_int + g - 1 as c_int) as usize]
                                    as c_uint
                                    == NOISE_BT as c_int as c_uint
                            {
                                dist2 += 5 as c_int as c_float;
                            } else {
                                dist2 += 9 as c_int as c_float;
                            }
                            energy_ratio = pns_tgt_energy / pns_energy;
                            (*sce).pns_ener[(w * 16 as c_int + g) as usize] =
                                energy_ratio * pns_tgt_energy;
                            if (*sce).zeroes[(w * 16 as c_int + g) as usize] as c_int != 0
                                || (*sce).band_alt[(w * 16 as c_int + g) as usize] as u64 == 0
                                || energy_ratio > 0.85f32 && energy_ratio < 1.25f32 && dist2 < dist1
                            {
                                (*sce).band_type[(w * 16 as c_int + g) as usize] = NOISE_BT;
                                (*sce).zeroes[(w * 16 as c_int + g) as usize] =
                                    0 as c_int as c_uchar;
                                prev = noise_sfi;
                            } else if (*sce).zeroes[(w * 16 as c_int + g) as usize] == 0 {
                                prev_sf = (*sce).sf_idx[(w * 16 as c_int + g) as usize];
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

pub(crate) unsafe fn mark(
    mut s: *mut AACEncContext,
    mut avctx: *mut AVCodecContext,
    mut sce: *mut SingleChannelElement,
) {
    let mut band: *mut FFPsyBand = ptr::null_mut::<FFPsyBand>();
    let mut w: c_int = 0;
    let mut g: c_int = 0;
    let mut w2: c_int = 0;
    let mut wlen: c_int = 1024 as c_int / (*sce).ics.num_windows;
    let mut bandwidth: c_int = 0;
    let mut cutoff: c_int = 0;
    let lambda: c_float = (*s).lambda;
    let freq_mult: c_float = (*avctx).sample_rate as c_float * 0.5f32 / wlen as c_float;
    let spread_threshold: c_float = if 0.75f32
        > 0.9f32
            * (if 0.5f32 > lambda / 100.0f32 {
                0.5f32
            } else {
                lambda / 100.0f32
            }) {
        0.9f32
            * (if 0.5f32 > lambda / 100.0f32 {
                0.5f32
            } else {
                lambda / 100.0f32
            })
    } else {
        0.75f32
    };
    let pns_transient_energy_r: c_float = if 0.7f32 > lambda / 140.0f32 {
        lambda / 140.0f32
    } else {
        0.7f32
    };
    let mut refbits: c_int = ((*avctx).bit_rate as c_double * 1024.0f64
        / (*avctx).sample_rate as c_double
        / (if (*avctx).flags & AV_CODEC_FLAG_QSCALE != 0 {
            2.0f32
        } else {
            (*avctx).ch_layout.nb_channels as c_float
        }) as c_double
        * (lambda / 120.0f32) as c_double) as c_int;
    let mut rate_bandwidth_multiplier: c_float = 1.5f32;
    let mut frame_bit_rate: c_int = (if (*avctx).flags & AV_CODEC_FLAG_QSCALE != 0 {
        refbits as c_float * rate_bandwidth_multiplier * (*avctx).sample_rate as c_float
            / 1024 as c_int as c_float
    } else {
        ((*avctx).bit_rate / (*avctx).ch_layout.nb_channels as c_long) as c_float
    }) as c_int;
    frame_bit_rate = (frame_bit_rate as c_float * 1.15f32) as c_int;
    if (*avctx).cutoff > 0 as c_int {
        bandwidth = (*avctx).cutoff;
    } else {
        bandwidth = if 3000 as c_int
            > (if frame_bit_rate != 0 {
                if (if (if (if (if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                {
                    3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                } else if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
                {
                    12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
                } else if (if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                {
                    3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                } else if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 22000 as c_int
                {
                    22000 as c_int
                } else if (if (if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                {
                    3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                } else if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
                {
                    12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
                } else if (if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                {
                    3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                } else if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > (*avctx).sample_rate / 2 as c_int
                {
                    (*avctx).sample_rate / 2 as c_int
                } else if (if (if (if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                {
                    3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                } else if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
                {
                    12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
                } else if (if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                {
                    3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                } else if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 22000 as c_int
                {
                    22000 as c_int
                } else if (if (if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                {
                    3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                } else if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
                {
                    12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
                } else if (if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                {
                    3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                } else if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }
            } else {
                (*avctx).sample_rate / 2 as c_int
            }) {
            3000 as c_int
        } else if frame_bit_rate != 0 {
            if (if (if (if (if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            {
                3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            } else if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
            {
                12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
            } else if (if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            {
                3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            } else if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 22000 as c_int
            {
                22000 as c_int
            } else if (if (if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            {
                3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            } else if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
            {
                12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
            } else if (if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            {
                3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            } else if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > (*avctx).sample_rate / 2 as c_int
            {
                (*avctx).sample_rate / 2 as c_int
            } else if (if (if (if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            {
                3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            } else if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
            {
                12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
            } else if (if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            {
                3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            } else if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 22000 as c_int
            {
                22000 as c_int
            } else if (if (if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            {
                3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            } else if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
            {
                12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
            } else if (if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            {
                3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            } else if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }
        } else {
            (*avctx).sample_rate / 2 as c_int
        };
    }
    cutoff = bandwidth * 2 as c_int * wlen / (*avctx).sample_rate;
    (*sce).band_alt = (*sce).band_type;
    w = 0 as c_int;
    while w < (*sce).ics.num_windows {
        g = 0 as c_int;
        while g < (*sce).ics.num_swb {
            let mut sfb_energy: c_float = 0.0f32;
            let mut threshold: c_float = 0.0f32;
            let mut spread: c_float = 2.0f32;
            let mut min_energy: c_float = -1.0f32;
            let mut max_energy: c_float = 0.0f32;
            let start: c_int = *((*sce).ics.swb_offset).offset(g as isize) as c_int;
            let freq: c_float = start as c_float * freq_mult;
            let freq_boost: c_float = if 0.88f32 * freq / 4000 as c_int as c_float > 1.0f32 {
                0.88f32 * freq / 4000 as c_int as c_float
            } else {
                1.0f32
            };
            if freq < 4000 as c_int as c_float || start >= cutoff {
                (*sce).can_pns[(w * 16 as c_int + g) as usize] = 0 as c_int as c_uchar;
            } else {
                w2 = 0 as c_int;
                while w2 < (*sce).ics.group_len[w as usize] as c_int {
                    band = &mut (*s).psy.ch[(*s).cur_channel as usize].psy_bands
                        [((w + w2) * 16 as c_int + g) as usize]
                        as *mut FFPsyBand;
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
                (*sce).pns_ener[(w * 16 as c_int + g) as usize] = sfb_energy;
                if sfb_energy < threshold * sqrtf(1.5f32 / freq_boost)
                    || spread < spread_threshold
                    || min_energy < pns_transient_energy_r * max_energy
                {
                    (*sce).can_pns[(w * 16 as c_int + g) as usize] = 0 as c_int as c_uchar;
                } else {
                    (*sce).can_pns[(w * 16 as c_int + g) as usize] = 1 as c_int as c_uchar;
                }
            }
            g += 1;
            g;
        }
        w += (*sce).ics.group_len[w as usize] as c_int;
    }
}
