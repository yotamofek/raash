use std::mem::size_of;

use ::libc;
use libc::{c_double, c_int, c_long, c_uint, c_ulong};

use crate::{common::*, types::*};

#[inline]
unsafe fn eval_poly(coeff: *const c_double, size: c_int, x: c_double) -> c_double {
    let mut sum: c_double = *coeff.offset((size - 1 as c_int) as isize);
    let mut i: c_int = 0;
    i = size - 2 as c_int;
    while i >= 0 as c_int {
        sum *= x;
        sum += *coeff.offset(i as isize);
        i -= 1;
        i;
    }
    sum
}

pub(crate) unsafe fn av_bessel_i0(mut x: c_double) -> c_double {
    static mut p1: [c_double; 15] = [
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
    static mut q1: [c_double; 6] = [
        -2.233_558_263_947_437_5e15_f64,
        7.885_869_256_675_101e12_f64,
        -1.220_706_739_780_897_9e10_f64,
        1.037_708_105_806_216_6e7_f64,
        -4.852_756_017_996_277_5e3_f64,
        1.0f64,
    ];
    static mut p2: [c_double; 7] = [
        -2.221_026_223_330_657_3e-4_f64,
        1.306_739_203_810_692_4e-2_f64,
        -4.470_080_572_117_445e-1_f64,
        5.567_451_837_124_076_f64,
        -2.351_794_567_923_948e1_f64,
        3.161_132_281_870_113e1_f64,
        -9.609_002_196_865_617_f64,
    ];
    static mut q2: [c_double; 8] = [
        -5.519_433_023_100_548e-4_f64,
        3.254_769_759_481_962e-2_f64,
        -1.115_175_918_874_131_3_f64,
        1.398_259_535_389_285_1e1_f64,
        -6.022_800_206_674_334e1_f64,
        8.553_956_325_801_293e1_f64,
        -3.144_669_027_513_549e1_f64,
        1.0f64,
    ];
    let mut y: c_double = 0.;
    let mut r: c_double = 0.;
    let mut factor: c_double = 0.;
    if x == 0 as c_int as c_double {
        return 1.0f64;
    }
    x = fabs(x);
    if x <= 15 as c_int as c_double {
        y = x * x;
        eval_poly(
            p1.as_ptr(),
            (size_of::<[c_double; 15]>() as c_ulong).wrapping_div(size_of::<c_double>() as c_ulong)
                as c_int,
            y,
        ) / eval_poly(
            q1.as_ptr(),
            (size_of::<[c_double; 6]>() as c_ulong).wrapping_div(size_of::<c_double>() as c_ulong)
                as c_int,
            y,
        )
    } else {
        y = 1 as c_int as c_double / x - 1.0f64 / 15 as c_int as c_double;
        r = eval_poly(
            p2.as_ptr(),
            (size_of::<[c_double; 7]>() as c_ulong).wrapping_div(size_of::<c_double>() as c_ulong)
                as c_int,
            y,
        ) / eval_poly(
            q2.as_ptr(),
            (size_of::<[c_double; 8]>() as c_ulong).wrapping_div(size_of::<c_double>() as c_ulong)
                as c_int,
            y,
        );
        factor = exp(x) / sqrt(x);
        factor * r
    }
}

pub(crate) unsafe fn av_rescale_rnd(
    a: c_long,
    b: c_long,
    c: c_long,
    mut rnd: AVRounding,
) -> c_long {
    let mut r: c_long = 0 as c_int as c_long;
    if c <= 0 as c_int as c_long
        || b < 0 as c_int as c_long
        || !(rnd as c_uint & !(AV_ROUND_PASS_MINMAX as c_int) as c_uint <= 5 as c_int as c_uint
            && rnd as c_uint & !(AV_ROUND_PASS_MINMAX as c_int) as c_uint != 4 as c_int as c_uint)
    {
        return -(9223372036854775807 as c_long) - 1 as c_int as c_long;
    }
    if rnd as c_uint & AV_ROUND_PASS_MINMAX as c_int as c_uint != 0 {
        if a == -(9223372036854775807 as c_long) - 1 as c_int as c_long
            || a == 9223372036854775807 as c_long
        {
            return a;
        }
        rnd = ::core::mem::transmute::<c_uint, AVRounding>(
            (rnd as c_uint).wrapping_sub(AV_ROUND_PASS_MINMAX as c_int as c_uint),
        ) as AVRounding;
    }
    if a < 0 as c_int as c_long {
        return (av_rescale_rnd(
            -if a > -(9223372036854775807 as c_long) {
                a
            } else {
                -(9223372036854775807 as c_long)
            },
            b,
            c,
            (rnd as c_uint ^ rnd as c_uint >> 1 as c_int & 1 as c_int as c_uint) as AVRounding,
        ) as c_ulong)
            .wrapping_neg() as c_long;
    }
    if rnd as c_uint == AV_ROUND_NEAR_INF as c_int as c_uint {
        r = c / 2 as c_int as c_long;
    } else if rnd as c_uint & 1 as c_int as c_uint != 0 {
        r = c - 1 as c_int as c_long;
    }
    if b <= 2147483647 as c_int as c_long && c <= 2147483647 as c_int as c_long {
        if a <= 2147483647 as c_int as c_long {
            (a * b + r) / c
        } else {
            let ad: c_long = a / c;
            let a2: c_long = (a % c * b + r) / c;
            if ad >= 2147483647 as c_int as c_long
                && b != 0
                && ad > (9223372036854775807 as c_long - a2) / b
            {
                return -(9223372036854775807 as c_long) - 1 as c_int as c_long;
            }
            ad * b + a2
        }
    } else {
        let mut a0: c_ulong = (a & 0xffffffff as c_uint as c_long) as c_ulong;
        let mut a1: c_ulong = (a >> 32 as c_int) as c_ulong;
        let b0: c_ulong = (b & 0xffffffff as c_uint as c_long) as c_ulong;
        let b1: c_ulong = (b >> 32 as c_int) as c_ulong;
        let mut t1: c_ulong = a0.wrapping_mul(b1).wrapping_add(a1.wrapping_mul(b0));
        let t1a: c_ulong = t1 << 32 as c_int;
        let mut i: c_int = 0;
        a0 = a0.wrapping_mul(b0).wrapping_add(t1a);
        a1 = a1
            .wrapping_mul(b1)
            .wrapping_add(t1 >> 32 as c_int)
            .wrapping_add((a0 < t1a) as c_int as c_ulong);
        a0 = (a0 as c_ulong).wrapping_add(r as c_ulong) as c_ulong as c_ulong;
        a1 = (a1 as c_ulong).wrapping_add((a0 < r as c_ulong) as c_int as c_ulong) as c_ulong
            as c_ulong;
        i = 63 as c_int;
        while i >= 0 as c_int {
            a1 = (a1 as c_ulong).wrapping_add(a1.wrapping_add(a0 >> i & 1 as c_int as c_ulong))
                as c_ulong as c_ulong;
            t1 = (t1 as c_ulong).wrapping_add(t1) as c_ulong as c_ulong;
            if c as c_ulong <= a1 {
                a1 = (a1 as c_ulong).wrapping_sub(c as c_ulong) as c_ulong as c_ulong;
                t1 = t1.wrapping_add(1);
                t1;
            }
            i -= 1;
            i;
        }
        if t1 > 9223372036854775807 as c_long as c_ulong {
            return -(9223372036854775807 as c_long) - 1 as c_int as c_long;
        }
        t1 as c_long
    }
}
#[allow(dead_code)]
pub(crate) unsafe fn av_rescale(a: c_long, b: c_long, c: c_long) -> c_long {
    av_rescale_rnd(a, b, c, AV_ROUND_NEAR_INF)
}
pub(crate) unsafe fn av_rescale_q_rnd(
    a: c_long,
    bq: AVRational,
    cq: AVRational,
    rnd: AVRounding,
) -> c_long {
    let b: c_long = bq.num as c_long * cq.den as c_long;
    let c: c_long = cq.num as c_long * bq.den as c_long;
    av_rescale_rnd(a, b, c, rnd)
}
pub(crate) unsafe fn av_rescale_q(a: c_long, bq: AVRational, cq: AVRational) -> c_long {
    av_rescale_q_rnd(a, bq, cq, AV_ROUND_NEAR_INF)
}
