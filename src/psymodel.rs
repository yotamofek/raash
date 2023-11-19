#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use crate::{
    aacpsy::ff_aac_psy_model,
    iirfilter::{
        ff_iir_filter_free_coeffsp, ff_iir_filter_free_statep, ff_iir_filter_init,
        ff_iir_filter_init_coeffs, ff_iir_filter_init_state,
    },
    types::*,
};

use std::{
    alloc::{alloc, alloc_zeroed, Layout},
    ptr,
};

#[cold]
pub(crate) unsafe fn ff_psy_init(
    mut ctx: *mut FFPsyContext,
    mut avctx: *mut AVCodecContext,
    mut num_lens: libc::c_int,
    mut bands: *mut *const uint8_t,
    mut num_bands: *const libc::c_int,
    mut num_groups: libc::c_int,
    mut group_map: *const uint8_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0 as libc::c_int;
    (*ctx).avctx = avctx;
    (*ctx).ch = alloc_zeroed(
        Layout::array::<[FFPsyChannel; 2]>((*avctx).ch_layout.nb_channels as usize).unwrap(),
    )
    .cast();
    (*ctx).group =
        alloc_zeroed(Layout::array::<FFPsyChannelGroup>(num_groups as usize).unwrap()).cast();
    (*ctx).bands = alloc(Layout::array::<*mut uint8_t>(num_lens as usize).unwrap()).cast();
    (*ctx).num_bands = alloc(Layout::array::<libc::c_int>(num_lens as usize).unwrap()).cast();
    (*ctx).cutoff = (*avctx).cutoff;
    if ((*ctx).ch).is_null()
        || ((*ctx).group).is_null()
        || ((*ctx).bands).is_null()
        || ((*ctx).num_bands).is_null()
    {
        ff_psy_end(ctx);
        return -(12 as libc::c_int);
    }
    ptr::copy_nonoverlapping(bands, (*ctx).bands as *mut _, num_lens as usize);
    ptr::copy_nonoverlapping(num_bands, (*ctx).num_bands as *mut _, num_lens as usize);
    i = 0 as libc::c_int;
    while i < num_groups {
        (*((*ctx).group).offset(i as isize)).num_ch =
            (*group_map.offset(i as isize) as libc::c_int + 1 as libc::c_int) as uint8_t;
        j = 0 as libc::c_int;
        while j < (*((*ctx).group).offset(i as isize)).num_ch as libc::c_int * 2 as libc::c_int {
            let fresh0 = k;
            k += 1;
            let fresh1 = &mut (*((*ctx).group).offset(i as isize)).ch[j as usize];
            *fresh1 = &mut *((*ctx).ch).offset(fresh0 as isize) as *mut FFPsyChannel;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    match (*(*ctx).avctx).codec_id as libc::c_uint {
        86018 => {
            (*ctx).model = &ff_aac_psy_model;
        }
        _ => {}
    }
    if ((*(*ctx).model).init).is_some() {
        return ((*(*ctx).model).init).expect("non-null function pointer")(ctx);
    }
    0 as libc::c_int
}

pub(crate) unsafe fn ff_psy_find_group(
    mut ctx: *mut FFPsyContext,
    mut channel: libc::c_int,
) -> *mut FFPsyChannelGroup {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut ch: libc::c_int = 0 as libc::c_int;
    while ch <= channel {
        let fresh2 = i;
        i += 1;
        ch += (*((*ctx).group).offset(fresh2 as isize)).num_ch as libc::c_int;
    }
    &mut *((*ctx).group).offset((i - 1 as libc::c_int) as isize) as *mut FFPsyChannelGroup
}

#[cold]
pub(crate) unsafe fn ff_psy_end(mut ctx: *mut FFPsyContext) {
    if !((*ctx).model).is_null() && ((*(*ctx).model).end).is_some() {
        ((*(*ctx).model).end).expect("non-null function pointer")(ctx);
    }
    // TODO: leaks 🚿
    // av_freep(&mut (*ctx).bands as *mut *mut *mut uint8_t as *mut libc::c_void);
    // av_freep(&mut (*ctx).num_bands as *mut *mut libc::c_int as *mut libc::c_void);
    // av_freep(&mut (*ctx).group as *mut *mut FFPsyChannelGroup as *mut libc::c_void);
    // av_freep(&mut (*ctx).ch as *mut *mut FFPsyChannel as *mut libc::c_void);
}

#[cold]
pub(crate) unsafe fn ff_psy_preprocess_init(
    mut avctx: *mut AVCodecContext,
) -> *mut FFPsyPreprocessContext {
    let mut ctx: *mut FFPsyPreprocessContext = std::ptr::null_mut::<FFPsyPreprocessContext>();
    let mut i: libc::c_int = 0;
    let mut cutoff_coeff: libc::c_float = 0 as libc::c_int as libc::c_float;
    ctx = alloc_zeroed(Layout::new::<FFPsyPreprocessContext>()).cast();
    if ctx.is_null() {
        return std::ptr::null_mut::<FFPsyPreprocessContext>();
    }
    (*ctx).avctx = avctx;
    if (*avctx).codec_id as libc::c_uint != AV_CODEC_ID_AAC as libc::c_int as libc::c_uint {
        if (*avctx).cutoff > 0 as libc::c_int {
            cutoff_coeff = (2.0f64 * (*avctx).cutoff as libc::c_double
                / (*avctx).sample_rate as libc::c_double)
                as libc::c_float;
        }
        if cutoff_coeff != 0. && (cutoff_coeff as libc::c_double) < 0.98f64 {
            (*ctx).fcoeffs = ff_iir_filter_init_coeffs(
                avctx as *mut libc::c_void,
                FF_FILTER_TYPE_BUTTERWORTH,
                FF_FILTER_MODE_LOWPASS,
                4 as libc::c_int,
                cutoff_coeff,
                0.0f64 as libc::c_float,
                0.0f64 as libc::c_float,
            );
        }
        if !((*ctx).fcoeffs).is_null() {
            (*ctx).fstate = alloc_zeroed(
                Layout::array::<*mut FFIIRFilterState>((*avctx).ch_layout.nb_channels as usize)
                    .unwrap(),
            )
            .cast();
            if ((*ctx).fstate).is_null() {
                // TODO: leaks 🚿
                // av_free((*ctx).fcoeffs as *mut libc::c_void);
                // av_free(ctx as *mut libc::c_void);
                return std::ptr::null_mut::<FFPsyPreprocessContext>();
            }
            i = 0 as libc::c_int;
            while i < (*avctx).ch_layout.nb_channels {
                let fresh3 = &mut (*((*ctx).fstate).offset(i as isize));
                *fresh3 = ff_iir_filter_init_state(4 as libc::c_int);
                i += 1;
                i;
            }
        }
    }
    ff_iir_filter_init(&mut (*ctx).fiir);
    ctx
}

pub(crate) unsafe fn ff_psy_preprocess(
    mut ctx: *mut FFPsyPreprocessContext,
    mut audio: *mut *mut libc::c_float,
    mut channels: libc::c_int,
) {
    let mut ch: libc::c_int = 0;
    let mut frame_size: libc::c_int = (*(*ctx).avctx).frame_size;
    let mut iir: *mut FFIIRFilterContext = &mut (*ctx).fiir;
    if !((*ctx).fstate).is_null() {
        ch = 0 as libc::c_int;
        while ch < channels {
            ((*iir).filter_flt).expect("non-null function pointer")(
                (*ctx).fcoeffs,
                *((*ctx).fstate).offset(ch as isize),
                frame_size,
                &mut *(*audio.offset(ch as isize)).offset(frame_size as isize),
                1 as libc::c_int as ptrdiff_t,
                &mut *(*audio.offset(ch as isize)).offset(frame_size as isize),
                1 as libc::c_int as ptrdiff_t,
            );
            ch += 1;
            ch;
        }
    }
}

#[cold]
pub(crate) unsafe fn ff_psy_preprocess_end(mut ctx: *mut FFPsyPreprocessContext) {
    let mut i: libc::c_int = 0;
    ff_iir_filter_free_coeffsp(&mut (*ctx).fcoeffs);
    if !((*ctx).fstate).is_null() {
        i = 0 as libc::c_int;
        while i < (*(*ctx).avctx).ch_layout.nb_channels {
            ff_iir_filter_free_statep(&mut *((*ctx).fstate).offset(i as isize));
            i += 1;
            i;
        }
    }
    // TODO: leaks 🚿
    // av_freep(&mut (*ctx).fstate as *mut *mut *mut FFIIRFilterState as *mut libc::c_void);
    // av_free(ctx as *mut libc::c_void);
}
