use crate::{common::*};

use ::libc;

#[inline]
unsafe fn eval_poly(
    coeff: *const libc::c_double,
    size: libc::c_int,
    x: libc::c_double,
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
    sum
}

pub(crate) unsafe fn av_bessel_i0(mut x: libc::c_double) -> libc::c_double {
    static mut p1: [libc::c_double; 15] = [
        -2.233_558_263_947_437_5e15_f64,
        -5.505_036_967_301_842_5e14_f64,
        -3.294_008_762_740_775e13_f64,
        -8.492_510_124_711_416e11_f64,
        -1.191_274_610_498_523_7e10_f64,
        -1.031_306_670_873_798_1e8_f64,
        -5.954_562_601_984_789e5_f64,
        -2.412_519_587_604_19e3_f64,
        -7.093_534_744_921_055_f64,
        -1.545_397_779_178_685e-2_f64,
        -2.517_264_467_068_897_6e-5_f64,
        -3.051_722_645_045_107e-8_f64,
        -2.684_344_857_346_848_4e-11_f64,
        -1.598_222_667_565_318_5e-14_f64,
        -5.248_786_662_794_57e-18_f64,
    ];
    static mut q1: [libc::c_double; 6] = [
        -2.233_558_263_947_437_5e15_f64,
        7.885_869_256_675_101e12_f64,
        -1.220_706_739_780_897_9e10_f64,
        1.037_708_105_806_216_6e7_f64,
        -4.852_756_017_996_277_5e3_f64,
        1.0f64,
    ];
    static mut p2: [libc::c_double; 7] = [
        -2.221_026_223_330_657_3e-4_f64,
        1.306_739_203_810_692_4e-2_f64,
        -4.470_080_572_117_445e-1_f64,
        5.567_451_837_124_076_f64,
        -2.351_794_567_923_948e1_f64,
        3.161_132_281_870_113e1_f64,
        -9.609_002_196_865_617_f64,
    ];
    static mut q2: [libc::c_double; 8] = [
        -5.519_433_023_100_548e-4_f64,
        3.254_769_759_481_962e-2_f64,
        -1.115_175_918_874_131_3_f64,
        1.398_259_535_389_285_1e1_f64,
        -6.022_800_206_674_334e1_f64,
        8.553_956_325_801_293e1_f64,
        -3.144_669_027_513_549e1_f64,
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
        eval_poly(
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
        )
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
        factor * r
    }
}
