pub(crate) fn sqrtf(n: libc::c_float) -> libc::c_float {
    n.sqrt()
}
pub(crate) fn sqrt(n: libc::c_double) -> libc::c_double {
    n.sqrt()
}
pub(crate) fn fabsf(n: libc::c_float) -> libc::c_float {
    n.abs()
}
pub(crate) fn fabs(n: libc::c_double) -> libc::c_double {
    n.abs()
}

pub(crate) fn expf(n: libc::c_float) -> libc::c_float {
    n.exp()
}
pub(crate) fn logf(n: libc::c_float) -> libc::c_float {
    n.ln()
}
pub(crate) fn log2f(n: libc::c_float) -> libc::c_float {
    n.log2()
}
pub(crate) fn cbrtf(n: libc::c_float) -> libc::c_float {
    n.cbrt()
}
pub(crate) fn ceilf(n: libc::c_float) -> libc::c_float {
    n.ceil()
}
pub(crate) fn roundf(n: libc::c_float) -> libc::c_float {
    n.round()
}

pub(crate) fn powf(n: libc::c_float, i: libc::c_float) -> libc::c_float {
    n.powf(i)
}
pub(crate) fn atanf(n: libc::c_float) -> libc::c_float {
    n.atan()
}
pub(crate) fn exp(n: libc::c_double) -> libc::c_double {
    n.exp()
}
pub(crate) fn tan(n: libc::c_double) -> libc::c_double {
    n.tan()
}
pub(crate) fn cos(n: libc::c_double) -> libc::c_double {
    n.cos()
}
pub(crate) fn sin(n: libc::c_double) -> libc::c_double {
    n.sin()
}
pub(crate) fn exp2f(n: libc::c_float) -> libc::c_float {
    n.exp2()
}
pub(crate) fn exp2(n: libc::c_double) -> libc::c_double {
    n.exp2()
}
pub(crate) fn pow(n: libc::c_double, i: libc::c_double) -> libc::c_double {
    n.powf(i)
}
