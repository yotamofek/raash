#![deny(dead_code)]

use std::{
    mem::MaybeUninit,
    ptr::{null, null_mut},
};

use ffi::{
    class::option::AVOptionType,
    codec::{AVCodecContext, AVCodecID},
    num::{AVComplexDouble, AVComplexFloat, AVComplexInt32},
};
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_schar, c_short, c_uchar, c_uint, c_ulong, c_ushort,
    c_void,
};

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct AVFloatDSPContext {
    pub(crate) vector_fmul:
        Option<unsafe extern "C" fn(*mut c_float, *const c_float, *const c_float, c_int) -> ()>,
    pub(crate) vector_fmac_scalar:
        Option<unsafe extern "C" fn(*mut c_float, *const c_float, c_float, c_int) -> ()>,
    pub(crate) vector_dmac_scalar:
        Option<unsafe extern "C" fn(*mut c_double, *const c_double, c_double, c_int) -> ()>,
    pub(crate) vector_fmul_scalar:
        Option<unsafe extern "C" fn(*mut c_float, *const c_float, c_float, c_int) -> ()>,
    pub(crate) vector_dmul_scalar:
        Option<unsafe extern "C" fn(*mut c_double, *const c_double, c_double, c_int) -> ()>,
    pub(crate) vector_fmul_window: Option<
        unsafe extern "C" fn(
            *mut c_float,
            *const c_float,
            *const c_float,
            *const c_float,
            c_int,
        ) -> (),
    >,
    pub(crate) vector_fmul_add: Option<
        unsafe extern "C" fn(
            *mut c_float,
            *const c_float,
            *const c_float,
            *const c_float,
            c_int,
        ) -> (),
    >,
    pub(crate) vector_fmul_reverse:
        Option<unsafe extern "C" fn(*mut c_float, *const c_float, *const c_float, c_int) -> ()>,
    pub(crate) butterflies_float:
        Option<unsafe extern "C" fn(*mut c_float, *mut c_float, c_int) -> ()>,
    pub(crate) scalarproduct_float:
        Option<unsafe extern "C" fn(*const c_float, *const c_float, c_int) -> c_float>,
    pub(crate) vector_dmul:
        Option<unsafe extern "C" fn(*mut c_double, *const c_double, *const c_double, c_int) -> ()>,
}

// pub(crate) const AVMEDIA_TYPE_AUDIO: AVMediaType = 1;

pub(crate) type ptrdiff_t = c_long;

// pub(crate) const AV_CLASS_CATEGORY_NA: AVClassCategory = 0;

pub(crate) const AV_OPT_TYPE_BOOL: AVOptionType = 18;
pub(crate) const AV_OPT_TYPE_CONST: AVOptionType = 10;
pub(crate) const AV_OPT_TYPE_INT: AVOptionType = 1;
pub(crate) const AV_OPT_TYPE_FLAGS: AVOptionType = 0;

pub(crate) const AV_CODEC_ID_AAC: AVCodecID = 86018;

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) union unaligned_32 {
    pub(crate) l: c_uint,
}
pub(crate) type BitBuf = c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct PutBitContext {
    pub(crate) bit_buf: BitBuf,
    pub(crate) bit_left: c_int,
    pub(crate) buf: *mut c_uchar,
    pub(crate) buf_ptr: *mut c_uchar,
    pub(crate) buf_end: *mut c_uchar,
}

impl PutBitContext {
    pub(crate) const fn zero() -> Self {
        Self {
            bit_buf: 0,
            bit_left: 0,
            buf: null_mut(),
            buf_ptr: null_mut(),
            buf_end: null_mut(),
        }
    }
}

pub(crate) type AudioObjectType = c_uint;
pub(crate) const AOT_SBR: AudioObjectType = 5;

// pub(crate) type AAC_SIGNE = c_uint;
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

#[derive(Copy, Clone)]
pub(crate) struct SingleChannelElement {
    pub(crate) ics: IndividualChannelStream,
    pub(crate) tns: TemporalNoiseShaping,
    pub(crate) pulse: Pulse,
    pub(crate) band_type: [BandType; 128],
    pub(crate) band_alt: [BandType; 128],
    // pub(crate) band_type_run_end: [c_int; 120],
    // pub(crate) sf: [c_float; 120],
    pub(crate) sf_idx: [c_int; 128],
    pub(crate) zeroes: [bool; 128],
    pub(crate) can_pns: [c_uchar; 128],
    pub(crate) is_ener: [c_float; 128],
    pub(crate) pns_ener: [c_float; 128],
    pub(crate) pcoeffs: [c_float; 1024],
    pub(crate) coeffs: [c_float; 1024],
    // pub(crate) saved: [c_float; 1536],
    pub(crate) ret_buf: [c_float; 2048],
    pub(crate) ltp_state: [c_float; 3072],
    pub(crate) lcoeffs: [c_float; 1024],
    // pub(crate) ret: *mut c_float,
}

pub(crate) type BandType = c_uint;
pub(crate) const INTENSITY_BT: BandType = 15;
pub(crate) const INTENSITY_BT2: BandType = 14;
pub(crate) const NOISE_BT: BandType = 13;
pub(crate) const RESERVED_BT: BandType = 12;
pub(crate) const ESC_BT: BandType = 11;
pub(crate) const ZERO_BT: BandType = 0;

#[derive(Copy, Clone)]
pub(crate) struct Pulse {
    pub(crate) num_pulse: c_int,
    pub(crate) start: c_int,
    pub(crate) pos: [c_int; 4],
    pub(crate) amp: [c_int; 4],
}

#[derive(Copy, Clone, Default)]
pub(crate) struct TemporalNoiseShaping {
    pub(crate) present: c_int,
    pub(crate) n_filt: [c_int; 8],
    pub(crate) length: [[c_int; 4]; 8],
    pub(crate) direction: [[c_int; 4]; 8],
    pub(crate) order: [[c_int; 4]; 8],
    pub(crate) coef_idx: [[[c_int; 20]; 4]; 8],
    pub(crate) coef: [[[c_float; 20]; 4]; 8],
}
#[derive(Copy, Clone)]
pub(crate) struct IndividualChannelStream {
    pub(crate) max_sfb: c_uchar,
    pub(crate) window_sequence: [WindowSequence; 2],
    pub(crate) use_kb_window: [c_uchar; 2],
    // pub(crate) num_window_groups: c_int,
    pub(crate) group_len: [c_uchar; 8],
    pub(crate) ltp: LongTermPrediction,
    pub(crate) swb_offset: *const c_ushort,
    pub(crate) swb_sizes: *const c_uchar,
    pub(crate) num_swb: c_int,
    pub(crate) num_windows: c_int,
    pub(crate) tns_max_bands: c_int,
    pub(crate) predictor_present: c_int,
    pub(crate) prediction_used: [c_uchar; 41],
    pub(crate) window_clipping: [c_uchar; 8],
    pub(crate) clip_avoidance_factor: c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct LongTermPrediction {
    pub(crate) present: c_schar,
    pub(crate) lag: c_short,
    pub(crate) coef_idx: c_int,
    pub(crate) coef: c_float,
    pub(crate) used: [c_schar; 40],
}

impl Default for LongTermPrediction {
    fn default() -> Self {
        Self {
            present: Default::default(),
            lag: Default::default(),
            coef_idx: Default::default(),
            coef: Default::default(),
            used: [0; 40],
        }
    }
}
pub(crate) type WindowSequence = c_uint;
pub(crate) const LONG_STOP_SEQUENCE: WindowSequence = 3;
pub(crate) const EIGHT_SHORT_SEQUENCE: WindowSequence = 2;
pub(crate) const LONG_START_SEQUENCE: WindowSequence = 1;
pub(crate) const ONLY_LONG_SEQUENCE: WindowSequence = 0;

#[derive(Copy, Clone)]
pub(crate) struct ChannelElement {
    // pub(crate) present: c_int,
    pub(crate) common_window: c_int,
    pub(crate) ms_mode: c_int,
    pub(crate) is_mode: c_uchar,
    pub(crate) ms_mask: [c_uchar; 128],
    pub(crate) is_mask: [c_uchar; 128],
    pub(crate) ch: [SingleChannelElement; 2],
    // pub(crate) coup: ChannelCoupling,
    // pub(crate) sbr: SpectralBandReplication,
}

impl ChannelElement {
    pub fn zero() -> Self {
        // all-zeroes is a valid repr for this struct
        // TODO: use default
        unsafe { MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Default)]
pub(crate) struct FFPsyBand {
    pub(crate) bits: c_int,
    pub(crate) energy: c_float,
    pub(crate) threshold: c_float,
    pub(crate) spread: c_float,
}

#[derive(Copy, Clone)]
pub(crate) struct FFPsyChannel {
    pub(crate) psy_bands: [FFPsyBand; 128],
    pub(crate) entropy: c_float,
}

impl Default for FFPsyChannel {
    fn default() -> Self {
        Self {
            psy_bands: [FFPsyBand::default(); 128],
            entropy: Default::default(),
        }
    }
}

#[derive(Copy, Clone, Default)]
pub(crate) struct FFPsyChannelGroup {
    // pub(crate) ch: [*mut FFPsyChannel; 20],
    pub(crate) num_ch: c_uchar,
    // pub(crate) coupling: [c_uchar; 128],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct FFPsyWindowInfo {
    pub(crate) window_type: [c_int; 3],
    pub(crate) window_shape: c_int,
    pub(crate) num_windows: c_int,
    pub(crate) grouping: [c_int; 8],
    pub(crate) clipping: [c_float; 8],
    pub(crate) window_sizes: *mut c_int,
}

#[derive(Clone)]
pub(crate) struct FFPsyContext {
    pub(crate) avctx: *mut AVCodecContext,
    pub(crate) model: *const FFPsyModel,
    pub(crate) ch: Box<[FFPsyChannel]>,
    pub(crate) group: Box<[FFPsyChannelGroup]>,
    // pub(crate) num_groups: c_int,
    pub(crate) cutoff: c_int,
    pub(crate) bands: Box<[&'static [c_uchar]]>,
    pub(crate) num_bands: Box<[c_int]>,
    pub(crate) bitres: C2RustUnnamed_2,
    pub(crate) model_priv_data: *mut c_void,
}

impl FFPsyContext {
    pub(crate) fn zero() -> Self {
        Self {
            avctx: null_mut(),
            model: null(),
            ch: Default::default(),
            group: Default::default(),
            cutoff: 0,
            bands: Default::default(),
            num_bands: Default::default(),
            bitres: C2RustUnnamed_2 {
                size: 0,
                bits: 0,
                alloc: 0,
            },
            model_priv_data: null_mut(),
        }
    }
}

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub(crate) struct C2RustUnnamed_2 {
    pub(crate) size: c_int,
    pub(crate) bits: c_int,
    pub(crate) alloc: c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct FFPsyModel {
    pub(crate) name: *const c_char,
    pub(crate) init: Option<unsafe extern "C" fn(*mut FFPsyContext) -> c_int>,
    pub(crate) window: Option<
        unsafe extern "C" fn(
            *mut FFPsyContext,
            *const c_float,
            *const c_float,
            c_int,
            c_int,
        ) -> FFPsyWindowInfo,
    >,
    pub(crate) analyze: Option<
        unsafe extern "C" fn(
            *mut FFPsyContext,
            c_int,
            *mut *const c_float,
            *const FFPsyWindowInfo,
        ) -> (),
    >,
    pub(crate) end: Option<unsafe extern "C" fn(*mut FFPsyContext) -> ()>,
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

pub(crate) static aac_maxval_cb: [c_uchar; 14] = [0, 1, 3, 5, 5, 7, 7, 7, 9, 9, 9, 9, 9, 11];

pub(crate) type LPC_TYPE = c_float;

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct AACISError {
    pub(crate) pass: c_int,
    pub(crate) phase: c_int,
    pub(crate) error: c_float,
    pub(crate) dist1: c_float,
    pub(crate) dist2: c_float,
    pub(crate) ener01: c_float,
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

pub(crate) const AV_CODEC_FLAG_QSCALE: c_int = 1 << 1;
