#![deny(dead_code)]

use std::ptr::null_mut;

use array_util::{Array, WindowedArray};
use ffi::{
    class::option::AVOptionType,
    codec::{AVCodecContext, AVCodecID},
    num::{AVComplexDouble, AVComplexFloat, AVComplexInt32},
};
use libc::{c_char, c_double, c_float, c_int, c_long, c_uchar, c_uint, c_ulong, c_ushort, c_void};

use crate::aac::{
    encoder::TemporalNoiseShaping, psy_model::AacPsyContext, IndividualChannelStream,
    WindowSequence,
};

#[allow(non_camel_case_types)]
pub(crate) type ptrdiff_t = c_long;

pub(crate) const AV_OPT_TYPE_BOOL: AVOptionType = 18;
pub(crate) const AV_OPT_TYPE_CONST: AVOptionType = 10;
pub(crate) const AV_OPT_TYPE_INT: AVOptionType = 1;
pub(crate) const AV_OPT_TYPE_FLAGS: AVOptionType = 0;

pub(crate) const AV_CODEC_ID_AAC: AVCodecID = 86018;

pub(crate) type AudioObjectType = c_uint;
pub(crate) const AOT_SBR: AudioObjectType = 5;

pub(crate) type AVTXType = c_uint;
pub(crate) const AV_TX_NB: AVTXType = 18;
pub(crate) const AV_TX_INT32_DST_I: AVTXType = 17;
pub(crate) const AV_TX_DOUBLE_DST_I: AVTXType = 16;
pub(crate) const AV_TX_FLOAT_DST_I: AVTXType = 15;
pub(crate) const AV_TX_INT32_DCT_I: AVTXType = 14;
pub(crate) const AV_TX_DOUBLE_DCT_I: AVTXType = 13;
pub(crate) const AV_TX_FLOAT_DCT_I: AVTXType = 12;
pub(crate) const AV_TX_INT32_DCT: AVTXType = 11;
pub(crate) const AV_TX_DOUBLE_DCT: AVTXType = 10;
pub(crate) const AV_TX_FLOAT_DCT: AVTXType = 9;
pub(crate) const AV_TX_INT32_RDFT: AVTXType = 8;
pub(crate) const AV_TX_DOUBLE_RDFT: AVTXType = 7;
pub(crate) const AV_TX_FLOAT_RDFT: AVTXType = 6;
pub(crate) const AV_TX_INT32_MDCT: AVTXType = 5;
pub(crate) const AV_TX_DOUBLE_MDCT: AVTXType = 3;
pub(crate) const AV_TX_FLOAT_MDCT: AVTXType = 1;
pub(crate) const AV_TX_INT32_FFT: AVTXType = 4;
pub(crate) const AV_TX_DOUBLE_FFT: AVTXType = 2;
pub(crate) const AV_TX_FLOAT_FFT: AVTXType = 0;
pub(crate) type av_tx_fn =
    Option<unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> ()>;

#[derive(Default, Copy, Clone)]
pub(crate) struct SingleChannelElement {
    pub(crate) ics: IndividualChannelStream,
    pub(crate) tns: TemporalNoiseShaping,
    pub(crate) pulse: Pulse,
    pub(crate) band_type: WindowedArray<Array<BandType, 128>, 16>,
    pub(crate) band_alt: WindowedArray<Array<BandType, 128>, 16>,
    pub(crate) sf_idx: WindowedArray<Array<c_int, 128>, 16>,
    pub(crate) zeroes: WindowedArray<Array<bool, 128>, 16>,
    pub(crate) can_pns: WindowedArray<Array<bool, 128>, 16>,
    pub(crate) is_ener: WindowedArray<Array<c_float, 128>, 16>,
    pub(crate) pns_ener: WindowedArray<Array<c_float, 128>, 16>,
    pub(crate) pcoeffs: WindowedArray<Array<c_float, 1024>, 128>,
    pub(crate) coeffs: WindowedArray<Array<c_float, 1024>, 128>,
    pub(crate) ret_buf: Array<c_float, 2048>,
}

pub(crate) type BandType = c_uint;
pub(crate) const INTENSITY_BT: BandType = 15;
pub(crate) const INTENSITY_BT2: BandType = 14;
pub(crate) const NOISE_BT: BandType = 13;
pub(crate) const RESERVED_BT: BandType = 12;
pub(crate) const ESC_BT: BandType = 11;
pub(crate) const ZERO_BT: BandType = 0;

#[derive(Default, Copy, Clone)]
pub(crate) struct Pulse {
    pub(crate) num_pulse: c_int,
    pub(crate) start: c_int,
    pub(crate) pos: [c_int; 4],
    pub(crate) amp: [c_int; 4],
}

#[derive(Default, Copy, Clone)]
pub(crate) struct ChannelElement {
    pub(crate) common_window: c_int,
    pub(crate) ms_mode: c_int,
    pub(crate) is_mode: bool,
    pub(crate) ms_mask: WindowedArray<Array<bool, 128>, 16>,
    pub(crate) is_mask: WindowedArray<Array<bool, 128>, 16>,
    pub(crate) ch: [SingleChannelElement; 2],
}

#[derive(Copy, Clone, Default)]
pub(crate) struct FFPsyBand {
    pub(crate) energy: c_float,
    pub(crate) threshold: c_float,
    pub(crate) spread: c_float,
}

#[derive(Default, Copy, Clone)]
pub(crate) struct FFPsyChannel {
    pub(crate) psy_bands: WindowedArray<Array<FFPsyBand, 128>, 16>,
    pub(crate) entropy: c_float,
}

#[derive(Copy, Clone, Default)]
pub(crate) struct FFPsyChannelGroup {
    pub(crate) num_ch: c_uchar,
}

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub(crate) struct FFPsyWindowInfo {
    pub(crate) window_type: [WindowSequence; 3],
    pub(crate) window_shape: c_int,
    pub(crate) num_windows: c_int,
    pub(crate) grouping: [c_int; 8],
}

#[derive(Clone)]
pub(crate) struct FFPsyContext {
    pub(crate) avctx: *const AVCodecContext,
    pub(crate) ch: Box<[FFPsyChannel]>,
    pub(crate) group: Box<[FFPsyChannelGroup]>,
    pub(crate) cutoff: c_int,
    pub(crate) bands: Box<[&'static [c_uchar]]>,
    pub(crate) num_bands: Box<[c_int]>,
    pub(crate) bitres: BitResolution,
    pub(crate) model_priv_data: Box<AacPsyContext>,
}

impl FFPsyContext {
    pub(crate) fn zero() -> Self {
        Self {
            avctx: null_mut(),
            ch: Default::default(),
            group: Default::default(),
            cutoff: 0,
            bands: Default::default(),
            num_bands: Default::default(),
            bitres: BitResolution {
                size: 0,
                bits: 0,
                alloc: 0,
            },
            model_priv_data: Default::default(),
        }
    }
}

#[derive(Copy, Clone, Default)]
pub(crate) struct BitResolution {
    pub(crate) size: c_int,
    pub(crate) bits: c_int,
    pub(crate) alloc: c_int,
}

pub(crate) type AACCoder = c_uint;
pub(crate) const AAC_CODER_NB: AACCoder = 3;
pub(crate) const AAC_CODER_FAST: AACCoder = 2;
pub(crate) const AAC_CODER_TWOLOOP: AACCoder = 1;
pub(crate) const AAC_CODER_ANMR: AACCoder = 0;
#[derive(Copy, Clone, Default, Debug)]
#[repr(C)]
pub(crate) struct AACEncOptions {
    pub(crate) coder: c_int,
    pub(crate) pns: c_int,
    pub(crate) tns: c_int,
    pub(crate) ltp: c_int,
    pub(crate) pce: c_int,
    pub(crate) pred: c_int,
    pub(crate) mid_side: c_int,
    pub(crate) intensity_stereo: c_int,
}

#[derive(Copy, Clone, Default)]
pub(crate) struct AACQuantizeBandCostCacheEntry {
    pub(crate) rd: c_float,
    pub(crate) energy: c_float,
    pub(crate) bits: c_int,
    pub(crate) cb: c_char,
    pub(crate) rtz: c_char,
    pub(crate) generation: c_ushort,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct AVTXContext {
    pub(crate) len: c_int,
    pub(crate) inv: c_int,
    pub(crate) map: *mut c_int,
    pub(crate) exp: AVTXNum,
    pub(crate) tmp: AVTXNum,
    pub(crate) sub: *mut AVTXContext,
    pub(crate) fn_0: [av_tx_fn; 4],
    pub(crate) nb_sub: c_int,
    pub(crate) cd: [*const FFTXCodelet; 4],
    pub(crate) cd_self: *const FFTXCodelet,
    pub(crate) type_0: AVTXType,
    pub(crate) flags: c_ulong,
    pub(crate) map_dir: FFTXMapDirection,
    pub(crate) scale_f: c_float,
    pub(crate) scale_d: c_double,
    pub(crate) opaque: *mut c_void,
}
#[derive(Copy, Clone)]
pub(crate) union AVTXNum {
    pub(crate) double: *mut AVComplexDouble,
    pub(crate) float: *mut AVComplexFloat,
    pub(crate) int32: *mut AVComplexInt32,
    pub(crate) void: *mut c_void,
}
pub(crate) type FFTXMapDirection = c_uint;
pub(crate) const FF_TX_MAP_SCATTER: FFTXMapDirection = 2;
pub(crate) const FF_TX_MAP_GATHER: FFTXMapDirection = 1;
pub(crate) const FF_TX_MAP_NONE: FFTXMapDirection = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct FFTXCodelet {
    pub(crate) name: *const c_char,
    pub(crate) function: av_tx_fn,
    pub(crate) type_0: AVTXType,
    pub(crate) flags: c_ulong,
    pub(crate) factors: [c_int; 16],
    pub(crate) nb_factors: c_int,
    pub(crate) min_len: c_int,
    pub(crate) max_len: c_int,
    pub(crate) init: Option<
        unsafe extern "C" fn(
            *mut AVTXContext,
            *const FFTXCodelet,
            c_ulong,
            *mut FFTXCodeletOptions,
            c_int,
            c_int,
            *const c_void,
        ) -> c_int,
    >,
    pub(crate) uninit: Option<unsafe extern "C" fn(*mut AVTXContext) -> c_int>,
    pub(crate) cpu_flags: c_int,
    pub(crate) prio: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct FFTXCodeletOptions {
    pub(crate) map_dir: FFTXMapDirection,
}
pub(crate) type AVTXFlags = c_uint;
pub(crate) const AV_TX_REAL_TO_IMAGINARY: AVTXFlags = 16;
pub(crate) const AV_TX_REAL_TO_REAL: AVTXFlags = 8;
pub(crate) const AV_TX_FULL_IMDCT: AVTXFlags = 4;
pub(crate) const AV_TX_UNALIGNED: AVTXFlags = 2;
pub(crate) const AV_TX_INPLACE: AVTXFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct TXCodeletMatch {
    pub(crate) cd: *const FFTXCodelet,
    pub(crate) prio: c_int,
}
pub(crate) const FF_TX_PRIO_MAX: FFTXCodeletPriority = 32768;
pub(crate) type FFTXCodeletPriority = c_int;
pub(crate) const FF_TX_PRIO_MIN: FFTXCodeletPriority = -131072;
pub(crate) const FF_TX_PRIO_BASE: FFTXCodeletPriority = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct FFTXLenDecomp {
    pub(crate) len: c_int,
    pub(crate) len2: c_int,
    pub(crate) prio: c_int,
    pub(crate) cd: *const FFTXCodelet,
}

pub(crate) type AVRounding = c_uint;
pub(crate) const AV_ROUND_PASS_MINMAX: AVRounding = 8192;
pub(crate) const AV_ROUND_NEAR_INF: AVRounding = 5;

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct FFTabInitData {
    pub(crate) func: Option<unsafe extern "C" fn() -> ()>,
    pub(crate) factors: [c_int; 4],
}
