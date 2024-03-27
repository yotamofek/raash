#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use std::{ptr, slice};

use ffi::codec::AVCodecContext;
use libc::{c_double, c_float, c_int, c_uchar, c_uint};

use crate::{
    aac::{
        coder::quantize_and_encode_band::quantize_and_encode_band_cost,
        encoder::{abs_pow34_v, ctx::AACEncContext},
        tables::POW_SF_TABLES,
    },
    common::*,
    types::*,
};

#[inline]
fn pos_pow34(mut a: c_float) -> c_float {
    sqrtf(a * sqrtf(a))
}

#[inline]
unsafe fn find_max_val(
    mut group_len: c_int,
    mut swb_size: c_int,
    mut scaled: *const c_float,
) -> c_float {
    let scaled = slice::from_raw_parts::<c_float>(scaled, 128 * group_len as usize);
    scaled
        .array_chunks::<128>()
        .flat_map(|row| &row[..swb_size as usize])
        .copied()
        .max_by(f32::total_cmp)
        .unwrap()
}

#[inline]
fn find_min_book(mut maxval: c_float, mut sf: c_int) -> c_int {
    let mut Q34: c_float = POW_SF_TABLES.pow34[(200 - sf + 140 - 36) as usize];
    let qmaxval = (maxval * Q34 + 0.405_4_f32) as usize;
    aac_maxval_cb.get(qmaxval).copied().unwrap_or(11) as c_int
}

#[inline]
unsafe fn ff_sfdelta_can_remove_band(
    mut sce: *const SingleChannelElement,
    mut nextband: *const c_uchar,
    mut prev_sf: c_int,
    mut band: c_int,
) -> c_int {
    (prev_sf >= 0
        && (*sce).sf_idx[*nextband.offset(band as isize) as usize] >= prev_sf - 60
        && (*sce).sf_idx[*nextband.offset(band as isize) as usize] <= prev_sf + 60) as c_int
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

    let [sce0, sce1] = &mut ((*cpe).ch);

    let mut L: *mut c_float = if use_pcoeffs != 0 {
        (sce0.pcoeffs).as_mut_ptr()
    } else {
        (sce0.coeffs).as_mut_ptr()
    };
    let mut R: *mut c_float = if use_pcoeffs != 0 {
        (sce1.pcoeffs).as_mut_ptr()
    } else {
        (sce1.coeffs).as_mut_ptr()
    };

    let [L34, R34, IS, I34] = [0, 1, 2, 3].map(|i| (*s).scoefs[256 * i..].as_mut_ptr());

    let mut dist1: c_float = 0.;
    let mut dist2: c_float = 0.;
    let mut is_error: AACISError = {
        AACISError {
            pass: 0,
            phase: 0,
            error: 0.,
            dist1: 0.,
            dist2: 0.,
            ener01: 0.,
        }
    };
    if ener01 <= 0. || ener0 <= 0. {
        is_error.pass = 0;
        return is_error;
    }
    w2 = 0;
    while w2 < sce0.ics.group_len[w as usize] as c_int {
        let mut band0: *mut FFPsyBand = &mut (*s).psy.ch[(*s).cur_channel as usize].psy_bands
            [((w + w2) * 16 + g) as usize]
            as *mut FFPsyBand;
        let mut band1: *mut FFPsyBand = &mut (*s).psy.ch[((*s).cur_channel + 1) as usize].psy_bands
            [((w + w2) * 16 + g) as usize]
            as *mut FFPsyBand;
        let mut is_band_type: c_int = 0;
        let mut is_sf_idx: c_int = if 1 > sce0.sf_idx[(w * 16 + g) as usize] - 4 {
            1
        } else {
            sce0.sf_idx[(w * 16 + g) as usize] - 4
        };
        let mut e01_34: c_float = phase as c_float * pos_pow34(ener1 / ener0);
        let mut maxval: c_float = 0.;
        let mut dist_spec_err: c_float = 0.;
        let mut minthr: c_float = if (*band0).threshold > (*band1).threshold {
            (*band1).threshold
        } else {
            (*band0).threshold
        };
        i = 0;
        while i < sce0.ics.swb_sizes[g as usize] as c_int {
            *IS.offset(i as isize) = ((*L.offset((start + (w + w2) * 128 + i) as isize)
                + phase as c_float * *R.offset((start + (w + w2) * 128 + i) as isize))
                as c_double
                * sqrt((ener0 / ener01) as c_double))
                as c_float;
            i += 1;
            i;
        }
        abs_pow34_v(
            L34,
            &mut *L.offset((start + (w + w2) * 128) as isize),
            sce0.ics.swb_sizes[g as usize] as c_int,
        );
        abs_pow34_v(
            R34,
            &mut *R.offset((start + (w + w2) * 128) as isize),
            sce0.ics.swb_sizes[g as usize] as c_int,
        );
        abs_pow34_v(I34, IS, sce0.ics.swb_sizes[g as usize] as c_int);
        maxval = find_max_val(1, sce0.ics.swb_sizes[g as usize] as c_int, I34);
        is_band_type = find_min_book(maxval, is_sf_idx);
        dist1 += quantize_band_cost(
            s,
            &mut *L.offset((start + (w + w2) * 128) as isize),
            L34,
            sce0.ics.swb_sizes[g as usize] as c_int,
            sce0.sf_idx[(w * 16 + g) as usize],
            sce0.band_type[(w * 16 + g) as usize] as c_int,
            (*s).lambda / (*band0).threshold,
            ::core::f32::INFINITY,
            std::ptr::null_mut::<c_int>(),
            std::ptr::null_mut::<c_float>(),
        );
        dist1 += quantize_band_cost(
            s,
            &mut *R.offset((start + (w + w2) * 128) as isize),
            R34,
            sce1.ics.swb_sizes[g as usize] as c_int,
            sce1.sf_idx[(w * 16 + g) as usize],
            sce1.band_type[(w * 16 + g) as usize] as c_int,
            (*s).lambda / (*band1).threshold,
            ::core::f32::INFINITY,
            std::ptr::null_mut::<c_int>(),
            std::ptr::null_mut::<c_float>(),
        );
        dist2 += quantize_band_cost(
            s,
            IS,
            I34,
            sce0.ics.swb_sizes[g as usize] as c_int,
            is_sf_idx,
            is_band_type,
            (*s).lambda / minthr,
            ::core::f32::INFINITY,
            std::ptr::null_mut::<c_int>(),
            std::ptr::null_mut::<c_float>(),
        );
        i = 0;
        while i < sce0.ics.swb_sizes[g as usize] as c_int {
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

pub(crate) unsafe fn search_for_is(
    mut s: *mut AACEncContext,
    mut avctx: *mut AVCodecContext,
    mut cpe: *mut ChannelElement,
) {
    let [sce0, sce1] = &mut (*cpe).ch;

    let mut start: c_int = 0;
    let mut count: c_int = 0;
    let mut w: c_int = 0;
    let mut w2: c_int = 0;
    let mut g: c_int = 0;
    let mut i: c_int = 0;
    let mut prev_sf1: c_int = -1;
    let mut prev_bt: c_int = -1;
    let mut prev_is: c_int = 0;
    let freq_mult: c_float =
        (*avctx).sample_rate as c_float / (1024. / sce0.ics.num_windows as c_float) / 2.;
    if (*cpe).common_window == 0 {
        return;
    }
    let mut nextband1 = ptr::from_mut(sce1).init_nextband_map();
    w = 0;
    while w < sce0.ics.num_windows {
        start = 0;
        g = 0;
        while g < sce0.ics.num_swb {
            if start as c_float * freq_mult > 6100. * ((*s).lambda / 170.)
                && (*cpe).ch[0].band_type[(w * 16 + g) as usize] as c_uint
                    != NOISE_BT as c_int as c_uint
                && !(*cpe).ch[0].zeroes[(w * 16 + g) as usize]
                && (*cpe).ch[1].band_type[(w * 16 + g) as usize] as c_uint
                    != NOISE_BT as c_int as c_uint
                && !(*cpe).ch[1].zeroes[(w * 16 + g) as usize]
                && ff_sfdelta_can_remove_band(sce1, nextband1.as_mut_ptr(), prev_sf1, w * 16 + g)
                    != 0
            {
                let mut ener0: c_float = 0.;
                let mut ener1: c_float = 0.;
                let mut ener01: c_float = 0.;
                let mut ener01p: c_float = 0.;
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
                w2 = 0;
                while w2 < sce0.ics.group_len[w as usize] as c_int {
                    i = 0;
                    while i < sce0.ics.swb_sizes[g as usize] as c_int {
                        let mut coef0: c_float = sce0.coeffs[(start + (w + w2) * 128 + i) as usize];
                        let mut coef1: c_float = sce1.coeffs[(start + (w + w2) * 128 + i) as usize];
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
                ph_err1 = ff_aac_is_encoding_err(s, cpe, start, w, g, ener0, ener1, ener01p, 0, -1);
                ph_err2 = ff_aac_is_encoding_err(s, cpe, start, w, g, ener0, ener1, ener01, 0, 1);
                best = if ph_err1.pass != 0 && ph_err1.error < ph_err2.error {
                    &mut ph_err1
                } else {
                    &mut ph_err2
                };
                if (*best).pass != 0 {
                    (*cpe).is_mask[(w * 16 + g) as usize] = 1;
                    (*cpe).ms_mask[(w * 16 + g) as usize] = 0;
                    (*cpe).ch[0].is_ener[(w * 16 + g) as usize] =
                        sqrt((ener0 / (*best).ener01) as c_double) as c_float;
                    (*cpe).ch[1].is_ener[(w * 16 + g) as usize] = ener0 / ener1;
                    (*cpe).ch[1].band_type[(w * 16 + g) as usize] = (if (*best).phase > 0 {
                        INTENSITY_BT as c_int
                    } else {
                        INTENSITY_BT2 as c_int
                    })
                        as BandType;
                    if prev_is != 0
                        && prev_bt as c_uint
                            != (*cpe).ch[1].band_type[(w * 16 + g) as usize] as c_uint
                    {
                        (*cpe).ms_mask[(w * 16 + g) as usize] = 1;
                        (*cpe).ch[1].band_type[(w * 16 + g) as usize] = (if (*best).phase > 0 {
                            INTENSITY_BT2 as c_int
                        } else {
                            INTENSITY_BT as c_int
                        })
                            as BandType;
                    }
                    prev_bt = (*cpe).ch[1].band_type[(w * 16 + g) as usize] as c_int;
                    count += 1;
                    count;
                }
            }
            if !sce1.zeroes[(w * 16 + g) as usize]
                && (sce1.band_type[(w * 16 + g) as usize] as c_uint)
                    < RESERVED_BT as c_int as c_uint
            {
                prev_sf1 = sce1.sf_idx[(w * 16 + g) as usize];
            }
            prev_is = (*cpe).is_mask[(w * 16 + g) as usize] as c_int;
            start += sce0.ics.swb_sizes[g as usize] as c_int;
            g += 1;
            g;
        }
        w += sce0.ics.group_len[w as usize] as c_int;
    }
    (*cpe).is_mode = (count != 0) as c_int as c_uchar;
}
