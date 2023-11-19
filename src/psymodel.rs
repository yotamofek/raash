#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use crate::{common::*, types::*};

use std::ptr;
extern "C" {
    pub type FFIIRFilterState;
    pub type FFIIRFilterCoeffs;
    fn av_mallocz(size: size_t) -> *mut libc::c_void;
    fn av_malloc_array(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn av_calloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn av_free(ptr: *mut libc::c_void);
    fn av_freep(ptr: *mut libc::c_void);
    fn ff_iir_filter_init(f: *mut FFIIRFilterContext);
    fn ff_iir_filter_init_coeffs(
        avc: *mut libc::c_void,
        filt_type: IIRFilterType,
        filt_mode: IIRFilterMode,
        order: libc::c_int,
        cutoff_ratio: libc::c_float,
        stopband: libc::c_float,
        ripple: libc::c_float,
    ) -> *mut FFIIRFilterCoeffs;
    fn ff_iir_filter_init_state(order: libc::c_int) -> *mut FFIIRFilterState;
    fn ff_iir_filter_free_coeffsp(coeffs: *mut *mut FFIIRFilterCoeffs);
    fn ff_iir_filter_free_statep(state: *mut *mut FFIIRFilterState);
    static ff_aac_psy_model: FFPsyModel;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFPsyWindowInfo {
    pub window_type: [libc::c_int; 3],
    pub window_shape: libc::c_int,
    pub num_windows: libc::c_int,
    pub grouping: [libc::c_int; 8],
    pub clipping: [libc::c_float; 8],
    pub window_sizes: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFPsyContext {
    pub avctx: *mut AVCodecContext,
    pub model: *const FFPsyModel,
    pub ch: *mut FFPsyChannel,
    pub group: *mut FFPsyChannelGroup,
    pub num_groups: libc::c_int,
    pub cutoff: libc::c_int,
    pub bands: *mut *mut uint8_t,
    pub num_bands: *mut libc::c_int,
    pub num_lens: libc::c_int,
    pub bitres: C2RustUnnamed_0,
    pub model_priv_data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub size: libc::c_int,
    pub bits: libc::c_int,
    pub alloc: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFPsyModel {
    pub name: *const libc::c_char,
    pub init: Option<unsafe extern "C" fn(*mut FFPsyContext) -> libc::c_int>,
    pub window: Option<
        unsafe extern "C" fn(
            *mut FFPsyContext,
            *const libc::c_float,
            *const libc::c_float,
            libc::c_int,
            libc::c_int,
        ) -> FFPsyWindowInfo,
    >,
    pub analyze: Option<
        unsafe extern "C" fn(
            *mut FFPsyContext,
            libc::c_int,
            *mut *const libc::c_float,
            *const FFPsyWindowInfo,
        ) -> (),
    >,
    pub end: Option<unsafe extern "C" fn(*mut FFPsyContext) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFPsyPreprocessContext {
    pub avctx: *mut AVCodecContext,
    pub stereo_att: libc::c_float,
    pub fcoeffs: *mut FFIIRFilterCoeffs,
    pub fstate: *mut *mut FFIIRFilterState,
    pub fiir: FFIIRFilterContext,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFIIRFilterContext {
    pub filter_flt: Option<
        unsafe extern "C" fn(
            *const FFIIRFilterCoeffs,
            *mut FFIIRFilterState,
            libc::c_int,
            *const libc::c_float,
            ptrdiff_t,
            *mut libc::c_float,
            ptrdiff_t,
        ) -> (),
    >,
}
pub type IIRFilterMode = libc::c_uint;
pub const FF_FILTER_MODE_BANDSTOP: IIRFilterMode = 3;
pub const FF_FILTER_MODE_BANDPASS: IIRFilterMode = 2;
pub const FF_FILTER_MODE_HIGHPASS: IIRFilterMode = 1;
pub const FF_FILTER_MODE_LOWPASS: IIRFilterMode = 0;
pub type IIRFilterType = libc::c_uint;
pub const FF_FILTER_TYPE_ELLIPTIC: IIRFilterType = 4;
pub const FF_FILTER_TYPE_CHEBYSHEV: IIRFilterType = 3;
pub const FF_FILTER_TYPE_BUTTERWORTH: IIRFilterType = 2;
pub const FF_FILTER_TYPE_BIQUAD: IIRFilterType = 1;
pub const FF_FILTER_TYPE_BESSEL: IIRFilterType = 0;
#[no_mangle]
#[cold]
pub unsafe extern "C" fn ff_psy_init(
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
    (*ctx).ch = av_calloc(
        (*avctx).ch_layout.nb_channels as size_t,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<FFPsyChannel>() as libc::c_ulong),
    ) as *mut FFPsyChannel;
    (*ctx).group = av_calloc(
        num_groups as size_t,
        ::core::mem::size_of::<FFPsyChannelGroup>() as libc::c_ulong,
    ) as *mut FFPsyChannelGroup;
    (*ctx).bands = av_malloc_array(
        ::core::mem::size_of::<*mut uint8_t>() as libc::c_ulong,
        num_lens as size_t,
    ) as *mut *mut uint8_t;
    (*ctx).num_bands = av_malloc_array(
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        num_lens as size_t,
    ) as *mut libc::c_int;
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
            k = k + 1;
            let ref mut fresh1 = (*((*ctx).group).offset(i as isize)).ch[j as usize];
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
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_psy_find_group(
    mut ctx: *mut FFPsyContext,
    mut channel: libc::c_int,
) -> *mut FFPsyChannelGroup {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut ch: libc::c_int = 0 as libc::c_int;
    while ch <= channel {
        let fresh2 = i;
        i = i + 1;
        ch += (*((*ctx).group).offset(fresh2 as isize)).num_ch as libc::c_int;
    }
    return &mut *((*ctx).group).offset((i - 1 as libc::c_int) as isize) as *mut FFPsyChannelGroup;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn ff_psy_end(mut ctx: *mut FFPsyContext) {
    if !((*ctx).model).is_null() && ((*(*ctx).model).end).is_some() {
        ((*(*ctx).model).end).expect("non-null function pointer")(ctx);
    }
    av_freep(&mut (*ctx).bands as *mut *mut *mut uint8_t as *mut libc::c_void);
    av_freep(&mut (*ctx).num_bands as *mut *mut libc::c_int as *mut libc::c_void);
    av_freep(&mut (*ctx).group as *mut *mut FFPsyChannelGroup as *mut libc::c_void);
    av_freep(&mut (*ctx).ch as *mut *mut FFPsyChannel as *mut libc::c_void);
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn ff_psy_preprocess_init(
    mut avctx: *mut AVCodecContext,
) -> *mut FFPsyPreprocessContext {
    let mut ctx: *mut FFPsyPreprocessContext = 0 as *mut FFPsyPreprocessContext;
    let mut i: libc::c_int = 0;
    let mut cutoff_coeff: libc::c_float = 0 as libc::c_int as libc::c_float;
    ctx = av_mallocz(::core::mem::size_of::<FFPsyPreprocessContext>() as libc::c_ulong)
        as *mut FFPsyPreprocessContext;
    if ctx.is_null() {
        return 0 as *mut FFPsyPreprocessContext;
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
            (*ctx).fstate = av_calloc(
                (*avctx).ch_layout.nb_channels as size_t,
                ::core::mem::size_of::<*mut FFIIRFilterState>() as libc::c_ulong,
            ) as *mut *mut FFIIRFilterState;
            if ((*ctx).fstate).is_null() {
                av_free((*ctx).fcoeffs as *mut libc::c_void);
                av_free(ctx as *mut libc::c_void);
                return 0 as *mut FFPsyPreprocessContext;
            }
            i = 0 as libc::c_int;
            while i < (*avctx).ch_layout.nb_channels {
                let ref mut fresh3 = *((*ctx).fstate).offset(i as isize);
                *fresh3 = ff_iir_filter_init_state(4 as libc::c_int);
                i += 1;
                i;
            }
        }
    }
    ff_iir_filter_init(&mut (*ctx).fiir);
    return ctx;
}
#[no_mangle]
pub unsafe extern "C" fn ff_psy_preprocess(
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
#[no_mangle]
#[cold]
pub unsafe extern "C" fn ff_psy_preprocess_end(mut ctx: *mut FFPsyPreprocessContext) {
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
    av_freep(&mut (*ctx).fstate as *mut *mut *mut FFIIRFilterState as *mut libc::c_void);
    av_free(ctx as *mut libc::c_void);
}
