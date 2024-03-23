#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use ffi::codec::AVCodecContext;
use libc::{c_int, c_uchar};

use crate::{aacpsy::ff_aac_psy_model, types::*};

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
    i = 0 as c_int;
    while i < num_groups {
        (*ctx).group[i as usize].num_ch =
            (*group_map.offset(i as isize) as c_int + 1 as c_int) as c_uchar;
        i += 1;
        i;
    }
    if (*(*ctx).avctx).codec_id == AV_CODEC_ID_AAC {
        (*ctx).model = &ff_aac_psy_model;
    }
    if ((*(*ctx).model).init).is_some() {
        return ((*(*ctx).model).init).expect("non-null function pointer")(ctx);
    }
    0 as c_int
}

pub(crate) unsafe fn ff_psy_find_group(
    mut ctx: *mut FFPsyContext,
    mut channel: c_int,
) -> *mut FFPsyChannelGroup {
    let mut i: c_int = 0 as c_int;
    let mut ch: c_int = 0 as c_int;
    while ch <= channel {
        let fresh2 = i;
        i += 1;
        ch += (*ctx).group[fresh2 as usize].num_ch as c_int;
    }
    &mut (*ctx).group[(i - 1 as c_int) as usize] as *mut FFPsyChannelGroup
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
