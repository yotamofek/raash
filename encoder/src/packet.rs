use std::{
    marker::PhantomData,
    ptr::{self, NonNull},
};

use bit_writer::BitWriter;
use ffi::codec::{AVCodecContext, AVPacket};
use libc::{c_int, c_long, c_uchar};

use crate::{ff_alloc_packet, impl_fields};

pub struct PacketBuilder<'a> {
    avctx: *mut AVCodecContext,
    avpkt: *mut AVPacket,
    allocated: &'a mut bool,
}

impl<'a> PacketBuilder<'a> {
    pub(super) fn new(
        avctx: *mut AVCodecContext,
        avpkt: *mut AVPacket,
        allocated: &'a mut bool,
    ) -> Self {
        Self {
            avctx,
            avpkt,
            allocated,
        }
    }

    pub fn allocate(self, size: c_long) -> Packet<'a> {
        unsafe {
            assert_eq!(ff_alloc_packet(self.avctx, self.avpkt, size), 0);
            *self.allocated = true;
            Packet::from_ptr(NonNull::new_unchecked(self.avpkt))
        }
    }
}

pub struct Packet<'a>(NonNull<ffi::codec::AVPacket>, PhantomData<&'a mut ()>);

impl Packet<'_> {
    unsafe fn from_ptr(ptr: NonNull<ffi::codec::AVPacket>) -> Self {
        Self(ptr, PhantomData)
    }

    fn len(&self) -> usize {
        self.size().get().try_into().unwrap()
    }

    pub fn data_mut(&mut self) -> *mut [c_uchar] {
        ptr::slice_from_raw_parts_mut(self.data().get(), self.len())
    }

    pub fn truncate(&mut self, len: usize) {
        assert!(len <= self.len(), "truncating packet to a larger size");
        self.size().set(len as c_int);
    }

    pub fn bit_writer(&mut self) -> BitWriter {
        unsafe { BitWriter::from_ptr_slice(self.data_mut()) }
    }
}

impl_fields! {
    struct Packet<'_> {
        data: *mut c_uchar,
        size: c_int,
        /// Duration of this packet in AVStream->time_base units, 0 if unknown.
        /// Equals next_pts - this_pts in presentation order.
        pub duration: c_long,
        /// Presentation timestamp in AVStream->time_base units; the time at which
        /// the decompressed packet will be presented to the user.
        /// Can be AV_NOPTS_VALUE if it is not stored in the file.
        /// pts MUST be larger or equal to dts as presentation cannot happen before
        /// decompression, unless one wants to view hex dumps. Some formats misuse
        /// the terms dts and pts/cts to mean something different. Such timestamps
        /// must be converted to true pts/dts before they are stored in AVPacket.
        pub pts: c_long,
    }
}
