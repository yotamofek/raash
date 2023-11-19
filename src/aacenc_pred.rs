#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use crate::{
    aaccoder::ff_quantize_and_encode_band_cost,
    aacenc_is::ff_aac_is_encoding_err,
    aactab::{ff_aac_pow34sf_tab, ff_aac_pred_sfb_max},
    types::*,
};

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
unsafe fn av_bswap32(mut x: uint32_t) -> uint32_t {
    (x << 8 as libc::c_int & 0xff00 as libc::c_int as libc::c_uint
        | x >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        << 16 as libc::c_int
        | ((x >> 16 as libc::c_int) << 8 as libc::c_int & 0xff00 as libc::c_int as libc::c_uint
            | x >> 16 as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
}
static mut BUF_BITS: libc::c_int = 0;
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
#[inline]
unsafe fn put_bits(mut s: *mut PutBitContext, mut n: libc::c_int, mut value: BitBuf) {
    put_bits_no_assert(s, n, value);
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

union av_intfloat32 {
    i: libc::c_uint,
    f: libc::c_float,
}
#[inline]
unsafe fn flt16_round(mut pf: libc::c_float) -> libc::c_float {
    let mut tmp: av_intfloat32 = av_intfloat32 { i: 0 };
    tmp.f = pf;
    tmp.i = (tmp.i).wrapping_add(0x8000 as libc::c_uint) & 0xffff0000 as libc::c_uint;
    tmp.f
}
#[inline]
unsafe fn flt16_even(mut pf: libc::c_float) -> libc::c_float {
    let mut tmp: av_intfloat32 = av_intfloat32 { i: 0 };
    tmp.f = pf;
    tmp.i = (tmp.i)
        .wrapping_add(0x7fff as libc::c_uint)
        .wrapping_add(tmp.i & 0x10000 as libc::c_uint >> 16 as libc::c_int)
        & 0xffff0000 as libc::c_uint;
    tmp.f
}
#[inline]
unsafe fn flt16_trunc(mut pf: libc::c_float) -> libc::c_float {
    let mut pun: av_intfloat32 = av_intfloat32 { i: 0 };
    pun.f = pf;
    pun.i &= 0xffff0000 as libc::c_uint;
    pun.f
}
#[inline]
unsafe fn predict(
    mut ps: *mut PredictorState,
    mut coef: *mut libc::c_float,
    mut rcoef: *mut libc::c_float,
    mut set: libc::c_int,
) {
    let mut k2: libc::c_float = 0.;
    let a: libc::c_float = 0.953125f64 as libc::c_float;
    let alpha: libc::c_float = 0.90625f64 as libc::c_float;
    let k1: libc::c_float = (*ps).k1;
    let r0: libc::c_float = (*ps).r0;
    let r1: libc::c_float = (*ps).r1;
    let cor0: libc::c_float = (*ps).cor0;
    let cor1: libc::c_float = (*ps).cor1;
    let var0: libc::c_float = (*ps).var0;
    let var1: libc::c_float = (*ps).var1;
    let e0: libc::c_float = *coef - (*ps).x_est;
    let e1: libc::c_float = e0 - k1 * r0;
    if set != 0 {
        *coef = e0;
    }
    (*ps).cor1 = flt16_trunc(alpha * cor1 + r1 * e1);
    (*ps).var1 = flt16_trunc(alpha * var1 + 0.5f32 * (r1 * r1 + e1 * e1));
    (*ps).cor0 = flt16_trunc(alpha * cor0 + r0 * e0);
    (*ps).var0 = flt16_trunc(alpha * var0 + 0.5f32 * (r0 * r0 + e0 * e0));
    (*ps).r1 = flt16_trunc(a * (r0 - k1 * e0));
    (*ps).r0 = flt16_trunc(a * e0);
    (*ps).k1 = if (*ps).var0 > 1 as libc::c_int as libc::c_float {
        (*ps).cor0 * flt16_even(a / (*ps).var0)
    } else {
        0 as libc::c_int as libc::c_float
    };
    k2 = if (*ps).var1 > 1 as libc::c_int as libc::c_float {
        (*ps).cor1 * flt16_even(a / (*ps).var1)
    } else {
        0 as libc::c_int as libc::c_float
    };
    (*ps).x_est = flt16_round((*ps).k1 * (*ps).r0 + k2 * (*ps).r1);
    *rcoef = (*ps).x_est;
}
#[inline]
unsafe fn reset_predict_state(mut ps: *mut PredictorState) {
    (*ps).r0 = 0.0f32;
    (*ps).r1 = 0.0f32;
    (*ps).k1 = 0.0f32;
    (*ps).cor0 = 0.0f32;
    (*ps).cor1 = 0.0f32;
    (*ps).var0 = 1.0f32;
    (*ps).var1 = 1.0f32;
    (*ps).x_est = 0.0f32;
}
#[inline]
unsafe fn reset_all_predictors(mut ps: *mut PredictorState) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 672 as libc::c_int {
        reset_predict_state(&mut *ps.offset(i as isize));
        i += 1;
        i;
    }
}
#[inline]
unsafe fn reset_predictor_group(mut sce: *mut SingleChannelElement, mut group_num: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut ps: *mut PredictorState = ((*sce).predictor_state).as_mut_ptr();
    i = group_num - 1 as libc::c_int;
    while i < 672 as libc::c_int {
        reset_predict_state(&mut *ps.offset(i as isize));
        i += 30 as libc::c_int;
    }
}

pub(crate) unsafe extern "C" fn ff_aac_apply_main_pred(
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
) {
    let mut sfb: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let pmax: libc::c_int = if (*sce).ics.max_sfb as libc::c_int
        > *ff_aac_pred_sfb_max
            .as_ptr()
            .offset((*s).samplerate_index as isize) as libc::c_int
    {
        *ff_aac_pred_sfb_max
            .as_ptr()
            .offset((*s).samplerate_index as isize) as libc::c_int
    } else {
        (*sce).ics.max_sfb as libc::c_int
    };
    if (*sce).ics.window_sequence[0 as libc::c_int as usize] as libc::c_uint
        != EIGHT_SHORT_SEQUENCE as libc::c_int as libc::c_uint
    {
        sfb = 0 as libc::c_int;
        while sfb < pmax {
            k = *((*sce).ics.swb_offset).offset(sfb as isize) as libc::c_int;
            while k < *((*sce).ics.swb_offset).offset((sfb + 1 as libc::c_int) as isize)
                as libc::c_int
            {
                predict(
                    &mut *((*sce).predictor_state).as_mut_ptr().offset(k as isize),
                    &mut *((*sce).coeffs).as_mut_ptr().offset(k as isize),
                    &mut *((*sce).prcoeffs).as_mut_ptr().offset(k as isize),
                    ((*sce).ics.predictor_present != 0
                        && (*sce).ics.prediction_used[sfb as usize] as libc::c_int != 0)
                        as libc::c_int,
                );
                k += 1;
                k;
            }
            sfb += 1;
            sfb;
        }
        if (*sce).ics.predictor_reset_group != 0 {
            reset_predictor_group(sce, (*sce).ics.predictor_reset_group);
        }
    } else {
        reset_all_predictors(((*sce).predictor_state).as_mut_ptr());
    };
}
#[inline]
unsafe fn update_counters(
    mut ics: *mut IndividualChannelStream,
    mut inc: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < 31 as libc::c_int {
        (*ics).predictor_reset_count[i as usize] += inc;
        if (*ics).predictor_reset_count[i as usize] > 240 as libc::c_int {
            return i;
        }
        i += 1;
        i;
    }
    0 as libc::c_int
}

pub(crate) unsafe extern "C" fn ff_aac_adjust_common_pred(
    mut s: *mut AACEncContext,
    mut cpe: *mut ChannelElement,
) {
    let mut start: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut w2: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut sce0: *mut SingleChannelElement =
        &mut *((*cpe).ch).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut SingleChannelElement;
    let mut sce1: *mut SingleChannelElement =
        &mut *((*cpe).ch).as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut SingleChannelElement;
    let pmax0: libc::c_int = if (*sce0).ics.max_sfb as libc::c_int
        > *ff_aac_pred_sfb_max
            .as_ptr()
            .offset((*s).samplerate_index as isize) as libc::c_int
    {
        *ff_aac_pred_sfb_max
            .as_ptr()
            .offset((*s).samplerate_index as isize) as libc::c_int
    } else {
        (*sce0).ics.max_sfb as libc::c_int
    };
    let pmax1: libc::c_int = if (*sce1).ics.max_sfb as libc::c_int
        > *ff_aac_pred_sfb_max
            .as_ptr()
            .offset((*s).samplerate_index as isize) as libc::c_int
    {
        *ff_aac_pred_sfb_max
            .as_ptr()
            .offset((*s).samplerate_index as isize) as libc::c_int
    } else {
        (*sce1).ics.max_sfb as libc::c_int
    };
    let pmax: libc::c_int = if pmax0 > pmax1 { pmax1 } else { pmax0 };
    if (*cpe).common_window == 0
        || (*sce0).ics.window_sequence[0 as libc::c_int as usize] as libc::c_uint
            == EIGHT_SHORT_SEQUENCE as libc::c_int as libc::c_uint
        || (*sce1).ics.window_sequence[0 as libc::c_int as usize] as libc::c_uint
            == EIGHT_SHORT_SEQUENCE as libc::c_int as libc::c_uint
    {
        return;
    }
    w = 0 as libc::c_int;
    while w < (*sce0).ics.num_windows {
        start = 0 as libc::c_int;
        g = 0 as libc::c_int;
        while g < (*sce0).ics.num_swb {
            let mut sfb: libc::c_int = w * 16 as libc::c_int + g;
            let mut sum: libc::c_int = (*sce0).ics.prediction_used[sfb as usize] as libc::c_int
                + (*sce1).ics.prediction_used[sfb as usize] as libc::c_int;
            let mut ener0: libc::c_float = 0.0f32;
            let mut ener1: libc::c_float = 0.0f32;
            let mut ener01: libc::c_float = 0.0f32;
            let mut ph_err1: AACISError = AACISError {
                pass: 0,
                phase: 0,
                error: 0.,
                dist1: 0.,
                dist2: 0.,
                ener01: 0.,
            };
            let mut ph_err2: AACISError = AACISError {
                pass: 0,
                phase: 0,
                error: 0.,
                dist1: 0.,
                dist2: 0.,
                ener01: 0.,
            };
            let mut erf: *mut AACISError = std::ptr::null_mut::<AACISError>();
            if sfb < 10 as libc::c_int || sfb > pmax || sum != 2 as libc::c_int {
                if (*sce0).ics.prediction_used[sfb as usize] != 0 {
                    (*sce0).ics.prediction_used[sfb as usize] = 0 as libc::c_int as uint8_t;
                    (*sce0).band_type[sfb as usize] = (*sce0).band_alt[sfb as usize];
                }
                if (*sce1).ics.prediction_used[sfb as usize] != 0 {
                    (*sce1).ics.prediction_used[sfb as usize] = 0 as libc::c_int as uint8_t;
                    (*sce1).band_type[sfb as usize] = (*sce1).band_alt[sfb as usize];
                }
                start += *((*sce0).ics.swb_sizes).offset(g as isize) as libc::c_int;
            } else {
                w2 = 0 as libc::c_int;
                while w2 < (*sce0).ics.group_len[w as usize] as libc::c_int {
                    i = 0 as libc::c_int;
                    while i < *((*sce0).ics.swb_sizes).offset(g as isize) as libc::c_int {
                        let mut coef0: libc::c_float =
                            (*sce0).pcoeffs[(start + (w + w2) * 128 as libc::c_int + i) as usize];
                        let mut coef1: libc::c_float =
                            (*sce1).pcoeffs[(start + (w + w2) * 128 as libc::c_int + i) as usize];
                        ener0 += coef0 * coef0;
                        ener1 += coef1 * coef1;
                        ener01 += (coef0 + coef1) * (coef0 + coef1);
                        i += 1;
                        i;
                    }
                    w2 += 1;
                    w2;
                }
                ph_err1 = ff_aac_is_encoding_err(
                    s,
                    cpe,
                    start,
                    w,
                    g,
                    ener0,
                    ener1,
                    ener01,
                    1 as libc::c_int,
                    -(1 as libc::c_int),
                );
                ph_err2 = ff_aac_is_encoding_err(
                    s,
                    cpe,
                    start,
                    w,
                    g,
                    ener0,
                    ener1,
                    ener01,
                    1 as libc::c_int,
                    1 as libc::c_int,
                );
                erf = if ph_err1.error < ph_err2.error {
                    &mut ph_err1
                } else {
                    &mut ph_err2
                };
                if (*erf).pass != 0 {
                    (*sce0).ics.prediction_used[sfb as usize] = 1 as libc::c_int as uint8_t;
                    (*sce1).ics.prediction_used[sfb as usize] = 1 as libc::c_int as uint8_t;
                    count += 1;
                    count;
                } else {
                    if (*sce0).ics.prediction_used[sfb as usize] != 0 {
                        (*sce0).ics.prediction_used[sfb as usize] = 0 as libc::c_int as uint8_t;
                        (*sce0).band_type[sfb as usize] = (*sce0).band_alt[sfb as usize];
                    }
                    if (*sce1).ics.prediction_used[sfb as usize] != 0 {
                        (*sce1).ics.prediction_used[sfb as usize] = 0 as libc::c_int as uint8_t;
                        (*sce1).band_type[sfb as usize] = (*sce1).band_alt[sfb as usize];
                    }
                }
                start += *((*sce0).ics.swb_sizes).offset(g as isize) as libc::c_int;
            }
            g += 1;
            g;
        }
        w += (*sce0).ics.group_len[w as usize] as libc::c_int;
    }
    (*sce0).ics.predictor_present = (count != 0) as libc::c_int;
    (*sce1).ics.predictor_present = (*sce0).ics.predictor_present;
}
unsafe fn update_pred_resets(mut sce: *mut SingleChannelElement) {
    let mut i: libc::c_int = 0;
    let mut max_group_id_c: libc::c_int = 0;
    let mut max_frame: libc::c_int = 0 as libc::c_int;
    let mut avg_frame: libc::c_float = 0.0f32;
    let mut ics: *mut IndividualChannelStream = &mut (*sce).ics;
    (*ics).predictor_reset_group = update_counters(&mut (*sce).ics, 1 as libc::c_int);
    if (*ics).predictor_reset_group != 0 {
        return;
    }
    i = 1 as libc::c_int;
    while i < 31 as libc::c_int {
        if (*ics).predictor_reset_count[i as usize] > max_frame {
            max_group_id_c = i;
            max_frame = (*ics).predictor_reset_count[i as usize];
        }
        avg_frame = ((*ics).predictor_reset_count[i as usize] as libc::c_float + avg_frame)
            / 2 as libc::c_int as libc::c_float;
        i += 1;
        i;
    }
    if max_frame > 64 as libc::c_int {
        (*ics).predictor_reset_group = max_group_id_c;
    } else {
        (*ics).predictor_reset_group = 0 as libc::c_int;
    };
}

pub(crate) unsafe extern "C" fn ff_aac_search_for_pred(
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
) {
    let mut sfb: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut cost_coeffs: libc::c_int = 0 as libc::c_int;
    let mut cost_pred: libc::c_int = 0 as libc::c_int;
    let pmax: libc::c_int = if (*sce).ics.max_sfb as libc::c_int
        > *ff_aac_pred_sfb_max
            .as_ptr()
            .offset((*s).samplerate_index as isize) as libc::c_int
    {
        *ff_aac_pred_sfb_max
            .as_ptr()
            .offset((*s).samplerate_index as isize) as libc::c_int
    } else {
        (*sce).ics.max_sfb as libc::c_int
    };
    let mut O34: *mut libc::c_float = &mut *((*s).scoefs)
        .as_mut_ptr()
        .offset((128 as libc::c_int * 0 as libc::c_int) as isize)
        as *mut libc::c_float;
    let mut P34: *mut libc::c_float = &mut *((*s).scoefs)
        .as_mut_ptr()
        .offset((128 as libc::c_int * 1 as libc::c_int) as isize)
        as *mut libc::c_float;
    let mut SENT: *mut libc::c_float = &mut *((*s).scoefs)
        .as_mut_ptr()
        .offset((128 as libc::c_int * 2 as libc::c_int) as isize)
        as *mut libc::c_float;
    let mut S34: *mut libc::c_float = &mut *((*s).scoefs)
        .as_mut_ptr()
        .offset((128 as libc::c_int * 3 as libc::c_int) as isize)
        as *mut libc::c_float;
    let mut QERR: *mut libc::c_float = &mut *((*s).scoefs)
        .as_mut_ptr()
        .offset((128 as libc::c_int * 4 as libc::c_int) as isize)
        as *mut libc::c_float;
    if (*sce).ics.window_sequence[0 as libc::c_int as usize] as libc::c_uint
        == EIGHT_SHORT_SEQUENCE as libc::c_int as libc::c_uint
    {
        (*sce).ics.predictor_present = 0 as libc::c_int;
        return;
    }
    if (*sce).ics.predictor_initialized == 0 {
        reset_all_predictors(((*sce).predictor_state).as_mut_ptr());
        (*sce).ics.predictor_initialized = 1 as libc::c_int;
        (*sce).prcoeffs = (*sce).coeffs;
        i = 1 as libc::c_int;
        while i < 31 as libc::c_int {
            (*sce).ics.predictor_reset_count[i as usize] = i;
            i += 1;
            i;
        }
    }
    update_pred_resets(sce);
    (*sce).band_alt = (*sce).band_type;
    sfb = 10 as libc::c_int;
    while sfb < pmax {
        let mut cost1: libc::c_int = 0;
        let mut cost2: libc::c_int = 0;
        let mut cb_p: libc::c_int = 0;
        let mut dist1: libc::c_float = 0.;
        let mut dist2: libc::c_float = 0.;
        let mut dist_spec_err: libc::c_float = 0.0f32;
        let cb_n: libc::c_int = (if (*sce).zeroes[sfb as usize] as libc::c_int != 0 {
            0 as libc::c_int as libc::c_uint
        } else {
            (*sce).band_type[sfb as usize] as libc::c_uint
        }) as libc::c_int;
        let cb_min: libc::c_int = if (*sce).zeroes[sfb as usize] as libc::c_int != 0 {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        };
        let cb_max: libc::c_int = if (*sce).zeroes[sfb as usize] as libc::c_int != 0 {
            0 as libc::c_int
        } else {
            RESERVED_BT as libc::c_int
        };
        let start_coef: libc::c_int = *((*sce).ics.swb_offset).offset(sfb as isize) as libc::c_int;
        let num_coeffs: libc::c_int =
            *((*sce).ics.swb_offset).offset((sfb + 1 as libc::c_int) as isize) as libc::c_int
                - start_coef;
        let mut band: *const FFPsyBand = &mut *((*((*s).psy.ch).offset((*s).cur_channel as isize))
            .psy_bands)
            .as_mut_ptr()
            .offset(sfb as isize) as *mut FFPsyBand;
        if !(start_coef + num_coeffs > 672 as libc::c_int
            || (*s).cur_channel != 0
                && (*sce).band_type[sfb as usize] as libc::c_uint
                    >= INTENSITY_BT2 as libc::c_int as libc::c_uint
            || (*sce).band_type[sfb as usize] as libc::c_uint
                == NOISE_BT as libc::c_int as libc::c_uint)
        {
            ((*s).abs_pow34).expect("non-null function pointer")(
                O34,
                &mut *((*sce).coeffs).as_mut_ptr().offset(start_coef as isize),
                num_coeffs,
            );
            dist1 = ff_quantize_and_encode_band_cost(
                s,
                std::ptr::null_mut::<PutBitContext>(),
                &mut *((*sce).coeffs).as_mut_ptr().offset(start_coef as isize),
                std::ptr::null_mut::<libc::c_float>(),
                O34,
                num_coeffs,
                (*sce).sf_idx[sfb as usize],
                cb_n,
                (*s).lambda / (*band).threshold,
                ::core::f32::INFINITY,
                &mut cost1,
                std::ptr::null_mut::<libc::c_float>(),
            );
            cost_coeffs += cost1;
            i = 0 as libc::c_int;
            while i < num_coeffs {
                *SENT.offset(i as isize) = (*sce).coeffs[(start_coef + i) as usize]
                    - (*sce).prcoeffs[(start_coef + i) as usize];
                i += 1;
                i;
            }
            ((*s).abs_pow34).expect("non-null function pointer")(S34, SENT, num_coeffs);
            if cb_n < RESERVED_BT as libc::c_int {
                cb_p = av_clip_c(
                    find_min_book(
                        find_max_val(1 as libc::c_int, num_coeffs, S34),
                        (*sce).sf_idx[sfb as usize],
                    ),
                    cb_min,
                    cb_max,
                );
            } else {
                cb_p = cb_n;
            }
            ff_quantize_and_encode_band_cost(
                s,
                std::ptr::null_mut::<PutBitContext>(),
                SENT,
                QERR,
                S34,
                num_coeffs,
                (*sce).sf_idx[sfb as usize],
                cb_p,
                (*s).lambda / (*band).threshold,
                ::core::f32::INFINITY,
                &mut cost2,
                std::ptr::null_mut::<libc::c_float>(),
            );
            i = 0 as libc::c_int;
            while i < num_coeffs {
                (*sce).prcoeffs[(start_coef + i) as usize] += if *QERR.offset(i as isize) != 0.0f32
                {
                    (*sce).prcoeffs[(start_coef + i) as usize] - *QERR.offset(i as isize)
                } else {
                    0.0f32
                };
                i += 1;
                i;
            }
            ((*s).abs_pow34).expect("non-null function pointer")(
                P34,
                &mut *((*sce).prcoeffs).as_mut_ptr().offset(start_coef as isize),
                num_coeffs,
            );
            if cb_n < RESERVED_BT as libc::c_int {
                cb_p = av_clip_c(
                    find_min_book(
                        find_max_val(1 as libc::c_int, num_coeffs, P34),
                        (*sce).sf_idx[sfb as usize],
                    ),
                    cb_min,
                    cb_max,
                );
            } else {
                cb_p = cb_n;
            }
            dist2 = ff_quantize_and_encode_band_cost(
                s,
                std::ptr::null_mut::<PutBitContext>(),
                &mut *((*sce).prcoeffs).as_mut_ptr().offset(start_coef as isize),
                std::ptr::null_mut::<libc::c_float>(),
                P34,
                num_coeffs,
                (*sce).sf_idx[sfb as usize],
                cb_p,
                (*s).lambda / (*band).threshold,
                ::core::f32::INFINITY,
                std::ptr::null_mut::<libc::c_int>(),
                std::ptr::null_mut::<libc::c_float>(),
            );
            i = 0 as libc::c_int;
            while i < num_coeffs {
                dist_spec_err += (*O34.offset(i as isize) - *P34.offset(i as isize))
                    * (*O34.offset(i as isize) - *P34.offset(i as isize));
                i += 1;
                i;
            }
            dist_spec_err *= (*s).lambda / (*band).threshold;
            dist2 += dist_spec_err;
            if dist2 <= dist1 && cb_p <= cb_n {
                cost_pred += cost2;
                (*sce).ics.prediction_used[sfb as usize] = 1 as libc::c_int as uint8_t;
                (*sce).band_alt[sfb as usize] = cb_n as BandType;
                (*sce).band_type[sfb as usize] = cb_p as BandType;
                count += 1;
                count;
            } else {
                cost_pred += cost1;
                (*sce).band_alt[sfb as usize] = cb_p as BandType;
            }
        }
        sfb += 1;
        sfb;
    }
    if count != 0 && cost_coeffs < cost_pred {
        count = 0 as libc::c_int;
        sfb = 10 as libc::c_int;
        while sfb < pmax {
            if (*sce).ics.prediction_used[sfb as usize] != 0 {
                (*sce).ics.prediction_used[sfb as usize] = 0 as libc::c_int as uint8_t;
                (*sce).band_type[sfb as usize] = (*sce).band_alt[sfb as usize];
            }
            sfb += 1;
            sfb;
        }
        (*sce).ics.prediction_used.fill(0);
    }
    (*sce).ics.predictor_present = (count != 0) as libc::c_int;
}

pub(crate) unsafe extern "C" fn ff_aac_encode_main_pred(
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
) {
    let mut sfb: libc::c_int = 0;
    let mut ics: *mut IndividualChannelStream = &mut (*sce).ics;
    let pmax: libc::c_int = if (*ics).max_sfb as libc::c_int
        > *ff_aac_pred_sfb_max
            .as_ptr()
            .offset((*s).samplerate_index as isize) as libc::c_int
    {
        *ff_aac_pred_sfb_max
            .as_ptr()
            .offset((*s).samplerate_index as isize) as libc::c_int
    } else {
        (*ics).max_sfb as libc::c_int
    };
    if (*s).profile != 0 as libc::c_int || (*ics).predictor_present == 0 {
        return;
    }
    put_bits(
        &mut (*s).pb,
        1 as libc::c_int,
        ((*ics).predictor_reset_group != 0) as libc::c_int as BitBuf,
    );
    if (*ics).predictor_reset_group != 0 {
        put_bits(
            &mut (*s).pb,
            5 as libc::c_int,
            (*ics).predictor_reset_group as BitBuf,
        );
    }
    sfb = 0 as libc::c_int;
    while sfb < pmax {
        put_bits(
            &mut (*s).pb,
            1 as libc::c_int,
            (*ics).prediction_used[sfb as usize] as BitBuf,
        );
        sfb += 1;
        sfb;
    }
}
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
