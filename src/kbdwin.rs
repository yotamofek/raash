#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use std::{
    alloc::{alloc, Layout},
    f64::consts::PI,
};

use libc::{c_double, c_float, c_int, c_void};

use crate::{bessel, common::*};

// rustified in aactab
#[cold]
pub(crate) unsafe fn kbd_window_init(
    mut float_window: *mut c_float,
    mut int_window: *mut c_int,
    mut alpha: c_float,
    mut n: c_int,
) -> c_int {
    let mut i: c_int = 0;
    let mut sum: c_double = 0.0f64;
    let mut tmp: c_double = 0.;
    let mut scale: c_double = 0.0f64;
    let mut temp_small: [c_double; 513] = [0.; 513];
    let mut temp: *mut c_double = (if n <= 1024 as c_int {
        temp_small.as_mut_ptr() as *mut c_void
    } else {
        alloc(Layout::array::<c_double>((n / 2 as c_int + 1 as c_int) as usize).unwrap()).cast()
    }) as *mut c_double;
    let mut alpha2: c_double =
        4. * (alpha as c_double * PI / n as c_double) * (alpha as c_double * PI / n as c_double);
    if temp.is_null() {
        return -(12 as c_int);
    }
    i = 0 as c_int;
    while i <= n / 2 as c_int {
        tmp = alpha2 * i as c_double * (n - i) as c_double;
        *temp.offset(i as isize) = bessel::i0(sqrt(tmp));
        scale += *temp.offset(i as isize)
            * (1 as c_int + (i != 0 && i < n / 2 as c_int) as c_int) as c_double;
        i += 1;
        i;
    }
    scale = 1.0f64 / (scale + 1.);
    i = 0 as c_int;
    while i <= n / 2 as c_int {
        sum += *temp.offset(i as isize);
        if !float_window.is_null() {
            *float_window.offset(i as isize) = sqrt(sum * scale) as c_float;
        } else {
            *int_window.offset(i as isize) = lrint(2147483647. * sqrt(sum * scale)) as c_int;
        }
        i += 1;
        i;
    }
    while i < n {
        sum += *temp.offset((n - i) as isize);
        if !float_window.is_null() {
            *float_window.offset(i as isize) = sqrt(sum * scale) as c_float;
        } else {
            *int_window.offset(i as isize) = lrint(2147483647. * sqrt(sum * scale)) as c_int;
        }
        i += 1;
        i;
    }
    if temp != temp_small.as_mut_ptr() {
        // TODO: this leaks 🚿
        // av_free(temp as *mut c_void);
    }
    0 as c_int
}

#[cold]
pub(crate) unsafe fn avpriv_kbd_window_init(
    mut window: *mut c_float,
    mut alpha: c_float,
    mut n: c_int,
) -> c_int {
    kbd_window_init(window, std::ptr::null_mut::<c_int>(), alpha, n)
}

#[cold]
pub(crate) unsafe fn avpriv_kbd_window_init_fixed(
    mut window: *mut c_int,
    mut alpha: c_float,
    mut n: c_int,
) -> c_int {
    kbd_window_init(std::ptr::null_mut::<c_float>(), window, alpha, n)
}
