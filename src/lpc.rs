#![allow(clippy::self_assignment)]

use crate::common::*;
use crate::types::*;

extern "C" {
    fn lrintf(_: libc::c_float) -> libc::c_long;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn av_mallocz(size: size_t) -> *mut libc::c_void;
    fn av_freep(ptr: *mut libc::c_void);
    fn avpriv_init_lls(m: *mut LLSModel, indep_count: libc::c_int);
    fn avpriv_solve_lls(m: *mut LLSModel, threshold: libc::c_double, min_order: libc::c_ushort);
    fn av_log(avcl: *mut libc::c_void, level: libc::c_int, fmt: *const libc::c_char, _: ...);
}
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
pub type size_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LLSModel {
    pub covariance: [[libc::c_double; 36]; 36],
    pub coeff: [[libc::c_double; 32]; 32],
    pub variance: [libc::c_double; 32],
    pub indep_count: libc::c_int,
    pub update_lls: Option<unsafe extern "C" fn(*mut LLSModel, *const libc::c_double) -> ()>,
    pub evaluate_lls: Option<
        unsafe extern "C" fn(*mut LLSModel, *const libc::c_double, libc::c_int) -> libc::c_double,
    >,
}
pub type FFLPCType = libc::c_int;
pub const FF_LPC_TYPE_NB: FFLPCType = 4;
pub const FF_LPC_TYPE_CHOLESKY: FFLPCType = 3;
pub const FF_LPC_TYPE_LEVINSON: FFLPCType = 2;
pub const FF_LPC_TYPE_FIXED: FFLPCType = 1;
pub const FF_LPC_TYPE_NONE: FFLPCType = 0;
pub const FF_LPC_TYPE_DEFAULT: FFLPCType = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LPCContext {
    pub blocksize: libc::c_int,
    pub max_order: libc::c_int,
    pub lpc_type: FFLPCType,
    pub windowed_buffer: *mut libc::c_double,
    pub windowed_samples: *mut libc::c_double,
    pub lpc_apply_welch_window:
        Option<unsafe extern "C" fn(*const int32_t, ptrdiff_t, *mut libc::c_double) -> ()>,
    pub lpc_compute_autocorr: Option<
        unsafe extern "C" fn(
            *const libc::c_double,
            ptrdiff_t,
            libc::c_int,
            *mut libc::c_double,
        ) -> (),
    >,
    pub lls_models: [LLSModel; 2],
}
pub type LPC_TYPE = libc::c_double;
pub type LPC_TYPE_U = libc::c_double;
#[inline(always)]
unsafe extern "C" fn av_clip_c(
    mut a: libc::c_int,
    mut amin: libc::c_int,
    mut amax: libc::c_int,
) -> libc::c_int {
    if a < amin {
        return amin;
    } else if a > amax {
        return amax;
    } else {
        return a;
    };
}
#[inline]
unsafe extern "C" fn compute_ref_coefs(
    mut autoc: *const LPC_TYPE,
    mut max_order: libc::c_int,
    mut ref_0: *mut LPC_TYPE,
    mut error: *mut LPC_TYPE,
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
            gen0[j as usize] = gen1[(j + 1 as libc::c_int) as usize]
                * *ref_0.offset((i - 1 as libc::c_int) as isize)
                + gen0[j as usize];
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
#[inline]
unsafe extern "C" fn compute_lpc_coefs(
    mut autoc: *const LPC_TYPE,
    mut max_order: libc::c_int,
    mut lpc: *mut LPC_TYPE,
    mut lpc_stride: libc::c_int,
    mut fail: libc::c_int,
    mut normalize: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut err: LPC_TYPE = 0 as libc::c_int as LPC_TYPE;
    let mut lpc_last: *mut LPC_TYPE = lpc;
    if normalize != 0 {
        let fresh0 = autoc;
        autoc = autoc.offset(1);
        err = *fresh0;
    }
    if fail != 0
        && (*autoc.offset((max_order - 1 as libc::c_int) as isize)
            == 0 as libc::c_int as libc::c_double
            || err <= 0 as libc::c_int as libc::c_double)
    {
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < max_order {
        let mut r: LPC_TYPE = -*autoc.offset(i as isize);
        if normalize != 0 {
            j = 0 as libc::c_int;
            while j < i {
                r -= *lpc_last.offset(j as isize)
                    * *autoc.offset((i - j - 1 as libc::c_int) as isize);
                j += 1;
                j;
            }
            if err != 0. {
                r /= err;
            }
            err *= 1.0f64 as libc::c_float as libc::c_double - r * r;
        }
        *lpc.offset(i as isize) = r;
        j = 0 as libc::c_int;
        while j < i + 1 as libc::c_int >> 1 as libc::c_int {
            let mut f: LPC_TYPE = *lpc_last.offset(j as isize);
            let mut b: LPC_TYPE = *lpc_last.offset((i - 1 as libc::c_int - j) as isize);
            *lpc.offset(j as isize) = f + r * b;
            *lpc.offset((i - 1 as libc::c_int - j) as isize) = b + r * f;
            j += 1;
            j;
        }
        if fail != 0 && err < 0 as libc::c_int as libc::c_double {
            return -(1 as libc::c_int);
        }
        lpc_last = lpc;
        lpc = lpc.offset(lpc_stride as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn lpc_apply_welch_window_c(
    mut data: *const int32_t,
    mut len: ptrdiff_t,
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
    mut data: *const libc::c_double,
    mut len: ptrdiff_t,
    mut lag: libc::c_int,
    mut autoc: *mut libc::c_double,
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
unsafe extern "C" fn quantize_lpc_coefs(
    mut lpc_in: *mut libc::c_double,
    mut order: libc::c_int,
    mut precision: libc::c_int,
    mut lpc_out: *mut int32_t,
    mut shift: *mut libc::c_int,
    mut min_shift: libc::c_int,
    mut max_shift: libc::c_int,
    mut zero_shift: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut cmax: libc::c_double = 0.;
    let mut error: libc::c_double = 0.;
    let mut qmax: int32_t = 0;
    let mut sh: libc::c_int = 0;
    qmax = ((1 as libc::c_int) << precision - 1 as libc::c_int) - 1 as libc::c_int;
    cmax = 0.0f64;
    i = 0 as libc::c_int;
    while i < order {
        cmax = if cmax > fabs(*lpc_in.offset(i as isize)) {
            cmax
        } else {
            fabs(*lpc_in.offset(i as isize))
        };
        i += 1;
        i;
    }
    if (cmax * ((1 as libc::c_int) << max_shift) as libc::c_double) < 1.0f64 {
        *shift = zero_shift;
        memset(
            lpc_out as *mut libc::c_void,
            0 as libc::c_int,
            (::core::mem::size_of::<int32_t>() as libc::c_ulong)
                .wrapping_mul(order as libc::c_ulong),
        );
        return;
    }
    sh = max_shift;
    while cmax * ((1 as libc::c_int) << sh) as libc::c_double > qmax as libc::c_double
        && sh > min_shift
    {
        sh -= 1;
        sh;
    }
    if sh == 0 as libc::c_int && cmax > qmax as libc::c_double {
        let mut scale: libc::c_double = qmax as libc::c_double / cmax;
        i = 0 as libc::c_int;
        while i < order {
            *lpc_in.offset(i as isize) *= scale;
            i += 1;
            i;
        }
    }
    error = 0 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int;
    while i < order {
        error -= *lpc_in.offset(i as isize) * ((1 as libc::c_int) << sh) as libc::c_double;
        *lpc_out.offset(i as isize) =
            av_clip_c(lrintf(error as libc::c_float) as libc::c_int, -qmax, qmax);
        error -= *lpc_out.offset(i as isize) as libc::c_double;
        i += 1;
        i;
    }
    *shift = sh;
}
unsafe extern "C" fn estimate_best_order(
    mut ref_0: *mut libc::c_double,
    mut min_order: libc::c_int,
    mut max_order: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut est: libc::c_int = 0;
    est = min_order;
    i = max_order - 1 as libc::c_int;
    while i >= min_order - 1 as libc::c_int {
        if *ref_0.offset(i as isize) > 0.10f64 {
            est = i + 1 as libc::c_int;
            break;
        } else {
            i -= 1;
            i;
        }
    }
    return est;
}
#[no_mangle]
pub unsafe extern "C" fn ff_lpc_calc_ref_coefs(
    mut s: *mut LPCContext,
    mut samples: *const int32_t,
    mut order: libc::c_int,
    mut ref_0: *mut libc::c_double,
) -> libc::c_int {
    let mut autoc: [libc::c_double; 33] = [0.; 33];
    ((*s).lpc_apply_welch_window).expect("non-null function pointer")(
        samples,
        (*s).blocksize as ptrdiff_t,
        (*s).windowed_samples,
    );
    ((*s).lpc_compute_autocorr).expect("non-null function pointer")(
        (*s).windowed_samples,
        (*s).blocksize as ptrdiff_t,
        order,
        autoc.as_mut_ptr(),
    );
    compute_ref_coefs(autoc.as_mut_ptr(), order, ref_0, 0 as *mut LPC_TYPE);
    return order;
}
#[no_mangle]
pub unsafe extern "C" fn ff_lpc_calc_ref_coefs_f(
    mut s: *mut LPCContext,
    mut samples: *const libc::c_float,
    mut len: libc::c_int,
    mut order: libc::c_int,
    mut ref_0: *mut libc::c_double,
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
        let mut weight: libc::c_double = a - b * cos(2 as libc::c_int as libc::c_double
            * 3.14159265358979323846f64
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
    return if avg_err != 0. {
        signal / avg_err
    } else {
        ::core::f32::NAN as libc::c_double
    };
}
#[no_mangle]
pub unsafe extern "C" fn ff_lpc_calc_coefs(
    mut s: *mut LPCContext,
    mut samples: *const int32_t,
    mut blocksize: libc::c_int,
    mut min_order: libc::c_int,
    mut max_order: libc::c_int,
    mut precision: libc::c_int,
    mut coefs: *mut [int32_t; 32],
    mut shift: *mut libc::c_int,
    mut lpc_type: FFLPCType,
    mut lpc_passes: libc::c_int,
    mut omethod: libc::c_int,
    mut min_shift: libc::c_int,
    mut max_shift: libc::c_int,
    mut zero_shift: libc::c_int,
) -> libc::c_int {
    let mut autoc: [libc::c_double; 33] = [0.; 33];
    let mut ref_0: [libc::c_double; 32] = [
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
    ];
    let mut lpc: [[libc::c_double; 32]; 32] = [[0.; 32]; 32];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pass: libc::c_int = 0 as libc::c_int;
    let mut opt_order: libc::c_int = 0;
    assert!([FF_LPC_TYPE_CHOLESKY, FF_LPC_TYPE_LEVINSON].contains(&lpc_type));
    if blocksize != (*s).blocksize
        || max_order != (*s).max_order
        || lpc_type as libc::c_int != (*s).lpc_type as libc::c_int
    {
        ff_lpc_end(s);
        ff_lpc_init(s, blocksize, max_order, lpc_type);
    }
    if lpc_passes <= 0 as libc::c_int {
        lpc_passes = 2 as libc::c_int;
    }
    if lpc_type as libc::c_int == FF_LPC_TYPE_LEVINSON as libc::c_int
        || lpc_type as libc::c_int == FF_LPC_TYPE_CHOLESKY as libc::c_int
            && lpc_passes > 1 as libc::c_int
    {
        ((*s).lpc_apply_welch_window).expect("non-null function pointer")(
            samples,
            blocksize as ptrdiff_t,
            (*s).windowed_samples,
        );
        ((*s).lpc_compute_autocorr).expect("non-null function pointer")(
            (*s).windowed_samples,
            blocksize as ptrdiff_t,
            max_order,
            autoc.as_mut_ptr(),
        );
        compute_lpc_coefs(
            autoc.as_mut_ptr(),
            max_order,
            &mut *(*lpc.as_mut_ptr().offset(0 as libc::c_int as isize))
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize),
            32 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
        );
        i = 0 as libc::c_int;
        while i < max_order {
            ref_0[i as usize] = fabs(lpc[i as usize][i as usize]);
            i += 1;
            i;
        }
        pass += 1;
        pass;
    }
    if lpc_type as libc::c_int == FF_LPC_TYPE_CHOLESKY as libc::c_int {
        let mut m: *mut LLSModel = ((*s).lls_models).as_mut_ptr();
        let mut la_var: [libc::c_double; 36] = [0.; 36];
        let mut var: *mut libc::c_double = la_var.as_mut_ptr();
        let mut weight: libc::c_double = 0.;
        weight = weight;
        memset(
            var as *mut libc::c_void,
            0 as libc::c_int,
            ((32 as libc::c_int + 1 as libc::c_int + 4 as libc::c_int - 1 as libc::c_int
                & !(4 as libc::c_int - 1 as libc::c_int)) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
        j = 0 as libc::c_int;
        while j < max_order {
            (*m.offset(0 as libc::c_int as isize)).coeff[(max_order - 1 as libc::c_int) as usize]
                [j as usize] = -lpc[(max_order - 1 as libc::c_int) as usize][j as usize];
            j += 1;
            j;
        }
        while pass < lpc_passes {
            avpriv_init_lls(
                &mut *m.offset((pass & 1 as libc::c_int) as isize),
                max_order,
            );
            weight = 0 as libc::c_int as libc::c_double;
            i = max_order;
            while i < blocksize {
                j = 0 as libc::c_int;
                while j <= max_order {
                    *var.offset(j as isize) = *samples.offset((i - j) as isize) as libc::c_double;
                    j += 1;
                    j;
                }
                if pass != 0 {
                    let mut eval: libc::c_double = 0.;
                    let mut inv: libc::c_double = 0.;
                    let mut rinv: libc::c_double = 0.;
                    eval = ((*m.offset((pass & 1 as libc::c_int) as isize)).evaluate_lls)
                        .expect("non-null function pointer")(
                        &mut *m.offset((pass - 1 as libc::c_int & 1 as libc::c_int) as isize),
                        var.offset(1 as libc::c_int as isize),
                        max_order - 1 as libc::c_int,
                    );
                    eval = (512 as libc::c_int >> pass) as libc::c_double
                        + fabs(eval - *var.offset(0 as libc::c_int as isize));
                    inv = 1 as libc::c_int as libc::c_double / eval;
                    rinv = sqrt(inv);
                    j = 0 as libc::c_int;
                    while j <= max_order {
                        let ref mut fresh1 = *var.offset(j as isize);
                        *fresh1 *= rinv;
                        j += 1;
                        j;
                    }
                    weight += inv;
                } else {
                    weight += 1.;
                    weight;
                }
                ((*m.offset((pass & 1 as libc::c_int) as isize)).update_lls)
                    .expect("non-null function pointer")(
                    &mut *m.offset((pass & 1 as libc::c_int) as isize),
                    var,
                );
                i += 1;
                i;
            }
            avpriv_solve_lls(
                &mut *m.offset((pass & 1 as libc::c_int) as isize),
                0.001f64,
                0 as libc::c_int as libc::c_ushort,
            );
            pass += 1;
            pass;
        }
        i = 0 as libc::c_int;
        while i < max_order {
            j = 0 as libc::c_int;
            while j < max_order {
                lpc[i as usize][j as usize] = -(*m
                    .offset((pass - 1 as libc::c_int & 1 as libc::c_int) as isize))
                .coeff[i as usize][j as usize];
                j += 1;
                j;
            }
            ref_0[i as usize] = sqrt(
                (*m.offset((pass - 1 as libc::c_int & 1 as libc::c_int) as isize)).variance
                    [i as usize]
                    / weight,
            ) * (blocksize - max_order) as libc::c_double
                / 4000 as libc::c_int as libc::c_double;
            i += 1;
            i;
        }
        i = max_order - 1 as libc::c_int;
        while i > 0 as libc::c_int {
            ref_0[i as usize] = ref_0[(i - 1 as libc::c_int) as usize] - ref_0[i as usize];
            i -= 1;
            i;
        }
    }
    opt_order = max_order;
    if omethod == 0 as libc::c_int {
        opt_order = estimate_best_order(ref_0.as_mut_ptr(), min_order, max_order);
        i = opt_order - 1 as libc::c_int;
        quantize_lpc_coefs(
            (lpc[i as usize]).as_mut_ptr(),
            i + 1 as libc::c_int,
            precision,
            (*coefs.offset(i as isize)).as_mut_ptr(),
            &mut *shift.offset(i as isize),
            min_shift,
            max_shift,
            zero_shift,
        );
    } else {
        i = min_order - 1 as libc::c_int;
        while i < max_order {
            quantize_lpc_coefs(
                (lpc[i as usize]).as_mut_ptr(),
                i + 1 as libc::c_int,
                precision,
                (*coefs.offset(i as isize)).as_mut_ptr(),
                &mut *shift.offset(i as isize),
                min_shift,
                max_shift,
                zero_shift,
            );
            i += 1;
            i;
        }
    }
    return opt_order;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn ff_lpc_init(
    mut s: *mut LPCContext,
    mut blocksize: libc::c_int,
    mut max_order: libc::c_int,
    mut lpc_type: FFLPCType,
) -> libc::c_int {
    (*s).blocksize = blocksize;
    (*s).max_order = max_order;
    (*s).lpc_type = lpc_type;
    (*s).windowed_buffer = av_mallocz(
        ((blocksize
            + 2 as libc::c_int
            + (max_order + 4 as libc::c_int - 1 as libc::c_int
                & !(4 as libc::c_int - 1 as libc::c_int))) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*s).windowed_buffer).is_null() {
        return -(12 as libc::c_int);
    }
    (*s).windowed_samples = ((*s).windowed_buffer).offset(
        (max_order + 4 as libc::c_int - 1 as libc::c_int & !(4 as libc::c_int - 1 as libc::c_int))
            as isize,
    );
    (*s).lpc_apply_welch_window = Some(
        lpc_apply_welch_window_c
            as unsafe extern "C" fn(*const int32_t, ptrdiff_t, *mut libc::c_double) -> (),
    );
    (*s).lpc_compute_autocorr = Some(
        lpc_compute_autocorr_c
            as unsafe extern "C" fn(
                *const libc::c_double,
                ptrdiff_t,
                libc::c_int,
                *mut libc::c_double,
            ) -> (),
    );
    return 0 as libc::c_int;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn ff_lpc_end(mut s: *mut LPCContext) {
    av_freep(&mut (*s).windowed_buffer as *mut *mut libc::c_double as *mut libc::c_void);
}
