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

use libc::{
    c_double, c_float, c_int, c_uchar, c_uint, c_ulong,
};

use crate::{
    aaccoder::ff_quantize_and_encode_band_cost, aactab::ff_aac_pow34sf_tab, common::*, types::*,
};

#[inline]
unsafe fn pos_pow34(mut a: c_float) -> c_float {
    sqrtf(a * sqrtf(a))
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
unsafe fn find_min_book(mut maxval: c_float, mut sf: c_int) -> c_int {
    let mut Q34: c_float =
        ff_aac_pow34sf_tab[(200 as c_int - sf + 140 as c_int - 36 as c_int) as usize];
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
        std::ptr::null_mut::<PutBitContext>(),
        in_0,
        std::ptr::null_mut::<c_float>(),
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

pub(crate) unsafe fn ff_aac_is_encoding_err(
    mut s: *mut AACEncContext,
    mut cpe: *mut ChannelElement,
    mut start: c_int,
    mut w: c_int,
    mut g: c_int,
    mut ener0: c_float,
    mut ener1: c_float,
    mut ener01: c_float,
    mut use_pcoeffs: c_int,
    mut phase: c_int,
) -> AACISError {
    let mut i: c_int = 0;
    let mut w2: c_int = 0;
    let mut sce0: *mut SingleChannelElement =
        &mut *((*cpe).ch).as_mut_ptr().offset(0 as c_int as isize) as *mut SingleChannelElement;
    let mut sce1: *mut SingleChannelElement =
        &mut *((*cpe).ch).as_mut_ptr().offset(1 as c_int as isize) as *mut SingleChannelElement;
    let mut L: *mut c_float = if use_pcoeffs != 0 {
        ((*sce0).pcoeffs).as_mut_ptr()
    } else {
        ((*sce0).coeffs).as_mut_ptr()
    };
    let mut R: *mut c_float = if use_pcoeffs != 0 {
        ((*sce1).pcoeffs).as_mut_ptr()
    } else {
        ((*sce1).coeffs).as_mut_ptr()
    };
    let mut L34: *mut c_float = &mut *((*s).scoefs)
        .as_mut_ptr()
        .offset((256 as c_int * 0 as c_int) as isize)
        as *mut c_float;
    let mut R34: *mut c_float = &mut *((*s).scoefs)
        .as_mut_ptr()
        .offset((256 as c_int * 1 as c_int) as isize)
        as *mut c_float;
    let mut IS: *mut c_float = &mut *((*s).scoefs)
        .as_mut_ptr()
        .offset((256 as c_int * 2 as c_int) as isize)
        as *mut c_float;
    let mut I34: *mut c_float = &mut *((*s).scoefs)
        .as_mut_ptr()
        .offset((256 as c_int * 3 as c_int) as isize)
        as *mut c_float;
    let mut dist1: c_float = 0.0f32;
    let mut dist2: c_float = 0.0f32;
    let mut is_error: AACISError = {
        AACISError {
            pass: 0 as c_int,
            phase: 0,
            error: 0.,
            dist1: 0.,
            dist2: 0.,
            ener01: 0.,
        }
    };
    if ener01 <= 0 as c_int as c_float || ener0 <= 0 as c_int as c_float {
        is_error.pass = 0 as c_int;
        return is_error;
    }
    w2 = 0 as c_int;
    while w2 < (*sce0).ics.group_len[w as usize] as c_int {
        let mut band0: *mut FFPsyBand =
            &mut *((*((*s).psy.ch).offset(((*s).cur_channel + 0 as c_int) as isize)).psy_bands)
                .as_mut_ptr()
                .offset(((w + w2) * 16 as c_int + g) as isize) as *mut FFPsyBand;
        let mut band1: *mut FFPsyBand =
            &mut *((*((*s).psy.ch).offset(((*s).cur_channel + 1 as c_int) as isize)).psy_bands)
                .as_mut_ptr()
                .offset(((w + w2) * 16 as c_int + g) as isize) as *mut FFPsyBand;
        let mut is_band_type: c_int = 0;
        let mut is_sf_idx: c_int =
            if 1 as c_int > (*sce0).sf_idx[(w * 16 as c_int + g) as usize] - 4 as c_int {
                1 as c_int
            } else {
                (*sce0).sf_idx[(w * 16 as c_int + g) as usize] - 4 as c_int
            };
        let mut e01_34: c_float = phase as c_float * pos_pow34(ener1 / ener0);
        let mut maxval: c_float = 0.;
        let mut dist_spec_err: c_float = 0.0f32;
        let mut minthr: c_float = if (*band0).threshold > (*band1).threshold {
            (*band1).threshold
        } else {
            (*band0).threshold
        };
        i = 0 as c_int;
        while i < *((*sce0).ics.swb_sizes).offset(g as isize) as c_int {
            *IS.offset(i as isize) = ((*L.offset((start + (w + w2) * 128 as c_int + i) as isize)
                + phase as c_float * *R.offset((start + (w + w2) * 128 as c_int + i) as isize))
                as c_double
                * sqrt((ener0 / ener01) as c_double))
                as c_float;
            i += 1;
            i;
        }
        ((*s).abs_pow34).expect("non-null function pointer")(
            L34,
            &mut *L.offset((start + (w + w2) * 128 as c_int) as isize),
            *((*sce0).ics.swb_sizes).offset(g as isize) as c_int,
        );
        ((*s).abs_pow34).expect("non-null function pointer")(
            R34,
            &mut *R.offset((start + (w + w2) * 128 as c_int) as isize),
            *((*sce0).ics.swb_sizes).offset(g as isize) as c_int,
        );
        ((*s).abs_pow34).expect("non-null function pointer")(
            I34,
            IS,
            *((*sce0).ics.swb_sizes).offset(g as isize) as c_int,
        );
        maxval = find_max_val(
            1 as c_int,
            *((*sce0).ics.swb_sizes).offset(g as isize) as c_int,
            I34,
        );
        is_band_type = find_min_book(maxval, is_sf_idx);
        dist1 += quantize_band_cost(
            s,
            &mut *L.offset((start + (w + w2) * 128 as c_int) as isize),
            L34,
            *((*sce0).ics.swb_sizes).offset(g as isize) as c_int,
            (*sce0).sf_idx[(w * 16 as c_int + g) as usize],
            (*sce0).band_type[(w * 16 as c_int + g) as usize] as c_int,
            (*s).lambda / (*band0).threshold,
            ::core::f32::INFINITY,
            std::ptr::null_mut::<c_int>(),
            std::ptr::null_mut::<c_float>(),
        );
        dist1 += quantize_band_cost(
            s,
            &mut *R.offset((start + (w + w2) * 128 as c_int) as isize),
            R34,
            *((*sce1).ics.swb_sizes).offset(g as isize) as c_int,
            (*sce1).sf_idx[(w * 16 as c_int + g) as usize],
            (*sce1).band_type[(w * 16 as c_int + g) as usize] as c_int,
            (*s).lambda / (*band1).threshold,
            ::core::f32::INFINITY,
            std::ptr::null_mut::<c_int>(),
            std::ptr::null_mut::<c_float>(),
        );
        dist2 += quantize_band_cost(
            s,
            IS,
            I34,
            *((*sce0).ics.swb_sizes).offset(g as isize) as c_int,
            is_sf_idx,
            is_band_type,
            (*s).lambda / minthr,
            ::core::f32::INFINITY,
            std::ptr::null_mut::<c_int>(),
            std::ptr::null_mut::<c_float>(),
        );
        i = 0 as c_int;
        while i < *((*sce0).ics.swb_sizes).offset(g as isize) as c_int {
            dist_spec_err += (*L34.offset(i as isize) - *I34.offset(i as isize))
                * (*L34.offset(i as isize) - *I34.offset(i as isize));
            dist_spec_err += (*R34.offset(i as isize) - *I34.offset(i as isize) * e01_34)
                * (*R34.offset(i as isize) - *I34.offset(i as isize) * e01_34);
            i += 1;
            i;
        }
        dist_spec_err *= (*s).lambda / minthr;
        dist2 += dist_spec_err;
        w2 += 1;
        w2;
    }
    is_error.pass = (dist2 <= dist1) as c_int;
    is_error.phase = phase;
    is_error.error = dist2 - dist1;
    is_error.dist1 = dist1;
    is_error.dist2 = dist2;
    is_error.ener01 = ener01;
    is_error
}

pub(crate) unsafe extern "C" fn ff_aac_search_for_is(
    mut s: *mut AACEncContext,
    mut avctx: *mut AVCodecContext,
    mut cpe: *mut ChannelElement,
) {
    let mut sce0: *mut SingleChannelElement =
        &mut *((*cpe).ch).as_mut_ptr().offset(0 as c_int as isize) as *mut SingleChannelElement;
    let mut sce1: *mut SingleChannelElement =
        &mut *((*cpe).ch).as_mut_ptr().offset(1 as c_int as isize) as *mut SingleChannelElement;
    let mut start: c_int = 0 as c_int;
    let mut count: c_int = 0 as c_int;
    let mut w: c_int = 0;
    let mut w2: c_int = 0;
    let mut g: c_int = 0;
    let mut i: c_int = 0;
    let mut prev_sf1: c_int = -(1 as c_int);
    let mut prev_bt: c_int = -(1 as c_int);
    let mut prev_is: c_int = 0 as c_int;
    let freq_mult: c_float =
        (*avctx).sample_rate as c_float / (1024.0f32 / (*sce0).ics.num_windows as c_float) / 2.0f32;
    let mut nextband1: [c_uchar; 128] = [0; 128];
    if (*cpe).common_window == 0 {
        return;
    }
    ff_init_nextband_map(sce1, nextband1.as_mut_ptr());
    w = 0 as c_int;
    while w < (*sce0).ics.num_windows {
        start = 0 as c_int;
        g = 0 as c_int;
        while g < (*sce0).ics.num_swb {
            if start as c_float * freq_mult > 6100 as c_int as c_float * ((*s).lambda / 170.0f32)
                && (*cpe).ch[0 as c_int as usize].band_type[(w * 16 as c_int + g) as usize]
                    as c_uint
                    != NOISE_BT as c_int as c_uint
                && (*cpe).ch[0 as c_int as usize].zeroes[(w * 16 as c_int + g) as usize] == 0
                && (*cpe).ch[1 as c_int as usize].band_type[(w * 16 as c_int + g) as usize]
                    as c_uint
                    != NOISE_BT as c_int as c_uint
                && (*cpe).ch[1 as c_int as usize].zeroes[(w * 16 as c_int + g) as usize] == 0
                && ff_sfdelta_can_remove_band(
                    sce1,
                    nextband1.as_mut_ptr(),
                    prev_sf1,
                    w * 16 as c_int + g,
                ) != 0
            {
                let mut ener0: c_float = 0.0f32;
                let mut ener1: c_float = 0.0f32;
                let mut ener01: c_float = 0.0f32;
                let mut ener01p: c_float = 0.0f32;
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
                let mut best: *mut AACISError = std::ptr::null_mut::<AACISError>();
                w2 = 0 as c_int;
                while w2 < (*sce0).ics.group_len[w as usize] as c_int {
                    i = 0 as c_int;
                    while i < *((*sce0).ics.swb_sizes).offset(g as isize) as c_int {
                        let mut coef0: c_float =
                            (*sce0).coeffs[(start + (w + w2) * 128 as c_int + i) as usize];
                        let mut coef1: c_float =
                            (*sce1).coeffs[(start + (w + w2) * 128 as c_int + i) as usize];
                        ener0 += coef0 * coef0;
                        ener1 += coef1 * coef1;
                        ener01 += (coef0 + coef1) * (coef0 + coef1);
                        ener01p += (coef0 - coef1) * (coef0 - coef1);
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
                    ener01p,
                    0 as c_int,
                    -(1 as c_int),
                );
                ph_err2 = ff_aac_is_encoding_err(
                    s, cpe, start, w, g, ener0, ener1, ener01, 0 as c_int, 1 as c_int,
                );
                best = if ph_err1.pass != 0 && ph_err1.error < ph_err2.error {
                    &mut ph_err1
                } else {
                    &mut ph_err2
                };
                if (*best).pass != 0 {
                    (*cpe).is_mask[(w * 16 as c_int + g) as usize] = 1 as c_int as c_uchar;
                    (*cpe).ms_mask[(w * 16 as c_int + g) as usize] = 0 as c_int as c_uchar;
                    (*cpe).ch[0 as c_int as usize].is_ener[(w * 16 as c_int + g) as usize] =
                        sqrt((ener0 / (*best).ener01) as c_double) as c_float;
                    (*cpe).ch[1 as c_int as usize].is_ener[(w * 16 as c_int + g) as usize] =
                        ener0 / ener1;
                    (*cpe).ch[1 as c_int as usize].band_type[(w * 16 as c_int + g) as usize] =
                        (if (*best).phase > 0 as c_int {
                            INTENSITY_BT as c_int
                        } else {
                            INTENSITY_BT2 as c_int
                        }) as BandType;
                    if prev_is != 0
                        && prev_bt as c_uint
                            != (*cpe).ch[1 as c_int as usize].band_type
                                [(w * 16 as c_int + g) as usize]
                                as c_uint
                    {
                        (*cpe).ms_mask[(w * 16 as c_int + g) as usize] = 1 as c_int as c_uchar;
                        (*cpe).ch[1 as c_int as usize].band_type[(w * 16 as c_int + g) as usize] =
                            (if (*best).phase > 0 as c_int {
                                INTENSITY_BT2 as c_int
                            } else {
                                INTENSITY_BT as c_int
                            }) as BandType;
                    }
                    prev_bt = (*cpe).ch[1 as c_int as usize].band_type
                        [(w * 16 as c_int + g) as usize] as c_int;
                    count += 1;
                    count;
                }
            }
            if (*sce1).zeroes[(w * 16 as c_int + g) as usize] == 0
                && ((*sce1).band_type[(w * 16 as c_int + g) as usize] as c_uint)
                    < RESERVED_BT as c_int as c_uint
            {
                prev_sf1 = (*sce1).sf_idx[(w * 16 as c_int + g) as usize];
            }
            prev_is = (*cpe).is_mask[(w * 16 as c_int + g) as usize] as c_int;
            start += *((*sce0).ics.swb_sizes).offset(g as isize) as c_int;
            g += 1;
            g;
        }
        w += (*sce0).ics.group_len[w as usize] as c_int;
    }
    (*cpe).is_mode = (count != 0) as c_int as c_uchar;
}
