#![allow(clippy::self_assignment)]

use std::{
    alloc::{alloc_zeroed, Layout},
    ptr,
};

use libc::{
    c_char, c_double, c_float, c_int, c_long, c_longlong, c_uchar, c_uint, c_ulong, c_ulonglong,
    c_void,
};

use crate::{common::*, types::*};

pub(crate) type LPC_TYPE = c_double;
#[inline]
unsafe fn compute_ref_coefs(
    autoc: *const LPC_TYPE,
    max_order: c_int,
    ref_0: *mut LPC_TYPE,
    error: *mut LPC_TYPE,
) {
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut err: LPC_TYPE = 0.;
    let mut gen0: [LPC_TYPE; 32] = [0.; 32];
    let mut gen1: [LPC_TYPE; 32] = [0.; 32];
    i = 0 as c_int;
    while i < max_order {
        gen1[i as usize] = *autoc.offset((i + 1 as c_int) as isize);
        gen0[i as usize] = gen1[i as usize];
        i += 1;
        i;
    }
    err = *autoc.offset(0 as c_int as isize);
    *ref_0.offset(0 as c_int as isize) = -gen1[0 as c_int as usize]
        / (if 0 as c_int != 0 || err != 0. {
            err
        } else {
            1 as c_int as c_double
        });
    err += gen1[0 as c_int as usize] * *ref_0.offset(0 as c_int as isize);
    if !error.is_null() {
        *error.offset(0 as c_int as isize) = err;
    }
    i = 1 as c_int;
    while i < max_order {
        j = 0 as c_int;
        while j < max_order - i {
            gen1[j as usize] = gen1[(j + 1 as c_int) as usize]
                + *ref_0.offset((i - 1 as c_int) as isize) * gen0[j as usize];
            gen0[j as usize] +=
                gen1[(j + 1 as c_int) as usize] * *ref_0.offset((i - 1 as c_int) as isize);
            j += 1;
            j;
        }
        *ref_0.offset(i as isize) = -gen1[0 as c_int as usize]
            / (if 0 as c_int != 0 || err != 0. {
                err
            } else {
                1 as c_int as c_double
            });
        err += gen1[0 as c_int as usize] * *ref_0.offset(i as isize);
        if !error.is_null() {
            *error.offset(i as isize) = err;
        }
        i += 1;
        i;
    }
}

unsafe extern "C" fn lpc_apply_welch_window_c(
    mut data: *const c_int,
    len: ptrdiff_t,
    mut w_data: *mut c_double,
) {
    let mut i: c_int = 0;
    let mut n2: c_int = 0;
    let mut w: c_double = 0.;
    let mut c: c_double = 0.;
    if len == 1 as c_int as c_long {
        *w_data.offset(0 as c_int as isize) = 0.0f64;
        return;
    }
    n2 = (len >> 1 as c_int) as c_int;
    c = 2.0f64 / (len as c_double - 1.0f64);
    if len & 1 as c_int as c_long != 0 {
        i = 0 as c_int;
        while i < n2 {
            w = c - i as c_double - 1.0f64;
            w = 1.0f64 - w * w;
            *w_data.offset(i as isize) = *data.offset(i as isize) as c_double * w;
            *w_data.offset((len - 1 as c_int as c_long - i as c_long) as isize) =
                *data.offset((len - 1 as c_int as c_long - i as c_long) as isize) as c_double * w;
            i += 1;
            i;
        }
        *w_data.offset(n2 as isize) = 0.0f64;
        return;
    }
    w_data = w_data.offset(n2 as isize);
    data = data.offset(n2 as isize);
    i = 0 as c_int;
    while i < n2 {
        w = c - n2 as c_double + i as c_double;
        w = 1.0f64 - w * w;
        *w_data.offset((-i - 1 as c_int) as isize) =
            *data.offset((-i - 1 as c_int) as isize) as c_double * w;
        *w_data.offset(i as isize) = *data.offset(i as isize) as c_double * w;
        i += 1;
        i;
    }
}
unsafe extern "C" fn lpc_compute_autocorr_c(
    data: *const c_double,
    len: ptrdiff_t,
    lag: c_int,
    autoc: *mut c_double,
) {
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    j = 0 as c_int;
    while j < lag {
        let mut sum0: c_double = 1.0f64;
        let mut sum1: c_double = 1.0f64;
        i = j;
        while (i as c_long) < len {
            sum0 += *data.offset(i as isize) * *data.offset((i - j) as isize);
            sum1 += *data.offset(i as isize) * *data.offset((i - j - 1 as c_int) as isize);
            i += 1;
            i;
        }
        *autoc.offset(j as isize) = sum0;
        *autoc.offset((j + 1 as c_int) as isize) = sum1;
        j += 2 as c_int;
    }
    if j == lag {
        let mut sum: c_double = 1.0f64;
        i = j - 1 as c_int;
        while (i as c_long) < len {
            sum += *data.offset(i as isize) * *data.offset((i - j) as isize)
                + *data.offset((i + 1 as c_int) as isize)
                    * *data.offset((i - j + 1 as c_int) as isize);
            i += 2 as c_int;
        }
        *autoc.offset(j as isize) = sum;
    }
}

pub(crate) unsafe fn ff_lpc_calc_ref_coefs_f(
    s: *mut LPCContext,
    samples: *const c_float,
    len: c_int,
    order: c_int,
    ref_0: *mut c_double,
) -> c_double {
    let mut i: c_int = 0;
    let mut signal: c_double = 0.0f32 as c_double;
    let mut avg_err: c_double = 0.0f32 as c_double;
    let mut autoc: [c_double; 33] = [
        0 as c_int as c_double,
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
    let mut error: [c_double; 33] = [
        0 as c_int as c_double,
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
    let a: c_double = 0.5f32 as c_double;
    let b: c_double = 1.0f32 as c_double - a;
    i = 0 as c_int;
    while i <= len / 2 as c_int {
        let weight: c_double = a - b * cos(2 as c_int as c_double
            * 3.141_592_653_589_793_f64
            * i as c_double
            / (len - 1 as c_int) as c_double);
        *((*s).windowed_samples).offset(i as isize) =
            weight * *samples.offset(i as isize) as c_double;
        *((*s).windowed_samples).offset((len - 1 as c_int - i) as isize) =
            weight * *samples.offset((len - 1 as c_int - i) as isize) as c_double;
        i += 1;
        i;
    }
    ((*s).lpc_compute_autocorr).expect("non-null function pointer")(
        (*s).windowed_samples,
        len as ptrdiff_t,
        order,
        autoc.as_mut_ptr(),
    );
    signal = autoc[0 as c_int as usize];
    compute_ref_coefs(autoc.as_mut_ptr(), order, ref_0, error.as_mut_ptr());
    i = 0 as c_int;
    while i < order {
        avg_err = (avg_err + error[i as usize]) / 2.0f32 as c_double;
        i += 1;
        i;
    }
    if avg_err != 0. {
        signal / avg_err
    } else {
        ::core::f32::NAN as c_double
    }
}

#[cold]
pub(crate) unsafe fn ff_lpc_init(
    s: *mut LPCContext,
    blocksize: c_int,
    max_order: c_int,
    lpc_type: FFLPCType,
) -> c_int {
    (*s).blocksize = blocksize;
    (*s).max_order = max_order;
    (*s).lpc_type = lpc_type;
    (*s).windowed_buffer = alloc_zeroed(
        Layout::array::<c_double>(
            (blocksize
                + 2 as c_int
                + (max_order + 4 as c_int - 1 as c_int & !(4 as c_int - 1 as c_int)))
                as usize,
        )
        .unwrap(),
    )
    .cast();
    if ((*s).windowed_buffer).is_null() {
        return -(12 as c_int);
    }
    (*s).windowed_samples = ((*s).windowed_buffer)
        .offset((max_order + 4 as c_int - 1 as c_int & !(4 as c_int - 1 as c_int)) as isize);
    (*s).lpc_apply_welch_window = Some(lpc_apply_welch_window_c);
    (*s).lpc_compute_autocorr = Some(lpc_compute_autocorr_c);
    0 as c_int
}

#[cold]
pub(crate) unsafe fn ff_lpc_end(_s: *mut LPCContext) {
    // TODO: this leaks 🚿
    // av_freep(&mut (*s).windowed_buffer as *mut *mut c_double as *mut c_void);
}
