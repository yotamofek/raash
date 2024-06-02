use std::{
    mem::size_of,
    ptr::{self, NonNull},
};

use ffi::codec::channel::AVChannelLayout;
use libc::{c_int, c_long, c_uchar};

use crate::{impl_fields, sample::SampleFormat};

pub struct Frame(NonNull<ffi::codec::frame::AVFrame>);

impl Frame {
    pub(super) unsafe fn from_ptr(ptr: NonNull<ffi::codec::frame::AVFrame>) -> Self {
        Self(ptr)
    }

    /// # Safety
    /// - `chan` must be in-range.
    /// - `T` must be correctly sized & aligned, in accordance to the sample
    ///   type.
    pub unsafe fn get_extended_data_unchecked<T>(&self, chan: usize) -> *const [T] {
        debug_assert_eq!(self.get_format().size(), size_of::<T>());
        debug_assert!(chan < self.ch_layout().get().nb_channels as usize);
        ptr::slice_from_raw_parts(
            (*self.extended_data().get().add(chan)).cast::<T>(),
            self.nb_samples().get() as usize,
        )
    }

    fn get_format(&self) -> SampleFormat {
        self.format().get().try_into().unwrap()
    }
}

impl_fields! {
    struct Frame {
        /// number of audio samples (per channel) described by this frame
        pub nb_samples: c_int,
        /// Presentation timestamp in time_base units (time when frame should be shown to user).
        pub pts: c_long,
        extended_data: *mut *mut c_uchar,
        /// Channel layout of the audio data.
        ch_layout: AVChannelLayout,
        format: c_int,
    }
}
