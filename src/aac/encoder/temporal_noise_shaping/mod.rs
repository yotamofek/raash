#![allow(
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

mod tables;

use std::{
    mem::size_of,
    ops::{Neg, RangeInclusive},
    ptr::addr_of_mut,
};

use array_util::W;
use ffmpeg_src_macro::ffmpeg_src;
use izip::izip;
use libc::{c_double, c_float, c_int, c_long, c_ulong};

use self::tables::{tns_min_sfb, tns_tmp2_map};
use crate::{
    aac::{encoder::ctx::AACEncContext, IndividualChannelStream, WindowSequence},
    common::*,
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

#[ffmpeg_src(file = "libavcodec/aac.h", lines = 193..=204)]
#[derive(Copy, Clone, Default)]
pub(crate) struct TemporalNoiseShaping {
    pub(super) present: c_int,
    pub(super) n_filt: [c_int; 8],
    pub(super) length: [[c_int; 4]; 8],
    pub(super) direction: [[c_int; 4]; 8],
    pub(super) order: [[c_int; 4]; 8],
    pub(super) coef_idx: [[[c_int; MAX_ORDER]; 4]; 8],
    pub(super) coef: [[[c_float; MAX_ORDER]; 4]; 8],
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

/// Levinson-Durbin recursion.
/// Produce LPC coefficients from autocorrelation data.
#[ffmpeg_src(file = "libavcodec/lpc.h", lines = 163..=212)]
#[inline]
fn compute_lpc_coefs(autoc: &[LPC_TYPE; MAX_ORDER], max_order: c_int) -> [LPC_TYPE; MAX_ORDER] {
    let mut lpc = [0.; MAX_ORDER];

    for (i, mut r) in autoc
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
fn quant_array_idx(val: c_float, mut arr: &[c_float], num: c_int) -> c_int {
    let mut i: c_int = 0;
    let mut index: c_int = 0;
    let mut quant_min_err: c_float = f32::INFINITY;
    i = 0;
    while i < num {
        let mut error: c_float = (val - arr[i as usize]) * (val - arr[i as usize]);
        if error < quant_min_err {
            quant_min_err = error;
            index = i;
        }
        i += 1;
        i;
    }
    index
}

enum Compressed {
    Yes,
    No,
}

#[inline]
fn compress_coeffs(mut coef: &mut [c_int], mut c_bits: c_int) -> Compressed {
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
pub(super) unsafe fn encode_info(s: &mut AACEncContext, sce: &mut SingleChannelElement) {
    let pb = addr_of_mut!(s.pb);
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

    if tns.present == 0 {
        return;
    }

    for (&n_filt, length, order, direction, coef_idx) in izip!(
        &tns.n_filt,
        &tns.length,
        &tns.order,
        &tns.direction,
        &mut tns.coef_idx
    )
    .take(num_windows as usize)
    {
        put_bits(pb, 2 - i32::from(is8), n_filt as BitBuf);

        if n_filt == 0 {
            continue;
        }

        put_bits(pb, 1, c_bits as BitBuf);
        for (&length, &order, &direction, coef_idx) in
            izip!(length, order, direction, coef_idx).take(n_filt as usize)
        {
            put_bits(pb, 6 - 2 * i32::from(is8), length as BitBuf);
            put_bits(pb, 5 - 2 * i32::from(is8), order as BitBuf);

            let coef_idx = match &mut coef_idx[..order as usize] {
                [] => continue,
                coef_idx => coef_idx,
            };

            put_bits(pb, 1, direction as BitBuf);
            let coef_compress = match compress_coeffs(coef_idx, c_bits) {
                Compressed::Yes => 1,
                Compressed::No => 0,
            };
            put_bits(pb, 1, coef_compress as BitBuf);
            let coef_len = c_bits + 3 - coef_compress;
            for &coef_idx in &*coef_idx {
                put_bits(pb, coef_len, coef_idx as BitBuf);
            }
        }
    }
}

/// Apply TNS filter
#[ffmpeg_src(file = "libavcodec/aacenc_tns.c", lines = 100..=141, name = "ff_aac_apply_tns")]
pub(crate) fn apply(sce: &mut SingleChannelElement) {
    let mut tns = &mut sce.tns;
    let mut ics = &mut sce.ics;
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
    let mmm = ics.tns_max_bands.min(ics.max_sfb as c_int);
    w = 0;
    while w < ics.num_windows {
        bottom = ics.num_swb;
        filt = 0;
        while filt < tns.n_filt[w as usize] {
            top = bottom;
            bottom = 0.max(top - tns.length[w as usize][filt as usize]);
            order = tns.order[w as usize][filt as usize];
            if order != 0 {
                let lpc = compute_lpc_coefs(&tns.coef[w as usize][filt as usize], order);
                start = ics.swb_offset[bottom.min(mmm) as usize] as c_int;
                end = ics.swb_offset[top.min(mmm) as usize] as c_int;
                size = end - start;
                if size > 0 {
                    if tns.direction[w as usize][filt as usize] != 0 {
                        inc = -1;
                        start = end - 1;
                    } else {
                        inc = 1;
                    }
                    start += w * 128;
                    m = 0;
                    while m < size {
                        i = 1;
                        while i <= m.min(order) {
                            (*sce.coeffs)[start as usize] +=
                                lpc[(i - 1) as usize] * (*sce.pcoeffs)[(start - i * inc) as usize];
                            i += 1;
                        }
                        m += 1;
                        start += inc;
                    }
                }
            }
            filt += 1;
        }
        w += 1;
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
    let mut quant_arr = tns_tmp2_map[c_bits as usize];
    i = 0;
    while i < order {
        *idx.offset(i as isize) = quant_array_idx(
            *coef.offset(i as isize) as c_float,
            quant_arr,
            if c_bits != 0 { 16 } else { 8 },
        );
        *lpc.offset(i as isize) = quant_arr[*idx.offset(i as isize) as usize];
        i += 1;
        i;
    }
}

/// 3 bits per coefficient with 8 short windows
#[ffmpeg_src(file = "libavcodec/aacenc_tns.c", lines = 157..=214, name = "ff_aac_search_for_tns")]
pub(crate) unsafe fn search(s: &mut AACEncContext, sce: &mut SingleChannelElement) {
    let mut tns: *mut TemporalNoiseShaping = &mut sce.tns;
    let mut w: c_int = 0;
    let mut g: c_int = 0;
    let mut count: c_int = 0;
    let mut gain: c_double = 0.;
    let mut coefs: [c_double; 32] = [0.; 32];
    let mmm: c_int = sce.ics.tns_max_bands.min(sce.ics.max_sfb as c_int);
    let is8 = sce.ics.window_sequence[0] == WindowSequence::EightShort;
    let c_bits = c_int::from(if is8 { Q_BITS_IS8 == 4 } else { Q_BITS == 4 });
    let sfb_start: c_int = av_clip_c(
        tns_min_sfb[is8 as usize][s.samplerate_index as usize] as c_int,
        0,
        mmm,
    );
    let sfb_end: c_int = av_clip_c(sce.ics.num_swb, 0, mmm);
    let order = if is8 {
        7
    } else if s.profile == 1 {
        12
    } else {
        MAX_ORDER as c_int
    };
    let slant: c_int = if sce.ics.window_sequence[0] == WindowSequence::LongStop {
        1
    } else if sce.ics.window_sequence[0] == WindowSequence::LongStart {
        0
    } else {
        2
    };
    let sfb_len: c_int = sfb_end - sfb_start;
    let coef_len: c_int = sce.ics.swb_offset[sfb_end as usize] as c_int
        - sce.ics.swb_offset[sfb_start as usize] as c_int;
    if coef_len <= 0 || sfb_len <= 0 {
        sce.tns.present = 0;
        return;
    }
    w = 0;
    while w < sce.ics.num_windows {
        let mut en: [c_float; 2] = [0., 0.];
        let mut oc_start: c_int = 0;
        let mut os_start: c_int = 0;
        let mut coef_start: c_int = sce.ics.swb_offset[sfb_start as usize] as c_int;
        g = sfb_start;
        while g < sce.ics.num_swb && g <= sfb_end {
            let band = &s.psy.ch[s.cur_channel as usize].psy_bands[W(w)][g as usize];
            if g > sfb_start + sfb_len / 2 {
                en[1] += band.energy;
            } else {
                en[0] += band.energy;
            }
            g += 1;
            g;
        }

        gain = s.lpc.calc_ref_coefs_f(
            &sce.coeffs[W(w)][coef_start as usize..][..coef_len as usize],
            order,
            &mut coefs,
        );

        if !(order == 0 || !gain.is_finite() || !GAIN_THRESHOLD.contains(&gain)) {
            (*tns).n_filt[w as usize] = if is8 {
                1
            } else if order != MAX_ORDER as c_int {
                2
            } else {
                3
            };
            g = 0;
            while g < (*tns).n_filt[w as usize] {
                (*tns).direction[w as usize][g as usize] = if slant != 2 {
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
    sce.tns.present = (count != 0) as c_int;
}

unsafe fn run_static_initializers() {
    BUF_BITS = (8 as c_ulong).wrapping_mul(size_of::<BitBuf>() as c_ulong) as c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe fn(); 1] = [run_static_initializers];
