use ffi::codec::{AVCodecContext, AVPacket};

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

    pub fn allocate(self, size: i64) -> Packet<'a> {
        unsafe {
            assert_eq!(ff_alloc_packet(self.avctx, self.avpkt, size), 0);
            *self.allocated = true;
            Packet(&mut *self.avpkt)
        }
    }
}

pub struct Packet<'a>(&'a mut AVPacket);

impl<'a> Packet<'a> {
    pub fn data(&self) -> &'a [u8] {
        unsafe { std::slice::from_raw_parts(self.0.data, self.0.size as usize) }
    }

    pub fn data_mut(&mut self) -> &'a mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.0.data, self.0.size as usize) }
    }

    pub fn truncate(&mut self, len: usize) {
        assert!(
            len <= self.0.size as usize,
            "truncating packet to a larger size"
        );
        self.0.size = len as i32;
    }

    pub fn set_pts(&mut self, pts: i64) {
        self.0.pts = pts;
    }

    pub fn set_duration(&mut self, duration: i64) {
        self.0.duration = duration;
    }
}
