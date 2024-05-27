use std::ptr::NonNull;

use ffi::{
    codec::{channel::AVChannelLayout, AVCodecID, Flags},
    num::AVRational,
};
use libc::{c_int, c_long, c_uchar};

use crate::impl_fields;

pub struct CodecContext(NonNull<ffi::codec::AVCodecContext>);

impl CodecContext {
    pub(super) unsafe fn from_ptr(ptr: NonNull<ffi::codec::AVCodecContext>) -> Self {
        Self(ptr)
    }

    // TODO(yotam): remove
    pub fn as_ptr(&self) -> *mut ffi::codec::AVCodecContext {
        self.0.as_ptr()
    }
}

impl_fields! {
    struct CodecContext {
        /// Number of samples per channel in an audio frame.
        pub frame_size: c_int,
        /// Audio only. The number of "priming" samples (padding) inserted by the
        /// encoder at the beginning of the audio. I.e. this number of leading
        /// decoded samples must be discarded by the caller to get the original audio
        /// without leading padding.
        pub initial_padding: c_int,
        /// Audio channel layout.
        pub ch_layout: AVChannelLayout,
        /// Global quality for codecs which cannot change it per frame.
        /// This should be proportional to MPEG-1/2/4 qscale.
        pub global_quality: c_int,
        /// samples per second
        pub sample_rate: c_int,
        /// This is the fundamental unit of time (in seconds) in terms
        /// of which frame timestamps are represented. For fixed-fps content,
        /// timebase should be 1/framerate and timestamp increments should be
        /// identically 1.
        /// This often, but not always is the inverse of the frame rate or field rate
        /// for video. 1/time_base is not the average frame rate if the frame rate is not
        /// constant.
        ///
        /// Like containers, elementary streams also can store timestamps, 1/time_base
        /// is the unit in which these timestamps are specified.
        /// As example of such codec time base see ISO/IEC 14496-2:2001(E)
        /// vop_time_increment_resolution and fixed_vop_rate
        /// (fixed_vop_rate == 0 implies that it is different from the framerate)
        pub time_base: AVRational,
        /// the average bitrate
        pub bit_rate: c_long,
        pub profile: c_int,
        pub strict_std_compliance: c_int,
        pub extradata: *mut c_uchar,
        pub extradata_size: c_int,
        pub flags: Flags,
        pub cutoff: c_int,
        pub codec_id: AVCodecID,
        pub frame_num: c_long,
        pub bit_rate_tolerance: c_int,
    }
}
