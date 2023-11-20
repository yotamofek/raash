#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

mod codes;

use std::{
    f32::consts::SQRT_2,
    f64::consts::PI,
    iter::zip,
    ptr,
    sync::{Once, OnceLock},
};

use libc::{c_double, c_float, c_int, c_uchar, c_uint, c_ushort};
use once_cell::sync::Lazy;

pub(crate) use self::codes::*;
use crate::{bessel, kbdwin::avpriv_kbd_window_init, sinewin::ff_init_ff_sine_windows};

pub(crate) struct PowSfTables {
    pub pow2: [c_float; 428],
    pub pow34: [c_float; 428],
}

pub(crate) static POW_SF_TABLES: Lazy<PowSfTables> = Lazy::new(|| {
    let mut pow2 = [0.; 428];
    let mut pow34 = [0.; 428];

    const EXP2_LUT: [c_float; 16] = [
        1.00000000000000000000,
        1.044_273_7,
        1.090_507_7,
        1.138_788_6,
        1.189_207_1,
        1.241_857_8,
        1.296_839_6,
        1.354_255_6,
        SQRT_2,
        1.476_826_2,
        1.542_210_8,
        1.610_490_3,
        1.681_792_9,
        1.756_252_2,
        1.834_008_1,
        1.915_206_6,
    ];
    let mut t1: c_float = 8.881_784e-16;
    let mut t2: c_float = 3.637_979e-12;
    let mut t1_inc_cur: c_int = 0;
    let mut t2_inc_cur: c_int = 0;
    let mut t1_inc_prev: c_int = 0 as c_int;
    let mut t2_inc_prev: c_int = 8 as c_int;
    for (i, (pow2, pow34)) in zip(&mut pow2, &mut pow34).enumerate() {
        t1_inc_cur = 4 as c_int * (i as c_int % 4);
        t2_inc_cur = (8 as c_int + 3 as c_int * i as c_int) % 16 as c_int;
        if t1_inc_cur < t1_inc_prev {
            t1 *= 2 as c_int as c_float;
        }
        if t2_inc_cur < t2_inc_prev {
            t2 *= 2 as c_int as c_float;
        }
        *pow2 = t1 * EXP2_LUT[t1_inc_cur as usize];
        *pow34 = t2 * EXP2_LUT[t2_inc_cur as usize];
        t1_inc_prev = t1_inc_cur;
        t2_inc_prev = t2_inc_cur;
    }

    PowSfTables { pow2, pow34 }
});

pub(crate) fn kbd_window_init<const N: usize>(mut alpha: c_float) -> [c_float; N]
where
    [(); N / 2 + 1]:,
{
    let mut float_window: [c_float; N] = [0.; N];
    let mut temp: [c_double; N / 2 + 1] = [0.; N / 2 + 1];
    let mut i: c_int = 0;
    let mut sum: c_double = 0.0f64;
    let mut scale: c_double = 0.0f64;
    let mut alpha2: c_double =
        4. * (alpha as c_double * PI / N as c_double) * (alpha as c_double * PI / N as c_double);

    i = 0 as c_int;
    for (i, temp) in temp.iter_mut().enumerate() {
        let tmp = alpha2 * i as c_double * (N - i) as c_double;
        *temp = bessel::i0(tmp.sqrt());
        scale += *temp * (1 as c_int + (i != 0 && i < (N / 2)) as c_int) as c_double;
    }

    scale = 1.0f64 / (scale + 1.);

    i = 0 as c_int;
    while i <= (N / 2) as c_int {
        sum += temp[i as usize];
        float_window[i as usize] = (sum * scale).sqrt() as c_float;

        i += 1;
    }
    while i < N as c_int {
        sum += temp[(N as c_int - i) as usize];
        float_window[i as usize] = (sum * scale).sqrt() as c_float;

        i += 1;
    }

    float_window
}

pub(crate) static KBD_LONG: Lazy<[c_float; 1024]> = Lazy::new(|| kbd_window_init(4.));
pub(crate) static KBD_SHORT: Lazy<[c_float; 128]> = Lazy::new(|| kbd_window_init(6.));

#[cold]
unsafe fn aac_float_common_init() {
    ff_init_ff_sine_windows(10 as c_int);
    ff_init_ff_sine_windows(7 as c_int);
}

#[cold]
pub(crate) unsafe fn ff_aac_float_common_init() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| aac_float_common_init());
}
