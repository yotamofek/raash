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

use std::{array, cell::Cell, iter::zip, ops::RangeInclusive};

use array_util::W;
use ffmpeg_src_macro::ffmpeg_src;
use izip::izip;
use libc::{c_char, c_float, c_int, c_uchar};

use self::{math::Float as _, quantize_and_encode_band::quantize_and_encode_band_cost};
use super::{
    encoder::ctx::QuantizeBandCostCache, tables::*, IndividualChannelStream, WindowedIteration,
    SCALE_MAX_DIFF,
};
use crate::types::*;

fn run_value_bits(num_windows: c_int) -> &'static [c_uchar] {
    static LONG: [c_uchar; 64] = [
        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        5, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
        10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 15,
    ];
    static SHORT: [c_uchar; 16] = [3, 3, 3, 3, 3, 3, 3, 6, 6, 6, 6, 6, 6, 6, 6, 9];

    if num_windows == 8 {
        &SHORT
    } else {
        &LONG
    }
}

static CB_OUT_MAP: [c_uchar; 15] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15];
static CB_IN_MAP: [c_uchar; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0, 12, 13, 14];

#[ffmpeg_src(file = "libavcodec/aacenctab.h", lines = 119, name = "aac_cb_range")]
static CB_RANGE: [c_uchar; 12] = [0, 3, 3, 3, 3, 9, 9, 8, 8, 13, 13, 17];

static CB_MAXVAL: [c_uchar; 12] = [0, 1, 1, 2, 2, 4, 4, 7, 7, 12, 12, 16];

static MAXVAL_CB: [c_uchar; 14] = [0, 1, 3, 5, 5, 7, 7, 7, 9, 9, 9, 9, 9, 11];

#[inline]
fn quant(coef: c_float, Q: c_float, rounding: c_float) -> c_int {
    let a = coef * Q;
    ((a * a.sqrt()).sqrt() + rounding) as c_int
}

#[inline]
fn find_min_book(maxval: c_float, sf: c_int) -> c_int {
    let Q34: c_float = POW_SF_TABLES.pow34()[(200 - sf + 140 - 36) as usize];
    let qmaxval = (maxval * Q34 + 0.4054) as c_int;
    MAXVAL_CB.get(qmaxval as usize).copied().unwrap_or(11) as c_int
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
                (s / thresh).fast_powf(nzslope)
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
pub(super) fn sfdelta_can_remove_band(
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
fn sfdelta_can_replace(
    sf_indices: &[Cell<c_int>; 128],
    prev_sf: c_int,
    new_sf: c_int,
    nextband: c_uchar,
) -> bool {
    sfdelta_encoding_range(prev_sf).contains(&new_sf)
        && sfdelta_encoding_range(new_sf).contains(&sf_indices[usize::from(nextband)].get())
}

impl SingleChannelElement {
    /// Compute a nextband map to be used with SF delta constraint utilities.
    /// The nextband array should contain 128 elements, and positions that don't
    /// map to valid, nonzero bands of the form w*16+g (with w being the initial
    /// window of the window group, only) are left indetermined.
    #[ffmpeg_src(file = "libavcodec/aacenc_utils.h", lines = 169..=191, name = "ff_init_nextband_map")]
    pub(super) fn init_next_band_map(&self) -> [c_uchar; 128] {
        let Self {
            ics: ics @ IndividualChannelStream { num_swb, .. },
            ref zeroes,
            ref band_type,
            ..
        } = *self;

        let mut prev_band = 0;
        let mut next_band = array::from_fn(|g| g as c_uchar);

        for WindowedIteration { w, .. } in ics.iter_windows() {
            let zeroes = &zeroes[W(w)][..num_swb as usize];
            let band_type = &band_type[W(w)][..num_swb as usize];
            for (g, _) in zip(zeroes, band_type)
                .enumerate()
                .filter(|(_, (&zero, &band_type))| !zero && band_type < RESERVED_BT)
            {
                let next_band = &mut next_band[usize::from(prev_band)];
                *next_band = (w * 16 + g as c_int) as c_uchar;
                prev_band = *next_band;
            }
        }

        next_band[usize::from(prev_band)] = prev_band;
        next_band
    }
}

impl QuantizeBandCostCache {
    #[inline]
    fn quantize_band_cost_cached(
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
pub(super) fn quantize_band_cost(
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
fn quantize_band_cost_bits(
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

#[ffmpeg_src(file = "libavcodec/aaccoder.c", lines = 419..=456)]
pub(crate) fn set_special_band_scalefactors(sce: &mut SingleChannelElement) {
    let mut prevscaler_n: c_int = -255;

    let SingleChannelElement {
        ics: ref ics @ IndividualChannelStream { mut num_swb, .. },
        ref zeroes,
        ref band_type,
        sf_idx,
        ref is_ener,
        ref pns_ener,
        ..
    } = sce;

    let found = {
        let sf_idx = sf_idx.as_array_of_cells_deref();
        ics.iter_windows()
            .flat_map(|WindowedIteration { w, .. }| {
                izip!(
                    &zeroes[W(w)],
                    &band_type[W(w)],
                    &is_ener[W(w)],
                    &pns_ener[W(w)],
                    &sf_idx[W(w)]
                )
                .take(num_swb as usize)
            })
            .filter(|&(zero, ..)| !zero)
            .fold(
                false,
                |found, (_, &band_type, &is_ener, &pns_ener, sf_idx)| {
                    sf_idx.set(match band_type {
                        INTENSITY_BT | INTENSITY_BT2 => {
                            ((is_ener.log2() * 2.).round() as c_int).clamp(-155, 100)
                        }
                        NOISE_BT => {
                            let sf_idx =
                                ((3. + (pns_ener.log2() * 2.).ceil()) as c_int).clamp(-100, 155);
                            if prevscaler_n == -255 {
                                prevscaler_n = sf_idx;
                            }
                            sf_idx
                        }
                        _ => return found,
                    });
                    true
                },
            )
    };

    if !found {
        return;
    }

    let mut prevscaler_i: c_int = 0;
    for WindowedIteration { w, .. } in ics.iter_windows() {
        for (_, &band_type, sf_idx) in izip!(&zeroes[W(w)], &band_type[W(w)], &mut sf_idx[W(w)])
            .take(num_swb as usize)
            .filter(|&(zero, ..)| !zero)
        {
            if let Some(prevscaler) = match band_type {
                INTENSITY_BT | INTENSITY_BT2 => Some(&mut prevscaler_i),
                NOISE_BT => Some(&mut prevscaler_n),
                _ => None,
            } {
                *prevscaler = (*sf_idx).clamp(*prevscaler - 60, *prevscaler + 60);
                *sf_idx = *prevscaler;
            }
        }
    }
}
