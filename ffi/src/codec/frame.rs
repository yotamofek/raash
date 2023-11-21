use libc::{c_int, c_long, c_uchar, c_uint, c_ulong, c_void};

use super::{
    channel::AVChannelLayout, AVBufferRef, AVChromaLocation, AVColorPrimaries, AVColorRange,
    AVColorSpace, AVColorTransferCharacteristic, AVDictionary, AVPictureType,
};
use crate::num::AVRational;

pub type AVFrameSideDataType = c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFrameSideData {
    pub type_0: AVFrameSideDataType,
    pub data: *mut c_uchar,
    pub size: c_ulong,
    pub metadata: *mut AVDictionary,
    pub buf: *mut AVBufferRef,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFrame {
    pub data: [*mut c_uchar; 8],
    pub linesize: [c_int; 8],
    pub extended_data: *mut *mut c_uchar,
    pub width: c_int,
    pub height: c_int,
    pub nb_samples: c_int,
    pub format: c_int,
    pub key_frame: c_int,
    pub pict_type: AVPictureType,
    pub sample_aspect_ratio: AVRational,
    pub pts: c_long,
    pub pkt_dts: c_long,
    pub time_base: AVRational,
    pub coded_picture_number: c_int,
    pub display_picture_number: c_int,
    pub quality: c_int,
    pub opaque: *mut c_void,
    pub repeat_pict: c_int,
    pub interlaced_frame: c_int,
    pub top_field_first: c_int,
    pub palette_has_changed: c_int,
    pub reordered_opaque: c_long,
    pub sample_rate: c_int,
    pub channel_layout: c_ulong,
    pub buf: [*mut AVBufferRef; 8],
    pub extended_buf: *mut *mut AVBufferRef,
    pub nb_extended_buf: c_int,
    pub side_data: *mut *mut AVFrameSideData,
    pub nb_side_data: c_int,
    pub flags: c_int,
    pub color_range: AVColorRange,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub colorspace: AVColorSpace,
    pub chroma_location: AVChromaLocation,
    pub best_effort_timestamp: c_long,
    pub pkt_pos: c_long,
    pub pkt_duration: c_long,
    pub metadata: *mut AVDictionary,
    pub decode_error_flags: c_int,
    pub channels: c_int,
    pub pkt_size: c_int,
    pub hw_frames_ctx: *mut AVBufferRef,
    pub opaque_ref: *mut AVBufferRef,
    pub crop_top: c_ulong,
    pub crop_bottom: c_ulong,
    pub crop_left: c_ulong,
    pub crop_right: c_ulong,
    pub private_ref: *mut AVBufferRef,
    pub ch_layout: AVChannelLayout,
    pub duration: c_long,
}
