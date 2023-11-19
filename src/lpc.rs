#![allow(clippy::self_assignment)]

use std::alloc::alloc_zeroed;
use std::alloc::Layout;
use std::ptr;

use crate::avutil::lls::avpriv_init_lls;
use crate::avutil::lls::avpriv_solve_lls;
use crate::common::*;
use crate::types::*;

pub type LPC_TYPE = libc::c_double;
pub type LPC_TYPE_U = libc::c_double;
#[inline(always)]
unsafe extern "C" fn av_clip_c(
    a: libc::c_int,
    amin: libc::c_int,
    amax: libc::c_int,
) -> libc::c_int {
    if a < amin {
        amin
    } else if a > amax {
        return amax;
    } else {
        return a;
    }
}
#[inline]
unsafe extern "C" fn compute_ref_coefs(
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
#[inline]
unsafe extern "C" fn compute_lpc_coefs(
    mut autoc: *const LPC_TYPE,
    max_order: libc::c_int,
    mut lpc: *mut LPC_TYPE,
    lpc_stride: libc::c_int,
    fail: libc::c_int,
    normalize: libc::c_int,
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
            let f: LPC_TYPE = *lpc_last.offset(j as isize);
            let b: LPC_TYPE = *lpc_last.offset((i - 1 as libc::c_int - j) as isize);
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
    0 as libc::c_int
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
unsafe extern "C" fn quantize_lpc_coefs(
    lpc_in: *mut libc::c_double,
    order: libc::c_int,
    precision: libc::c_int,
    lpc_out: *mut int32_t,
    shift: *mut libc::c_int,
    min_shift: libc::c_int,
    max_shift: libc::c_int,
    zero_shift: libc::c_int,
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
        ptr::write_bytes(lpc_out, 0, order as usize);
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
        let scale: libc::c_double = qmax as libc::c_double / cmax;
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
    ref_0: *mut libc::c_double,
    min_order: libc::c_int,
    max_order: libc::c_int,
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
    est
}
#[no_mangle]
pub unsafe extern "C" fn ff_lpc_calc_ref_coefs(
    s: *mut LPCContext,
    samples: *const int32_t,
    order: libc::c_int,
    ref_0: *mut libc::c_double,
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
    compute_ref_coefs(autoc.as_mut_ptr(), order, ref_0, std::ptr::null_mut::<LPC_TYPE>());
    order
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

pub(crate) unsafe fn ff_lpc_calc_coefs(
    s: *mut LPCContext,
    samples: *const int32_t,
    blocksize: libc::c_int,
    min_order: libc::c_int,
    max_order: libc::c_int,
    precision: libc::c_int,
    coefs: *mut [int32_t; 32],
    shift: *mut libc::c_int,
    lpc_type: FFLPCType,
    mut lpc_passes: libc::c_int,
    omethod: libc::c_int,
    min_shift: libc::c_int,
    max_shift: libc::c_int,
    zero_shift: libc::c_int,
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
        let m: *mut LLSModel = ((*s).lls_models).as_mut_ptr();
        let mut la_var: [libc::c_double; 36] = [0.; 36];
        let var: *mut libc::c_double = la_var.as_mut_ptr();
        let mut weight: libc::c_double = 0.;
        weight = weight;
        ptr::write_bytes(
            var,
            0,
            (32 as libc::c_int + 1 as libc::c_int + 4 as libc::c_int - 1 as libc::c_int
                & !(4 as libc::c_int - 1 as libc::c_int)) as usize,
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
                        let fresh1 = &mut (*var.offset(j as isize));
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
    opt_order
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
    0 as libc::c_int
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn ff_lpc_end(_s: *mut LPCContext) {
    // TODO: this leaks 🚿
    // av_freep(&mut (*s).windowed_buffer as *mut *mut libc::c_double as *mut libc::c_void);
}
