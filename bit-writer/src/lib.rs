#![feature(slice_ptr_get)]

mod buf;

use std::{
    ffi::{c_int, c_uchar},
    marker::PhantomData,
};

use ffmpeg_src_macro::ffmpeg_src;

use crate::buf::Buf;

#[cfg(target_arch = "x86_64")]
pub type BitBuf = std::ffi::c_ulong;
#[cfg(not(target_arch = "x86_64"))]
pub type BitBuf = std::ffi::c_uint;

const BUF_BITS: u8 = BitBuf::BITS as _;

#[ffmpeg_src(file = "libavcodec/put_bits.h", lines = 50..=54, name = "PutBitContext")]
pub struct BitWriter<'a> {
    buf: Buf,
    _marker: PhantomData<&'a mut [c_uchar]>,
    bits_left: u8,
    bit_buf: BitBuf,
}

impl<'a> BitWriter<'a> {
    pub fn new(bytes: &'a mut [c_uchar]) -> Self {
        unsafe { Self::from_ptr_slice(bytes) }
    }

    /// # Safety
    /// The byte array must be writable for the duration of lifetime `'a`.
    pub unsafe fn from_ptr_slice(bytes: *mut [c_uchar]) -> Self {
        Self {
            buf: bytes.into(),
            _marker: PhantomData,
            bits_left: BUF_BITS,
            bit_buf: 0,
        }
    }

    /// Write up to 31 bits into a bitstream.
    #[ffmpeg_src(file = "libavcodec/put_bits.h", lines = 243..=251, name = "put_bits")]
    pub fn put(&mut self, bits: u8, value: BitBuf) {
        debug_assert!(bits <= 31 && value < (1 << bits));

        let Self {
            buf,
            bits_left,
            bit_buf,
            ..
        } = self;

        if bits < *bits_left {
            *bit_buf = *bit_buf << bits | value;
            *bits_left -= bits;
        } else {
            *bit_buf <<= *bits_left;
            *bit_buf |= value >> (bits - *bits_left);

            unsafe { buf.write_bit_buf(bit_buf.swap_bytes()) };

            *bits_left += BUF_BITS - bits;
            *bit_buf = value;
        }
    }

    #[ffmpeg_src(file = "libavcodec/put_bits.h", lines = 281..=286, name = "put_sbits")]
    pub fn put_signed(&mut self, bits: u8, value: c_int) {
        /// Clear high bits from an unsigned integer starting with specific bit
        /// position.
        #[ffmpeg_src(file = "libavutil/common.h", lines = 285..=294)]
        const fn mod_uintp2_c(a: BitBuf, p: u8) -> BitBuf {
            a & ((1 as BitBuf) << p).wrapping_sub(1)
        }

        self.put(bits, mod_uintp2_c(value as BitBuf, bits));
    }

    /// Pad the bitstream with zeros up to the next byte boundary.
    #[ffmpeg_src(file = "libavcodec/put_bits.h", lines = 417..=423, name = "align_put_bits")]
    pub fn align(&mut self) {
        self.put(self.bits_left & 7, 0);
    }

    /// Pad the end of the output stream with zeros.
    #[ffmpeg_src(file = "libavcodec/put_bits.h", lines = 140..=162, name = "flush_put_bits")]
    pub fn flush(&mut self) {
        let Self {
            buf,
            bits_left,
            bit_buf,
            ..
        } = self;

        if *bits_left < BUF_BITS {
            *bit_buf <<= *bits_left;
        }
        while *bits_left < BUF_BITS {
            unsafe { buf.write_byte((*bit_buf >> (BUF_BITS - 8)) as _) };

            *bit_buf <<= 8;
            *bits_left += 8;
        }

        *bits_left = BUF_BITS;
        *bit_buf = 0;
    }

    pub fn clear(&mut self) {
        let Self {
            buf,
            bits_left,
            bit_buf,
            ..
        } = self;

        buf.reset();
        *bits_left = BUF_BITS;
        *bit_buf = 0;
    }

    /// Returns the total number of bits written to the bitstream.
    #[ffmpeg_src(file = "libavcodec/put_bits.h", lines = 77..=83, name = "put_bits_count")]
    pub fn bits_written(&self) -> usize {
        self.buf.byte_offset() * 8 + usize::from(BUF_BITS - self.bits_left)
    }

    /// Returns the number of bytes output so far; may only be called when the
    /// `BitWriter` is freshly initialized or flushed.
    #[ffmpeg_src(file = "libavcodec/put_bits.h", lines = 85..=93, name = "put_bytes_output")]
    pub fn total_bytes_written(&self) -> usize {
        debug_assert_eq!(self.bits_left, BUF_BITS);
        self.buf.byte_offset()
    }
}
