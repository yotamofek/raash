use std::ptr;

use libc::{c_float, c_int, c_uint};

use super::{aac_cb_maxval, aac_cb_range, clip_uintp2_c, ff_log2_c, put_bits, put_sbits, quant};
use crate::{
    aacenc::{abs_pow34_v, ctx::AACEncContext, quantize_bands},
    aactab::{ff_aac_codebook_vectors, ff_aac_spectral_bits, ff_aac_spectral_codes, POW_SF_TABLES},
    common::*,
    types::*,
};

type quantize_and_encode_band_func = unsafe fn(
    *mut AACEncContext,
    *mut PutBitContext,
    *const c_float,
    *mut c_float,
    *const c_float,
    c_int,
    c_int,
    c_int,
    c_float,
    c_float,
    *mut c_int,
    *mut c_float,
) -> c_float;

#[inline(always)]
unsafe fn cost_template(
    mut s: *mut AACEncContext,
    mut pb: *mut PutBitContext,
    mut in_0: *const c_float,
    mut out: *mut c_float,
    mut scaled: *const c_float,
    mut size: c_int,
    mut scale_idx: c_int,
    mut cb: c_int,
    lambda: c_float,
    uplim: c_float,
    bits: *mut c_int,
    energy: *mut c_float,
    mut BT_ZERO: c_int,
    mut BT_UNSIGNED: c_int,
    mut BT_PAIR: c_int,
    mut BT_ESC: c_int,
    mut BT_NOISE: c_int,
    mut BT_STEREO: c_int,
    ROUNDING: c_float,
) -> c_float {
    let q_idx: c_int = 200 as c_int - scale_idx + 140 as c_int - 36 as c_int;
    let Q: c_float = POW_SF_TABLES.pow2[q_idx as usize];
    let Q34: c_float = POW_SF_TABLES.pow34[q_idx as usize];
    let IQ: c_float =
        POW_SF_TABLES.pow2[(200 as c_int + scale_idx - 140 as c_int + 36 as c_int) as usize];
    let CLIPPED_ESCAPE: c_float = 165140.0f32 * IQ;
    let mut cost: c_float = 0 as c_int as c_float;
    let mut qenergy: c_float = 0 as c_int as c_float;
    let dim: c_int = if BT_PAIR != 0 { 2 as c_int } else { 4 as c_int };
    let mut resbits: c_int = 0 as c_int;
    let mut off: c_int = 0;
    if BT_ZERO != 0 || BT_NOISE != 0 || BT_STEREO != 0 {
        let mut i: c_int = 0 as c_int;
        while i < size {
            cost += *in_0.offset(i as isize) * *in_0.offset(i as isize);
            i += 1;
            i;
        }
        if !bits.is_null() {
            *bits = 0 as c_int;
        }
        if !energy.is_null() {
            *energy = qenergy;
        }
        if !out.is_null() {
            let mut i_0: c_int = 0 as c_int;
            while i_0 < size {
                let mut j: c_int = 0 as c_int;
                while j < dim {
                    *out.offset((i_0 + j) as isize) = 0.0f32;
                    j += 1;
                    j;
                }
                i_0 += dim;
            }
        }
        return cost * lambda;
    }
    if scaled.is_null() {
        abs_pow34_v(((*s).scoefs).as_mut_ptr(), in_0, size);
        scaled = ((*s).scoefs).as_mut_ptr();
    }
    quantize_bands(
        ((*s).qcoefs).as_mut_ptr(),
        in_0,
        scaled,
        size,
        (BT_UNSIGNED == 0) as c_int,
        aac_cb_maxval[cb as usize] as c_int,
        Q34,
        ROUNDING,
    );
    if BT_UNSIGNED != 0 {
        off = 0 as c_int;
    } else {
        off = aac_cb_maxval[cb as usize] as c_int;
    }
    let mut i_1: c_int = 0 as c_int;
    while i_1 < size {
        let mut vec: *const c_float = ptr::null::<c_float>();
        let mut quants: *mut c_int = ((*s).qcoefs).as_mut_ptr().offset(i_1 as isize);
        let mut curidx: c_int = 0 as c_int;
        let mut curbits: c_int = 0;
        let mut quantized: c_float = 0.;
        let mut rd: c_float = 0.0f32;
        let mut j_0: c_int = 0 as c_int;
        while j_0 < dim {
            curidx *= aac_cb_range[cb as usize] as c_int;
            curidx += *quants.offset(j_0 as isize) + off;
            j_0 += 1;
            j_0;
        }
        curbits = ff_aac_spectral_bits[(cb - 1 as c_int) as usize][curidx as usize] as c_int;
        vec = &(*ff_aac_codebook_vectors[(cb - 1 as c_int) as usize])[(curidx * dim) as usize]
            as *const c_float;
        if BT_UNSIGNED != 0 {
            let mut j_1: c_int = 0 as c_int;
            while j_1 < dim {
                let mut t: c_float = fabsf(*in_0.offset((i_1 + j_1) as isize));
                let mut di: c_float = 0.;
                if BT_ESC != 0 && *vec.offset(j_1 as isize) == 64.0f32 {
                    if t >= CLIPPED_ESCAPE {
                        quantized = CLIPPED_ESCAPE;
                        curbits += 21 as c_int;
                    } else {
                        let mut c: c_int =
                            clip_uintp2_c(quant(t, Q, ROUNDING), 13 as c_int) as c_int;
                        quantized = c as c_float * cbrtf(c as c_float) * IQ;
                        curbits += ff_log2_c(c as c_uint) * 2 as c_int - 4 as c_int + 1 as c_int;
                    }
                } else {
                    quantized = *vec.offset(j_1 as isize) * IQ;
                }
                di = t - quantized;
                if !out.is_null() {
                    *out.offset((i_1 + j_1) as isize) =
                        if *in_0.offset((i_1 + j_1) as isize) >= 0 as c_int as c_float {
                            quantized
                        } else {
                            -quantized
                        };
                }
                if *vec.offset(j_1 as isize) != 0.0f32 {
                    curbits += 1;
                    curbits;
                }
                qenergy += quantized * quantized;
                rd += di * di;
                j_1 += 1;
                j_1;
            }
        } else {
            let mut j_2: c_int = 0 as c_int;
            while j_2 < dim {
                quantized = *vec.offset(j_2 as isize) * IQ;
                qenergy += quantized * quantized;
                if !out.is_null() {
                    *out.offset((i_1 + j_2) as isize) = quantized;
                }
                rd += (*in_0.offset((i_1 + j_2) as isize) - quantized)
                    * (*in_0.offset((i_1 + j_2) as isize) - quantized);
                j_2 += 1;
                j_2;
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
                ff_aac_spectral_bits[(cb - 1 as c_int) as usize][curidx as usize] as c_int,
                ff_aac_spectral_codes[(cb - 1 as c_int) as usize][curidx as usize] as BitBuf,
            );
            if BT_UNSIGNED != 0 {
                let mut j_3: c_int = 0 as c_int;
                while j_3 < dim {
                    if ff_aac_codebook_vectors[(cb - 1 as c_int) as usize]
                        [(curidx * dim + j_3) as usize]
                        != 0.0f32
                    {
                        put_bits(
                            pb,
                            1,
                            (*in_0.offset((i_1 + j_3) as isize) < 0.0f32) as c_int as BitBuf,
                        );
                    }
                    j_3 += 1;
                    j_3;
                }
            }
            if BT_ESC != 0 {
                let mut j_4: c_int = 0 as c_int;
                while j_4 < 2 as c_int {
                    if ff_aac_codebook_vectors[(cb - 1) as usize][(curidx * 2 + j_4) as usize]
                        == 64.0f32
                    {
                        let mut coef: c_int = clip_uintp2_c(
                            quant(fabsf(*in_0.offset((i_1 + j_4) as isize)), Q, ROUNDING),
                            13,
                        ) as c_int;
                        let mut len: c_int = ff_log2_c(coef as c_uint);
                        put_bits(
                            pb,
                            len - 4 as c_int + 1,
                            (((1 as c_int) << len - 4 as c_int + 1 as c_int) - 2 as c_int)
                                as BitBuf,
                        );
                        put_sbits(pb, len, coef);
                    }
                    j_4 += 1;
                    j_4;
                }
            }
        }
        i_1 += dim;
    }
    if !bits.is_null() {
        *bits = resbits;
    }
    if !energy.is_null() {
        *energy = qenergy;
    }
    cost
}

#[inline]
unsafe fn cost_NONE(
    mut _s: *mut AACEncContext,
    mut _pb: *mut PutBitContext,
    mut _in_0: *const c_float,
    mut _quant_0: *mut c_float,
    mut _scaled: *const c_float,
    mut _size: c_int,
    mut _scale_idx: c_int,
    mut _cb: c_int,
    _lambda: c_float,
    _uplim: c_float,
    mut _bits: *mut c_int,
    mut _energy: *mut c_float,
) -> c_float {
    0.0f32
}
unsafe fn cost_ZERO(
    s: *mut AACEncContext,
    pb: *mut PutBitContext,
    in_0: *const c_float,
    quant_0: *mut c_float,
    scaled: *const c_float,
    size: c_int,
    scale_idx: c_int,
    cb: c_int,
    lambda: c_float,
    uplim: c_float,
    bits: *mut c_int,
    energy: *mut c_float,
) -> c_float {
    cost_template(
        s, pb, in_0, quant_0, scaled, size, scale_idx, cb, lambda, uplim, bits, energy, 1, 0, 0, 0,
        0, 0, 0.4054f32,
    )
}

unsafe fn cost_SQUAD(
    s: *mut AACEncContext,
    pb: *mut PutBitContext,
    in_0: *const c_float,
    quant_0: *mut c_float,
    scaled: *const c_float,
    size: c_int,
    scale_idx: c_int,
    cb: c_int,
    lambda: c_float,
    uplim: c_float,
    bits: *mut c_int,
    energy: *mut c_float,
) -> c_float {
    cost_template(
        s, pb, in_0, quant_0, scaled, size, scale_idx, cb, lambda, uplim, bits, energy, 0, 0, 0, 0,
        0, 0, 0.4054f32,
    )
}

unsafe fn cost_UQUAD(
    s: *mut AACEncContext,
    pb: *mut PutBitContext,
    in_0: *const c_float,
    quant_0: *mut c_float,
    scaled: *const c_float,
    size: c_int,
    scale_idx: c_int,
    cb: c_int,
    lambda: c_float,
    uplim: c_float,
    bits: *mut c_int,
    energy: *mut c_float,
) -> c_float {
    cost_template(
        s, pb, in_0, quant_0, scaled, size, scale_idx, cb, lambda, uplim, bits, energy, 0, 1, 0, 0,
        0, 0, 0.4054f32,
    )
}

unsafe fn cost_SPAIR(
    s: *mut AACEncContext,
    pb: *mut PutBitContext,
    in_0: *const c_float,
    quant_0: *mut c_float,
    scaled: *const c_float,
    size: c_int,
    scale_idx: c_int,
    cb: c_int,
    lambda: c_float,
    uplim: c_float,
    bits: *mut c_int,
    energy: *mut c_float,
) -> c_float {
    cost_template(
        s, pb, in_0, quant_0, scaled, size, scale_idx, cb, lambda, uplim, bits, energy, 0, 0, 1, 0,
        0, 0, 0.4054f32,
    )
}

unsafe fn cost_UPAIR(
    s: *mut AACEncContext,
    pb: *mut PutBitContext,
    in_0: *const c_float,
    quant_0: *mut c_float,
    scaled: *const c_float,
    size: c_int,
    scale_idx: c_int,
    cb: c_int,
    lambda: c_float,
    uplim: c_float,
    bits: *mut c_int,
    energy: *mut c_float,
) -> c_float {
    cost_template(
        s, pb, in_0, quant_0, scaled, size, scale_idx, cb, lambda, uplim, bits, energy, 0, 1, 1, 0,
        0, 0, 0.4054f32,
    )
}

unsafe fn cost_ESC(
    s: *mut AACEncContext,
    pb: *mut PutBitContext,
    in_0: *const c_float,
    quant_0: *mut c_float,
    scaled: *const c_float,
    size: c_int,
    scale_idx: c_int,
    cb: c_int,
    lambda: c_float,
    uplim: c_float,
    bits: *mut c_int,
    energy: *mut c_float,
) -> c_float {
    cost_template(
        s,
        pb,
        in_0,
        quant_0,
        scaled,
        size,
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
        0.4054f32,
    )
}

unsafe fn cost_ESC_RTZ(
    s: *mut AACEncContext,
    pb: *mut PutBitContext,
    in_0: *const c_float,
    quant_0: *mut c_float,
    scaled: *const c_float,
    size: c_int,
    scale_idx: c_int,
    cb: c_int,
    lambda: c_float,
    uplim: c_float,
    bits: *mut c_int,
    energy: *mut c_float,
) -> c_float {
    cost_template(
        s,
        pb,
        in_0,
        quant_0,
        scaled,
        size,
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
    in_0: *const c_float,
    quant_0: *mut c_float,
    scaled: *const c_float,
    size: c_int,
    scale_idx: c_int,
    cb: c_int,
    lambda: c_float,
    uplim: c_float,
    bits: *mut c_int,
    energy: *mut c_float,
) -> c_float {
    cost_template(
        s, pb, in_0, quant_0, scaled, size, scale_idx, cb, lambda, uplim, bits, energy, 0, 0, 0, 0,
        1, 0, 0.4054f32,
    )
}

unsafe fn cost_STEREO(
    s: *mut AACEncContext,
    pb: *mut PutBitContext,
    in_0: *const c_float,
    quant_0: *mut c_float,
    scaled: *const c_float,
    size: c_int,
    scale_idx: c_int,
    cb: c_int,
    lambda: c_float,
    uplim: c_float,
    bits: *mut c_int,
    energy: *mut c_float,
) -> c_float {
    cost_template(
        s, pb, in_0, quant_0, scaled, size, scale_idx, cb, lambda, uplim, bits, energy, 0, 0, 0, 0,
        0, 1, 0.4054f32,
    )
}

const quantize_and_encode_band_cost_arr: [quantize_and_encode_band_func; 16] = [
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

const quantize_and_encode_band_cost_rtz_arr: [quantize_and_encode_band_func; 16] = [
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
    in_0: *const c_float,
    quant_0: *mut c_float,
    scaled: *const c_float,
    size: c_int,
    scale_idx: c_int,
    cb: c_int,
    lambda: c_float,
    uplim: c_float,
    bits: *mut c_int,
    energy: *mut c_float,
) -> c_float {
    (quantize_and_encode_band_cost_arr[cb as usize])(
        s, pb, in_0, quant_0, scaled, size, scale_idx, cb, lambda, uplim, bits, energy,
    )
}

#[inline]
pub(super) unsafe fn quantize_and_encode_band(
    mut s: *mut AACEncContext,
    mut pb: *mut PutBitContext,
    mut in_0: *const c_float,
    mut out: *mut c_float,
    mut size: c_int,
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
        ptr::null(),
        size,
        scale_idx,
        cb,
        lambda,
        f32::INFINITY,
        ptr::null_mut(),
        ptr::null_mut(),
    );
}
