#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

pub(crate) mod coder;
mod math;
pub(crate) mod ms;
pub(crate) mod pns;
pub(crate) mod quantize_and_encode_band;
mod quantizers;
mod trellis;

use std::{mem::size_of, ptr};

use ffi::codec::AVCodecContext;
use libc::{c_char, c_double, c_float, c_int, c_long, c_uchar, c_uint, c_ulong};

use self::{
    math::{ff_fast_powf, mod_uintp2_c},
    quantize_and_encode_band::quantize_and_encode_band_cost,
};
use crate::{
    aacenc::{abs_pow34_v, ctx::AACEncContext, ff_quantize_band_cost_cache_init},
    aactab::*,
    common::*,
    types::*,
};

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
struct BandCodingPath {
    prev_idx: c_int,
    cost: c_float,
    run: c_int,
}

const BUF_BITS: c_int = BitBuf::BITS as c_int;

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
    let mut a = coef * Q;
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
    let Q34: c_float = POW_SF_TABLES.pow34[(200 - sf + 140 - 36) as usize];
    let qmaxval = (maxval * Q34 + 0.4054f32) as c_int;
    aac_maxval_cb.get(qmaxval as usize).copied().unwrap_or(11) as c_int
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
    quantize_and_encode_band_cost(
        s,
        ptr::null_mut(),
        in_0,
        ptr::null_mut(),
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
    quantize_and_encode_band_cost(
        s,
        ptr::null_mut(),
        in_0,
        ptr::null_mut(),
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

/// Source: [libavcodec/psymodel.h](https://github.com/FFmpeg/FFmpeg/blob/2d9ed64859c9887d0504cd71dbd5b2c15e14251a/libavcodec/psymodel.h#L35-L40)
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

pub(crate) unsafe fn encode_window_bands_info(
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
    mut win: c_int,
    mut group_len: c_int,
    lambda: c_float,
) {
    let mut path: [[BandCodingPath; 15]; 120] = [[BandCodingPath::default(); 15]; 120];
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
    abs_pow34_v(
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
                            ptr::null_mut(),
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

pub(crate) unsafe fn set_special_band_scalefactors(
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

unsafe fn search_for_quantizers_fast(
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
    let mut dists: [c_float; 128] = [0.; 128];
    let mut uplims: [c_float; 128] = [0.; 128];
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
    abs_pow34_v(
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
                                ptr::null_mut(),
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
