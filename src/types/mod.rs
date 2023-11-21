#![deny(dead_code)]

use ffi::{
    class::{option::AVOptionType, AVClassCategory},
    codec::{
        AVChannel, AVChannelLayout, AVChannelOrder, AVCodecContext, AVCodecID, AVMediaType,
        AVSampleFormat,
    },
    num::{AVComplexDouble, AVComplexFloat, AVComplexInt32},
};
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_schar, c_short, c_uchar, c_uint, c_ulong, c_ushort,
    c_void,
};

use crate::aacenc::ctx::{AACContext, AACEncContext};

pub(crate) const AV_CHAN_WIDE_RIGHT: AVChannel = 32;
pub(crate) const AV_CHAN_WIDE_LEFT: AVChannel = 31;
pub(crate) const AV_CHAN_TOP_BACK_RIGHT: AVChannel = 17;
pub(crate) const AV_CHAN_TOP_BACK_CENTER: AVChannel = 16;
pub(crate) const AV_CHAN_TOP_BACK_LEFT: AVChannel = 15;
pub(crate) const AV_CHAN_TOP_FRONT_RIGHT: AVChannel = 14;
pub(crate) const AV_CHAN_TOP_FRONT_CENTER: AVChannel = 13;
pub(crate) const AV_CHAN_TOP_FRONT_LEFT: AVChannel = 12;
pub(crate) const AV_CHAN_TOP_CENTER: AVChannel = 11;
pub(crate) const AV_CHAN_SIDE_RIGHT: AVChannel = 10;
pub(crate) const AV_CHAN_SIDE_LEFT: AVChannel = 9;
pub(crate) const AV_CHAN_BACK_CENTER: AVChannel = 8;
pub(crate) const AV_CHAN_FRONT_RIGHT_OF_CENTER: AVChannel = 7;
pub(crate) const AV_CHAN_FRONT_LEFT_OF_CENTER: AVChannel = 6;
pub(crate) const AV_CHAN_BACK_RIGHT: AVChannel = 5;
pub(crate) const AV_CHAN_BACK_LEFT: AVChannel = 4;
pub(crate) const AV_CHAN_LOW_FREQUENCY: AVChannel = 3;
pub(crate) const AV_CHAN_FRONT_CENTER: AVChannel = 2;
pub(crate) const AV_CHAN_FRONT_RIGHT: AVChannel = 1;
pub(crate) const AV_CHAN_FRONT_LEFT: AVChannel = 0;

pub(crate) const AV_CHANNEL_ORDER_NATIVE: AVChannelOrder = 1;

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

pub(crate) const AVMEDIA_TYPE_AUDIO: AVMediaType = 1;

pub(crate) type ptrdiff_t = c_long;

pub(crate) const AV_CLASS_CATEGORY_NA: AVClassCategory = 0;

pub(crate) const AV_OPT_TYPE_BOOL: AVOptionType = 18;
pub(crate) const AV_OPT_TYPE_CONST: AVOptionType = 10;
pub(crate) const AV_OPT_TYPE_INT: AVOptionType = 1;
pub(crate) const AV_OPT_TYPE_FLAGS: AVOptionType = 0;

#[allow(dead_code)]
pub(crate) const AV_SAMPLE_FMT_FLTP: AVSampleFormat = 8;

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
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct MPEG4AudioConfig {
    pub(crate) object_type: c_int,
    pub(crate) sampling_index: c_int,
    pub(crate) sample_rate: c_int,
    pub(crate) chan_config: c_int,
    pub(crate) sbr: c_int,
    pub(crate) ext_object_type: c_int,
    pub(crate) ext_sampling_index: c_int,
    pub(crate) ext_sample_rate: c_int,
    pub(crate) ext_chan_config: c_int,
    pub(crate) channels: c_int,
    pub(crate) ps: c_int,
    pub(crate) frame_length_short: c_int,
}
pub(crate) type AudioObjectType = c_uint;
pub(crate) const AOT_SBR: AudioObjectType = 5;

pub(crate) type AAC_SIGNE = c_uint;
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
#[repr(C)]
pub(crate) struct PSDSPContext {
    pub(crate) add_squares:
        Option<unsafe extern "C" fn(*mut c_float, *const [c_float; 2], c_int) -> ()>,
    pub(crate) mul_pair_single: Option<
        unsafe extern "C" fn(*mut [c_float; 2], *mut [c_float; 2], *mut c_float, c_int) -> (),
    >,
    pub(crate) hybrid_analysis: Option<
        unsafe extern "C" fn(
            *mut [c_float; 2],
            *mut [c_float; 2],
            *const [[c_float; 2]; 8],
            ptrdiff_t,
            c_int,
        ) -> (),
    >,
    pub(crate) hybrid_analysis_ileave: Option<
        unsafe extern "C" fn(*mut [[c_float; 2]; 32], *mut [[c_float; 64]; 38], c_int, c_int) -> (),
    >,
    pub(crate) hybrid_synthesis_deint: Option<
        unsafe extern "C" fn(*mut [[c_float; 64]; 38], *mut [[c_float; 2]; 32], c_int, c_int) -> (),
    >,
    pub(crate) decorrelate: Option<
        unsafe extern "C" fn(
            *mut [c_float; 2],
            *mut [c_float; 2],
            *mut [[c_float; 2]; 37],
            *const c_float,
            *const [c_float; 2],
            *const c_float,
            c_float,
            c_int,
        ) -> (),
    >,
    pub(crate) stereo_interpolate: [Option<
        unsafe extern "C" fn(
            *mut [c_float; 2],
            *mut [c_float; 2],
            *mut [c_float; 4],
            *mut [c_float; 4],
            c_int,
        ) -> (),
    >; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct PSCommonContext {
    pub(crate) start: c_int,
    pub(crate) enable_iid: c_int,
    pub(crate) iid_quant: c_int,
    pub(crate) nr_iid_par: c_int,
    pub(crate) nr_ipdopd_par: c_int,
    pub(crate) enable_icc: c_int,
    pub(crate) icc_mode: c_int,
    pub(crate) nr_icc_par: c_int,
    pub(crate) enable_ext: c_int,
    pub(crate) frame_class: c_int,
    pub(crate) num_env_old: c_int,
    pub(crate) num_env: c_int,
    pub(crate) enable_ipdopd: c_int,
    pub(crate) border_position: [c_int; 6],
    pub(crate) iid_par: [[c_schar; 34]; 5],
    pub(crate) icc_par: [[c_schar; 34]; 5],
    pub(crate) ipd_par: [[c_schar; 34]; 5],
    pub(crate) opd_par: [[c_schar; 34]; 5],
    pub(crate) is34bands: c_int,
    pub(crate) is34bands_old: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct PSContext {
    pub(crate) common: PSCommonContext,
    pub(crate) in_buf: [[[c_float; 2]; 44]; 5],
    pub(crate) delay: [[[c_float; 2]; 46]; 91],
    pub(crate) ap_delay: [[[[c_float; 2]; 37]; 3]; 50],
    pub(crate) peak_decay_nrg: [c_float; 34],
    pub(crate) power_smooth: [c_float; 34],
    pub(crate) peak_decay_diff_smooth: [c_float; 34],
    pub(crate) H11: [[[c_float; 34]; 6]; 2],
    pub(crate) H12: [[[c_float; 34]; 6]; 2],
    pub(crate) H21: [[[c_float; 34]; 6]; 2],
    pub(crate) H22: [[[c_float; 34]; 6]; 2],
    pub(crate) Lbuf: [[[c_float; 2]; 32]; 91],
    pub(crate) Rbuf: [[[c_float; 2]; 32]; 91],
    pub(crate) opd_hist: [c_schar; 34],
    pub(crate) ipd_hist: [c_schar; 34],
    pub(crate) dsp: PSDSPContext,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct SBRDSPContext {
    pub(crate) sum64x5: Option<unsafe extern "C" fn(*mut c_float) -> ()>,
    pub(crate) sum_square: Option<unsafe extern "C" fn(*mut [c_float; 2], c_int) -> c_float>,
    pub(crate) neg_odd_64: Option<unsafe extern "C" fn(*mut c_float) -> ()>,
    pub(crate) qmf_pre_shuffle: Option<unsafe extern "C" fn(*mut c_float) -> ()>,
    pub(crate) qmf_post_shuffle:
        Option<unsafe extern "C" fn(*mut [c_float; 2], *const c_float) -> ()>,
    pub(crate) qmf_deint_neg: Option<unsafe extern "C" fn(*mut c_float, *const c_float) -> ()>,
    pub(crate) qmf_deint_bfly:
        Option<unsafe extern "C" fn(*mut c_float, *const c_float, *const c_float) -> ()>,
    pub(crate) autocorrelate:
        Option<unsafe extern "C" fn(*const [c_float; 2], *mut [[c_float; 2]; 2]) -> ()>,
    pub(crate) hf_gen: Option<
        unsafe extern "C" fn(
            *mut [c_float; 2],
            *const [c_float; 2],
            *const c_float,
            *const c_float,
            c_float,
            c_int,
            c_int,
        ) -> (),
    >,
    pub(crate) hf_g_filt: Option<
        unsafe extern "C" fn(
            *mut [c_float; 2],
            *const [[c_float; 2]; 40],
            *const c_float,
            c_int,
            c_long,
        ) -> (),
    >,
    pub(crate) hf_apply_noise: [Option<
        unsafe extern "C" fn(
            *mut [c_float; 2],
            *const c_float,
            *const c_float,
            c_int,
            c_int,
            c_int,
        ) -> (),
    >; 4],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct SingleChannelElement {
    pub(crate) ics: IndividualChannelStream,
    pub(crate) tns: TemporalNoiseShaping,
    pub(crate) pulse: Pulse,
    pub(crate) band_type: [BandType; 128],
    pub(crate) band_alt: [BandType; 128],
    pub(crate) band_type_run_end: [c_int; 120],
    pub(crate) sf: [c_float; 120],
    pub(crate) sf_idx: [c_int; 128],
    pub(crate) zeroes: [c_uchar; 128],
    pub(crate) can_pns: [c_uchar; 128],
    pub(crate) is_ener: [c_float; 128],
    pub(crate) pns_ener: [c_float; 128],
    pub(crate) pcoeffs: [c_float; 1024],
    pub(crate) coeffs: [c_float; 1024],
    pub(crate) saved: [c_float; 1536],
    pub(crate) ret_buf: [c_float; 2048],
    pub(crate) ltp_state: [c_float; 3072],
    pub(crate) lcoeffs: [c_float; 1024],
    pub(crate) prcoeffs: [c_float; 1024],
    pub(crate) predictor_state: [PredictorState; 672],
    pub(crate) ret: *mut c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct PredictorState {
    pub(crate) cor0: c_float,
    pub(crate) cor1: c_float,
    pub(crate) var0: c_float,
    pub(crate) var1: c_float,
    pub(crate) r0: c_float,
    pub(crate) r1: c_float,
    pub(crate) k1: c_float,
    pub(crate) x_est: c_float,
}
pub(crate) type BandType = c_uint;
pub(crate) const INTENSITY_BT: BandType = 15;
pub(crate) const INTENSITY_BT2: BandType = 14;
pub(crate) const NOISE_BT: BandType = 13;
pub(crate) const RESERVED_BT: BandType = 12;
pub(crate) const ESC_BT: BandType = 11;
pub(crate) const ZERO_BT: BandType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct Pulse {
    pub(crate) num_pulse: c_int,
    pub(crate) start: c_int,
    pub(crate) pos: [c_int; 4],
    pub(crate) amp: [c_int; 4],
}
#[derive(Copy, Clone, Default)]
#[repr(C)]
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
#[repr(C)]
pub(crate) struct IndividualChannelStream {
    pub(crate) max_sfb: c_uchar,
    pub(crate) window_sequence: [WindowSequence; 2],
    pub(crate) use_kb_window: [c_uchar; 2],
    pub(crate) num_window_groups: c_int,
    pub(crate) group_len: [c_uchar; 8],
    pub(crate) ltp: LongTermPrediction,
    pub(crate) swb_offset: *const c_ushort,
    pub(crate) swb_sizes: *const c_uchar,
    pub(crate) num_swb: c_int,
    pub(crate) num_windows: c_int,
    pub(crate) tns_max_bands: c_int,
    pub(crate) predictor_present: c_int,
    pub(crate) predictor_initialized: c_int,
    pub(crate) predictor_reset_group: c_int,
    pub(crate) predictor_reset_count: [c_int; 31],
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
#[repr(C)]
pub(crate) struct OutputConfiguration {
    pub(crate) m4ac: MPEG4AudioConfig,
    pub(crate) layout_map: [[c_uchar; 3]; 64],
    pub(crate) layout_map_tags: c_int,
    pub(crate) ch_layout: AVChannelLayout,
    pub(crate) status: OCStatus,
}
pub(crate) type OCStatus = c_uint;
pub(crate) type AACOutputChannelOrder = c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct ChannelElement {
    pub(crate) present: c_int,
    pub(crate) common_window: c_int,
    pub(crate) ms_mode: c_int,
    pub(crate) is_mode: c_uchar,
    pub(crate) ms_mask: [c_uchar; 128],
    pub(crate) is_mask: [c_uchar; 128],
    pub(crate) ch: [SingleChannelElement; 2],
    pub(crate) coup: ChannelCoupling,
    pub(crate) sbr: SpectralBandReplication,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct SpectralBandReplication {
    pub(crate) sample_rate: c_int,
    pub(crate) start: c_int,
    pub(crate) ready_for_dequant: c_int,
    pub(crate) id_aac: c_int,
    pub(crate) reset: c_int,
    pub(crate) spectrum_params: SpectrumParameters,
    pub(crate) bs_amp_res_header: c_int,
    pub(crate) bs_limiter_bands: c_uint,
    pub(crate) bs_limiter_gains: c_uint,
    pub(crate) bs_interpol_freq: c_uint,
    pub(crate) bs_smoothing_mode: c_uint,
    pub(crate) bs_coupling: c_uint,
    pub(crate) k: [AAC_SIGNE; 5],
    pub(crate) kx: [AAC_SIGNE; 2],
    pub(crate) m: [AAC_SIGNE; 2],
    pub(crate) kx_and_m_pushed: c_uint,
    pub(crate) n_master: AAC_SIGNE,
    pub(crate) data: [SBRData; 2],
    pub(crate) ps: PSContext,
    pub(crate) n: [AAC_SIGNE; 2],
    pub(crate) n_q: AAC_SIGNE,
    pub(crate) n_lim: AAC_SIGNE,
    pub(crate) f_master: [c_ushort; 49],
    pub(crate) f_tablelow: [c_ushort; 25],
    pub(crate) f_tablehigh: [c_ushort; 49],
    pub(crate) f_tablenoise: [c_ushort; 6],
    pub(crate) f_tablelim: [c_ushort; 30],
    pub(crate) num_patches: AAC_SIGNE,
    pub(crate) patch_num_subbands: [c_uchar; 6],
    pub(crate) patch_start_subband: [c_uchar; 6],
    pub(crate) X_low: [[[c_float; 2]; 40]; 32],
    pub(crate) X_high: [[[c_float; 2]; 40]; 64],
    pub(crate) X: [[[[c_float; 64]; 38]; 2]; 2],
    pub(crate) alpha0: [[c_float; 2]; 64],
    pub(crate) alpha1: [[c_float; 2]; 64],
    pub(crate) e_origmapped: [[c_float; 48]; 7],
    pub(crate) q_mapped: [[c_float; 48]; 7],
    pub(crate) s_mapped: [[c_uchar; 48]; 7],
    pub(crate) e_curr: [[c_float; 48]; 7],
    pub(crate) q_m: [[c_float; 48]; 7],
    pub(crate) s_m: [[c_float; 48]; 7],
    pub(crate) gain: [[c_float; 48]; 7],
    pub(crate) qmf_filter_scratch: [[c_float; 64]; 5],
    pub(crate) mdct_ana: *mut AVTXContext,
    pub(crate) mdct_ana_fn: av_tx_fn,
    pub(crate) mdct: *mut AVTXContext,
    pub(crate) mdct_fn: av_tx_fn,
    pub(crate) dsp: SBRDSPContext,
    pub(crate) c: AACSBRContext,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct AACSBRContext {
    pub(crate) sbr_lf_gen: Option<
        unsafe extern "C" fn(
            *mut AACContext,
            *mut SpectralBandReplication,
            *mut [[c_float; 2]; 40],
            *const [[[c_float; 2]; 32]; 32],
            c_int,
        ) -> c_int,
    >,
    pub(crate) sbr_hf_assemble: Option<
        unsafe extern "C" fn(
            *mut [[c_float; 2]; 64],
            *const [[c_float; 2]; 40],
            *mut SpectralBandReplication,
            *mut SBRData,
            *const c_int,
        ) -> (),
    >,
    pub(crate) sbr_x_gen: Option<
        unsafe extern "C" fn(
            *mut SpectralBandReplication,
            *mut [[c_float; 64]; 38],
            *const [[c_float; 2]; 64],
            *const [[c_float; 2]; 64],
            *const [[c_float; 2]; 40],
            c_int,
        ) -> c_int,
    >,
    pub(crate) sbr_hf_inverse_filter: Option<
        unsafe extern "C" fn(
            *mut SBRDSPContext,
            *mut [c_float; 2],
            *mut [c_float; 2],
            *const [[c_float; 2]; 40],
            c_int,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct SBRData {
    pub(crate) bs_frame_class: c_uint,
    pub(crate) bs_add_harmonic_flag: c_uint,
    pub(crate) bs_num_env: AAC_SIGNE,
    pub(crate) bs_freq_res: [c_uchar; 7],
    pub(crate) bs_num_noise: AAC_SIGNE,
    pub(crate) bs_df_env: [c_uchar; 5],
    pub(crate) bs_df_noise: [c_uchar; 2],
    pub(crate) bs_invf_mode: [[c_uchar; 5]; 2],
    pub(crate) bs_add_harmonic: [c_uchar; 48],
    pub(crate) bs_amp_res: c_uint,
    pub(crate) synthesis_filterbank_samples: [c_float; 2304],
    pub(crate) analysis_filterbank_samples: [c_float; 1312],
    pub(crate) synthesis_filterbank_samples_offset: c_int,
    pub(crate) e_a: [c_int; 2],
    pub(crate) bw_array: [c_float; 5],
    pub(crate) W: [[[[c_float; 2]; 32]; 32]; 2],
    pub(crate) Ypos: c_int,
    pub(crate) Y: [[[[c_float; 2]; 64]; 38]; 2],
    pub(crate) g_temp: [[c_float; 48]; 42],
    pub(crate) q_temp: [[c_float; 48]; 42],
    pub(crate) s_indexmapped: [[c_uchar; 48]; 8],
    pub(crate) env_facs_q: [[c_uchar; 48]; 6],
    pub(crate) env_facs: [[c_float; 48]; 6],
    pub(crate) noise_facs_q: [[c_uchar; 5]; 3],
    pub(crate) noise_facs: [[c_float; 5]; 3],
    pub(crate) t_env: [c_uchar; 8],
    pub(crate) t_env_num_env_old: c_uchar,
    pub(crate) t_q: [c_uchar; 3],
    pub(crate) f_indexnoise: c_uint,
    pub(crate) f_indexsine: c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct SpectrumParameters {
    pub(crate) bs_start_freq: c_uchar,
    pub(crate) bs_stop_freq: c_uchar,
    pub(crate) bs_xover_band: c_uchar,
    pub(crate) bs_freq_scale: c_uchar,
    pub(crate) bs_alter_scale: c_uchar,
    pub(crate) bs_noise_bands: c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct ChannelCoupling {
    pub(crate) coupling_point: CouplingPoint,
    pub(crate) num_coupled: c_int,
    pub(crate) type_0: [RawDataBlockType; 8],
    pub(crate) id_select: [c_int; 8],
    pub(crate) ch_select: [c_int; 8],
    pub(crate) gain: [[c_float; 120]; 16],
}
pub(crate) type RawDataBlockType = c_uint;
pub(crate) const TYPE_END: RawDataBlockType = 7;
pub(crate) const TYPE_FIL: RawDataBlockType = 6;
pub(crate) const TYPE_LFE: RawDataBlockType = 3;
pub(crate) const TYPE_CPE: RawDataBlockType = 1;
pub(crate) const TYPE_SCE: RawDataBlockType = 0;
pub(crate) type CouplingPoint = c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct DynamicRangeControl {
    pub(crate) pce_instance_tag: c_int,
    pub(crate) dyn_rng_sgn: [c_int; 17],
    pub(crate) dyn_rng_ctl: [c_int; 17],
    pub(crate) exclude_mask: [c_int; 64],
    pub(crate) band_incr: c_int,
    pub(crate) interpolation_scheme: c_int,
    pub(crate) band_top: [c_int; 17],
    pub(crate) prog_ref_level: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct AudioFrame {
    pub(crate) pts: c_long,
    pub(crate) duration: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct AudioFrameQueue {
    pub(crate) avctx: *mut AVCodecContext,
    pub(crate) remaining_delay: c_int,
    pub(crate) remaining_samples: c_int,
    pub(crate) frames: *mut AudioFrame,
    pub(crate) frame_count: c_uint,
    pub(crate) frame_alloc: c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct FFPsyBand {
    pub(crate) bits: c_int,
    pub(crate) energy: c_float,
    pub(crate) threshold: c_float,
    pub(crate) spread: c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct FFPsyChannel {
    pub(crate) psy_bands: [FFPsyBand; 128],
    pub(crate) entropy: c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct FFPsyChannelGroup {
    pub(crate) ch: [*mut FFPsyChannel; 20],
    pub(crate) num_ch: c_uchar,
    pub(crate) coupling: [c_uchar; 128],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct FFPsyContext {
    pub(crate) avctx: *mut AVCodecContext,
    pub(crate) model: *const FFPsyModel,
    pub(crate) ch: *mut FFPsyChannel,
    pub(crate) group: *mut FFPsyChannelGroup,
    pub(crate) num_groups: c_int,
    pub(crate) cutoff: c_int,
    pub(crate) bands: *mut *mut c_uchar,
    pub(crate) num_bands: *mut c_int,
    pub(crate) num_lens: c_int,
    pub(crate) bitres: C2RustUnnamed_2,
    pub(crate) model_priv_data: *mut c_void,
}
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct LLSModel {
    pub(crate) covariance: [[c_double; 36]; 36],
    pub(crate) coeff: [[c_double; 32]; 32],
    pub(crate) variance: [c_double; 32],
    pub(crate) indep_count: c_int,
    pub(crate) update_lls: Option<unsafe extern "C" fn(*mut LLSModel, *const c_double) -> ()>,
    pub(crate) evaluate_lls:
        Option<unsafe extern "C" fn(*mut LLSModel, *const c_double, c_int) -> c_double>,
}

impl Default for LLSModel {
    fn default() -> Self {
        Self {
            covariance: [[0.; 36]; 36],
            coeff: Default::default(),
            variance: Default::default(),
            indep_count: Default::default(),
            update_lls: Default::default(),
            evaluate_lls: Default::default(),
        }
    }
}
pub(crate) type FFLPCType = c_int;
pub(crate) const FF_LPC_TYPE_LEVINSON: FFLPCType = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct LPCContext {
    pub(crate) blocksize: c_int,
    pub(crate) max_order: c_int,
    pub(crate) lpc_type: FFLPCType,
    pub(crate) windowed_buffer: *mut c_double,
    pub(crate) windowed_samples: *mut c_double,
    pub(crate) lpc_apply_welch_window:
        Option<unsafe extern "C" fn(*const c_int, ptrdiff_t, *mut c_double) -> ()>,
    pub(crate) lpc_compute_autocorr:
        Option<unsafe extern "C" fn(*const c_double, ptrdiff_t, c_int, *mut c_double) -> ()>,
    pub(crate) lls_models: [LLSModel; 2],
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
#[repr(C)]
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
pub(crate) struct AACPCEInfo {
    pub(crate) layout: AVChannelLayout,
    pub(crate) num_ele: [c_int; 4],
    pub(crate) pairing: [[c_int; 8]; 3],
    pub(crate) index: [[c_int; 8]; 4],
    pub(crate) config_map: [c_uchar; 16],
    pub(crate) reorder_map: [c_uchar; 16],
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
pub(crate) struct FFPsyPreprocessContext {
    pub(crate) avctx: *mut AVCodecContext,
    pub(crate) stereo_att: c_float,
    pub(crate) fcoeffs: *mut FFIIRFilterCoeffs,
    pub(crate) fstate: *mut *mut FFIIRFilterState,
    pub(crate) fiir: FFIIRFilterContext,
}
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub(crate) struct FFIIRFilterContext {
    pub(crate) filter_flt: Option<
        unsafe extern "C" fn(
            *const FFIIRFilterCoeffs,
            *mut FFIIRFilterState,
            c_int,
            *const c_float,
            ptrdiff_t,
            *mut c_float,
            ptrdiff_t,
        ) -> (),
    >,
}
pub(crate) type IIRFilterMode = c_uint;
pub(crate) const FF_FILTER_MODE_HIGHPASS: IIRFilterMode = 1;
pub(crate) const FF_FILTER_MODE_LOWPASS: IIRFilterMode = 0;
pub(crate) type IIRFilterType = c_uint;
pub(crate) const FF_FILTER_TYPE_BUTTERWORTH: IIRFilterType = 2;

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct FFIIRFilterCoeffs {
    pub(crate) order: c_int,
    pub(crate) gain: c_float,
    pub(crate) cx: *mut c_int,
    pub(crate) cy: *mut c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct FFIIRFilterState {
    pub(crate) x: [c_float; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct AVBPrint {
    pub(crate) str_0: *mut c_char,
    pub(crate) len: c_uint,
    pub(crate) size: c_uint,
    pub(crate) size_max: c_uint,
    pub(crate) reserved_internal_buffer: [c_char; 1],
    pub(crate) reserved_padding: [c_char; 1000],
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
