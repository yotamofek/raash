use std::alloc::{alloc, alloc_zeroed, Layout};

use crate::{common::*, types::*};

#[cold]
unsafe fn butterworth_init_coeffs(
    _avc: *mut libc::c_void,
    c: *mut FFIIRFilterCoeffs,
    filt_mode: IIRFilterMode,
    order: libc::c_int,
    cutoff_ratio: libc::c_float,
    _stopband: libc::c_float,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut wa: libc::c_double = 0.;
    let mut p: [[libc::c_double; 2]; 31] = [[0.; 2]; 31];
    assert_eq!(
        filt_mode, FF_FILTER_MODE_LOWPASS,
        "Butterworth filter currently only supports low-pass filter mode"
    );
    assert_eq!(
        order & 1 as libc::c_int,
        0,
        "Butterworth filter currently only supports even filter orders"
    );
    wa = 2 as libc::c_int as libc::c_double
        * tan(3.141_592_653_589_793_f64 * 0.5f64 * cutoff_ratio as libc::c_double);
    *((*c).cx).offset(0 as libc::c_int as isize) = 1 as libc::c_int;
    i = 1 as libc::c_int;
    while i < (order >> 1 as libc::c_int) + 1 as libc::c_int {
        *((*c).cx).offset(i as isize) = (*((*c).cx).offset((i - 1 as libc::c_int) as isize)
            as libc::c_longlong
            * ((order - i) as libc::c_longlong + 1 as libc::c_longlong)
            / i as libc::c_longlong) as libc::c_int;
        i += 1;
        i;
    }
    p[0 as libc::c_int as usize][0 as libc::c_int as usize] = 1.0f64;
    p[0 as libc::c_int as usize][1 as libc::c_int as usize] = 0.0f64;
    i = 1 as libc::c_int;
    while i <= order {
        p[i as usize][1 as libc::c_int as usize] = 0.0f64;
        p[i as usize][0 as libc::c_int as usize] = p[i as usize][1 as libc::c_int as usize];
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < order {
        let mut zp: [libc::c_double; 2] = [0.; 2];
        let th: libc::c_double = ((i + (order >> 1 as libc::c_int)) as libc::c_double + 0.5f64)
            * 3.141_592_653_589_793_f64
            / order as libc::c_double;
        let mut a_re: libc::c_double = 0.;
        let mut a_im: libc::c_double = 0.;
        let mut c_re: libc::c_double = 0.;
        let mut c_im: libc::c_double = 0.;
        zp[0 as libc::c_int as usize] = cos(th) * wa;
        zp[1 as libc::c_int as usize] = sin(th) * wa;
        a_re = zp[0 as libc::c_int as usize] + 2.0f64;
        c_re = zp[0 as libc::c_int as usize] - 2.0f64;
        c_im = zp[1 as libc::c_int as usize];
        a_im = c_im;
        zp[0 as libc::c_int as usize] = (a_re * c_re + a_im * c_im) / (c_re * c_re + c_im * c_im);
        zp[1 as libc::c_int as usize] = (a_im * c_re - a_re * c_im) / (c_re * c_re + c_im * c_im);
        j = order;
        while j >= 1 as libc::c_int {
            a_re = p[j as usize][0 as libc::c_int as usize];
            a_im = p[j as usize][1 as libc::c_int as usize];
            p[j as usize][0 as libc::c_int as usize] = a_re * zp[0 as libc::c_int as usize]
                - a_im * zp[1 as libc::c_int as usize]
                + p[(j - 1 as libc::c_int) as usize][0 as libc::c_int as usize];
            p[j as usize][1 as libc::c_int as usize] = a_re * zp[1 as libc::c_int as usize]
                + a_im * zp[0 as libc::c_int as usize]
                + p[(j - 1 as libc::c_int) as usize][1 as libc::c_int as usize];
            j -= 1;
            j;
        }
        a_re = p[0 as libc::c_int as usize][0 as libc::c_int as usize]
            * zp[0 as libc::c_int as usize]
            - p[0 as libc::c_int as usize][1 as libc::c_int as usize]
                * zp[1 as libc::c_int as usize];
        p[0 as libc::c_int as usize][1 as libc::c_int as usize] =
            p[0 as libc::c_int as usize][0 as libc::c_int as usize] * zp[1 as libc::c_int as usize]
                + p[0 as libc::c_int as usize][1 as libc::c_int as usize]
                    * zp[0 as libc::c_int as usize];
        p[0 as libc::c_int as usize][0 as libc::c_int as usize] = a_re;
        i += 1;
        i;
    }
    (*c).gain = p[order as usize][0 as libc::c_int as usize] as libc::c_float;
    i = 0 as libc::c_int;
    while i < order {
        (*c).gain = ((*c).gain as libc::c_double + p[i as usize][0 as libc::c_int as usize])
            as libc::c_float;
        *((*c).cy).offset(i as isize) = ((-p[i as usize][0 as libc::c_int as usize]
            * p[order as usize][0 as libc::c_int as usize]
            + -p[i as usize][1 as libc::c_int as usize]
                * p[order as usize][1 as libc::c_int as usize])
            / (p[order as usize][0 as libc::c_int as usize]
                * p[order as usize][0 as libc::c_int as usize]
                + p[order as usize][1 as libc::c_int as usize]
                    * p[order as usize][1 as libc::c_int as usize]))
            as libc::c_float;
        i += 1;
        i;
    }
    (*c).gain /= ((1 as libc::c_int) << order) as libc::c_float;
    0 as libc::c_int
}
#[cold]
unsafe fn biquad_init_coeffs(
    _avc: *mut libc::c_void,
    c: *mut FFIIRFilterCoeffs,
    filt_mode: IIRFilterMode,
    order: libc::c_int,
    cutoff_ratio: libc::c_float,
    _stopband: libc::c_float,
) -> libc::c_int {
    let mut cos_w0: libc::c_double = 0.;
    let mut sin_w0: libc::c_double = 0.;
    let mut a0: libc::c_double = 0.;
    let mut x0: libc::c_double = 0.;
    let mut x1: libc::c_double = 0.;
    assert!(
        [FF_FILTER_MODE_HIGHPASS, FF_FILTER_MODE_LOWPASS].contains(&filt_mode),
        "Biquad filter currently only supports high-pass and low-pass filter modes"
    );
    assert_eq!(order, 2, "Biquad filter must have order of 2");
    cos_w0 = cos(3.141_592_653_589_793_f64 * cutoff_ratio as libc::c_double);
    sin_w0 = sin(3.141_592_653_589_793_f64 * cutoff_ratio as libc::c_double);
    a0 = 1.0f64 + sin_w0 / 2.0f64;
    if filt_mode as libc::c_uint == FF_FILTER_MODE_HIGHPASS as libc::c_int as libc::c_uint {
        (*c).gain = ((1.0f64 + cos_w0) / 2.0f64 / a0) as libc::c_float;
        x0 = (1.0f64 + cos_w0) / 2.0f64 / a0;
        x1 = -(1.0f64 + cos_w0) / a0;
    } else {
        (*c).gain = ((1.0f64 - cos_w0) / 2.0f64 / a0) as libc::c_float;
        x0 = (1.0f64 - cos_w0) / 2.0f64 / a0;
        x1 = (1.0f64 - cos_w0) / a0;
    }
    *((*c).cy).offset(0 as libc::c_int as isize) =
        ((-1.0f64 + sin_w0 / 2.0f64) / a0) as libc::c_float;
    *((*c).cy).offset(1 as libc::c_int as isize) = (2.0f64 * cos_w0 / a0) as libc::c_float;
    *((*c).cx).offset(0 as libc::c_int as isize) =
        lrintf((x0 / (*c).gain as libc::c_double) as libc::c_float) as libc::c_int;
    *((*c).cx).offset(1 as libc::c_int as isize) =
        lrintf((x1 / (*c).gain as libc::c_double) as libc::c_float) as libc::c_int;
    0 as libc::c_int
}

#[cold]
pub(crate) unsafe fn ff_iir_filter_init_coeffs(
    avc: *mut libc::c_void,
    filt_type: IIRFilterType,
    filt_mode: IIRFilterMode,
    order: libc::c_int,
    cutoff_ratio: libc::c_float,
    stopband: libc::c_float,
    _ripple: libc::c_float,
) -> *mut FFIIRFilterCoeffs {
    let current_block: u64;
    let mut c: *mut FFIIRFilterCoeffs = std::ptr::null_mut::<FFIIRFilterCoeffs>();
    let mut ret: libc::c_int = 0 as libc::c_int;
    if order <= 0 as libc::c_int
        || order > 30 as libc::c_int
        || cutoff_ratio as libc::c_double >= 1.0f64
    {
        return std::ptr::null_mut::<FFIIRFilterCoeffs>();
    }
    c = alloc_zeroed(Layout::new::<FFIIRFilterCoeffs>()).cast();
    if !(c.is_null()
        || {
            (*c).cx = alloc(
                Layout::array::<libc::c_int>(
                    ((order >> 1 as libc::c_int) + 1 as libc::c_int) as usize,
                )
                .unwrap(),
            )
            .cast();
            ((*c).cx).is_null()
        }
        || {
            (*c).cy = alloc(Layout::array::<libc::c_float>(order as usize).unwrap()).cast();
            ((*c).cy).is_null()
        })
    {
        (*c).order = order;
        match filt_type as libc::c_uint {
            2 => {
                ret = butterworth_init_coeffs(avc, c, filt_mode, order, cutoff_ratio, stopband);
                current_block = 13513818773234778473;
            }
            1 => {
                ret = biquad_init_coeffs(avc, c, filt_mode, order, cutoff_ratio, stopband);
                current_block = 13513818773234778473;
            }
            _ => {
                panic!("filter type is not currently implemented");
                current_block = 9061800508960952076;
            }
        }
        match current_block {
            9061800508960952076 => {}
            _ => {
                if ret == 0 {
                    return c;
                }
            }
        }
    }
    ff_iir_filter_free_coeffsp(&mut c);
    std::ptr::null_mut::<FFIIRFilterCoeffs>()
}

#[cold]
pub(crate) unsafe fn ff_iir_filter_init_state(order: libc::c_int) -> *mut FFIIRFilterState {
    // TODO: is this correct?
    let s: *mut FFIIRFilterState =
        alloc_zeroed(Layout::array::<libc::c_float>(order as usize).unwrap()).cast();
    // let mut s: *mut FFIIRFilterState = av_mallocz(
    //     (::core::mem::size_of::<FFIIRFilterState>() as libc::c_ulong).wrapping_add(
    //         (::core::mem::size_of::<libc::c_float>() as libc::c_ulong)
    //             .wrapping_mul((order - 1 as libc::c_int) as libc::c_ulong),
    //     ),
    // ) as *mut FFIIRFilterState;
    s
}

unsafe fn iir_filter_flt(
    c: *const FFIIRFilterCoeffs,
    s: *mut FFIIRFilterState,
    size: libc::c_int,
    src: *const libc::c_float,
    sstep: ptrdiff_t,
    dst: *mut libc::c_float,
    dstep: ptrdiff_t,
) {
    if (*c).order == 2 as libc::c_int {
        let mut i: libc::c_int = 0;
        let mut src0: *const libc::c_float = src;
        let mut dst0: *mut libc::c_float = dst;
        i = 0 as libc::c_int;
        while i < size {
            let in_0: libc::c_float = *src0 * (*c).gain
                + *((*s).x).as_mut_ptr().offset(0 as libc::c_int as isize)
                    * *((*c).cy).offset(0 as libc::c_int as isize)
                + *((*s).x).as_mut_ptr().offset(1 as libc::c_int as isize)
                    * *((*c).cy).offset(1 as libc::c_int as isize);
            *dst0 = *((*s).x).as_mut_ptr().offset(0 as libc::c_int as isize)
                + in_0
                + *((*s).x).as_mut_ptr().offset(1 as libc::c_int as isize)
                    * *((*c).cx).offset(1 as libc::c_int as isize) as libc::c_float;
            *((*s).x).as_mut_ptr().offset(0 as libc::c_int as isize) =
                *((*s).x).as_mut_ptr().offset(1 as libc::c_int as isize);
            *((*s).x).as_mut_ptr().offset(1 as libc::c_int as isize) = in_0;
            src0 = src0.offset(sstep as isize);
            dst0 = dst0.offset(dstep as isize);
            i += 1;
            i;
        }
    } else if (*c).order == 4 as libc::c_int {
        let mut i_0: libc::c_int = 0;
        let mut src0_0: *const libc::c_float = src;
        let mut dst0_0: *mut libc::c_float = dst;
        i_0 = 0 as libc::c_int;
        while i_0 < size {
            let mut in_1: libc::c_float = 0.;
            let mut res: libc::c_float = 0.;
            in_1 = *src0_0 * (*c).gain
                + *((*c).cy).offset(0 as libc::c_int as isize)
                    * *((*s).x).as_mut_ptr().offset(0 as libc::c_int as isize)
                + *((*c).cy).offset(1 as libc::c_int as isize)
                    * *((*s).x).as_mut_ptr().offset(1 as libc::c_int as isize)
                + *((*c).cy).offset(2 as libc::c_int as isize)
                    * *((*s).x).as_mut_ptr().offset(2 as libc::c_int as isize)
                + *((*c).cy).offset(3 as libc::c_int as isize)
                    * *((*s).x).as_mut_ptr().offset(3 as libc::c_int as isize);
            res = (*((*s).x).as_mut_ptr().offset(0 as libc::c_int as isize) + in_1)
                * 1 as libc::c_int as libc::c_float
                + (*((*s).x).as_mut_ptr().offset(1 as libc::c_int as isize)
                    + *((*s).x).as_mut_ptr().offset(3 as libc::c_int as isize))
                    * 4 as libc::c_int as libc::c_float
                + *((*s).x).as_mut_ptr().offset(2 as libc::c_int as isize)
                    * 6 as libc::c_int as libc::c_float;
            *dst0_0 = res;
            *((*s).x).as_mut_ptr().offset(0 as libc::c_int as isize) = in_1;
            src0_0 = src0_0.offset(sstep as isize);
            dst0_0 = dst0_0.offset(dstep as isize);
            in_1 = *src0_0 * (*c).gain
                + *((*c).cy).offset(0 as libc::c_int as isize)
                    * *((*s).x).as_mut_ptr().offset(1 as libc::c_int as isize)
                + *((*c).cy).offset(1 as libc::c_int as isize)
                    * *((*s).x).as_mut_ptr().offset(2 as libc::c_int as isize)
                + *((*c).cy).offset(2 as libc::c_int as isize)
                    * *((*s).x).as_mut_ptr().offset(3 as libc::c_int as isize)
                + *((*c).cy).offset(3 as libc::c_int as isize)
                    * *((*s).x).as_mut_ptr().offset(0 as libc::c_int as isize);
            res = (*((*s).x).as_mut_ptr().offset(1 as libc::c_int as isize) + in_1)
                * 1 as libc::c_int as libc::c_float
                + (*((*s).x).as_mut_ptr().offset(2 as libc::c_int as isize)
                    + *((*s).x).as_mut_ptr().offset(0 as libc::c_int as isize))
                    * 4 as libc::c_int as libc::c_float
                + *((*s).x).as_mut_ptr().offset(3 as libc::c_int as isize)
                    * 6 as libc::c_int as libc::c_float;
            *dst0_0 = res;
            *((*s).x).as_mut_ptr().offset(1 as libc::c_int as isize) = in_1;
            src0_0 = src0_0.offset(sstep as isize);
            dst0_0 = dst0_0.offset(dstep as isize);
            in_1 = *src0_0 * (*c).gain
                + *((*c).cy).offset(0 as libc::c_int as isize)
                    * *((*s).x).as_mut_ptr().offset(2 as libc::c_int as isize)
                + *((*c).cy).offset(1 as libc::c_int as isize)
                    * *((*s).x).as_mut_ptr().offset(3 as libc::c_int as isize)
                + *((*c).cy).offset(2 as libc::c_int as isize)
                    * *((*s).x).as_mut_ptr().offset(0 as libc::c_int as isize)
                + *((*c).cy).offset(3 as libc::c_int as isize)
                    * *((*s).x).as_mut_ptr().offset(1 as libc::c_int as isize);
            res = (*((*s).x).as_mut_ptr().offset(2 as libc::c_int as isize) + in_1)
                * 1 as libc::c_int as libc::c_float
                + (*((*s).x).as_mut_ptr().offset(3 as libc::c_int as isize)
                    + *((*s).x).as_mut_ptr().offset(1 as libc::c_int as isize))
                    * 4 as libc::c_int as libc::c_float
                + *((*s).x).as_mut_ptr().offset(0 as libc::c_int as isize)
                    * 6 as libc::c_int as libc::c_float;
            *dst0_0 = res;
            *((*s).x).as_mut_ptr().offset(2 as libc::c_int as isize) = in_1;
            src0_0 = src0_0.offset(sstep as isize);
            dst0_0 = dst0_0.offset(dstep as isize);
            in_1 = *src0_0 * (*c).gain
                + *((*c).cy).offset(0 as libc::c_int as isize)
                    * *((*s).x).as_mut_ptr().offset(3 as libc::c_int as isize)
                + *((*c).cy).offset(1 as libc::c_int as isize)
                    * *((*s).x).as_mut_ptr().offset(0 as libc::c_int as isize)
                + *((*c).cy).offset(2 as libc::c_int as isize)
                    * *((*s).x).as_mut_ptr().offset(1 as libc::c_int as isize)
                + *((*c).cy).offset(3 as libc::c_int as isize)
                    * *((*s).x).as_mut_ptr().offset(2 as libc::c_int as isize);
            res = (*((*s).x).as_mut_ptr().offset(3 as libc::c_int as isize) + in_1)
                * 1 as libc::c_int as libc::c_float
                + (*((*s).x).as_mut_ptr().offset(0 as libc::c_int as isize)
                    + *((*s).x).as_mut_ptr().offset(2 as libc::c_int as isize))
                    * 4 as libc::c_int as libc::c_float
                + *((*s).x).as_mut_ptr().offset(1 as libc::c_int as isize)
                    * 6 as libc::c_int as libc::c_float;
            *dst0_0 = res;
            *((*s).x).as_mut_ptr().offset(3 as libc::c_int as isize) = in_1;
            src0_0 = src0_0.offset(sstep as isize);
            dst0_0 = dst0_0.offset(dstep as isize);
            i_0 += 4 as libc::c_int;
        }
    } else {
        let mut i_1: libc::c_int = 0;
        let mut src0_1: *const libc::c_float = src;
        let mut dst0_1: *mut libc::c_float = dst;
        i_1 = 0 as libc::c_int;
        while i_1 < size {
            let mut j: libc::c_int = 0;
            let mut in_2: libc::c_float = 0.;
            let mut res_0: libc::c_float = 0.;
            in_2 = *src0_1 * (*c).gain;
            j = 0 as libc::c_int;
            while j < (*c).order {
                in_2 += *((*c).cy).offset(j as isize) * *((*s).x).as_mut_ptr().offset(j as isize);
                j += 1;
                j;
            }
            res_0 = *((*s).x).as_mut_ptr().offset(0 as libc::c_int as isize)
                + in_2
                + *((*s).x)
                    .as_mut_ptr()
                    .offset(((*c).order >> 1 as libc::c_int) as isize)
                    * *((*c).cx).offset(((*c).order >> 1 as libc::c_int) as isize) as libc::c_float;
            j = 1 as libc::c_int;
            while j < (*c).order >> 1 as libc::c_int {
                res_0 += (*((*s).x).as_mut_ptr().offset(j as isize)
                    + *((*s).x).as_mut_ptr().offset(((*c).order - j) as isize))
                    * *((*c).cx).offset(j as isize) as libc::c_float;
                j += 1;
                j;
            }
            j = 0 as libc::c_int;
            while j < (*c).order - 1 as libc::c_int {
                *((*s).x).as_mut_ptr().offset(j as isize) = *((*s).x)
                    .as_mut_ptr()
                    .offset((j + 1 as libc::c_int) as isize);
                j += 1;
                j;
            }
            *dst0_1 = res_0;
            *((*s).x)
                .as_mut_ptr()
                .offset(((*c).order - 1 as libc::c_int) as isize) = in_2;
            src0_1 = src0_1.offset(sstep as isize);
            dst0_1 = dst0_1.offset(dstep as isize);
            i_1 += 1;
            i_1;
        }
    };
}

#[cold]
pub(crate) unsafe fn ff_iir_filter_free_statep(_state: *mut *mut FFIIRFilterState) {
    // TODO: leaks 🚿

    // av_freep(state as *mut libc::c_void);
}

#[cold]
pub(crate) unsafe fn ff_iir_filter_free_coeffsp(coeffsp: *mut *mut FFIIRFilterCoeffs) {
    let coeffs: *mut FFIIRFilterCoeffs = *coeffsp;
    // TODO: leaks 🚿
    if !coeffs.is_null() {
        // av_freep(&mut (*coeffs).cx as *mut *mut libc::c_int as *mut libc::c_void);
        // av_freep(&mut (*coeffs).cy as *mut *mut libc::c_float as *mut libc::c_void);
    }
    // av_freep(coeffsp as *mut libc::c_void);
}

pub(crate) unsafe fn ff_iir_filter_init(f: *mut FFIIRFilterContext) {
    (*f).filter_flt = Some(
        iir_filter_flt
            as unsafe fn(
                *const FFIIRFilterCoeffs,
                *mut FFIIRFilterState,
                libc::c_int,
                *const libc::c_float,
                ptrdiff_t,
                *mut libc::c_float,
                ptrdiff_t,
            ) -> (),
    );
}
