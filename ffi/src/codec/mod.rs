pub mod channel;
pub mod frame;
pub mod profile;
pub mod subtitle;

use std::{ffi::CStr, ptr};

use c2rust_bitfields::BitfieldStruct;
use libc::{c_char, c_float, c_int, c_long, c_uchar, c_uint, c_ulong, c_ushort, c_void};

use self::{channel::AVChannelLayout, frame::AVFrame, subtitle::AVSubtitle};
use super::{class::AVClass, num::AVRational};

extern "C" {
    pub type AVCodecInternal;
    pub type AVBuffer;
    pub type AVDictionary;
    pub type AVCodecDescriptor;
    pub type AVCodecHWConfigInternal;
}

pub type AVMediaType = c_int;
pub type AVCodecID = c_uint;
pub type AVSampleFormat = c_int;
pub type AVPixelFormat = c_int;
pub type AVPictureType = c_uint;
pub type AVDiscard = c_int;
pub type AVFieldOrder = c_uint;
pub type AVAudioServiceType = c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVBufferRef {
    pub buffer: *mut AVBuffer,
    pub data: *mut c_uchar,
    pub size: c_ulong,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVProfile {
    pub profile: c_int,
    pub name: *const c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodec {
    pub name: *const c_char,
    pub long_name: *const c_char,
    pub type_0: AVMediaType,
    pub id: AVCodecID,
    pub capabilities: c_int,
    pub max_lowres: c_uchar,
    pub supported_framerates: *const AVRational,
    pub pix_fmts: *const AVPixelFormat,
    pub supported_samplerates: *const c_int,
    pub sample_fmts: *const AVSampleFormat,
    pub channel_layouts: *const c_ulong,
    pub priv_class: *const AVClass,
    pub profiles: *const AVProfile,
    pub wrapper_name: *const c_char,
    pub ch_layouts: *const AVChannelLayout,
}

pub type Execute2Fn = unsafe extern "C" fn(
    *mut AVCodecContext,
    Option<unsafe extern "C" fn(*mut AVCodecContext, *mut c_void, c_int, c_int) -> c_int>,
    *mut c_void,
    *mut c_int,
    c_int,
) -> c_int;

pub type ExecuteFn = Option<
    unsafe extern "C" fn(
        *mut AVCodecContext,
        Option<unsafe extern "C" fn(*mut AVCodecContext, *mut c_void) -> c_int>,
        *mut c_void,
        *mut c_int,
        c_int,
        c_int,
    ) -> c_int,
>;

#[derive(Clone)]
#[repr(C)]
pub struct AVCodecContext {
    pub av_class: *const AVClass,
    pub log_level_offset: c_int,
    pub codec_type: AVMediaType,
    pub codec: *const AVCodec,
    pub codec_id: AVCodecID,
    pub codec_tag: c_uint,
    // TODO: make this generic?
    pub priv_data: *mut c_void,
    pub internal: *mut AVCodecInternal,
    pub opaque: *mut c_void,
    pub bit_rate: c_long,
    pub bit_rate_tolerance: c_int,
    pub global_quality: c_int,
    pub compression_level: c_int,
    pub flags: c_int,
    pub flags2: c_int,
    pub extradata: *mut c_uchar,
    pub extradata_size: c_int,
    pub time_base: AVRational,
    pub ticks_per_frame: c_int,
    pub delay: c_int,
    pub width: c_int,
    pub height: c_int,
    pub coded_width: c_int,
    pub coded_height: c_int,
    pub gop_size: c_int,
    pub pix_fmt: AVPixelFormat,
    pub draw_horiz_band: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *const AVFrame,
            *mut c_int,
            c_int,
            c_int,
            c_int,
        ) -> (),
    >,
    pub get_format:
        Option<unsafe extern "C" fn(*mut AVCodecContext, *const AVPixelFormat) -> AVPixelFormat>,
    pub max_b_frames: c_int,
    pub b_quant_factor: c_float,
    pub b_quant_offset: c_float,
    pub has_b_frames: c_int,
    pub i_quant_factor: c_float,
    pub i_quant_offset: c_float,
    pub lumi_masking: c_float,
    pub temporal_cplx_masking: c_float,
    pub spatial_cplx_masking: c_float,
    pub p_masking: c_float,
    pub dark_masking: c_float,
    pub slice_count: c_int,
    pub slice_offset: *mut c_int,
    pub sample_aspect_ratio: AVRational,
    pub me_cmp: c_int,
    pub me_sub_cmp: c_int,
    pub mb_cmp: c_int,
    pub ildct_cmp: c_int,
    pub dia_size: c_int,
    pub last_predictor_count: c_int,
    pub me_pre_cmp: c_int,
    pub pre_dia_size: c_int,
    pub me_subpel_quality: c_int,
    pub me_range: c_int,
    pub slice_flags: c_int,
    pub mb_decision: c_int,
    pub intra_matrix: *mut c_ushort,
    pub inter_matrix: *mut c_ushort,
    pub intra_dc_precision: c_int,
    pub skip_top: c_int,
    pub skip_bottom: c_int,
    pub mb_lmin: c_int,
    pub mb_lmax: c_int,
    pub bidir_refine: c_int,
    pub keyint_min: c_int,
    pub refs: c_int,
    pub mv0_threshold: c_int,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub colorspace: AVColorSpace,
    pub color_range: AVColorRange,
    pub chroma_sample_location: AVChromaLocation,
    pub slices: c_int,
    pub field_order: AVFieldOrder,
    pub sample_rate: c_int,
    pub channels: c_int,
    pub sample_fmt: AVSampleFormat,
    pub frame_size: c_int,
    pub frame_number: c_int,
    pub block_align: c_int,
    pub cutoff: c_int,
    pub channel_layout: c_ulong,
    pub request_channel_layout: c_ulong,
    pub audio_service_type: AVAudioServiceType,
    pub request_sample_fmt: AVSampleFormat,
    pub get_buffer2:
        Option<unsafe extern "C" fn(*mut AVCodecContext, *mut AVFrame, c_int) -> c_int>,
    pub qcompress: c_float,
    pub qblur: c_float,
    pub qmin: c_int,
    pub qmax: c_int,
    pub max_qdiff: c_int,
    pub rc_buffer_size: c_int,
    pub rc_override_count: c_int,
    pub rc_override: *mut RcOverride,
    pub rc_max_rate: c_long,
    pub rc_min_rate: c_long,
    pub rc_max_available_vbv_use: c_float,
    pub rc_min_vbv_overflow_use: c_float,
    pub rc_initial_buffer_occupancy: c_int,
    pub trellis: c_int,
    pub stats_out: *mut c_char,
    pub stats_in: *mut c_char,
    pub workaround_bugs: c_int,
    pub strict_std_compliance: c_int,
    pub error_concealment: c_int,
    pub debug: c_int,
    pub err_recognition: c_int,
    pub reordered_opaque: c_long,
    pub hwaccel: *const AVHWAccel,
    pub hwaccel_context: *mut c_void,
    pub error: [c_ulong; 8],
    pub dct_algo: c_int,
    pub idct_algo: c_int,
    pub bits_per_coded_sample: c_int,
    pub bits_per_raw_sample: c_int,
    pub lowres: c_int,
    pub thread_count: c_int,
    pub thread_type: c_int,
    pub active_thread_type: c_int,
    pub execute: ExecuteFn,
    pub execute2: Option<Execute2Fn>,
    pub nsse_weight: c_int,
    pub profile: c_int,
    pub level: c_int,
    pub skip_loop_filter: AVDiscard,
    pub skip_idct: AVDiscard,
    pub skip_frame: AVDiscard,
    pub subtitle_header: *mut c_uchar,
    pub subtitle_header_size: c_int,
    pub initial_padding: c_int,
    pub framerate: AVRational,
    pub sw_pix_fmt: AVPixelFormat,
    pub pkt_timebase: AVRational,
    pub codec_descriptor: *const AVCodecDescriptor,
    pub pts_correction_num_faulty_pts: c_long,
    pub pts_correction_num_faulty_dts: c_long,
    pub pts_correction_last_pts: c_long,
    pub pts_correction_last_dts: c_long,
    pub sub_charenc: *mut c_char,
    pub sub_charenc_mode: c_int,
    pub skip_alpha: c_int,
    pub seek_preroll: c_int,
    pub chroma_intra_matrix: *mut c_ushort,
    pub dump_separator: *mut c_uchar,
    pub codec_whitelist: *mut c_char,
    pub properties: c_uint,
    pub coded_side_data: *mut AVPacketSideData,
    pub nb_coded_side_data: c_int,
    pub hw_frames_ctx: *mut AVBufferRef,
    pub trailing_padding: c_int,
    pub max_pixels: c_long,
    pub hw_device_ctx: *mut AVBufferRef,
    pub hwaccel_flags: c_int,
    pub apply_cropping: c_int,
    pub extra_hw_frames: c_int,
    pub discard_damaged_percentage: c_int,
    pub max_samples: c_long,
    pub export_side_data: c_int,
    pub get_encode_buffer:
        Option<unsafe extern "C" fn(*mut AVCodecContext, *mut AVPacket, c_int) -> c_int>,
    pub ch_layout: AVChannelLayout,
    pub frame_num: c_long,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVPacket {
    pub buf: *mut AVBufferRef,
    pub pts: c_long,
    pub dts: c_long,
    pub data: *mut c_uchar,
    pub size: c_int,
    pub stream_index: c_int,
    pub flags: c_int,
    pub side_data: *mut AVPacketSideData,
    pub side_data_elems: c_int,
    pub duration: c_long,
    pub pos: c_long,
    pub opaque: *mut c_void,
    pub opaque_ref: *mut AVBufferRef,
    pub time_base: AVRational,
}

pub type AVPacketSideDataType = c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVPacketSideData {
    pub data: *mut c_uchar,
    pub size: c_ulong,
    pub type_0: AVPacketSideDataType,
}

pub type AVColorPrimaries = c_uint;
pub type AVColorTransferCharacteristic = c_uint;
pub type AVColorSpace = c_uint;
pub type AVColorRange = c_uint;
pub type AVChromaLocation = c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVHWAccel {
    pub name: *const c_char,
    pub type_0: AVMediaType,
    pub id: AVCodecID,
    pub pix_fmt: AVPixelFormat,
    pub capabilities: c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct RcOverride {
    pub start_frame: c_int,
    pub end_frame: c_int,
    pub qscale: c_int,
    pub quality_factor: c_float,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFCodecDefault {
    pub key: *const c_char,
    pub value: *const c_char,
}

impl FFCodecDefault {
    pub const fn new(key: &'static CStr, value: &'static CStr) -> Self {
        Self {
            key: key.as_ptr(),
            value: value.as_ptr(),
        }
    }

    pub const fn null() -> Self {
        Self {
            key: ptr::null(),
            value: ptr::null(),
        }
    }
}

pub type FFCodecType = c_uint;
pub const FF_CODEC_CB_TYPE_ENCODE: FFCodecType = 3;

#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct FFCodec {
    pub p: AVCodec,
    #[bitfield(name = "caps_internal", ty = "c_uint", bits = "0..=28")]
    #[bitfield(name = "cb_type", ty = "c_uint", bits = "29..=31")]
    pub caps_internal_cb_type: [u8; 4],
    pub priv_data_size: c_int,
    pub update_thread_context:
        Option<unsafe extern "C" fn(*mut AVCodecContext, *const AVCodecContext) -> c_int>,
    pub update_thread_context_for_user:
        Option<unsafe extern "C" fn(*mut AVCodecContext, *const AVCodecContext) -> c_int>,
    pub defaults: *const FFCodecDefault,
    pub init_static_data: Option<unsafe extern "C" fn(*mut FFCodec) -> ()>,
    pub init: Option<unsafe extern "C" fn(*mut AVCodecContext) -> c_int>,
    pub cb: CodecCallback,
    pub close: Option<unsafe extern "C" fn(*mut AVCodecContext) -> c_int>,
    pub flush: Option<unsafe extern "C" fn(*mut AVCodecContext) -> ()>,
    pub bsfs: *const c_char,
    pub hw_configs: *const *const AVCodecHWConfigInternal,
    pub codec_tags: *const c_uint,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union CodecCallback {
    pub decode: Option<
        unsafe extern "C" fn(*mut AVCodecContext, *mut AVFrame, *mut c_int, *mut AVPacket) -> c_int,
    >,
    pub decode_sub: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut AVSubtitle,
            *mut c_int,
            *const AVPacket,
        ) -> c_int,
    >,
    pub receive_frame: Option<unsafe extern "C" fn(*mut AVCodecContext, *mut AVFrame) -> c_int>,
    pub encode: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut AVPacket,
            *const AVFrame,
            *mut c_int,
        ) -> c_int,
    >,
    pub encode_sub: Option<
        unsafe extern "C" fn(*mut AVCodecContext, *mut c_uchar, c_int, *const AVSubtitle) -> c_int,
    >,
    pub receive_packet: Option<unsafe extern "C" fn(*mut AVCodecContext, *mut AVPacket) -> c_int>,
}
