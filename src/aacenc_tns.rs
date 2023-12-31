#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use std::mem::size_of;

use libc::{c_double, c_float, c_int, c_long, c_uchar, c_uint, c_ulong};

use crate::{aacenc::ctx::AACEncContext, common::*, types::*};

static mut BUF_BITS: c_int = 0;
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
#[inline]
unsafe fn put_bits(mut s: *mut PutBitContext, mut n: c_int, mut value: BitBuf) {
    put_bits_no_assert(s, n, value);
}
#[inline]
unsafe fn compute_lpc_coefs(
    mut autoc: *const LPC_TYPE,
    mut max_order: c_int,
    mut lpc: *mut LPC_TYPE,
    mut lpc_stride: c_int,
    mut fail: c_int,
    mut normalize: c_int,
) -> c_int {
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut err: LPC_TYPE = 0 as c_int as LPC_TYPE;
    let mut lpc_last: *mut LPC_TYPE = lpc;
    if normalize != 0 {
        let fresh0 = autoc;
        autoc = autoc.offset(1);
        err = *fresh0;
    }
    if fail != 0
        && (*autoc.offset((max_order - 1 as c_int) as isize) == 0 as c_int as c_float
            || err <= 0 as c_int as c_float)
    {
        return -(1 as c_int);
    }
    i = 0 as c_int;
    while i < max_order {
        let mut r: LPC_TYPE = -*autoc.offset(i as isize);
        if normalize != 0 {
            j = 0 as c_int;
            while j < i {
                r -= *lpc_last.offset(j as isize) * *autoc.offset((i - j - 1 as c_int) as isize);
                j += 1;
                j;
            }
            if err != 0. {
                r /= err;
            }
            err *= 1.0f64 as c_float - r * r;
        }
        *lpc.offset(i as isize) = r;
        j = 0 as c_int;
        while j < i + 1 as c_int >> 1 as c_int {
            let mut f: LPC_TYPE = *lpc_last.offset(j as isize);
            let mut b: LPC_TYPE = *lpc_last.offset((i - 1 as c_int - j) as isize);
            *lpc.offset(j as isize) = f + r * b;
            *lpc.offset((i - 1 as c_int - j) as isize) = b + r * f;
            j += 1;
            j;
        }
        if fail != 0 && err < 0 as c_int as c_float {
            return -(1 as c_int);
        }
        lpc_last = lpc;
        lpc = lpc.offset(lpc_stride as isize);
        i += 1;
        i;
    }
    0 as c_int
}
static mut tns_tmp2_map_1_3: [c_float; 4] = [
    0.00000000f64 as c_float,
    -0.43388373f64 as c_float,
    0.64278758f64 as c_float,
    0.34202015f64 as c_float,
];
static mut tns_tmp2_map_0_3: [c_float; 8] = [
    0.00000000f64 as c_float,
    -0.43388373f64 as c_float,
    -0.78183150f64 as c_float,
    -0.97492790f64 as c_float,
    0.98480773f64 as c_float,
    0.86602539f64 as c_float,
    0.64278758f64 as c_float,
    0.34202015f64 as c_float,
];
static mut tns_tmp2_map_1_4: [c_float; 8] = [
    0.00000000f64 as c_float,
    -0.20791170f64 as c_float,
    -0.40673664f64 as c_float,
    -0.58778524f64 as c_float,
    0.67369562f64 as c_float,
    0.52643216f64 as c_float,
    0.36124167f64 as c_float,
    0.18374951f64 as c_float,
];
static mut tns_tmp2_map_0_4: [c_float; 16] = [
    0.00000000f64 as c_float,
    -0.20791170f64 as c_float,
    -0.40673664f64 as c_float,
    -0.58778524f64 as c_float,
    -0.74314481f64 as c_float,
    -0.86602539f64 as c_float,
    -0.95105654f64 as c_float,
    -0.99452192f64 as c_float,
    0.99573416f64 as c_float,
    0.96182561f64 as c_float,
    0.89516330f64 as c_float,
    0.79801720f64 as c_float,
    0.67369562f64 as c_float,
    0.52643216f64 as c_float,
    0.36124167f64 as c_float,
    0.18374951f64 as c_float,
];
static mut tns_tmp2_map: [*const c_float; 4] = unsafe {
    [
        tns_tmp2_map_0_3.as_ptr(),
        tns_tmp2_map_0_4.as_ptr(),
        tns_tmp2_map_1_3.as_ptr(),
        tns_tmp2_map_1_4.as_ptr(),
    ]
};
static mut tns_min_sfb: [*const c_uchar; 2] =
    unsafe { [tns_min_sfb_long.as_ptr(), tns_min_sfb_short.as_ptr()] };
static mut tns_min_sfb_short: [c_uchar; 16] = [
    2 as c_int as c_uchar,
    2 as c_int as c_uchar,
    2 as c_int as c_uchar,
    3 as c_int as c_uchar,
    3 as c_int as c_uchar,
    4 as c_int as c_uchar,
    6 as c_int as c_uchar,
    6 as c_int as c_uchar,
    8 as c_int as c_uchar,
    10 as c_int as c_uchar,
    10 as c_int as c_uchar,
    12 as c_int as c_uchar,
    12 as c_int as c_uchar,
    12 as c_int as c_uchar,
    12 as c_int as c_uchar,
    12 as c_int as c_uchar,
];
static mut tns_min_sfb_long: [c_uchar; 16] = [
    12 as c_int as c_uchar,
    13 as c_int as c_uchar,
    15 as c_int as c_uchar,
    16 as c_int as c_uchar,
    17 as c_int as c_uchar,
    20 as c_int as c_uchar,
    25 as c_int as c_uchar,
    26 as c_int as c_uchar,
    24 as c_int as c_uchar,
    28 as c_int as c_uchar,
    30 as c_int as c_uchar,
    31 as c_int as c_uchar,
    31 as c_int as c_uchar,
    31 as c_int as c_uchar,
    31 as c_int as c_uchar,
    31 as c_int as c_uchar,
];
#[inline]
unsafe fn quant_array_idx(val: c_float, mut arr: *const c_float, num: c_int) -> c_int {
    let mut i: c_int = 0;
    let mut index: c_int = 0 as c_int;
    let mut quant_min_err: c_float = ::core::f32::INFINITY;
    i = 0 as c_int;
    while i < num {
        let mut error: c_float = (val - *arr.offset(i as isize)) * (val - *arr.offset(i as isize));
        if error < quant_min_err {
            quant_min_err = error;
            index = i;
        }
        i += 1;
        i;
    }
    index
}
#[inline]
unsafe fn compress_coeffs(mut coef: *mut c_int, mut order: c_int, mut c_bits: c_int) -> c_int {
    let mut i: c_int = 0;
    let low_idx: c_int = if c_bits != 0 { 4 as c_int } else { 2 as c_int };
    let shift_val: c_int = if c_bits != 0 { 8 as c_int } else { 4 as c_int };
    let high_idx: c_int = if c_bits != 0 { 11 as c_int } else { 5 as c_int };
    i = 0 as c_int;
    while i < order {
        if *coef.offset(i as isize) >= low_idx && *coef.offset(i as isize) <= high_idx {
            return 0 as c_int;
        }
        i += 1;
        i;
    }
    i = 0 as c_int;
    while i < order {
        *coef.offset(i as isize) -= if *coef.offset(i as isize) > high_idx {
            shift_val
        } else {
            0 as c_int
        };
        i += 1;
        i;
    }
    1 as c_int
}

pub(crate) unsafe fn encode_tns_info(
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
) {
    let mut tns: *mut TemporalNoiseShaping = &mut (*sce).tns;
    let mut i: c_int = 0;
    let mut w: c_int = 0;
    let mut filt: c_int = 0;
    let mut coef_compress: c_int = 0 as c_int;
    let mut coef_len: c_int = 0;
    let is8: c_int = ((*sce).ics.window_sequence[0 as c_int as usize] as c_uint
        == EIGHT_SHORT_SEQUENCE as c_int as c_uint) as c_int;
    let c_bits: c_int = if is8 != 0 {
        (4 as c_int == 4 as c_int) as c_int
    } else {
        (4 as c_int == 4 as c_int) as c_int
    };
    if (*sce).tns.present == 0 {
        return;
    }
    i = 0 as c_int;
    while i < (*sce).ics.num_windows {
        put_bits(
            &mut (*s).pb,
            2 as c_int - is8,
            (*sce).tns.n_filt[i as usize] as BitBuf,
        );
        if (*tns).n_filt[i as usize] != 0 {
            put_bits(&mut (*s).pb, 1 as c_int, c_bits as BitBuf);
            filt = 0 as c_int;
            while filt < (*tns).n_filt[i as usize] {
                put_bits(
                    &mut (*s).pb,
                    6 as c_int - 2 as c_int * is8,
                    (*tns).length[i as usize][filt as usize] as BitBuf,
                );
                put_bits(
                    &mut (*s).pb,
                    5 as c_int - 2 as c_int * is8,
                    (*tns).order[i as usize][filt as usize] as BitBuf,
                );
                if (*tns).order[i as usize][filt as usize] != 0 {
                    put_bits(
                        &mut (*s).pb,
                        1 as c_int,
                        (*tns).direction[i as usize][filt as usize] as BitBuf,
                    );
                    coef_compress = compress_coeffs(
                        ((*tns).coef_idx[i as usize][filt as usize]).as_mut_ptr(),
                        (*tns).order[i as usize][filt as usize],
                        c_bits,
                    );
                    put_bits(&mut (*s).pb, 1 as c_int, coef_compress as BitBuf);
                    coef_len = c_bits + 3 as c_int - coef_compress;
                    w = 0 as c_int;
                    while w < (*tns).order[i as usize][filt as usize] {
                        put_bits(
                            &mut (*s).pb,
                            coef_len,
                            (*tns).coef_idx[i as usize][filt as usize][w as usize] as BitBuf,
                        );
                        w += 1;
                        w;
                    }
                }
                filt += 1;
                filt;
            }
        }
        i += 1;
        i;
    }
}

pub(crate) unsafe fn apply_tns(mut _s: *mut AACEncContext, mut sce: *mut SingleChannelElement) {
    let mut tns: *mut TemporalNoiseShaping = &mut (*sce).tns;
    let mut ics: *mut IndividualChannelStream = &mut (*sce).ics;
    let mut w: c_int = 0;
    let mut filt: c_int = 0;
    let mut m: c_int = 0;
    let mut i: c_int = 0;
    let mut top: c_int = 0;
    let mut order: c_int = 0;
    let mut bottom: c_int = 0;
    let mut start: c_int = 0;
    let mut end: c_int = 0;
    let mut size: c_int = 0;
    let mut inc: c_int = 0;
    let mmm: c_int = if (*ics).tns_max_bands > (*ics).max_sfb as c_int {
        (*ics).max_sfb as c_int
    } else {
        (*ics).tns_max_bands
    };
    let mut lpc: [c_float; 20] = [0.; 20];
    w = 0 as c_int;
    while w < (*ics).num_windows {
        bottom = (*ics).num_swb;
        filt = 0 as c_int;
        while filt < (*tns).n_filt[w as usize] {
            top = bottom;
            bottom = if 0 as c_int > top - (*tns).length[w as usize][filt as usize] {
                0 as c_int
            } else {
                top - (*tns).length[w as usize][filt as usize]
            };
            order = (*tns).order[w as usize][filt as usize];
            if order != 0 as c_int {
                compute_lpc_coefs(
                    ((*tns).coef[w as usize][filt as usize]).as_mut_ptr(),
                    order,
                    lpc.as_mut_ptr(),
                    0 as c_int,
                    0 as c_int,
                    0 as c_int,
                );
                start = *((*ics).swb_offset)
                    .offset((if bottom > mmm { mmm } else { bottom }) as isize)
                    as c_int;
                end = *((*ics).swb_offset).offset((if top > mmm { mmm } else { top }) as isize)
                    as c_int;
                size = end - start;
                if size > 0 as c_int {
                    if (*tns).direction[w as usize][filt as usize] != 0 {
                        inc = -(1 as c_int);
                        start = end - 1 as c_int;
                    } else {
                        inc = 1 as c_int;
                    }
                    start += w * 128 as c_int;
                    m = 0 as c_int;
                    while m < size {
                        i = 1 as c_int;
                        while i <= (if m > order { order } else { m }) {
                            (*sce).coeffs[start as usize] += lpc[(i - 1 as c_int) as usize]
                                * (*sce).pcoeffs[(start - i * inc) as usize];
                            i += 1;
                            i;
                        }
                        m += 1;
                        m;
                        start += inc;
                    }
                }
            }
            filt += 1;
            filt;
        }
        w += 1;
        w;
    }
}
#[inline]
unsafe fn quantize_coefs(
    mut coef: *mut c_double,
    mut idx: *mut c_int,
    mut lpc: *mut c_float,
    mut order: c_int,
    mut c_bits: c_int,
) {
    let mut i: c_int = 0;
    let mut quant_arr: *const c_float = tns_tmp2_map[c_bits as usize];
    i = 0 as c_int;
    while i < order {
        *idx.offset(i as isize) = quant_array_idx(
            *coef.offset(i as isize) as c_float,
            quant_arr,
            if c_bits != 0 { 16 as c_int } else { 8 as c_int },
        );
        *lpc.offset(i as isize) = *quant_arr.offset(*idx.offset(i as isize) as isize);
        i += 1;
        i;
    }
}

pub(crate) unsafe fn search_for_tns(mut s: *mut AACEncContext, mut sce: *mut SingleChannelElement) {
    let mut tns: *mut TemporalNoiseShaping = &mut (*sce).tns;
    let mut w: c_int = 0;
    let mut g: c_int = 0;
    let mut count: c_int = 0 as c_int;
    let mut gain: c_double = 0.;
    let mut coefs: [c_double; 32] = [0.; 32];
    let mmm: c_int = if (*sce).ics.tns_max_bands > (*sce).ics.max_sfb as c_int {
        (*sce).ics.max_sfb as c_int
    } else {
        (*sce).ics.tns_max_bands
    };
    let is8: c_int = ((*sce).ics.window_sequence[0 as c_int as usize] as c_uint
        == EIGHT_SHORT_SEQUENCE as c_int as c_uint) as c_int;
    let c_bits: c_int = if is8 != 0 {
        (4 as c_int == 4 as c_int) as c_int
    } else {
        (4 as c_int == 4 as c_int) as c_int
    };
    let sfb_start: c_int = av_clip_c(
        *(tns_min_sfb[is8 as usize]).offset((*s).samplerate_index as isize) as c_int,
        0 as c_int,
        mmm,
    );
    let sfb_end: c_int = av_clip_c((*sce).ics.num_swb, 0 as c_int, mmm);
    let order: c_int = if is8 != 0 {
        7 as c_int
    } else if (*s).profile == 1 as c_int {
        12 as c_int
    } else {
        20 as c_int
    };
    let slant: c_int = if (*sce).ics.window_sequence[0 as c_int as usize] as c_uint
        == LONG_STOP_SEQUENCE as c_int as c_uint
    {
        1 as c_int
    } else if (*sce).ics.window_sequence[0 as c_int as usize] as c_uint
        == LONG_START_SEQUENCE as c_int as c_uint
    {
        0 as c_int
    } else {
        2 as c_int
    };
    let sfb_len: c_int = sfb_end - sfb_start;
    let coef_len: c_int = *((*sce).ics.swb_offset).offset(sfb_end as isize) as c_int
        - *((*sce).ics.swb_offset).offset(sfb_start as isize) as c_int;
    if coef_len <= 0 as c_int || sfb_len <= 0 as c_int {
        (*sce).tns.present = 0 as c_int;
        return;
    }
    w = 0 as c_int;
    while w < (*sce).ics.num_windows {
        let mut en: [c_float; 2] = [0.0f32, 0.0f32];
        let mut oc_start: c_int = 0 as c_int;
        let mut os_start: c_int = 0 as c_int;
        let mut coef_start: c_int = *((*sce).ics.swb_offset).offset(sfb_start as isize) as c_int;
        g = sfb_start;
        while g < (*sce).ics.num_swb && g <= sfb_end {
            let mut band: *mut FFPsyBand =
                &mut *((*((*s).psy.ch).offset((*s).cur_channel as isize)).psy_bands)
                    .as_mut_ptr()
                    .offset((w * 16 as c_int + g) as isize) as *mut FFPsyBand;
            if g > sfb_start + sfb_len / 2 as c_int {
                en[1 as c_int as usize] += (*band).energy;
            } else {
                en[0 as c_int as usize] += (*band).energy;
            }
            g += 1;
            g;
        }

        gain = (*s).lpc.calc_ref_coefs_f(
            &(*sce).coeffs[(w * 128 as c_int + coef_start) as usize..][..coef_len as usize],
            order,
            &mut coefs,
        );

        if !(order == 0
            || gain.is_finite() as i32 == 0
            || gain < 1.4f32 as c_double
            || gain > (1.16f32 * 1.4f32) as c_double)
        {
            (*tns).n_filt[w as usize] = if is8 != 0 {
                1 as c_int
            } else if order != 20 as c_int {
                2 as c_int
            } else {
                3 as c_int
            };
            g = 0 as c_int;
            while g < (*tns).n_filt[w as usize] {
                (*tns).direction[w as usize][g as usize] = if slant != 2 as c_int {
                    slant
                } else {
                    (en[g as usize] < en[(g == 0) as c_int as usize]) as c_int
                };
                (*tns).order[w as usize][g as usize] = if g < (*tns).n_filt[w as usize] {
                    order / (*tns).n_filt[w as usize]
                } else {
                    order - oc_start
                };
                (*tns).length[w as usize][g as usize] = if g < (*tns).n_filt[w as usize] {
                    sfb_len / (*tns).n_filt[w as usize]
                } else {
                    sfb_len - os_start
                };
                quantize_coefs(
                    &mut *coefs.as_mut_ptr().offset(oc_start as isize),
                    ((*tns).coef_idx[w as usize][g as usize]).as_mut_ptr(),
                    ((*tns).coef[w as usize][g as usize]).as_mut_ptr(),
                    (*tns).order[w as usize][g as usize],
                    c_bits,
                );
                oc_start += (*tns).order[w as usize][g as usize];
                os_start += (*tns).length[w as usize][g as usize];
                g += 1;
                g;
            }
            count += 1;
            count;
        }
        w += 1;
        w;
    }
    (*sce).tns.present = (count != 0) as c_int;
}
unsafe fn run_static_initializers() {
    BUF_BITS = (8 as c_int as c_ulong).wrapping_mul(size_of::<BitBuf>() as c_ulong) as c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe fn(); 1] = [run_static_initializers];
