use libc::{
    c_char, c_double, c_float, c_int, c_long, c_longlong, c_uchar, c_uint, c_ulong, c_ulonglong,
    c_void,
};

pub(crate) fn sqrtf(n: c_float) -> c_float {
    n.sqrt()
}
pub(crate) fn sqrt(n: c_double) -> c_double {
    n.sqrt()
}
pub(crate) fn fabsf(n: c_float) -> c_float {
    n.abs()
}
pub(crate) fn fabs(n: c_double) -> c_double {
    n.abs()
}

pub(crate) fn expf(n: c_float) -> c_float {
    n.exp()
}
pub(crate) fn logf(n: c_float) -> c_float {
    n.ln()
}
pub(crate) fn log2f(n: c_float) -> c_float {
    n.log2()
}
pub(crate) fn cbrtf(n: c_float) -> c_float {
    n.cbrt()
}
pub(crate) fn ceilf(n: c_float) -> c_float {
    n.ceil()
}
pub(crate) fn roundf(n: c_float) -> c_float {
    n.round()
}

pub(crate) fn powf(n: c_float, i: c_float) -> c_float {
    n.powf(i)
}
pub(crate) fn atanf(n: c_float) -> c_float {
    n.atan()
}
pub(crate) fn exp(n: c_double) -> c_double {
    n.exp()
}
pub(crate) fn tan(n: c_double) -> c_double {
    n.tan()
}
pub(crate) fn cos(n: c_double) -> c_double {
    n.cos()
}
pub(crate) fn sin(n: c_double) -> c_double {
    n.sin()
}
pub(crate) fn exp2f(n: c_float) -> c_float {
    n.exp2()
}
pub(crate) fn exp2(n: c_double) -> c_double {
    n.exp2()
}
pub(crate) fn pow(n: c_double, i: c_double) -> c_double {
    n.powf(i)
}
pub(crate) fn sinf(n: c_float) -> c_float {
    n.sin()
}

pub(crate) fn lrintf(n: c_float) -> c_long {
    // TODO: is this correct???
    n as c_long
}
pub(crate) fn lrint(n: c_double) -> c_long {
    // TODO: is this correct???
    n as c_long
}
