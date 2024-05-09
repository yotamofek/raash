use std::{f32, f64};

use libc::{c_double, c_float, c_int};

pub(crate) fn sqrtf(n: c_float) -> c_float {
    n.sqrt()
}
pub(crate) fn fabsf(n: c_float) -> c_float {
    n.abs()
}

pub(crate) fn log2f(n: c_float) -> c_float {
    n.log2()
}
pub(crate) fn cbrtf(n: c_float) -> c_float {
    n.cbrt()
}
pub(crate) fn roundf(n: c_float) -> c_float {
    n.round()
}

pub(crate) fn av_clip_c(a: c_int, amin: c_int, amax: c_int) -> c_int {
    a.clamp(amin, amax)
}

pub(crate) trait Exp10 {
    fn exp10(x: Self) -> Self;
}

impl Exp10 for c_double {
    fn exp10(x: Self) -> Self {
        (f64::consts::LOG2_10 * x).exp2()
    }
}

impl Exp10 for c_float {
    fn exp10(x: Self) -> Self {
        (f32::consts::LOG2_10 * x).exp2()
    }
}
