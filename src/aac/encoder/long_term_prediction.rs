#![allow(
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use std::{mem::size_of, ptr};

use array_util::{Array, W};
use ffmpeg_src_macro::ffmpeg_src;
use libc::{c_double, c_float, c_int, c_long, c_schar, c_short, c_uint, c_ulong};

use crate::{
    aac::{
        coder::quantize_band_cost,
        encoder::{abs_pow34_v, ctx::AACEncContext},
        IndividualChannelStream, SyntaxElementType, WindowedIteration, EIGHT_SHORT_SEQUENCE,
    },
    common::*,
    types::*,
};

#[ffmpeg_src(file = "libavcodec/aac.h", lines = 50, name = "MAX_LTP_LONG_SFB")]
const MAX_LONG_SFB: usize = 40;

#[ffmpeg_src(file = "libavcodec/aac.h", lines = 158..=167)]
#[derive(Default, Copy, Clone)]
pub(crate) struct LongTermPrediction {
    pub(super) present: c_schar,
    pub(super) lag: c_short,
    pub(super) coef_idx: c_int,
    pub(super) coef: c_float,
    pub(super) used: Array<c_schar, MAX_LONG_SFB>,
}

#[inline(always)]
unsafe fn av_clip_uintp2_c(mut a: c_int, mut p: c_int) -> c_uint {
    if a & !(((1) << p) - 1) != 0 {
        (!a >> 31 & ((1) << p) - 1) as c_uint
    } else {
        a as c_uint
    }
}

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
unsafe fn quant_array_idx(val: c_float, mut arr: *const c_float, num: c_int) -> c_int {
    let mut i: c_int = 0;
    let mut index: c_int = 0;
    let mut quant_min_err: c_float = f32::INFINITY;
    i = 0;
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
static mut ltp_coef: [c_float; 8] = [
    0.570829, 0.696616, 0.813004, 0.911304, 0.984900, 1.067894, 1.194601, 1.369533,
];

pub(crate) unsafe fn encode_ltp_info(
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
    mut common_window: c_int,
) {
    let mut i: c_int = 0;
    let mut ics: *mut IndividualChannelStream = &mut (*sce).ics;
    if (*s).profile != 3 || (*ics).predictor_present == 0 {
        return;
    }
    if common_window != 0 {
        put_bits(&mut (*s).pb, 1, 0 as BitBuf);
    }
    put_bits(&mut (*s).pb, 1, (*ics).ltp.present as BitBuf);
    if (*ics).ltp.present == 0 {
        return;
    }
    put_bits(&mut (*s).pb, 11, (*ics).ltp.lag as BitBuf);
    put_bits(&mut (*s).pb, 3, (*ics).ltp.coef_idx as BitBuf);
    i = 0;
    while i
        < (if (*ics).max_sfb as c_int > 40 {
            40
        } else {
            (*ics).max_sfb as c_int
        })
    {
        put_bits(&mut (*s).pb, 1, (*ics).ltp.used[i as usize] as BitBuf);
        i += 1;
        i;
    }
}

pub(crate) unsafe fn ltp_insert_new_frame(mut s: *mut AACEncContext) {
    let mut i: c_int = 0;
    let mut ch: c_int = 0;
    let mut tag: c_int = 0;
    let mut chans: c_int = 0;
    let mut cur_channel: c_int = 0;
    let mut start_ch: c_int = 0;
    let mut cpe: *mut ChannelElement = std::ptr::null_mut::<ChannelElement>();
    let mut sce: *mut SingleChannelElement = std::ptr::null_mut::<SingleChannelElement>();
    i = 0;
    while i < (*s).chan_map[0] as c_int {
        cpe = &mut *((*s).cpe.as_mut_ptr()).offset(i as isize) as *mut ChannelElement;
        tag = (*s).chan_map[(i + 1) as usize] as c_int;
        chans = if tag == SyntaxElementType::ChannelPairElement as c_int {
            2
        } else {
            1
        };
        ch = 0;
        while ch < chans {
            sce = &mut *((*cpe).ch).as_mut_ptr().offset(ch as isize) as *mut SingleChannelElement;
            cur_channel = start_ch + ch;
            (*sce).ltp_state[0] = (*sce).ltp_state[1];
            (*s).planar_samples[cur_channel as usize][2048..][..1024]
                .copy_from_slice(&*(*sce).ltp_state[1]);
            (*sce).ltp_state[2].copy_from_slice(&(*sce).ret_buf[..1024]);
            (*sce).ics.ltp.lag = 0 as c_short;
            ch += 1;
            ch;
        }
        start_ch += chans;
        i += 1;
        i;
    }
}
unsafe fn get_lag(
    mut buf: *mut c_float,
    mut new: *const c_float,
    mut ltp: *mut LongTermPrediction,
) {
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut lag: c_int = 0;
    let mut max_corr: c_int = 0;
    let mut max_ratio: c_float = 0.;
    i = 0;
    while i < 2048 {
        let mut corr: c_float = 0.;
        let mut s0: c_float = 0.;
        let mut s1: c_float = 0.;
        let start: c_int = if 0 > i - 1024 { 0 } else { i - 1024 };
        j = start;
        while j < 2048 {
            let idx: c_int = j - i + 1024;
            s0 += *new.offset(j as isize) * *buf.offset(idx as isize);
            s1 += *buf.offset(idx as isize) * *buf.offset(idx as isize);
            j += 1;
            j;
        }
        corr = (if s1 > 0. {
            s0 as c_double / sqrt(s1 as c_double)
        } else {
            0.
        }) as c_float;
        if corr > max_corr as c_float {
            max_corr = corr as c_int;
            lag = i;
            max_ratio = corr / (2048 - start) as c_float;
        }
        i += 1;
        i;
    }
    (*ltp).lag = (if av_clip_uintp2_c(lag, 11) > 0 as c_uint {
        av_clip_uintp2_c(lag, 11)
    } else {
        0 as c_uint
    }) as c_short;
    (*ltp).coef_idx = quant_array_idx(max_ratio, ltp_coef.as_ptr(), 8);
    (*ltp).coef = ltp_coef[(*ltp).coef_idx as usize];
}
unsafe fn generate_samples(mut buf: *mut c_float, mut ltp: *mut LongTermPrediction) {
    let mut i: c_int = 0;
    let mut samples_num: c_int = 2048;
    if (*ltp).lag == 0 {
        (*ltp).present = 0;
        return;
    } else if ((*ltp).lag as c_int) < 1024 {
        samples_num = (*ltp).lag as c_int + 1024;
    }
    i = 0;
    while i < samples_num {
        *buf.offset(i as isize) =
            (*ltp).coef * *buf.offset((i + 2048 - (*ltp).lag as c_int) as isize);
        i += 1;
        i;
    }
    ptr::write_bytes(
        &mut *buf.offset(i as isize) as *mut c_float,
        0,
        (2048 - i) as usize,
    );
}

pub(crate) unsafe fn update_ltp(mut s: *mut AACEncContext, mut sce: *mut SingleChannelElement) {
    let mut pred_signal = ((*sce).ltp_state).as_mut_ptr().cast::<c_float>();
    let mut samples: *const c_float =
        ((*s).planar_samples)[(*s).cur_channel as usize][1024..].as_mut_ptr();
    if (*s).profile != 3 {
        return;
    }
    get_lag(pred_signal, samples, &mut (*sce).ics.ltp);
    generate_samples(pred_signal, &mut (*sce).ics.ltp);
}

pub(crate) unsafe fn adjust_common_ltp(mut _s: *mut AACEncContext, mut cpe: *mut ChannelElement) {
    let mut sfb: c_int = 0;
    let mut count: c_int = 0;
    let mut sce0: *mut SingleChannelElement =
        &mut *((*cpe).ch).as_mut_ptr().offset(0) as *mut SingleChannelElement;
    let mut sce1: *mut SingleChannelElement =
        &mut *((*cpe).ch).as_mut_ptr().offset(1) as *mut SingleChannelElement;
    if (*cpe).common_window == 0
        || (*sce0).ics.window_sequence[0] as c_uint == EIGHT_SHORT_SEQUENCE as c_int as c_uint
        || (*sce1).ics.window_sequence[0] as c_uint == EIGHT_SHORT_SEQUENCE as c_int as c_uint
    {
        (*sce0).ics.ltp.present = 0;
        return;
    }
    sfb = 0;
    while sfb
        < (if (*sce0).ics.max_sfb as c_int > 40 {
            40
        } else {
            (*sce0).ics.max_sfb as c_int
        })
    {
        let mut sum: c_int = (*sce0).ics.ltp.used[sfb as usize] as c_int
            + (*sce1).ics.ltp.used[sfb as usize] as c_int;
        if sum != 2 {
            (*sce0).ics.ltp.used[sfb as usize] = 0;
        } else {
            count += 1;
            count;
        }
        sfb += 1;
        sfb;
    }
    (*sce0).ics.ltp.present = (count != 0) as c_int as c_schar;
    (*sce0).ics.predictor_present = (count != 0) as c_int;
}

pub(crate) unsafe fn search_for_ltp(
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
    mut _common_window: c_int,
) {
    let mut g: c_int = 0;
    let mut w2: c_int = 0;
    let mut i: c_int = 0;
    let mut start: c_int = 0;
    let mut count: c_int = 0;
    let mut saved_bits: c_int = -(15
        + (if (*sce).ics.max_sfb as c_int > 40 {
            40
        } else {
            (*sce).ics.max_sfb as c_int
        }));

    let ([C34, PCD, PCD34, ..], []) = (*s).scaled_coeffs.as_chunks_mut::<128>() else {
        unreachable!();
    };

    let max_ltp: c_int = if (*sce).ics.max_sfb as c_int > 40 {
        40
    } else {
        (*sce).ics.max_sfb as c_int
    };
    if (*sce).ics.window_sequence[0] as c_uint == EIGHT_SHORT_SEQUENCE as c_int as c_uint {
        if (*sce).ics.ltp.lag != 0 {
            (*sce).ltp_state = Default::default();
            (*sce).ics.ltp = LongTermPrediction::default();
        }
        return;
    }
    if (*sce).ics.ltp.lag == 0 || (*s).lambda > 120. {
        return;
    }
    for WindowedIteration { w, group_len } in (*sce).ics.iter_windows() {
        start = 0;
        g = 0;
        while g < (*sce).ics.num_swb {
            let mut bits1: c_int = 0;
            let mut bits2: c_int = 0;
            let mut dist1: c_float = 0.;
            let mut dist2: c_float = 0.;
            if w * 16 + g > max_ltp {
                start += (*sce).ics.swb_sizes[g as usize] as c_int;
            } else {
                w2 = 0;
                while w2 < group_len as c_int {
                    let mut bits_tmp1: c_int = 0;
                    let mut bits_tmp2: c_int = 0;
                    let mut band =
                        &(*s).psy.ch[(*s).cur_channel as usize].psy_bands[W(w + w2)][g as usize];
                    i = 0;
                    while i < (*sce).ics.swb_sizes[g as usize] as c_int {
                        PCD[i as usize] = (*sce).coeffs[W(w + w2)][(start + i) as usize]
                            - (*sce).lcoeffs[(start + (w + w2) * 128 + i) as usize];
                        i += 1;
                        i;
                    }
                    abs_pow34_v(
                        C34.as_mut_ptr(),
                        &mut *((*sce).coeffs)
                            .as_mut_ptr()
                            .offset((start + (w + w2) * 128) as isize),
                        (*sce).ics.swb_sizes[g as usize] as c_int,
                    );
                    abs_pow34_v(
                        PCD34.as_mut_ptr(),
                        PCD.as_ptr(),
                        (*sce).ics.swb_sizes[g as usize] as c_int,
                    );
                    dist1 += quantize_band_cost(
                        &(*sce).coeffs[W(w + w2)][start as usize..]
                            [..(*sce).ics.swb_sizes[g as usize].into()],
                        &C34[..(*sce).ics.swb_sizes[g as usize].into()],
                        (*sce).sf_idx[W(w + w2)][g as usize],
                        (*sce).band_type[W(w + w2)][g as usize] as c_int,
                        (*s).lambda / band.threshold,
                        f32::INFINITY,
                        Some(&mut bits_tmp1),
                        None,
                    );
                    dist2 += quantize_band_cost(
                        &PCD[..(*sce).ics.swb_sizes[g as usize].into()],
                        &PCD34[..(*sce).ics.swb_sizes[g as usize].into()],
                        (*sce).sf_idx[W(w + w2)][g as usize],
                        (*sce).band_type[W(w + w2)][g as usize] as c_int,
                        (*s).lambda / band.threshold,
                        f32::INFINITY,
                        Some(&mut bits_tmp2),
                        None,
                    );
                    bits1 += bits_tmp1;
                    bits2 += bits_tmp2;
                    w2 += 1;
                    w2;
                }
                if dist2 < dist1 && bits2 < bits1 {
                    w2 = 0;
                    while w2 < group_len as c_int {
                        i = 0;
                        while i < (*sce).ics.swb_sizes[g as usize] as c_int {
                            (*sce).coeffs[W(w + w2)][(start + i) as usize] -=
                                (*sce).lcoeffs[(start + (w + w2) * 128 + i) as usize];
                            i += 1;
                            i;
                        }
                        w2 += 1;
                        w2;
                    }
                    (*sce).ics.ltp.used[(w * 16 + g) as usize] = 1;
                    saved_bits += bits1 - bits2;
                    count += 1;
                    count;
                }
                start += (*sce).ics.swb_sizes[g as usize] as c_int;
            }
            g += 1;
            g;
        }
    }
    (*sce).ics.ltp.present = (count != 0 && saved_bits >= 0) as c_int as c_schar;
    (*sce).ics.predictor_present = ((*sce).ics.ltp.present != 0) as c_int;
    if (*sce).ics.ltp.present == 0 && count != 0 {
        for WindowedIteration { w, group_len } in (*sce).ics.iter_windows() {
            start = 0;
            g = 0;
            while g < (*sce).ics.num_swb {
                if (*sce).ics.ltp.used[(w * 16 + g) as usize] != 0 {
                    w2 = 0;
                    while w2 < group_len as c_int {
                        i = 0;
                        while i < (*sce).ics.swb_sizes[g as usize] as c_int {
                            (*sce).coeffs[W(w + w2)][(start + i) as usize] +=
                                (*sce).lcoeffs[(start + (w + w2) * 128 + i) as usize];
                            i += 1;
                            i;
                        }
                        w2 += 1;
                        w2;
                    }
                }
                start += (*sce).ics.swb_sizes[g as usize] as c_int;
                g += 1;
                g;
            }
        }
    }
}
unsafe fn run_static_initializers() {
    BUF_BITS = (8 as c_ulong).wrapping_mul(size_of::<BitBuf>() as c_ulong) as c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe fn(); 1] = [run_static_initializers];
