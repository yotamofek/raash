#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use crate::aaccoder::ff_quantize_and_encode_band_cost;
use crate::aactab::ff_aac_pow34sf_tab;
use crate::common::*;
use crate::types::*;

#[inline]
unsafe extern "C" fn pos_pow34(mut a: libc::c_float) -> libc::c_float {
    return sqrtf(a * sqrtf(a));
}
#[inline]
unsafe extern "C" fn find_max_val(
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
    return maxval;
}
#[inline]
unsafe extern "C" fn find_min_book(mut maxval: libc::c_float, mut sf: libc::c_int) -> libc::c_int {
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
    return cb;
}
#[inline]
unsafe extern "C" fn ff_init_nextband_map(
    mut sce: *const SingleChannelElement,
    mut nextband: *mut uint8_t,
) {
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
                let ref mut fresh0 = *nextband.offset(prevband as isize);
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
unsafe extern "C" fn ff_sfdelta_can_remove_band(
    mut sce: *const SingleChannelElement,
    mut nextband: *const uint8_t,
    mut prev_sf: libc::c_int,
    mut band: libc::c_int,
) -> libc::c_int {
    return (prev_sf >= 0 as libc::c_int
        && (*sce).sf_idx[*nextband.offset(band as isize) as usize] >= prev_sf - 60 as libc::c_int
        && (*sce).sf_idx[*nextband.offset(band as isize) as usize] <= prev_sf + 60 as libc::c_int)
        as libc::c_int;
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
#[no_mangle]
pub unsafe extern "C" fn ff_aac_is_encoding_err(
    mut s: *mut AACEncContext,
    mut cpe: *mut ChannelElement,
    mut start: libc::c_int,
    mut w: libc::c_int,
    mut g: libc::c_int,
    mut ener0: libc::c_float,
    mut ener1: libc::c_float,
    mut ener01: libc::c_float,
    mut use_pcoeffs: libc::c_int,
    mut phase: libc::c_int,
) -> AACISError {
    let mut i: libc::c_int = 0;
    let mut w2: libc::c_int = 0;
    let mut sce0: *mut SingleChannelElement =
        &mut *((*cpe).ch).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut SingleChannelElement;
    let mut sce1: *mut SingleChannelElement =
        &mut *((*cpe).ch).as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut SingleChannelElement;
    let mut L: *mut libc::c_float = if use_pcoeffs != 0 {
        ((*sce0).pcoeffs).as_mut_ptr()
    } else {
        ((*sce0).coeffs).as_mut_ptr()
    };
    let mut R: *mut libc::c_float = if use_pcoeffs != 0 {
        ((*sce1).pcoeffs).as_mut_ptr()
    } else {
        ((*sce1).coeffs).as_mut_ptr()
    };
    let mut L34: *mut libc::c_float = &mut *((*s).scoefs)
        .as_mut_ptr()
        .offset((256 as libc::c_int * 0 as libc::c_int) as isize)
        as *mut libc::c_float;
    let mut R34: *mut libc::c_float = &mut *((*s).scoefs)
        .as_mut_ptr()
        .offset((256 as libc::c_int * 1 as libc::c_int) as isize)
        as *mut libc::c_float;
    let mut IS: *mut libc::c_float = &mut *((*s).scoefs)
        .as_mut_ptr()
        .offset((256 as libc::c_int * 2 as libc::c_int) as isize)
        as *mut libc::c_float;
    let mut I34: *mut libc::c_float = &mut *((*s).scoefs)
        .as_mut_ptr()
        .offset((256 as libc::c_int * 3 as libc::c_int) as isize)
        as *mut libc::c_float;
    let mut dist1: libc::c_float = 0.0f32;
    let mut dist2: libc::c_float = 0.0f32;
    let mut is_error: AACISError = {
        let mut init = AACISError {
            pass: 0 as libc::c_int,
            phase: 0,
            error: 0.,
            dist1: 0.,
            dist2: 0.,
            ener01: 0.,
        };
        init
    };
    if ener01 <= 0 as libc::c_int as libc::c_float || ener0 <= 0 as libc::c_int as libc::c_float {
        is_error.pass = 0 as libc::c_int;
        return is_error;
    }
    w2 = 0 as libc::c_int;
    while w2 < (*sce0).ics.group_len[w as usize] as libc::c_int {
        let mut band0: *mut FFPsyBand =
            &mut *((*((*s).psy.ch).offset(((*s).cur_channel + 0 as libc::c_int) as isize))
                .psy_bands)
                .as_mut_ptr()
                .offset(((w + w2) * 16 as libc::c_int + g) as isize) as *mut FFPsyBand;
        let mut band1: *mut FFPsyBand =
            &mut *((*((*s).psy.ch).offset(((*s).cur_channel + 1 as libc::c_int) as isize))
                .psy_bands)
                .as_mut_ptr()
                .offset(((w + w2) * 16 as libc::c_int + g) as isize) as *mut FFPsyBand;
        let mut is_band_type: libc::c_int = 0;
        let mut is_sf_idx: libc::c_int = if 1 as libc::c_int
            > (*sce0).sf_idx[(w * 16 as libc::c_int + g) as usize] - 4 as libc::c_int
        {
            1 as libc::c_int
        } else {
            (*sce0).sf_idx[(w * 16 as libc::c_int + g) as usize] - 4 as libc::c_int
        };
        let mut e01_34: libc::c_float = phase as libc::c_float * pos_pow34(ener1 / ener0);
        let mut maxval: libc::c_float = 0.;
        let mut dist_spec_err: libc::c_float = 0.0f32;
        let mut minthr: libc::c_float = if (*band0).threshold > (*band1).threshold {
            (*band1).threshold
        } else {
            (*band0).threshold
        };
        i = 0 as libc::c_int;
        while i < *((*sce0).ics.swb_sizes).offset(g as isize) as libc::c_int {
            *IS.offset(i as isize) =
                ((*L.offset((start + (w + w2) * 128 as libc::c_int + i) as isize)
                    + phase as libc::c_float
                        * *R.offset((start + (w + w2) * 128 as libc::c_int + i) as isize))
                    as libc::c_double
                    * sqrt((ener0 / ener01) as libc::c_double)) as libc::c_float;
            i += 1;
            i;
        }
        ((*s).abs_pow34).expect("non-null function pointer")(
            L34,
            &mut *L.offset((start + (w + w2) * 128 as libc::c_int) as isize),
            *((*sce0).ics.swb_sizes).offset(g as isize) as libc::c_int,
        );
        ((*s).abs_pow34).expect("non-null function pointer")(
            R34,
            &mut *R.offset((start + (w + w2) * 128 as libc::c_int) as isize),
            *((*sce0).ics.swb_sizes).offset(g as isize) as libc::c_int,
        );
        ((*s).abs_pow34).expect("non-null function pointer")(
            I34,
            IS,
            *((*sce0).ics.swb_sizes).offset(g as isize) as libc::c_int,
        );
        maxval = find_max_val(
            1 as libc::c_int,
            *((*sce0).ics.swb_sizes).offset(g as isize) as libc::c_int,
            I34,
        );
        is_band_type = find_min_book(maxval, is_sf_idx);
        dist1 += quantize_band_cost(
            s,
            &mut *L.offset((start + (w + w2) * 128 as libc::c_int) as isize),
            L34,
            *((*sce0).ics.swb_sizes).offset(g as isize) as libc::c_int,
            (*sce0).sf_idx[(w * 16 as libc::c_int + g) as usize],
            (*sce0).band_type[(w * 16 as libc::c_int + g) as usize] as libc::c_int,
            (*s).lambda / (*band0).threshold,
            ::core::f32::INFINITY,
            0 as *mut libc::c_int,
            0 as *mut libc::c_float,
        );
        dist1 += quantize_band_cost(
            s,
            &mut *R.offset((start + (w + w2) * 128 as libc::c_int) as isize),
            R34,
            *((*sce1).ics.swb_sizes).offset(g as isize) as libc::c_int,
            (*sce1).sf_idx[(w * 16 as libc::c_int + g) as usize],
            (*sce1).band_type[(w * 16 as libc::c_int + g) as usize] as libc::c_int,
            (*s).lambda / (*band1).threshold,
            ::core::f32::INFINITY,
            0 as *mut libc::c_int,
            0 as *mut libc::c_float,
        );
        dist2 += quantize_band_cost(
            s,
            IS,
            I34,
            *((*sce0).ics.swb_sizes).offset(g as isize) as libc::c_int,
            is_sf_idx,
            is_band_type,
            (*s).lambda / minthr,
            ::core::f32::INFINITY,
            0 as *mut libc::c_int,
            0 as *mut libc::c_float,
        );
        i = 0 as libc::c_int;
        while i < *((*sce0).ics.swb_sizes).offset(g as isize) as libc::c_int {
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
    is_error.pass = (dist2 <= dist1) as libc::c_int;
    is_error.phase = phase;
    is_error.error = dist2 - dist1;
    is_error.dist1 = dist1;
    is_error.dist2 = dist2;
    is_error.ener01 = ener01;
    return is_error;
}
#[no_mangle]
pub unsafe extern "C" fn ff_aac_search_for_is(
    mut s: *mut AACEncContext,
    mut avctx: *mut AVCodecContext,
    mut cpe: *mut ChannelElement,
) {
    let mut sce0: *mut SingleChannelElement =
        &mut *((*cpe).ch).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut SingleChannelElement;
    let mut sce1: *mut SingleChannelElement =
        &mut *((*cpe).ch).as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut SingleChannelElement;
    let mut start: libc::c_int = 0 as libc::c_int;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut w: libc::c_int = 0;
    let mut w2: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut prev_sf1: libc::c_int = -(1 as libc::c_int);
    let mut prev_bt: libc::c_int = -(1 as libc::c_int);
    let mut prev_is: libc::c_int = 0 as libc::c_int;
    let freq_mult: libc::c_float = (*avctx).sample_rate as libc::c_float
        / (1024.0f32 / (*sce0).ics.num_windows as libc::c_float)
        / 2.0f32;
    let mut nextband1: [uint8_t; 128] = [0; 128];
    if (*cpe).common_window == 0 {
        return;
    }
    ff_init_nextband_map(sce1, nextband1.as_mut_ptr());
    w = 0 as libc::c_int;
    while w < (*sce0).ics.num_windows {
        start = 0 as libc::c_int;
        g = 0 as libc::c_int;
        while g < (*sce0).ics.num_swb {
            if start as libc::c_float * freq_mult
                > 6100 as libc::c_int as libc::c_float * ((*s).lambda / 170.0f32)
                && (*cpe).ch[0 as libc::c_int as usize].band_type
                    [(w * 16 as libc::c_int + g) as usize] as libc::c_uint
                    != NOISE_BT as libc::c_int as libc::c_uint
                && (*cpe).ch[0 as libc::c_int as usize].zeroes[(w * 16 as libc::c_int + g) as usize]
                    == 0
                && (*cpe).ch[1 as libc::c_int as usize].band_type
                    [(w * 16 as libc::c_int + g) as usize] as libc::c_uint
                    != NOISE_BT as libc::c_int as libc::c_uint
                && (*cpe).ch[1 as libc::c_int as usize].zeroes[(w * 16 as libc::c_int + g) as usize]
                    == 0
                && ff_sfdelta_can_remove_band(
                    sce1,
                    nextband1.as_mut_ptr(),
                    prev_sf1,
                    w * 16 as libc::c_int + g,
                ) != 0
            {
                let mut ener0: libc::c_float = 0.0f32;
                let mut ener1: libc::c_float = 0.0f32;
                let mut ener01: libc::c_float = 0.0f32;
                let mut ener01p: libc::c_float = 0.0f32;
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
                let mut best: *mut AACISError = 0 as *mut AACISError;
                w2 = 0 as libc::c_int;
                while w2 < (*sce0).ics.group_len[w as usize] as libc::c_int {
                    i = 0 as libc::c_int;
                    while i < *((*sce0).ics.swb_sizes).offset(g as isize) as libc::c_int {
                        let mut coef0: libc::c_float =
                            (*sce0).coeffs[(start + (w + w2) * 128 as libc::c_int + i) as usize];
                        let mut coef1: libc::c_float =
                            (*sce1).coeffs[(start + (w + w2) * 128 as libc::c_int + i) as usize];
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
                    0 as libc::c_int,
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
                    0 as libc::c_int,
                    1 as libc::c_int,
                );
                best = if ph_err1.pass != 0 && ph_err1.error < ph_err2.error {
                    &mut ph_err1
                } else {
                    &mut ph_err2
                };
                if (*best).pass != 0 {
                    (*cpe).is_mask[(w * 16 as libc::c_int + g) as usize] =
                        1 as libc::c_int as uint8_t;
                    (*cpe).ms_mask[(w * 16 as libc::c_int + g) as usize] =
                        0 as libc::c_int as uint8_t;
                    (*cpe).ch[0 as libc::c_int as usize].is_ener
                        [(w * 16 as libc::c_int + g) as usize] =
                        sqrt((ener0 / (*best).ener01) as libc::c_double) as libc::c_float;
                    (*cpe).ch[1 as libc::c_int as usize].is_ener
                        [(w * 16 as libc::c_int + g) as usize] = ener0 / ener1;
                    (*cpe).ch[1 as libc::c_int as usize].band_type
                        [(w * 16 as libc::c_int + g) as usize] =
                        (if (*best).phase > 0 as libc::c_int {
                            INTENSITY_BT as libc::c_int
                        } else {
                            INTENSITY_BT2 as libc::c_int
                        }) as BandType;
                    if prev_is != 0
                        && prev_bt as libc::c_uint
                            != (*cpe).ch[1 as libc::c_int as usize].band_type
                                [(w * 16 as libc::c_int + g) as usize]
                                as libc::c_uint
                    {
                        (*cpe).ms_mask[(w * 16 as libc::c_int + g) as usize] =
                            1 as libc::c_int as uint8_t;
                        (*cpe).ch[1 as libc::c_int as usize].band_type
                            [(w * 16 as libc::c_int + g) as usize] =
                            (if (*best).phase > 0 as libc::c_int {
                                INTENSITY_BT2 as libc::c_int
                            } else {
                                INTENSITY_BT as libc::c_int
                            }) as BandType;
                    }
                    prev_bt = (*cpe).ch[1 as libc::c_int as usize].band_type
                        [(w * 16 as libc::c_int + g) as usize]
                        as libc::c_int;
                    count += 1;
                    count;
                }
            }
            if (*sce1).zeroes[(w * 16 as libc::c_int + g) as usize] == 0
                && ((*sce1).band_type[(w * 16 as libc::c_int + g) as usize] as libc::c_uint)
                    < RESERVED_BT as libc::c_int as libc::c_uint
            {
                prev_sf1 = (*sce1).sf_idx[(w * 16 as libc::c_int + g) as usize];
            }
            prev_is = (*cpe).is_mask[(w * 16 as libc::c_int + g) as usize] as libc::c_int;
            start += *((*sce0).ics.swb_sizes).offset(g as isize) as libc::c_int;
            g += 1;
            g;
        }
        w += (*sce0).ics.group_len[w as usize] as libc::c_int;
    }
    (*cpe).is_mode = (count != 0) as libc::c_int as uint8_t;
}
