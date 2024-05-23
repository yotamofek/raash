use ffi::num::AVRational;
use libc::{c_int, c_long, c_uint, c_ulong};

use crate::types::*;

pub(crate) fn av_rescale_rnd(a: i64, b: i64, c: i64, mut rnd: AVRounding) -> i64 {
    if c <= 0
        || b < 0
        || !(rnd as c_uint & !(AV_ROUND_PASS_MINMAX as c_int) as c_uint <= 5
            && rnd as c_uint & !(AV_ROUND_PASS_MINMAX as c_int) as c_uint != 4)
    {
        if cfg!(debug_assertions) {
            panic!();
        } else {
            return i64::MIN;
        }
    }

    if rnd & AV_ROUND_PASS_MINMAX != 0 {
        if [i64::MIN, i64::MAX].contains(&a) {
            return a;
        }
        rnd = rnd.wrapping_sub(AV_ROUND_PASS_MINMAX);
    }

    if a < 0 {
        return (av_rescale_rnd(-a.max(-i64::MAX), b, c, rnd ^ ((rnd >> 1) & 1)) as c_ulong)
            .wrapping_neg() as c_long;
    }

    let r = match rnd {
        AV_ROUND_NEAR_INF => c / 2,
        _ if rnd & 1 != 0 => c - 1,
        _ => 0,
    };

    if b <= c_int::MAX.into() && c <= c_int::MAX.into() {
        if a <= c_int::MAX.into() {
            (a * b + r) / c
        } else {
            let ad = a / c;
            let a2 = (a % c * b + r) / c;
            if ad >= i32::MAX.into() && b != 0 && ad > (i64::MAX - a2) / b {
                return i64::MIN;
            }
            ad * b + a2
        }
    } else {
        let mut a0: c_ulong = (a & 0xffffffff as c_uint as c_long) as c_ulong;
        let mut a1: c_ulong = (a >> 32) as c_ulong;
        let b0: c_ulong = (b & 0xffffffff as c_uint as c_long) as c_ulong;
        let b1: c_ulong = (b >> 32) as c_ulong;
        let mut t1: c_ulong = a0.wrapping_mul(b1).wrapping_add(a1.wrapping_mul(b0));
        let t1a: c_ulong = t1 << 32;
        a0 = a0.wrapping_mul(b0).wrapping_add(t1a);
        a1 = a1
            .wrapping_mul(b1)
            .wrapping_add(t1 >> 32)
            .wrapping_add((a0 < t1a) as c_int as c_ulong);
        a0 = (a0 as c_ulong).wrapping_add(r as c_ulong) as c_ulong as c_ulong;
        a1 = (a1 as c_ulong).wrapping_add((a0 < r as c_ulong) as c_int as c_ulong) as c_ulong
            as c_ulong;
        let mut i = 63;
        while i >= 0 {
            a1 = (a1 as c_ulong).wrapping_add(a1.wrapping_add(a0 >> i & 1 as c_ulong)) as c_ulong
                as c_ulong;
            t1 = (t1 as c_ulong).wrapping_add(t1) as c_ulong as c_ulong;
            if c as c_ulong <= a1 {
                a1 = (a1 as c_ulong).wrapping_sub(c as c_ulong) as c_ulong as c_ulong;
                t1 = t1.wrapping_add(1);
            }
            i -= 1;
        }
        if t1 > 9223372036854775807 as c_long as c_ulong {
            return -(9223372036854775807 as c_long) - 1 as c_long;
        }
        t1 as c_long
    }
}

pub(crate) fn av_rescale_q_rnd(
    a: c_long,
    bq: AVRational,
    cq: AVRational,
    rnd: AVRounding,
) -> c_long {
    let b: c_long = bq.num as c_long * cq.den as c_long;
    let c: c_long = cq.num as c_long * bq.den as c_long;
    av_rescale_rnd(a, b, c, rnd)
}

pub(crate) fn av_rescale_q(a: c_long, bq: AVRational, cq: AVRational) -> c_long {
    av_rescale_q_rnd(a, bq, cq, AV_ROUND_NEAR_INF)
}
