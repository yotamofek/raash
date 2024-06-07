mod tables;

use std::ops::{AddAssign, Mul, Neg, RangeInclusive};

use bit_writer::{BitBuf, BitWriter};
use ffmpeg_src_macro::ffmpeg_src;
use itertools::Itertools as _;
use izip::izip;
use libc::{c_double, c_float, c_int, c_uchar};
use lpc::RefCoeffs;

use self::tables::{tns_min_sfb, tns_tmp2_map};
use crate::{
    aac::{encoder::ctx::AACEncContext, IndividualChannelStream, WindowSequence},
    types::*,
};

#[ffmpeg_src(file = "libavcodec/aac.h", lines = 49, name = "TNS_MAX_ORDER")]
const MAX_ORDER: usize = 20;

/// Could be set to 3 to save an additional bit at the cost of little quality
#[ffmpeg_src(file = "libavcodec/aacenc_tns.c", lines = 35, name = "TNS_Q_BITS")]
const Q_BITS: u8 = 4;

/// Coefficient resolution in short windows
#[ffmpeg_src(file = "libavcodec/aacenc_tns.c", lines = 38, name = "TNS_Q_BITS_IS8")]
const Q_BITS_IS8: u8 = 4;

/// TNS will only be used if the LPC gain is within these margins
#[ffmpeg_src(file = "libavcodec/aacenc_tns.c", lines = 43..=45)]
#[doc(alias = "TNS_GAIN_THRESHOLD_LOW")]
#[doc(alias = "TNS_GAIN_THRESHOLD_HIGH")]
const GAIN_THRESHOLD: RangeInclusive<f64> = {
    const LOW: f64 = 1.4;
    LOW..=1.16 * LOW
};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum Direction {
    #[default]
    Forward,
    Backward,
}

impl Mul<Direction> for c_int {
    type Output = c_int;

    fn mul(self, rhs: Direction) -> Self::Output {
        self * match rhs {
            Direction::Forward => 1,
            Direction::Backward => -1,
        }
    }
}

impl AddAssign<Direction> for c_int {
    fn add_assign(&mut self, rhs: Direction) {
        *self += 1 * rhs;
    }
}

#[ffmpeg_src(file = "libavcodec/aac.h", lines = 193..=204)]
#[derive(Copy, Clone, Default)]
pub(crate) struct TemporalNoiseShaping {
    pub(super) present: bool,
    n_filt: [c_int; 8],
    length: [[c_int; 4]; 8],
    direction: [[Direction; 4]; 8],
    order: [[c_int; 4]; 8],
    coef_idx: [[[c_int; MAX_ORDER]; 4]; 8],
    coef: [[[c_float; MAX_ORDER]; 4]; 8],
}

/// Levinson-Durbin recursion.
/// Produce LPC coefficients from autocorrelation data.
#[ffmpeg_src(file = "libavcodec/lpc.h", lines = 163..=212)]
#[inline]
fn compute_lpc_coefs(autoc: &[c_float; MAX_ORDER], max_order: c_int) -> [c_float; MAX_ORDER] {
    let mut lpc = [0.; MAX_ORDER];

    for (i, r) in autoc
        .iter()
        .map(Neg::neg)
        .take(max_order as usize)
        .enumerate()
    {
        lpc[i] = r;

        let (mut front, mut back) = lpc[..i].split_at_mut((i + 1) / 2);
        while let Some(f) = front.take_first_mut() {
            if let Some(b) = back.take_last_mut() {
                (*f, *b) = (*f + r * *b, *b + r * *f);
            } else {
                *f = *f + r * *f;
            }
        }
    }

    lpc
}

#[inline]
fn quant_array_idx(val: c_float, arr: &[c_float]) -> c_int {
    arr.iter()
        .map(|&arr_val| (val - arr_val).powi(2))
        .position_min_by(c_float::total_cmp)
        .unwrap_or_default() as c_int
}

enum Compressed {
    Yes,
    No,
}

#[inline]
fn compress_coeffs(coef: &mut [c_int], c_bits: c_int) -> Compressed {
    let low_idx: c_int = if c_bits != 0 { 4 } else { 2 };
    let shift_val: c_int = if c_bits != 0 { 8 } else { 4 };
    let high_idx: c_int = if c_bits != 0 { 11 } else { 5 };

    if coef.iter().any(|coef| (low_idx..=high_idx).contains(coef)) {
        return Compressed::No;
    }

    for coef in coef.iter_mut().filter(|&&mut coef| coef > high_idx) {
        *coef -= shift_val;
    }

    Compressed::Yes
}

/// Encode TNS data.
///
/// Coefficient compression is simply not lossless as it should be
/// on any decoder tested and as such is not active.
#[ffmpeg_src(file = "libavcodec/aacenc_tns.c", lines = 64..=98, name = "ff_aac_encode_tns_info")]
pub(super) fn encode_info(pb: &mut BitWriter, sce: &mut SingleChannelElement) {
    let SingleChannelElement {
        ref mut tns,
        ics:
            IndividualChannelStream {
                window_sequence: [window_sequence, _],
                num_windows,
                ..
            },
        ..
    } = *sce;

    let is8 = window_sequence == WindowSequence::EightShort;
    let c_bits = c_int::from(if is8 { Q_BITS_IS8 == 4 } else { Q_BITS == 4 });

    if !tns.present {
        return;
    }

    for (&n_filt, length, order, direction, coef_idx) in izip!(
        &tns.n_filt,
        &tns.length,
        &tns.order,
        &tns.direction,
        &mut tns.coef_idx
    )
    .take(c_uchar::from(num_windows).into())
    {
        pb.put(2 - u8::from(is8), n_filt as BitBuf);

        if n_filt == 0 {
            continue;
        }

        pb.put(1, c_bits as BitBuf);
        for (&length, &order, &direction, coef_idx) in
            izip!(length, order, direction, coef_idx).take(n_filt as usize)
        {
            pb.put(6 - 2 * u8::from(is8), length as BitBuf);
            pb.put(5 - 2 * u8::from(is8), order as BitBuf);

            let coef_idx = match &mut coef_idx[..order as usize] {
                [] => continue,
                coef_idx => coef_idx,
            };

            pb.put(1, direction as BitBuf);
            let coef_compress = match compress_coeffs(coef_idx, c_bits) {
                Compressed::Yes => 1,
                Compressed::No => 0,
            };
            pb.put(1, coef_compress as BitBuf);
            let coef_len = c_bits + 3 - coef_compress;
            for &coef_idx in &*coef_idx {
                pb.put(coef_len as u8, coef_idx as BitBuf);
            }
        }
    }
}

/// Apply TNS filter
#[ffmpeg_src(file = "libavcodec/aacenc_tns.c", lines = 100..=141, name = "ff_aac_apply_tns")]
pub(crate) fn apply(sce: &mut SingleChannelElement) {
    let tns = &mut sce.tns;
    let ics = &mut sce.ics;
    let mmm = ics.tns_max_bands.min(ics.max_sfb as c_int);
    for (w, (&n_filt, lengths, orders, coeffs, directions)) in izip!(
        &tns.n_filt,
        &tns.length,
        &tns.order,
        &tns.coef,
        &tns.direction
    )
    .enumerate()
    {
        let mut bottom = ics.num_swb;
        for (&length, &order, coeffs, &direction) in
            izip!(lengths, orders, coeffs, directions).take(n_filt as usize)
        {
            let top = bottom;
            bottom = 0.max(top - length);

            if order == 0 {
                continue;
            }

            let lpc = compute_lpc_coefs(coeffs, order);
            let mut start = c_int::from(ics.swb_offset[bottom.min(mmm) as usize]);
            let end = c_int::from(ics.swb_offset[top.min(mmm) as usize]);
            let size = end - start;

            if size <= 0 {
                continue;
            }

            if direction == Direction::Backward {
                start = end - 1;
            }

            start += w as c_int * 128;
            for m in 0..size {
                (*sce.coeffs)[start as usize] += lpc
                    .iter()
                    .take(m.min(order) as usize)
                    .enumerate()
                    .map(|(i, &lpc)| {
                        lpc * (*sce.pcoeffs)[(start - (i + 1) as c_int * direction) as usize]
                    })
                    .sum::<c_float>();
                start += direction;
            }
        }
    }
}

#[inline]
fn quantize_coefs(
    coef: &[c_double],
    idx: &mut [c_int],
    lpc: &mut [c_float],
    order: c_int,
    c_bits: bool,
) {
    let quant_arr = tns_tmp2_map[usize::from(c_bits)];
    for (idx, lpc, &coef) in izip!(idx, lpc, coef).take(order as usize) {
        *idx = quant_array_idx(coef as c_float, quant_arr);
        *lpc = quant_arr[*idx as usize];
    }
}

/// 3 bits per coefficient with 8 short windows
#[ffmpeg_src(file = "libavcodec/aacenc_tns.c", lines = 157..=214, name = "ff_aac_search_for_tns")]
pub(crate) fn search(s: &mut AACEncContext, sce: &mut SingleChannelElement) {
    let tns = &mut sce.tns;
    let mmm = sce.ics.tns_max_bands.min(sce.ics.max_sfb.into());
    let is8 = sce.ics.window_sequence[0] == WindowSequence::EightShort;
    let c_bits = if is8 { Q_BITS_IS8 == 4 } else { Q_BITS == 4 };
    let sfb_start =
        (tns_min_sfb[usize::from(is8)][s.samplerate_index as usize] as c_int).clamp(0, mmm);
    let sfb_end = sce.ics.num_swb.clamp(0, mmm);
    let order = if is8 {
        7
    } else if s.profile == 1 {
        12
    } else {
        MAX_ORDER as c_int
    };
    let slant = match sce.ics.window_sequence[0] {
        WindowSequence::LongStop => Some(Direction::Backward),
        WindowSequence::LongStart => Some(Direction::Forward),
        _ => None,
    };
    let sfb_len = sfb_end - sfb_start;
    let coef_len = c_int::from(sce.ics.swb_offset[sfb_end as usize])
        - c_int::from(sce.ics.swb_offset[sfb_start as usize]);
    if coef_len <= 0 || sfb_len <= 0 {
        sce.tns.present = false;
        return;
    }
    for (psy_bands, coeffs, n_filt, directions, orders, lengths, tns_coef_idx, tns_coef) in izip!(
        &s.psy.ch[s.cur_channel as usize].psy_bands,
        &sce.coeffs,
        &mut tns.n_filt,
        &mut tns.direction,
        &mut tns.order,
        &mut tns.length,
        &mut tns.coef_idx,
        &mut tns.coef,
    )
    .take(c_uchar::from(sce.ics.num_windows).into())
    {
        let mut oc_start = 0;
        let coef_start = sce.ics.swb_offset[sfb_start as usize];
        let [en0, en1]: [c_float; _] = {
            let mut ens = psy_bands
                .iter()
                .skip(sfb_start as usize)
                .take(sfb_len as usize)
                .take((sce.ics.num_swb - sfb_start) as usize)
                .map(|&FFPsyBand { energy, .. }| energy);
            [ens.by_ref().take(sfb_len as usize / 2 + 1).sum(), ens.sum()]
        };

        let RefCoeffs { gain, coeffs } = s.lpc.calc_ref_coefs_f(
            &coeffs[usize::from(coef_start)..][..coef_len as usize],
            order,
        );

        if order == 0 || !gain.is_finite() || !GAIN_THRESHOLD.contains(&gain) {
            continue;
        }

        *n_filt = if is8 {
            1
        } else if order != MAX_ORDER as c_int {
            2
        } else {
            3
        };
        for (direction, cur_order, length, tns_coef_idx, tns_coef, [en0, en1]) in izip!(
            directions,
            orders,
            lengths,
            tns_coef_idx,
            tns_coef,
            [[en0, en1], [en1, en0]]
        )
        .take(*n_filt as usize)
        {
            *direction = slant.unwrap_or_else(|| {
                (en0 < en1)
                    .then_some(Direction::Backward)
                    .unwrap_or_default()
            });
            *cur_order = order / *n_filt;
            *length = sfb_len / *n_filt;
            quantize_coefs(
                &coeffs[oc_start as usize..],
                tns_coef_idx,
                tns_coef,
                *cur_order,
                c_bits,
            );
            oc_start += *cur_order;
        }
        tns.present = true;
    }
}
