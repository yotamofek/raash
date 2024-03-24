#![allow(
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use std::ptr::addr_of;

use ffi::codec::AVCodecContext;
use ffmpeg_src_macro::ffmpeg_src;
use libc::{c_int, c_uchar};

use crate::{aac::psy_model::ff_aac_psy_model, types::*};

#[ffmpeg_src(file = "libavcodec/psymodel.h", lines = 41..=45, name = "AAC_CUTOFF")]
pub(crate) unsafe fn aac_cutoff(ctx: *const AVCodecContext) -> c_int {
    if (*ctx).flags & AV_CODEC_FLAG_QSCALE != 0 {
        (*ctx).sample_rate / 2
    } else {
        cutoff_from_bitrate(
            (*ctx).bit_rate.try_into().unwrap(),
            (*ctx).ch_layout.nb_channels,
            (*ctx).sample_rate,
        )
    }
}

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

#[cold]
pub(crate) unsafe fn ff_psy_init(
    mut ctx: *mut FFPsyContext,
    mut avctx: *mut AVCodecContext,
    mut bands: &[&'static [c_uchar]],
    mut num_bands: &[c_int],
    mut num_groups: c_int,
    mut group_map: *const c_uchar,
) -> c_int {
    assert_eq!(bands.len(), num_bands.len());
    let mut i: c_int = 0;
    (*ctx).avctx = avctx;
    (*ctx).ch = vec![FFPsyChannel::default(); (*avctx).ch_layout.nb_channels as usize * 2]
        .into_boxed_slice();
    (*ctx).group = vec![FFPsyChannelGroup::default(); num_groups as usize].into_boxed_slice();
    (*ctx).bands = bands.to_vec().into_boxed_slice();
    (*ctx).num_bands = num_bands.to_vec().into_boxed_slice();
    (*ctx).cutoff = (*avctx).cutoff;
    i = 0;
    while i < num_groups {
        (*ctx).group[i as usize].num_ch = (*group_map.offset(i as isize) as c_int + 1) as c_uchar;
        i += 1;
        i;
    }
    if (*(*ctx).avctx).codec_id == AV_CODEC_ID_AAC {
        (*ctx).model = addr_of!(ff_aac_psy_model);
    }
    if ((*(*ctx).model).init).is_some() {
        return ((*(*ctx).model).init).expect("non-null function pointer")(ctx);
    }
    0
}

pub(crate) unsafe fn ff_psy_find_group(
    mut ctx: *mut FFPsyContext,
    mut channel: c_int,
) -> *mut FFPsyChannelGroup {
    let mut i: c_int = 0;
    let mut ch: c_int = 0;
    while ch <= channel {
        let fresh2 = i;
        i += 1;
        ch += (*ctx).group[fresh2 as usize].num_ch as c_int;
    }
    &mut (*ctx).group[(i - 1) as usize] as *mut FFPsyChannelGroup
}

#[cold]
pub(crate) unsafe fn ff_psy_end(mut ctx: *mut FFPsyContext) {
    if !((*ctx).model).is_null() && ((*(*ctx).model).end).is_some() {
        ((*(*ctx).model).end).expect("non-null function pointer")(ctx);
    }
    // TODO: leaks 🚿
    // av_freep(&mut (*ctx).bands as *mut *mut *mut uint8_t as *mut c_void);
    // av_freep(&mut (*ctx).num_bands as *mut *mut c_int as *mut c_void);
    // av_freep(&mut (*ctx).group as *mut *mut FFPsyChannelGroup as *mut
    // c_void); av_freep(&mut (*ctx).ch as *mut *mut FFPsyChannel as *mut
    // c_void);
}
