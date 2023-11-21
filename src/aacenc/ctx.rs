use ffi::{
    class::AVClass,
    codec::{frame::AVFrame, AVCodecContext},
};
use libc::{c_float, c_int, c_uchar, c_uint, c_ushort, c_void};

use crate::types::*;

// TODO: I think this can be opaque?
#[derive(Copy, Clone)]
pub(crate) struct AACContext {
    pub class: *mut AVClass,
    pub avctx: *mut AVCodecContext,
    pub frame: *mut AVFrame,
    pub is_saved: c_int,
    pub che_drc: DynamicRangeControl,
    pub che: [[*mut ChannelElement; 16]; 4],
    pub tag_che_map: [[*mut ChannelElement; 16]; 4],
    pub tags_mapped: c_int,
    pub warned_remapping_once: c_int,
    pub buf_mdct: [c_float; 1024],
    pub mdct120: *mut AVTXContext,
    pub mdct128: *mut AVTXContext,
    pub mdct480: *mut AVTXContext,
    pub mdct512: *mut AVTXContext,
    pub mdct960: *mut AVTXContext,
    pub mdct1024: *mut AVTXContext,
    pub mdct_ltp: *mut AVTXContext,
    pub mdct120_fn: av_tx_fn,
    pub mdct128_fn: av_tx_fn,
    pub mdct480_fn: av_tx_fn,
    pub mdct512_fn: av_tx_fn,
    pub mdct960_fn: av_tx_fn,
    pub mdct1024_fn: av_tx_fn,
    pub mdct_ltp_fn: av_tx_fn,
    pub fdsp: *mut AVFloatDSPContext,
    pub random_state: c_int,
    pub output_element: [*mut SingleChannelElement; 64],
    pub force_dmono_mode: c_int,
    pub dmono_mode: c_int,
    pub output_channel_order: AACOutputChannelOrder,
    pub temp: [c_float; 128],
    pub oc: [OutputConfiguration; 2],
    pub warned_num_aac_frames: c_int,
    pub warned_960_sbr: c_int,
    pub warned_71_wide: c_uint,
    pub warned_gain_control: c_int,
    pub warned_he_aac_mono: c_int,
    pub imdct_and_windowing:
        Option<unsafe extern "C" fn(*mut AACContext, *mut SingleChannelElement) -> ()>,
    pub apply_ltp: Option<unsafe extern "C" fn(*mut AACContext, *mut SingleChannelElement) -> ()>,
    pub apply_tns: Option<
        unsafe extern "C" fn(
            *mut c_float,
            *mut TemporalNoiseShaping,
            *mut IndividualChannelStream,
            c_int,
        ) -> (),
    >,
    pub windowing_and_mdct_ltp: Option<
        unsafe extern "C" fn(
            *mut AACContext,
            *mut c_float,
            *mut c_float,
            *mut IndividualChannelStream,
        ) -> (),
    >,
    pub update_ltp: Option<unsafe extern "C" fn(*mut AACContext, *mut SingleChannelElement) -> ()>,
    pub vector_pow43: Option<unsafe extern "C" fn(*mut c_int, c_int) -> ()>,
    pub subband_scale: Option<
        unsafe extern "C" fn(*mut c_int, *mut c_int, c_int, c_int, c_int, *mut c_void) -> (),
    >,
}

#[derive(Clone)]
#[repr(C)]
pub(crate) struct AACEncContext {
    pub av_class: *mut AVClass,
    pub options: AACEncOptions,
    pub pb: PutBitContext,
    pub mdct1024: *mut AVTXContext,
    pub mdct1024_fn: av_tx_fn,
    pub mdct128: *mut AVTXContext,
    pub mdct128_fn: av_tx_fn,
    pub fdsp: *mut AVFloatDSPContext,
    pub pce: AACPCEInfo,
    pub planar_samples: Box<[[c_float; 3 * 1024]]>,
    pub profile: c_int,
    pub needs_pce: c_int,
    pub lpc: LPCContext,
    pub samplerate_index: c_int,
    pub channels: c_int,
    pub reorder_map: *const c_uchar,
    pub chan_map: *const c_uchar,
    pub cpe: *mut ChannelElement,
    pub psy: FFPsyContext,
    pub psypp: *mut FFPsyPreprocessContext,
    pub coder: *const AACCoefficientsEncoder,
    pub cur_channel: c_int,
    pub random_state: c_int,
    pub lambda: c_float,
    pub last_frame_pb_count: c_int,
    pub lambda_sum: c_float,
    pub lambda_count: c_int,
    pub cur_type: RawDataBlockType,
    pub afq: AudioFrameQueue,
    pub qcoefs: [c_int; 96],
    pub scoefs: [c_float; 1024],
    pub quantize_band_cost_cache_generation: c_ushort,
    pub quantize_band_cost_cache: [[AACQuantizeBandCostCacheEntry; 128]; 256],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct PrivData {
    pub class: *mut AVClass,
    pub options: AACEncOptions,
    pub pb: PutBitContext,
    pub ctx: *mut AACEncContext,
}
