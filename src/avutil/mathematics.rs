use ffi::num::AVRational;
use libc::{c_int, c_long, c_uint, c_ulong};

use crate::types::*;

pub(crate) fn av_rescale_rnd(a: c_long, b: c_long, c: c_long, mut rnd: AVRounding) -> c_long {
    let mut r: c_long = 0 as c_long;
    if c <= 0 as c_long
        || b < 0 as c_long
        || !(rnd as c_uint & !(AV_ROUND_PASS_MINMAX as c_int) as c_uint <= 5 as c_uint
            && rnd as c_uint & !(AV_ROUND_PASS_MINMAX as c_int) as c_uint != 4 as c_uint)
    {
        return -(9223372036854775807 as c_long) - 1 as c_long;
    }
    if rnd as c_uint & AV_ROUND_PASS_MINMAX as c_int as c_uint != 0 {
        if a == -(9223372036854775807 as c_long) - 1 as c_long || a == 9223372036854775807 as c_long
        {
            return a;
        }
        rnd = (rnd as c_uint).wrapping_sub(AV_ROUND_PASS_MINMAX as c_int as c_uint);
    }
    if a < 0 as c_long {
        return (av_rescale_rnd(
            -if a > -(9223372036854775807 as c_long) {
                a
            } else {
                -(9223372036854775807 as c_long)
            },
            b,
            c,
            (rnd as c_uint ^ rnd as c_uint >> 1 & 1 as c_uint) as AVRounding,
        ) as c_ulong)
            .wrapping_neg() as c_long;
    }
    if rnd as c_uint == AV_ROUND_NEAR_INF as c_int as c_uint {
        r = c / 2 as c_long;
    } else if rnd as c_uint & 1 as c_uint != 0 {
        r = c - 1 as c_long;
    }
    if b <= 2147483647 as c_long && c <= 2147483647 as c_long {
        if a <= 2147483647 as c_long {
            (a * b + r) / c
        } else {
            let ad: c_long = a / c;
            let a2: c_long = (a % c * b + r) / c;
            if ad >= 2147483647 as c_long && b != 0 && ad > (9223372036854775807 as c_long - a2) / b
            {
                return -(9223372036854775807 as c_long) - 1 as c_long;
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
        let mut i: c_int = 0;
        a0 = a0.wrapping_mul(b0).wrapping_add(t1a);
        a1 = a1
            .wrapping_mul(b1)
            .wrapping_add(t1 >> 32)
            .wrapping_add((a0 < t1a) as c_int as c_ulong);
        a0 = (a0 as c_ulong).wrapping_add(r as c_ulong) as c_ulong as c_ulong;
        a1 = (a1 as c_ulong).wrapping_add((a0 < r as c_ulong) as c_int as c_ulong) as c_ulong
            as c_ulong;
        i = 63;
        while i >= 0 {
            a1 = (a1 as c_ulong).wrapping_add(a1.wrapping_add(a0 >> i & 1 as c_ulong)) as c_ulong
                as c_ulong;
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
