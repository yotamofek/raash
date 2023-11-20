#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

mod quantizers;

use std::{mem::size_of, ptr};

use ilog::IntLog;
use libc::{c_char, c_double, c_float, c_int, c_long, c_uchar, c_uint, c_ulong};

use crate::{
    aacenc::ff_quantize_band_cost_cache_init, aacenc_is::*, aacenc_ltp::*, aacenc_pred::*,
    aacenc_tns::*, aactab::*, common::*, types::*,
};

type quantize_and_encode_band_func = unsafe fn(
    *mut AACEncContext,
    *mut PutBitContext,
    *const c_float,
    *mut c_float,
    *const c_float,
    c_int,
    c_int,
    c_int,
    c_float,
    c_float,
    *mut c_int,
    *mut c_float,
) -> c_float;
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) union C2RustUnnamed_2 {
    pub(crate) u: c_uint,
    pub(crate) s: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct TrellisBandCodingPath {
    pub(crate) prev_idx: c_int,
    pub(crate) cost: c_float,
    pub(crate) run: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct BandCodingPath {
    pub(crate) prev_idx: c_int,
    pub(crate) cost: c_float,
    pub(crate) run: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct TrellisPath {
    pub(crate) cost: c_float,
    pub(crate) prev: c_int,
}
#[inline]
unsafe fn ff_sqrf(mut a: c_float) -> c_float {
    a * a
}
#[inline(always)]
fn ff_log2_c(mut v: c_uint) -> c_int {
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

fn clip_uint8_c(mut a: c_int) -> c_uchar {
    a.clamp(c_uchar::MIN.into(), c_uchar::MAX.into()) as u8
}

fn clip_uintp2_c(mut a: c_int, mut p: c_int) -> c_uint {
    if a & !(((1 as c_int) << p) - 1 as c_int) != 0 {
        (!a >> 31 as c_int & ((1 as c_int) << p) - 1 as c_int) as c_uint
    } else {
        a as c_uint
    }
}

/// Clear high bits from an unsigned integer starting with specific bit position.
fn mod_uintp2_c(mut a: c_uint, mut p: c_uint) -> c_uint {
    a & ((1 as c_uint) << p).wrapping_sub(1 as c_int as c_uint)
}

static mut BUF_BITS: c_int = 0;
#[inline]
unsafe fn put_sbits(mut pb: *mut PutBitContext, mut n: c_int, mut value: c_int) {
    put_bits(pb, n, mod_uintp2_c(value as c_uint, n as c_uint));
}
#[inline]
unsafe fn put_bits(mut s: *mut PutBitContext, mut n: c_int, mut value: BitBuf) {
    put_bits_no_assert(s, n, value);
}
#[inline]
unsafe fn put_bits_no_assert(mut s: *mut PutBitContext, mut n: c_int, mut value: BitBuf) {
    let mut bit_buf: BitBuf = 0;
    let mut bit_left: c_int = 0;
    bit_buf = (*s).bit_buf;
    bit_left = (*s).bit_left;
    if n < bit_left {
        bit_buf = bit_buf << n | value;
        bit_left -= n;
    } else {
        bit_buf <<= bit_left;
        bit_buf |= value >> n - bit_left;
        if ((*s).buf_end).offset_from((*s).buf_ptr) as c_long as c_ulong
            >= size_of::<BitBuf>() as c_ulong
        {
            (*((*s).buf_ptr as *mut unaligned_32)).l = bit_buf.swap_bytes();
            (*s).buf_ptr = ((*s).buf_ptr).offset(size_of::<BitBuf>() as c_ulong as isize);
        } else {
            panic!("Internal error, put_bits buffer too small");
        }
        bit_left += BUF_BITS - n;
        bit_buf = value;
    }
    (*s).bit_buf = bit_buf;
    (*s).bit_left = bit_left;
}
static mut run_value_bits_long: [c_uchar; 64] = [
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
    10, 10, 10, 10, 10, 10, 10, 10, 15,
];
static mut run_value_bits_short: [c_uchar; 16] = [3, 3, 3, 3, 3, 3, 3, 6, 6, 6, 6, 6, 6, 6, 6, 9];
static mut run_value_bits: [*const c_uchar; 2] =
    unsafe { [run_value_bits_long.as_ptr(), run_value_bits_short.as_ptr()] };
static aac_cb_out_map: [c_uchar; 15] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15];
static aac_cb_in_map: [c_uchar; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0, 12, 13, 14];
static aac_cb_range: [c_uchar; 12] = [0, 3, 3, 3, 3, 9, 9, 8, 8, 13, 13, 17];
static aac_cb_maxval: [c_uchar; 12] = [0, 1, 1, 2, 2, 4, 4, 7, 7, 12, 12, 16];
static aac_maxval_cb: [c_uchar; 14] = [0, 1, 3, 5, 5, 7, 7, 7, 9, 9, 9, 9, 9, 11];
#[inline]
unsafe fn quant(mut coef: c_float, Q: c_float, rounding: c_float) -> c_int {
    let mut a: c_float = coef * Q;
    (sqrtf(a * sqrtf(a)) + rounding) as c_int
}
#[inline]
unsafe fn find_max_val(
    mut group_len: c_int,
    mut swb_size: c_int,
    mut scaled: *const c_float,
) -> c_float {
    let mut maxval: c_float = 0.0f32;
    let mut w2: c_int = 0;
    let mut i: c_int = 0;
    w2 = 0 as c_int;
    while w2 < group_len {
        i = 0 as c_int;
        while i < swb_size {
            maxval = if maxval > *scaled.offset((w2 * 128 as c_int + i) as isize) {
                maxval
            } else {
                *scaled.offset((w2 * 128 as c_int + i) as isize)
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
fn find_min_book(mut maxval: c_float, mut sf: c_int) -> c_int {
    let mut Q34: c_float =
        POW_SF_TABLES.pow34[(200 as c_int - sf + 140 as c_int - 36 as c_int) as usize];
    let mut qmaxval: c_int = 0;
    let mut cb: c_int = 0;
    qmaxval = (maxval * Q34 + 0.4054f32) as c_int;
    if qmaxval as c_ulong
        >= (size_of::<[c_uchar; 14]>() as c_ulong).wrapping_div(size_of::<c_uchar>() as c_ulong)
    {
        cb = 11 as c_int;
    } else {
        cb = aac_maxval_cb[qmaxval as usize] as c_int;
    }
    cb
}
#[inline]
unsafe fn find_form_factor(
    mut group_len: c_int,
    mut swb_size: c_int,
    mut thresh: c_float,
    mut scaled: *const c_float,
    mut nzslope: c_float,
) -> c_float {
    let iswb_size: c_float = 1.0f32 / swb_size as c_float;
    let iswb_sizem1: c_float = 1.0f32 / (swb_size - 1 as c_int) as c_float;
    let ethresh: c_float = thresh;
    let mut form: c_float = 0.0f32;
    let mut weight: c_float = 0.0f32;
    let mut w2: c_int = 0;
    let mut i: c_int = 0;
    w2 = 0 as c_int;
    while w2 < group_len {
        let mut e: c_float = 0.0f32;
        let mut e2: c_float = 0.0f32;
        let mut var: c_float = 0.0f32;
        let mut maxval: c_float = 0.0f32;
        let mut nzl: c_float = 0 as c_int as c_float;
        i = 0 as c_int;
        while i < swb_size {
            let mut s: c_float = fabsf(*scaled.offset((w2 * 128 as c_int + i) as isize));
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
            let mut frm: c_float = 0.;
            e *= iswb_size;
            i = 0 as c_int;
            while i < swb_size {
                let mut d: c_float = fabsf(*scaled.offset((w2 * 128 as c_int + i) as isize)) - e;
                var += d * d;
                i += 1;
                i;
            }
            var = sqrtf(var * iswb_sizem1);
            e2 *= iswb_size;
            frm = e
                / (if e + 4 as c_int as c_float * var > maxval {
                    maxval
                } else {
                    e + 4 as c_int as c_float * var
                });
            form += e2 * sqrtf(frm) / (if 0.5f32 > nzl { 0.5f32 } else { nzl });
            weight += e2;
        }
        w2 += 1;
        w2;
    }
    if weight > 0 as c_int as c_float {
        form / weight
    } else {
        1.0f32
    }
}
#[inline]
unsafe fn coef2minsf(mut coef: c_float) -> c_uchar {
    clip_uint8_c(
        (log2f(coef) * 4 as c_int as c_float - 69 as c_int as c_float + 140 as c_int as c_float
            - 36 as c_int as c_float) as c_int,
    )
}
#[inline(always)]
unsafe fn ff_fast_powf(mut x: c_float, mut y: c_float) -> c_float {
    expf(logf(x) * y)
}
#[inline(always)]
unsafe fn bval2bmax(mut b: c_float) -> c_float {
    0.001f32 + 0.0035f32 * (b * b * b) / (15.5f32 * 15.5f32 * 15.5f32)
}
#[inline]
unsafe fn ff_sfdelta_can_remove_band(
    mut sce: *const SingleChannelElement,
    mut nextband: *const c_uchar,
    mut prev_sf: c_int,
    mut band: c_int,
) -> c_int {
    (prev_sf >= 0 as c_int
        && (*sce).sf_idx[*nextband.offset(band as isize) as usize] >= prev_sf - 60 as c_int
        && (*sce).sf_idx[*nextband.offset(band as isize) as usize] <= prev_sf + 60 as c_int)
        as c_int
}
#[inline]
unsafe fn coef2maxsf(mut coef: c_float) -> c_uchar {
    clip_uint8_c(
        (log2f(coef) * 4 as c_int as c_float + 6 as c_int as c_float + 140 as c_int as c_float
            - 36 as c_int as c_float) as c_int,
    )
}
#[inline(always)]
unsafe fn lcg_random(mut previous_val: c_uint) -> c_int {
    let mut v: C2RustUnnamed_2 = C2RustUnnamed_2 {
        u: previous_val
            .wrapping_mul(1664525 as c_uint)
            .wrapping_add(1013904223 as c_int as c_uint),
    };
    v.s
}
#[inline]
unsafe fn ff_init_nextband_map(mut sce: *const SingleChannelElement, mut nextband: *mut c_uchar) {
    let mut prevband: c_uchar = 0 as c_int as c_uchar;
    let mut w: c_int = 0;
    let mut g: c_int = 0;
    g = 0 as c_int;
    while g < 128 as c_int {
        *nextband.offset(g as isize) = g as c_uchar;
        g += 1;
        g;
    }
    w = 0 as c_int;
    while w < (*sce).ics.num_windows {
        g = 0 as c_int;
        while g < (*sce).ics.num_swb {
            if (*sce).zeroes[(w * 16 as c_int + g) as usize] == 0
                && ((*sce).band_type[(w * 16 as c_int + g) as usize] as c_uint)
                    < RESERVED_BT as c_int as c_uint
            {
                let fresh0 = &mut (*nextband.offset(prevband as isize));
                *fresh0 = (w * 16 as c_int + g) as c_uchar;
                prevband = *fresh0;
            }
            g += 1;
            g;
        }
        w += (*sce).ics.group_len[w as usize] as c_int;
    }
    *nextband.offset(prevband as isize) = prevband;
}
#[inline]
unsafe fn ff_sfdelta_can_replace(
    mut sce: *const SingleChannelElement,
    mut nextband: *const c_uchar,
    mut prev_sf: c_int,
    mut new_sf: c_int,
    mut band: c_int,
) -> c_int {
    (new_sf >= prev_sf - 60 as c_int
        && new_sf <= prev_sf + 60 as c_int
        && (*sce).sf_idx[*nextband.offset(band as isize) as usize] >= new_sf - 60 as c_int
        && (*sce).sf_idx[*nextband.offset(band as isize) as usize] <= new_sf + 60 as c_int)
        as c_int
}
#[inline]
unsafe fn quantize_band_cost_cached(
    mut s: *mut AACEncContext,
    mut w: c_int,
    mut g: c_int,
    mut in_0: *const c_float,
    mut scaled: *const c_float,
    mut size: c_int,
    mut scale_idx: c_int,
    mut cb: c_int,
    lambda: c_float,
    uplim: c_float,
    mut bits: *mut c_int,
    mut energy: *mut c_float,
    mut rtz: c_int,
) -> c_float {
    let mut entry: *mut AACQuantizeBandCostCacheEntry =
        ptr::null_mut::<AACQuantizeBandCostCacheEntry>();
    entry = &mut *(*((*s).quantize_band_cost_cache)
        .as_mut_ptr()
        .offset(scale_idx as isize))
    .as_mut_ptr()
    .offset((w * 16 as c_int + g) as isize) as *mut AACQuantizeBandCostCacheEntry;
    if (*entry).generation as c_int != (*s).quantize_band_cost_cache_generation as c_int
        || (*entry).cb as c_int != cb
        || (*entry).rtz as c_int != rtz
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
        (*entry).cb = cb as c_char;
        (*entry).rtz = rtz as c_char;
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
    mut in_0: *const c_float,
    mut scaled: *const c_float,
    mut size: c_int,
    mut scale_idx: c_int,
    mut cb: c_int,
    lambda: c_float,
    uplim: c_float,
    mut bits: *mut c_int,
    mut energy: *mut c_float,
) -> c_float {
    ff_quantize_and_encode_band_cost(
        s,
        ptr::null_mut::<PutBitContext>(),
        in_0,
        ptr::null_mut::<c_float>(),
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
    mut in_0: *const c_float,
    mut scaled: *const c_float,
    mut size: c_int,
    mut scale_idx: c_int,
    mut cb: c_int,
    _lambda: c_float,
    uplim: c_float,
    mut bits: *mut c_int,
    mut energy: *mut c_float,
) -> c_int {
    let mut auxbits: c_int = 0;
    ff_quantize_and_encode_band_cost(
        s,
        ptr::null_mut::<PutBitContext>(),
        in_0,
        ptr::null_mut::<c_float>(),
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
unsafe fn ff_pns_bits(mut sce: *mut SingleChannelElement, mut w: c_int, mut g: c_int) -> c_int {
    if g == 0
        || (*sce).zeroes[(w * 16 as c_int + g - 1 as c_int) as usize] == 0
        || (*sce).can_pns[(w * 16 as c_int + g - 1 as c_int) as usize] == 0
    {
        9 as c_int
    } else {
        5 as c_int
    }
}

fn cutoff_from_bitrate(bit_rate: c_int, channels: c_int, sample_rate: c_int) -> c_int {
    if bit_rate == 0 {
        return sample_rate / 2;
    }

    (bit_rate / channels / 5)
        .max(bit_rate / channels * 15 / 32 - 5500)
        .min(3000 + bit_rate / channels / 4)
        .min(12000 + bit_rate / channels / 16)
        .min(22000)
        .min(sample_rate / 2)
}

unsafe extern "C" fn codebook_trellis_rate(
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
    mut win: c_int,
    mut group_len: c_int,
    _lambda: c_float,
) {
    let mut path: [[TrellisBandCodingPath; 15]; 120] = [[TrellisBandCodingPath {
        prev_idx: 0,
        cost: 0.,
        run: 0,
    }; 15]; 120];
    let mut w: c_int = 0;
    let mut swb: c_int = 0;
    let mut cb: c_int = 0;
    let mut start: c_int = 0;
    let mut size: c_int = 0;
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let max_sfb: c_int = (*sce).ics.max_sfb as c_int;
    let run_bits: c_int = if (*sce).ics.num_windows == 1 as c_int {
        5 as c_int
    } else {
        3 as c_int
    };
    let run_esc: c_int = ((1 as c_int) << run_bits) - 1 as c_int;
    let mut idx: c_int = 0;
    let mut ppos: c_int = 0;
    let mut count: c_int = 0;
    let mut stackrun: [c_int; 120] = [0; 120];
    let mut stackcb: [c_int; 120] = [0; 120];
    let mut stack_len: c_int = 0;
    let mut next_minbits: c_float = ::core::f32::INFINITY;
    let mut next_mincb: c_int = 0 as c_int;
    ((*s).abs_pow34).expect("non-null function pointer")(
        ((*s).scoefs).as_mut_ptr(),
        ((*sce).coeffs).as_mut_ptr(),
        1024 as c_int,
    );
    start = win * 128 as c_int;
    cb = 0 as c_int;
    while cb < 15 as c_int {
        path[0 as c_int as usize][cb as usize].cost = (run_bits + 4 as c_int) as c_float;
        path[0 as c_int as usize][cb as usize].prev_idx = -(1 as c_int);
        path[0 as c_int as usize][cb as usize].run = 0 as c_int;
        cb += 1;
        cb;
    }
    swb = 0 as c_int;
    while swb < max_sfb {
        size = *((*sce).ics.swb_sizes).offset(swb as isize) as c_int;
        if (*sce).zeroes[(win * 16 as c_int + swb) as usize] != 0 {
            let mut cost_stay_here: c_float = path[swb as usize][0 as c_int as usize].cost;
            let mut cost_get_here: c_float =
                next_minbits + run_bits as c_float + 4 as c_int as c_float;
            if *(run_value_bits[((*sce).ics.num_windows == 8 as c_int) as c_int as usize])
                .offset(path[swb as usize][0 as c_int as usize].run as isize)
                as c_int
                != *(run_value_bits[((*sce).ics.num_windows == 8 as c_int) as c_int as usize])
                    .offset((path[swb as usize][0 as c_int as usize].run + 1 as c_int) as isize)
                    as c_int
            {
                cost_stay_here += run_bits as c_float;
            }
            if cost_get_here < cost_stay_here {
                path[(swb + 1 as c_int) as usize][0 as c_int as usize].prev_idx = next_mincb;
                path[(swb + 1 as c_int) as usize][0 as c_int as usize].cost = cost_get_here;
                path[(swb + 1 as c_int) as usize][0 as c_int as usize].run = 1 as c_int;
            } else {
                path[(swb + 1 as c_int) as usize][0 as c_int as usize].prev_idx = 0 as c_int;
                path[(swb + 1 as c_int) as usize][0 as c_int as usize].cost = cost_stay_here;
                path[(swb + 1 as c_int) as usize][0 as c_int as usize].run =
                    path[swb as usize][0 as c_int as usize].run + 1 as c_int;
            }
            next_minbits = path[(swb + 1 as c_int) as usize][0 as c_int as usize].cost;
            next_mincb = 0 as c_int;
            cb = 1 as c_int;
            while cb < 15 as c_int {
                path[(swb + 1 as c_int) as usize][cb as usize].cost = 61450 as c_int as c_float;
                path[(swb + 1 as c_int) as usize][cb as usize].prev_idx = -(1 as c_int);
                path[(swb + 1 as c_int) as usize][cb as usize].run = 0 as c_int;
                cb += 1;
                cb;
            }
        } else {
            let mut minbits: c_float = next_minbits;
            let mut mincb: c_int = next_mincb;
            let mut startcb: c_int = (*sce).band_type[(win * 16 as c_int + swb) as usize] as c_int;
            startcb = aac_cb_in_map[startcb as usize] as c_int;
            next_minbits = ::core::f32::INFINITY;
            next_mincb = 0 as c_int;
            cb = 0 as c_int;
            while cb < startcb {
                path[(swb + 1 as c_int) as usize][cb as usize].cost = 61450 as c_int as c_float;
                path[(swb + 1 as c_int) as usize][cb as usize].prev_idx = -(1 as c_int);
                path[(swb + 1 as c_int) as usize][cb as usize].run = 0 as c_int;
                cb += 1;
                cb;
            }
            cb = startcb;
            while cb < 15 as c_int {
                let mut cost_stay_here_0: c_float = 0.;
                let mut cost_get_here_0: c_float = 0.;
                let mut bits: c_float = 0.0f32;
                if cb >= 12 as c_int
                    && (*sce).band_type[(win * 16 as c_int + swb) as usize] as c_uint
                        != aac_cb_out_map[cb as usize] as c_uint
                {
                    path[(swb + 1 as c_int) as usize][cb as usize].cost = 61450 as c_int as c_float;
                    path[(swb + 1 as c_int) as usize][cb as usize].prev_idx = -(1 as c_int);
                    path[(swb + 1 as c_int) as usize][cb as usize].run = 0 as c_int;
                } else {
                    w = 0 as c_int;
                    while w < group_len {
                        bits += quantize_band_cost_bits(
                            s,
                            &mut *((*sce).coeffs)
                                .as_mut_ptr()
                                .offset((start + w * 128 as c_int) as isize),
                            &mut *((*s).scoefs)
                                .as_mut_ptr()
                                .offset((start + w * 128 as c_int) as isize),
                            size,
                            (*sce).sf_idx[(win * 16 as c_int + swb) as usize],
                            aac_cb_out_map[cb as usize] as c_int,
                            0 as c_int as c_float,
                            ::core::f32::INFINITY,
                            ptr::null_mut::<c_int>(),
                            ptr::null_mut::<c_float>(),
                        ) as c_float;
                        w += 1;
                        w;
                    }
                    cost_stay_here_0 = path[swb as usize][cb as usize].cost + bits;
                    cost_get_here_0 = minbits + bits + run_bits as c_float + 4 as c_int as c_float;
                    if *(run_value_bits[((*sce).ics.num_windows == 8 as c_int) as c_int as usize])
                        .offset(path[swb as usize][cb as usize].run as isize)
                        as c_int
                        != *(run_value_bits
                            [((*sce).ics.num_windows == 8 as c_int) as c_int as usize])
                            .offset((path[swb as usize][cb as usize].run + 1 as c_int) as isize)
                            as c_int
                    {
                        cost_stay_here_0 += run_bits as c_float;
                    }
                    if cost_get_here_0 < cost_stay_here_0 {
                        path[(swb + 1 as c_int) as usize][cb as usize].prev_idx = mincb;
                        path[(swb + 1 as c_int) as usize][cb as usize].cost = cost_get_here_0;
                        path[(swb + 1 as c_int) as usize][cb as usize].run = 1 as c_int;
                    } else {
                        path[(swb + 1 as c_int) as usize][cb as usize].prev_idx = cb;
                        path[(swb + 1 as c_int) as usize][cb as usize].cost = cost_stay_here_0;
                        path[(swb + 1 as c_int) as usize][cb as usize].run =
                            path[swb as usize][cb as usize].run + 1 as c_int;
                    }
                    if path[(swb + 1 as c_int) as usize][cb as usize].cost < next_minbits {
                        next_minbits = path[(swb + 1 as c_int) as usize][cb as usize].cost;
                        next_mincb = cb;
                    }
                }
                cb += 1;
                cb;
            }
        }
        start += *((*sce).ics.swb_sizes).offset(swb as isize) as c_int;
        swb += 1;
        swb;
    }
    stack_len = 0 as c_int;
    idx = 0 as c_int;
    cb = 1 as c_int;
    while cb < 15 as c_int {
        if path[max_sfb as usize][cb as usize].cost < path[max_sfb as usize][idx as usize].cost {
            idx = cb;
        }
        cb += 1;
        cb;
    }
    ppos = max_sfb;
    while ppos > 0 as c_int {
        cb = idx;
        stackrun[stack_len as usize] = path[ppos as usize][cb as usize].run;
        stackcb[stack_len as usize] = cb;
        idx = path[(ppos - path[ppos as usize][cb as usize].run + 1 as c_int) as usize]
            [cb as usize]
            .prev_idx;
        ppos -= path[ppos as usize][cb as usize].run;
        stack_len += 1;
        stack_len;
    }
    start = 0 as c_int;
    i = stack_len - 1 as c_int;
    while i >= 0 as c_int {
        cb = aac_cb_out_map[stackcb[i as usize] as usize] as c_int;
        put_bits(&mut (*s).pb, 4 as c_int, cb as BitBuf);
        count = stackrun[i as usize];
        ptr::write_bytes(
            (*sce)
                .zeroes
                .as_mut_ptr()
                .offset((win * 16 as c_int) as isize)
                .offset(start as isize),
            u8::from(cb == 0),
            count as usize,
        );
        j = 0 as c_int;
        while j < count {
            (*sce).band_type[(win * 16 as c_int + start) as usize] = cb as BandType;
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
    mut in_0: *const c_float,
    mut out: *mut c_float,
    mut scaled: *const c_float,
    mut size: c_int,
    mut scale_idx: c_int,
    mut cb: c_int,
    lambda: c_float,
    uplim: c_float,
    mut bits: *mut c_int,
    mut energy: *mut c_float,
    mut BT_ZERO: c_int,
    mut BT_UNSIGNED: c_int,
    mut BT_PAIR: c_int,
    mut BT_ESC: c_int,
    mut BT_NOISE: c_int,
    mut BT_STEREO: c_int,
    ROUNDING: c_float,
) -> c_float {
    let q_idx: c_int = 200 as c_int - scale_idx + 140 as c_int - 36 as c_int;
    let Q: c_float = POW_SF_TABLES.pow2[q_idx as usize];
    let Q34: c_float = POW_SF_TABLES.pow34[q_idx as usize];
    let IQ: c_float =
        POW_SF_TABLES.pow2[(200 as c_int + scale_idx - 140 as c_int + 36 as c_int) as usize];
    let CLIPPED_ESCAPE: c_float = 165140.0f32 * IQ;
    let mut cost: c_float = 0 as c_int as c_float;
    let mut qenergy: c_float = 0 as c_int as c_float;
    let dim: c_int = if BT_PAIR != 0 { 2 as c_int } else { 4 as c_int };
    let mut resbits: c_int = 0 as c_int;
    let mut off: c_int = 0;
    if BT_ZERO != 0 || BT_NOISE != 0 || BT_STEREO != 0 {
        let mut i: c_int = 0 as c_int;
        while i < size {
            cost += *in_0.offset(i as isize) * *in_0.offset(i as isize);
            i += 1;
            i;
        }
        if !bits.is_null() {
            *bits = 0 as c_int;
        }
        if !energy.is_null() {
            *energy = qenergy;
        }
        if !out.is_null() {
            let mut i_0: c_int = 0 as c_int;
            while i_0 < size {
                let mut j: c_int = 0 as c_int;
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
        (BT_UNSIGNED == 0) as c_int,
        aac_cb_maxval[cb as usize] as c_int,
        Q34,
        ROUNDING,
    );
    if BT_UNSIGNED != 0 {
        off = 0 as c_int;
    } else {
        off = aac_cb_maxval[cb as usize] as c_int;
    }
    let mut i_1: c_int = 0 as c_int;
    while i_1 < size {
        let mut vec: *const c_float = ptr::null::<c_float>();
        let mut quants: *mut c_int = ((*s).qcoefs).as_mut_ptr().offset(i_1 as isize);
        let mut curidx: c_int = 0 as c_int;
        let mut curbits: c_int = 0;
        let mut quantized: c_float = 0.;
        let mut rd: c_float = 0.0f32;
        let mut j_0: c_int = 0 as c_int;
        while j_0 < dim {
            curidx *= aac_cb_range[cb as usize] as c_int;
            curidx += *quants.offset(j_0 as isize) + off;
            j_0 += 1;
            j_0;
        }
        curbits = ff_aac_spectral_bits[(cb - 1 as c_int) as usize][curidx as usize] as c_int;
        vec = &(*ff_aac_codebook_vectors[(cb - 1 as c_int) as usize])[(curidx * dim) as usize]
            as *const c_float;
        if BT_UNSIGNED != 0 {
            let mut j_1: c_int = 0 as c_int;
            while j_1 < dim {
                let mut t: c_float = fabsf(*in_0.offset((i_1 + j_1) as isize));
                let mut di: c_float = 0.;
                if BT_ESC != 0 && *vec.offset(j_1 as isize) == 64.0f32 {
                    if t >= CLIPPED_ESCAPE {
                        quantized = CLIPPED_ESCAPE;
                        curbits += 21 as c_int;
                    } else {
                        let mut c: c_int =
                            clip_uintp2_c(quant(t, Q, ROUNDING), 13 as c_int) as c_int;
                        quantized = c as c_float * cbrtf(c as c_float) * IQ;
                        curbits += ff_log2_c(c as c_uint) * 2 as c_int - 4 as c_int + 1 as c_int;
                    }
                } else {
                    quantized = *vec.offset(j_1 as isize) * IQ;
                }
                di = t - quantized;
                if !out.is_null() {
                    *out.offset((i_1 + j_1) as isize) =
                        if *in_0.offset((i_1 + j_1) as isize) >= 0 as c_int as c_float {
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
            let mut j_2: c_int = 0 as c_int;
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
        cost += rd * lambda + curbits as c_float;
        resbits += curbits;
        if cost >= uplim {
            return uplim;
        }
        if !pb.is_null() {
            put_bits(
                pb,
                ff_aac_spectral_bits[(cb - 1 as c_int) as usize][curidx as usize] as c_int,
                ff_aac_spectral_codes[(cb - 1 as c_int) as usize][curidx as usize] as BitBuf,
            );
            if BT_UNSIGNED != 0 {
                let mut j_3: c_int = 0 as c_int;
                while j_3 < dim {
                    if ff_aac_codebook_vectors[(cb - 1 as c_int) as usize]
                        [(curidx * dim + j_3) as usize]
                        != 0.0f32
                    {
                        put_bits(
                            pb,
                            1 as c_int,
                            (*in_0.offset((i_1 + j_3) as isize) < 0.0f32) as c_int as BitBuf,
                        );
                    }
                    j_3 += 1;
                    j_3;
                }
            }
            if BT_ESC != 0 {
                let mut j_4: c_int = 0 as c_int;
                while j_4 < 2 as c_int {
                    if ff_aac_codebook_vectors[(cb - 1) as usize][(curidx * 2 + j_4) as usize]
                        == 64.0f32
                    {
                        let mut coef: c_int = clip_uintp2_c(
                            quant(fabsf(*in_0.offset((i_1 + j_4) as isize)), Q, ROUNDING),
                            13 as c_int,
                        ) as c_int;
                        let mut len: c_int = ff_log2_c(coef as c_uint);
                        put_bits(
                            pb,
                            len - 4 as c_int + 1 as c_int,
                            (((1 as c_int) << len - 4 as c_int + 1 as c_int) - 2 as c_int)
                                as BitBuf,
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
    mut _in_0: *const c_float,
    mut _quant_0: *mut c_float,
    mut _scaled: *const c_float,
    mut _size: c_int,
    mut _scale_idx: c_int,
    mut _cb: c_int,
    _lambda: c_float,
    _uplim: c_float,
    mut _bits: *mut c_int,
    mut _energy: *mut c_float,
) -> c_float {
    0.0f32
}
unsafe fn quantize_and_encode_band_cost_ZERO(
    mut s: *mut AACEncContext,
    mut pb: *mut PutBitContext,
    mut in_0: *const c_float,
    mut quant_0: *mut c_float,
    mut scaled: *const c_float,
    mut size: c_int,
    mut scale_idx: c_int,
    mut cb: c_int,
    lambda: c_float,
    uplim: c_float,
    mut bits: *mut c_int,
    mut energy: *mut c_float,
) -> c_float {
    quantize_and_encode_band_cost_template(
        s,
        pb,
        in_0,
        quant_0,
        scaled,
        size,
        scale_idx,
        if 0 as c_int != 0 { ESC_BT as c_int } else { cb },
        lambda,
        uplim,
        bits,
        energy,
        1 as c_int,
        0 as c_int,
        0 as c_int,
        0 as c_int,
        0 as c_int,
        0 as c_int,
        0.4054f32,
    )
}
unsafe fn quantize_and_encode_band_cost_SQUAD(
    mut s: *mut AACEncContext,
    mut pb: *mut PutBitContext,
    mut in_0: *const c_float,
    mut quant_0: *mut c_float,
    mut scaled: *const c_float,
    mut size: c_int,
    mut scale_idx: c_int,
    mut cb: c_int,
    lambda: c_float,
    uplim: c_float,
    mut bits: *mut c_int,
    mut energy: *mut c_float,
) -> c_float {
    quantize_and_encode_band_cost_template(
        s,
        pb,
        in_0,
        quant_0,
        scaled,
        size,
        scale_idx,
        if 0 as c_int != 0 { ESC_BT as c_int } else { cb },
        lambda,
        uplim,
        bits,
        energy,
        0 as c_int,
        0 as c_int,
        0 as c_int,
        0 as c_int,
        0 as c_int,
        0 as c_int,
        0.4054f32,
    )
}
unsafe fn quantize_and_encode_band_cost_UQUAD(
    mut s: *mut AACEncContext,
    mut pb: *mut PutBitContext,
    mut in_0: *const c_float,
    mut quant_0: *mut c_float,
    mut scaled: *const c_float,
    mut size: c_int,
    mut scale_idx: c_int,
    mut cb: c_int,
    lambda: c_float,
    uplim: c_float,
    mut bits: *mut c_int,
    mut energy: *mut c_float,
) -> c_float {
    quantize_and_encode_band_cost_template(
        s,
        pb,
        in_0,
        quant_0,
        scaled,
        size,
        scale_idx,
        if 0 as c_int != 0 { ESC_BT as c_int } else { cb },
        lambda,
        uplim,
        bits,
        energy,
        0 as c_int,
        1 as c_int,
        0 as c_int,
        0 as c_int,
        0 as c_int,
        0 as c_int,
        0.4054f32,
    )
}
unsafe fn quantize_and_encode_band_cost_SPAIR(
    mut s: *mut AACEncContext,
    mut pb: *mut PutBitContext,
    mut in_0: *const c_float,
    mut quant_0: *mut c_float,
    mut scaled: *const c_float,
    mut size: c_int,
    mut scale_idx: c_int,
    mut cb: c_int,
    lambda: c_float,
    uplim: c_float,
    mut bits: *mut c_int,
    mut energy: *mut c_float,
) -> c_float {
    quantize_and_encode_band_cost_template(
        s,
        pb,
        in_0,
        quant_0,
        scaled,
        size,
        scale_idx,
        if 0 as c_int != 0 { ESC_BT as c_int } else { cb },
        lambda,
        uplim,
        bits,
        energy,
        0 as c_int,
        0 as c_int,
        1 as c_int,
        0 as c_int,
        0 as c_int,
        0 as c_int,
        0.4054f32,
    )
}
unsafe fn quantize_and_encode_band_cost_UPAIR(
    mut s: *mut AACEncContext,
    mut pb: *mut PutBitContext,
    mut in_0: *const c_float,
    mut quant_0: *mut c_float,
    mut scaled: *const c_float,
    mut size: c_int,
    mut scale_idx: c_int,
    mut cb: c_int,
    lambda: c_float,
    uplim: c_float,
    mut bits: *mut c_int,
    mut energy: *mut c_float,
) -> c_float {
    quantize_and_encode_band_cost_template(
        s,
        pb,
        in_0,
        quant_0,
        scaled,
        size,
        scale_idx,
        if 0 as c_int != 0 { ESC_BT as c_int } else { cb },
        lambda,
        uplim,
        bits,
        energy,
        0 as c_int,
        1 as c_int,
        1 as c_int,
        0 as c_int,
        0 as c_int,
        0 as c_int,
        0.4054f32,
    )
}
unsafe fn quantize_and_encode_band_cost_ESC(
    mut s: *mut AACEncContext,
    mut pb: *mut PutBitContext,
    mut in_0: *const c_float,
    mut quant_0: *mut c_float,
    mut scaled: *const c_float,
    mut size: c_int,
    mut scale_idx: c_int,
    mut cb: c_int,
    lambda: c_float,
    uplim: c_float,
    mut bits: *mut c_int,
    mut energy: *mut c_float,
) -> c_float {
    quantize_and_encode_band_cost_template(
        s,
        pb,
        in_0,
        quant_0,
        scaled,
        size,
        scale_idx,
        if 1 as c_int != 0 { ESC_BT as c_int } else { cb },
        lambda,
        uplim,
        bits,
        energy,
        0 as c_int,
        1 as c_int,
        1 as c_int,
        1 as c_int,
        0 as c_int,
        0 as c_int,
        0.4054f32,
    )
}
unsafe fn quantize_and_encode_band_cost_ESC_RTZ(
    mut s: *mut AACEncContext,
    mut pb: *mut PutBitContext,
    mut in_0: *const c_float,
    mut quant_0: *mut c_float,
    mut scaled: *const c_float,
    mut size: c_int,
    mut scale_idx: c_int,
    mut cb: c_int,
    lambda: c_float,
    uplim: c_float,
    mut bits: *mut c_int,
    mut energy: *mut c_float,
) -> c_float {
    quantize_and_encode_band_cost_template(
        s,
        pb,
        in_0,
        quant_0,
        scaled,
        size,
        scale_idx,
        if 1 as c_int != 0 { ESC_BT as c_int } else { cb },
        lambda,
        uplim,
        bits,
        energy,
        0 as c_int,
        1 as c_int,
        1 as c_int,
        1 as c_int,
        0 as c_int,
        0 as c_int,
        0.1054f32,
    )
}
unsafe fn quantize_and_encode_band_cost_NOISE(
    mut s: *mut AACEncContext,
    mut pb: *mut PutBitContext,
    mut in_0: *const c_float,
    mut quant_0: *mut c_float,
    mut scaled: *const c_float,
    mut size: c_int,
    mut scale_idx: c_int,
    mut cb: c_int,
    lambda: c_float,
    uplim: c_float,
    mut bits: *mut c_int,
    mut energy: *mut c_float,
) -> c_float {
    quantize_and_encode_band_cost_template(
        s,
        pb,
        in_0,
        quant_0,
        scaled,
        size,
        scale_idx,
        if 0 as c_int != 0 { ESC_BT as c_int } else { cb },
        lambda,
        uplim,
        bits,
        energy,
        0 as c_int,
        0 as c_int,
        0 as c_int,
        0 as c_int,
        1 as c_int,
        0 as c_int,
        0.4054f32,
    )
}
unsafe fn quantize_and_encode_band_cost_STEREO(
    mut s: *mut AACEncContext,
    mut pb: *mut PutBitContext,
    mut in_0: *const c_float,
    mut quant_0: *mut c_float,
    mut scaled: *const c_float,
    mut size: c_int,
    mut scale_idx: c_int,
    mut cb: c_int,
    lambda: c_float,
    uplim: c_float,
    mut bits: *mut c_int,
    mut energy: *mut c_float,
) -> c_float {
    quantize_and_encode_band_cost_template(
        s,
        pb,
        in_0,
        quant_0,
        scaled,
        size,
        scale_idx,
        if 0 as c_int != 0 { ESC_BT as c_int } else { cb },
        lambda,
        uplim,
        bits,
        energy,
        0 as c_int,
        0 as c_int,
        0 as c_int,
        0 as c_int,
        0 as c_int,
        1 as c_int,
        0.4054f32,
    )
}
static mut quantize_and_encode_band_cost_arr: [quantize_and_encode_band_func; 16] = {
    [
        quantize_and_encode_band_cost_ZERO
            as unsafe fn(
                *mut AACEncContext,
                *mut PutBitContext,
                *const c_float,
                *mut c_float,
                *const c_float,
                c_int,
                c_int,
                c_int,
                c_float,
                c_float,
                *mut c_int,
                *mut c_float,
            ) -> c_float,
        quantize_and_encode_band_cost_SQUAD
            as unsafe fn(
                *mut AACEncContext,
                *mut PutBitContext,
                *const c_float,
                *mut c_float,
                *const c_float,
                c_int,
                c_int,
                c_int,
                c_float,
                c_float,
                *mut c_int,
                *mut c_float,
            ) -> c_float,
        (quantize_and_encode_band_cost_SQUAD
            as unsafe fn(
                *mut AACEncContext,
                *mut PutBitContext,
                *const c_float,
                *mut c_float,
                *const c_float,
                c_int,
                c_int,
                c_int,
                c_float,
                c_float,
                *mut c_int,
                *mut c_float,
            ) -> c_float),
        (quantize_and_encode_band_cost_UQUAD
            as unsafe fn(
                *mut AACEncContext,
                *mut PutBitContext,
                *const c_float,
                *mut c_float,
                *const c_float,
                c_int,
                c_int,
                c_int,
                c_float,
                c_float,
                *mut c_int,
                *mut c_float,
            ) -> c_float),
        (quantize_and_encode_band_cost_UQUAD
            as unsafe fn(
                *mut AACEncContext,
                *mut PutBitContext,
                *const c_float,
                *mut c_float,
                *const c_float,
                c_int,
                c_int,
                c_int,
                c_float,
                c_float,
                *mut c_int,
                *mut c_float,
            ) -> c_float),
        (quantize_and_encode_band_cost_SPAIR
            as unsafe fn(
                *mut AACEncContext,
                *mut PutBitContext,
                *const c_float,
                *mut c_float,
                *const c_float,
                c_int,
                c_int,
                c_int,
                c_float,
                c_float,
                *mut c_int,
                *mut c_float,
            ) -> c_float),
        (quantize_and_encode_band_cost_SPAIR
            as unsafe fn(
                *mut AACEncContext,
                *mut PutBitContext,
                *const c_float,
                *mut c_float,
                *const c_float,
                c_int,
                c_int,
                c_int,
                c_float,
                c_float,
                *mut c_int,
                *mut c_float,
            ) -> c_float),
        (quantize_and_encode_band_cost_UPAIR
            as unsafe fn(
                *mut AACEncContext,
                *mut PutBitContext,
                *const c_float,
                *mut c_float,
                *const c_float,
                c_int,
                c_int,
                c_int,
                c_float,
                c_float,
                *mut c_int,
                *mut c_float,
            ) -> c_float),
        (quantize_and_encode_band_cost_UPAIR
            as unsafe fn(
                *mut AACEncContext,
                *mut PutBitContext,
                *const c_float,
                *mut c_float,
                *const c_float,
                c_int,
                c_int,
                c_int,
                c_float,
                c_float,
                *mut c_int,
                *mut c_float,
            ) -> c_float),
        (quantize_and_encode_band_cost_UPAIR
            as unsafe fn(
                *mut AACEncContext,
                *mut PutBitContext,
                *const c_float,
                *mut c_float,
                *const c_float,
                c_int,
                c_int,
                c_int,
                c_float,
                c_float,
                *mut c_int,
                *mut c_float,
            ) -> c_float),
        (quantize_and_encode_band_cost_UPAIR
            as unsafe fn(
                *mut AACEncContext,
                *mut PutBitContext,
                *const c_float,
                *mut c_float,
                *const c_float,
                c_int,
                c_int,
                c_int,
                c_float,
                c_float,
                *mut c_int,
                *mut c_float,
            ) -> c_float),
        (quantize_and_encode_band_cost_ESC
            as unsafe fn(
                *mut AACEncContext,
                *mut PutBitContext,
                *const c_float,
                *mut c_float,
                *const c_float,
                c_int,
                c_int,
                c_int,
                c_float,
                c_float,
                *mut c_int,
                *mut c_float,
            ) -> c_float),
        (quantize_and_encode_band_cost_NONE
            as unsafe fn(
                *mut AACEncContext,
                *mut PutBitContext,
                *const c_float,
                *mut c_float,
                *const c_float,
                c_int,
                c_int,
                c_int,
                c_float,
                c_float,
                *mut c_int,
                *mut c_float,
            ) -> c_float),
        (quantize_and_encode_band_cost_NOISE
            as unsafe fn(
                *mut AACEncContext,
                *mut PutBitContext,
                *const c_float,
                *mut c_float,
                *const c_float,
                c_int,
                c_int,
                c_int,
                c_float,
                c_float,
                *mut c_int,
                *mut c_float,
            ) -> c_float),
        (quantize_and_encode_band_cost_STEREO
            as unsafe fn(
                *mut AACEncContext,
                *mut PutBitContext,
                *const c_float,
                *mut c_float,
                *const c_float,
                c_int,
                c_int,
                c_int,
                c_float,
                c_float,
                *mut c_int,
                *mut c_float,
            ) -> c_float),
        (quantize_and_encode_band_cost_STEREO
            as unsafe fn(
                *mut AACEncContext,
                *mut PutBitContext,
                *const c_float,
                *mut c_float,
                *const c_float,
                c_int,
                c_int,
                c_int,
                c_float,
                c_float,
                *mut c_int,
                *mut c_float,
            ) -> c_float),
    ]
};
static mut quantize_and_encode_band_cost_rtz_arr: [quantize_and_encode_band_func; 16] = unsafe {
    [
        (quantize_and_encode_band_cost_ZERO
            as unsafe fn(
                *mut AACEncContext,
                *mut PutBitContext,
                *const c_float,
                *mut c_float,
                *const c_float,
                c_int,
                c_int,
                c_int,
                c_float,
                c_float,
                *mut c_int,
                *mut c_float,
            ) -> c_float),
        (quantize_and_encode_band_cost_SQUAD
            as unsafe fn(
                *mut AACEncContext,
                *mut PutBitContext,
                *const c_float,
                *mut c_float,
                *const c_float,
                c_int,
                c_int,
                c_int,
                c_float,
                c_float,
                *mut c_int,
                *mut c_float,
            ) -> c_float),
        (quantize_and_encode_band_cost_SQUAD
            as unsafe fn(
                *mut AACEncContext,
                *mut PutBitContext,
                *const c_float,
                *mut c_float,
                *const c_float,
                c_int,
                c_int,
                c_int,
                c_float,
                c_float,
                *mut c_int,
                *mut c_float,
            ) -> c_float),
        (quantize_and_encode_band_cost_UQUAD
            as unsafe fn(
                *mut AACEncContext,
                *mut PutBitContext,
                *const c_float,
                *mut c_float,
                *const c_float,
                c_int,
                c_int,
                c_int,
                c_float,
                c_float,
                *mut c_int,
                *mut c_float,
            ) -> c_float),
        (quantize_and_encode_band_cost_UQUAD
            as unsafe fn(
                *mut AACEncContext,
                *mut PutBitContext,
                *const c_float,
                *mut c_float,
                *const c_float,
                c_int,
                c_int,
                c_int,
                c_float,
                c_float,
                *mut c_int,
                *mut c_float,
            ) -> c_float),
        (quantize_and_encode_band_cost_SPAIR
            as unsafe fn(
                *mut AACEncContext,
                *mut PutBitContext,
                *const c_float,
                *mut c_float,
                *const c_float,
                c_int,
                c_int,
                c_int,
                c_float,
                c_float,
                *mut c_int,
                *mut c_float,
            ) -> c_float),
        (quantize_and_encode_band_cost_SPAIR
            as unsafe fn(
                *mut AACEncContext,
                *mut PutBitContext,
                *const c_float,
                *mut c_float,
                *const c_float,
                c_int,
                c_int,
                c_int,
                c_float,
                c_float,
                *mut c_int,
                *mut c_float,
            ) -> c_float),
        (quantize_and_encode_band_cost_UPAIR
            as unsafe fn(
                *mut AACEncContext,
                *mut PutBitContext,
                *const c_float,
                *mut c_float,
                *const c_float,
                c_int,
                c_int,
                c_int,
                c_float,
                c_float,
                *mut c_int,
                *mut c_float,
            ) -> c_float),
        (quantize_and_encode_band_cost_UPAIR
            as unsafe fn(
                *mut AACEncContext,
                *mut PutBitContext,
                *const c_float,
                *mut c_float,
                *const c_float,
                c_int,
                c_int,
                c_int,
                c_float,
                c_float,
                *mut c_int,
                *mut c_float,
            ) -> c_float),
        (quantize_and_encode_band_cost_UPAIR
            as unsafe fn(
                *mut AACEncContext,
                *mut PutBitContext,
                *const c_float,
                *mut c_float,
                *const c_float,
                c_int,
                c_int,
                c_int,
                c_float,
                c_float,
                *mut c_int,
                *mut c_float,
            ) -> c_float),
        (quantize_and_encode_band_cost_UPAIR
            as unsafe fn(
                *mut AACEncContext,
                *mut PutBitContext,
                *const c_float,
                *mut c_float,
                *const c_float,
                c_int,
                c_int,
                c_int,
                c_float,
                c_float,
                *mut c_int,
                *mut c_float,
            ) -> c_float),
        (quantize_and_encode_band_cost_ESC_RTZ
            as unsafe fn(
                *mut AACEncContext,
                *mut PutBitContext,
                *const c_float,
                *mut c_float,
                *const c_float,
                c_int,
                c_int,
                c_int,
                c_float,
                c_float,
                *mut c_int,
                *mut c_float,
            ) -> c_float),
        (quantize_and_encode_band_cost_NONE
            as unsafe fn(
                *mut AACEncContext,
                *mut PutBitContext,
                *const c_float,
                *mut c_float,
                *const c_float,
                c_int,
                c_int,
                c_int,
                c_float,
                c_float,
                *mut c_int,
                *mut c_float,
            ) -> c_float),
        (quantize_and_encode_band_cost_NOISE
            as unsafe fn(
                *mut AACEncContext,
                *mut PutBitContext,
                *const c_float,
                *mut c_float,
                *const c_float,
                c_int,
                c_int,
                c_int,
                c_float,
                c_float,
                *mut c_int,
                *mut c_float,
            ) -> c_float),
        (quantize_and_encode_band_cost_STEREO
            as unsafe fn(
                *mut AACEncContext,
                *mut PutBitContext,
                *const c_float,
                *mut c_float,
                *const c_float,
                c_int,
                c_int,
                c_int,
                c_float,
                c_float,
                *mut c_int,
                *mut c_float,
            ) -> c_float),
        (quantize_and_encode_band_cost_STEREO
            as unsafe fn(
                *mut AACEncContext,
                *mut PutBitContext,
                *const c_float,
                *mut c_float,
                *const c_float,
                c_int,
                c_int,
                c_int,
                c_float,
                c_float,
                *mut c_int,
                *mut c_float,
            ) -> c_float),
    ]
};

pub(crate) unsafe fn ff_quantize_and_encode_band_cost(
    mut s: *mut AACEncContext,
    mut pb: *mut PutBitContext,
    mut in_0: *const c_float,
    mut quant_0: *mut c_float,
    mut scaled: *const c_float,
    mut size: c_int,
    mut scale_idx: c_int,
    mut cb: c_int,
    lambda: c_float,
    uplim: c_float,
    mut bits: *mut c_int,
    mut energy: *mut c_float,
) -> c_float {
    (quantize_and_encode_band_cost_arr[cb as usize])(
        s, pb, in_0, quant_0, scaled, size, scale_idx, cb, lambda, uplim, bits, energy,
    )
}
#[inline]
unsafe extern "C" fn quantize_and_encode_band(
    mut s: *mut AACEncContext,
    mut pb: *mut PutBitContext,
    mut in_0: *const c_float,
    mut out: *mut c_float,
    mut size: c_int,
    mut scale_idx: c_int,
    mut cb: c_int,
    lambda: c_float,
    mut rtz: c_int,
) {
    let arr = if rtz != 0 {
        &quantize_and_encode_band_cost_rtz_arr
    } else {
        &quantize_and_encode_band_cost_arr
    };
    arr[cb as usize](
        s,
        pb,
        in_0,
        out,
        ptr::null(),
        size,
        scale_idx,
        cb,
        lambda,
        f32::INFINITY,
        ptr::null_mut(),
        ptr::null_mut(),
    );
}
unsafe extern "C" fn encode_window_bands_info(
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
    mut win: c_int,
    mut group_len: c_int,
    lambda: c_float,
) {
    let mut path: [[BandCodingPath; 15]; 120] = [[BandCodingPath {
        prev_idx: 0,
        cost: 0.,
        run: 0,
    }; 15]; 120];
    let mut w: c_int = 0;
    let mut swb: c_int = 0;
    let mut cb: c_int = 0;
    let mut start: c_int = 0;
    let mut size: c_int = 0;
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let max_sfb: c_int = (*sce).ics.max_sfb as c_int;
    let run_bits: c_int = if (*sce).ics.num_windows == 1 as c_int {
        5 as c_int
    } else {
        3 as c_int
    };
    let run_esc: c_int = ((1 as c_int) << run_bits) - 1 as c_int;
    let mut idx: c_int = 0;
    let mut ppos: c_int = 0;
    let mut count: c_int = 0;
    let mut stackrun: [c_int; 120] = [0; 120];
    let mut stackcb: [c_int; 120] = [0; 120];
    let mut stack_len: c_int = 0;
    let mut next_minrd: c_float = ::core::f32::INFINITY;
    let mut next_mincb: c_int = 0 as c_int;
    ((*s).abs_pow34).expect("non-null function pointer")(
        ((*s).scoefs).as_mut_ptr(),
        ((*sce).coeffs).as_mut_ptr(),
        1024 as c_int,
    );
    start = win * 128 as c_int;
    cb = 0 as c_int;
    while cb < 15 as c_int {
        path[0 as c_int as usize][cb as usize].cost = 0.0f32;
        path[0 as c_int as usize][cb as usize].prev_idx = -(1 as c_int);
        path[0 as c_int as usize][cb as usize].run = 0 as c_int;
        cb += 1;
        cb;
    }
    swb = 0 as c_int;
    while swb < max_sfb {
        size = *((*sce).ics.swb_sizes).offset(swb as isize) as c_int;
        if (*sce).zeroes[(win * 16 as c_int + swb) as usize] != 0 {
            cb = 0 as c_int;
            while cb < 15 as c_int {
                path[(swb + 1 as c_int) as usize][cb as usize].prev_idx = cb;
                path[(swb + 1 as c_int) as usize][cb as usize].cost =
                    path[swb as usize][cb as usize].cost;
                path[(swb + 1 as c_int) as usize][cb as usize].run =
                    path[swb as usize][cb as usize].run + 1 as c_int;
                cb += 1;
                cb;
            }
        } else {
            let mut minrd: c_float = next_minrd;
            let mut mincb: c_int = next_mincb;
            next_minrd = ::core::f32::INFINITY;
            next_mincb = 0 as c_int;
            cb = 0 as c_int;
            while cb < 15 as c_int {
                let mut cost_stay_here: c_float = 0.;
                let mut cost_get_here: c_float = 0.;
                let mut rd: c_float = 0.0f32;
                if cb >= 12 as c_int
                    && ((*sce).band_type[(win * 16 as c_int + swb) as usize] as c_uint)
                        < aac_cb_out_map[cb as usize] as c_uint
                    || cb
                        < aac_cb_in_map
                            [(*sce).band_type[(win * 16 as c_int + swb) as usize] as usize]
                            as c_int
                        && (*sce).band_type[(win * 16 as c_int + swb) as usize] as c_uint
                            > aac_cb_out_map[cb as usize] as c_uint
                {
                    path[(swb + 1 as c_int) as usize][cb as usize].prev_idx = -(1 as c_int);
                    path[(swb + 1 as c_int) as usize][cb as usize].cost = ::core::f32::INFINITY;
                    path[(swb + 1 as c_int) as usize][cb as usize].run =
                        path[swb as usize][cb as usize].run + 1 as c_int;
                } else {
                    w = 0 as c_int;
                    while w < group_len {
                        let mut band: *mut FFPsyBand =
                            &mut *((*((*s).psy.ch).offset((*s).cur_channel as isize)).psy_bands)
                                .as_mut_ptr()
                                .offset(((win + w) * 16 as c_int + swb) as isize)
                                as *mut FFPsyBand;
                        rd += quantize_band_cost(
                            s,
                            &mut *((*sce).coeffs)
                                .as_mut_ptr()
                                .offset((start + w * 128 as c_int) as isize),
                            &mut *((*s).scoefs)
                                .as_mut_ptr()
                                .offset((start + w * 128 as c_int) as isize),
                            size,
                            (*sce).sf_idx[((win + w) * 16 as c_int + swb) as usize],
                            aac_cb_out_map[cb as usize] as c_int,
                            lambda / (*band).threshold,
                            ::core::f32::INFINITY,
                            ptr::null_mut::<c_int>(),
                            ptr::null_mut::<c_float>(),
                        );
                        w += 1;
                        w;
                    }
                    cost_stay_here = path[swb as usize][cb as usize].cost + rd;
                    cost_get_here = minrd + rd + run_bits as c_float + 4 as c_int as c_float;
                    if *(run_value_bits[((*sce).ics.num_windows == 8 as c_int) as c_int as usize])
                        .offset(path[swb as usize][cb as usize].run as isize)
                        as c_int
                        != *(run_value_bits
                            [((*sce).ics.num_windows == 8 as c_int) as c_int as usize])
                            .offset((path[swb as usize][cb as usize].run + 1 as c_int) as isize)
                            as c_int
                    {
                        cost_stay_here += run_bits as c_float;
                    }
                    if cost_get_here < cost_stay_here {
                        path[(swb + 1 as c_int) as usize][cb as usize].prev_idx = mincb;
                        path[(swb + 1 as c_int) as usize][cb as usize].cost = cost_get_here;
                        path[(swb + 1 as c_int) as usize][cb as usize].run = 1 as c_int;
                    } else {
                        path[(swb + 1 as c_int) as usize][cb as usize].prev_idx = cb;
                        path[(swb + 1 as c_int) as usize][cb as usize].cost = cost_stay_here;
                        path[(swb + 1 as c_int) as usize][cb as usize].run =
                            path[swb as usize][cb as usize].run + 1 as c_int;
                    }
                    if path[(swb + 1 as c_int) as usize][cb as usize].cost < next_minrd {
                        next_minrd = path[(swb + 1 as c_int) as usize][cb as usize].cost;
                        next_mincb = cb;
                    }
                }
                cb += 1;
                cb;
            }
        }
        start += *((*sce).ics.swb_sizes).offset(swb as isize) as c_int;
        swb += 1;
        swb;
    }
    stack_len = 0 as c_int;
    idx = 0 as c_int;
    cb = 1 as c_int;
    while cb < 15 as c_int {
        if path[max_sfb as usize][cb as usize].cost < path[max_sfb as usize][idx as usize].cost {
            idx = cb;
        }
        cb += 1;
        cb;
    }
    ppos = max_sfb;
    while ppos > 0 as c_int {
        cb = idx;
        stackrun[stack_len as usize] = path[ppos as usize][cb as usize].run;
        stackcb[stack_len as usize] = cb;
        idx = path[(ppos - path[ppos as usize][cb as usize].run + 1 as c_int) as usize]
            [cb as usize]
            .prev_idx;
        ppos -= path[ppos as usize][cb as usize].run;
        stack_len += 1;
        stack_len;
    }
    start = 0 as c_int;
    i = stack_len - 1 as c_int;
    while i >= 0 as c_int {
        cb = aac_cb_out_map[stackcb[i as usize] as usize] as c_int;
        put_bits(&mut (*s).pb, 4 as c_int, cb as BitBuf);
        count = stackrun[i as usize];
        ptr::write_bytes(
            ((*sce).zeroes)
                .as_mut_ptr()
                .offset((win * 16 as c_int) as isize)
                .offset(start as isize),
            u8::from(cb == 0),
            count as usize,
        );
        j = 0 as c_int;
        while j < count {
            (*sce).band_type[(win * 16 as c_int + start) as usize] = cb as BandType;
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
    let mut w: c_int = 0;
    let mut g: c_int = 0;
    let mut prevscaler_n: c_int = -(255 as c_int);
    let mut prevscaler_i: c_int = 0 as c_int;
    let mut bands: c_int = 0 as c_int;
    w = 0 as c_int;
    while w < (*sce).ics.num_windows {
        g = 0 as c_int;
        while g < (*sce).ics.num_swb {
            if (*sce).zeroes[(w * 16 as c_int + g) as usize] == 0 {
                if (*sce).band_type[(w * 16 as c_int + g) as usize] as c_uint
                    == INTENSITY_BT as c_int as c_uint
                    || (*sce).band_type[(w * 16 as c_int + g) as usize] as c_uint
                        == INTENSITY_BT2 as c_int as c_uint
                {
                    (*sce).sf_idx[(w * 16 as c_int + g) as usize] = av_clip_c(
                        roundf(
                            log2f((*sce).is_ener[(w * 16 as c_int + g) as usize])
                                * 2 as c_int as c_float,
                        ) as c_int,
                        -(155 as c_int),
                        100 as c_int,
                    );
                    bands += 1;
                    bands;
                } else if (*sce).band_type[(w * 16 as c_int + g) as usize] as c_uint
                    == NOISE_BT as c_int as c_uint
                {
                    (*sce).sf_idx[(w * 16 as c_int + g) as usize] = av_clip_c(
                        (3 as c_int as c_float
                            + ceilf(
                                log2f((*sce).pns_ener[(w * 16 as c_int + g) as usize])
                                    * 2 as c_int as c_float,
                            )) as c_int,
                        -(100 as c_int),
                        155 as c_int,
                    );
                    if prevscaler_n == -(255 as c_int) {
                        prevscaler_n = (*sce).sf_idx[(w * 16 as c_int + g) as usize];
                    }
                    bands += 1;
                    bands;
                }
            }
            g += 1;
            g;
        }
        w += (*sce).ics.group_len[w as usize] as c_int;
    }
    if bands == 0 {
        return;
    }
    w = 0 as c_int;
    while w < (*sce).ics.num_windows {
        g = 0 as c_int;
        while g < (*sce).ics.num_swb {
            if (*sce).zeroes[(w * 16 as c_int + g) as usize] == 0 {
                if (*sce).band_type[(w * 16 as c_int + g) as usize] as c_uint
                    == INTENSITY_BT as c_int as c_uint
                    || (*sce).band_type[(w * 16 as c_int + g) as usize] as c_uint
                        == INTENSITY_BT2 as c_int as c_uint
                {
                    prevscaler_i = av_clip_c(
                        (*sce).sf_idx[(w * 16 as c_int + g) as usize],
                        prevscaler_i - 60 as c_int,
                        prevscaler_i + 60 as c_int,
                    );
                    (*sce).sf_idx[(w * 16 as c_int + g) as usize] = prevscaler_i;
                } else if (*sce).band_type[(w * 16 as c_int + g) as usize] as c_uint
                    == NOISE_BT as c_int as c_uint
                {
                    prevscaler_n = av_clip_c(
                        (*sce).sf_idx[(w * 16 as c_int + g) as usize],
                        prevscaler_n - 60 as c_int,
                        prevscaler_n + 60 as c_int,
                    );
                    (*sce).sf_idx[(w * 16 as c_int + g) as usize] = prevscaler_n;
                }
            }
            g += 1;
            g;
        }
        w += (*sce).ics.group_len[w as usize] as c_int;
    }
}
unsafe extern "C" fn search_for_quantizers_anmr(
    mut _avctx: *mut AVCodecContext,
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
    lambda: c_float,
) {
    let mut q: c_int = 0;
    let mut w: c_int = 0;
    let mut w2: c_int = 0;
    let mut g: c_int = 0;
    let mut start: c_int = 0 as c_int;
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut idx: c_int = 0;
    let mut paths: [[TrellisPath; 61]; 121] = [[TrellisPath { cost: 0., prev: 0 }; 61]; 121];
    let mut bandaddr: [c_int; 121] = [0; 121];
    let mut minq: c_int = 0;
    let mut mincost: c_float = 0.;
    let mut q0f: c_float = 3.402_823_5e38_f32;
    let mut q1f: c_float = 0.0f32;
    let mut qnrgf: c_float = 0.0f32;
    let mut q0: c_int = 0;
    let mut q1: c_int = 0;
    let mut qcnt: c_int = 0 as c_int;
    i = 0 as c_int;
    while i < 1024 as c_int {
        let mut t: c_float = fabsf((*sce).coeffs[i as usize]);
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
        coef2minsf(q0f) as c_int,
        0 as c_int,
        255 as c_int - 1 as c_int,
    );
    q1 = av_clip_c(coef2maxsf(q1f) as c_int, 1 as c_int, 255 as c_int);
    if q1 - q0 > 60 as c_int {
        let mut q0low: c_int = q0;
        let mut q1high: c_int = q1;
        let mut qnrg: c_int = clip_uint8_c(
            (log2f(sqrtf(qnrgf / qcnt as c_float)) * 4 as c_int as c_float - 31 as c_int as c_float
                + 140 as c_int as c_float
                - 36 as c_int as c_float) as c_int,
        ) as c_int;
        q1 = qnrg + 30 as c_int;
        q0 = qnrg - 30 as c_int;
        if q0 < q0low {
            q1 += q0low - q0;
            q0 = q0low;
        } else if q1 > q1high {
            q0 -= q1 - q1high;
            q1 = q1high;
        }
    }
    if q0 == q1 {
        q1 = av_clip_c(q0 + 1 as c_int, 1 as c_int, 255 as c_int);
        q0 = av_clip_c(q1 - 1 as c_int, 0 as c_int, 255 as c_int - 1 as c_int);
    }
    i = 0 as c_int;
    while i < 60 as c_int + 1 as c_int {
        paths[0 as c_int as usize][i as usize].cost = 0.0f32;
        paths[0 as c_int as usize][i as usize].prev = -(1 as c_int);
        i += 1;
        i;
    }
    j = 1 as c_int;
    while j < 121 as c_int {
        i = 0 as c_int;
        while i < 60 as c_int + 1 as c_int {
            paths[j as usize][i as usize].cost = ::core::f32::INFINITY;
            paths[j as usize][i as usize].prev = -(2 as c_int);
            i += 1;
            i;
        }
        j += 1;
        j;
    }
    idx = 1 as c_int;
    ((*s).abs_pow34).expect("non-null function pointer")(
        ((*s).scoefs).as_mut_ptr(),
        ((*sce).coeffs).as_mut_ptr(),
        1024 as c_int,
    );
    w = 0 as c_int;
    while w < (*sce).ics.num_windows {
        start = w * 128 as c_int;
        g = 0 as c_int;
        while g < (*sce).ics.num_swb {
            let mut coefs: *const c_float =
                &mut *((*sce).coeffs).as_mut_ptr().offset(start as isize) as *mut c_float;
            let mut qmin: c_float = 0.;
            let mut qmax: c_float = 0.;
            let mut nz: c_int = 0 as c_int;
            bandaddr[idx as usize] = w * 16 as c_int + g;
            qmin = 2147483647 as c_int as c_float;
            qmax = 0.0f32;
            w2 = 0 as c_int;
            while w2 < (*sce).ics.group_len[w as usize] as c_int {
                let mut band: *mut FFPsyBand =
                    &mut *((*((*s).psy.ch).offset((*s).cur_channel as isize)).psy_bands)
                        .as_mut_ptr()
                        .offset(((w + w2) * 16 as c_int + g) as isize)
                        as *mut FFPsyBand;
                if (*band).energy <= (*band).threshold || (*band).threshold == 0.0f32 {
                    (*sce).zeroes[((w + w2) * 16 as c_int + g) as usize] = 1 as c_int as c_uchar;
                } else {
                    (*sce).zeroes[((w + w2) * 16 as c_int + g) as usize] = 0 as c_int as c_uchar;
                    nz = 1 as c_int;
                    i = 0 as c_int;
                    while i < *((*sce).ics.swb_sizes).offset(g as isize) as c_int {
                        let mut t_0: c_float =
                            fabsf(*coefs.offset((w2 * 128 as c_int + i) as isize));
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
                let mut minscale: c_int = 0;
                let mut maxscale: c_int = 0;
                let mut minrd: c_float = ::core::f32::INFINITY;
                let mut maxval: c_float = 0.;
                minscale = coef2minsf(qmin) as c_int;
                maxscale = coef2maxsf(qmax) as c_int;
                minscale = av_clip_c(
                    minscale - q0,
                    0 as c_int,
                    60 as c_int + 1 as c_int - 1 as c_int,
                );
                maxscale = av_clip_c(maxscale - q0, 0 as c_int, 60 as c_int + 1 as c_int);
                if minscale == maxscale {
                    maxscale =
                        av_clip_c(minscale + 1 as c_int, 1 as c_int, 60 as c_int + 1 as c_int);
                    minscale = av_clip_c(
                        maxscale - 1 as c_int,
                        0 as c_int,
                        60 as c_int + 1 as c_int - 1 as c_int,
                    );
                }
                maxval = find_max_val(
                    (*sce).ics.group_len[w as usize] as c_int,
                    *((*sce).ics.swb_sizes).offset(g as isize) as c_int,
                    ((*s).scoefs).as_mut_ptr().offset(start as isize),
                );
                q = minscale;
                while q < maxscale {
                    let mut dist: c_float = 0 as c_int as c_float;
                    let mut cb: c_int =
                        find_min_book(maxval, (*sce).sf_idx[(w * 16 as c_int + g) as usize]);
                    w2 = 0 as c_int;
                    while w2 < (*sce).ics.group_len[w as usize] as c_int {
                        let mut band_0: *mut FFPsyBand =
                            &mut *((*((*s).psy.ch).offset((*s).cur_channel as isize)).psy_bands)
                                .as_mut_ptr()
                                .offset(((w + w2) * 16 as c_int + g) as isize)
                                as *mut FFPsyBand;
                        dist += quantize_band_cost(
                            s,
                            coefs.offset((w2 * 128 as c_int) as isize),
                            ((*s).scoefs)
                                .as_mut_ptr()
                                .offset(start as isize)
                                .offset((w2 * 128 as c_int) as isize),
                            *((*sce).ics.swb_sizes).offset(g as isize) as c_int,
                            q + q0,
                            cb,
                            lambda / (*band_0).threshold,
                            ::core::f32::INFINITY,
                            ptr::null_mut::<c_int>(),
                            ptr::null_mut::<c_float>(),
                        );
                        w2 += 1;
                        w2;
                    }
                    minrd = if minrd > dist { dist } else { minrd };
                    i = 0 as c_int;
                    while i < q1 - q0 {
                        let mut cost: c_float = 0.;
                        cost = paths[(idx - 1 as c_int) as usize][i as usize].cost
                            + dist
                            + ff_aac_scalefactor_bits[(q - i + 60 as c_int) as usize] as c_int
                                as c_float;
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
                q = 0 as c_int;
                while q < q1 - q0 {
                    paths[idx as usize][q as usize].cost =
                        paths[(idx - 1 as c_int) as usize][q as usize].cost + 1 as c_int as c_float;
                    paths[idx as usize][q as usize].prev = q;
                    q += 1;
                    q;
                }
            }
            (*sce).zeroes[(w * 16 as c_int + g) as usize] = (nz == 0) as c_int as c_uchar;
            start += *((*sce).ics.swb_sizes).offset(g as isize) as c_int;
            idx += 1;
            idx;
            g += 1;
            g;
        }
        w += (*sce).ics.group_len[w as usize] as c_int;
    }
    idx -= 1;
    idx;
    mincost = paths[idx as usize][0 as c_int as usize].cost;
    minq = 0 as c_int;
    i = 1 as c_int;
    while i < 60 as c_int + 1 as c_int {
        if paths[idx as usize][i as usize].cost < mincost {
            mincost = paths[idx as usize][i as usize].cost;
            minq = i;
        }
        i += 1;
        i;
    }
    while idx != 0 {
        (*sce).sf_idx[bandaddr[idx as usize] as usize] = minq + q0;
        minq = if paths[idx as usize][minq as usize].prev > 0 as c_int {
            paths[idx as usize][minq as usize].prev
        } else {
            0 as c_int
        };
        idx -= 1;
        idx;
    }
    w = 0 as c_int;
    while w < (*sce).ics.num_windows {
        g = 0 as c_int;
        while g < (*sce).ics.num_swb {
            w2 = 1 as c_int;
            while w2 < (*sce).ics.group_len[w as usize] as c_int {
                (*sce).sf_idx[((w + w2) * 16 as c_int + g) as usize] =
                    (*sce).sf_idx[(w * 16 as c_int + g) as usize];
                w2 += 1;
                w2;
            }
            g += 1;
            g;
        }
        w += (*sce).ics.group_len[w as usize] as c_int;
    }
}
unsafe extern "C" fn search_for_quantizers_fast(
    mut avctx: *mut AVCodecContext,
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
    lambda: c_float,
) {
    let mut start: c_int = 0 as c_int;
    let mut i: c_int = 0;
    let mut w: c_int = 0;
    let mut w2: c_int = 0;
    let mut g: c_int = 0;
    let mut destbits: c_int = ((*avctx).bit_rate as c_double * 1024.0f64
        / (*avctx).sample_rate as c_double
        / (*avctx).ch_layout.nb_channels as c_double
        * (lambda / 120.0f32) as c_double) as c_int;
    let mut dists: [c_float; 128] = [
        0 as c_int as c_float,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
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
    let mut uplims: [c_float; 128] = [
        0 as c_int as c_float,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
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
    let mut maxvals: [c_float; 128] = [0.; 128];
    let mut fflag: c_int = 0;
    let mut minscaler: c_int = 0;
    let mut its: c_int = 0 as c_int;
    let mut allz: c_int = 0 as c_int;
    let mut minthr: c_float = ::core::f32::INFINITY;
    destbits = if destbits > 5800 as c_int {
        5800 as c_int
    } else {
        destbits
    };
    w = 0 as c_int;
    while w < (*sce).ics.num_windows {
        start = 0 as c_int;
        g = 0 as c_int;
        while g < (*sce).ics.num_swb {
            let mut nz: c_int = 0 as c_int;
            let mut uplim: c_float = 0.0f32;
            w2 = 0 as c_int;
            while w2 < (*sce).ics.group_len[w as usize] as c_int {
                let mut band: *mut FFPsyBand =
                    &mut *((*((*s).psy.ch).offset((*s).cur_channel as isize)).psy_bands)
                        .as_mut_ptr()
                        .offset(((w + w2) * 16 as c_int + g) as isize)
                        as *mut FFPsyBand;
                uplim += (*band).threshold;
                if (*band).energy <= (*band).threshold || (*band).threshold == 0.0f32 {
                    (*sce).zeroes[((w + w2) * 16 as c_int + g) as usize] = 1 as c_int as c_uchar;
                } else {
                    nz = 1 as c_int;
                }
                w2 += 1;
                w2;
            }
            uplims[(w * 16 as c_int + g) as usize] = uplim * 512 as c_int as c_float;
            (*sce).band_type[(w * 16 as c_int + g) as usize] = ZERO_BT;
            (*sce).zeroes[(w * 16 as c_int + g) as usize] = (nz == 0) as c_int as c_uchar;
            if nz != 0 {
                minthr = if minthr > uplim { uplim } else { minthr };
            }
            allz |= nz;
            start += *((*sce).ics.swb_sizes).offset(g as isize) as c_int;
            g += 1;
            g;
        }
        w += (*sce).ics.group_len[w as usize] as c_int;
    }
    w = 0 as c_int;
    while w < (*sce).ics.num_windows {
        g = 0 as c_int;
        while g < (*sce).ics.num_swb {
            if (*sce).zeroes[(w * 16 as c_int + g) as usize] != 0 {
                (*sce).sf_idx[(w * 16 as c_int + g) as usize] = 140 as c_int;
            } else {
                (*sce).sf_idx[(w * 16 as c_int + g) as usize] = (140 as c_int as c_float
                    + (if log2f(uplims[(w * 16 as c_int + g) as usize] / minthr)
                        * 4 as c_int as c_float
                        > 59 as c_int as c_float
                    {
                        59 as c_int as c_float
                    } else {
                        log2f(uplims[(w * 16 as c_int + g) as usize] / minthr)
                            * 4 as c_int as c_float
                    })) as c_int;
            }
            g += 1;
            g;
        }
        w += (*sce).ics.group_len[w as usize] as c_int;
    }
    if allz == 0 {
        return;
    }
    ((*s).abs_pow34).expect("non-null function pointer")(
        ((*s).scoefs).as_mut_ptr(),
        ((*sce).coeffs).as_mut_ptr(),
        1024 as c_int,
    );
    ff_quantize_band_cost_cache_init(s);
    w = 0 as c_int;
    while w < (*sce).ics.num_windows {
        start = w * 128 as c_int;
        g = 0 as c_int;
        while g < (*sce).ics.num_swb {
            let mut scaled: *const c_float = ((*s).scoefs).as_mut_ptr().offset(start as isize);
            maxvals[(w * 16 as c_int + g) as usize] = find_max_val(
                (*sce).ics.group_len[w as usize] as c_int,
                *((*sce).ics.swb_sizes).offset(g as isize) as c_int,
                scaled,
            );
            start += *((*sce).ics.swb_sizes).offset(g as isize) as c_int;
            g += 1;
            g;
        }
        w += (*sce).ics.group_len[w as usize] as c_int;
    }
    loop {
        let mut tbits: c_int = 0;
        let mut qstep: c_int = 0;
        minscaler = (*sce).sf_idx[0 as c_int as usize];
        qstep = if its != 0 { 1 as c_int } else { 32 as c_int };
        loop {
            let mut prev: c_int = -(1 as c_int);
            tbits = 0 as c_int;
            w = 0 as c_int;
            while w < (*sce).ics.num_windows {
                start = w * 128 as c_int;
                g = 0 as c_int;
                while g < (*sce).ics.num_swb {
                    let mut coefs: *const c_float =
                        ((*sce).coeffs).as_mut_ptr().offset(start as isize);
                    let mut scaled_0: *const c_float =
                        ((*s).scoefs).as_mut_ptr().offset(start as isize);
                    let mut bits: c_int = 0 as c_int;
                    let mut cb: c_int = 0;
                    let mut dist: c_float = 0.0f32;
                    if (*sce).zeroes[(w * 16 as c_int + g) as usize] as c_int != 0
                        || (*sce).sf_idx[(w * 16 as c_int + g) as usize] >= 218 as c_int
                    {
                        start += *((*sce).ics.swb_sizes).offset(g as isize) as c_int;
                    } else {
                        minscaler = if minscaler > (*sce).sf_idx[(w * 16 as c_int + g) as usize] {
                            (*sce).sf_idx[(w * 16 as c_int + g) as usize]
                        } else {
                            minscaler
                        };
                        cb = find_min_book(
                            maxvals[(w * 16 as c_int + g) as usize],
                            (*sce).sf_idx[(w * 16 as c_int + g) as usize],
                        );
                        w2 = 0 as c_int;
                        while w2 < (*sce).ics.group_len[w as usize] as c_int {
                            let mut b: c_int = 0;
                            dist += quantize_band_cost_cached(
                                s,
                                w + w2,
                                g,
                                coefs.offset((w2 * 128 as c_int) as isize),
                                scaled_0.offset((w2 * 128 as c_int) as isize),
                                *((*sce).ics.swb_sizes).offset(g as isize) as c_int,
                                (*sce).sf_idx[(w * 16 as c_int + g) as usize],
                                cb,
                                1.0f32,
                                ::core::f32::INFINITY,
                                &mut b,
                                ptr::null_mut::<c_float>(),
                                0 as c_int,
                            );
                            bits += b;
                            w2 += 1;
                            w2;
                        }
                        dists[(w * 16 as c_int + g) as usize] = dist - bits as c_float;
                        if prev != -(1 as c_int) {
                            bits += ff_aac_scalefactor_bits[((*sce).sf_idx
                                [(w * 16 as c_int + g) as usize]
                                - prev
                                + 60 as c_int)
                                as usize] as c_int;
                        }
                        tbits += bits;
                        start += *((*sce).ics.swb_sizes).offset(g as isize) as c_int;
                        prev = (*sce).sf_idx[(w * 16 as c_int + g) as usize];
                    }
                    g += 1;
                    g;
                }
                w += (*sce).ics.group_len[w as usize] as c_int;
            }
            if tbits > destbits {
                i = 0 as c_int;
                while i < 128 as c_int {
                    if (*sce).sf_idx[i as usize] < 218 as c_int - qstep {
                        (*sce).sf_idx[i as usize] += qstep;
                    }
                    i += 1;
                    i;
                }
            } else {
                i = 0 as c_int;
                while i < 128 as c_int {
                    if (*sce).sf_idx[i as usize] > 60 as c_int - qstep {
                        (*sce).sf_idx[i as usize] -= qstep;
                    }
                    i += 1;
                    i;
                }
            }
            qstep >>= 1 as c_int;
            if qstep == 0
                && tbits as c_double > destbits as c_double * 1.02f64
                && (*sce).sf_idx[0 as c_int as usize] < 217 as c_int
            {
                qstep = 1 as c_int;
            }
            if qstep == 0 {
                break;
            }
        }
        fflag = 0 as c_int;
        minscaler = av_clip_c(minscaler, 60 as c_int, 255 as c_int - 60 as c_int);
        w = 0 as c_int;
        while w < (*sce).ics.num_windows {
            g = 0 as c_int;
            while g < (*sce).ics.num_swb {
                let mut prevsc: c_int = (*sce).sf_idx[(w * 16 as c_int + g) as usize];
                if dists[(w * 16 as c_int + g) as usize] > uplims[(w * 16 as c_int + g) as usize]
                    && (*sce).sf_idx[(w * 16 as c_int + g) as usize] > 60 as c_int
                {
                    if find_min_book(
                        maxvals[(w * 16 as c_int + g) as usize],
                        (*sce).sf_idx[(w * 16 as c_int + g) as usize] - 1 as c_int,
                    ) != 0
                    {
                        (*sce).sf_idx[(w * 16 as c_int + g) as usize] -= 1;
                        (*sce).sf_idx[(w * 16 as c_int + g) as usize];
                    } else {
                        (*sce).sf_idx[(w * 16 as c_int + g) as usize] -= 2 as c_int;
                    }
                }
                (*sce).sf_idx[(w * 16 as c_int + g) as usize] = av_clip_c(
                    (*sce).sf_idx[(w * 16 as c_int + g) as usize],
                    minscaler,
                    minscaler + 60 as c_int,
                );
                (*sce).sf_idx[(w * 16 as c_int + g) as usize] =
                    if (*sce).sf_idx[(w * 16 as c_int + g) as usize] > 219 as c_int {
                        219 as c_int
                    } else {
                        (*sce).sf_idx[(w * 16 as c_int + g) as usize]
                    };
                if (*sce).sf_idx[(w * 16 as c_int + g) as usize] != prevsc {
                    fflag = 1 as c_int;
                }
                (*sce).band_type[(w * 16 as c_int + g) as usize] = find_min_book(
                    maxvals[(w * 16 as c_int + g) as usize],
                    (*sce).sf_idx[(w * 16 as c_int + g) as usize],
                ) as BandType;
                g += 1;
                g;
            }
            w += (*sce).ics.group_len[w as usize] as c_int;
        }
        its += 1;
        its;
        if !(fflag != 0 && its < 10 as c_int) {
            break;
        }
    }
}
unsafe extern "C" fn search_for_pns(
    mut s: *mut AACEncContext,
    mut avctx: *mut AVCodecContext,
    mut sce: *mut SingleChannelElement,
) {
    let mut band: *mut FFPsyBand = ptr::null_mut::<FFPsyBand>();
    let mut w: c_int = 0;
    let mut g: c_int = 0;
    let mut w2: c_int = 0;
    let mut i: c_int = 0;
    let mut wlen: c_int = 1024 as c_int / (*sce).ics.num_windows;
    let mut bandwidth: c_int = 0;
    let mut cutoff: c_int = 0;
    let mut PNS: *mut c_float = &mut *((*s).scoefs)
        .as_mut_ptr()
        .offset((0 as c_int * 128 as c_int) as isize)
        as *mut c_float;
    let mut PNS34: *mut c_float = &mut *((*s).scoefs)
        .as_mut_ptr()
        .offset((1 as c_int * 128 as c_int) as isize)
        as *mut c_float;
    let mut NOR34: *mut c_float = &mut *((*s).scoefs)
        .as_mut_ptr()
        .offset((3 as c_int * 128 as c_int) as isize)
        as *mut c_float;
    let mut nextband: [c_uchar; 128] = [0; 128];
    let lambda: c_float = (*s).lambda;
    let freq_mult: c_float = (*avctx).sample_rate as c_float * 0.5f32 / wlen as c_float;
    let thr_mult: c_float = 1.948f32 * (100.0f32 / lambda);
    let spread_threshold: c_float = if 0.75f32
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
    let dist_bias: c_float = (4.0f32 * 120 as c_int as c_float / lambda).clamp(0.25, 4.);
    let pns_transient_energy_r: c_float = if 0.7f32 > lambda / 140.0f32 {
        lambda / 140.0f32
    } else {
        0.7f32
    };
    let mut refbits: c_int = ((*avctx).bit_rate as c_double * 1024.0f64
        / (*avctx).sample_rate as c_double
        / (if (*avctx).flags & (1 as c_int) << 1 as c_int != 0 {
            2.0f32
        } else {
            (*avctx).ch_layout.nb_channels as c_float
        }) as c_double
        * (lambda / 120.0f32) as c_double) as c_int;
    let mut rate_bandwidth_multiplier: c_float = 1.5f32;
    let mut prev: c_int = -(1000 as c_int);
    let mut prev_sf: c_int = -(1 as c_int);
    let mut frame_bit_rate: c_int = (if (*avctx).flags & (1 as c_int) << 1 as c_int != 0 {
        refbits as c_float * rate_bandwidth_multiplier * (*avctx).sample_rate as c_float
            / 1024 as c_int as c_float
    } else {
        ((*avctx).bit_rate / (*avctx).ch_layout.nb_channels as c_long) as c_float
    }) as c_int;
    frame_bit_rate = (frame_bit_rate as c_float * 1.15f32) as c_int;
    if (*avctx).cutoff > 0 as c_int {
        bandwidth = (*avctx).cutoff;
    } else {
        bandwidth = if 3000 as c_int
            > (if frame_bit_rate != 0 {
                if (if (if (if (if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                {
                    3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                } else if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
                {
                    12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
                } else if (if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                {
                    3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                } else if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 22000 as c_int
                {
                    22000 as c_int
                } else if (if (if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                {
                    3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                } else if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
                {
                    12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
                } else if (if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                {
                    3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                } else if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > (*avctx).sample_rate / 2 as c_int
                {
                    (*avctx).sample_rate / 2 as c_int
                } else if (if (if (if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                {
                    3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                } else if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
                {
                    12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
                } else if (if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                {
                    3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                } else if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 22000 as c_int
                {
                    22000 as c_int
                } else if (if (if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                {
                    3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                } else if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
                {
                    12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
                } else if (if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                {
                    3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                } else if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }
            } else {
                (*avctx).sample_rate / 2 as c_int
            }) {
            3000 as c_int
        } else if frame_bit_rate != 0 {
            if (if (if (if (if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            {
                3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            } else if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
            {
                12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
            } else if (if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            {
                3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            } else if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 22000 as c_int
            {
                22000 as c_int
            } else if (if (if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            {
                3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            } else if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
            {
                12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
            } else if (if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            {
                3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            } else if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > (*avctx).sample_rate / 2 as c_int
            {
                (*avctx).sample_rate / 2 as c_int
            } else if (if (if (if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            {
                3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            } else if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
            {
                12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
            } else if (if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            {
                3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            } else if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 22000 as c_int
            {
                22000 as c_int
            } else if (if (if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            {
                3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            } else if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
            {
                12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
            } else if (if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            {
                3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            } else if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }
        } else {
            (*avctx).sample_rate / 2 as c_int
        };
    }
    cutoff = bandwidth * 2 as c_int * wlen / (*avctx).sample_rate;
    (*sce).band_alt = (*sce).band_type;
    ff_init_nextband_map(sce, nextband.as_mut_ptr());
    w = 0 as c_int;
    while w < (*sce).ics.num_windows {
        let mut wstart: c_int = w * 128 as c_int;
        let mut current_block_67: u64;
        g = 0 as c_int;
        while g < (*sce).ics.num_swb {
            let mut noise_sfi: c_int = 0;
            let mut dist1: c_float = 0.0f32;
            let mut dist2: c_float = 0.0f32;
            let mut noise_amp: c_float = 0.;
            let mut pns_energy: c_float = 0.0f32;
            let mut pns_tgt_energy: c_float = 0.;
            let mut energy_ratio: c_float = 0.;
            let mut dist_thresh: c_float = 0.;
            let mut sfb_energy: c_float = 0.0f32;
            let mut threshold: c_float = 0.0f32;
            let mut spread: c_float = 2.0f32;
            let mut min_energy: c_float = -1.0f32;
            let mut max_energy: c_float = 0.0f32;
            let start: c_int = wstart + *((*sce).ics.swb_offset).offset(g as isize) as c_int;
            let freq: c_float = (start - wstart) as c_float * freq_mult;
            let freq_boost: c_float = if 0.88f32 * freq / 4000 as c_int as c_float > 1.0f32 {
                0.88f32 * freq / 4000 as c_int as c_float
            } else {
                1.0f32
            };
            if freq < 4000 as c_int as c_float || start - wstart >= cutoff {
                if (*sce).zeroes[(w * 16 as c_int + g) as usize] == 0 {
                    prev_sf = (*sce).sf_idx[(w * 16 as c_int + g) as usize];
                }
            } else {
                w2 = 0 as c_int;
                while w2 < (*sce).ics.group_len[w as usize] as c_int {
                    band = &mut *((*((*s).psy.ch).offset((*s).cur_channel as isize)).psy_bands)
                        .as_mut_ptr()
                        .offset(((w + w2) * 16 as c_int + g) as isize)
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
                dist_thresh =
                    (2.5f32 * 4000 as c_int as c_float / freq).clamp(0.5, 2.5) * dist_bias;
                if (*sce).zeroes[(w * 16 as c_int + g) as usize] == 0
                    && ff_sfdelta_can_remove_band(
                        sce,
                        nextband.as_mut_ptr(),
                        prev_sf,
                        w * 16 as c_int + g,
                    ) == 0
                    || ((*sce).zeroes[(w * 16 as c_int + g) as usize] as c_int != 0
                        || (*sce).band_alt[(w * 16 as c_int + g) as usize] as u64 == 0)
                        && sfb_energy < threshold * sqrtf(1.0f32 / freq_boost)
                    || spread < spread_threshold
                    || (*sce).zeroes[(w * 16 as c_int + g) as usize] == 0
                        && (*sce).band_alt[(w * 16 as c_int + g) as usize] as c_uint != 0
                        && sfb_energy > threshold * thr_mult * freq_boost
                    || min_energy < pns_transient_energy_r * max_energy
                {
                    (*sce).pns_ener[(w * 16 as c_int + g) as usize] = sfb_energy;
                    if (*sce).zeroes[(w * 16 as c_int + g) as usize] == 0 {
                        prev_sf = (*sce).sf_idx[(w * 16 as c_int + g) as usize];
                    }
                } else {
                    pns_tgt_energy = sfb_energy
                        * (if 1.0f32 > spread * spread {
                            spread * spread
                        } else {
                            1.0f32
                        });
                    noise_sfi = av_clip_c(
                        roundf(log2f(pns_tgt_energy) * 2 as c_int as c_float) as c_int,
                        -(100 as c_int),
                        155 as c_int,
                    );
                    noise_amp = -POW_SF_TABLES.pow2[(noise_sfi + 200 as c_int) as usize];
                    if prev != -(1000 as c_int) {
                        let mut noise_sfdiff: c_int = noise_sfi - prev + 60 as c_int;
                        if noise_sfdiff < 0 as c_int || noise_sfdiff > 2 as c_int * 60 as c_int {
                            if (*sce).zeroes[(w * 16 as c_int + g) as usize] == 0 {
                                prev_sf = (*sce).sf_idx[(w * 16 as c_int + g) as usize];
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
                            w2 = 0 as c_int;
                            while w2 < (*sce).ics.group_len[w as usize] as c_int {
                                let mut band_energy: c_float = 0.;
                                let mut scale: c_float = 0.;
                                let mut pns_senergy: c_float = 0.;
                                let start_c: c_int = (w + w2) * 128 as c_int
                                    + *((*sce).ics.swb_offset).offset(g as isize) as c_int;
                                band = &mut *((*((*s).psy.ch).offset((*s).cur_channel as isize))
                                    .psy_bands)
                                    .as_mut_ptr()
                                    .offset(((w + w2) * 16 as c_int + g) as isize)
                                    as *mut FFPsyBand;
                                i = 0 as c_int;
                                while i < *((*sce).ics.swb_sizes).offset(g as isize) as c_int {
                                    (*s).random_state = lcg_random((*s).random_state as c_uint);
                                    *PNS.offset(i as isize) = (*s).random_state as c_float;
                                    i += 1;
                                    i;
                                }
                                band_energy = ((*(*s).fdsp).scalarproduct_float)
                                    .expect("non-null function pointer")(
                                    PNS,
                                    PNS,
                                    *((*sce).ics.swb_sizes).offset(g as isize) as c_int,
                                );
                                scale = noise_amp / sqrtf(band_energy);
                                ((*(*s).fdsp).vector_fmul_scalar)
                                    .expect("non-null function pointer")(
                                    PNS,
                                    PNS,
                                    scale,
                                    *((*sce).ics.swb_sizes).offset(g as isize) as c_int,
                                );
                                pns_senergy = ((*(*s).fdsp).scalarproduct_float)
                                    .expect("non-null function pointer")(
                                    PNS,
                                    PNS,
                                    *((*sce).ics.swb_sizes).offset(g as isize) as c_int,
                                );
                                pns_energy += pns_senergy;
                                ((*s).abs_pow34).expect("non-null function pointer")(
                                    NOR34,
                                    &mut *((*sce).coeffs).as_mut_ptr().offset(start_c as isize),
                                    *((*sce).ics.swb_sizes).offset(g as isize) as c_int,
                                );
                                ((*s).abs_pow34).expect("non-null function pointer")(
                                    PNS34,
                                    PNS,
                                    *((*sce).ics.swb_sizes).offset(g as isize) as c_int,
                                );
                                dist1 += quantize_band_cost(
                                    s,
                                    &mut *((*sce).coeffs).as_mut_ptr().offset(start_c as isize),
                                    NOR34,
                                    *((*sce).ics.swb_sizes).offset(g as isize) as c_int,
                                    (*sce).sf_idx[((w + w2) * 16 as c_int + g) as usize],
                                    (*sce).band_alt[((w + w2) * 16 as c_int + g) as usize] as c_int,
                                    lambda / (*band).threshold,
                                    ::core::f32::INFINITY,
                                    ptr::null_mut::<c_int>(),
                                    ptr::null_mut::<c_float>(),
                                );
                                dist2 += (*band).energy / ((*band).spread * (*band).spread)
                                    * lambda
                                    * dist_thresh
                                    / (*band).threshold;
                                w2 += 1;
                                w2;
                            }
                            if g != 0
                                && (*sce).band_type[(w * 16 as c_int + g - 1 as c_int) as usize]
                                    as c_uint
                                    == NOISE_BT as c_int as c_uint
                            {
                                dist2 += 5 as c_int as c_float;
                            } else {
                                dist2 += 9 as c_int as c_float;
                            }
                            energy_ratio = pns_tgt_energy / pns_energy;
                            (*sce).pns_ener[(w * 16 as c_int + g) as usize] =
                                energy_ratio * pns_tgt_energy;
                            if (*sce).zeroes[(w * 16 as c_int + g) as usize] as c_int != 0
                                || (*sce).band_alt[(w * 16 as c_int + g) as usize] as u64 == 0
                                || energy_ratio > 0.85f32 && energy_ratio < 1.25f32 && dist2 < dist1
                            {
                                (*sce).band_type[(w * 16 as c_int + g) as usize] = NOISE_BT;
                                (*sce).zeroes[(w * 16 as c_int + g) as usize] =
                                    0 as c_int as c_uchar;
                                prev = noise_sfi;
                            } else if (*sce).zeroes[(w * 16 as c_int + g) as usize] == 0 {
                                prev_sf = (*sce).sf_idx[(w * 16 as c_int + g) as usize];
                            }
                        }
                    }
                }
            }
            g += 1;
            g;
        }
        w += (*sce).ics.group_len[w as usize] as c_int;
    }
}
unsafe extern "C" fn mark_pns(
    mut s: *mut AACEncContext,
    mut avctx: *mut AVCodecContext,
    mut sce: *mut SingleChannelElement,
) {
    let mut band: *mut FFPsyBand = ptr::null_mut::<FFPsyBand>();
    let mut w: c_int = 0;
    let mut g: c_int = 0;
    let mut w2: c_int = 0;
    let mut wlen: c_int = 1024 as c_int / (*sce).ics.num_windows;
    let mut bandwidth: c_int = 0;
    let mut cutoff: c_int = 0;
    let lambda: c_float = (*s).lambda;
    let freq_mult: c_float = (*avctx).sample_rate as c_float * 0.5f32 / wlen as c_float;
    let spread_threshold: c_float = if 0.75f32
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
    let pns_transient_energy_r: c_float = if 0.7f32 > lambda / 140.0f32 {
        lambda / 140.0f32
    } else {
        0.7f32
    };
    let mut refbits: c_int = ((*avctx).bit_rate as c_double * 1024.0f64
        / (*avctx).sample_rate as c_double
        / (if (*avctx).flags & (1 as c_int) << 1 as c_int != 0 {
            2.0f32
        } else {
            (*avctx).ch_layout.nb_channels as c_float
        }) as c_double
        * (lambda / 120.0f32) as c_double) as c_int;
    let mut rate_bandwidth_multiplier: c_float = 1.5f32;
    let mut frame_bit_rate: c_int = (if (*avctx).flags & (1 as c_int) << 1 as c_int != 0 {
        refbits as c_float * rate_bandwidth_multiplier * (*avctx).sample_rate as c_float
            / 1024 as c_int as c_float
    } else {
        ((*avctx).bit_rate / (*avctx).ch_layout.nb_channels as c_long) as c_float
    }) as c_int;
    frame_bit_rate = (frame_bit_rate as c_float * 1.15f32) as c_int;
    if (*avctx).cutoff > 0 as c_int {
        bandwidth = (*avctx).cutoff;
    } else {
        bandwidth = if 3000 as c_int
            > (if frame_bit_rate != 0 {
                if (if (if (if (if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                {
                    3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                } else if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
                {
                    12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
                } else if (if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                {
                    3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                } else if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 22000 as c_int
                {
                    22000 as c_int
                } else if (if (if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                {
                    3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                } else if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
                {
                    12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
                } else if (if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                {
                    3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                } else if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > (*avctx).sample_rate / 2 as c_int
                {
                    (*avctx).sample_rate / 2 as c_int
                } else if (if (if (if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                {
                    3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                } else if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
                {
                    12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
                } else if (if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                {
                    3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                } else if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 22000 as c_int
                {
                    22000 as c_int
                } else if (if (if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                {
                    3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                } else if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
                {
                    12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
                } else if (if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                {
                    3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
                } else if frame_bit_rate / 1 as c_int / 5 as c_int
                    > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                {
                    frame_bit_rate / 1 as c_int / 5 as c_int
                } else {
                    frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
                }
            } else {
                (*avctx).sample_rate / 2 as c_int
            }) {
            3000 as c_int
        } else if frame_bit_rate != 0 {
            if (if (if (if (if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            {
                3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            } else if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
            {
                12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
            } else if (if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            {
                3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            } else if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 22000 as c_int
            {
                22000 as c_int
            } else if (if (if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            {
                3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            } else if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
            {
                12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
            } else if (if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            {
                3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            } else if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > (*avctx).sample_rate / 2 as c_int
            {
                (*avctx).sample_rate / 2 as c_int
            } else if (if (if (if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            {
                3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            } else if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
            {
                12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
            } else if (if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            {
                3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            } else if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 22000 as c_int
            {
                22000 as c_int
            } else if (if (if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            {
                3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            } else if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
            {
                12000 as c_int + frame_bit_rate / 1 as c_int / 16 as c_int
            } else if (if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }) > 3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            {
                3000 as c_int + frame_bit_rate / 1 as c_int / 4 as c_int
            } else if frame_bit_rate / 1 as c_int / 5 as c_int
                > frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            {
                frame_bit_rate / 1 as c_int / 5 as c_int
            } else {
                frame_bit_rate / 1 as c_int * 15 as c_int / 32 as c_int - 5500 as c_int
            }
        } else {
            (*avctx).sample_rate / 2 as c_int
        };
    }
    cutoff = bandwidth * 2 as c_int * wlen / (*avctx).sample_rate;
    (*sce).band_alt = (*sce).band_type;
    w = 0 as c_int;
    while w < (*sce).ics.num_windows {
        g = 0 as c_int;
        while g < (*sce).ics.num_swb {
            let mut sfb_energy: c_float = 0.0f32;
            let mut threshold: c_float = 0.0f32;
            let mut spread: c_float = 2.0f32;
            let mut min_energy: c_float = -1.0f32;
            let mut max_energy: c_float = 0.0f32;
            let start: c_int = *((*sce).ics.swb_offset).offset(g as isize) as c_int;
            let freq: c_float = start as c_float * freq_mult;
            let freq_boost: c_float = if 0.88f32 * freq / 4000 as c_int as c_float > 1.0f32 {
                0.88f32 * freq / 4000 as c_int as c_float
            } else {
                1.0f32
            };
            if freq < 4000 as c_int as c_float || start >= cutoff {
                (*sce).can_pns[(w * 16 as c_int + g) as usize] = 0 as c_int as c_uchar;
            } else {
                w2 = 0 as c_int;
                while w2 < (*sce).ics.group_len[w as usize] as c_int {
                    band = &mut *((*((*s).psy.ch).offset((*s).cur_channel as isize)).psy_bands)
                        .as_mut_ptr()
                        .offset(((w + w2) * 16 as c_int + g) as isize)
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
                (*sce).pns_ener[(w * 16 as c_int + g) as usize] = sfb_energy;
                if sfb_energy < threshold * sqrtf(1.5f32 / freq_boost)
                    || spread < spread_threshold
                    || min_energy < pns_transient_energy_r * max_energy
                {
                    (*sce).can_pns[(w * 16 as c_int + g) as usize] = 0 as c_int as c_uchar;
                } else {
                    (*sce).can_pns[(w * 16 as c_int + g) as usize] = 1 as c_int as c_uchar;
                }
            }
            g += 1;
            g;
        }
        w += (*sce).ics.group_len[w as usize] as c_int;
    }
}
unsafe extern "C" fn search_for_ms(mut s: *mut AACEncContext, mut cpe: *mut ChannelElement) {
    let mut start: c_int = 0 as c_int;
    let mut i: c_int = 0;
    let mut w: c_int = 0;
    let mut w2: c_int = 0;
    let mut g: c_int = 0;
    let mut sid_sf_boost: c_int = 0;
    let mut prev_mid: c_int = 0;
    let mut prev_side: c_int = 0;
    let mut nextband0: [c_uchar; 128] = [0; 128];
    let mut nextband1: [c_uchar; 128] = [0; 128];
    let mut M: *mut c_float = ((*s).scoefs)
        .as_mut_ptr()
        .offset((128 as c_int * 0 as c_int) as isize);
    let mut S: *mut c_float = ((*s).scoefs)
        .as_mut_ptr()
        .offset((128 as c_int * 1 as c_int) as isize);
    let mut L34: *mut c_float = ((*s).scoefs)
        .as_mut_ptr()
        .offset((128 as c_int * 2 as c_int) as isize);
    let mut R34: *mut c_float = ((*s).scoefs)
        .as_mut_ptr()
        .offset((128 as c_int * 3 as c_int) as isize);
    let mut M34: *mut c_float = ((*s).scoefs)
        .as_mut_ptr()
        .offset((128 as c_int * 4 as c_int) as isize);
    let mut S34: *mut c_float = ((*s).scoefs)
        .as_mut_ptr()
        .offset((128 as c_int * 5 as c_int) as isize);
    let lambda: c_float = (*s).lambda;
    let mslambda: c_float = if 1.0f32 > lambda / 120.0f32 {
        lambda / 120.0f32
    } else {
        1.0f32
    };
    let mut sce0: *mut SingleChannelElement =
        &mut *((*cpe).ch).as_mut_ptr().offset(0 as c_int as isize) as *mut SingleChannelElement;
    let mut sce1: *mut SingleChannelElement =
        &mut *((*cpe).ch).as_mut_ptr().offset(1 as c_int as isize) as *mut SingleChannelElement;
    if (*cpe).common_window == 0 {
        return;
    }
    ff_init_nextband_map(sce0, nextband0.as_mut_ptr());
    ff_init_nextband_map(sce1, nextband1.as_mut_ptr());
    prev_mid = (*sce0).sf_idx[0 as c_int as usize];
    prev_side = (*sce1).sf_idx[0 as c_int as usize];
    w = 0 as c_int;
    while w < (*sce0).ics.num_windows {
        start = 0 as c_int;
        g = 0 as c_int;
        while g < (*sce0).ics.num_swb {
            let mut bmax: c_float =
                bval2bmax(g as c_float * 17.0f32 / (*sce0).ics.num_swb as c_float) / 0.0045f32;
            if (*cpe).is_mask[(w * 16 as c_int + g) as usize] == 0 {
                (*cpe).ms_mask[(w * 16 as c_int + g) as usize] = 0 as c_int as c_uchar;
            }
            if (*sce0).zeroes[(w * 16 as c_int + g) as usize] == 0
                && (*sce1).zeroes[(w * 16 as c_int + g) as usize] == 0
                && (*cpe).is_mask[(w * 16 as c_int + g) as usize] == 0
            {
                let mut Mmax: c_float = 0.0f32;
                let mut Smax: c_float = 0.0f32;
                w2 = 0 as c_int;
                while w2 < (*sce0).ics.group_len[w as usize] as c_int {
                    i = 0 as c_int;
                    while i < *((*sce0).ics.swb_sizes).offset(g as isize) as c_int {
                        *M.offset(i as isize) = (((*sce0).coeffs
                            [(start + (w + w2) * 128 as c_int + i) as usize]
                            + (*sce1).coeffs[(start + (w + w2) * 128 as c_int + i) as usize])
                            as c_double
                            * 0.5f64) as c_float;
                        *S.offset(i as isize) = *M.offset(i as isize)
                            - (*sce1).coeffs[(start + (w + w2) * 128 as c_int + i) as usize];
                        i += 1;
                        i;
                    }
                    ((*s).abs_pow34).expect("non-null function pointer")(
                        M34,
                        M,
                        *((*sce0).ics.swb_sizes).offset(g as isize) as c_int,
                    );
                    ((*s).abs_pow34).expect("non-null function pointer")(
                        S34,
                        S,
                        *((*sce0).ics.swb_sizes).offset(g as isize) as c_int,
                    );
                    i = 0 as c_int;
                    while i < *((*sce0).ics.swb_sizes).offset(g as isize) as c_int {
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
                sid_sf_boost = 0 as c_int;
                while sid_sf_boost < 4 as c_int {
                    let mut dist1: c_float = 0.0f32;
                    let mut dist2: c_float = 0.0f32;
                    let mut B0: c_int = 0 as c_int;
                    let mut B1: c_int = 0 as c_int;
                    let mut minidx: c_int = 0;
                    let mut mididx: c_int = 0;
                    let mut sididx: c_int = 0;
                    let mut midcb: c_int = 0;
                    let mut sidcb: c_int = 0;
                    minidx = if (*sce0).sf_idx[(w * 16 as c_int + g) as usize]
                        > (*sce1).sf_idx[(w * 16 as c_int + g) as usize]
                    {
                        (*sce1).sf_idx[(w * 16 as c_int + g) as usize]
                    } else {
                        (*sce0).sf_idx[(w * 16 as c_int + g) as usize]
                    };
                    mididx = av_clip_c(minidx, 0 as c_int, 255 as c_int - 36 as c_int);
                    sididx = av_clip_c(
                        minidx - sid_sf_boost * 3 as c_int,
                        0 as c_int,
                        255 as c_int - 36 as c_int,
                    );
                    if !((*sce0).band_type[(w * 16 as c_int + g) as usize] as c_uint
                        != NOISE_BT as c_int as c_uint
                        && (*sce1).band_type[(w * 16 as c_int + g) as usize] as c_uint
                            != NOISE_BT as c_int as c_uint
                        && (ff_sfdelta_can_replace(
                            sce0,
                            nextband0.as_mut_ptr(),
                            prev_mid,
                            mididx,
                            w * 16 as c_int + g,
                        ) == 0
                            || ff_sfdelta_can_replace(
                                sce1,
                                nextband1.as_mut_ptr(),
                                prev_side,
                                sididx,
                                w * 16 as c_int + g,
                            ) == 0))
                    {
                        midcb = find_min_book(Mmax, mididx);
                        sidcb = find_min_book(Smax, sididx);
                        midcb = if 1 as c_int > midcb {
                            1 as c_int
                        } else {
                            midcb
                        };
                        sidcb = if 1 as c_int > sidcb {
                            1 as c_int
                        } else {
                            sidcb
                        };
                        w2 = 0 as c_int;
                        while w2 < (*sce0).ics.group_len[w as usize] as c_int {
                            let mut band0: *mut FFPsyBand = &mut *((*((*s).psy.ch)
                                .offset(((*s).cur_channel + 0 as c_int) as isize))
                            .psy_bands)
                                .as_mut_ptr()
                                .offset(((w + w2) * 16 as c_int + g) as isize)
                                as *mut FFPsyBand;
                            let mut band1: *mut FFPsyBand = &mut *((*((*s).psy.ch)
                                .offset(((*s).cur_channel + 1 as c_int) as isize))
                            .psy_bands)
                                .as_mut_ptr()
                                .offset(((w + w2) * 16 as c_int + g) as isize)
                                as *mut FFPsyBand;
                            let mut minthr: c_float = if (*band0).threshold > (*band1).threshold {
                                (*band1).threshold
                            } else {
                                (*band0).threshold
                            };
                            let mut b1: c_int = 0;
                            let mut b2: c_int = 0;
                            let mut b3: c_int = 0;
                            let mut b4: c_int = 0;
                            i = 0 as c_int;
                            while i < *((*sce0).ics.swb_sizes).offset(g as isize) as c_int {
                                *M.offset(i as isize) = (((*sce0).coeffs
                                    [(start + (w + w2) * 128 as c_int + i) as usize]
                                    + (*sce1).coeffs
                                        [(start + (w + w2) * 128 as c_int + i) as usize])
                                    as c_double
                                    * 0.5f64)
                                    as c_float;
                                *S.offset(i as isize) = *M.offset(i as isize)
                                    - (*sce1).coeffs
                                        [(start + (w + w2) * 128 as c_int + i) as usize];
                                i += 1;
                                i;
                            }
                            ((*s).abs_pow34).expect("non-null function pointer")(
                                L34,
                                ((*sce0).coeffs)
                                    .as_mut_ptr()
                                    .offset(start as isize)
                                    .offset(((w + w2) * 128 as c_int) as isize),
                                *((*sce0).ics.swb_sizes).offset(g as isize) as c_int,
                            );
                            ((*s).abs_pow34).expect("non-null function pointer")(
                                R34,
                                ((*sce1).coeffs)
                                    .as_mut_ptr()
                                    .offset(start as isize)
                                    .offset(((w + w2) * 128 as c_int) as isize),
                                *((*sce0).ics.swb_sizes).offset(g as isize) as c_int,
                            );
                            ((*s).abs_pow34).expect("non-null function pointer")(
                                M34,
                                M,
                                *((*sce0).ics.swb_sizes).offset(g as isize) as c_int,
                            );
                            ((*s).abs_pow34).expect("non-null function pointer")(
                                S34,
                                S,
                                *((*sce0).ics.swb_sizes).offset(g as isize) as c_int,
                            );
                            dist1 += quantize_band_cost(
                                s,
                                &mut *((*sce0).coeffs)
                                    .as_mut_ptr()
                                    .offset((start + (w + w2) * 128 as c_int) as isize),
                                L34,
                                *((*sce0).ics.swb_sizes).offset(g as isize) as c_int,
                                (*sce0).sf_idx[(w * 16 as c_int + g) as usize],
                                (*sce0).band_type[(w * 16 as c_int + g) as usize] as c_int,
                                lambda / ((*band0).threshold + 1.175_494_4e-38_f32),
                                ::core::f32::INFINITY,
                                &mut b1,
                                ptr::null_mut::<c_float>(),
                            );
                            dist1 += quantize_band_cost(
                                s,
                                &mut *((*sce1).coeffs)
                                    .as_mut_ptr()
                                    .offset((start + (w + w2) * 128 as c_int) as isize),
                                R34,
                                *((*sce1).ics.swb_sizes).offset(g as isize) as c_int,
                                (*sce1).sf_idx[(w * 16 as c_int + g) as usize],
                                (*sce1).band_type[(w * 16 as c_int + g) as usize] as c_int,
                                lambda / ((*band1).threshold + 1.175_494_4e-38_f32),
                                ::core::f32::INFINITY,
                                &mut b2,
                                ptr::null_mut::<c_float>(),
                            );
                            dist2 += quantize_band_cost(
                                s,
                                M,
                                M34,
                                *((*sce0).ics.swb_sizes).offset(g as isize) as c_int,
                                mididx,
                                midcb,
                                lambda / (minthr + 1.175_494_4e-38_f32),
                                ::core::f32::INFINITY,
                                &mut b3,
                                ptr::null_mut::<c_float>(),
                            );
                            dist2 += quantize_band_cost(
                                s,
                                S,
                                S34,
                                *((*sce1).ics.swb_sizes).offset(g as isize) as c_int,
                                sididx,
                                sidcb,
                                mslambda / (minthr * bmax + 1.175_494_4e-38_f32),
                                ::core::f32::INFINITY,
                                &mut b4,
                                ptr::null_mut::<c_float>(),
                            );
                            B0 += b1 + b2;
                            B1 += b3 + b4;
                            dist1 -= (b1 + b2) as c_float;
                            dist2 -= (b3 + b4) as c_float;
                            w2 += 1;
                            w2;
                        }
                        (*cpe).ms_mask[(w * 16 as c_int + g) as usize] =
                            (dist2 <= dist1 && B1 < B0) as c_int as c_uchar;
                        if (*cpe).ms_mask[(w * 16 as c_int + g) as usize] != 0 {
                            if (*sce0).band_type[(w * 16 as c_int + g) as usize] as c_uint
                                != NOISE_BT as c_int as c_uint
                                && (*sce1).band_type[(w * 16 as c_int + g) as usize] as c_uint
                                    != NOISE_BT as c_int as c_uint
                            {
                                (*sce0).sf_idx[(w * 16 as c_int + g) as usize] = mididx;
                                (*sce1).sf_idx[(w * 16 as c_int + g) as usize] = sididx;
                                (*sce0).band_type[(w * 16 as c_int + g) as usize] =
                                    midcb as BandType;
                                (*sce1).band_type[(w * 16 as c_int + g) as usize] =
                                    sidcb as BandType;
                            } else if ((*sce0).band_type[(w * 16 as c_int + g) as usize] as c_uint
                                != NOISE_BT as c_int as c_uint)
                                as c_int
                                ^ ((*sce1).band_type[(w * 16 as c_int + g) as usize] as c_uint
                                    != NOISE_BT as c_int as c_uint)
                                    as c_int
                                != 0
                            {
                                (*cpe).ms_mask[(w * 16 as c_int + g) as usize] =
                                    0 as c_int as c_uchar;
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
            if (*sce0).zeroes[(w * 16 as c_int + g) as usize] == 0
                && ((*sce0).band_type[(w * 16 as c_int + g) as usize] as c_uint)
                    < RESERVED_BT as c_int as c_uint
            {
                prev_mid = (*sce0).sf_idx[(w * 16 as c_int + g) as usize];
            }
            if (*sce1).zeroes[(w * 16 as c_int + g) as usize] == 0
                && (*cpe).is_mask[(w * 16 as c_int + g) as usize] == 0
                && ((*sce1).band_type[(w * 16 as c_int + g) as usize] as c_uint)
                    < RESERVED_BT as c_int as c_uint
            {
                prev_side = (*sce1).sf_idx[(w * 16 as c_int + g) as usize];
            }
            start += *((*sce0).ics.swb_sizes).offset(g as isize) as c_int;
            g += 1;
            g;
        }
        w += (*sce0).ics.group_len[w as usize] as c_int;
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
                search_for_quantizers: Some(quantizers::twoloop::search),
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
    BUF_BITS = (8 as c_int as c_ulong).wrapping_mul(size_of::<BitBuf>() as c_ulong) as c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe fn(); 1] = [run_static_initializers];
