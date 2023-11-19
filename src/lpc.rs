#![allow(clippy::self_assignment)]

use std::alloc::alloc_zeroed;
use std::alloc::Layout;
use std::ptr;

use crate::common::*;
use crate::types::*;

pub(crate) type LPC_TYPE = libc::c_double;
#[inline]
unsafe fn compute_ref_coefs(
    autoc: *const LPC_TYPE,
    max_order: libc::c_int,
    ref_0: *mut LPC_TYPE,
    error: *mut LPC_TYPE,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut err: LPC_TYPE = 0.;
    let mut gen0: [LPC_TYPE; 32] = [0.; 32];
    let mut gen1: [LPC_TYPE; 32] = [0.; 32];
    i = 0 as libc::c_int;
    while i < max_order {
        gen1[i as usize] = *autoc.offset((i + 1 as libc::c_int) as isize);
        gen0[i as usize] = gen1[i as usize];
        i += 1;
        i;
    }
    err = *autoc.offset(0 as libc::c_int as isize);
    *ref_0.offset(0 as libc::c_int as isize) = -gen1[0 as libc::c_int as usize]
        / (if 0 as libc::c_int != 0 || err != 0. {
            err
        } else {
            1 as libc::c_int as libc::c_double
        });
    err += gen1[0 as libc::c_int as usize] * *ref_0.offset(0 as libc::c_int as isize);
    if !error.is_null() {
        *error.offset(0 as libc::c_int as isize) = err;
    }
    i = 1 as libc::c_int;
    while i < max_order {
        j = 0 as libc::c_int;
        while j < max_order - i {
            gen1[j as usize] = gen1[(j + 1 as libc::c_int) as usize]
                + *ref_0.offset((i - 1 as libc::c_int) as isize) * gen0[j as usize];
            gen0[j as usize] += gen1[(j + 1 as libc::c_int) as usize]
                * *ref_0.offset((i - 1 as libc::c_int) as isize);
            j += 1;
            j;
        }
        *ref_0.offset(i as isize) = -gen1[0 as libc::c_int as usize]
            / (if 0 as libc::c_int != 0 || err != 0. {
                err
            } else {
                1 as libc::c_int as libc::c_double
            });
        err += gen1[0 as libc::c_int as usize] * *ref_0.offset(i as isize);
        if !error.is_null() {
            *error.offset(i as isize) = err;
        }
        i += 1;
        i;
    }
}

unsafe extern "C" fn lpc_apply_welch_window_c(
    mut data: *const int32_t,
    len: ptrdiff_t,
    mut w_data: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut n2: libc::c_int = 0;
    let mut w: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    if len == 1 as libc::c_int as libc::c_long {
        *w_data.offset(0 as libc::c_int as isize) = 0.0f64;
        return;
    }
    n2 = (len >> 1 as libc::c_int) as libc::c_int;
    c = 2.0f64 / (len as libc::c_double - 1.0f64);
    if len & 1 as libc::c_int as libc::c_long != 0 {
        i = 0 as libc::c_int;
        while i < n2 {
            w = c - i as libc::c_double - 1.0f64;
            w = 1.0f64 - w * w;
            *w_data.offset(i as isize) = *data.offset(i as isize) as libc::c_double * w;
            *w_data.offset((len - 1 as libc::c_int as libc::c_long - i as libc::c_long) as isize) =
                *data.offset((len - 1 as libc::c_int as libc::c_long - i as libc::c_long) as isize)
                    as libc::c_double
                    * w;
            i += 1;
            i;
        }
        *w_data.offset(n2 as isize) = 0.0f64;
        return;
    }
    w_data = w_data.offset(n2 as isize);
    data = data.offset(n2 as isize);
    i = 0 as libc::c_int;
    while i < n2 {
        w = c - n2 as libc::c_double + i as libc::c_double;
        w = 1.0f64 - w * w;
        *w_data.offset((-i - 1 as libc::c_int) as isize) =
            *data.offset((-i - 1 as libc::c_int) as isize) as libc::c_double * w;
        *w_data.offset(i as isize) = *data.offset(i as isize) as libc::c_double * w;
        i += 1;
        i;
    }
}
unsafe extern "C" fn lpc_compute_autocorr_c(
    data: *const libc::c_double,
    len: ptrdiff_t,
    lag: libc::c_int,
    autoc: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < lag {
        let mut sum0: libc::c_double = 1.0f64;
        let mut sum1: libc::c_double = 1.0f64;
        i = j;
        while (i as libc::c_long) < len {
            sum0 += *data.offset(i as isize) * *data.offset((i - j) as isize);
            sum1 += *data.offset(i as isize) * *data.offset((i - j - 1 as libc::c_int) as isize);
            i += 1;
            i;
        }
        *autoc.offset(j as isize) = sum0;
        *autoc.offset((j + 1 as libc::c_int) as isize) = sum1;
        j += 2 as libc::c_int;
    }
    if j == lag {
        let mut sum: libc::c_double = 1.0f64;
        i = j - 1 as libc::c_int;
        while (i as libc::c_long) < len {
            sum += *data.offset(i as isize) * *data.offset((i - j) as isize)
                + *data.offset((i + 1 as libc::c_int) as isize)
                    * *data.offset((i - j + 1 as libc::c_int) as isize);
            i += 2 as libc::c_int;
        }
        *autoc.offset(j as isize) = sum;
    }
}

pub(crate) unsafe fn ff_lpc_calc_ref_coefs_f(
    s: *mut LPCContext,
    samples: *const libc::c_float,
    len: libc::c_int,
    order: libc::c_int,
    ref_0: *mut libc::c_double,
) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut signal: libc::c_double = 0.0f32 as libc::c_double;
    let mut avg_err: libc::c_double = 0.0f32 as libc::c_double;
    let mut autoc: [libc::c_double; 33] = [
        0 as libc::c_int as libc::c_double,
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
    let mut error: [libc::c_double; 33] = [
        0 as libc::c_int as libc::c_double,
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
    let a: libc::c_double = 0.5f32 as libc::c_double;
    let b: libc::c_double = 1.0f32 as libc::c_double - a;
    i = 0 as libc::c_int;
    while i <= len / 2 as libc::c_int {
        let weight: libc::c_double = a - b * cos(2 as libc::c_int as libc::c_double
            * 3.141_592_653_589_793_f64
            * i as libc::c_double
            / (len - 1 as libc::c_int) as libc::c_double);
        *((*s).windowed_samples).offset(i as isize) =
            weight * *samples.offset(i as isize) as libc::c_double;
        *((*s).windowed_samples).offset((len - 1 as libc::c_int - i) as isize) =
            weight * *samples.offset((len - 1 as libc::c_int - i) as isize) as libc::c_double;
        i += 1;
        i;
    }
    ((*s).lpc_compute_autocorr).expect("non-null function pointer")(
        (*s).windowed_samples,
        len as ptrdiff_t,
        order,
        autoc.as_mut_ptr(),
    );
    signal = autoc[0 as libc::c_int as usize];
    compute_ref_coefs(autoc.as_mut_ptr(), order, ref_0, error.as_mut_ptr());
    i = 0 as libc::c_int;
    while i < order {
        avg_err = (avg_err + error[i as usize]) / 2.0f32 as libc::c_double;
        i += 1;
        i;
    }
    if avg_err != 0. {
        signal / avg_err
    } else {
        ::core::f32::NAN as libc::c_double
    }
}

#[cold]
pub(crate) unsafe fn ff_lpc_init(
    s: *mut LPCContext,
    blocksize: libc::c_int,
    max_order: libc::c_int,
    lpc_type: FFLPCType,
) -> libc::c_int {
    (*s).blocksize = blocksize;
    (*s).max_order = max_order;
    (*s).lpc_type = lpc_type;
    (*s).windowed_buffer = alloc_zeroed(
        Layout::array::<libc::c_double>(
            (blocksize
                + 2 as libc::c_int
                + (max_order + 4 as libc::c_int - 1 as libc::c_int
                    & !(4 as libc::c_int - 1 as libc::c_int))) as usize,
        )
        .unwrap(),
    )
    .cast();
    if ((*s).windowed_buffer).is_null() {
        return -(12 as libc::c_int);
    }
    (*s).windowed_samples = ((*s).windowed_buffer).offset(
        (max_order + 4 as libc::c_int - 1 as libc::c_int & !(4 as libc::c_int - 1 as libc::c_int))
            as isize,
    );
    (*s).lpc_apply_welch_window = Some(lpc_apply_welch_window_c);
    (*s).lpc_compute_autocorr = Some(lpc_compute_autocorr_c);
    0 as libc::c_int
}

#[cold]
pub(crate) unsafe fn ff_lpc_end(_s: *mut LPCContext) {
    // TODO: this leaks 🚿
    // av_freep(&mut (*s).windowed_buffer as *mut *mut libc::c_double as *mut libc::c_void);
}
