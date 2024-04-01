use std::{iter::zip, ptr};

use ffmpeg_src_macro::ffmpeg_src;
use libc::{c_float, c_int, c_uint};

use super::{
    aac_cb_maxval,
    math::{clip_uintp2_c, ff_log2_c},
    put_bits, put_sbits, quant, CB_RANGE,
};
use crate::{
    aac::{
        encoder::{ctx::AACEncContext, pow::Pow34, quantize_bands},
        tables::{
            ff_aac_codebook_vectors, ff_aac_spectral_bits, ff_aac_spectral_codes, POW_SF_TABLES,
        },
        POW_SF2_ZERO, SCALE_DIV_512, SCALE_ONE_POS,
    },
    common::*,
    types::*,
};

type QuantizeAndEncodeBandFunc = unsafe fn(
    *mut AACEncContext,
    *mut PutBitContext,
    &[c_float],
    Option<&mut [c_float]>,
    Option<&[c_float]>,
    c_int,
    c_int,
    c_float,
    c_float,
    Option<&mut c_int>,
    Option<&mut c_float>,
) -> c_float;

/// Calculate rate distortion cost for quantizing with given codebook
///
/// Returns quantization distortion
#[ffmpeg_src(file = "libavcodec/aaccoder.c", lines = 71..=194, name = "quantize_and_encode_band_cost_template")]
#[inline(always)]
unsafe fn cost_template(
    mut s: *mut AACEncContext,
    mut pb: *mut PutBitContext,
    mut in_: &[c_float],
    mut out: Option<&mut [c_float]>,
    mut scaled: Option<&[c_float]>,
    mut scale_idx: c_int,
    mut cb: c_int,
    lambda: c_float,
    uplim: c_float,
    bits: Option<&mut c_int>,
    energy: Option<&mut c_float>,
    mut BT_ZERO: c_int,
    mut BT_UNSIGNED: c_int,
    mut BT_PAIR: c_int,
    mut BT_ESC: c_int,
    mut BT_NOISE: c_int,
    mut BT_STEREO: c_int,
    ROUNDING: c_float,
) -> c_float {
    let q_idx = c_int::from(POW_SF2_ZERO) - scale_idx + c_int::from(SCALE_ONE_POS)
        - c_int::from(SCALE_DIV_512);
    let Q: c_float = POW_SF_TABLES.pow2()[q_idx as usize];
    let Q34: c_float = POW_SF_TABLES.pow34()[q_idx as usize];
    let IQ: c_float = POW_SF_TABLES.pow2()[(c_int::from(POW_SF2_ZERO) + scale_idx
        - c_int::from(SCALE_ONE_POS)
        + c_int::from(SCALE_DIV_512)) as usize];
    let CLIPPED_ESCAPE: c_float = 165140. * IQ;
    let dim = if BT_PAIR != 0 { 2 } else { 4 };
    let mut cost: c_float = 0.;
    let mut qenergy: c_float = 0.;
    let mut resbits: c_int = 0;
    if BT_ZERO != 0 || BT_NOISE != 0 || BT_STEREO != 0 {
        let cost = in_.iter().map(|in_| in_.powi(2)).sum::<c_float>();
        if let Some(bits) = bits {
            *bits = 0;
        }
        if let Some(energy) = energy {
            *energy = qenergy;
        }
        if let Some(out) = out {
            out.chunks_mut(dim as usize).for_each(|out| out.fill(0.));
        }
        return cost * lambda;
    }
    let scaled = scaled.get_or_insert_with(|| {
        let scoefs = &mut (*s).scoefs[..in_.len()];
        for (scoef, in_) in zip(&mut *scoefs, in_) {
            *scoef = in_.abs_pow34();
        }
        &*scoefs
    });
    quantize_bands(
        &mut ((*s).qcoefs)[..in_.len()],
        in_,
        scaled,
        BT_UNSIGNED == 0,
        aac_cb_maxval[cb as usize] as c_int,
        Q34,
        ROUNDING,
    );
    let off = if BT_UNSIGNED != 0 {
        0
    } else {
        aac_cb_maxval[cb as usize] as c_int
    };
    for i in (0..in_.len() as c_int).step_by(dim as usize) {
        let curidx = (*s).qcoefs[i as usize..][..dim as usize]
            .iter()
            .fold(0, |acc, quant| {
                acc * CB_RANGE[cb as usize] as c_int + quant + off
            });
        let mut rd: c_float = 0.;
        let mut curbits = ff_aac_spectral_bits[(cb - 1) as usize][curidx as usize] as c_int;
        let vec = &ff_aac_codebook_vectors[(cb - 1) as usize][(curidx * dim) as usize..];
        if BT_UNSIGNED != 0 {
            for j in 0..dim {
                let t = in_[(i + j) as usize].abs();
                let quantized = if BT_ESC != 0 && vec[j as usize] == 64. {
                    if t >= CLIPPED_ESCAPE {
                        curbits += 21;
                        CLIPPED_ESCAPE
                    } else {
                        let c = clip_uintp2_c(quant(t, Q, ROUNDING), 13) as c_int;
                        curbits += ff_log2_c(c as c_uint) * 2 - 4 + 1;
                        c as c_float * cbrtf(c as c_float) * IQ
                    }
                } else {
                    vec[j as usize] * IQ
                };
                let di = t - quantized;
                if let Some(out) = &mut out {
                    out[(i + j) as usize] = if in_[(i + j) as usize] >= 0. {
                        quantized
                    } else {
                        -quantized
                    };
                }
                if vec[j as usize] != 0. {
                    curbits += 1;
                }
                qenergy += quantized.powi(2);
                rd += di.powi(2);
            }
        } else {
            for j in 0..dim {
                let quantized = vec[j as usize] * IQ;
                qenergy += quantized.powi(2);
                if let Some(out) = &mut out {
                    out[(i + j) as usize] = quantized;
                }
                rd += (in_[(i + j) as usize] - quantized).powi(2);
            }
        }
        cost += rd * lambda + curbits as c_float;
        resbits += curbits;
        if cost >= uplim {
            return uplim;
        }
        if !pb.is_null() {
            put_bits(
                pb,
                ff_aac_spectral_bits[(cb - 1) as usize][curidx as usize] as c_int,
                ff_aac_spectral_codes[(cb - 1) as usize][curidx as usize] as BitBuf,
            );
            if BT_UNSIGNED != 0 {
                for j in 0..dim {
                    if ff_aac_codebook_vectors[(cb - 1) as usize][(curidx * dim + j) as usize] != 0.
                    {
                        put_bits(pb, 1, (in_[(i + j) as usize] < 0.) as c_int as BitBuf);
                    }
                }
            }
            if BT_ESC != 0 {
                for j in 0..2 {
                    if ff_aac_codebook_vectors[(cb - 1) as usize][(curidx * 2 + j) as usize] == 64.
                    {
                        let coef =
                            clip_uintp2_c(quant(fabsf(in_[(i + j) as usize]), Q, ROUNDING), 13)
                                as c_int;
                        let len: c_int = ff_log2_c(coef as c_uint);
                        put_bits(pb, len - 4 + 1, ((1 << (len - 4 + 1)) - 2) as BitBuf);
                        put_sbits(pb, len, coef);
                    }
                }
            }
        }
    }
    if let Some(bits) = bits {
        *bits = resbits;
    }
    if let Some(energy) = energy {
        *energy = qenergy;
    }
    cost
}

#[inline]
unsafe fn cost_NONE(
    mut _s: *mut AACEncContext,
    mut _pb: *mut PutBitContext,
    mut _in_0: &[c_float],
    mut _quant_0: Option<&mut [c_float]>,
    mut _scaled: Option<&[c_float]>,
    mut _scale_idx: c_int,
    mut _cb: c_int,
    _lambda: c_float,
    _uplim: c_float,
    _bits: Option<&mut c_int>,
    _energy: Option<&mut c_float>,
) -> c_float {
    0.
}
unsafe fn cost_ZERO(
    s: *mut AACEncContext,
    pb: *mut PutBitContext,
    in_0: &[c_float],
    quant_0: Option<&mut [c_float]>,
    scaled: Option<&[c_float]>,
    scale_idx: c_int,
    cb: c_int,
    lambda: c_float,
    uplim: c_float,
    bits: Option<&mut c_int>,
    energy: Option<&mut c_float>,
) -> c_float {
    cost_template(
        s, pb, in_0, quant_0, scaled, scale_idx, cb, lambda, uplim, bits, energy, 1, 0, 0, 0, 0, 0,
        0.4054,
    )
}

unsafe fn cost_SQUAD(
    s: *mut AACEncContext,
    pb: *mut PutBitContext,
    in_0: &[c_float],
    quant_0: Option<&mut [c_float]>,
    scaled: Option<&[c_float]>,
    scale_idx: c_int,
    cb: c_int,
    lambda: c_float,
    uplim: c_float,
    bits: Option<&mut c_int>,
    energy: Option<&mut c_float>,
) -> c_float {
    cost_template(
        s, pb, in_0, quant_0, scaled, scale_idx, cb, lambda, uplim, bits, energy, 0, 0, 0, 0, 0, 0,
        0.4054,
    )
}

unsafe fn cost_UQUAD(
    s: *mut AACEncContext,
    pb: *mut PutBitContext,
    in_0: &[c_float],
    quant_0: Option<&mut [c_float]>,
    scaled: Option<&[c_float]>,
    scale_idx: c_int,
    cb: c_int,
    lambda: c_float,
    uplim: c_float,
    bits: Option<&mut c_int>,
    energy: Option<&mut c_float>,
) -> c_float {
    cost_template(
        s, pb, in_0, quant_0, scaled, scale_idx, cb, lambda, uplim, bits, energy, 0, 1, 0, 0, 0, 0,
        0.4054,
    )
}

unsafe fn cost_SPAIR(
    s: *mut AACEncContext,
    pb: *mut PutBitContext,
    in_0: &[c_float],
    quant_0: Option<&mut [c_float]>,
    scaled: Option<&[c_float]>,
    scale_idx: c_int,
    cb: c_int,
    lambda: c_float,
    uplim: c_float,
    bits: Option<&mut c_int>,
    energy: Option<&mut c_float>,
) -> c_float {
    cost_template(
        s, pb, in_0, quant_0, scaled, scale_idx, cb, lambda, uplim, bits, energy, 0, 0, 1, 0, 0, 0,
        0.4054,
    )
}

unsafe fn cost_UPAIR(
    s: *mut AACEncContext,
    pb: *mut PutBitContext,
    in_0: &[c_float],
    quant_0: Option<&mut [c_float]>,
    scaled: Option<&[c_float]>,
    scale_idx: c_int,
    cb: c_int,
    lambda: c_float,
    uplim: c_float,
    bits: Option<&mut c_int>,
    energy: Option<&mut c_float>,
) -> c_float {
    cost_template(
        s, pb, in_0, quant_0, scaled, scale_idx, cb, lambda, uplim, bits, energy, 0, 1, 1, 0, 0, 0,
        0.4054,
    )
}

unsafe fn cost_ESC(
    s: *mut AACEncContext,
    pb: *mut PutBitContext,
    in_0: &[c_float],
    quant_0: Option<&mut [c_float]>,
    scaled: Option<&[c_float]>,
    scale_idx: c_int,
    _cb: c_int,
    lambda: c_float,
    uplim: c_float,
    bits: Option<&mut c_int>,
    energy: Option<&mut c_float>,
) -> c_float {
    cost_template(
        s,
        pb,
        in_0,
        quant_0,
        scaled,
        scale_idx,
        ESC_BT as c_int,
        lambda,
        uplim,
        bits,
        energy,
        0,
        1,
        1,
        1,
        0,
        0,
        0.4054,
    )
}

unsafe fn cost_ESC_RTZ(
    s: *mut AACEncContext,
    pb: *mut PutBitContext,
    in_0: &[c_float],
    quant_0: Option<&mut [c_float]>,
    scaled: Option<&[c_float]>,
    scale_idx: c_int,
    _cb: c_int,
    lambda: c_float,
    uplim: c_float,
    bits: Option<&mut c_int>,
    energy: Option<&mut c_float>,
) -> c_float {
    cost_template(
        s,
        pb,
        in_0,
        quant_0,
        scaled,
        scale_idx,
        ESC_BT as c_int,
        lambda,
        uplim,
        bits,
        energy,
        0,
        1,
        1,
        1,
        0,
        0,
        0.1054,
    )
}

unsafe fn cost_NOISE(
    s: *mut AACEncContext,
    pb: *mut PutBitContext,
    in_0: &[c_float],
    quant_0: Option<&mut [c_float]>,
    scaled: Option<&[c_float]>,
    scale_idx: c_int,
    cb: c_int,
    lambda: c_float,
    uplim: c_float,
    bits: Option<&mut c_int>,
    energy: Option<&mut c_float>,
) -> c_float {
    cost_template(
        s, pb, in_0, quant_0, scaled, scale_idx, cb, lambda, uplim, bits, energy, 0, 0, 0, 0, 1, 0,
        0.4054,
    )
}

unsafe fn cost_STEREO(
    s: *mut AACEncContext,
    pb: *mut PutBitContext,
    in_0: &[c_float],
    quant_0: Option<&mut [c_float]>,
    scaled: Option<&[c_float]>,
    scale_idx: c_int,
    cb: c_int,
    lambda: c_float,
    uplim: c_float,
    bits: Option<&mut c_int>,
    energy: Option<&mut c_float>,
) -> c_float {
    cost_template(
        s, pb, in_0, quant_0, scaled, scale_idx, cb, lambda, uplim, bits, energy, 0, 0, 0, 0, 0, 1,
        0.4054,
    )
}

const quantize_and_encode_band_cost_arr: [QuantizeAndEncodeBandFunc; 16] = [
    cost_ZERO,
    cost_SQUAD,
    cost_SQUAD,
    cost_UQUAD,
    cost_UQUAD,
    cost_SPAIR,
    cost_SPAIR,
    cost_UPAIR,
    cost_UPAIR,
    cost_UPAIR,
    cost_UPAIR,
    cost_ESC,
    cost_NONE,
    cost_NOISE,
    cost_STEREO,
    cost_STEREO,
];

const quantize_and_encode_band_cost_rtz_arr: [QuantizeAndEncodeBandFunc; 16] = [
    cost_ZERO,
    cost_SQUAD,
    cost_SQUAD,
    cost_UQUAD,
    cost_UQUAD,
    cost_SPAIR,
    cost_SPAIR,
    cost_UPAIR,
    cost_UPAIR,
    cost_UPAIR,
    cost_UPAIR,
    cost_ESC_RTZ,
    cost_NONE,
    cost_NOISE,
    cost_STEREO,
    cost_STEREO,
];

pub(crate) unsafe fn quantize_and_encode_band_cost(
    s: *mut AACEncContext,
    pb: *mut PutBitContext,
    in_0: &[c_float],
    quant_0: Option<&mut [c_float]>,
    scaled: Option<&[c_float]>,
    scale_idx: c_int,
    cb: c_int,
    lambda: c_float,
    uplim: c_float,
    bits: Option<&mut c_int>,
    energy: Option<&mut c_float>,
) -> c_float {
    (quantize_and_encode_band_cost_arr[cb as usize])(
        s, pb, in_0, quant_0, scaled, scale_idx, cb, lambda, uplim, bits, energy,
    )
}

#[inline]
pub(crate) unsafe fn quantize_and_encode_band(
    mut s: *mut AACEncContext,
    mut pb: *mut PutBitContext,
    mut in_0: &[c_float],
    mut out: Option<&mut [c_float]>,
    mut scale_idx: c_int,
    mut cb: c_int,
    lambda: c_float,
    mut rtz: c_int,
) {
    let arr = if rtz != 0 {
        &quantize_and_encode_band_cost_rtz_arr
    } else {
        &quantize_and_encode_band_cost_arr
    };
    arr[cb as usize](
        s,
        pb,
        in_0,
        out,
        None,
        scale_idx,
        cb,
        lambda,
        f32::INFINITY,
        None,
        None,
    );
}
