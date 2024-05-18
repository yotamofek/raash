use std::{
    marker::PhantomData,
    ptr::{self, addr_of, addr_of_mut},
};

use bit_writer::BitWriter;
use ffi::codec::{AVCodecContext, AVPacket};
use libc::{c_int, c_long, c_uchar};

use crate::ff_alloc_packet;

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
            Packet(self.avpkt, PhantomData)
        }
    }
}

pub struct Packet<'a>(*mut AVPacket, PhantomData<&'a mut ()>);

impl<'a> Packet<'a> {
    fn data_ptr(&self) -> *mut c_uchar {
        unsafe { ptr::read(addr_of!((*self.0).data)) }
    }

    fn len(&self) -> usize {
        unsafe { ptr::read(addr_of!((*self.0).size)) }
            .try_into()
            .unwrap()
    }

    pub fn data_mut(&mut self) -> *mut [c_uchar] {
        ptr::slice_from_raw_parts_mut(self.data_ptr(), self.len())
    }

    pub fn truncate(&mut self, len: usize) {
        assert!(len <= self.len(), "truncating packet to a larger size");

        unsafe { ptr::write(addr_of_mut!((*self.0).size), len as c_int) };
    }

    pub fn set_pts(&mut self, pts: c_long) {
        unsafe { ptr::write(addr_of_mut!((*self.0).pts), pts) };
    }

    pub fn set_duration(&mut self, duration: c_long) {
        unsafe { ptr::write(addr_of_mut!((*self.0).duration), duration) };
    }

    pub fn bit_writer(&mut self) -> BitWriter {
        unsafe { BitWriter::from_ptr_slice(self.data_mut()) }
    }
}
