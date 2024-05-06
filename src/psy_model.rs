#![allow(
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use ffi::codec::AVCodecContext;
use ffmpeg_src_macro::ffmpeg_src;
use libc::{c_int, c_uchar};

use crate::{
    aac::psy_model::{psy_3gpp_end, psy_3gpp_init},
    types::*,
};

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
    assert_eq!((*(*ctx).avctx).codec_id, AV_CODEC_ID_AAC);
    psy_3gpp_init(ctx);
    0
}

#[ffmpeg_src(file = "libavcodec/psymodel.c", lines = 73..=81, name = "ff_psy_find_group")]
pub(crate) fn find_group(ctx: &FFPsyContext, channel: c_int) -> &FFPsyChannelGroup {
    let pos = ctx
        .group
        .iter()
        .scan(0, |channels, &FFPsyChannelGroup { num_ch }| {
            *channels += c_int::from(num_ch);
            Some(*channels)
        })
        .position(|ch| ch > channel)
        .unwrap();

    &ctx.group[pos]
}

#[cold]
pub(crate) unsafe fn ff_psy_end(mut ctx: *mut FFPsyContext) {
    psy_3gpp_end(ctx);
    // TODO: leaks 🚿
    // av_freep(&mut (*ctx).bands as *mut *mut *mut uint8_t as *mut c_void);
    // av_freep(&mut (*ctx).num_bands as *mut *mut c_int as *mut c_void);
    // av_freep(&mut (*ctx).group as *mut *mut FFPsyChannelGroup as *mut
    // c_void); av_freep(&mut (*ctx).ch as *mut *mut FFPsyChannel as *mut
    // c_void);
}
