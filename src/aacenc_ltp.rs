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

use crate::aaccoder::ff_quantize_and_encode_band_cost;
use crate::common::*;
use crate::types::*;

#[inline(always)]
unsafe extern "C" fn av_clip_uintp2_c(mut a: libc::c_int, mut p: libc::c_int) -> libc::c_uint {
    if a & !(((1 as libc::c_int) << p) - 1 as libc::c_int) != 0 {
        return (!a >> 31 as libc::c_int & ((1 as libc::c_int) << p) - 1 as libc::c_int)
            as libc::c_uint;
    } else {
        return a as libc::c_uint;
    };
}
#[inline(always)]
unsafe extern "C" fn av_bswap32(mut x: uint32_t) -> uint32_t {
    return (x << 8 as libc::c_int & 0xff00 as libc::c_int as libc::c_uint
        | x >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        << 16 as libc::c_int
        | ((x >> 16 as libc::c_int) << 8 as libc::c_int & 0xff00 as libc::c_int as libc::c_uint
            | x >> 16 as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint);
}
static mut BUF_BITS: libc::c_int = 0;
#[inline]
unsafe extern "C" fn put_bits_no_assert(
    mut s: *mut PutBitContext,
    mut n: libc::c_int,
    mut value: BitBuf,
) {
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
#[inline]
unsafe extern "C" fn put_bits(mut s: *mut PutBitContext, mut n: libc::c_int, mut value: BitBuf) {
    put_bits_no_assert(s, n, value);
}
#[inline]
unsafe extern "C" fn quantize_band_cost(
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
    return ff_quantize_and_encode_band_cost(
        s,
        0 as *mut PutBitContext,
        in_0,
        0 as *mut libc::c_float,
        scaled,
        size,
        scale_idx,
        cb,
        lambda,
        uplim,
        bits,
        energy,
    );
}
#[inline]
unsafe extern "C" fn quant_array_idx(
    val: libc::c_float,
    mut arr: *const libc::c_float,
    num: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut index: libc::c_int = 0 as libc::c_int;
    let mut quant_min_err: libc::c_float = ::core::f32::INFINITY;
    i = 0 as libc::c_int;
    while i < num {
        let mut error: libc::c_float =
            (val - *arr.offset(i as isize)) * (val - *arr.offset(i as isize));
        if error < quant_min_err {
            quant_min_err = error;
            index = i;
        }
        i += 1;
        i;
    }
    return index;
}
static mut ltp_coef: [INTFLOAT; 8] = [
    0.570829f64 as libc::c_float,
    0.696616f64 as libc::c_float,
    0.813004f64 as libc::c_float,
    0.911304f64 as libc::c_float,
    0.984900f64 as libc::c_float,
    1.067894f64 as libc::c_float,
    1.194601f64 as libc::c_float,
    1.369533f64 as libc::c_float,
];
#[no_mangle]
pub unsafe extern "C" fn ff_aac_encode_ltp_info(
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
    mut common_window: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut ics: *mut IndividualChannelStream = &mut (*sce).ics;
    if (*s).profile != 3 as libc::c_int || (*ics).predictor_present == 0 {
        return;
    }
    if common_window != 0 {
        put_bits(&mut (*s).pb, 1 as libc::c_int, 0 as libc::c_int as BitBuf);
    }
    put_bits(&mut (*s).pb, 1 as libc::c_int, (*ics).ltp.present as BitBuf);
    if (*ics).ltp.present == 0 {
        return;
    }
    put_bits(&mut (*s).pb, 11 as libc::c_int, (*ics).ltp.lag as BitBuf);
    put_bits(
        &mut (*s).pb,
        3 as libc::c_int,
        (*ics).ltp.coef_idx as BitBuf,
    );
    i = 0 as libc::c_int;
    while i
        < (if (*ics).max_sfb as libc::c_int > 40 as libc::c_int {
            40 as libc::c_int
        } else {
            (*ics).max_sfb as libc::c_int
        })
    {
        put_bits(
            &mut (*s).pb,
            1 as libc::c_int,
            (*ics).ltp.used[i as usize] as BitBuf,
        );
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ff_aac_ltp_insert_new_frame(mut s: *mut AACEncContext) {
    let mut i: libc::c_int = 0;
    let mut ch: libc::c_int = 0;
    let mut tag: libc::c_int = 0;
    let mut chans: libc::c_int = 0;
    let mut cur_channel: libc::c_int = 0;
    let mut start_ch: libc::c_int = 0 as libc::c_int;
    let mut cpe: *mut ChannelElement = 0 as *mut ChannelElement;
    let mut sce: *mut SingleChannelElement = 0 as *mut SingleChannelElement;
    i = 0 as libc::c_int;
    while i < *((*s).chan_map).offset(0 as libc::c_int as isize) as libc::c_int {
        cpe = &mut *((*s).cpe).offset(i as isize) as *mut ChannelElement;
        tag = *((*s).chan_map).offset((i + 1 as libc::c_int) as isize) as libc::c_int;
        chans = if tag == TYPE_CPE as libc::c_int {
            2 as libc::c_int
        } else {
            1 as libc::c_int
        };
        ch = 0 as libc::c_int;
        while ch < chans {
            sce = &mut *((*cpe).ch).as_mut_ptr().offset(ch as isize) as *mut SingleChannelElement;
            cur_channel = start_ch + ch;
            (*sce).ltp_state.copy_within(1024..2048, 0);
            ptr::copy_nonoverlapping(
                &*(*((*s).planar_samples).as_ptr().offset(cur_channel as isize))
                    .offset(2048 as libc::c_int as isize),
                &mut *((*sce).ltp_state)
                    .as_mut_ptr()
                    .offset(1024 as libc::c_int as isize),
                1024,
            );
            (*sce).ltp_state[2048..][..1024].copy_from_slice(&(*sce).ret_buf[..1024]);
            (*sce).ics.ltp.lag = 0 as libc::c_int as int16_t;
            ch += 1;
            ch;
        }
        start_ch += chans;
        i += 1;
        i;
    }
}
unsafe extern "C" fn get_lag(
    mut buf: *mut libc::c_float,
    mut new: *const libc::c_float,
    mut ltp: *mut LongTermPrediction,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut lag: libc::c_int = 0 as libc::c_int;
    let mut max_corr: libc::c_int = 0 as libc::c_int;
    let mut max_ratio: libc::c_float = 0.0f32;
    i = 0 as libc::c_int;
    while i < 2048 as libc::c_int {
        let mut corr: libc::c_float = 0.;
        let mut s0: libc::c_float = 0.0f32;
        let mut s1: libc::c_float = 0.0f32;
        let start: libc::c_int = if 0 as libc::c_int > i - 1024 as libc::c_int {
            0 as libc::c_int
        } else {
            i - 1024 as libc::c_int
        };
        j = start;
        while j < 2048 as libc::c_int {
            let idx: libc::c_int = j - i + 1024 as libc::c_int;
            s0 += *new.offset(j as isize) * *buf.offset(idx as isize);
            s1 += *buf.offset(idx as isize) * *buf.offset(idx as isize);
            j += 1;
            j;
        }
        corr = (if s1 > 0.0f32 {
            s0 as libc::c_double / sqrt(s1 as libc::c_double)
        } else {
            0.0f32 as libc::c_double
        }) as libc::c_float;
        if corr > max_corr as libc::c_float {
            max_corr = corr as libc::c_int;
            lag = i;
            max_ratio = corr / (2048 as libc::c_int - start) as libc::c_float;
        }
        i += 1;
        i;
    }
    (*ltp).lag = (if av_clip_uintp2_c(lag, 11 as libc::c_int) > 0 as libc::c_int as libc::c_uint {
        av_clip_uintp2_c(lag, 11 as libc::c_int)
    } else {
        0 as libc::c_int as libc::c_uint
    }) as int16_t;
    (*ltp).coef_idx = quant_array_idx(max_ratio, ltp_coef.as_ptr(), 8 as libc::c_int);
    (*ltp).coef = ltp_coef[(*ltp).coef_idx as usize];
}
unsafe extern "C" fn generate_samples(
    mut buf: *mut libc::c_float,
    mut ltp: *mut LongTermPrediction,
) {
    let mut i: libc::c_int = 0;
    let mut samples_num: libc::c_int = 2048 as libc::c_int;
    if (*ltp).lag == 0 {
        (*ltp).present = 0 as libc::c_int as int8_t;
        return;
    } else if ((*ltp).lag as libc::c_int) < 1024 as libc::c_int {
        samples_num = (*ltp).lag as libc::c_int + 1024 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < samples_num {
        *buf.offset(i as isize) = (*ltp).coef
            * *buf.offset((i + 2048 as libc::c_int - (*ltp).lag as libc::c_int) as isize);
        i += 1;
        i;
    }
    ptr::write_bytes(
        &mut *buf.offset(i as isize) as *mut libc::c_float,
        0,
        (2048 as libc::c_int - i) as usize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ff_aac_update_ltp(
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
) {
    let mut pred_signal: *mut libc::c_float = &mut *((*sce).ltp_state)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize)
        as *mut INTFLOAT;
    let mut samples: *const libc::c_float = &mut *(*((*s).planar_samples)
        .as_mut_ptr()
        .offset((*s).cur_channel as isize))
    .offset(1024 as libc::c_int as isize)
        as *mut libc::c_float;
    if (*s).profile != 3 as libc::c_int {
        return;
    }
    get_lag(pred_signal, samples, &mut (*sce).ics.ltp);
    generate_samples(pred_signal, &mut (*sce).ics.ltp);
}
#[no_mangle]
pub unsafe extern "C" fn ff_aac_adjust_common_ltp(
    mut s: *mut AACEncContext,
    mut cpe: *mut ChannelElement,
) {
    let mut sfb: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut sce0: *mut SingleChannelElement =
        &mut *((*cpe).ch).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut SingleChannelElement;
    let mut sce1: *mut SingleChannelElement =
        &mut *((*cpe).ch).as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut SingleChannelElement;
    if (*cpe).common_window == 0
        || (*sce0).ics.window_sequence[0 as libc::c_int as usize] as libc::c_uint
            == EIGHT_SHORT_SEQUENCE as libc::c_int as libc::c_uint
        || (*sce1).ics.window_sequence[0 as libc::c_int as usize] as libc::c_uint
            == EIGHT_SHORT_SEQUENCE as libc::c_int as libc::c_uint
    {
        (*sce0).ics.ltp.present = 0 as libc::c_int as int8_t;
        return;
    }
    sfb = 0 as libc::c_int;
    while sfb
        < (if (*sce0).ics.max_sfb as libc::c_int > 40 as libc::c_int {
            40 as libc::c_int
        } else {
            (*sce0).ics.max_sfb as libc::c_int
        })
    {
        let mut sum: libc::c_int = (*sce0).ics.ltp.used[sfb as usize] as libc::c_int
            + (*sce1).ics.ltp.used[sfb as usize] as libc::c_int;
        if sum != 2 as libc::c_int {
            (*sce0).ics.ltp.used[sfb as usize] = 0 as libc::c_int as int8_t;
        } else {
            count += 1;
            count;
        }
        sfb += 1;
        sfb;
    }
    (*sce0).ics.ltp.present = (count != 0) as libc::c_int as int8_t;
    (*sce0).ics.predictor_present = (count != 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_aac_search_for_ltp(
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
    mut common_window: libc::c_int,
) {
    let mut w: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut w2: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut start: libc::c_int = 0 as libc::c_int;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut saved_bits: libc::c_int = -(15 as libc::c_int
        + (if (*sce).ics.max_sfb as libc::c_int > 40 as libc::c_int {
            40 as libc::c_int
        } else {
            (*sce).ics.max_sfb as libc::c_int
        }));
    let mut C34: *mut libc::c_float = &mut *((*s).scoefs)
        .as_mut_ptr()
        .offset((128 as libc::c_int * 0 as libc::c_int) as isize)
        as *mut libc::c_float;
    let mut PCD: *mut libc::c_float = &mut *((*s).scoefs)
        .as_mut_ptr()
        .offset((128 as libc::c_int * 1 as libc::c_int) as isize)
        as *mut libc::c_float;
    let mut PCD34: *mut libc::c_float = &mut *((*s).scoefs)
        .as_mut_ptr()
        .offset((128 as libc::c_int * 2 as libc::c_int) as isize)
        as *mut libc::c_float;
    let max_ltp: libc::c_int = if (*sce).ics.max_sfb as libc::c_int > 40 as libc::c_int {
        40 as libc::c_int
    } else {
        (*sce).ics.max_sfb as libc::c_int
    };
    if (*sce).ics.window_sequence[0 as libc::c_int as usize] as libc::c_uint
        == EIGHT_SHORT_SEQUENCE as libc::c_int as libc::c_uint
    {
        if (*sce).ics.ltp.lag != 0 {
            (*sce).ltp_state.fill(0.);
            (*sce).ics.ltp = LongTermPrediction::default();
        }
        return;
    }
    if (*sce).ics.ltp.lag == 0 || (*s).lambda > 120.0f32 {
        return;
    }
    w = 0 as libc::c_int;
    while w < (*sce).ics.num_windows {
        start = 0 as libc::c_int;
        g = 0 as libc::c_int;
        while g < (*sce).ics.num_swb {
            let mut bits1: libc::c_int = 0 as libc::c_int;
            let mut bits2: libc::c_int = 0 as libc::c_int;
            let mut dist1: libc::c_float = 0.0f32;
            let mut dist2: libc::c_float = 0.0f32;
            if w * 16 as libc::c_int + g > max_ltp {
                start += *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int;
            } else {
                w2 = 0 as libc::c_int;
                while w2 < (*sce).ics.group_len[w as usize] as libc::c_int {
                    let mut bits_tmp1: libc::c_int = 0;
                    let mut bits_tmp2: libc::c_int = 0;
                    let mut band: *mut FFPsyBand =
                        &mut *((*((*s).psy.ch).offset((*s).cur_channel as isize)).psy_bands)
                            .as_mut_ptr()
                            .offset(((w + w2) * 16 as libc::c_int + g) as isize)
                            as *mut FFPsyBand;
                    i = 0 as libc::c_int;
                    while i < *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int {
                        *PCD.offset(i as isize) = (*sce).coeffs
                            [(start + (w + w2) * 128 as libc::c_int + i) as usize]
                            - (*sce).lcoeffs[(start + (w + w2) * 128 as libc::c_int + i) as usize];
                        i += 1;
                        i;
                    }
                    ((*s).abs_pow34).expect("non-null function pointer")(
                        C34,
                        &mut *((*sce).coeffs)
                            .as_mut_ptr()
                            .offset((start + (w + w2) * 128 as libc::c_int) as isize),
                        *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int,
                    );
                    ((*s).abs_pow34).expect("non-null function pointer")(
                        PCD34,
                        PCD,
                        *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int,
                    );
                    dist1 += quantize_band_cost(
                        s,
                        &mut *((*sce).coeffs)
                            .as_mut_ptr()
                            .offset((start + (w + w2) * 128 as libc::c_int) as isize),
                        C34,
                        *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int,
                        (*sce).sf_idx[((w + w2) * 16 as libc::c_int + g) as usize],
                        (*sce).band_type[((w + w2) * 16 as libc::c_int + g) as usize]
                            as libc::c_int,
                        (*s).lambda / (*band).threshold,
                        ::core::f32::INFINITY,
                        &mut bits_tmp1,
                        0 as *mut libc::c_float,
                    );
                    dist2 += quantize_band_cost(
                        s,
                        PCD,
                        PCD34,
                        *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int,
                        (*sce).sf_idx[((w + w2) * 16 as libc::c_int + g) as usize],
                        (*sce).band_type[((w + w2) * 16 as libc::c_int + g) as usize]
                            as libc::c_int,
                        (*s).lambda / (*band).threshold,
                        ::core::f32::INFINITY,
                        &mut bits_tmp2,
                        0 as *mut libc::c_float,
                    );
                    bits1 += bits_tmp1;
                    bits2 += bits_tmp2;
                    w2 += 1;
                    w2;
                }
                if dist2 < dist1 && bits2 < bits1 {
                    w2 = 0 as libc::c_int;
                    while w2 < (*sce).ics.group_len[w as usize] as libc::c_int {
                        i = 0 as libc::c_int;
                        while i < *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int {
                            (*sce).coeffs[(start + (w + w2) * 128 as libc::c_int + i) as usize] -=
                                (*sce).lcoeffs
                                    [(start + (w + w2) * 128 as libc::c_int + i) as usize];
                            i += 1;
                            i;
                        }
                        w2 += 1;
                        w2;
                    }
                    (*sce).ics.ltp.used[(w * 16 as libc::c_int + g) as usize] =
                        1 as libc::c_int as int8_t;
                    saved_bits += bits1 - bits2;
                    count += 1;
                    count;
                }
                start += *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int;
            }
            g += 1;
            g;
        }
        w += (*sce).ics.group_len[w as usize] as libc::c_int;
    }
    (*sce).ics.ltp.present =
        (count != 0 && saved_bits >= 0 as libc::c_int) as libc::c_int as int8_t;
    (*sce).ics.predictor_present = ((*sce).ics.ltp.present != 0) as libc::c_int;
    if (*sce).ics.ltp.present == 0 && count != 0 {
        w = 0 as libc::c_int;
        while w < (*sce).ics.num_windows {
            start = 0 as libc::c_int;
            g = 0 as libc::c_int;
            while g < (*sce).ics.num_swb {
                if (*sce).ics.ltp.used[(w * 16 as libc::c_int + g) as usize] != 0 {
                    w2 = 0 as libc::c_int;
                    while w2 < (*sce).ics.group_len[w as usize] as libc::c_int {
                        i = 0 as libc::c_int;
                        while i < *((*sce).ics.swb_sizes).offset(g as isize) as libc::c_int {
                            (*sce).coeffs[(start + (w + w2) * 128 as libc::c_int + i) as usize] +=
                                (*sce).lcoeffs
                                    [(start + (w + w2) * 128 as libc::c_int + i) as usize];
                            i += 1;
                            i;
                        }
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
    }
}
unsafe extern "C" fn run_static_initializers() {
    BUF_BITS = (8 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<BitBuf>() as libc::c_ulong)
        as libc::c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
