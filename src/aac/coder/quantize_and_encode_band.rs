use arrayvec::ArrayVec;
use bit_writer::{BitBuf, BitWriter};
use ffmpeg_src_macro::ffmpeg_src;
use libc::{c_float, c_int, c_uint};

use super::{
    aac_cb_maxval,
    math::{clip_uintp2_c, ff_log2_c},
    quant, CB_RANGE,
};
use crate::{
    aac::{
        encoder::{pow::Pow34, quantize_bands},
        tables::{
            ff_aac_codebook_vectors, ff_aac_spectral_bits, ff_aac_spectral_codes, POW_SF_TABLES,
        },
        POW_SF2_ZERO, SCALE_DIV_512, SCALE_ONE_POS,
    },
    common::*,
    types::*,
};

type QuantizeAndEncodeBandFunc = fn(
    Option<&mut BitWriter>,
    &[c_float],
    Option<&mut [c_float]>,
    &[c_float],
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
fn cost_template(
    mut pb: Option<&mut BitWriter>,
    in_: &[c_float],
    mut out: Option<&mut [c_float]>,
    scaled: &[c_float],
    scale_idx: c_int,
    cb: c_int,
    lambda: c_float,
    uplim: c_float,
    bits: Option<&mut c_int>,
    energy: Option<&mut c_float>,
    BT_ZERO: c_int,
    BT_UNSIGNED: c_int,
    BT_PAIR: c_int,
    BT_ESC: c_int,
    BT_NOISE: c_int,
    BT_STEREO: c_int,
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
    let quantized = quantize_bands(
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
        let curidx = quantized[i as usize..][..dim as usize]
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
        if let Some(pb) = &mut pb {
            pb.put(
                ff_aac_spectral_bits[(cb - 1) as usize][curidx as usize],
                BitBuf::from(ff_aac_spectral_codes[(cb - 1) as usize][curidx as usize]),
            );
            if BT_UNSIGNED != 0 {
                for j in 0..dim {
                    if ff_aac_codebook_vectors[(cb - 1) as usize][(curidx * dim + j) as usize] != 0.
                    {
                        pb.put(1, BitBuf::from(in_[(i + j) as usize] < 0.));
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
                        pb.put((len - 4 + 1) as u8, ((1 << (len - 4 + 1)) - 2) as BitBuf);
                        pb.put_signed(len as u8, coef);
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
fn cost_NONE(
    _pb: Option<&mut BitWriter>,
    _in_: &[c_float],
    _quant: Option<&mut [c_float]>,
    _scaled: &[c_float],
    _scale_idx: c_int,
    _cb: c_int,
    _lambda: c_float,
    _uplim: c_float,
    _bits: Option<&mut c_int>,
    _energy: Option<&mut c_float>,
) -> c_float {
    0.
}
fn cost_ZERO(
    pb: Option<&mut BitWriter>,
    in_: &[c_float],
    quant: Option<&mut [c_float]>,
    scaled: &[c_float],
    scale_idx: c_int,
    cb: c_int,
    lambda: c_float,
    uplim: c_float,
    bits: Option<&mut c_int>,
    energy: Option<&mut c_float>,
) -> c_float {
    cost_template(
        pb, in_, quant, scaled, scale_idx, cb, lambda, uplim, bits, energy, 1, 0, 0, 0, 0, 0,
        0.4054,
    )
}

fn cost_SQUAD(
    pb: Option<&mut BitWriter>,
    in_: &[c_float],
    quant: Option<&mut [c_float]>,
    scaled: &[c_float],
    scale_idx: c_int,
    cb: c_int,
    lambda: c_float,
    uplim: c_float,
    bits: Option<&mut c_int>,
    energy: Option<&mut c_float>,
) -> c_float {
    cost_template(
        pb, in_, quant, scaled, scale_idx, cb, lambda, uplim, bits, energy, 0, 0, 0, 0, 0, 0,
        0.4054,
    )
}

fn cost_UQUAD(
    pb: Option<&mut BitWriter>,
    in_: &[c_float],
    quant: Option<&mut [c_float]>,
    scaled: &[c_float],
    scale_idx: c_int,
    cb: c_int,
    lambda: c_float,
    uplim: c_float,
    bits: Option<&mut c_int>,
    energy: Option<&mut c_float>,
) -> c_float {
    cost_template(
        pb, in_, quant, scaled, scale_idx, cb, lambda, uplim, bits, energy, 0, 1, 0, 0, 0, 0,
        0.4054,
    )
}

fn cost_SPAIR(
    pb: Option<&mut BitWriter>,
    in_: &[c_float],
    quant: Option<&mut [c_float]>,
    scaled: &[c_float],
    scale_idx: c_int,
    cb: c_int,
    lambda: c_float,
    uplim: c_float,
    bits: Option<&mut c_int>,
    energy: Option<&mut c_float>,
) -> c_float {
    cost_template(
        pb, in_, quant, scaled, scale_idx, cb, lambda, uplim, bits, energy, 0, 0, 1, 0, 0, 0,
        0.4054,
    )
}

fn cost_UPAIR(
    pb: Option<&mut BitWriter>,
    in_: &[c_float],
    quant: Option<&mut [c_float]>,
    scaled: &[c_float],
    scale_idx: c_int,
    cb: c_int,
    lambda: c_float,
    uplim: c_float,
    bits: Option<&mut c_int>,
    energy: Option<&mut c_float>,
) -> c_float {
    cost_template(
        pb, in_, quant, scaled, scale_idx, cb, lambda, uplim, bits, energy, 0, 1, 1, 0, 0, 0,
        0.4054,
    )
}

fn cost_ESC(
    pb: Option<&mut BitWriter>,
    in_: &[c_float],
    quant: Option<&mut [c_float]>,
    scaled: &[c_float],
    scale_idx: c_int,
    _cb: c_int,
    lambda: c_float,
    uplim: c_float,
    bits: Option<&mut c_int>,
    energy: Option<&mut c_float>,
) -> c_float {
    cost_template(
        pb,
        in_,
        quant,
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

fn cost_ESC_RTZ(
    pb: Option<&mut BitWriter>,
    in_: &[c_float],
    quant: Option<&mut [c_float]>,
    scaled: &[c_float],
    scale_idx: c_int,
    _cb: c_int,
    lambda: c_float,
    uplim: c_float,
    bits: Option<&mut c_int>,
    energy: Option<&mut c_float>,
) -> c_float {
    cost_template(
        pb,
        in_,
        quant,
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

fn cost_NOISE(
    pb: Option<&mut BitWriter>,
    in_: &[c_float],
    quant: Option<&mut [c_float]>,
    scaled: &[c_float],
    scale_idx: c_int,
    cb: c_int,
    lambda: c_float,
    uplim: c_float,
    bits: Option<&mut c_int>,
    energy: Option<&mut c_float>,
) -> c_float {
    cost_template(
        pb, in_, quant, scaled, scale_idx, cb, lambda, uplim, bits, energy, 0, 0, 0, 0, 1, 0,
        0.4054,
    )
}

fn cost_STEREO(
    pb: Option<&mut BitWriter>,
    in_: &[c_float],
    quant: Option<&mut [c_float]>,
    scaled: &[c_float],
    scale_idx: c_int,
    cb: c_int,
    lambda: c_float,
    uplim: c_float,
    bits: Option<&mut c_int>,
    energy: Option<&mut c_float>,
) -> c_float {
    cost_template(
        pb, in_, quant, scaled, scale_idx, cb, lambda, uplim, bits, energy, 0, 0, 0, 0, 0, 1,
        0.4054,
    )
}

const fn_arr: [QuantizeAndEncodeBandFunc; 16] = [
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

const fn_rtz_arr: [QuantizeAndEncodeBandFunc; 16] = [
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

pub(crate) fn quantize_and_encode_band_cost(
    in_: &[c_float],
    quant: Option<&mut [c_float]>,
    scaled: &[c_float],
    scale_idx: c_int,
    cb: c_int,
    lambda: c_float,
    uplim: c_float,
    bits: Option<&mut c_int>,
    energy: Option<&mut c_float>,
) -> c_float {
    fn_arr[cb as usize](
        None, in_, quant, scaled, scale_idx, cb, lambda, uplim, bits, energy,
    )
}

#[inline]
pub(crate) fn quantize_and_encode_band(
    pb: &mut BitWriter,
    in_: &[c_float],
    scale_idx: c_int,
    cb: c_int,
    lambda: c_float,
    rtz: bool,
) {
    let arr = if rtz { &fn_rtz_arr } else { &fn_arr };
    let scaled = ArrayVec::<_, 1024>::from_iter(in_.iter().copied().map(Pow34::abs_pow34));
    arr[cb as usize](
        Some(pb),
        in_,
        None,
        &scaled,
        scale_idx,
        cb,
        lambda,
        f32::INFINITY,
        None,
        None,
    );
}
