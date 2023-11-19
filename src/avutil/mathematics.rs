use crate::{common::*, types::*};

use ::libc;

#[inline]
unsafe fn eval_poly(
    mut coeff: *const libc::c_double,
    mut size: libc::c_int,
    mut x: libc::c_double,
) -> libc::c_double {
    let mut sum: libc::c_double = *coeff.offset((size - 1 as libc::c_int) as isize);
    let mut i: libc::c_int = 0;
    i = size - 2 as libc::c_int;
    while i >= 0 as libc::c_int {
        sum *= x;
        sum += *coeff.offset(i as isize);
        i -= 1;
        i;
    }
    return sum;
}

pub(crate) unsafe fn av_bessel_i0(mut x: libc::c_double) -> libc::c_double {
    static mut p1: [libc::c_double; 15] = [
        -2.2335582639474375249e+15f64,
        -5.5050369673018427753e+14f64,
        -3.2940087627407749166e+13f64,
        -8.4925101247114157499e+11f64,
        -1.1912746104985237192e+10f64,
        -1.0313066708737980747e+08f64,
        -5.9545626019847898221e+05f64,
        -2.4125195876041896775e+03f64,
        -7.0935347449210549190e+00f64,
        -1.5453977791786851041e-02f64,
        -2.5172644670688975051e-05f64,
        -3.0517226450451067446e-08f64,
        -2.6843448573468483278e-11f64,
        -1.5982226675653184646e-14f64,
        -5.2487866627945699800e-18f64,
    ];
    static mut q1: [libc::c_double; 6] = [
        -2.2335582639474375245e+15f64,
        7.8858692566751002988e+12f64,
        -1.2207067397808979846e+10f64,
        1.0377081058062166144e+07f64,
        -4.8527560179962773045e+03f64,
        1.0f64,
    ];
    static mut p2: [libc::c_double; 7] = [
        -2.2210262233306573296e-04f64,
        1.3067392038106924055e-02f64,
        -4.4700805721174453923e-01f64,
        5.5674518371240761397e+00f64,
        -2.3517945679239481621e+01f64,
        3.1611322818701131207e+01f64,
        -9.6090021968656180000e+00f64,
    ];
    static mut q2: [libc::c_double; 8] = [
        -5.5194330231005480228e-04f64,
        3.2547697594819615062e-02f64,
        -1.1151759188741312645e+00f64,
        1.3982595353892851542e+01f64,
        -6.0228002066743340583e+01f64,
        8.5539563258012929600e+01f64,
        -3.1446690275135491500e+01f64,
        1.0f64,
    ];
    let mut y: libc::c_double = 0.;
    let mut r: libc::c_double = 0.;
    let mut factor: libc::c_double = 0.;
    if x == 0 as libc::c_int as libc::c_double {
        return 1.0f64;
    }
    x = fabs(x);
    if x <= 15 as libc::c_int as libc::c_double {
        y = x * x;
        return eval_poly(
            p1.as_ptr(),
            (::core::mem::size_of::<[libc::c_double; 15]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
                as libc::c_int,
            y,
        ) / eval_poly(
            q1.as_ptr(),
            (::core::mem::size_of::<[libc::c_double; 6]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
                as libc::c_int,
            y,
        );
    } else {
        y = 1 as libc::c_int as libc::c_double / x - 1.0f64 / 15 as libc::c_int as libc::c_double;
        r = eval_poly(
            p2.as_ptr(),
            (::core::mem::size_of::<[libc::c_double; 7]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
                as libc::c_int,
            y,
        ) / eval_poly(
            q2.as_ptr(),
            (::core::mem::size_of::<[libc::c_double; 8]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
                as libc::c_int,
            y,
        );
        factor = exp(x) / sqrt(x);
        return factor * r;
    };
}
