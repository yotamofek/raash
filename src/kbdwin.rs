#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use crate::common::*;

extern "C" {
    fn lrint(_: libc::c_double) -> libc::c_long;
    fn av_bessel_i0(x: libc::c_double) -> libc::c_double;
    fn av_malloc(size: size_t) -> *mut libc::c_void;
    fn av_free(ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
#[cold]
unsafe extern "C" fn kbd_window_init(
    mut float_window: *mut libc::c_float,
    mut int_window: *mut libc::c_int,
    mut alpha: libc::c_float,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut sum: libc::c_double = 0.0f64;
    let mut tmp: libc::c_double = 0.;
    let mut scale: libc::c_double = 0.0f64;
    let mut temp_small: [libc::c_double; 513] = [0.; 513];
    let mut temp: *mut libc::c_double = (if n <= 1024 as libc::c_int {
        temp_small.as_mut_ptr() as *mut libc::c_void
    } else {
        av_malloc(
            ((n / 2 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
        )
    }) as *mut libc::c_double;
    let mut alpha2: libc::c_double = 4 as libc::c_int as libc::c_double
        * (alpha as libc::c_double * 3.14159265358979323846f64 / n as libc::c_double)
        * (alpha as libc::c_double * 3.14159265358979323846f64 / n as libc::c_double);
    if temp.is_null() {
        return -(12 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i <= n / 2 as libc::c_int {
        tmp = alpha2 * i as libc::c_double * (n - i) as libc::c_double;
        *temp.offset(i as isize) = av_bessel_i0(sqrt(tmp));
        scale += *temp.offset(i as isize)
            * (1 as libc::c_int + (i != 0 && i < n / 2 as libc::c_int) as libc::c_int)
                as libc::c_double;
        i += 1;
        i;
    }
    scale = 1.0f64 / (scale + 1 as libc::c_int as libc::c_double);
    i = 0 as libc::c_int;
    while i <= n / 2 as libc::c_int {
        sum += *temp.offset(i as isize);
        if !float_window.is_null() {
            *float_window.offset(i as isize) = sqrt(sum * scale) as libc::c_float;
        } else {
            *int_window.offset(i as isize) =
                lrint(2147483647 as libc::c_int as libc::c_double * sqrt(sum * scale))
                    as libc::c_int;
        }
        i += 1;
        i;
    }
    while i < n {
        sum += *temp.offset((n - i) as isize);
        if !float_window.is_null() {
            *float_window.offset(i as isize) = sqrt(sum * scale) as libc::c_float;
        } else {
            *int_window.offset(i as isize) =
                lrint(2147483647 as libc::c_int as libc::c_double * sqrt(sum * scale))
                    as libc::c_int;
        }
        i += 1;
        i;
    }
    if temp != temp_small.as_mut_ptr() {
        av_free(temp as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn avpriv_kbd_window_init(
    mut window: *mut libc::c_float,
    mut alpha: libc::c_float,
    mut n: libc::c_int,
) -> libc::c_int {
    return kbd_window_init(window, 0 as *mut libc::c_int, alpha, n);
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn avpriv_kbd_window_init_fixed(
    mut window: *mut int32_t,
    mut alpha: libc::c_float,
    mut n: libc::c_int,
) -> libc::c_int {
    return kbd_window_init(0 as *mut libc::c_float, window, alpha, n);
}
