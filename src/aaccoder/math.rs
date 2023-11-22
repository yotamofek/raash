use ilog::IntLog;
use libc::{c_float, c_int, c_uchar, c_uint};

pub(super) fn coef2minsf(mut coef: c_float) -> c_uchar {
    clip_uint8_c((coef.log2() * 4. - 69. + 140. - 36.) as c_int)
}

#[inline(always)]
pub(super) fn ff_fast_powf(mut x: c_float, mut y: c_float) -> c_float {
    (x.ln() * y).exp()
}

#[inline(always)]
pub(super) fn bval2bmax(mut b: c_float) -> c_float {
    0.001f32 + 0.0035f32 * b.powi(3) / 15.5f32.powi(3)
}

#[inline(always)]
pub(super) fn ff_log2_c(mut v: c_uint) -> c_int {
    // TODO: is this (the cast) correct??
    v.log2() as c_int
    // let mut n: c_int = 0 as c_int;
    // if v & 0xffff0000 as c_uint != 0 {
    //     v >>= 16 as c_int;
    //     n += 16 as c_int;
    // }
    // if v & 0xff00 as c_int as c_uint != 0 {
    //     v >>= 8 as c_int;
    //     n += 8 as c_int;
    // }
    // n += ff_log2_tab[v as usize] as c_int;
    // return n;
}

pub(super) fn clip_uint8_c(mut a: c_int) -> c_uchar {
    a.clamp(c_uchar::MIN.into(), c_uchar::MAX.into()) as u8
}

pub(super) fn clip_uintp2_c(mut a: c_int, mut p: c_int) -> c_uint {
    if a & !(((1 as c_int) << p) - 1 as c_int) != 0 {
        (!a >> 31 as c_int & ((1 as c_int) << p) - 1 as c_int) as c_uint
    } else {
        a as c_uint
    }
}

/// Clear high bits from an unsigned integer starting with specific bit position.
pub(super) fn mod_uintp2_c(mut a: c_uint, mut p: c_uint) -> c_uint {
    a & ((1 as c_uint) << p).wrapping_sub(1 as c_int as c_uint)
}

#[inline]
pub(super) unsafe fn coef2maxsf(mut coef: c_float) -> c_uchar {
    clip_uint8_c((coef.log2() * 4. + 6. + 140. - 36.) as c_int)
}

#[inline(always)]
pub(super) unsafe fn lcg_random(mut previous_val: c_uint) -> c_int {
    #[repr(C)]
    union C2RustUnnamed_2 {
        u: c_uint,
        s: c_int,
    }

    let mut v: C2RustUnnamed_2 = C2RustUnnamed_2 {
        u: previous_val
            .wrapping_mul(1664525 as c_uint)
            .wrapping_add(1013904223 as c_int as c_uint),
    };
    v.s
}
