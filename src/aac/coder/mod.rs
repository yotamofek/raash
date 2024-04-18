#![allow(
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

mod math;
pub(super) mod mid_side;
pub(super) mod perceptual_noise_substitution;
pub(super) mod quantize_and_encode_band;
pub(super) mod quantizers;
pub(super) mod trellis;

use std::{array, iter, mem::size_of, ops::RangeInclusive};

use array_util::W;
use ffmpeg_src_macro::ffmpeg_src;
use libc::{c_char, c_float, c_int, c_long, c_uchar, c_uint, c_ulong};

use self::{
    math::{ff_fast_powf, mod_uintp2_c},
    quantize_and_encode_band::quantize_and_encode_band_cost,
};
use super::{
    encoder::ctx::QuantizeBandCostCache, tables::*, IndividualChannelStream, WindowedIteration,
    SCALE_MAX_DIFF,
};
use crate::{common::*, types::*};

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
static run_value_bits_long: [c_uchar; 64] = [
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
    10, 10, 10, 10, 10, 10, 10, 10, 15,
];
static run_value_bits_short: [c_uchar; 16] = [3, 3, 3, 3, 3, 3, 3, 6, 6, 6, 6, 6, 6, 6, 6, 9];
fn run_value_bits(num_windows: c_int) -> &'static [c_uchar] {
    if num_windows == 8 {
        &run_value_bits_short
    } else {
        &run_value_bits_long
    }
}
static aac_cb_out_map: [c_uchar; 15] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15];
static aac_cb_in_map: [c_uchar; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0, 12, 13, 14];

#[ffmpeg_src(file = "libavcodec/aacenctab.h", lines = 119, name = "aac_cb_range")]
static CB_RANGE: [c_uchar; 12] = [0, 3, 3, 3, 3, 9, 9, 8, 8, 13, 13, 17];
static aac_cb_maxval: [c_uchar; 12] = [0, 1, 1, 2, 2, 4, 4, 7, 7, 12, 12, 16];

static aac_maxval_cb: [c_uchar; 14] = [0, 1, 3, 5, 5, 7, 7, 7, 9, 9, 9, 9, 9, 11];
#[inline]
fn quant(mut coef: c_float, Q: c_float, rounding: c_float) -> c_int {
    let mut a = coef * Q;
    (sqrtf(a * sqrtf(a)) + rounding) as c_int
}

#[inline]
fn find_min_book(maxval: c_float, sf: c_int) -> c_int {
    let Q34: c_float = POW_SF_TABLES.pow34()[(200 - sf + 140 - 36) as usize];
    let qmaxval = (maxval * Q34 + 0.4054) as c_int;
    aac_maxval_cb.get(qmaxval as usize).copied().unwrap_or(11) as c_int
}

#[ffmpeg_src(file = "libavcodec/aacenc_utils.h", lines = 104..=154)]
#[inline]
fn find_form_factor(
    group_len: u8,
    swb_size: u8,
    thresh: c_float,
    scaled: &[c_float],
    nzslope: c_float,
) -> c_float {
    #[cfg(debug_assertions)]
    let _ = scaled[..128 * usize::from(group_len - 1)];

    let iswb_size: c_float = 1. / swb_size as c_float;
    let iswb_sizem1: c_float = 1. / (swb_size - 1) as c_float;
    let mut form: c_float = 0.;
    let mut weight: c_float = 0.;
    for scaled_window in scaled.chunks(128).take(group_len.into()) {
        let mut e = 0.;
        let mut e2 = 0.;
        let mut maxval = 0.;
        let mut nzl = 0.;
        for s in &scaled_window[..swb_size.into()] {
            let mut s = s.abs();
            maxval = c_float::max(maxval, s);
            e += s;
            s *= s;
            e2 += s;
            // We really don't want a hard non-zero-line count, since
            // even below-threshold lines do add up towards band spectral power.
            // So, fall steeply towards zero, but smoothly
            nzl += if s >= thresh {
                1.
            } else if nzslope == 2. {
                (s / thresh).powi(2)
            } else {
                ff_fast_powf(s / thresh, nzslope)
            };
        }
        if e2 > thresh {
            e *= iswb_size;

            // compute variance
            let var = scaled_window[..swb_size.into()]
                .iter()
                .map(|s| (s.abs() - e).powi(2))
                .sum::<c_float>();
            let var = (var * iswb_sizem1).sqrt();

            e2 *= iswb_size;
            let frm = e / c_float::min(e + 4. * var, maxval);
            form += e2 * frm.sqrt() / c_float::max(0.5, nzl);
            weight += e2;
        }
    }
    if weight > 0. {
        form / weight
    } else {
        1.
    }
}

pub(super) fn sfdelta_encoding_range(sf: c_int) -> RangeInclusive<c_int> {
    sf - c_int::from(SCALE_MAX_DIFF)..=sf + c_int::from(SCALE_MAX_DIFF)
}

/// Checks whether the specified band could be removed without inducing
/// scalefactor delta that violates SF delta encoding constraints.
/// prev_sf has to be the scalefactor of the previous nonzero, nonspecial
/// band, in encoding order, or negative if there was no such band.
#[ffmpeg_src(file = "libavcodec/aacenc_utils.h", lines = 226..=238, name = "ff_sfdelta_can_remove_band")]
#[inline]
pub(super) unsafe fn sfdelta_can_remove_band(
    mut sf_idx: &[c_int; 128],
    mut nextband: &[c_uchar; 128],
    mut prev_sf: c_int,
    mut band: c_int,
) -> bool {
    prev_sf >= 0
        && sfdelta_encoding_range(prev_sf).contains(&sf_idx[usize::from(nextband[band as usize])])
}

/// Checks whether the specified band's scalefactor could be replaced
/// with another one without violating SF delta encoding constraints.
/// prev_sf has to be the scalefactor of the previous nonzero, nonsepcial
/// band, in encoding order, or negative if there was no such band.
#[ffmpeg_src(file = "libavcodec/aacenc_utils.h", lines = 240..=253, name = "ff_sfdelta_can_replace")]
#[inline]
unsafe fn sfdelta_can_replace(
    mut sce: *const SingleChannelElement,
    mut nextband: &[c_uchar; 128],
    mut prev_sf: c_int,
    mut new_sf: c_int,
    mut band: c_int,
) -> bool {
    sfdelta_encoding_range(prev_sf).contains(&new_sf)
        && sfdelta_encoding_range(new_sf)
            .contains(&(*(*sce).sf_idx)[usize::from(nextband[band as usize])])
}

impl SingleChannelElement {
    /// Compute a nextband map to be used with SF delta constraint utilities.
    /// The nextband array should contain 128 elements, and positions that don't
    /// map to valid, nonzero bands of the form w*16+g (with w being the initial
    /// window of the window group, only) are left indetermined.
    #[ffmpeg_src(file = "libavcodec/aacenc_utils.h", lines = 169..=191, name = "ff_init_nextband_map")]
    pub(super) unsafe fn init_nextband_map(self: *const Self) -> [c_uchar; 128] {
        let Self {
            ics: ics @ IndividualChannelStream { num_swb, .. },
            ref zeroes,
            ref band_type,
            ..
        } = *self;
        let mut prevband: c_uchar = 0;
        let mut nextband = array::from_fn(|g| g as c_uchar);
        for WindowedIteration { w, .. } in ics.iter_windows() {
            let zeroes = &zeroes[W(w)][..num_swb as usize];
            let band_type = &band_type[W(w)][..num_swb as usize];
            for (g, _) in iter::zip(zeroes, band_type)
                .enumerate()
                .filter(|(_, (&zero, &band_type))| !zero && band_type < RESERVED_BT)
            {
                let fresh0 = &mut nextband[prevband as usize];
                *fresh0 = (w * 16 + g as c_int) as c_uchar;
                prevband = *fresh0;
            }
        }
        nextband[prevband as usize] = prevband;
        nextband
    }
}

impl QuantizeBandCostCache {
    #[inline]
    unsafe fn quantize_band_cost_cached(
        &mut self,
        w: c_int,
        g: c_int,
        in_: &[c_float],
        scaled: &[c_float],
        scale_idx: c_int,
        cb: c_int,
        lambda: c_float,
        uplim: c_float,
        bits: &mut c_int,
        energy: &mut c_float,
        rtz: c_int,
    ) -> c_float {
        let entry = &mut self.cache[scale_idx as usize][(w * 16 + g) as usize];
        if entry.generation != self.cache_generation
            || c_int::from(entry.cb) != cb
            || c_int::from(entry.rtz) != rtz
        {
            entry.rd = quantize_band_cost(
                in_,
                scaled,
                scale_idx,
                cb,
                lambda,
                uplim,
                Some(&mut entry.bits),
                Some(&mut entry.energy),
            );
            entry.cb = cb as c_char;
            entry.rtz = rtz as c_char;
            entry.generation = self.cache_generation;
        }
        *bits = entry.bits;
        *energy = entry.energy;
        entry.rd
    }
}

#[inline]
pub(super) unsafe fn quantize_band_cost(
    mut in_: &[c_float],
    mut scaled: &[c_float],
    mut scale_idx: c_int,
    mut cb: c_int,
    lambda: c_float,
    uplim: c_float,
    bits: Option<&mut c_int>,
    energy: Option<&mut c_float>,
) -> c_float {
    quantize_and_encode_band_cost(
        in_, None, scaled, scale_idx, cb, lambda, uplim, bits, energy,
    )
}

#[inline]
unsafe fn quantize_band_cost_bits(
    mut in_: &[c_float],
    mut scaled: &[c_float],
    mut scale_idx: c_int,
    mut cb: c_int,
    uplim: c_float,
) -> c_int {
    let mut auxbits: c_int = 0;
    quantize_and_encode_band_cost(
        in_,
        None,
        scaled,
        scale_idx,
        cb,
        0.,
        uplim,
        Some(&mut auxbits),
        None,
    );
    auxbits
}

pub(crate) unsafe fn set_special_band_scalefactors(mut sce: *mut SingleChannelElement) {
    let mut g: c_int = 0;
    let mut prevscaler_n: c_int = -255;
    let mut prevscaler_i: c_int = 0;
    let mut bands: c_int = 0;
    for WindowedIteration { w, .. } in (*sce).ics.iter_windows() {
        g = 0;
        while g < (*sce).ics.num_swb {
            if !(*sce).zeroes[W(w)][g as usize] {
                if (*sce).band_type[W(w)][g as usize] as c_uint == INTENSITY_BT as c_int as c_uint
                    || (*sce).band_type[W(w)][g as usize] as c_uint
                        == INTENSITY_BT2 as c_int as c_uint
                {
                    (*sce).sf_idx[W(w)][g as usize] = av_clip_c(
                        roundf(log2f((*sce).is_ener[W(w)][g as usize]) * 2.) as c_int,
                        -155,
                        100,
                    );
                    bands += 1;
                    bands;
                } else if (*sce).band_type[W(w)][g as usize] as c_uint
                    == NOISE_BT as c_int as c_uint
                {
                    (*sce).sf_idx[W(w)][g as usize] = av_clip_c(
                        (3. + ceilf(log2f((*sce).pns_ener[W(w)][g as usize]) * 2.)) as c_int,
                        -100,
                        155,
                    );
                    if prevscaler_n == -255 {
                        prevscaler_n = (*sce).sf_idx[W(w)][g as usize];
                    }
                    bands += 1;
                    bands;
                }
            }
            g += 1;
            g;
        }
    }
    if bands == 0 {
        return;
    }
    for WindowedIteration { w, .. } in (*sce).ics.iter_windows() {
        g = 0;
        while g < (*sce).ics.num_swb {
            if !(*sce).zeroes[W(w)][g as usize] {
                if (*sce).band_type[W(w)][g as usize] as c_uint == INTENSITY_BT as c_int as c_uint
                    || (*sce).band_type[W(w)][g as usize] as c_uint
                        == INTENSITY_BT2 as c_int as c_uint
                {
                    prevscaler_i = av_clip_c(
                        (*sce).sf_idx[W(w)][g as usize],
                        prevscaler_i - 60,
                        prevscaler_i + 60,
                    );
                    (*sce).sf_idx[W(w)][g as usize] = prevscaler_i;
                } else if (*sce).band_type[W(w)][g as usize] as c_uint
                    == NOISE_BT as c_int as c_uint
                {
                    prevscaler_n = av_clip_c(
                        (*sce).sf_idx[W(w)][g as usize],
                        prevscaler_n - 60,
                        prevscaler_n + 60,
                    );
                    (*sce).sf_idx[W(w)][g as usize] = prevscaler_n;
                }
            }
            g += 1;
            g;
        }
    }
}
