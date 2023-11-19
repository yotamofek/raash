#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use std::ptr;

use ilog::IntLog;

use crate::aacenc::ff_quantize_band_cost_cache_init;
use crate::common::*;
use crate::types::*;
use crate::{aacenc_is::*, aacenc_ltp::*, aacenc_pred::*, aacenc_tns::*, aactab::*};

pub(crate) type quantize_and_encode_band_func = Option<
    unsafe fn(
        *mut AACEncContext,
        *mut PutBitContext,
        *const libc::c_float,
        *mut libc::c_float,
        *const libc::c_float,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_float,
        libc::c_float,
        *mut libc::c_int,
        *mut libc::c_float,
    ) -> libc::c_float,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) union C2RustUnnamed_2 {
    pub(crate) u: libc::c_uint,
    pub(crate) s: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct TrellisBandCodingPath {
    pub(crate) prev_idx: libc::c_int,
    pub(crate) cost: libc::c_float,
    pub(crate) run: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct BandCodingPath {
    pub(crate) prev_idx: libc::c_int,
    pub(crate) cost: libc::c_float,
    pub(crate) run: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct TrellisPath {
    pub(crate) cost: libc::c_float,
    pub(crate) prev: libc::c_int,
}
#[inline]
unsafe fn ff_sqrf(mut a: libc::c_float) -> libc::c_float {
    a * a
}
#[inline(always)]
unsafe fn ff_log2_c(mut v: libc::c_uint) -> libc::c_int {
    // TODO: is this (the cast) correct??
    v.log2() as libc::c_int
    // let mut n: libc::c_int = 0 as libc::c_int;
    // if v & 0xffff0000 as libc::c_uint != 0 {
    //     v >>= 16 as libc::c_int;
    //     n += 16 as libc::c_int;
    // }
    // if v & 0xff00 as libc::c_int as libc::c_uint != 0 {
    //     v >>= 8 as libc::c_int;
    //     n += 8 as libc::c_int;
    // }
    // n += ff_log2_tab[v as usize] as libc::c_int;
    // return n;
}
#[inline(always)]
unsafe fn av_clip_c(
    mut a: libc::c_int,
    mut amin: libc::c_int,
    mut amax: libc::c_int,
) -> libc::c_int {
    if a < amin {
        amin
    } else if a > amax {
        return amax;
    } else {
        return a;
    }
}
#[inline(always)]
unsafe fn av_clip_uint8_c(mut a: libc::c_int) -> uint8_t {
    if a & !(0xff as libc::c_int) != 0 {
        (!a >> 31 as libc::c_int) as uint8_t
    } else {
        a as uint8_t
    }
}
#[inline(always)]
unsafe fn av_clip_uintp2_c(mut a: libc::c_int, mut p: libc::c_int) -> libc::c_uint {
    if a & !(((1 as libc::c_int) << p) - 1 as libc::c_int) != 0 {
        (!a >> 31 as libc::c_int & ((1 as libc::c_int) << p) - 1 as libc::c_int) as libc::c_uint
    } else {
        a as libc::c_uint
    }
}
#[inline(always)]
unsafe fn av_mod_uintp2_c(mut a: libc::c_uint, mut p: libc::c_uint) -> libc::c_uint {
    a & ((1 as libc::c_uint) << p).wrapping_sub(1 as libc::c_int as libc::c_uint)
}
#[inline(always)]
unsafe fn av_clipf_c(
    mut a: libc::c_float,
    mut amin: libc::c_float,
    mut amax: libc::c_float,
) -> libc::c_float {
    if (if a > amin { a } else { amin }) > amax {
        amax
    } else if a > amin {
        a
    } else {
        amin
    }
}
static mut BUF_BITS: libc::c_int = 0;
#[inline]
unsafe fn put_sbits(mut pb: *mut PutBitContext, mut n: libc::c_int, mut value: int32_t) {
    put_bits(
        pb,
        n,
        av_mod_uintp2_c(value as libc::c_uint, n as libc::c_uint),
    );
}
#[inline]
unsafe fn put_bits(mut s: *mut PutBitContext, mut n: libc::c_int, mut value: BitBuf) {
    put_bits_no_assert(s, n, value);
}
#[inline]
unsafe fn put_bits_no_assert(mut s: *mut PutBitContext, mut n: libc::c_int, mut value: BitBuf) {
    let mut bit_buf: BitBuf = 0;
    let mut bit_left: libc::c_int = 0;
    bit_buf = (*s).bit_buf;
    bit_left = (*s).bit_left;
    if n < bit_left {
        bit_buf = bit_buf << n | value;
        bit_left -= n;
    } else {
        bit_buf <<= bit_left;
        bit_buf |= value >> n - bit_left;
        if ((*s).buf_end).offset_from((*s).buf_ptr) as libc::c_long as libc::c_ulong
            >= ::core::mem::size_of::<BitBuf>() as libc::c_ulong
        {
            (*((*s).buf_ptr as *mut unaligned_32)).l = av_bswap32(bit_buf);
            (*s).buf_ptr =
                ((*s).buf_ptr).offset(::core::mem::size_of::<BitBuf>() as libc::c_ulong as isize);
        } else {
            panic!("Internal error, put_bits buffer too small");
        }
        bit_left += BUF_BITS - n;
        bit_buf = value;
    }
    (*s).bit_buf = bit_buf;
    (*s).bit_left = bit_left;
}
#[inline(always)]
unsafe fn av_bswap32(mut x: uint32_t) -> uint32_t {
    (x << 8 as libc::c_int & 0xff00 as libc::c_int as libc::c_uint
        | x >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        << 16 as libc::c_int
        | ((x >> 16 as libc::c_int) << 8 as libc::c_int & 0xff00 as libc::c_int as libc::c_uint
            | x >> 16 as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
}
static mut run_value_bits_long: [uint8_t; 64] = [
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
];
static mut run_value_bits_short: [uint8_t; 16] = [
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
];
static mut run_value_bits: [*const uint8_t; 2] =
    unsafe { [run_value_bits_long.as_ptr(), run_value_bits_short.as_ptr()] };
static mut aac_cb_out_map: [uint8_t; 15] = [
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
];
static mut aac_cb_in_map: [uint8_t; 16] = [
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
];
static mut aac_cb_range: [uint8_t; 12] = [
    0 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    17 as libc::c_int as uint8_t,
];
static mut aac_cb_maxval: [uint8_t; 12] = [
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    16 as libc::c_int as uint8_t,
];
static mut aac_maxval_cb: [libc::c_uchar; 14] = [
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
];
#[inline]
unsafe fn quant(mut coef: libc::c_float, Q: libc::c_float, rounding: libc::c_float) -> libc::c_int {
    let mut a: libc::c_float = coef * Q;
    (sqrtf(a * sqrtf(a)) + rounding) as libc::c_int
}
#[inline]
unsafe fn find_max_val(
    mut group_len: libc::c_int,
    mut swb_size: libc::c_int,
    mut scaled: *const libc::c_float,
) -> libc::c_float {
    let mut maxval: libc::c_float = 0.0f32;
    let mut w2: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    w2 = 0 as libc::c_int;
    while w2 < group_len {
        i = 0 as libc::c_int;
        while i < swb_size {
            maxval = if maxval > *scaled.offset((w2 * 128 as libc::c_int + i) as isize) {
                maxval
            } else {
                *scaled.offset((w2 * 128 as libc::c_int + i) as isize)
            };
            i += 1;
            i;
        }
        w2 += 1;
        w2;
    }
    maxval
}
#[inline]
unsafe fn find_min_book(mut maxval: libc::c_float, mut sf: libc::c_int) -> libc::c_int {
    let mut Q34: libc::c_float = ff_aac_pow34sf_tab
        [(200 as libc::c_int - sf + 140 as libc::c_int - 36 as libc::c_int) as usize];
    let mut qmaxval: libc::c_int = 0;
    let mut cb: libc::c_int = 0;
    qmaxval = (maxval * Q34 + 0.4054f32) as libc::c_int;
    if qmaxval as libc::c_ulong
        >= (::core::mem::size_of::<[libc::c_uchar; 14]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
    {
        cb = 11 as libc::c_int;
    } else {
        cb = aac_maxval_cb[qmaxval as usize] as libc::c_int;
    }
    cb
}
#[inline]
unsafe fn find_form_factor(
    mut group_len: libc::c_int,
    mut swb_size: libc::c_int,
    mut thresh: libc::c_float,
    mut scaled: *const libc::c_float,
    mut nzslope: libc::c_float,
) -> libc::c_float {
    let iswb_size: libc::c_float = 1.0f32 / swb_size as libc::c_float;
    let iswb_sizem1: libc::c_float = 1.0f32 / (swb_size - 1 as libc::c_int) as libc::c_float;
    let ethresh: libc::c_float = thresh;
    let mut form: libc::c_float = 0.0f32;
    let mut weight: libc::c_float = 0.0f32;
    let mut w2: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    w2 = 0 as libc::c_int;
    while w2 < group_len {
        let mut e: libc::c_float = 0.0f32;
        let mut e2: libc::c_float = 0.0f32;
        let mut var: libc::c_float = 0.0f32;
        let mut maxval: libc::c_float = 0.0f32;
        let mut nzl: libc::c_float = 0 as libc::c_int as libc::c_float;
        i = 0 as libc::c_int;
        while i < swb_size {
            let mut s: libc::c_float =
                fabsf(*scaled.offset((w2 * 128 as libc::c_int + i) as isize));
            maxval = if maxval > s { maxval } else { s };
            e += s;
            s *= s;
            e2 += s;
            if s >= ethresh {
                nzl += 1.0f32;
            } else if nzslope == 2.0f32 {
                nzl += s / ethresh * (s / ethresh);
            } else {
                nzl += ff_fast_powf(s / ethresh, nzslope);
            }
            i += 1;
            i;
        }
        if e2 > thresh {
            let mut frm: libc::c_float = 0.;
            e *= iswb_size;
            i = 0 as libc::c_int;
            while i < swb_size {
                let mut d: libc::c_float =
                    fabsf(*scaled.offset((w2 * 128 as libc::c_int + i) as isize)) - e;
                var += d * d;
                i += 1;
                i;
            }
            var = sqrtf(var * iswb_sizem1);
            e2 *= iswb_size;
            frm = e
                / (if e + 4 as libc::c_int as libc::c_float * var > maxval {
                    maxval
                } else {
                    e + 4 as libc::c_int as libc::c_float * var
                });
            form += e2 * sqrtf(frm) / (if 0.5f32 > nzl { 0.5f32 } else { nzl });
            weight += e2;
        }
        w2 += 1;
        w2;
    }
    if weight > 0 as libc::c_int as libc::c_float {
        form / weight
    } else {
        1.0f32
    }
}
#[inline]
unsafe fn coef2minsf(mut coef: libc::c_float) -> uint8_t {
    av_clip_uint8_c(
        (log2f(coef) * 4 as libc::c_int as libc::c_float - 69 as libc::c_int as libc::c_float
            + 140 as libc::c_int as libc::c_float
            - 36 as libc::c_int as libc::c_float) as libc::c_int,
    )
}
#[inline(always)]
unsafe fn ff_fast_powf(mut x: libc::c_float, mut y: libc::c_float) -> libc::c_float {
    expf(logf(x) * y)
}
#[inline(always)]
unsafe fn bval2bmax(mut b: libc::c_float) -> libc::c_float {
    0.001f32 + 0.0035f32 * (b * b * b) / (15.5f32 * 15.5f32 * 15.5f32)
}
#[inline]
unsafe fn ff_sfdelta_can_remove_band(
    mut sce: *const SingleChannelElement,
    mut nextband: *const uint8_t,
    mut prev_sf: libc::c_int,
    mut band: libc::c_int,
) -> libc::c_int {
    (prev_sf >= 0 as libc::c_int
        && (*sce).sf_idx[*nextband.offset(band as isize) as usize] >= prev_sf - 60 as libc::c_int
        && (*sce).sf_idx[*nextband.offset(band as isize) as usize] <= prev_sf + 60 as libc::c_int)
        as libc::c_int
}
#[inline]
unsafe fn coef2maxsf(mut coef: libc::c_float) -> uint8_t {
    av_clip_uint8_c(
        (log2f(coef) * 4 as libc::c_int as libc::c_float
            + 6 as libc::c_int as libc::c_float
            + 140 as libc::c_int as libc::c_float
            - 36 as libc::c_int as libc::c_float) as libc::c_int,
    )
}
#[inline(always)]
unsafe fn lcg_random(mut previous_val: libc::c_uint) -> libc::c_int {
    let mut v: C2RustUnnamed_2 = C2RustUnnamed_2 {
        u: previous_val
            .wrapping_mul(1664525 as libc::c_uint)
            .wrapping_add(1013904223 as libc::c_int as libc::c_uint),
    };
    v.s
}
#[inline]
unsafe fn ff_init_nextband_map(mut sce: *const SingleChannelElement, mut nextband: *mut uint8_t) {
    let mut prevband: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    let mut w: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    g = 0 as libc::c_int;
    while g < 128 as libc::c_int {
        *nextband.offset(g as isize) = g as uint8_t;
        g += 1;
        g;
    }
    w = 0 as libc::c_int;
    while w < (*sce).ics.num_windows {
        g = 0 as libc::c_int;
        while g < (*sce).ics.num_swb {
            if (*sce).zeroes[(w * 16 as libc::c_int + g) as usize] == 0
                && ((*sce).band_type[(w * 16 as libc::c_int + g) as usize] as libc::c_uint)
                    < RESERVED_BT as libc::c_int as libc::c_uint
            {
                let fresh0 = &mut (*nextband.offset(prevband as isize));
                *fresh0 = (w * 16 as libc::c_int + g) as uint8_t;
                prevband = *fresh0;
            }
            g += 1;
            g;
        }
        w += (*sce).ics.group_len[w as usize] as libc::c_int;
    }
    *nextband.offset(prevband as isize) = prevband;
}
#[inline]
unsafe fn ff_sfdelta_can_replace(
    mut sce: *const SingleChannelElement,
    mut nextband: *const uint8_t,
    mut prev_sf: libc::c_int,
    mut new_sf: libc::c_int,
    mut band: libc::c_int,
) -> libc::c_int {
    (new_sf >= prev_sf - 60 as libc::c_int
        && new_sf <= prev_sf + 60 as libc::c_int
        && (*sce).sf_idx[*nextband.offset(band as isize) as usize] >= new_sf - 60 as libc::c_int
        && (*sce).sf_idx[*nextband.offset(band as isize) as usize] <= new_sf + 60 as libc::c_int)
        as libc::c_int
}
#[inline]
unsafe fn quantize_band_cost_cached(
    mut s: *mut AACEncContext,
    mut w: libc::c_int,
    mut g: libc::c_int,
    mut in_0: *const libc::c_float,
    mut scaled: *const libc::c_float,
    mut size: libc::c_int,
    mut scale_idx: libc::c_int,
    mut cb: libc::c_int,
    lambda: libc::c_float,
    uplim: libc::c_float,
    mut bits: *mut libc::c_int,
    mut energy: *mut libc::c_float,
    mut rtz: libc::c_int,
) -> libc::c_float {
    let mut entry: *mut AACQuantizeBandCostCacheEntry =
        std::ptr::null_mut::<AACQuantizeBandCostCacheEntry>();
    entry = &mut *(*((*s).quantize_band_cost_cache)
        .as_mut_ptr()
        .offset(scale_idx as isize))
    .as_mut_ptr()
    .offset((w * 16 as libc::c_int + g) as isize) as *mut AACQuantizeBandCostCacheEntry;
    if (*entry).generation as libc::c_int != (*s).quantize_band_cost_cache_generation as libc::c_int
        || (*entry).cb as libc::c_int != cb
        || (*entry).rtz as libc::c_int != rtz
    {
        (*entry).rd = quantize_band_cost(
            s,
            in_0,
            scaled,
            size,
            scale_idx,
            cb,
            lambda,
            uplim,
            &mut (*entry).bits,
            &mut (*entry).energy,
        );
        (*entry).cb = cb as libc::c_char;
        (*entry).rtz = rtz as libc::c_char;
        (*entry).generation = (*s).quantize_band_cost_cache_generation;
    }
    if !bits.is_null() {
        *bits = (*entry).bits;
    }
    if !energy.is_null() {
        *energy = (*entry).energy;
    }
    (*entry).rd
}
#[inline]
unsafe fn quantize_band_cost(
    mut s: *mut AACEncContext,
    mut in_0: *const libc::c_float,
    mut scaled: *const libc::c_float,
    mut size: libc::c_int,
    mut scale_idx: libc::c_int,
    mut cb: libc::c_int,
    lambda: libc::c_float,
    uplim: libc::c_float,
    mut bits: *mut libc::c_int,
    mut energy: *mut libc::c_float,
) -> libc::c_float {
    ff_quantize_and_encode_band_cost(
        s,
        std::ptr::null_mut::<PutBitContext>(),
        in_0,
        std::ptr::null_mut::<libc::c_float>(),
        scaled,
        size,
        scale_idx,
        cb,
        lambda,
        uplim,
        bits,
        energy,
    )
}
#[inline]
unsafe fn quantize_band_cost_bits(
    mut s: *mut AACEncContext,
    mut in_0: *const libc::c_float,
    mut scaled: *const libc::c_float,
    mut size: libc::c_int,
    mut scale_idx: libc::c_int,
    mut cb: libc::c_int,
    _lambda: libc::c_float,
    uplim: libc::c_float,
    mut bits: *mut libc::c_int,
    mut energy: *mut libc::c_float,
) -> libc::c_int {
    let mut auxbits: libc::c_int = 0;
    ff_quantize_and_encode_band_cost(
        s,
        std::ptr::null_mut::<PutBitContext>(),
        in_0,
        std::ptr::null_mut::<libc::c_float>(),
        scaled,
        size,
        scale_idx,
        cb,
        0.0f32,
        uplim,
        &mut auxbits,
        energy,
    );
    if !bits.is_null() {
        *bits = auxbits;
    }
    auxbits
}
#[inline]
unsafe fn ff_pns_bits(
    mut sce: *mut SingleChannelElement,
    mut w: libc::c_int,
    mut g: libc::c_int,
) -> libc::c_int {
    if g == 0
        || (*sce).zeroes[(w * 16 as libc::c_int + g - 1 as libc::c_int) as usize] == 0
        || (*sce).can_pns[(w * 16 as libc::c_int + g - 1 as libc::c_int) as usize] == 0
    {
        9 as libc::c_int
    } else {
        5 as libc::c_int
    }
}
unsafe extern "C" fn search_for_quantizers_twoloop(
    mut avctx: *mut AVCodecContext,
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
    lambda: libc::c_float,
) {
    let mut start: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut w2: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut recomprd: libc::c_int = 0;
    let mut destbits: libc::c_int = ((*avctx).bit_rate as libc::c_double * 1024.0f64
        / (*avctx).sample_rate as libc::c_double
        / (if (*avctx).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
            2.0f32
        } else {
            (*avctx).ch_layout.nb_channels as libc::c_float
        }) as libc::c_double
        * (lambda / 120.0f32) as libc::c_double) as libc::c_int;
    let mut refbits: libc::c_int = destbits;
    let mut toomanybits: libc::c_int = 0;
    let mut toofewbits: libc::c_int = 0;
    let mut nzs: [libc::c_char; 128] = [0; 128];
    let mut nextband: [uint8_t; 128] = [0; 128];
    let mut maxsf: [libc::c_int; 128] = [0; 128];
    let mut minsf: [libc::c_int; 128] = [0; 128];
    let mut dists: [libc::c_float; 128] = [
        0 as libc::c_int as libc::c_float,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ];
    let mut qenergies: [libc::c_float; 128] = [
        0 as libc::c_int as libc::c_float,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ];
    let mut uplims: [libc::c_float; 128] = [0.; 128];
    let mut euplims: [libc::c_float; 128] = [0.; 128];
    let mut energies: [libc::c_float; 128] = [0.; 128];
    let mut maxvals: [libc::c_float; 128] = [0.; 128];
    let mut spread_thr_r: [libc::c_float; 128] = [0.; 128];
    let mut min_spread_thr_r: libc::c_float = 0.;
    let mut max_spread_thr_r: libc::c_float = 0.;
    let mut rdlambda: libc::c_float = av_clipf_c(2.0f32 * 120.0f32 / lambda, 0.0625f32, 16.0f32);
    let nzslope: libc::c_float = 1.5f32;
    let mut rdmin: libc::c_float = 0.03125f32;
    let mut rdmax: libc::c_float = 1.0f32;
    let mut sfoffs: libc::c_float = av_clipf_c(
        log2f(120.0f32 / lambda) * 4.0f32,
        -(5 as libc::c_int) as libc::c_float,
        10 as libc::c_int as libc::c_float,
    );
    let mut fflag: libc::c_int = 0;
    let mut minscaler: libc::c_int = 0;
    let mut maxscaler: libc::c_int = 0;
    let mut nminscaler: libc::c_int = 0;
    let mut its: libc::c_int = 0 as libc::c_int;
    let mut maxits: libc::c_int = 30 as libc::c_int;
    let mut allz: libc::c_int = 0 as libc::c_int;
    let mut tbits: libc::c_int = 0;
    let mut cutoff: libc::c_int = 1024 as libc::c_int;
    let mut pns_start_pos: libc::c_int = 0;
    let mut prev: libc::c_int = 0;
    let mut zeroscale: libc::c_float = 0.;
    if lambda > 120.0f32 {
        zeroscale = av_clipf_c(powf(120.0f32 / lambda, 0.25f32), 0.0625f32, 1.0f32);
    } else {
        zeroscale = 1.0f32;
    }
    if (*s).psy.bitres.alloc >= 0 as libc::c_int {
        destbits = ((*s).psy.bitres.alloc as libc::c_float
            * (lambda
                / (if (*avctx).global_quality != 0 {
                    (*avctx).global_quality
                } else {
                    120 as libc::c_int
                }) as libc::c_float)) as libc::c_int;
    }
    if (*avctx).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        if (*s).options.mid_side != 0
            && (*s).cur_type as libc::c_uint == TYPE_CPE as libc::c_int as libc::c_uint
        {
            destbits *= 2 as libc::c_int;
        }
        toomanybits = 5800 as libc::c_int;
        toofewbits = destbits / 16 as libc::c_int;
        sfoffs = ((*sce).ics.num_windows - 1 as libc::c_int) as libc::c_float;
        rdlambda = sqrtf(rdlambda);
        maxits *= 2 as libc::c_int;
    } else {
        toomanybits = destbits + destbits / 8 as libc::c_int;
        toofewbits = destbits - destbits / 8 as libc::c_int;
        sfoffs = 0 as libc::c_int as libc::c_float;
        rdlambda = sqrtf(rdlambda);
    }
    let mut wlen: libc::c_int = 1024 as libc::c_int / (*sce).ics.num_windows;
    let mut bandwidth: libc::c_int = 0;
    let mut rate_bandwidth_multiplier: libc::c_float = 1.5f32;
    let mut frame_bit_rate: libc::c_int = (if (*avctx).flags
        & (1 as libc::c_int) << 1 as libc::c_int
        != 0
    {
        refbits as libc::c_float * rate_bandwidth_multiplier * (*avctx).sample_rate as libc::c_float
            / 1024 as libc::c_int as libc::c_float
    } else {
        ((*avctx).bit_rate / (*avctx).ch_layout.nb_channels as libc::c_long) as libc::c_float
    }) as libc::c_int;
    if (*s).options.pns != 0 || (*s).options.intensity_stereo != 0 {
        frame_bit_rate = (frame_bit_rate as libc::c_float * 1.15f32) as libc::c_int;
    }
    if (*avctx).cutoff > 0 as libc::c_int {
        bandwidth = (*avctx).cutoff;
    } else {
        bandwidth = if 3000 as libc::c_int
            > (if frame_bit_rate != 0 {
                if (if (if (if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 3000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                {
                    3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 12000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
                {
                    12000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
                } else if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 3000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                {
                    3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 22000 as libc::c_int
                {
                    22000 as libc::c_int
                } else if (if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 3000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                {
                    3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 12000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
                {
                    12000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
                } else if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 3000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                {
                    3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > (*avctx).sample_rate / 2 as libc::c_int
                {
                    (*avctx).sample_rate / 2 as libc::c_int
                } else if (if (if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 3000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                {
                    3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 12000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
                {
                    12000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
                } else if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 3000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                {
                    3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 22000 as libc::c_int
                {
                    22000 as libc::c_int
                } else if (if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 3000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                {
                    3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 12000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
                {
                    12000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
                } else if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 3000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                {
                    3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }
            } else {
                (*avctx).sample_rate / 2 as libc::c_int
            }) {
            3000 as libc::c_int
        } else if frame_bit_rate != 0 {
            if (if (if (if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 3000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            {
                3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 12000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
            {
                12000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
            } else if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 3000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            {
                3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 22000 as libc::c_int
            {
                22000 as libc::c_int
            } else if (if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 3000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            {
                3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 12000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
            {
                12000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
            } else if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 3000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            {
                3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > (*avctx).sample_rate / 2 as libc::c_int
            {
                (*avctx).sample_rate / 2 as libc::c_int
            } else if (if (if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 3000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            {
                3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 12000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
            {
                12000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
            } else if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 3000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            {
                3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 22000 as libc::c_int
            {
                22000 as libc::c_int
            } else if (if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 3000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            {
                3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 12000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
            {
                12000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
            } else if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 3000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            {
                3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }
        } else {
            (*avctx).sample_rate / 2 as libc::c_int
        };
        (*s).psy.cutoff = bandwidth;
    }
    cutoff = bandwidth * 2 as libc::c_int * wlen / (*avctx).sample_rate;
    pns_start_pos = 4000 as libc::c_int * 2 as libc::c_int * wlen / (*avctx).sample_rate;
    destbits = if destbits > 5800 as libc::c_int {
        5800 as libc::c_int
    } else {
        destbits
    };
    toomanybits = if toomanybits > 5800 as libc::c_int {
        5800 as libc::c_int
    } else {
        toomanybits
    };
    toofewbits = if toofewbits > 5800 as libc::c_int {
        5800 as libc::c_int
    } else {
        toofewbits
    };
    min_spread_thr_r = -(1 as libc::c_int) as libc::c_float;
    max_spread_thr_r = -(1 as libc::c_int) as libc::c_float;
    w = 0 as libc::c_int;
    while w < (*sce).ics.num_windows {
        start = 0 as libc::c_int;
        g = start;
        while g < (*sce).ics.num_swb {
            let mut nz: libc::c_int = 0 as libc::c_int;
            let mut uplim: libc::c_float = 0.0f32;
            let mut energy: libc::c_float = 0.0f32;
            let mut spread: libc::c_float = 0.0f32;
            w2 = 0 as libc::c_int;
            while w2 < (*sce).ics.group_len[w as usize] as libc::c_int {
                let mut band: *mut FFPsyBand =
                    &mut *((*((*s).psy.ch).offset((*s).cur_channel as isize)).psy_bands)
                        .as_mut_ptr()
                        .offset(((w + w2) * 16 as libc::c_int + g) as isize)
                        as *mut FFPsyBand;
                if start >= cutoff
                    || (*band).energy <= (*band).threshold * zeroscale
                    || (*band).threshold == 0.0f32
                {
                    (*sce).zeroes[((w + w2) * 16 as libc::c_int + g) as usize] =
                        1 as libc::c_int as uint8_t;
                } else {
                    nz = 1 as libc::c_int;
                }
                w2 += 1;
                w2;
            }
            if nz == 0 {
                uplim = 0.0f32;
            } else {
                nz = 0 as libc::c_int;
                w2 = 0 as libc::c_int;
                while w2 < (*sce).ics.group_len[w as usize] as libc::c_int {
                    let mut band_0: *mut FFPsyBand =
                        &mut *((*((*s).psy.ch).offset((*s).cur_channel as isize)).psy_bands)
                            .as_mut_ptr()
                            .offset(((w + w2) * 16 as libc::c_int + g) as isize)
                            as *mut FFPsyBand;
                    if !((*band_0).energy <= (*band_0).threshold * zeroscale
                        || (*band_0).threshold == 0.0f32)
                    {
                        uplim += (*band_0).threshold;
                        energy += (*band_0).energy;
                        spread += (*band_0).spread;
                        nz += 1;
                        nz;
                    }
                    w2 += 1;
                    w2;
                }
            }
            uplims[(w * 16 as libc::c_int + g) as usize] = uplim;
            energies[(w * 16 as libc::c_int + g) as usize] = energy;
            nzs[(w * 16 as libc::c_int + g) as usize] = nz as libc::c_char;
            (*sce).zeroes[(w * 16 as libc::c_int + g) as usize] =
                (nz == 0) as libc::c_int as uint8_t;
            allz |= nz;
            if nz != 0 && (*sce).can_pns[(w * 16 as libc::c_int + g) as usize] as libc::c_int != 0 {
                spread_thr_r[(w * 16 as libc::c_int + g) as usize] =
                    energy * nz as libc::c_float / (uplim * spread);
                if min_spread_thr_r < 0 as libc::c_int as libc::c_float {
                    max_spread_thr_r = spread_thr_r[(w * 16 as libc::c_int + g) as usize];
                    min_spread_thr_r = max_spread_thr_r;
                } else {
                    min_spread_thr_r =
                        if min_spread_thr_r > spread_thr_r[(w * 16 as libc::c_int + g) as usize] {
                            spread_thr_r[(w * 16 as libc::c_int + g) as usize]
                        } else {
                            min_spread_thr_r
                        };
                    max_spread_thr_r =
                        if max_spread_thr_r > spread_thr_r[(w * 16 as libc::c_int + g) as usize] {
                            max_spread_thr_r
                        } else {
                            spread_thr_r[(w * 16 as libc::c_int + g) as usize]
                        };
                }
            }
            let fresh1 = g;
            g += 1;
            start += *((*sce).ics.swb_sizes).offset(fresh1 as isize) as libc::c_int;
        }
        w += (*sce).ics.group_len[w as usize] as libc::c_int;
    }
    minscaler = 65535 as libc::c_int;
    w = 0 as libc::c_int;
    while w < (*sce).ics.num_windows {
        g = 0 as libc::c_int;
        while g < (*sce).ics.num_swb {
            if (*sce).zeroes[(w * 16 as libc::c_int + g) as usize] != 0 {
                (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize] = 140 as libc::c_int;
            } else {
                (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize] = av_clip_c(
                    (140 as libc::c_int as libc::c_double
                        + 1.75f64
                            * log2f(
                                (if 0.00125f32 > uplims[(w * 16 as libc::c_int + g) as usize] {
                                    0.00125f32
                                } else {
                                    uplims[(w * 16 as libc::c_int + g) as usize]
                                }) / *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int
                                    as libc::c_float,
                            ) as libc::c_double
                        + sfoffs as libc::c_double) as libc::c_int,
                    60 as libc::c_int,
                    255 as libc::c_int,
                );
                minscaler = if minscaler > (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize] {
                    (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize]
                } else {
                    minscaler
                };
            }
            g += 1;
            g;
        }
        w += (*sce).ics.group_len[w as usize] as libc::c_int;
    }
    minscaler = av_clip_c(
        minscaler,
        140 as libc::c_int - 36 as libc::c_int,
        255 as libc::c_int - 36 as libc::c_int,
    );
    w = 0 as libc::c_int;
    while w < (*sce).ics.num_windows {
        g = 0 as libc::c_int;
        while g < (*sce).ics.num_swb {
            if (*sce).zeroes[(w * 16 as libc::c_int + g) as usize] == 0 {
                (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize] = av_clip_c(
                    (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize],
                    minscaler,
                    minscaler + 60 as libc::c_int - 1 as libc::c_int,
                );
            }
            g += 1;
            g;
        }
        w += (*sce).ics.group_len[w as usize] as libc::c_int;
    }
    if allz == 0 {
        return;
    }
    ((*s).abs_pow34).expect("non-null function pointer")(
        ((*s).scoefs).as_mut_ptr(),
        ((*sce).coeffs).as_mut_ptr(),
        1024 as libc::c_int,
    );
    ff_quantize_band_cost_cache_init(s);
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[libc::c_int; 128]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
    {
        minsf[i as usize] = 0 as libc::c_int;
        i += 1;
        i;
    }
    w = 0 as libc::c_int;
    while w < (*sce).ics.num_windows {
        start = w * 128 as libc::c_int;
        g = 0 as libc::c_int;
        while g < (*sce).ics.num_swb {
            let mut scaled: *const libc::c_float =
                ((*s).scoefs).as_mut_ptr().offset(start as isize);
            let mut minsfidx: libc::c_int = 0;
            maxvals[(w * 16 as libc::c_int + g) as usize] = find_max_val(
                (*sce).ics.group_len[w as usize] as libc::c_int,
                *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int,
                scaled,
            );
            if maxvals[(w * 16 as libc::c_int + g) as usize] > 0 as libc::c_int as libc::c_float {
                minsfidx = coef2minsf(maxvals[(w * 16 as libc::c_int + g) as usize]) as libc::c_int;
                w2 = 0 as libc::c_int;
                while w2 < (*sce).ics.group_len[w as usize] as libc::c_int {
                    minsf[((w + w2) * 16 as libc::c_int + g) as usize] = minsfidx;
                    w2 += 1;
                    w2;
                }
            }
            start += *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int;
            g += 1;
            g;
        }
        w += (*sce).ics.group_len[w as usize] as libc::c_int;
    }
    euplims = uplims;
    w = 0 as libc::c_int;
    while w < (*sce).ics.num_windows {
        let mut de_psy_factor: libc::c_float = if (*sce).ics.num_windows > 1 as libc::c_int {
            8.0f32 / (*sce).ics.group_len[w as usize] as libc::c_int as libc::c_float
        } else {
            1.0f32
        };
        start = w * 128 as libc::c_int;
        g = 0 as libc::c_int;
        while g < (*sce).ics.num_swb {
            if nzs[g as usize] as libc::c_int > 0 as libc::c_int {
                let mut cleanup_factor: libc::c_float = ff_sqrf(av_clipf_c(
                    start as libc::c_float / (cutoff as libc::c_float * 0.75f32),
                    1.0f32,
                    2.0f32,
                ));
                let mut energy2uplim: libc::c_float = find_form_factor(
                    (*sce).ics.group_len[w as usize] as libc::c_int,
                    *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int,
                    uplims[(w * 16 as libc::c_int + g) as usize]
                        / (nzs[g as usize] as libc::c_int
                            * *((*sce).ics.swb_sizes).offset(w as isize) as libc::c_int)
                            as libc::c_float,
                    ((*sce).coeffs).as_mut_ptr().offset(start as isize),
                    nzslope * cleanup_factor,
                );
                energy2uplim *= de_psy_factor;
                if (*avctx).flags & (1 as libc::c_int) << 1 as libc::c_int == 0 {
                    energy2uplim = sqrtf(energy2uplim);
                }
                energy2uplim = if 0.015625f32
                    > (if 1.0f32 > energy2uplim {
                        energy2uplim
                    } else {
                        1.0f32
                    }) {
                    0.015625f32
                } else if 1.0f32 > energy2uplim {
                    energy2uplim
                } else {
                    1.0f32
                };
                uplims[(w * 16 as libc::c_int + g) as usize] *=
                    av_clipf_c(rdlambda * energy2uplim, rdmin, rdmax)
                        * (*sce).ics.group_len[w as usize] as libc::c_int as libc::c_float;
                energy2uplim = find_form_factor(
                    (*sce).ics.group_len[w as usize] as libc::c_int,
                    *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int,
                    uplims[(w * 16 as libc::c_int + g) as usize]
                        / (nzs[g as usize] as libc::c_int
                            * *((*sce).ics.swb_sizes).offset(w as isize) as libc::c_int)
                            as libc::c_float,
                    ((*sce).coeffs).as_mut_ptr().offset(start as isize),
                    2.0f32,
                );
                energy2uplim *= de_psy_factor;
                if (*avctx).flags & (1 as libc::c_int) << 1 as libc::c_int == 0 {
                    energy2uplim = sqrtf(energy2uplim);
                }
                energy2uplim = if 0.015625f32
                    > (if 1.0f32 > energy2uplim {
                        energy2uplim
                    } else {
                        1.0f32
                    }) {
                    0.015625f32
                } else if 1.0f32 > energy2uplim {
                    energy2uplim
                } else {
                    1.0f32
                };
                euplims[(w * 16 as libc::c_int + g) as usize] *= av_clipf_c(
                    rdlambda
                        * energy2uplim
                        * (*sce).ics.group_len[w as usize] as libc::c_int as libc::c_float,
                    0.5f32,
                    1.0f32,
                );
            }
            start += *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int;
            g += 1;
            g;
        }
        w += (*sce).ics.group_len[w as usize] as libc::c_int;
    }
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[libc::c_int; 128]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
    {
        maxsf[i as usize] = 255 as libc::c_int;
        i += 1;
        i;
    }
    loop {
        let mut overdist: libc::c_int = 0;
        let mut qstep: libc::c_int = if its != 0 {
            1 as libc::c_int
        } else {
            32 as libc::c_int
        };
        loop {
            let mut changed: libc::c_int = 0 as libc::c_int;
            prev = -(1 as libc::c_int);
            recomprd = 0 as libc::c_int;
            tbits = 0 as libc::c_int;
            w = 0 as libc::c_int;
            while w < (*sce).ics.num_windows {
                start = w * 128 as libc::c_int;
                g = 0 as libc::c_int;
                while g < (*sce).ics.num_swb {
                    let mut coefs: *const libc::c_float =
                        &mut *((*sce).coeffs).as_mut_ptr().offset(start as isize) as *mut INTFLOAT;
                    let mut scaled_0: *const libc::c_float =
                        &mut *((*s).scoefs).as_mut_ptr().offset(start as isize)
                            as *mut libc::c_float;
                    let mut bits: libc::c_int = 0 as libc::c_int;
                    let mut cb: libc::c_int = 0;
                    let mut dist: libc::c_float = 0.0f32;
                    let mut qenergy: libc::c_float = 0.0f32;
                    if (*sce).zeroes[(w * 16 as libc::c_int + g) as usize] as libc::c_int != 0
                        || (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize] >= 218 as libc::c_int
                    {
                        start += *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int;
                        if (*sce).can_pns[(w * 16 as libc::c_int + g) as usize] != 0 {
                            tbits += ff_pns_bits(sce, w, g);
                        }
                    } else {
                        cb = find_min_book(
                            maxvals[(w * 16 as libc::c_int + g) as usize],
                            (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize],
                        );
                        w2 = 0 as libc::c_int;
                        while w2 < (*sce).ics.group_len[w as usize] as libc::c_int {
                            let mut b: libc::c_int = 0;
                            let mut sqenergy: libc::c_float = 0.;
                            dist += quantize_band_cost_cached(
                                s,
                                w + w2,
                                g,
                                coefs.offset((w2 * 128 as libc::c_int) as isize),
                                scaled_0.offset((w2 * 128 as libc::c_int) as isize),
                                *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int,
                                (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize],
                                cb,
                                1.0f32,
                                ::core::f32::INFINITY,
                                &mut b,
                                &mut sqenergy,
                                0 as libc::c_int,
                            );
                            bits += b;
                            qenergy += sqenergy;
                            w2 += 1;
                            w2;
                        }
                        dists[(w * 16 as libc::c_int + g) as usize] = dist - bits as libc::c_float;
                        qenergies[(w * 16 as libc::c_int + g) as usize] = qenergy;
                        if prev != -(1 as libc::c_int) {
                            let mut sfdiff: libc::c_int = av_clip_c(
                                (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize] - prev
                                    + 60 as libc::c_int,
                                0 as libc::c_int,
                                2 as libc::c_int * 60 as libc::c_int,
                            );
                            bits += ff_aac_scalefactor_bits[sfdiff as usize] as libc::c_int;
                        }
                        tbits += bits;
                        start += *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int;
                        prev = (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize];
                    }
                    g += 1;
                    g;
                }
                w += (*sce).ics.group_len[w as usize] as libc::c_int;
            }
            if tbits > toomanybits {
                recomprd = 1 as libc::c_int;
                i = 0 as libc::c_int;
                while i < 128 as libc::c_int {
                    if (*sce).sf_idx[i as usize] < 255 as libc::c_int - 36 as libc::c_int {
                        let mut maxsf_i: libc::c_int = if tbits > 5800 as libc::c_int {
                            255 as libc::c_int
                        } else {
                            maxsf[i as usize]
                        };
                        let mut new_sf: libc::c_int = if maxsf_i > (*sce).sf_idx[i as usize] + qstep
                        {
                            (*sce).sf_idx[i as usize] + qstep
                        } else {
                            maxsf_i
                        };
                        if new_sf != (*sce).sf_idx[i as usize] {
                            (*sce).sf_idx[i as usize] = new_sf;
                            changed = 1 as libc::c_int;
                        }
                    }
                    i += 1;
                    i;
                }
            } else if tbits < toofewbits {
                recomprd = 1 as libc::c_int;
                i = 0 as libc::c_int;
                while i < 128 as libc::c_int {
                    if (*sce).sf_idx[i as usize] > 140 as libc::c_int {
                        let mut new_sf_0: libc::c_int =
                            if (if minsf[i as usize] > 140 as libc::c_int {
                                minsf[i as usize]
                            } else {
                                140 as libc::c_int
                            }) > (*sce).sf_idx[i as usize] - qstep
                            {
                                if minsf[i as usize] > 140 as libc::c_int {
                                    minsf[i as usize]
                                } else {
                                    140 as libc::c_int
                                }
                            } else {
                                (*sce).sf_idx[i as usize] - qstep
                            };
                        if new_sf_0 != (*sce).sf_idx[i as usize] {
                            (*sce).sf_idx[i as usize] = new_sf_0;
                            changed = 1 as libc::c_int;
                        }
                    }
                    i += 1;
                    i;
                }
            }
            qstep >>= 1 as libc::c_int;
            if qstep == 0
                && tbits > toomanybits
                && (*sce).sf_idx[0 as libc::c_int as usize] < 217 as libc::c_int
                && changed != 0
            {
                qstep = 1 as libc::c_int;
            }
            if qstep == 0 {
                break;
            }
        }
        overdist = 1 as libc::c_int;
        fflag = (tbits < toofewbits) as libc::c_int;
        i = 0 as libc::c_int;
        while i < 2 as libc::c_int && (overdist != 0 || recomprd != 0) {
            if recomprd != 0 {
                prev = -(1 as libc::c_int);
                tbits = 0 as libc::c_int;
                w = 0 as libc::c_int;
                while w < (*sce).ics.num_windows {
                    start = w * 128 as libc::c_int;
                    g = 0 as libc::c_int;
                    while g < (*sce).ics.num_swb {
                        let mut coefs_0: *const libc::c_float =
                            ((*sce).coeffs).as_mut_ptr().offset(start as isize);
                        let mut scaled_1: *const libc::c_float =
                            ((*s).scoefs).as_mut_ptr().offset(start as isize);
                        let mut bits_0: libc::c_int = 0 as libc::c_int;
                        let mut cb_0: libc::c_int = 0;
                        let mut dist_0: libc::c_float = 0.0f32;
                        let mut qenergy_0: libc::c_float = 0.0f32;
                        if (*sce).zeroes[(w * 16 as libc::c_int + g) as usize] as libc::c_int != 0
                            || (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize]
                                >= 218 as libc::c_int
                        {
                            start += *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int;
                            if (*sce).can_pns[(w * 16 as libc::c_int + g) as usize] != 0 {
                                tbits += ff_pns_bits(sce, w, g);
                            }
                        } else {
                            cb_0 = find_min_book(
                                maxvals[(w * 16 as libc::c_int + g) as usize],
                                (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize],
                            );
                            w2 = 0 as libc::c_int;
                            while w2 < (*sce).ics.group_len[w as usize] as libc::c_int {
                                let mut b_0: libc::c_int = 0;
                                let mut sqenergy_0: libc::c_float = 0.;
                                dist_0 += quantize_band_cost_cached(
                                    s,
                                    w + w2,
                                    g,
                                    coefs_0.offset((w2 * 128 as libc::c_int) as isize),
                                    scaled_1.offset((w2 * 128 as libc::c_int) as isize),
                                    *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int,
                                    (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize],
                                    cb_0,
                                    1.0f32,
                                    ::core::f32::INFINITY,
                                    &mut b_0,
                                    &mut sqenergy_0,
                                    0 as libc::c_int,
                                );
                                bits_0 += b_0;
                                qenergy_0 += sqenergy_0;
                                w2 += 1;
                                w2;
                            }
                            dists[(w * 16 as libc::c_int + g) as usize] =
                                dist_0 - bits_0 as libc::c_float;
                            qenergies[(w * 16 as libc::c_int + g) as usize] = qenergy_0;
                            if prev != -(1 as libc::c_int) {
                                let mut sfdiff_0: libc::c_int = av_clip_c(
                                    (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize] - prev
                                        + 60 as libc::c_int,
                                    0 as libc::c_int,
                                    2 as libc::c_int * 60 as libc::c_int,
                                );
                                bits_0 += ff_aac_scalefactor_bits[sfdiff_0 as usize] as libc::c_int;
                            }
                            tbits += bits_0;
                            start += *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int;
                            prev = (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize];
                        }
                        g += 1;
                        g;
                    }
                    w += (*sce).ics.group_len[w as usize] as libc::c_int;
                }
            }
            if i == 0
                && (*s).options.pns != 0
                && its > maxits / 2 as libc::c_int
                && tbits > toofewbits
            {
                let mut maxoverdist: libc::c_float = 0.0f32;
                let mut ovrfactor: libc::c_float =
                    1.0f32 + (maxits - its) as libc::c_float * 16.0f32 / maxits as libc::c_float;
                recomprd = 0 as libc::c_int;
                overdist = recomprd;
                w = 0 as libc::c_int;
                while w < (*sce).ics.num_windows {
                    start = 0 as libc::c_int;
                    g = start;
                    while g < (*sce).ics.num_swb {
                        if (*sce).zeroes[(w * 16 as libc::c_int + g) as usize] == 0
                            && (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize]
                                > 140 as libc::c_int
                            && dists[(w * 16 as libc::c_int + g) as usize]
                                > uplims[(w * 16 as libc::c_int + g) as usize] * ovrfactor
                        {
                            let mut ovrdist: libc::c_float = dists
                                [(w * 16 as libc::c_int + g) as usize]
                                / (if uplims[(w * 16 as libc::c_int + g) as usize]
                                    > euplims[(w * 16 as libc::c_int + g) as usize]
                                {
                                    uplims[(w * 16 as libc::c_int + g) as usize]
                                } else {
                                    euplims[(w * 16 as libc::c_int + g) as usize]
                                });
                            maxoverdist = if maxoverdist > ovrdist {
                                maxoverdist
                            } else {
                                ovrdist
                            };
                            overdist += 1;
                            overdist;
                        }
                        let fresh2 = g;
                        g += 1;
                        start += *((*sce).ics.swb_sizes).offset(fresh2 as isize) as libc::c_int;
                    }
                    w += (*sce).ics.group_len[w as usize] as libc::c_int;
                }
                if overdist != 0 {
                    let mut minspread: libc::c_float = max_spread_thr_r;
                    let mut maxspread: libc::c_float = min_spread_thr_r;
                    let mut zspread: libc::c_float = 0.;
                    let mut zeroable: libc::c_int = 0 as libc::c_int;
                    let mut zeroed: libc::c_int = 0 as libc::c_int;
                    let mut maxzeroed: libc::c_int = 0;
                    let mut zloop: libc::c_int = 0;
                    w = 0 as libc::c_int;
                    while w < (*sce).ics.num_windows {
                        start = 0 as libc::c_int;
                        g = start;
                        while g < (*sce).ics.num_swb {
                            if start >= pns_start_pos
                                && (*sce).zeroes[(w * 16 as libc::c_int + g) as usize] == 0
                                && (*sce).can_pns[(w * 16 as libc::c_int + g) as usize]
                                    as libc::c_int
                                    != 0
                            {
                                minspread = if minspread
                                    > spread_thr_r[(w * 16 as libc::c_int + g) as usize]
                                {
                                    spread_thr_r[(w * 16 as libc::c_int + g) as usize]
                                } else {
                                    minspread
                                };
                                maxspread = if maxspread
                                    > spread_thr_r[(w * 16 as libc::c_int + g) as usize]
                                {
                                    maxspread
                                } else {
                                    spread_thr_r[(w * 16 as libc::c_int + g) as usize]
                                };
                                zeroable += 1;
                                zeroable;
                            }
                            let fresh3 = g;
                            g += 1;
                            start += *((*sce).ics.swb_sizes).offset(fresh3 as isize) as libc::c_int;
                        }
                        w += (*sce).ics.group_len[w as usize] as libc::c_int;
                    }
                    zspread = (maxspread - minspread) * 0.0125f32 + minspread;
                    zspread = if (if min_spread_thr_r * 8.0f32 > zspread {
                        zspread
                    } else {
                        min_spread_thr_r * 8.0f32
                    }) > ((toomanybits - tbits) as libc::c_float * min_spread_thr_r
                        + (tbits - toofewbits) as libc::c_float * max_spread_thr_r)
                        / (toomanybits - toofewbits + 1 as libc::c_int) as libc::c_float
                    {
                        ((toomanybits - tbits) as libc::c_float * min_spread_thr_r
                            + (tbits - toofewbits) as libc::c_float * max_spread_thr_r)
                            / (toomanybits - toofewbits + 1 as libc::c_int) as libc::c_float
                    } else if min_spread_thr_r * 8.0f32 > zspread {
                        zspread
                    } else {
                        min_spread_thr_r * 8.0f32
                    };
                    maxzeroed = if zeroable
                        > (if 1 as libc::c_int
                            > (zeroable * its + maxits - 1 as libc::c_int)
                                / (2 as libc::c_int * maxits)
                        {
                            1 as libc::c_int
                        } else {
                            (zeroable * its + maxits - 1 as libc::c_int)
                                / (2 as libc::c_int * maxits)
                        }) {
                        if 1 as libc::c_int
                            > (zeroable * its + maxits - 1 as libc::c_int)
                                / (2 as libc::c_int * maxits)
                        {
                            1 as libc::c_int
                        } else {
                            (zeroable * its + maxits - 1 as libc::c_int)
                                / (2 as libc::c_int * maxits)
                        }
                    } else {
                        zeroable
                    };
                    zloop = 0 as libc::c_int;
                    while zloop < 2 as libc::c_int {
                        let mut loopovrfactor: libc::c_float =
                            if zloop != 0 { 1.0f32 } else { ovrfactor };
                        let mut loopminsf: libc::c_int = if zloop != 0 {
                            140 as libc::c_int - 36 as libc::c_int
                        } else {
                            140 as libc::c_int
                        };
                        let mut mcb: libc::c_int = 0;
                        g = (*sce).ics.num_swb - 1 as libc::c_int;
                        while g > 0 as libc::c_int && zeroed < maxzeroed {
                            if (*((*sce).ics.swb_offset).offset(g as isize) as libc::c_int)
                                >= pns_start_pos
                            {
                                w = 0 as libc::c_int;
                                while w < (*sce).ics.num_windows {
                                    if (*sce).zeroes[(w * 16 as libc::c_int + g) as usize] == 0
                                        && (*sce).can_pns[(w * 16 as libc::c_int + g) as usize]
                                            as libc::c_int
                                            != 0
                                        && spread_thr_r[(w * 16 as libc::c_int + g) as usize]
                                            <= zspread
                                        && (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize]
                                            > loopminsf
                                        && (dists[(w * 16 as libc::c_int + g) as usize]
                                            > loopovrfactor
                                                * uplims[(w * 16 as libc::c_int + g) as usize]
                                            || {
                                                mcb = find_min_book(
                                                    maxvals[(w * 16 as libc::c_int + g) as usize],
                                                    (*sce).sf_idx
                                                        [(w * 16 as libc::c_int + g) as usize],
                                                );
                                                mcb == 0
                                            }
                                            || mcb <= 1 as libc::c_int
                                                && dists[(w * 16 as libc::c_int + g) as usize]
                                                    > (if uplims
                                                        [(w * 16 as libc::c_int + g) as usize]
                                                        > euplims
                                                            [(w * 16 as libc::c_int + g) as usize]
                                                    {
                                                        euplims
                                                            [(w * 16 as libc::c_int + g) as usize]
                                                    } else {
                                                        uplims[(w * 16 as libc::c_int + g) as usize]
                                                    }))
                                    {
                                        (*sce).zeroes[(w * 16 as libc::c_int + g) as usize] =
                                            1 as libc::c_int as uint8_t;
                                        (*sce).band_type[(w * 16 as libc::c_int + g) as usize] =
                                            ZERO_BT;
                                        zeroed += 1;
                                        zeroed;
                                    }
                                    w += (*sce).ics.group_len[w as usize] as libc::c_int;
                                }
                            }
                            g -= 1;
                            g;
                        }
                        zloop += 1;
                        zloop;
                    }
                    if zeroed != 0 {
                        fflag = 1 as libc::c_int;
                        recomprd = fflag;
                    }
                } else {
                    overdist = 0 as libc::c_int;
                }
            }
            i += 1;
            i;
        }
        minscaler = 255 as libc::c_int;
        maxscaler = 0 as libc::c_int;
        w = 0 as libc::c_int;
        while w < (*sce).ics.num_windows {
            g = 0 as libc::c_int;
            while g < (*sce).ics.num_swb {
                if (*sce).zeroes[(w * 16 as libc::c_int + g) as usize] == 0 {
                    minscaler = if minscaler > (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize] {
                        (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize]
                    } else {
                        minscaler
                    };
                    maxscaler = if maxscaler > (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize] {
                        maxscaler
                    } else {
                        (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize]
                    };
                }
                g += 1;
                g;
            }
            w += (*sce).ics.group_len[w as usize] as libc::c_int;
        }
        nminscaler = av_clip_c(
            minscaler,
            140 as libc::c_int - 36 as libc::c_int,
            255 as libc::c_int - 36 as libc::c_int,
        );
        minscaler = nminscaler;
        prev = -(1 as libc::c_int);
        w = 0 as libc::c_int;
        while w < (*sce).ics.num_windows {
            let mut depth: libc::c_int = if its > maxits / 2 as libc::c_int {
                if its > maxits * 2 as libc::c_int / 3 as libc::c_int {
                    1 as libc::c_int
                } else {
                    3 as libc::c_int
                }
            } else {
                10 as libc::c_int
            };
            let mut edepth: libc::c_int = depth + 2 as libc::c_int;
            let mut uplmax: libc::c_float =
                its as libc::c_float / (maxits as libc::c_float * 0.25f32) + 1.0f32;
            uplmax *= if tbits > destbits {
                if 2.0f32
                    > tbits as libc::c_float
                        / (if 1 as libc::c_int > destbits {
                            1 as libc::c_int
                        } else {
                            destbits
                        }) as libc::c_float
                {
                    tbits as libc::c_float
                        / (if 1 as libc::c_int > destbits {
                            1 as libc::c_int
                        } else {
                            destbits
                        }) as libc::c_float
                } else {
                    2.0f32
                }
            } else {
                1.0f32
            };
            start = w * 128 as libc::c_int;
            g = 0 as libc::c_int;
            while g < (*sce).ics.num_swb {
                let mut prevsc: libc::c_int = (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize];
                if prev < 0 as libc::c_int
                    && (*sce).zeroes[(w * 16 as libc::c_int + g) as usize] == 0
                {
                    prev = (*sce).sf_idx[0 as libc::c_int as usize];
                }
                if (*sce).zeroes[(w * 16 as libc::c_int + g) as usize] == 0 {
                    let mut coefs_1: *const libc::c_float =
                        ((*sce).coeffs).as_mut_ptr().offset(start as isize);
                    let mut scaled_2: *const libc::c_float =
                        ((*s).scoefs).as_mut_ptr().offset(start as isize);
                    let mut cmb: libc::c_int = find_min_book(
                        maxvals[(w * 16 as libc::c_int + g) as usize],
                        (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize],
                    );
                    let mut mindeltasf: libc::c_int = if 0 as libc::c_int > prev - 60 as libc::c_int
                    {
                        0 as libc::c_int
                    } else {
                        prev - 60 as libc::c_int
                    };
                    let mut maxdeltasf: libc::c_int =
                        if 255 as libc::c_int - 36 as libc::c_int > prev + 60 as libc::c_int {
                            prev + 60 as libc::c_int
                        } else {
                            255 as libc::c_int - 36 as libc::c_int
                        };
                    if (cmb == 0
                        || dists[(w * 16 as libc::c_int + g) as usize]
                            > uplims[(w * 16 as libc::c_int + g) as usize])
                        && (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize]
                            > (if mindeltasf > minsf[(w * 16 as libc::c_int + g) as usize] {
                                mindeltasf
                            } else {
                                minsf[(w * 16 as libc::c_int + g) as usize]
                            })
                    {
                        i = 0 as libc::c_int;
                        while i < edepth
                            && (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize] > mindeltasf
                        {
                            let mut cb_1: libc::c_int = 0;
                            let mut bits_1: libc::c_int = 0;
                            let mut dist_1: libc::c_float = 0.;
                            let mut qenergy_1: libc::c_float = 0.;
                            let mut mb: libc::c_int = find_min_book(
                                maxvals[(w * 16 as libc::c_int + g) as usize],
                                (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize]
                                    - 1 as libc::c_int,
                            );
                            cb_1 = find_min_book(
                                maxvals[(w * 16 as libc::c_int + g) as usize],
                                (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize],
                            );
                            qenergy_1 = 0.0f32;
                            dist_1 = qenergy_1;
                            bits_1 = 0 as libc::c_int;
                            if cb_1 == 0 {
                                maxsf[(w * 16 as libc::c_int + g) as usize] = if (*sce).sf_idx
                                    [(w * 16 as libc::c_int + g) as usize]
                                    - 1 as libc::c_int
                                    > maxsf[(w * 16 as libc::c_int + g) as usize]
                                {
                                    maxsf[(w * 16 as libc::c_int + g) as usize]
                                } else {
                                    (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize]
                                        - 1 as libc::c_int
                                };
                            } else if i >= depth
                                && dists[(w * 16 as libc::c_int + g) as usize]
                                    < euplims[(w * 16 as libc::c_int + g) as usize]
                            {
                                break;
                            }
                            if g == 0
                                && (*sce).ics.num_windows > 1 as libc::c_int
                                && dists[(w * 16 as libc::c_int + g) as usize]
                                    >= euplims[(w * 16 as libc::c_int + g) as usize]
                            {
                                maxsf[(w * 16 as libc::c_int + g) as usize] = if (*sce).sf_idx
                                    [(w * 16 as libc::c_int + g) as usize]
                                    > maxsf[(w * 16 as libc::c_int + g) as usize]
                                {
                                    maxsf[(w * 16 as libc::c_int + g) as usize]
                                } else {
                                    (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize]
                                };
                            }
                            w2 = 0 as libc::c_int;
                            while w2 < (*sce).ics.group_len[w as usize] as libc::c_int {
                                let mut b_1: libc::c_int = 0;
                                let mut sqenergy_1: libc::c_float = 0.;
                                dist_1 += quantize_band_cost_cached(
                                    s,
                                    w + w2,
                                    g,
                                    coefs_1.offset((w2 * 128 as libc::c_int) as isize),
                                    scaled_2.offset((w2 * 128 as libc::c_int) as isize),
                                    *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int,
                                    (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize]
                                        - 1 as libc::c_int,
                                    cb_1,
                                    1.0f32,
                                    ::core::f32::INFINITY,
                                    &mut b_1,
                                    &mut sqenergy_1,
                                    0 as libc::c_int,
                                );
                                bits_1 += b_1;
                                qenergy_1 += sqenergy_1;
                                w2 += 1;
                                w2;
                            }
                            (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize] -= 1;
                            (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize];
                            dists[(w * 16 as libc::c_int + g) as usize] =
                                dist_1 - bits_1 as libc::c_float;
                            qenergies[(w * 16 as libc::c_int + g) as usize] = qenergy_1;
                            if mb != 0
                                && ((*sce).sf_idx[(w * 16 as libc::c_int + g) as usize]
                                    < mindeltasf
                                    || dists[(w * 16 as libc::c_int + g) as usize]
                                        < (if uplmax * uplims[(w * 16 as libc::c_int + g) as usize]
                                            > euplims[(w * 16 as libc::c_int + g) as usize]
                                        {
                                            euplims[(w * 16 as libc::c_int + g) as usize]
                                        } else {
                                            uplmax * uplims[(w * 16 as libc::c_int + g) as usize]
                                        })
                                        && fabsf(
                                            qenergies[(w * 16 as libc::c_int + g) as usize]
                                                - energies[(w * 16 as libc::c_int + g) as usize],
                                        ) < euplims[(w * 16 as libc::c_int + g) as usize])
                            {
                                break;
                            }
                            i += 1;
                            i;
                        }
                    } else if tbits > toofewbits
                        && (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize]
                            < (if maxdeltasf > maxsf[(w * 16 as libc::c_int + g) as usize] {
                                maxsf[(w * 16 as libc::c_int + g) as usize]
                            } else {
                                maxdeltasf
                            })
                        && dists[(w * 16 as libc::c_int + g) as usize]
                            < (if euplims[(w * 16 as libc::c_int + g) as usize]
                                > uplims[(w * 16 as libc::c_int + g) as usize]
                            {
                                uplims[(w * 16 as libc::c_int + g) as usize]
                            } else {
                                euplims[(w * 16 as libc::c_int + g) as usize]
                            })
                        && fabsf(
                            qenergies[(w * 16 as libc::c_int + g) as usize]
                                - energies[(w * 16 as libc::c_int + g) as usize],
                        ) < euplims[(w * 16 as libc::c_int + g) as usize]
                    {
                        i = 0 as libc::c_int;
                        while i < depth
                            && (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize] < maxdeltasf
                        {
                            let mut cb_2: libc::c_int = 0;
                            let mut bits_2: libc::c_int = 0;
                            let mut dist_2: libc::c_float = 0.;
                            let mut qenergy_2: libc::c_float = 0.;
                            cb_2 = find_min_book(
                                maxvals[(w * 16 as libc::c_int + g) as usize],
                                (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize]
                                    + 1 as libc::c_int,
                            );
                            if cb_2 > 0 as libc::c_int {
                                qenergy_2 = 0.0f32;
                                dist_2 = qenergy_2;
                                bits_2 = 0 as libc::c_int;
                                w2 = 0 as libc::c_int;
                                while w2 < (*sce).ics.group_len[w as usize] as libc::c_int {
                                    let mut b_2: libc::c_int = 0;
                                    let mut sqenergy_2: libc::c_float = 0.;
                                    dist_2 += quantize_band_cost_cached(
                                        s,
                                        w + w2,
                                        g,
                                        coefs_1.offset((w2 * 128 as libc::c_int) as isize),
                                        scaled_2.offset((w2 * 128 as libc::c_int) as isize),
                                        *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int,
                                        (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize]
                                            + 1 as libc::c_int,
                                        cb_2,
                                        1.0f32,
                                        ::core::f32::INFINITY,
                                        &mut b_2,
                                        &mut sqenergy_2,
                                        0 as libc::c_int,
                                    );
                                    bits_2 += b_2;
                                    qenergy_2 += sqenergy_2;
                                    w2 += 1;
                                    w2;
                                }
                                dist_2 -= bits_2 as libc::c_float;
                                if !(dist_2
                                    < (if euplims[(w * 16 as libc::c_int + g) as usize]
                                        > uplims[(w * 16 as libc::c_int + g) as usize]
                                    {
                                        uplims[(w * 16 as libc::c_int + g) as usize]
                                    } else {
                                        euplims[(w * 16 as libc::c_int + g) as usize]
                                    }))
                                {
                                    break;
                                }
                                (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize] += 1;
                                (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize];
                                dists[(w * 16 as libc::c_int + g) as usize] = dist_2;
                                qenergies[(w * 16 as libc::c_int + g) as usize] = qenergy_2;
                                i += 1;
                                i;
                            } else {
                                maxsf[(w * 16 as libc::c_int + g) as usize] = if (*sce).sf_idx
                                    [(w * 16 as libc::c_int + g) as usize]
                                    > maxsf[(w * 16 as libc::c_int + g) as usize]
                                {
                                    maxsf[(w * 16 as libc::c_int + g) as usize]
                                } else {
                                    (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize]
                                };
                                break;
                            }
                        }
                    }
                    (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize] = av_clip_c(
                        (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize],
                        mindeltasf,
                        maxdeltasf,
                    );
                    prev = (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize];
                    if (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize] != prevsc {
                        fflag = 1 as libc::c_int;
                    }
                    nminscaler = if nminscaler > (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize]
                    {
                        (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize]
                    } else {
                        nminscaler
                    };
                    (*sce).band_type[(w * 16 as libc::c_int + g) as usize] = find_min_book(
                        maxvals[(w * 16 as libc::c_int + g) as usize],
                        (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize],
                    )
                        as BandType;
                }
                start += *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int;
                g += 1;
                g;
            }
            w += (*sce).ics.group_len[w as usize] as libc::c_int;
        }
        prev = -(1 as libc::c_int);
        w = 0 as libc::c_int;
        while w < (*sce).ics.num_windows {
            g = 0 as libc::c_int;
            while g < (*sce).ics.num_swb {
                if (*sce).zeroes[(w * 16 as libc::c_int + g) as usize] == 0 {
                    let mut prevsf: libc::c_int =
                        (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize];
                    if prev < 0 as libc::c_int {
                        prev = prevsf;
                    }
                    (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize] = av_clip_c(
                        (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize],
                        prev - 60 as libc::c_int,
                        prev + 60 as libc::c_int,
                    );
                    (*sce).band_type[(w * 16 as libc::c_int + g) as usize] = find_min_book(
                        maxvals[(w * 16 as libc::c_int + g) as usize],
                        (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize],
                    )
                        as BandType;
                    prev = (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize];
                    if fflag == 0 && prevsf != (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize] {
                        fflag = 1 as libc::c_int;
                    }
                }
                g += 1;
                g;
            }
            w += (*sce).ics.group_len[w as usize] as libc::c_int;
        }
        its += 1;
        its;
        if !(fflag != 0 && its < maxits) {
            break;
        }
    }
    ff_init_nextband_map(sce, nextband.as_mut_ptr());
    prev = -(1 as libc::c_int);
    w = 0 as libc::c_int;
    while w < (*sce).ics.num_windows {
        g = 0 as libc::c_int;
        while g < (*sce).ics.num_swb {
            if (*sce).zeroes[(w * 16 as libc::c_int + g) as usize] == 0 {
                (*sce).band_type[(w * 16 as libc::c_int + g) as usize] = find_min_book(
                    maxvals[(w * 16 as libc::c_int + g) as usize],
                    (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize],
                )
                    as BandType;
                if (*sce).band_type[(w * 16 as libc::c_int + g) as usize] as libc::c_uint
                    <= 0 as libc::c_int as libc::c_uint
                {
                    if ff_sfdelta_can_remove_band(
                        sce,
                        nextband.as_mut_ptr(),
                        prev,
                        w * 16 as libc::c_int + g,
                    ) == 0
                    {
                        (*sce).band_type[(w * 16 as libc::c_int + g) as usize] = 1 as BandType;
                    } else {
                        (*sce).zeroes[(w * 16 as libc::c_int + g) as usize] =
                            1 as libc::c_int as uint8_t;
                        (*sce).band_type[(w * 16 as libc::c_int + g) as usize] = ZERO_BT;
                    }
                }
            } else {
                (*sce).band_type[(w * 16 as libc::c_int + g) as usize] = ZERO_BT;
            }
            if (*sce).zeroes[(w * 16 as libc::c_int + g) as usize] == 0 {
                if prev != -(1 as libc::c_int) {
                    let mut _sfdiff_1: libc::c_int =
                        (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize] - prev
                            + 60 as libc::c_int;
                } else if (*sce).zeroes[0 as libc::c_int as usize] != 0 {
                    (*sce).sf_idx[0 as libc::c_int as usize] =
                        (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize];
                }
                prev = (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize];
            }
            g += 1;
            g;
        }
        w += (*sce).ics.group_len[w as usize] as libc::c_int;
    }
}
unsafe extern "C" fn codebook_trellis_rate(
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
    mut win: libc::c_int,
    mut group_len: libc::c_int,
    _lambda: libc::c_float,
) {
    let mut path: [[TrellisBandCodingPath; 15]; 120] = [[TrellisBandCodingPath {
        prev_idx: 0,
        cost: 0.,
        run: 0,
    }; 15]; 120];
    let mut w: libc::c_int = 0;
    let mut swb: libc::c_int = 0;
    let mut cb: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let max_sfb: libc::c_int = (*sce).ics.max_sfb as libc::c_int;
    let run_bits: libc::c_int = if (*sce).ics.num_windows == 1 as libc::c_int {
        5 as libc::c_int
    } else {
        3 as libc::c_int
    };
    let run_esc: libc::c_int = ((1 as libc::c_int) << run_bits) - 1 as libc::c_int;
    let mut idx: libc::c_int = 0;
    let mut ppos: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut stackrun: [libc::c_int; 120] = [0; 120];
    let mut stackcb: [libc::c_int; 120] = [0; 120];
    let mut stack_len: libc::c_int = 0;
    let mut next_minbits: libc::c_float = ::core::f32::INFINITY;
    let mut next_mincb: libc::c_int = 0 as libc::c_int;
    ((*s).abs_pow34).expect("non-null function pointer")(
        ((*s).scoefs).as_mut_ptr(),
        ((*sce).coeffs).as_mut_ptr(),
        1024 as libc::c_int,
    );
    start = win * 128 as libc::c_int;
    cb = 0 as libc::c_int;
    while cb < 15 as libc::c_int {
        path[0 as libc::c_int as usize][cb as usize].cost =
            (run_bits + 4 as libc::c_int) as libc::c_float;
        path[0 as libc::c_int as usize][cb as usize].prev_idx = -(1 as libc::c_int);
        path[0 as libc::c_int as usize][cb as usize].run = 0 as libc::c_int;
        cb += 1;
        cb;
    }
    swb = 0 as libc::c_int;
    while swb < max_sfb {
        size = *((*sce).ics.swb_sizes).offset(swb as isize) as libc::c_int;
        if (*sce).zeroes[(win * 16 as libc::c_int + swb) as usize] != 0 {
            let mut cost_stay_here: libc::c_float =
                path[swb as usize][0 as libc::c_int as usize].cost;
            let mut cost_get_here: libc::c_float =
                next_minbits + run_bits as libc::c_float + 4 as libc::c_int as libc::c_float;
            if *(run_value_bits
                [((*sce).ics.num_windows == 8 as libc::c_int) as libc::c_int as usize])
                .offset(path[swb as usize][0 as libc::c_int as usize].run as isize)
                as libc::c_int
                != *(run_value_bits
                    [((*sce).ics.num_windows == 8 as libc::c_int) as libc::c_int as usize])
                    .offset(
                        (path[swb as usize][0 as libc::c_int as usize].run + 1 as libc::c_int)
                            as isize,
                    ) as libc::c_int
            {
                cost_stay_here += run_bits as libc::c_float;
            }
            if cost_get_here < cost_stay_here {
                path[(swb + 1 as libc::c_int) as usize][0 as libc::c_int as usize].prev_idx =
                    next_mincb;
                path[(swb + 1 as libc::c_int) as usize][0 as libc::c_int as usize].cost =
                    cost_get_here;
                path[(swb + 1 as libc::c_int) as usize][0 as libc::c_int as usize].run =
                    1 as libc::c_int;
            } else {
                path[(swb + 1 as libc::c_int) as usize][0 as libc::c_int as usize].prev_idx =
                    0 as libc::c_int;
                path[(swb + 1 as libc::c_int) as usize][0 as libc::c_int as usize].cost =
                    cost_stay_here;
                path[(swb + 1 as libc::c_int) as usize][0 as libc::c_int as usize].run =
                    path[swb as usize][0 as libc::c_int as usize].run + 1 as libc::c_int;
            }
            next_minbits = path[(swb + 1 as libc::c_int) as usize][0 as libc::c_int as usize].cost;
            next_mincb = 0 as libc::c_int;
            cb = 1 as libc::c_int;
            while cb < 15 as libc::c_int {
                path[(swb + 1 as libc::c_int) as usize][cb as usize].cost =
                    61450 as libc::c_int as libc::c_float;
                path[(swb + 1 as libc::c_int) as usize][cb as usize].prev_idx = -(1 as libc::c_int);
                path[(swb + 1 as libc::c_int) as usize][cb as usize].run = 0 as libc::c_int;
                cb += 1;
                cb;
            }
        } else {
            let mut minbits: libc::c_float = next_minbits;
            let mut mincb: libc::c_int = next_mincb;
            let mut startcb: libc::c_int =
                (*sce).band_type[(win * 16 as libc::c_int + swb) as usize] as libc::c_int;
            startcb = aac_cb_in_map[startcb as usize] as libc::c_int;
            next_minbits = ::core::f32::INFINITY;
            next_mincb = 0 as libc::c_int;
            cb = 0 as libc::c_int;
            while cb < startcb {
                path[(swb + 1 as libc::c_int) as usize][cb as usize].cost =
                    61450 as libc::c_int as libc::c_float;
                path[(swb + 1 as libc::c_int) as usize][cb as usize].prev_idx = -(1 as libc::c_int);
                path[(swb + 1 as libc::c_int) as usize][cb as usize].run = 0 as libc::c_int;
                cb += 1;
                cb;
            }
            cb = startcb;
            while cb < 15 as libc::c_int {
                let mut cost_stay_here_0: libc::c_float = 0.;
                let mut cost_get_here_0: libc::c_float = 0.;
                let mut bits: libc::c_float = 0.0f32;
                if cb >= 12 as libc::c_int
                    && (*sce).band_type[(win * 16 as libc::c_int + swb) as usize] as libc::c_uint
                        != aac_cb_out_map[cb as usize] as libc::c_uint
                {
                    path[(swb + 1 as libc::c_int) as usize][cb as usize].cost =
                        61450 as libc::c_int as libc::c_float;
                    path[(swb + 1 as libc::c_int) as usize][cb as usize].prev_idx =
                        -(1 as libc::c_int);
                    path[(swb + 1 as libc::c_int) as usize][cb as usize].run = 0 as libc::c_int;
                } else {
                    w = 0 as libc::c_int;
                    while w < group_len {
                        bits += quantize_band_cost_bits(
                            s,
                            &mut *((*sce).coeffs)
                                .as_mut_ptr()
                                .offset((start + w * 128 as libc::c_int) as isize),
                            &mut *((*s).scoefs)
                                .as_mut_ptr()
                                .offset((start + w * 128 as libc::c_int) as isize),
                            size,
                            (*sce).sf_idx[(win * 16 as libc::c_int + swb) as usize],
                            aac_cb_out_map[cb as usize] as libc::c_int,
                            0 as libc::c_int as libc::c_float,
                            ::core::f32::INFINITY,
                            std::ptr::null_mut::<libc::c_int>(),
                            std::ptr::null_mut::<libc::c_float>(),
                        ) as libc::c_float;
                        w += 1;
                        w;
                    }
                    cost_stay_here_0 = path[swb as usize][cb as usize].cost + bits;
                    cost_get_here_0 = minbits
                        + bits
                        + run_bits as libc::c_float
                        + 4 as libc::c_int as libc::c_float;
                    if *(run_value_bits
                        [((*sce).ics.num_windows == 8 as libc::c_int) as libc::c_int as usize])
                        .offset(path[swb as usize][cb as usize].run as isize)
                        as libc::c_int
                        != *(run_value_bits
                            [((*sce).ics.num_windows == 8 as libc::c_int) as libc::c_int as usize])
                            .offset(
                                (path[swb as usize][cb as usize].run + 1 as libc::c_int) as isize,
                            ) as libc::c_int
                    {
                        cost_stay_here_0 += run_bits as libc::c_float;
                    }
                    if cost_get_here_0 < cost_stay_here_0 {
                        path[(swb + 1 as libc::c_int) as usize][cb as usize].prev_idx = mincb;
                        path[(swb + 1 as libc::c_int) as usize][cb as usize].cost = cost_get_here_0;
                        path[(swb + 1 as libc::c_int) as usize][cb as usize].run = 1 as libc::c_int;
                    } else {
                        path[(swb + 1 as libc::c_int) as usize][cb as usize].prev_idx = cb;
                        path[(swb + 1 as libc::c_int) as usize][cb as usize].cost =
                            cost_stay_here_0;
                        path[(swb + 1 as libc::c_int) as usize][cb as usize].run =
                            path[swb as usize][cb as usize].run + 1 as libc::c_int;
                    }
                    if path[(swb + 1 as libc::c_int) as usize][cb as usize].cost < next_minbits {
                        next_minbits = path[(swb + 1 as libc::c_int) as usize][cb as usize].cost;
                        next_mincb = cb;
                    }
                }
                cb += 1;
                cb;
            }
        }
        start += *((*sce).ics.swb_sizes).offset(swb as isize) as libc::c_int;
        swb += 1;
        swb;
    }
    stack_len = 0 as libc::c_int;
    idx = 0 as libc::c_int;
    cb = 1 as libc::c_int;
    while cb < 15 as libc::c_int {
        if path[max_sfb as usize][cb as usize].cost < path[max_sfb as usize][idx as usize].cost {
            idx = cb;
        }
        cb += 1;
        cb;
    }
    ppos = max_sfb;
    while ppos > 0 as libc::c_int {
        cb = idx;
        stackrun[stack_len as usize] = path[ppos as usize][cb as usize].run;
        stackcb[stack_len as usize] = cb;
        idx = path[(ppos - path[ppos as usize][cb as usize].run + 1 as libc::c_int) as usize]
            [cb as usize]
            .prev_idx;
        ppos -= path[ppos as usize][cb as usize].run;
        stack_len += 1;
        stack_len;
    }
    start = 0 as libc::c_int;
    i = stack_len - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        cb = aac_cb_out_map[stackcb[i as usize] as usize] as libc::c_int;
        put_bits(&mut (*s).pb, 4 as libc::c_int, cb as BitBuf);
        count = stackrun[i as usize];
        ptr::write_bytes(
            (*sce)
                .zeroes
                .as_mut_ptr()
                .offset((win * 16 as libc::c_int) as isize)
                .offset(start as isize),
            u8::from(cb == 0),
            count as usize,
        );
        j = 0 as libc::c_int;
        while j < count {
            (*sce).band_type[(win * 16 as libc::c_int + start) as usize] = cb as BandType;
            start += 1;
            start;
            j += 1;
            j;
        }
        while count >= run_esc {
            put_bits(&mut (*s).pb, run_bits, run_esc as BitBuf);
            count -= run_esc;
        }
        put_bits(&mut (*s).pb, run_bits, count as BitBuf);
        i -= 1;
        i;
    }
}
#[inline(always)]
unsafe fn quantize_and_encode_band_cost_template(
    mut s: *mut AACEncContext,
    mut pb: *mut PutBitContext,
    mut in_0: *const libc::c_float,
    mut out: *mut libc::c_float,
    mut scaled: *const libc::c_float,
    mut size: libc::c_int,
    mut scale_idx: libc::c_int,
    mut cb: libc::c_int,
    lambda: libc::c_float,
    uplim: libc::c_float,
    mut bits: *mut libc::c_int,
    mut energy: *mut libc::c_float,
    mut BT_ZERO: libc::c_int,
    mut BT_UNSIGNED: libc::c_int,
    mut BT_PAIR: libc::c_int,
    mut BT_ESC: libc::c_int,
    mut BT_NOISE: libc::c_int,
    mut BT_STEREO: libc::c_int,
    ROUNDING: libc::c_float,
) -> libc::c_float {
    let q_idx: libc::c_int =
        200 as libc::c_int - scale_idx + 140 as libc::c_int - 36 as libc::c_int;
    let Q: libc::c_float = ff_aac_pow2sf_tab[q_idx as usize];
    let Q34: libc::c_float = ff_aac_pow34sf_tab[q_idx as usize];
    let IQ: libc::c_float = ff_aac_pow2sf_tab
        [(200 as libc::c_int + scale_idx - 140 as libc::c_int + 36 as libc::c_int) as usize];
    let CLIPPED_ESCAPE: libc::c_float = 165140.0f32 * IQ;
    let mut cost: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut qenergy: libc::c_float = 0 as libc::c_int as libc::c_float;
    let dim: libc::c_int = if BT_PAIR != 0 {
        2 as libc::c_int
    } else {
        4 as libc::c_int
    };
    let mut resbits: libc::c_int = 0 as libc::c_int;
    let mut off: libc::c_int = 0;
    if BT_ZERO != 0 || BT_NOISE != 0 || BT_STEREO != 0 {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < size {
            cost += *in_0.offset(i as isize) * *in_0.offset(i as isize);
            i += 1;
            i;
        }
        if !bits.is_null() {
            *bits = 0 as libc::c_int;
        }
        if !energy.is_null() {
            *energy = qenergy;
        }
        if !out.is_null() {
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while i_0 < size {
                let mut j: libc::c_int = 0 as libc::c_int;
                while j < dim {
                    *out.offset((i_0 + j) as isize) = 0.0f32;
                    j += 1;
                    j;
                }
                i_0 += dim;
            }
        }
        return cost * lambda;
    }
    if scaled.is_null() {
        ((*s).abs_pow34).expect("non-null function pointer")(
            ((*s).scoefs).as_mut_ptr(),
            in_0,
            size,
        );
        scaled = ((*s).scoefs).as_mut_ptr();
    }
    ((*s).quant_bands).expect("non-null function pointer")(
        ((*s).qcoefs).as_mut_ptr(),
        in_0,
        scaled,
        size,
        (BT_UNSIGNED == 0) as libc::c_int,
        aac_cb_maxval[cb as usize] as libc::c_int,
        Q34,
        ROUNDING,
    );
    if BT_UNSIGNED != 0 {
        off = 0 as libc::c_int;
    } else {
        off = aac_cb_maxval[cb as usize] as libc::c_int;
    }
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < size {
        let mut vec: *const libc::c_float = std::ptr::null::<libc::c_float>();
        let mut quants: *mut libc::c_int = ((*s).qcoefs).as_mut_ptr().offset(i_1 as isize);
        let mut curidx: libc::c_int = 0 as libc::c_int;
        let mut curbits: libc::c_int = 0;
        let mut quantized: libc::c_float = 0.;
        let mut rd: libc::c_float = 0.0f32;
        let mut j_0: libc::c_int = 0 as libc::c_int;
        while j_0 < dim {
            curidx *= aac_cb_range[cb as usize] as libc::c_int;
            curidx += *quants.offset(j_0 as isize) + off;
            j_0 += 1;
            j_0;
        }
        curbits = *(ff_aac_spectral_bits[(cb - 1 as libc::c_int) as usize]).offset(curidx as isize)
            as libc::c_int;
        vec = &*(*ff_aac_codebook_vectors
            .as_ptr()
            .offset((cb - 1 as libc::c_int) as isize))
        .offset((curidx * dim) as isize) as *const libc::c_float;
        if BT_UNSIGNED != 0 {
            let mut j_1: libc::c_int = 0 as libc::c_int;
            while j_1 < dim {
                let mut t: libc::c_float = fabsf(*in_0.offset((i_1 + j_1) as isize));
                let mut di: libc::c_float = 0.;
                if BT_ESC != 0 && *vec.offset(j_1 as isize) == 64.0f32 {
                    if t >= CLIPPED_ESCAPE {
                        quantized = CLIPPED_ESCAPE;
                        curbits += 21 as libc::c_int;
                    } else {
                        let mut c: libc::c_int =
                            av_clip_uintp2_c(quant(t, Q, ROUNDING), 13 as libc::c_int)
                                as libc::c_int;
                        quantized = c as libc::c_float * cbrtf(c as libc::c_float) * IQ;
                        curbits += ff_log2_c(c as libc::c_uint) * 2 as libc::c_int
                            - 4 as libc::c_int
                            + 1 as libc::c_int;
                    }
                } else {
                    quantized = *vec.offset(j_1 as isize) * IQ;
                }
                di = t - quantized;
                if !out.is_null() {
                    *out.offset((i_1 + j_1) as isize) = if *in_0.offset((i_1 + j_1) as isize)
                        >= 0 as libc::c_int as libc::c_float
                    {
                        quantized
                    } else {
                        -quantized
                    };
                }
                if *vec.offset(j_1 as isize) != 0.0f32 {
                    curbits += 1;
                    curbits;
                }
                qenergy += quantized * quantized;
                rd += di * di;
                j_1 += 1;
                j_1;
            }
        } else {
            let mut j_2: libc::c_int = 0 as libc::c_int;
            while j_2 < dim {
                quantized = *vec.offset(j_2 as isize) * IQ;
                qenergy += quantized * quantized;
                if !out.is_null() {
                    *out.offset((i_1 + j_2) as isize) = quantized;
                }
                rd += (*in_0.offset((i_1 + j_2) as isize) - quantized)
                    * (*in_0.offset((i_1 + j_2) as isize) - quantized);
                j_2 += 1;
                j_2;
            }
        }
        cost += rd * lambda + curbits as libc::c_float;
        resbits += curbits;
        if cost >= uplim {
            return uplim;
        }
        if !pb.is_null() {
            put_bits(
                pb,
                *(ff_aac_spectral_bits[(cb - 1 as libc::c_int) as usize]).offset(curidx as isize)
                    as libc::c_int,
                *(ff_aac_spectral_codes[(cb - 1 as libc::c_int) as usize]).offset(curidx as isize)
                    as BitBuf,
            );
            if BT_UNSIGNED != 0 {
                let mut j_3: libc::c_int = 0 as libc::c_int;
                while j_3 < dim {
                    if *(*ff_aac_codebook_vectors
                        .as_ptr()
                        .offset((cb - 1 as libc::c_int) as isize))
                    .offset((curidx * dim + j_3) as isize)
                        != 0.0f32
                    {
                        put_bits(
                            pb,
                            1 as libc::c_int,
                            (*in_0.offset((i_1 + j_3) as isize) < 0.0f32) as libc::c_int as BitBuf,
                        );
                    }
                    j_3 += 1;
                    j_3;
                }
            }
            if BT_ESC != 0 {
                let mut j_4: libc::c_int = 0 as libc::c_int;
                while j_4 < 2 as libc::c_int {
                    if *(*ff_aac_codebook_vectors
                        .as_ptr()
                        .offset((cb - 1 as libc::c_int) as isize))
                    .offset((curidx * 2 as libc::c_int + j_4) as isize)
                        == 64.0f32
                    {
                        let mut coef: libc::c_int = av_clip_uintp2_c(
                            quant(fabsf(*in_0.offset((i_1 + j_4) as isize)), Q, ROUNDING),
                            13 as libc::c_int,
                        ) as libc::c_int;
                        let mut len: libc::c_int = ff_log2_c(coef as libc::c_uint);
                        put_bits(
                            pb,
                            len - 4 as libc::c_int + 1 as libc::c_int,
                            (((1 as libc::c_int) << len - 4 as libc::c_int + 1 as libc::c_int)
                                - 2 as libc::c_int) as BitBuf,
                        );
                        put_sbits(pb, len, coef);
                    }
                    j_4 += 1;
                    j_4;
                }
            }
        }
        i_1 += dim;
    }
    if !bits.is_null() {
        *bits = resbits;
    }
    if !energy.is_null() {
        *energy = qenergy;
    }
    cost
}
#[inline]
unsafe fn quantize_and_encode_band_cost_NONE(
    mut _s: *mut AACEncContext,
    mut _pb: *mut PutBitContext,
    mut _in_0: *const libc::c_float,
    mut _quant_0: *mut libc::c_float,
    mut _scaled: *const libc::c_float,
    mut _size: libc::c_int,
    mut _scale_idx: libc::c_int,
    mut _cb: libc::c_int,
    _lambda: libc::c_float,
    _uplim: libc::c_float,
    mut _bits: *mut libc::c_int,
    mut _energy: *mut libc::c_float,
) -> libc::c_float {
    0.0f32
}
unsafe fn quantize_and_encode_band_cost_ZERO(
    mut s: *mut AACEncContext,
    mut pb: *mut PutBitContext,
    mut in_0: *const libc::c_float,
    mut quant_0: *mut libc::c_float,
    mut scaled: *const libc::c_float,
    mut size: libc::c_int,
    mut scale_idx: libc::c_int,
    mut cb: libc::c_int,
    lambda: libc::c_float,
    uplim: libc::c_float,
    mut bits: *mut libc::c_int,
    mut energy: *mut libc::c_float,
) -> libc::c_float {
    quantize_and_encode_band_cost_template(
        s,
        pb,
        in_0,
        quant_0,
        scaled,
        size,
        scale_idx,
        if 0 as libc::c_int != 0 {
            ESC_BT as libc::c_int
        } else {
            cb
        },
        lambda,
        uplim,
        bits,
        energy,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0.4054f32,
    )
}
unsafe fn quantize_and_encode_band_cost_SQUAD(
    mut s: *mut AACEncContext,
    mut pb: *mut PutBitContext,
    mut in_0: *const libc::c_float,
    mut quant_0: *mut libc::c_float,
    mut scaled: *const libc::c_float,
    mut size: libc::c_int,
    mut scale_idx: libc::c_int,
    mut cb: libc::c_int,
    lambda: libc::c_float,
    uplim: libc::c_float,
    mut bits: *mut libc::c_int,
    mut energy: *mut libc::c_float,
) -> libc::c_float {
    quantize_and_encode_band_cost_template(
        s,
        pb,
        in_0,
        quant_0,
        scaled,
        size,
        scale_idx,
        if 0 as libc::c_int != 0 {
            ESC_BT as libc::c_int
        } else {
            cb
        },
        lambda,
        uplim,
        bits,
        energy,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0.4054f32,
    )
}
unsafe fn quantize_and_encode_band_cost_UQUAD(
    mut s: *mut AACEncContext,
    mut pb: *mut PutBitContext,
    mut in_0: *const libc::c_float,
    mut quant_0: *mut libc::c_float,
    mut scaled: *const libc::c_float,
    mut size: libc::c_int,
    mut scale_idx: libc::c_int,
    mut cb: libc::c_int,
    lambda: libc::c_float,
    uplim: libc::c_float,
    mut bits: *mut libc::c_int,
    mut energy: *mut libc::c_float,
) -> libc::c_float {
    quantize_and_encode_band_cost_template(
        s,
        pb,
        in_0,
        quant_0,
        scaled,
        size,
        scale_idx,
        if 0 as libc::c_int != 0 {
            ESC_BT as libc::c_int
        } else {
            cb
        },
        lambda,
        uplim,
        bits,
        energy,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0.4054f32,
    )
}
unsafe fn quantize_and_encode_band_cost_SPAIR(
    mut s: *mut AACEncContext,
    mut pb: *mut PutBitContext,
    mut in_0: *const libc::c_float,
    mut quant_0: *mut libc::c_float,
    mut scaled: *const libc::c_float,
    mut size: libc::c_int,
    mut scale_idx: libc::c_int,
    mut cb: libc::c_int,
    lambda: libc::c_float,
    uplim: libc::c_float,
    mut bits: *mut libc::c_int,
    mut energy: *mut libc::c_float,
) -> libc::c_float {
    quantize_and_encode_band_cost_template(
        s,
        pb,
        in_0,
        quant_0,
        scaled,
        size,
        scale_idx,
        if 0 as libc::c_int != 0 {
            ESC_BT as libc::c_int
        } else {
            cb
        },
        lambda,
        uplim,
        bits,
        energy,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0.4054f32,
    )
}
unsafe fn quantize_and_encode_band_cost_UPAIR(
    mut s: *mut AACEncContext,
    mut pb: *mut PutBitContext,
    mut in_0: *const libc::c_float,
    mut quant_0: *mut libc::c_float,
    mut scaled: *const libc::c_float,
    mut size: libc::c_int,
    mut scale_idx: libc::c_int,
    mut cb: libc::c_int,
    lambda: libc::c_float,
    uplim: libc::c_float,
    mut bits: *mut libc::c_int,
    mut energy: *mut libc::c_float,
) -> libc::c_float {
    quantize_and_encode_band_cost_template(
        s,
        pb,
        in_0,
        quant_0,
        scaled,
        size,
        scale_idx,
        if 0 as libc::c_int != 0 {
            ESC_BT as libc::c_int
        } else {
            cb
        },
        lambda,
        uplim,
        bits,
        energy,
        0 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0.4054f32,
    )
}
unsafe fn quantize_and_encode_band_cost_ESC(
    mut s: *mut AACEncContext,
    mut pb: *mut PutBitContext,
    mut in_0: *const libc::c_float,
    mut quant_0: *mut libc::c_float,
    mut scaled: *const libc::c_float,
    mut size: libc::c_int,
    mut scale_idx: libc::c_int,
    mut cb: libc::c_int,
    lambda: libc::c_float,
    uplim: libc::c_float,
    mut bits: *mut libc::c_int,
    mut energy: *mut libc::c_float,
) -> libc::c_float {
    quantize_and_encode_band_cost_template(
        s,
        pb,
        in_0,
        quant_0,
        scaled,
        size,
        scale_idx,
        if 1 as libc::c_int != 0 {
            ESC_BT as libc::c_int
        } else {
            cb
        },
        lambda,
        uplim,
        bits,
        energy,
        0 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0.4054f32,
    )
}
unsafe fn quantize_and_encode_band_cost_ESC_RTZ(
    mut s: *mut AACEncContext,
    mut pb: *mut PutBitContext,
    mut in_0: *const libc::c_float,
    mut quant_0: *mut libc::c_float,
    mut scaled: *const libc::c_float,
    mut size: libc::c_int,
    mut scale_idx: libc::c_int,
    mut cb: libc::c_int,
    lambda: libc::c_float,
    uplim: libc::c_float,
    mut bits: *mut libc::c_int,
    mut energy: *mut libc::c_float,
) -> libc::c_float {
    quantize_and_encode_band_cost_template(
        s,
        pb,
        in_0,
        quant_0,
        scaled,
        size,
        scale_idx,
        if 1 as libc::c_int != 0 {
            ESC_BT as libc::c_int
        } else {
            cb
        },
        lambda,
        uplim,
        bits,
        energy,
        0 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0.1054f32,
    )
}
unsafe fn quantize_and_encode_band_cost_NOISE(
    mut s: *mut AACEncContext,
    mut pb: *mut PutBitContext,
    mut in_0: *const libc::c_float,
    mut quant_0: *mut libc::c_float,
    mut scaled: *const libc::c_float,
    mut size: libc::c_int,
    mut scale_idx: libc::c_int,
    mut cb: libc::c_int,
    lambda: libc::c_float,
    uplim: libc::c_float,
    mut bits: *mut libc::c_int,
    mut energy: *mut libc::c_float,
) -> libc::c_float {
    quantize_and_encode_band_cost_template(
        s,
        pb,
        in_0,
        quant_0,
        scaled,
        size,
        scale_idx,
        if 0 as libc::c_int != 0 {
            ESC_BT as libc::c_int
        } else {
            cb
        },
        lambda,
        uplim,
        bits,
        energy,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        0.4054f32,
    )
}
unsafe fn quantize_and_encode_band_cost_STEREO(
    mut s: *mut AACEncContext,
    mut pb: *mut PutBitContext,
    mut in_0: *const libc::c_float,
    mut quant_0: *mut libc::c_float,
    mut scaled: *const libc::c_float,
    mut size: libc::c_int,
    mut scale_idx: libc::c_int,
    mut cb: libc::c_int,
    lambda: libc::c_float,
    uplim: libc::c_float,
    mut bits: *mut libc::c_int,
    mut energy: *mut libc::c_float,
) -> libc::c_float {
    quantize_and_encode_band_cost_template(
        s,
        pb,
        in_0,
        quant_0,
        scaled,
        size,
        scale_idx,
        if 0 as libc::c_int != 0 {
            ESC_BT as libc::c_int
        } else {
            cb
        },
        lambda,
        uplim,
        bits,
        energy,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        0.4054f32,
    )
}
static mut quantize_and_encode_band_cost_arr: [quantize_and_encode_band_func; 16] = {
    [
        Some(
            quantize_and_encode_band_cost_ZERO
                as unsafe fn(
                    *mut AACEncContext,
                    *mut PutBitContext,
                    *const libc::c_float,
                    *mut libc::c_float,
                    *const libc::c_float,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_float,
                    libc::c_float,
                    *mut libc::c_int,
                    *mut libc::c_float,
                ) -> libc::c_float,
        ),
        Some(
            quantize_and_encode_band_cost_SQUAD
                as unsafe fn(
                    *mut AACEncContext,
                    *mut PutBitContext,
                    *const libc::c_float,
                    *mut libc::c_float,
                    *const libc::c_float,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_float,
                    libc::c_float,
                    *mut libc::c_int,
                    *mut libc::c_float,
                ) -> libc::c_float,
        ),
        Some(
            quantize_and_encode_band_cost_SQUAD
                as unsafe fn(
                    *mut AACEncContext,
                    *mut PutBitContext,
                    *const libc::c_float,
                    *mut libc::c_float,
                    *const libc::c_float,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_float,
                    libc::c_float,
                    *mut libc::c_int,
                    *mut libc::c_float,
                ) -> libc::c_float,
        ),
        Some(
            quantize_and_encode_band_cost_UQUAD
                as unsafe fn(
                    *mut AACEncContext,
                    *mut PutBitContext,
                    *const libc::c_float,
                    *mut libc::c_float,
                    *const libc::c_float,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_float,
                    libc::c_float,
                    *mut libc::c_int,
                    *mut libc::c_float,
                ) -> libc::c_float,
        ),
        Some(
            quantize_and_encode_band_cost_UQUAD
                as unsafe fn(
                    *mut AACEncContext,
                    *mut PutBitContext,
                    *const libc::c_float,
                    *mut libc::c_float,
                    *const libc::c_float,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_float,
                    libc::c_float,
                    *mut libc::c_int,
                    *mut libc::c_float,
                ) -> libc::c_float,
        ),
        Some(
            quantize_and_encode_band_cost_SPAIR
                as unsafe fn(
                    *mut AACEncContext,
                    *mut PutBitContext,
                    *const libc::c_float,
                    *mut libc::c_float,
                    *const libc::c_float,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_float,
                    libc::c_float,
                    *mut libc::c_int,
                    *mut libc::c_float,
                ) -> libc::c_float,
        ),
        Some(
            quantize_and_encode_band_cost_SPAIR
                as unsafe fn(
                    *mut AACEncContext,
                    *mut PutBitContext,
                    *const libc::c_float,
                    *mut libc::c_float,
                    *const libc::c_float,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_float,
                    libc::c_float,
                    *mut libc::c_int,
                    *mut libc::c_float,
                ) -> libc::c_float,
        ),
        Some(
            quantize_and_encode_band_cost_UPAIR
                as unsafe fn(
                    *mut AACEncContext,
                    *mut PutBitContext,
                    *const libc::c_float,
                    *mut libc::c_float,
                    *const libc::c_float,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_float,
                    libc::c_float,
                    *mut libc::c_int,
                    *mut libc::c_float,
                ) -> libc::c_float,
        ),
        Some(
            quantize_and_encode_band_cost_UPAIR
                as unsafe fn(
                    *mut AACEncContext,
                    *mut PutBitContext,
                    *const libc::c_float,
                    *mut libc::c_float,
                    *const libc::c_float,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_float,
                    libc::c_float,
                    *mut libc::c_int,
                    *mut libc::c_float,
                ) -> libc::c_float,
        ),
        Some(
            quantize_and_encode_band_cost_UPAIR
                as unsafe fn(
                    *mut AACEncContext,
                    *mut PutBitContext,
                    *const libc::c_float,
                    *mut libc::c_float,
                    *const libc::c_float,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_float,
                    libc::c_float,
                    *mut libc::c_int,
                    *mut libc::c_float,
                ) -> libc::c_float,
        ),
        Some(
            quantize_and_encode_band_cost_UPAIR
                as unsafe fn(
                    *mut AACEncContext,
                    *mut PutBitContext,
                    *const libc::c_float,
                    *mut libc::c_float,
                    *const libc::c_float,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_float,
                    libc::c_float,
                    *mut libc::c_int,
                    *mut libc::c_float,
                ) -> libc::c_float,
        ),
        Some(
            quantize_and_encode_band_cost_ESC
                as unsafe fn(
                    *mut AACEncContext,
                    *mut PutBitContext,
                    *const libc::c_float,
                    *mut libc::c_float,
                    *const libc::c_float,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_float,
                    libc::c_float,
                    *mut libc::c_int,
                    *mut libc::c_float,
                ) -> libc::c_float,
        ),
        Some(
            quantize_and_encode_band_cost_NONE
                as unsafe fn(
                    *mut AACEncContext,
                    *mut PutBitContext,
                    *const libc::c_float,
                    *mut libc::c_float,
                    *const libc::c_float,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_float,
                    libc::c_float,
                    *mut libc::c_int,
                    *mut libc::c_float,
                ) -> libc::c_float,
        ),
        Some(
            quantize_and_encode_band_cost_NOISE
                as unsafe fn(
                    *mut AACEncContext,
                    *mut PutBitContext,
                    *const libc::c_float,
                    *mut libc::c_float,
                    *const libc::c_float,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_float,
                    libc::c_float,
                    *mut libc::c_int,
                    *mut libc::c_float,
                ) -> libc::c_float,
        ),
        Some(
            quantize_and_encode_band_cost_STEREO
                as unsafe fn(
                    *mut AACEncContext,
                    *mut PutBitContext,
                    *const libc::c_float,
                    *mut libc::c_float,
                    *const libc::c_float,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_float,
                    libc::c_float,
                    *mut libc::c_int,
                    *mut libc::c_float,
                ) -> libc::c_float,
        ),
        Some(
            quantize_and_encode_band_cost_STEREO
                as unsafe fn(
                    *mut AACEncContext,
                    *mut PutBitContext,
                    *const libc::c_float,
                    *mut libc::c_float,
                    *const libc::c_float,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_float,
                    libc::c_float,
                    *mut libc::c_int,
                    *mut libc::c_float,
                ) -> libc::c_float,
        ),
    ]
};
static mut quantize_and_encode_band_cost_rtz_arr: [quantize_and_encode_band_func; 16] = unsafe {
    [
        Some(
            quantize_and_encode_band_cost_ZERO
                as unsafe fn(
                    *mut AACEncContext,
                    *mut PutBitContext,
                    *const libc::c_float,
                    *mut libc::c_float,
                    *const libc::c_float,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_float,
                    libc::c_float,
                    *mut libc::c_int,
                    *mut libc::c_float,
                ) -> libc::c_float,
        ),
        Some(
            quantize_and_encode_band_cost_SQUAD
                as unsafe fn(
                    *mut AACEncContext,
                    *mut PutBitContext,
                    *const libc::c_float,
                    *mut libc::c_float,
                    *const libc::c_float,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_float,
                    libc::c_float,
                    *mut libc::c_int,
                    *mut libc::c_float,
                ) -> libc::c_float,
        ),
        Some(
            quantize_and_encode_band_cost_SQUAD
                as unsafe fn(
                    *mut AACEncContext,
                    *mut PutBitContext,
                    *const libc::c_float,
                    *mut libc::c_float,
                    *const libc::c_float,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_float,
                    libc::c_float,
                    *mut libc::c_int,
                    *mut libc::c_float,
                ) -> libc::c_float,
        ),
        Some(
            quantize_and_encode_band_cost_UQUAD
                as unsafe fn(
                    *mut AACEncContext,
                    *mut PutBitContext,
                    *const libc::c_float,
                    *mut libc::c_float,
                    *const libc::c_float,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_float,
                    libc::c_float,
                    *mut libc::c_int,
                    *mut libc::c_float,
                ) -> libc::c_float,
        ),
        Some(
            quantize_and_encode_band_cost_UQUAD
                as unsafe fn(
                    *mut AACEncContext,
                    *mut PutBitContext,
                    *const libc::c_float,
                    *mut libc::c_float,
                    *const libc::c_float,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_float,
                    libc::c_float,
                    *mut libc::c_int,
                    *mut libc::c_float,
                ) -> libc::c_float,
        ),
        Some(
            quantize_and_encode_band_cost_SPAIR
                as unsafe fn(
                    *mut AACEncContext,
                    *mut PutBitContext,
                    *const libc::c_float,
                    *mut libc::c_float,
                    *const libc::c_float,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_float,
                    libc::c_float,
                    *mut libc::c_int,
                    *mut libc::c_float,
                ) -> libc::c_float,
        ),
        Some(
            quantize_and_encode_band_cost_SPAIR
                as unsafe fn(
                    *mut AACEncContext,
                    *mut PutBitContext,
                    *const libc::c_float,
                    *mut libc::c_float,
                    *const libc::c_float,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_float,
                    libc::c_float,
                    *mut libc::c_int,
                    *mut libc::c_float,
                ) -> libc::c_float,
        ),
        Some(
            quantize_and_encode_band_cost_UPAIR
                as unsafe fn(
                    *mut AACEncContext,
                    *mut PutBitContext,
                    *const libc::c_float,
                    *mut libc::c_float,
                    *const libc::c_float,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_float,
                    libc::c_float,
                    *mut libc::c_int,
                    *mut libc::c_float,
                ) -> libc::c_float,
        ),
        Some(
            quantize_and_encode_band_cost_UPAIR
                as unsafe fn(
                    *mut AACEncContext,
                    *mut PutBitContext,
                    *const libc::c_float,
                    *mut libc::c_float,
                    *const libc::c_float,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_float,
                    libc::c_float,
                    *mut libc::c_int,
                    *mut libc::c_float,
                ) -> libc::c_float,
        ),
        Some(
            quantize_and_encode_band_cost_UPAIR
                as unsafe fn(
                    *mut AACEncContext,
                    *mut PutBitContext,
                    *const libc::c_float,
                    *mut libc::c_float,
                    *const libc::c_float,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_float,
                    libc::c_float,
                    *mut libc::c_int,
                    *mut libc::c_float,
                ) -> libc::c_float,
        ),
        Some(
            quantize_and_encode_band_cost_UPAIR
                as unsafe fn(
                    *mut AACEncContext,
                    *mut PutBitContext,
                    *const libc::c_float,
                    *mut libc::c_float,
                    *const libc::c_float,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_float,
                    libc::c_float,
                    *mut libc::c_int,
                    *mut libc::c_float,
                ) -> libc::c_float,
        ),
        Some(
            quantize_and_encode_band_cost_ESC_RTZ
                as unsafe fn(
                    *mut AACEncContext,
                    *mut PutBitContext,
                    *const libc::c_float,
                    *mut libc::c_float,
                    *const libc::c_float,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_float,
                    libc::c_float,
                    *mut libc::c_int,
                    *mut libc::c_float,
                ) -> libc::c_float,
        ),
        Some(
            quantize_and_encode_band_cost_NONE
                as unsafe fn(
                    *mut AACEncContext,
                    *mut PutBitContext,
                    *const libc::c_float,
                    *mut libc::c_float,
                    *const libc::c_float,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_float,
                    libc::c_float,
                    *mut libc::c_int,
                    *mut libc::c_float,
                ) -> libc::c_float,
        ),
        Some(
            quantize_and_encode_band_cost_NOISE
                as unsafe fn(
                    *mut AACEncContext,
                    *mut PutBitContext,
                    *const libc::c_float,
                    *mut libc::c_float,
                    *const libc::c_float,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_float,
                    libc::c_float,
                    *mut libc::c_int,
                    *mut libc::c_float,
                ) -> libc::c_float,
        ),
        Some(
            quantize_and_encode_band_cost_STEREO
                as unsafe fn(
                    *mut AACEncContext,
                    *mut PutBitContext,
                    *const libc::c_float,
                    *mut libc::c_float,
                    *const libc::c_float,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_float,
                    libc::c_float,
                    *mut libc::c_int,
                    *mut libc::c_float,
                ) -> libc::c_float,
        ),
        Some(
            quantize_and_encode_band_cost_STEREO
                as unsafe fn(
                    *mut AACEncContext,
                    *mut PutBitContext,
                    *const libc::c_float,
                    *mut libc::c_float,
                    *const libc::c_float,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_float,
                    libc::c_float,
                    *mut libc::c_int,
                    *mut libc::c_float,
                ) -> libc::c_float,
        ),
    ]
};

pub(crate) unsafe fn ff_quantize_and_encode_band_cost(
    mut s: *mut AACEncContext,
    mut pb: *mut PutBitContext,
    mut in_0: *const libc::c_float,
    mut quant_0: *mut libc::c_float,
    mut scaled: *const libc::c_float,
    mut size: libc::c_int,
    mut scale_idx: libc::c_int,
    mut cb: libc::c_int,
    lambda: libc::c_float,
    uplim: libc::c_float,
    mut bits: *mut libc::c_int,
    mut energy: *mut libc::c_float,
) -> libc::c_float {
    (quantize_and_encode_band_cost_arr[cb as usize]).expect("non-null function pointer")(
        s, pb, in_0, quant_0, scaled, size, scale_idx, cb, lambda, uplim, bits, energy,
    )
}
#[inline]
unsafe extern "C" fn quantize_and_encode_band(
    mut s: *mut AACEncContext,
    mut pb: *mut PutBitContext,
    mut in_0: *const libc::c_float,
    mut out: *mut libc::c_float,
    mut size: libc::c_int,
    mut scale_idx: libc::c_int,
    mut cb: libc::c_int,
    lambda: libc::c_float,
    mut rtz: libc::c_int,
) {
    (*if rtz != 0 {
        quantize_and_encode_band_cost_rtz_arr.as_ptr()
    } else {
        quantize_and_encode_band_cost_arr.as_ptr()
    }
    .offset(cb as isize))
    .expect("non-null function pointer")(
        s,
        pb,
        in_0,
        out,
        std::ptr::null::<libc::c_float>(),
        size,
        scale_idx,
        cb,
        lambda,
        ::core::f32::INFINITY,
        std::ptr::null_mut::<libc::c_int>(),
        std::ptr::null_mut::<libc::c_float>(),
    );
}
unsafe extern "C" fn encode_window_bands_info(
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
    mut win: libc::c_int,
    mut group_len: libc::c_int,
    lambda: libc::c_float,
) {
    let mut path: [[BandCodingPath; 15]; 120] = [[BandCodingPath {
        prev_idx: 0,
        cost: 0.,
        run: 0,
    }; 15]; 120];
    let mut w: libc::c_int = 0;
    let mut swb: libc::c_int = 0;
    let mut cb: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let max_sfb: libc::c_int = (*sce).ics.max_sfb as libc::c_int;
    let run_bits: libc::c_int = if (*sce).ics.num_windows == 1 as libc::c_int {
        5 as libc::c_int
    } else {
        3 as libc::c_int
    };
    let run_esc: libc::c_int = ((1 as libc::c_int) << run_bits) - 1 as libc::c_int;
    let mut idx: libc::c_int = 0;
    let mut ppos: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut stackrun: [libc::c_int; 120] = [0; 120];
    let mut stackcb: [libc::c_int; 120] = [0; 120];
    let mut stack_len: libc::c_int = 0;
    let mut next_minrd: libc::c_float = ::core::f32::INFINITY;
    let mut next_mincb: libc::c_int = 0 as libc::c_int;
    ((*s).abs_pow34).expect("non-null function pointer")(
        ((*s).scoefs).as_mut_ptr(),
        ((*sce).coeffs).as_mut_ptr(),
        1024 as libc::c_int,
    );
    start = win * 128 as libc::c_int;
    cb = 0 as libc::c_int;
    while cb < 15 as libc::c_int {
        path[0 as libc::c_int as usize][cb as usize].cost = 0.0f32;
        path[0 as libc::c_int as usize][cb as usize].prev_idx = -(1 as libc::c_int);
        path[0 as libc::c_int as usize][cb as usize].run = 0 as libc::c_int;
        cb += 1;
        cb;
    }
    swb = 0 as libc::c_int;
    while swb < max_sfb {
        size = *((*sce).ics.swb_sizes).offset(swb as isize) as libc::c_int;
        if (*sce).zeroes[(win * 16 as libc::c_int + swb) as usize] != 0 {
            cb = 0 as libc::c_int;
            while cb < 15 as libc::c_int {
                path[(swb + 1 as libc::c_int) as usize][cb as usize].prev_idx = cb;
                path[(swb + 1 as libc::c_int) as usize][cb as usize].cost =
                    path[swb as usize][cb as usize].cost;
                path[(swb + 1 as libc::c_int) as usize][cb as usize].run =
                    path[swb as usize][cb as usize].run + 1 as libc::c_int;
                cb += 1;
                cb;
            }
        } else {
            let mut minrd: libc::c_float = next_minrd;
            let mut mincb: libc::c_int = next_mincb;
            next_minrd = ::core::f32::INFINITY;
            next_mincb = 0 as libc::c_int;
            cb = 0 as libc::c_int;
            while cb < 15 as libc::c_int {
                let mut cost_stay_here: libc::c_float = 0.;
                let mut cost_get_here: libc::c_float = 0.;
                let mut rd: libc::c_float = 0.0f32;
                if cb >= 12 as libc::c_int
                    && ((*sce).band_type[(win * 16 as libc::c_int + swb) as usize] as libc::c_uint)
                        < aac_cb_out_map[cb as usize] as libc::c_uint
                    || cb
                        < aac_cb_in_map
                            [(*sce).band_type[(win * 16 as libc::c_int + swb) as usize] as usize]
                            as libc::c_int
                        && (*sce).band_type[(win * 16 as libc::c_int + swb) as usize]
                            as libc::c_uint
                            > aac_cb_out_map[cb as usize] as libc::c_uint
                {
                    path[(swb + 1 as libc::c_int) as usize][cb as usize].prev_idx =
                        -(1 as libc::c_int);
                    path[(swb + 1 as libc::c_int) as usize][cb as usize].cost =
                        ::core::f32::INFINITY;
                    path[(swb + 1 as libc::c_int) as usize][cb as usize].run =
                        path[swb as usize][cb as usize].run + 1 as libc::c_int;
                } else {
                    w = 0 as libc::c_int;
                    while w < group_len {
                        let mut band: *mut FFPsyBand =
                            &mut *((*((*s).psy.ch).offset((*s).cur_channel as isize)).psy_bands)
                                .as_mut_ptr()
                                .offset(((win + w) * 16 as libc::c_int + swb) as isize)
                                as *mut FFPsyBand;
                        rd += quantize_band_cost(
                            s,
                            &mut *((*sce).coeffs)
                                .as_mut_ptr()
                                .offset((start + w * 128 as libc::c_int) as isize),
                            &mut *((*s).scoefs)
                                .as_mut_ptr()
                                .offset((start + w * 128 as libc::c_int) as isize),
                            size,
                            (*sce).sf_idx[((win + w) * 16 as libc::c_int + swb) as usize],
                            aac_cb_out_map[cb as usize] as libc::c_int,
                            lambda / (*band).threshold,
                            ::core::f32::INFINITY,
                            std::ptr::null_mut::<libc::c_int>(),
                            std::ptr::null_mut::<libc::c_float>(),
                        );
                        w += 1;
                        w;
                    }
                    cost_stay_here = path[swb as usize][cb as usize].cost + rd;
                    cost_get_here =
                        minrd + rd + run_bits as libc::c_float + 4 as libc::c_int as libc::c_float;
                    if *(run_value_bits
                        [((*sce).ics.num_windows == 8 as libc::c_int) as libc::c_int as usize])
                        .offset(path[swb as usize][cb as usize].run as isize)
                        as libc::c_int
                        != *(run_value_bits
                            [((*sce).ics.num_windows == 8 as libc::c_int) as libc::c_int as usize])
                            .offset(
                                (path[swb as usize][cb as usize].run + 1 as libc::c_int) as isize,
                            ) as libc::c_int
                    {
                        cost_stay_here += run_bits as libc::c_float;
                    }
                    if cost_get_here < cost_stay_here {
                        path[(swb + 1 as libc::c_int) as usize][cb as usize].prev_idx = mincb;
                        path[(swb + 1 as libc::c_int) as usize][cb as usize].cost = cost_get_here;
                        path[(swb + 1 as libc::c_int) as usize][cb as usize].run = 1 as libc::c_int;
                    } else {
                        path[(swb + 1 as libc::c_int) as usize][cb as usize].prev_idx = cb;
                        path[(swb + 1 as libc::c_int) as usize][cb as usize].cost = cost_stay_here;
                        path[(swb + 1 as libc::c_int) as usize][cb as usize].run =
                            path[swb as usize][cb as usize].run + 1 as libc::c_int;
                    }
                    if path[(swb + 1 as libc::c_int) as usize][cb as usize].cost < next_minrd {
                        next_minrd = path[(swb + 1 as libc::c_int) as usize][cb as usize].cost;
                        next_mincb = cb;
                    }
                }
                cb += 1;
                cb;
            }
        }
        start += *((*sce).ics.swb_sizes).offset(swb as isize) as libc::c_int;
        swb += 1;
        swb;
    }
    stack_len = 0 as libc::c_int;
    idx = 0 as libc::c_int;
    cb = 1 as libc::c_int;
    while cb < 15 as libc::c_int {
        if path[max_sfb as usize][cb as usize].cost < path[max_sfb as usize][idx as usize].cost {
            idx = cb;
        }
        cb += 1;
        cb;
    }
    ppos = max_sfb;
    while ppos > 0 as libc::c_int {
        cb = idx;
        stackrun[stack_len as usize] = path[ppos as usize][cb as usize].run;
        stackcb[stack_len as usize] = cb;
        idx = path[(ppos - path[ppos as usize][cb as usize].run + 1 as libc::c_int) as usize]
            [cb as usize]
            .prev_idx;
        ppos -= path[ppos as usize][cb as usize].run;
        stack_len += 1;
        stack_len;
    }
    start = 0 as libc::c_int;
    i = stack_len - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        cb = aac_cb_out_map[stackcb[i as usize] as usize] as libc::c_int;
        put_bits(&mut (*s).pb, 4 as libc::c_int, cb as BitBuf);
        count = stackrun[i as usize];
        ptr::write_bytes(
            ((*sce).zeroes)
                .as_mut_ptr()
                .offset((win * 16 as libc::c_int) as isize)
                .offset(start as isize),
            u8::from(cb == 0),
            count as usize,
        );
        j = 0 as libc::c_int;
        while j < count {
            (*sce).band_type[(win * 16 as libc::c_int + start) as usize] = cb as BandType;
            start += 1;
            start;
            j += 1;
            j;
        }
        while count >= run_esc {
            put_bits(&mut (*s).pb, run_bits, run_esc as BitBuf);
            count -= run_esc;
        }
        put_bits(&mut (*s).pb, run_bits, count as BitBuf);
        i -= 1;
        i;
    }
}
unsafe extern "C" fn set_special_band_scalefactors(
    mut _s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
) {
    let mut w: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut prevscaler_n: libc::c_int = -(255 as libc::c_int);
    let mut prevscaler_i: libc::c_int = 0 as libc::c_int;
    let mut bands: libc::c_int = 0 as libc::c_int;
    w = 0 as libc::c_int;
    while w < (*sce).ics.num_windows {
        g = 0 as libc::c_int;
        while g < (*sce).ics.num_swb {
            if (*sce).zeroes[(w * 16 as libc::c_int + g) as usize] == 0 {
                if (*sce).band_type[(w * 16 as libc::c_int + g) as usize] as libc::c_uint
                    == INTENSITY_BT as libc::c_int as libc::c_uint
                    || (*sce).band_type[(w * 16 as libc::c_int + g) as usize] as libc::c_uint
                        == INTENSITY_BT2 as libc::c_int as libc::c_uint
                {
                    (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize] = av_clip_c(
                        roundf(
                            log2f((*sce).is_ener[(w * 16 as libc::c_int + g) as usize])
                                * 2 as libc::c_int as libc::c_float,
                        ) as libc::c_int,
                        -(155 as libc::c_int),
                        100 as libc::c_int,
                    );
                    bands += 1;
                    bands;
                } else if (*sce).band_type[(w * 16 as libc::c_int + g) as usize] as libc::c_uint
                    == NOISE_BT as libc::c_int as libc::c_uint
                {
                    (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize] = av_clip_c(
                        (3 as libc::c_int as libc::c_float
                            + ceilf(
                                log2f((*sce).pns_ener[(w * 16 as libc::c_int + g) as usize])
                                    * 2 as libc::c_int as libc::c_float,
                            )) as libc::c_int,
                        -(100 as libc::c_int),
                        155 as libc::c_int,
                    );
                    if prevscaler_n == -(255 as libc::c_int) {
                        prevscaler_n = (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize];
                    }
                    bands += 1;
                    bands;
                }
            }
            g += 1;
            g;
        }
        w += (*sce).ics.group_len[w as usize] as libc::c_int;
    }
    if bands == 0 {
        return;
    }
    w = 0 as libc::c_int;
    while w < (*sce).ics.num_windows {
        g = 0 as libc::c_int;
        while g < (*sce).ics.num_swb {
            if (*sce).zeroes[(w * 16 as libc::c_int + g) as usize] == 0 {
                if (*sce).band_type[(w * 16 as libc::c_int + g) as usize] as libc::c_uint
                    == INTENSITY_BT as libc::c_int as libc::c_uint
                    || (*sce).band_type[(w * 16 as libc::c_int + g) as usize] as libc::c_uint
                        == INTENSITY_BT2 as libc::c_int as libc::c_uint
                {
                    prevscaler_i = av_clip_c(
                        (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize],
                        prevscaler_i - 60 as libc::c_int,
                        prevscaler_i + 60 as libc::c_int,
                    );
                    (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize] = prevscaler_i;
                } else if (*sce).band_type[(w * 16 as libc::c_int + g) as usize] as libc::c_uint
                    == NOISE_BT as libc::c_int as libc::c_uint
                {
                    prevscaler_n = av_clip_c(
                        (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize],
                        prevscaler_n - 60 as libc::c_int,
                        prevscaler_n + 60 as libc::c_int,
                    );
                    (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize] = prevscaler_n;
                }
            }
            g += 1;
            g;
        }
        w += (*sce).ics.group_len[w as usize] as libc::c_int;
    }
}
unsafe extern "C" fn search_for_quantizers_anmr(
    mut _avctx: *mut AVCodecContext,
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
    lambda: libc::c_float,
) {
    let mut q: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut w2: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut start: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut paths: [[TrellisPath; 61]; 121] = [[TrellisPath { cost: 0., prev: 0 }; 61]; 121];
    let mut bandaddr: [libc::c_int; 121] = [0; 121];
    let mut minq: libc::c_int = 0;
    let mut mincost: libc::c_float = 0.;
    let mut q0f: libc::c_float = 3.402_823_5e38_f32;
    let mut q1f: libc::c_float = 0.0f32;
    let mut qnrgf: libc::c_float = 0.0f32;
    let mut q0: libc::c_int = 0;
    let mut q1: libc::c_int = 0;
    let mut qcnt: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 1024 as libc::c_int {
        let mut t: libc::c_float = fabsf((*sce).coeffs[i as usize]);
        if t > 0.0f32 {
            q0f = if q0f > t { t } else { q0f };
            q1f = if q1f > t { q1f } else { t };
            qnrgf += t * t;
            qcnt += 1;
            qcnt;
        }
        i += 1;
        i;
    }
    if qcnt == 0 {
        ((*sce).sf_idx).fill(0);
        ((*sce).zeroes).fill(1);
        return;
    }
    q0 = av_clip_c(
        coef2minsf(q0f) as libc::c_int,
        0 as libc::c_int,
        255 as libc::c_int - 1 as libc::c_int,
    );
    q1 = av_clip_c(
        coef2maxsf(q1f) as libc::c_int,
        1 as libc::c_int,
        255 as libc::c_int,
    );
    if q1 - q0 > 60 as libc::c_int {
        let mut q0low: libc::c_int = q0;
        let mut q1high: libc::c_int = q1;
        let mut qnrg: libc::c_int = av_clip_uint8_c(
            (log2f(sqrtf(qnrgf / qcnt as libc::c_float)) * 4 as libc::c_int as libc::c_float
                - 31 as libc::c_int as libc::c_float
                + 140 as libc::c_int as libc::c_float
                - 36 as libc::c_int as libc::c_float) as libc::c_int,
        ) as libc::c_int;
        q1 = qnrg + 30 as libc::c_int;
        q0 = qnrg - 30 as libc::c_int;
        if q0 < q0low {
            q1 += q0low - q0;
            q0 = q0low;
        } else if q1 > q1high {
            q0 -= q1 - q1high;
            q1 = q1high;
        }
    }
    if q0 == q1 {
        q1 = av_clip_c(q0 + 1 as libc::c_int, 1 as libc::c_int, 255 as libc::c_int);
        q0 = av_clip_c(
            q1 - 1 as libc::c_int,
            0 as libc::c_int,
            255 as libc::c_int - 1 as libc::c_int,
        );
    }
    i = 0 as libc::c_int;
    while i < 60 as libc::c_int + 1 as libc::c_int {
        paths[0 as libc::c_int as usize][i as usize].cost = 0.0f32;
        paths[0 as libc::c_int as usize][i as usize].prev = -(1 as libc::c_int);
        i += 1;
        i;
    }
    j = 1 as libc::c_int;
    while j < 121 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 60 as libc::c_int + 1 as libc::c_int {
            paths[j as usize][i as usize].cost = ::core::f32::INFINITY;
            paths[j as usize][i as usize].prev = -(2 as libc::c_int);
            i += 1;
            i;
        }
        j += 1;
        j;
    }
    idx = 1 as libc::c_int;
    ((*s).abs_pow34).expect("non-null function pointer")(
        ((*s).scoefs).as_mut_ptr(),
        ((*sce).coeffs).as_mut_ptr(),
        1024 as libc::c_int,
    );
    w = 0 as libc::c_int;
    while w < (*sce).ics.num_windows {
        start = w * 128 as libc::c_int;
        g = 0 as libc::c_int;
        while g < (*sce).ics.num_swb {
            let mut coefs: *const libc::c_float =
                &mut *((*sce).coeffs).as_mut_ptr().offset(start as isize) as *mut INTFLOAT;
            let mut qmin: libc::c_float = 0.;
            let mut qmax: libc::c_float = 0.;
            let mut nz: libc::c_int = 0 as libc::c_int;
            bandaddr[idx as usize] = w * 16 as libc::c_int + g;
            qmin = 2147483647 as libc::c_int as libc::c_float;
            qmax = 0.0f32;
            w2 = 0 as libc::c_int;
            while w2 < (*sce).ics.group_len[w as usize] as libc::c_int {
                let mut band: *mut FFPsyBand =
                    &mut *((*((*s).psy.ch).offset((*s).cur_channel as isize)).psy_bands)
                        .as_mut_ptr()
                        .offset(((w + w2) * 16 as libc::c_int + g) as isize)
                        as *mut FFPsyBand;
                if (*band).energy <= (*band).threshold || (*band).threshold == 0.0f32 {
                    (*sce).zeroes[((w + w2) * 16 as libc::c_int + g) as usize] =
                        1 as libc::c_int as uint8_t;
                } else {
                    (*sce).zeroes[((w + w2) * 16 as libc::c_int + g) as usize] =
                        0 as libc::c_int as uint8_t;
                    nz = 1 as libc::c_int;
                    i = 0 as libc::c_int;
                    while i < *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int {
                        let mut t_0: libc::c_float =
                            fabsf(*coefs.offset((w2 * 128 as libc::c_int + i) as isize));
                        if t_0 > 0.0f32 {
                            qmin = if qmin > t_0 { t_0 } else { qmin };
                        }
                        qmax = if qmax > t_0 { qmax } else { t_0 };
                        i += 1;
                        i;
                    }
                }
                w2 += 1;
                w2;
            }
            if nz != 0 {
                let mut minscale: libc::c_int = 0;
                let mut maxscale: libc::c_int = 0;
                let mut minrd: libc::c_float = ::core::f32::INFINITY;
                let mut maxval: libc::c_float = 0.;
                minscale = coef2minsf(qmin) as libc::c_int;
                maxscale = coef2maxsf(qmax) as libc::c_int;
                minscale = av_clip_c(
                    minscale - q0,
                    0 as libc::c_int,
                    60 as libc::c_int + 1 as libc::c_int - 1 as libc::c_int,
                );
                maxscale = av_clip_c(
                    maxscale - q0,
                    0 as libc::c_int,
                    60 as libc::c_int + 1 as libc::c_int,
                );
                if minscale == maxscale {
                    maxscale = av_clip_c(
                        minscale + 1 as libc::c_int,
                        1 as libc::c_int,
                        60 as libc::c_int + 1 as libc::c_int,
                    );
                    minscale = av_clip_c(
                        maxscale - 1 as libc::c_int,
                        0 as libc::c_int,
                        60 as libc::c_int + 1 as libc::c_int - 1 as libc::c_int,
                    );
                }
                maxval = find_max_val(
                    (*sce).ics.group_len[w as usize] as libc::c_int,
                    *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int,
                    ((*s).scoefs).as_mut_ptr().offset(start as isize),
                );
                q = minscale;
                while q < maxscale {
                    let mut dist: libc::c_float = 0 as libc::c_int as libc::c_float;
                    let mut cb: libc::c_int =
                        find_min_book(maxval, (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize]);
                    w2 = 0 as libc::c_int;
                    while w2 < (*sce).ics.group_len[w as usize] as libc::c_int {
                        let mut band_0: *mut FFPsyBand =
                            &mut *((*((*s).psy.ch).offset((*s).cur_channel as isize)).psy_bands)
                                .as_mut_ptr()
                                .offset(((w + w2) * 16 as libc::c_int + g) as isize)
                                as *mut FFPsyBand;
                        dist += quantize_band_cost(
                            s,
                            coefs.offset((w2 * 128 as libc::c_int) as isize),
                            ((*s).scoefs)
                                .as_mut_ptr()
                                .offset(start as isize)
                                .offset((w2 * 128 as libc::c_int) as isize),
                            *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int,
                            q + q0,
                            cb,
                            lambda / (*band_0).threshold,
                            ::core::f32::INFINITY,
                            std::ptr::null_mut::<libc::c_int>(),
                            std::ptr::null_mut::<libc::c_float>(),
                        );
                        w2 += 1;
                        w2;
                    }
                    minrd = if minrd > dist { dist } else { minrd };
                    i = 0 as libc::c_int;
                    while i < q1 - q0 {
                        let mut cost: libc::c_float = 0.;
                        cost = paths[(idx - 1 as libc::c_int) as usize][i as usize].cost
                            + dist
                            + ff_aac_scalefactor_bits[(q - i + 60 as libc::c_int) as usize]
                                as libc::c_int as libc::c_float;
                        if cost < paths[idx as usize][q as usize].cost {
                            paths[idx as usize][q as usize].cost = cost;
                            paths[idx as usize][q as usize].prev = i;
                        }
                        i += 1;
                        i;
                    }
                    q += 1;
                    q;
                }
            } else {
                q = 0 as libc::c_int;
                while q < q1 - q0 {
                    paths[idx as usize][q as usize].cost =
                        paths[(idx - 1 as libc::c_int) as usize][q as usize].cost
                            + 1 as libc::c_int as libc::c_float;
                    paths[idx as usize][q as usize].prev = q;
                    q += 1;
                    q;
                }
            }
            (*sce).zeroes[(w * 16 as libc::c_int + g) as usize] =
                (nz == 0) as libc::c_int as uint8_t;
            start += *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int;
            idx += 1;
            idx;
            g += 1;
            g;
        }
        w += (*sce).ics.group_len[w as usize] as libc::c_int;
    }
    idx -= 1;
    idx;
    mincost = paths[idx as usize][0 as libc::c_int as usize].cost;
    minq = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < 60 as libc::c_int + 1 as libc::c_int {
        if paths[idx as usize][i as usize].cost < mincost {
            mincost = paths[idx as usize][i as usize].cost;
            minq = i;
        }
        i += 1;
        i;
    }
    while idx != 0 {
        (*sce).sf_idx[bandaddr[idx as usize] as usize] = minq + q0;
        minq = if paths[idx as usize][minq as usize].prev > 0 as libc::c_int {
            paths[idx as usize][minq as usize].prev
        } else {
            0 as libc::c_int
        };
        idx -= 1;
        idx;
    }
    w = 0 as libc::c_int;
    while w < (*sce).ics.num_windows {
        g = 0 as libc::c_int;
        while g < (*sce).ics.num_swb {
            w2 = 1 as libc::c_int;
            while w2 < (*sce).ics.group_len[w as usize] as libc::c_int {
                (*sce).sf_idx[((w + w2) * 16 as libc::c_int + g) as usize] =
                    (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize];
                w2 += 1;
                w2;
            }
            g += 1;
            g;
        }
        w += (*sce).ics.group_len[w as usize] as libc::c_int;
    }
}
unsafe extern "C" fn search_for_quantizers_fast(
    mut avctx: *mut AVCodecContext,
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
    lambda: libc::c_float,
) {
    let mut start: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut w2: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut destbits: libc::c_int = ((*avctx).bit_rate as libc::c_double * 1024.0f64
        / (*avctx).sample_rate as libc::c_double
        / (*avctx).ch_layout.nb_channels as libc::c_double
        * (lambda / 120.0f32) as libc::c_double) as libc::c_int;
    let mut dists: [libc::c_float; 128] = [
        0 as libc::c_int as libc::c_float,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ];
    let mut uplims: [libc::c_float; 128] = [
        0 as libc::c_int as libc::c_float,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ];
    let mut maxvals: [libc::c_float; 128] = [0.; 128];
    let mut fflag: libc::c_int = 0;
    let mut minscaler: libc::c_int = 0;
    let mut its: libc::c_int = 0 as libc::c_int;
    let mut allz: libc::c_int = 0 as libc::c_int;
    let mut minthr: libc::c_float = ::core::f32::INFINITY;
    destbits = if destbits > 5800 as libc::c_int {
        5800 as libc::c_int
    } else {
        destbits
    };
    w = 0 as libc::c_int;
    while w < (*sce).ics.num_windows {
        start = 0 as libc::c_int;
        g = 0 as libc::c_int;
        while g < (*sce).ics.num_swb {
            let mut nz: libc::c_int = 0 as libc::c_int;
            let mut uplim: libc::c_float = 0.0f32;
            w2 = 0 as libc::c_int;
            while w2 < (*sce).ics.group_len[w as usize] as libc::c_int {
                let mut band: *mut FFPsyBand =
                    &mut *((*((*s).psy.ch).offset((*s).cur_channel as isize)).psy_bands)
                        .as_mut_ptr()
                        .offset(((w + w2) * 16 as libc::c_int + g) as isize)
                        as *mut FFPsyBand;
                uplim += (*band).threshold;
                if (*band).energy <= (*band).threshold || (*band).threshold == 0.0f32 {
                    (*sce).zeroes[((w + w2) * 16 as libc::c_int + g) as usize] =
                        1 as libc::c_int as uint8_t;
                } else {
                    nz = 1 as libc::c_int;
                }
                w2 += 1;
                w2;
            }
            uplims[(w * 16 as libc::c_int + g) as usize] =
                uplim * 512 as libc::c_int as libc::c_float;
            (*sce).band_type[(w * 16 as libc::c_int + g) as usize] = ZERO_BT;
            (*sce).zeroes[(w * 16 as libc::c_int + g) as usize] =
                (nz == 0) as libc::c_int as uint8_t;
            if nz != 0 {
                minthr = if minthr > uplim { uplim } else { minthr };
            }
            allz |= nz;
            start += *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int;
            g += 1;
            g;
        }
        w += (*sce).ics.group_len[w as usize] as libc::c_int;
    }
    w = 0 as libc::c_int;
    while w < (*sce).ics.num_windows {
        g = 0 as libc::c_int;
        while g < (*sce).ics.num_swb {
            if (*sce).zeroes[(w * 16 as libc::c_int + g) as usize] != 0 {
                (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize] = 140 as libc::c_int;
            } else {
                (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize] =
                    (140 as libc::c_int as libc::c_float
                        + (if log2f(uplims[(w * 16 as libc::c_int + g) as usize] / minthr)
                            * 4 as libc::c_int as libc::c_float
                            > 59 as libc::c_int as libc::c_float
                        {
                            59 as libc::c_int as libc::c_float
                        } else {
                            log2f(uplims[(w * 16 as libc::c_int + g) as usize] / minthr)
                                * 4 as libc::c_int as libc::c_float
                        })) as libc::c_int;
            }
            g += 1;
            g;
        }
        w += (*sce).ics.group_len[w as usize] as libc::c_int;
    }
    if allz == 0 {
        return;
    }
    ((*s).abs_pow34).expect("non-null function pointer")(
        ((*s).scoefs).as_mut_ptr(),
        ((*sce).coeffs).as_mut_ptr(),
        1024 as libc::c_int,
    );
    ff_quantize_band_cost_cache_init(s);
    w = 0 as libc::c_int;
    while w < (*sce).ics.num_windows {
        start = w * 128 as libc::c_int;
        g = 0 as libc::c_int;
        while g < (*sce).ics.num_swb {
            let mut scaled: *const libc::c_float =
                ((*s).scoefs).as_mut_ptr().offset(start as isize);
            maxvals[(w * 16 as libc::c_int + g) as usize] = find_max_val(
                (*sce).ics.group_len[w as usize] as libc::c_int,
                *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int,
                scaled,
            );
            start += *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int;
            g += 1;
            g;
        }
        w += (*sce).ics.group_len[w as usize] as libc::c_int;
    }
    loop {
        let mut tbits: libc::c_int = 0;
        let mut qstep: libc::c_int = 0;
        minscaler = (*sce).sf_idx[0 as libc::c_int as usize];
        qstep = if its != 0 {
            1 as libc::c_int
        } else {
            32 as libc::c_int
        };
        loop {
            let mut prev: libc::c_int = -(1 as libc::c_int);
            tbits = 0 as libc::c_int;
            w = 0 as libc::c_int;
            while w < (*sce).ics.num_windows {
                start = w * 128 as libc::c_int;
                g = 0 as libc::c_int;
                while g < (*sce).ics.num_swb {
                    let mut coefs: *const libc::c_float =
                        ((*sce).coeffs).as_mut_ptr().offset(start as isize);
                    let mut scaled_0: *const libc::c_float =
                        ((*s).scoefs).as_mut_ptr().offset(start as isize);
                    let mut bits: libc::c_int = 0 as libc::c_int;
                    let mut cb: libc::c_int = 0;
                    let mut dist: libc::c_float = 0.0f32;
                    if (*sce).zeroes[(w * 16 as libc::c_int + g) as usize] as libc::c_int != 0
                        || (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize] >= 218 as libc::c_int
                    {
                        start += *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int;
                    } else {
                        minscaler =
                            if minscaler > (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize] {
                                (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize]
                            } else {
                                minscaler
                            };
                        cb = find_min_book(
                            maxvals[(w * 16 as libc::c_int + g) as usize],
                            (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize],
                        );
                        w2 = 0 as libc::c_int;
                        while w2 < (*sce).ics.group_len[w as usize] as libc::c_int {
                            let mut b: libc::c_int = 0;
                            dist += quantize_band_cost_cached(
                                s,
                                w + w2,
                                g,
                                coefs.offset((w2 * 128 as libc::c_int) as isize),
                                scaled_0.offset((w2 * 128 as libc::c_int) as isize),
                                *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int,
                                (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize],
                                cb,
                                1.0f32,
                                ::core::f32::INFINITY,
                                &mut b,
                                std::ptr::null_mut::<libc::c_float>(),
                                0 as libc::c_int,
                            );
                            bits += b;
                            w2 += 1;
                            w2;
                        }
                        dists[(w * 16 as libc::c_int + g) as usize] = dist - bits as libc::c_float;
                        if prev != -(1 as libc::c_int) {
                            bits += ff_aac_scalefactor_bits[((*sce).sf_idx
                                [(w * 16 as libc::c_int + g) as usize]
                                - prev
                                + 60 as libc::c_int)
                                as usize] as libc::c_int;
                        }
                        tbits += bits;
                        start += *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int;
                        prev = (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize];
                    }
                    g += 1;
                    g;
                }
                w += (*sce).ics.group_len[w as usize] as libc::c_int;
            }
            if tbits > destbits {
                i = 0 as libc::c_int;
                while i < 128 as libc::c_int {
                    if (*sce).sf_idx[i as usize] < 218 as libc::c_int - qstep {
                        (*sce).sf_idx[i as usize] += qstep;
                    }
                    i += 1;
                    i;
                }
            } else {
                i = 0 as libc::c_int;
                while i < 128 as libc::c_int {
                    if (*sce).sf_idx[i as usize] > 60 as libc::c_int - qstep {
                        (*sce).sf_idx[i as usize] -= qstep;
                    }
                    i += 1;
                    i;
                }
            }
            qstep >>= 1 as libc::c_int;
            if qstep == 0
                && tbits as libc::c_double > destbits as libc::c_double * 1.02f64
                && (*sce).sf_idx[0 as libc::c_int as usize] < 217 as libc::c_int
            {
                qstep = 1 as libc::c_int;
            }
            if qstep == 0 {
                break;
            }
        }
        fflag = 0 as libc::c_int;
        minscaler = av_clip_c(
            minscaler,
            60 as libc::c_int,
            255 as libc::c_int - 60 as libc::c_int,
        );
        w = 0 as libc::c_int;
        while w < (*sce).ics.num_windows {
            g = 0 as libc::c_int;
            while g < (*sce).ics.num_swb {
                let mut prevsc: libc::c_int = (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize];
                if dists[(w * 16 as libc::c_int + g) as usize]
                    > uplims[(w * 16 as libc::c_int + g) as usize]
                    && (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize] > 60 as libc::c_int
                {
                    if find_min_book(
                        maxvals[(w * 16 as libc::c_int + g) as usize],
                        (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize] - 1 as libc::c_int,
                    ) != 0
                    {
                        (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize] -= 1;
                        (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize];
                    } else {
                        (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize] -= 2 as libc::c_int;
                    }
                }
                (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize] = av_clip_c(
                    (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize],
                    minscaler,
                    minscaler + 60 as libc::c_int,
                );
                (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize] =
                    if (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize] > 219 as libc::c_int {
                        219 as libc::c_int
                    } else {
                        (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize]
                    };
                if (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize] != prevsc {
                    fflag = 1 as libc::c_int;
                }
                (*sce).band_type[(w * 16 as libc::c_int + g) as usize] = find_min_book(
                    maxvals[(w * 16 as libc::c_int + g) as usize],
                    (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize],
                )
                    as BandType;
                g += 1;
                g;
            }
            w += (*sce).ics.group_len[w as usize] as libc::c_int;
        }
        its += 1;
        its;
        if !(fflag != 0 && its < 10 as libc::c_int) {
            break;
        }
    }
}
unsafe extern "C" fn search_for_pns(
    mut s: *mut AACEncContext,
    mut avctx: *mut AVCodecContext,
    mut sce: *mut SingleChannelElement,
) {
    let mut band: *mut FFPsyBand = std::ptr::null_mut::<FFPsyBand>();
    let mut w: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut w2: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut wlen: libc::c_int = 1024 as libc::c_int / (*sce).ics.num_windows;
    let mut bandwidth: libc::c_int = 0;
    let mut cutoff: libc::c_int = 0;
    let mut PNS: *mut libc::c_float = &mut *((*s).scoefs)
        .as_mut_ptr()
        .offset((0 as libc::c_int * 128 as libc::c_int) as isize)
        as *mut libc::c_float;
    let mut PNS34: *mut libc::c_float = &mut *((*s).scoefs)
        .as_mut_ptr()
        .offset((1 as libc::c_int * 128 as libc::c_int) as isize)
        as *mut libc::c_float;
    let mut NOR34: *mut libc::c_float = &mut *((*s).scoefs)
        .as_mut_ptr()
        .offset((3 as libc::c_int * 128 as libc::c_int) as isize)
        as *mut libc::c_float;
    let mut nextband: [uint8_t; 128] = [0; 128];
    let lambda: libc::c_float = (*s).lambda;
    let freq_mult: libc::c_float =
        (*avctx).sample_rate as libc::c_float * 0.5f32 / wlen as libc::c_float;
    let thr_mult: libc::c_float = 1.948f32 * (100.0f32 / lambda);
    let spread_threshold: libc::c_float = if 0.75f32
        > 0.9f32
            * (if 0.5f32 > lambda / 100.0f32 {
                0.5f32
            } else {
                lambda / 100.0f32
            }) {
        0.9f32
            * (if 0.5f32 > lambda / 100.0f32 {
                0.5f32
            } else {
                lambda / 100.0f32
            })
    } else {
        0.75f32
    };
    let dist_bias: libc::c_float = av_clipf_c(
        4.0f32 * 120 as libc::c_int as libc::c_float / lambda,
        0.25f32,
        4.0f32,
    );
    let pns_transient_energy_r: libc::c_float = if 0.7f32 > lambda / 140.0f32 {
        lambda / 140.0f32
    } else {
        0.7f32
    };
    let mut refbits: libc::c_int = ((*avctx).bit_rate as libc::c_double * 1024.0f64
        / (*avctx).sample_rate as libc::c_double
        / (if (*avctx).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
            2.0f32
        } else {
            (*avctx).ch_layout.nb_channels as libc::c_float
        }) as libc::c_double
        * (lambda / 120.0f32) as libc::c_double) as libc::c_int;
    let mut rate_bandwidth_multiplier: libc::c_float = 1.5f32;
    let mut prev: libc::c_int = -(1000 as libc::c_int);
    let mut prev_sf: libc::c_int = -(1 as libc::c_int);
    let mut frame_bit_rate: libc::c_int = (if (*avctx).flags
        & (1 as libc::c_int) << 1 as libc::c_int
        != 0
    {
        refbits as libc::c_float * rate_bandwidth_multiplier * (*avctx).sample_rate as libc::c_float
            / 1024 as libc::c_int as libc::c_float
    } else {
        ((*avctx).bit_rate / (*avctx).ch_layout.nb_channels as libc::c_long) as libc::c_float
    }) as libc::c_int;
    frame_bit_rate = (frame_bit_rate as libc::c_float * 1.15f32) as libc::c_int;
    if (*avctx).cutoff > 0 as libc::c_int {
        bandwidth = (*avctx).cutoff;
    } else {
        bandwidth = if 3000 as libc::c_int
            > (if frame_bit_rate != 0 {
                if (if (if (if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 3000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                {
                    3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 12000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
                {
                    12000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
                } else if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 3000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                {
                    3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 22000 as libc::c_int
                {
                    22000 as libc::c_int
                } else if (if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 3000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                {
                    3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 12000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
                {
                    12000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
                } else if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 3000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                {
                    3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > (*avctx).sample_rate / 2 as libc::c_int
                {
                    (*avctx).sample_rate / 2 as libc::c_int
                } else if (if (if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 3000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                {
                    3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 12000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
                {
                    12000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
                } else if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 3000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                {
                    3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 22000 as libc::c_int
                {
                    22000 as libc::c_int
                } else if (if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 3000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                {
                    3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 12000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
                {
                    12000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
                } else if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 3000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                {
                    3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }
            } else {
                (*avctx).sample_rate / 2 as libc::c_int
            }) {
            3000 as libc::c_int
        } else if frame_bit_rate != 0 {
            if (if (if (if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 3000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            {
                3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 12000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
            {
                12000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
            } else if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 3000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            {
                3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 22000 as libc::c_int
            {
                22000 as libc::c_int
            } else if (if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 3000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            {
                3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 12000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
            {
                12000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
            } else if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 3000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            {
                3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > (*avctx).sample_rate / 2 as libc::c_int
            {
                (*avctx).sample_rate / 2 as libc::c_int
            } else if (if (if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 3000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            {
                3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 12000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
            {
                12000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
            } else if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 3000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            {
                3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 22000 as libc::c_int
            {
                22000 as libc::c_int
            } else if (if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 3000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            {
                3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 12000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
            {
                12000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
            } else if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 3000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            {
                3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }
        } else {
            (*avctx).sample_rate / 2 as libc::c_int
        };
    }
    cutoff = bandwidth * 2 as libc::c_int * wlen / (*avctx).sample_rate;
    (*sce).band_alt = (*sce).band_type;
    ff_init_nextband_map(sce, nextband.as_mut_ptr());
    w = 0 as libc::c_int;
    while w < (*sce).ics.num_windows {
        let mut wstart: libc::c_int = w * 128 as libc::c_int;
        let mut current_block_67: u64;
        g = 0 as libc::c_int;
        while g < (*sce).ics.num_swb {
            let mut noise_sfi: libc::c_int = 0;
            let mut dist1: libc::c_float = 0.0f32;
            let mut dist2: libc::c_float = 0.0f32;
            let mut noise_amp: libc::c_float = 0.;
            let mut pns_energy: libc::c_float = 0.0f32;
            let mut pns_tgt_energy: libc::c_float = 0.;
            let mut energy_ratio: libc::c_float = 0.;
            let mut dist_thresh: libc::c_float = 0.;
            let mut sfb_energy: libc::c_float = 0.0f32;
            let mut threshold: libc::c_float = 0.0f32;
            let mut spread: libc::c_float = 2.0f32;
            let mut min_energy: libc::c_float = -1.0f32;
            let mut max_energy: libc::c_float = 0.0f32;
            let start: libc::c_int =
                wstart + *((*sce).ics.swb_offset).offset(g as isize) as libc::c_int;
            let freq: libc::c_float = (start - wstart) as libc::c_float * freq_mult;
            let freq_boost: libc::c_float =
                if 0.88f32 * freq / 4000 as libc::c_int as libc::c_float > 1.0f32 {
                    0.88f32 * freq / 4000 as libc::c_int as libc::c_float
                } else {
                    1.0f32
                };
            if freq < 4000 as libc::c_int as libc::c_float || start - wstart >= cutoff {
                if (*sce).zeroes[(w * 16 as libc::c_int + g) as usize] == 0 {
                    prev_sf = (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize];
                }
            } else {
                w2 = 0 as libc::c_int;
                while w2 < (*sce).ics.group_len[w as usize] as libc::c_int {
                    band = &mut *((*((*s).psy.ch).offset((*s).cur_channel as isize)).psy_bands)
                        .as_mut_ptr()
                        .offset(((w + w2) * 16 as libc::c_int + g) as isize)
                        as *mut FFPsyBand;
                    sfb_energy += (*band).energy;
                    spread = if spread > (*band).spread {
                        (*band).spread
                    } else {
                        spread
                    };
                    threshold += (*band).threshold;
                    if w2 == 0 {
                        max_energy = (*band).energy;
                        min_energy = max_energy;
                    } else {
                        min_energy = if min_energy > (*band).energy {
                            (*band).energy
                        } else {
                            min_energy
                        };
                        max_energy = if max_energy > (*band).energy {
                            max_energy
                        } else {
                            (*band).energy
                        };
                    }
                    w2 += 1;
                    w2;
                }
                dist_thresh = av_clipf_c(
                    2.5f32 * 4000 as libc::c_int as libc::c_float / freq,
                    0.5f32,
                    2.5f32,
                ) * dist_bias;
                if (*sce).zeroes[(w * 16 as libc::c_int + g) as usize] == 0
                    && ff_sfdelta_can_remove_band(
                        sce,
                        nextband.as_mut_ptr(),
                        prev_sf,
                        w * 16 as libc::c_int + g,
                    ) == 0
                    || ((*sce).zeroes[(w * 16 as libc::c_int + g) as usize] as libc::c_int != 0
                        || (*sce).band_alt[(w * 16 as libc::c_int + g) as usize] as u64 == 0)
                        && sfb_energy < threshold * sqrtf(1.0f32 / freq_boost)
                    || spread < spread_threshold
                    || (*sce).zeroes[(w * 16 as libc::c_int + g) as usize] == 0
                        && (*sce).band_alt[(w * 16 as libc::c_int + g) as usize] as libc::c_uint
                            != 0
                        && sfb_energy > threshold * thr_mult * freq_boost
                    || min_energy < pns_transient_energy_r * max_energy
                {
                    (*sce).pns_ener[(w * 16 as libc::c_int + g) as usize] = sfb_energy;
                    if (*sce).zeroes[(w * 16 as libc::c_int + g) as usize] == 0 {
                        prev_sf = (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize];
                    }
                } else {
                    pns_tgt_energy = sfb_energy
                        * (if 1.0f32 > spread * spread {
                            spread * spread
                        } else {
                            1.0f32
                        });
                    noise_sfi = av_clip_c(
                        roundf(log2f(pns_tgt_energy) * 2 as libc::c_int as libc::c_float)
                            as libc::c_int,
                        -(100 as libc::c_int),
                        155 as libc::c_int,
                    );
                    noise_amp = -ff_aac_pow2sf_tab[(noise_sfi + 200 as libc::c_int) as usize];
                    if prev != -(1000 as libc::c_int) {
                        let mut noise_sfdiff: libc::c_int = noise_sfi - prev + 60 as libc::c_int;
                        if noise_sfdiff < 0 as libc::c_int
                            || noise_sfdiff > 2 as libc::c_int * 60 as libc::c_int
                        {
                            if (*sce).zeroes[(w * 16 as libc::c_int + g) as usize] == 0 {
                                prev_sf = (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize];
                            }
                            current_block_67 = 1054647088692577877;
                        } else {
                            current_block_67 = 1847472278776910194;
                        }
                    } else {
                        current_block_67 = 1847472278776910194;
                    }
                    match current_block_67 {
                        1054647088692577877 => {}
                        _ => {
                            w2 = 0 as libc::c_int;
                            while w2 < (*sce).ics.group_len[w as usize] as libc::c_int {
                                let mut band_energy: libc::c_float = 0.;
                                let mut scale: libc::c_float = 0.;
                                let mut pns_senergy: libc::c_float = 0.;
                                let start_c: libc::c_int = (w + w2) * 128 as libc::c_int
                                    + *((*sce).ics.swb_offset).offset(g as isize) as libc::c_int;
                                band = &mut *((*((*s).psy.ch).offset((*s).cur_channel as isize))
                                    .psy_bands)
                                    .as_mut_ptr()
                                    .offset(((w + w2) * 16 as libc::c_int + g) as isize)
                                    as *mut FFPsyBand;
                                i = 0 as libc::c_int;
                                while i < *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int
                                {
                                    (*s).random_state =
                                        lcg_random((*s).random_state as libc::c_uint);
                                    *PNS.offset(i as isize) = (*s).random_state as libc::c_float;
                                    i += 1;
                                    i;
                                }
                                band_energy = ((*(*s).fdsp).scalarproduct_float)
                                    .expect("non-null function pointer")(
                                    PNS,
                                    PNS,
                                    *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int,
                                );
                                scale = noise_amp / sqrtf(band_energy);
                                ((*(*s).fdsp).vector_fmul_scalar)
                                    .expect("non-null function pointer")(
                                    PNS,
                                    PNS,
                                    scale,
                                    *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int,
                                );
                                pns_senergy = ((*(*s).fdsp).scalarproduct_float)
                                    .expect("non-null function pointer")(
                                    PNS,
                                    PNS,
                                    *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int,
                                );
                                pns_energy += pns_senergy;
                                ((*s).abs_pow34).expect("non-null function pointer")(
                                    NOR34,
                                    &mut *((*sce).coeffs).as_mut_ptr().offset(start_c as isize),
                                    *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int,
                                );
                                ((*s).abs_pow34).expect("non-null function pointer")(
                                    PNS34,
                                    PNS,
                                    *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int,
                                );
                                dist1 += quantize_band_cost(
                                    s,
                                    &mut *((*sce).coeffs).as_mut_ptr().offset(start_c as isize),
                                    NOR34,
                                    *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int,
                                    (*sce).sf_idx[((w + w2) * 16 as libc::c_int + g) as usize],
                                    (*sce).band_alt[((w + w2) * 16 as libc::c_int + g) as usize]
                                        as libc::c_int,
                                    lambda / (*band).threshold,
                                    ::core::f32::INFINITY,
                                    std::ptr::null_mut::<libc::c_int>(),
                                    std::ptr::null_mut::<libc::c_float>(),
                                );
                                dist2 += (*band).energy / ((*band).spread * (*band).spread)
                                    * lambda
                                    * dist_thresh
                                    / (*band).threshold;
                                w2 += 1;
                                w2;
                            }
                            if g != 0
                                && (*sce).band_type
                                    [(w * 16 as libc::c_int + g - 1 as libc::c_int) as usize]
                                    as libc::c_uint
                                    == NOISE_BT as libc::c_int as libc::c_uint
                            {
                                dist2 += 5 as libc::c_int as libc::c_float;
                            } else {
                                dist2 += 9 as libc::c_int as libc::c_float;
                            }
                            energy_ratio = pns_tgt_energy / pns_energy;
                            (*sce).pns_ener[(w * 16 as libc::c_int + g) as usize] =
                                energy_ratio * pns_tgt_energy;
                            if (*sce).zeroes[(w * 16 as libc::c_int + g) as usize] as libc::c_int
                                != 0
                                || (*sce).band_alt[(w * 16 as libc::c_int + g) as usize] as u64 == 0
                                || energy_ratio > 0.85f32 && energy_ratio < 1.25f32 && dist2 < dist1
                            {
                                (*sce).band_type[(w * 16 as libc::c_int + g) as usize] = NOISE_BT;
                                (*sce).zeroes[(w * 16 as libc::c_int + g) as usize] =
                                    0 as libc::c_int as uint8_t;
                                prev = noise_sfi;
                            } else if (*sce).zeroes[(w * 16 as libc::c_int + g) as usize] == 0 {
                                prev_sf = (*sce).sf_idx[(w * 16 as libc::c_int + g) as usize];
                            }
                        }
                    }
                }
            }
            g += 1;
            g;
        }
        w += (*sce).ics.group_len[w as usize] as libc::c_int;
    }
}
unsafe extern "C" fn mark_pns(
    mut s: *mut AACEncContext,
    mut avctx: *mut AVCodecContext,
    mut sce: *mut SingleChannelElement,
) {
    let mut band: *mut FFPsyBand = std::ptr::null_mut::<FFPsyBand>();
    let mut w: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut w2: libc::c_int = 0;
    let mut wlen: libc::c_int = 1024 as libc::c_int / (*sce).ics.num_windows;
    let mut bandwidth: libc::c_int = 0;
    let mut cutoff: libc::c_int = 0;
    let lambda: libc::c_float = (*s).lambda;
    let freq_mult: libc::c_float =
        (*avctx).sample_rate as libc::c_float * 0.5f32 / wlen as libc::c_float;
    let spread_threshold: libc::c_float = if 0.75f32
        > 0.9f32
            * (if 0.5f32 > lambda / 100.0f32 {
                0.5f32
            } else {
                lambda / 100.0f32
            }) {
        0.9f32
            * (if 0.5f32 > lambda / 100.0f32 {
                0.5f32
            } else {
                lambda / 100.0f32
            })
    } else {
        0.75f32
    };
    let pns_transient_energy_r: libc::c_float = if 0.7f32 > lambda / 140.0f32 {
        lambda / 140.0f32
    } else {
        0.7f32
    };
    let mut refbits: libc::c_int = ((*avctx).bit_rate as libc::c_double * 1024.0f64
        / (*avctx).sample_rate as libc::c_double
        / (if (*avctx).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
            2.0f32
        } else {
            (*avctx).ch_layout.nb_channels as libc::c_float
        }) as libc::c_double
        * (lambda / 120.0f32) as libc::c_double) as libc::c_int;
    let mut rate_bandwidth_multiplier: libc::c_float = 1.5f32;
    let mut frame_bit_rate: libc::c_int = (if (*avctx).flags
        & (1 as libc::c_int) << 1 as libc::c_int
        != 0
    {
        refbits as libc::c_float * rate_bandwidth_multiplier * (*avctx).sample_rate as libc::c_float
            / 1024 as libc::c_int as libc::c_float
    } else {
        ((*avctx).bit_rate / (*avctx).ch_layout.nb_channels as libc::c_long) as libc::c_float
    }) as libc::c_int;
    frame_bit_rate = (frame_bit_rate as libc::c_float * 1.15f32) as libc::c_int;
    if (*avctx).cutoff > 0 as libc::c_int {
        bandwidth = (*avctx).cutoff;
    } else {
        bandwidth = if 3000 as libc::c_int
            > (if frame_bit_rate != 0 {
                if (if (if (if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 3000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                {
                    3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 12000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
                {
                    12000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
                } else if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 3000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                {
                    3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 22000 as libc::c_int
                {
                    22000 as libc::c_int
                } else if (if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 3000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                {
                    3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 12000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
                {
                    12000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
                } else if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 3000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                {
                    3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > (*avctx).sample_rate / 2 as libc::c_int
                {
                    (*avctx).sample_rate / 2 as libc::c_int
                } else if (if (if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 3000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                {
                    3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 12000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
                {
                    12000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
                } else if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 3000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                {
                    3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 22000 as libc::c_int
                {
                    22000 as libc::c_int
                } else if (if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 3000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                {
                    3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 12000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
                {
                    12000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
                } else if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }) > 3000 as libc::c_int
                    + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                {
                    3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
                } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                    > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                {
                    frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                } else {
                    frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                        - 5500 as libc::c_int
                }
            } else {
                (*avctx).sample_rate / 2 as libc::c_int
            }) {
            3000 as libc::c_int
        } else if frame_bit_rate != 0 {
            if (if (if (if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 3000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            {
                3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 12000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
            {
                12000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
            } else if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 3000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            {
                3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 22000 as libc::c_int
            {
                22000 as libc::c_int
            } else if (if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 3000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            {
                3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 12000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
            {
                12000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
            } else if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 3000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            {
                3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > (*avctx).sample_rate / 2 as libc::c_int
            {
                (*avctx).sample_rate / 2 as libc::c_int
            } else if (if (if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 3000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            {
                3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 12000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
            {
                12000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
            } else if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 3000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            {
                3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 22000 as libc::c_int
            {
                22000 as libc::c_int
            } else if (if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 3000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            {
                3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 12000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
            {
                12000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 16 as libc::c_int
            } else if (if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }) > 3000 as libc::c_int
                + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            {
                3000 as libc::c_int + frame_bit_rate / 1 as libc::c_int / 4 as libc::c_int
            } else if frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
                > frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            {
                frame_bit_rate / 1 as libc::c_int / 5 as libc::c_int
            } else {
                frame_bit_rate / 1 as libc::c_int * 15 as libc::c_int / 32 as libc::c_int
                    - 5500 as libc::c_int
            }
        } else {
            (*avctx).sample_rate / 2 as libc::c_int
        };
    }
    cutoff = bandwidth * 2 as libc::c_int * wlen / (*avctx).sample_rate;
    (*sce).band_alt = (*sce).band_type;
    w = 0 as libc::c_int;
    while w < (*sce).ics.num_windows {
        g = 0 as libc::c_int;
        while g < (*sce).ics.num_swb {
            let mut sfb_energy: libc::c_float = 0.0f32;
            let mut threshold: libc::c_float = 0.0f32;
            let mut spread: libc::c_float = 2.0f32;
            let mut min_energy: libc::c_float = -1.0f32;
            let mut max_energy: libc::c_float = 0.0f32;
            let start: libc::c_int = *((*sce).ics.swb_offset).offset(g as isize) as libc::c_int;
            let freq: libc::c_float = start as libc::c_float * freq_mult;
            let freq_boost: libc::c_float =
                if 0.88f32 * freq / 4000 as libc::c_int as libc::c_float > 1.0f32 {
                    0.88f32 * freq / 4000 as libc::c_int as libc::c_float
                } else {
                    1.0f32
                };
            if freq < 4000 as libc::c_int as libc::c_float || start >= cutoff {
                (*sce).can_pns[(w * 16 as libc::c_int + g) as usize] = 0 as libc::c_int as uint8_t;
            } else {
                w2 = 0 as libc::c_int;
                while w2 < (*sce).ics.group_len[w as usize] as libc::c_int {
                    band = &mut *((*((*s).psy.ch).offset((*s).cur_channel as isize)).psy_bands)
                        .as_mut_ptr()
                        .offset(((w + w2) * 16 as libc::c_int + g) as isize)
                        as *mut FFPsyBand;
                    sfb_energy += (*band).energy;
                    spread = if spread > (*band).spread {
                        (*band).spread
                    } else {
                        spread
                    };
                    threshold += (*band).threshold;
                    if w2 == 0 {
                        max_energy = (*band).energy;
                        min_energy = max_energy;
                    } else {
                        min_energy = if min_energy > (*band).energy {
                            (*band).energy
                        } else {
                            min_energy
                        };
                        max_energy = if max_energy > (*band).energy {
                            max_energy
                        } else {
                            (*band).energy
                        };
                    }
                    w2 += 1;
                    w2;
                }
                (*sce).pns_ener[(w * 16 as libc::c_int + g) as usize] = sfb_energy;
                if sfb_energy < threshold * sqrtf(1.5f32 / freq_boost)
                    || spread < spread_threshold
                    || min_energy < pns_transient_energy_r * max_energy
                {
                    (*sce).can_pns[(w * 16 as libc::c_int + g) as usize] =
                        0 as libc::c_int as uint8_t;
                } else {
                    (*sce).can_pns[(w * 16 as libc::c_int + g) as usize] =
                        1 as libc::c_int as uint8_t;
                }
            }
            g += 1;
            g;
        }
        w += (*sce).ics.group_len[w as usize] as libc::c_int;
    }
}
unsafe extern "C" fn search_for_ms(mut s: *mut AACEncContext, mut cpe: *mut ChannelElement) {
    let mut start: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut w2: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut sid_sf_boost: libc::c_int = 0;
    let mut prev_mid: libc::c_int = 0;
    let mut prev_side: libc::c_int = 0;
    let mut nextband0: [uint8_t; 128] = [0; 128];
    let mut nextband1: [uint8_t; 128] = [0; 128];
    let mut M: *mut libc::c_float = ((*s).scoefs)
        .as_mut_ptr()
        .offset((128 as libc::c_int * 0 as libc::c_int) as isize);
    let mut S: *mut libc::c_float = ((*s).scoefs)
        .as_mut_ptr()
        .offset((128 as libc::c_int * 1 as libc::c_int) as isize);
    let mut L34: *mut libc::c_float = ((*s).scoefs)
        .as_mut_ptr()
        .offset((128 as libc::c_int * 2 as libc::c_int) as isize);
    let mut R34: *mut libc::c_float = ((*s).scoefs)
        .as_mut_ptr()
        .offset((128 as libc::c_int * 3 as libc::c_int) as isize);
    let mut M34: *mut libc::c_float = ((*s).scoefs)
        .as_mut_ptr()
        .offset((128 as libc::c_int * 4 as libc::c_int) as isize);
    let mut S34: *mut libc::c_float = ((*s).scoefs)
        .as_mut_ptr()
        .offset((128 as libc::c_int * 5 as libc::c_int) as isize);
    let lambda: libc::c_float = (*s).lambda;
    let mslambda: libc::c_float = if 1.0f32 > lambda / 120.0f32 {
        lambda / 120.0f32
    } else {
        1.0f32
    };
    let mut sce0: *mut SingleChannelElement =
        &mut *((*cpe).ch).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut SingleChannelElement;
    let mut sce1: *mut SingleChannelElement =
        &mut *((*cpe).ch).as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut SingleChannelElement;
    if (*cpe).common_window == 0 {
        return;
    }
    ff_init_nextband_map(sce0, nextband0.as_mut_ptr());
    ff_init_nextband_map(sce1, nextband1.as_mut_ptr());
    prev_mid = (*sce0).sf_idx[0 as libc::c_int as usize];
    prev_side = (*sce1).sf_idx[0 as libc::c_int as usize];
    w = 0 as libc::c_int;
    while w < (*sce0).ics.num_windows {
        start = 0 as libc::c_int;
        g = 0 as libc::c_int;
        while g < (*sce0).ics.num_swb {
            let mut bmax: libc::c_float =
                bval2bmax(g as libc::c_float * 17.0f32 / (*sce0).ics.num_swb as libc::c_float)
                    / 0.0045f32;
            if (*cpe).is_mask[(w * 16 as libc::c_int + g) as usize] == 0 {
                (*cpe).ms_mask[(w * 16 as libc::c_int + g) as usize] = 0 as libc::c_int as uint8_t;
            }
            if (*sce0).zeroes[(w * 16 as libc::c_int + g) as usize] == 0
                && (*sce1).zeroes[(w * 16 as libc::c_int + g) as usize] == 0
                && (*cpe).is_mask[(w * 16 as libc::c_int + g) as usize] == 0
            {
                let mut Mmax: libc::c_float = 0.0f32;
                let mut Smax: libc::c_float = 0.0f32;
                w2 = 0 as libc::c_int;
                while w2 < (*sce0).ics.group_len[w as usize] as libc::c_int {
                    i = 0 as libc::c_int;
                    while i < *((*sce0).ics.swb_sizes).offset(g as isize) as libc::c_int {
                        *M.offset(i as isize) = (((*sce0).coeffs
                            [(start + (w + w2) * 128 as libc::c_int + i) as usize]
                            + (*sce1).coeffs[(start + (w + w2) * 128 as libc::c_int + i) as usize])
                            as libc::c_double
                            * 0.5f64)
                            as libc::c_float;
                        *S.offset(i as isize) = *M.offset(i as isize)
                            - (*sce1).coeffs[(start + (w + w2) * 128 as libc::c_int + i) as usize];
                        i += 1;
                        i;
                    }
                    ((*s).abs_pow34).expect("non-null function pointer")(
                        M34,
                        M,
                        *((*sce0).ics.swb_sizes).offset(g as isize) as libc::c_int,
                    );
                    ((*s).abs_pow34).expect("non-null function pointer")(
                        S34,
                        S,
                        *((*sce0).ics.swb_sizes).offset(g as isize) as libc::c_int,
                    );
                    i = 0 as libc::c_int;
                    while i < *((*sce0).ics.swb_sizes).offset(g as isize) as libc::c_int {
                        Mmax = if Mmax > *M34.offset(i as isize) {
                            Mmax
                        } else {
                            *M34.offset(i as isize)
                        };
                        Smax = if Smax > *S34.offset(i as isize) {
                            Smax
                        } else {
                            *S34.offset(i as isize)
                        };
                        i += 1;
                        i;
                    }
                    w2 += 1;
                    w2;
                }
                sid_sf_boost = 0 as libc::c_int;
                while sid_sf_boost < 4 as libc::c_int {
                    let mut dist1: libc::c_float = 0.0f32;
                    let mut dist2: libc::c_float = 0.0f32;
                    let mut B0: libc::c_int = 0 as libc::c_int;
                    let mut B1: libc::c_int = 0 as libc::c_int;
                    let mut minidx: libc::c_int = 0;
                    let mut mididx: libc::c_int = 0;
                    let mut sididx: libc::c_int = 0;
                    let mut midcb: libc::c_int = 0;
                    let mut sidcb: libc::c_int = 0;
                    minidx = if (*sce0).sf_idx[(w * 16 as libc::c_int + g) as usize]
                        > (*sce1).sf_idx[(w * 16 as libc::c_int + g) as usize]
                    {
                        (*sce1).sf_idx[(w * 16 as libc::c_int + g) as usize]
                    } else {
                        (*sce0).sf_idx[(w * 16 as libc::c_int + g) as usize]
                    };
                    mididx = av_clip_c(
                        minidx,
                        0 as libc::c_int,
                        255 as libc::c_int - 36 as libc::c_int,
                    );
                    sididx = av_clip_c(
                        minidx - sid_sf_boost * 3 as libc::c_int,
                        0 as libc::c_int,
                        255 as libc::c_int - 36 as libc::c_int,
                    );
                    if !((*sce0).band_type[(w * 16 as libc::c_int + g) as usize] as libc::c_uint
                        != NOISE_BT as libc::c_int as libc::c_uint
                        && (*sce1).band_type[(w * 16 as libc::c_int + g) as usize] as libc::c_uint
                            != NOISE_BT as libc::c_int as libc::c_uint
                        && (ff_sfdelta_can_replace(
                            sce0,
                            nextband0.as_mut_ptr(),
                            prev_mid,
                            mididx,
                            w * 16 as libc::c_int + g,
                        ) == 0
                            || ff_sfdelta_can_replace(
                                sce1,
                                nextband1.as_mut_ptr(),
                                prev_side,
                                sididx,
                                w * 16 as libc::c_int + g,
                            ) == 0))
                    {
                        midcb = find_min_book(Mmax, mididx);
                        sidcb = find_min_book(Smax, sididx);
                        midcb = if 1 as libc::c_int > midcb {
                            1 as libc::c_int
                        } else {
                            midcb
                        };
                        sidcb = if 1 as libc::c_int > sidcb {
                            1 as libc::c_int
                        } else {
                            sidcb
                        };
                        w2 = 0 as libc::c_int;
                        while w2 < (*sce0).ics.group_len[w as usize] as libc::c_int {
                            let mut band0: *mut FFPsyBand = &mut *((*((*s).psy.ch)
                                .offset(((*s).cur_channel + 0 as libc::c_int) as isize))
                            .psy_bands)
                                .as_mut_ptr()
                                .offset(((w + w2) * 16 as libc::c_int + g) as isize)
                                as *mut FFPsyBand;
                            let mut band1: *mut FFPsyBand = &mut *((*((*s).psy.ch)
                                .offset(((*s).cur_channel + 1 as libc::c_int) as isize))
                            .psy_bands)
                                .as_mut_ptr()
                                .offset(((w + w2) * 16 as libc::c_int + g) as isize)
                                as *mut FFPsyBand;
                            let mut minthr: libc::c_float =
                                if (*band0).threshold > (*band1).threshold {
                                    (*band1).threshold
                                } else {
                                    (*band0).threshold
                                };
                            let mut b1: libc::c_int = 0;
                            let mut b2: libc::c_int = 0;
                            let mut b3: libc::c_int = 0;
                            let mut b4: libc::c_int = 0;
                            i = 0 as libc::c_int;
                            while i < *((*sce0).ics.swb_sizes).offset(g as isize) as libc::c_int {
                                *M.offset(i as isize) = (((*sce0).coeffs
                                    [(start + (w + w2) * 128 as libc::c_int + i) as usize]
                                    + (*sce1).coeffs
                                        [(start + (w + w2) * 128 as libc::c_int + i) as usize])
                                    as libc::c_double
                                    * 0.5f64)
                                    as libc::c_float;
                                *S.offset(i as isize) = *M.offset(i as isize)
                                    - (*sce1).coeffs
                                        [(start + (w + w2) * 128 as libc::c_int + i) as usize];
                                i += 1;
                                i;
                            }
                            ((*s).abs_pow34).expect("non-null function pointer")(
                                L34,
                                ((*sce0).coeffs)
                                    .as_mut_ptr()
                                    .offset(start as isize)
                                    .offset(((w + w2) * 128 as libc::c_int) as isize),
                                *((*sce0).ics.swb_sizes).offset(g as isize) as libc::c_int,
                            );
                            ((*s).abs_pow34).expect("non-null function pointer")(
                                R34,
                                ((*sce1).coeffs)
                                    .as_mut_ptr()
                                    .offset(start as isize)
                                    .offset(((w + w2) * 128 as libc::c_int) as isize),
                                *((*sce0).ics.swb_sizes).offset(g as isize) as libc::c_int,
                            );
                            ((*s).abs_pow34).expect("non-null function pointer")(
                                M34,
                                M,
                                *((*sce0).ics.swb_sizes).offset(g as isize) as libc::c_int,
                            );
                            ((*s).abs_pow34).expect("non-null function pointer")(
                                S34,
                                S,
                                *((*sce0).ics.swb_sizes).offset(g as isize) as libc::c_int,
                            );
                            dist1 += quantize_band_cost(
                                s,
                                &mut *((*sce0).coeffs)
                                    .as_mut_ptr()
                                    .offset((start + (w + w2) * 128 as libc::c_int) as isize),
                                L34,
                                *((*sce0).ics.swb_sizes).offset(g as isize) as libc::c_int,
                                (*sce0).sf_idx[(w * 16 as libc::c_int + g) as usize],
                                (*sce0).band_type[(w * 16 as libc::c_int + g) as usize]
                                    as libc::c_int,
                                lambda / ((*band0).threshold + 1.175_494_4e-38_f32),
                                ::core::f32::INFINITY,
                                &mut b1,
                                std::ptr::null_mut::<libc::c_float>(),
                            );
                            dist1 += quantize_band_cost(
                                s,
                                &mut *((*sce1).coeffs)
                                    .as_mut_ptr()
                                    .offset((start + (w + w2) * 128 as libc::c_int) as isize),
                                R34,
                                *((*sce1).ics.swb_sizes).offset(g as isize) as libc::c_int,
                                (*sce1).sf_idx[(w * 16 as libc::c_int + g) as usize],
                                (*sce1).band_type[(w * 16 as libc::c_int + g) as usize]
                                    as libc::c_int,
                                lambda / ((*band1).threshold + 1.175_494_4e-38_f32),
                                ::core::f32::INFINITY,
                                &mut b2,
                                std::ptr::null_mut::<libc::c_float>(),
                            );
                            dist2 += quantize_band_cost(
                                s,
                                M,
                                M34,
                                *((*sce0).ics.swb_sizes).offset(g as isize) as libc::c_int,
                                mididx,
                                midcb,
                                lambda / (minthr + 1.175_494_4e-38_f32),
                                ::core::f32::INFINITY,
                                &mut b3,
                                std::ptr::null_mut::<libc::c_float>(),
                            );
                            dist2 += quantize_band_cost(
                                s,
                                S,
                                S34,
                                *((*sce1).ics.swb_sizes).offset(g as isize) as libc::c_int,
                                sididx,
                                sidcb,
                                mslambda / (minthr * bmax + 1.175_494_4e-38_f32),
                                ::core::f32::INFINITY,
                                &mut b4,
                                std::ptr::null_mut::<libc::c_float>(),
                            );
                            B0 += b1 + b2;
                            B1 += b3 + b4;
                            dist1 -= (b1 + b2) as libc::c_float;
                            dist2 -= (b3 + b4) as libc::c_float;
                            w2 += 1;
                            w2;
                        }
                        (*cpe).ms_mask[(w * 16 as libc::c_int + g) as usize] =
                            (dist2 <= dist1 && B1 < B0) as libc::c_int as uint8_t;
                        if (*cpe).ms_mask[(w * 16 as libc::c_int + g) as usize] != 0 {
                            if (*sce0).band_type[(w * 16 as libc::c_int + g) as usize]
                                as libc::c_uint
                                != NOISE_BT as libc::c_int as libc::c_uint
                                && (*sce1).band_type[(w * 16 as libc::c_int + g) as usize]
                                    as libc::c_uint
                                    != NOISE_BT as libc::c_int as libc::c_uint
                            {
                                (*sce0).sf_idx[(w * 16 as libc::c_int + g) as usize] = mididx;
                                (*sce1).sf_idx[(w * 16 as libc::c_int + g) as usize] = sididx;
                                (*sce0).band_type[(w * 16 as libc::c_int + g) as usize] =
                                    midcb as BandType;
                                (*sce1).band_type[(w * 16 as libc::c_int + g) as usize] =
                                    sidcb as BandType;
                            } else if ((*sce0).band_type[(w * 16 as libc::c_int + g) as usize]
                                as libc::c_uint
                                != NOISE_BT as libc::c_int as libc::c_uint)
                                as libc::c_int
                                ^ ((*sce1).band_type[(w * 16 as libc::c_int + g) as usize]
                                    as libc::c_uint
                                    != NOISE_BT as libc::c_int as libc::c_uint)
                                    as libc::c_int
                                != 0
                            {
                                (*cpe).ms_mask[(w * 16 as libc::c_int + g) as usize] =
                                    0 as libc::c_int as uint8_t;
                            }
                            break;
                        } else if B1 > B0 {
                            break;
                        }
                    }
                    sid_sf_boost += 1;
                    sid_sf_boost;
                }
            }
            if (*sce0).zeroes[(w * 16 as libc::c_int + g) as usize] == 0
                && ((*sce0).band_type[(w * 16 as libc::c_int + g) as usize] as libc::c_uint)
                    < RESERVED_BT as libc::c_int as libc::c_uint
            {
                prev_mid = (*sce0).sf_idx[(w * 16 as libc::c_int + g) as usize];
            }
            if (*sce1).zeroes[(w * 16 as libc::c_int + g) as usize] == 0
                && (*cpe).is_mask[(w * 16 as libc::c_int + g) as usize] == 0
                && ((*sce1).band_type[(w * 16 as libc::c_int + g) as usize] as libc::c_uint)
                    < RESERVED_BT as libc::c_int as libc::c_uint
            {
                prev_side = (*sce1).sf_idx[(w * 16 as libc::c_int + g) as usize];
            }
            start += *((*sce0).ics.swb_sizes).offset(g as isize) as libc::c_int;
            g += 1;
            g;
        }
        w += (*sce0).ics.group_len[w as usize] as libc::c_int;
    }
}

pub(crate) static mut ff_aac_coders: [AACCoefficientsEncoder; 3] = {
    [
        {
            AACCoefficientsEncoder {
                search_for_quantizers: Some(search_for_quantizers_anmr),
                encode_window_bands_info: Some(encode_window_bands_info),
                quantize_and_encode_band: Some(quantize_and_encode_band),
                encode_tns_info: Some(ff_aac_encode_tns_info),
                encode_ltp_info: Some(ff_aac_encode_ltp_info),
                encode_main_pred: Some(ff_aac_encode_main_pred),
                adjust_common_pred: Some(ff_aac_adjust_common_pred),
                adjust_common_ltp: Some(ff_aac_adjust_common_ltp),
                apply_main_pred: Some(ff_aac_apply_main_pred),
                apply_tns_filt: Some(ff_aac_apply_tns),
                update_ltp: Some(ff_aac_update_ltp),
                ltp_insert_new_frame: Some(ff_aac_ltp_insert_new_frame),
                set_special_band_scalefactors: Some(set_special_band_scalefactors),
                search_for_pns: Some(search_for_pns),
                mark_pns: Some(mark_pns),
                search_for_tns: Some(ff_aac_search_for_tns),
                search_for_ltp: Some(ff_aac_search_for_ltp),
                search_for_ms: Some(search_for_ms),
                search_for_is: Some(ff_aac_search_for_is),
                search_for_pred: Some(ff_aac_search_for_pred),
            }
        },
        {
            AACCoefficientsEncoder {
                search_for_quantizers: Some(search_for_quantizers_twoloop),
                encode_window_bands_info: Some(codebook_trellis_rate),
                quantize_and_encode_band: Some(quantize_and_encode_band),
                encode_tns_info: Some(ff_aac_encode_tns_info),
                encode_ltp_info: Some(ff_aac_encode_ltp_info),
                encode_main_pred: Some(ff_aac_encode_main_pred),
                adjust_common_pred: Some(ff_aac_adjust_common_pred),
                adjust_common_ltp: Some(ff_aac_adjust_common_ltp),
                apply_main_pred: Some(ff_aac_apply_main_pred),
                apply_tns_filt: Some(ff_aac_apply_tns),
                update_ltp: Some(ff_aac_update_ltp),
                ltp_insert_new_frame: Some(ff_aac_ltp_insert_new_frame),
                set_special_band_scalefactors: Some(set_special_band_scalefactors),
                search_for_pns: Some(search_for_pns),
                mark_pns: Some(mark_pns),
                search_for_tns: Some(ff_aac_search_for_tns),
                search_for_ltp: Some(ff_aac_search_for_ltp),
                search_for_ms: Some(search_for_ms),
                search_for_is: Some(ff_aac_search_for_is),
                search_for_pred: Some(ff_aac_search_for_pred),
            }
        },
        {
            AACCoefficientsEncoder {
                search_for_quantizers: Some(search_for_quantizers_fast),
                encode_window_bands_info: Some(codebook_trellis_rate),
                quantize_and_encode_band: Some(quantize_and_encode_band),
                encode_tns_info: Some(ff_aac_encode_tns_info),
                encode_ltp_info: Some(ff_aac_encode_ltp_info),
                encode_main_pred: Some(ff_aac_encode_main_pred),
                adjust_common_pred: Some(ff_aac_adjust_common_pred),
                adjust_common_ltp: Some(ff_aac_adjust_common_ltp),
                apply_main_pred: Some(ff_aac_apply_main_pred),
                apply_tns_filt: Some(ff_aac_apply_tns),
                update_ltp: Some(ff_aac_update_ltp),
                ltp_insert_new_frame: Some(ff_aac_ltp_insert_new_frame),
                set_special_band_scalefactors: Some(set_special_band_scalefactors),
                search_for_pns: Some(search_for_pns),
                mark_pns: Some(mark_pns),
                search_for_tns: Some(ff_aac_search_for_tns),
                search_for_ltp: Some(ff_aac_search_for_ltp),
                search_for_ms: Some(search_for_ms),
                search_for_is: Some(ff_aac_search_for_is),
                search_for_pred: Some(ff_aac_search_for_pred),
            }
        },
    ]
};
unsafe fn run_static_initializers() {
    BUF_BITS = (8 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<BitBuf>() as libc::c_ulong)
        as libc::c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe fn(); 1] = [run_static_initializers];
