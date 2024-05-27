use arrayvec::ArrayVec;
use bit_writer::{BitBuf, BitWriter};
use ffmpeg_src_macro::ffmpeg_src;
use libc::{c_float, c_int, c_uint};

use super::{
    math::{clip_uintp2_c, ff_log2_c},
    quant, CB_MAXVAL, CB_RANGE,
};
use crate::aac::{
    encoder::{pow::Pow34, quantize_bands},
    tables::{ff_aac_codebook_vectors, ff_aac_spectral_bits, ff_aac_spectral_codes, POW_SF_TABLES},
    POW_SF2_ZERO, SCALE_DIV_512, SCALE_ONE_POS,
};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CostParams {
    zero: bool,
    unsigned: bool,
    pair: bool,
    esc: bool,
    noise: bool,
    stereo: bool,

    cb: Option<c_int>,
}

mod flags {
    use libc::c_int;

    use super::CostParams as Params;
    use crate::aac::coder::ESC_BT;

    const fn default() -> Params {
        Params {
            zero: false,
            unsigned: false,
            pair: false,
            esc: false,
            noise: false,
            stereo: false,
            cb: None,
        }
    }

    pub(super) const ZERO: Params = Params {
        zero: true,
        ..default()
    };
    pub(super) const SQUAD: Params = Params { ..default() };
    pub(super) const UQUAD: Params = Params {
        unsigned: true,
        ..default()
    };
    pub(super) const SPAIR: Params = Params {
        pair: true,
        ..default()
    };
    pub(super) const UPAIR: Params = Params {
        unsigned: true,
        pair: true,
        ..default()
    };
    pub(super) const ESC: Params = Params {
        unsigned: true,
        pair: true,
        esc: true,
        cb: Some(ESC_BT as c_int),
        ..default()
    };
    pub(super) const NOISE: Params = Params {
        noise: true,
        ..default()
    };
    pub(super) const STEREO: Params = Params {
        stereo: true,
        ..default()
    };
}

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
    flags: CostParams,
    rounding: c_float,
) -> c_float {
    let cb = flags.cb.unwrap_or(cb);
    let q_idx = c_int::from(POW_SF2_ZERO) - scale_idx + c_int::from(SCALE_ONE_POS)
        - c_int::from(SCALE_DIV_512);
    let Q: c_float = POW_SF_TABLES.pow2()[q_idx as usize];
    let Q34: c_float = POW_SF_TABLES.pow34()[q_idx as usize];
    let IQ: c_float = POW_SF_TABLES.pow2()[(c_int::from(POW_SF2_ZERO) + scale_idx
        - c_int::from(SCALE_ONE_POS)
        + c_int::from(SCALE_DIV_512)) as usize];
    let CLIPPED_ESCAPE: c_float = 165140. * IQ;
    let dim = if flags.pair { 2 } else { 4 };
    let mut cost: c_float = 0.;
    let mut qenergy: c_float = 0.;
    let mut resbits: c_int = 0;
    if flags.zero || flags.noise || flags.stereo {
        let cost = in_.iter().map(|in_| in_.powi(2)).sum::<c_float>();
        if let Some(bits) = bits {
            *bits = 0;
        }
        if let Some(energy) = energy {
            *energy = 0.;
        }
        if let Some(out) = out {
            out.chunks_exact_mut(dim as usize)
                .for_each(|out| out.fill(0.));
        }
        return cost * lambda;
    }
    let quantized = quantize_bands(
        in_,
        scaled,
        !flags.unsigned,
        CB_MAXVAL[cb as usize] as c_int,
        Q34,
        rounding,
    );
    let off = if flags.unsigned {
        0
    } else {
        CB_MAXVAL[cb as usize] as c_int
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
        if flags.unsigned {
            for j in 0..dim {
                let t = in_[(i + j) as usize].abs();
                let quantized = if flags.esc && vec[j as usize] == 64. {
                    if t >= CLIPPED_ESCAPE {
                        curbits += 21;
                        CLIPPED_ESCAPE
                    } else {
                        let c = clip_uintp2_c(quant(t, Q, rounding), 13) as c_int;
                        curbits += ff_log2_c(c as c_uint) * 2 - 4 + 1;
                        c as c_float * (c as c_float).cbrt() * IQ
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
            if flags.unsigned {
                for j in 0..dim {
                    if ff_aac_codebook_vectors[(cb - 1) as usize][(curidx * dim + j) as usize] != 0.
                    {
                        pb.put(1, BitBuf::from(in_[(i + j) as usize] < 0.));
                    }
                }
            }
            if flags.esc {
                for j in 0..2 {
                    if ff_aac_codebook_vectors[(cb - 1) as usize][(curidx * 2 + j) as usize] == 64.
                    {
                        let coef =
                            clip_uintp2_c(quant(in_[(i + j) as usize].abs(), Q, rounding), 13)
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

const COST_PARAMS: [Option<CostParams>; 16] = {
    use self::flags::*;
    [
        Some(ZERO),
        Some(SQUAD),
        Some(SQUAD),
        Some(UQUAD),
        Some(UQUAD),
        Some(SPAIR),
        Some(SPAIR),
        Some(UPAIR),
        Some(UPAIR),
        Some(UPAIR),
        Some(UPAIR),
        Some(ESC),
        None,
        Some(NOISE),
        Some(STEREO),
        Some(STEREO),
    ]
};

const ROUNDING: c_float = 0.4054;
const ROUNDING_RTZ: c_float = 0.1054;

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
    COST_PARAMS[cb as usize]
        .map(|flags| {
            cost_template(
                None, in_, quant, scaled, scale_idx, cb, lambda, uplim, bits, energy, flags,
                ROUNDING,
            )
        })
        .unwrap_or_default()
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
    let Some(flags) = COST_PARAMS[cb as usize] else {
        return;
    };

    let scaled = ArrayVec::<_, 1024>::from_iter(in_.iter().copied().map(Pow34::abs_pow34));
    let rounding = if rtz && flags == self::flags::ESC {
        ROUNDING_RTZ
    } else {
        ROUNDING
    };

    cost_template(
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
        flags,
        rounding,
    );
}
