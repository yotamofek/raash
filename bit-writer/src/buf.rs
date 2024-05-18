use std::{ffi::c_uchar, mem::size_of, ptr};

use crate::BitBuf;

pub(crate) struct Buf {
    start: *mut c_uchar,
    pos: *mut c_uchar,
    end: *mut c_uchar,
}

impl From<*mut [c_uchar]> for Buf {
    fn from(value: *mut [c_uchar]) -> Self {
        Self {
            start: value.as_mut_ptr(),
            pos: value.as_mut_ptr(),
            end: unsafe { value.as_mut_ptr().add(value.len()) },
        }
    }
}

impl Buf {
    pub(crate) unsafe fn write_byte(&mut self, value: c_uchar) {
        assert!(self.pos < self.end);
        ptr::write(self.pos, value);
        self.pos = self.pos.add(1);
    }

    pub(crate) unsafe fn write_bit_buf(&mut self, value: BitBuf) {
        assert!(self.end.offset_from(self.pos) >= size_of::<BitBuf>() as isize);
        ptr::write_unaligned(self.pos.cast(), value);
        self.pos = self.pos.add(size_of::<BitBuf>());
    }

    pub(crate) fn reset(&mut self) {
        self.pos = self.start;
    }

    pub(crate) fn byte_offset(&self) -> usize {
        unsafe { self.pos.offset_from(self.start) as usize }
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::c_uchar;

    use super::*;

    fn buf(buf: &mut [c_uchar]) -> Buf {
        Buf::from(ptr::slice_from_raw_parts_mut(buf.as_mut_ptr(), buf.len()))
    }

    #[test]
    #[cfg(not(target_arch = "x86_64"))]
    fn test_buf_write_bit_buf() {
        let mut orig = [0; size_of::<BitBuf>() * 2 + 1];
        let mut buf = buf(&mut orig[1..]);
        unsafe {
            buf.write_bit_buf(0x12345678);
            buf.write_bit_buf(0x23456789);
        }
        assert_eq!(orig, [0x0, 0x78, 0x56, 0x34, 0x12, 0x89, 0x67, 0x45, 0x23]);
    }

    #[test]
    #[cfg(target_arch = "x86_64")]
    fn test_buf_write_bit_buf() {
        let mut orig = [0; size_of::<BitBuf>() * 2 + 1];
        let mut buf = buf(&mut orig[1..]);
        unsafe {
            buf.write_bit_buf(0x12345678);
            buf.write_bit_buf(0x23456789);
        }
        assert_eq!(
            orig,
            [
                0x0, 0x78, 0x56, 0x34, 0x12, 0x0, 0x0, 0x0, 0x0, 0x89, 0x67, 0x45, 0x23, 0x0, 0x0,
                0x0, 0x0
            ]
        );
    }

    #[test]
    fn test_buf_write_byte() {
        let mut orig = [0; 3];
        let mut buf = buf(&mut orig[1..]);
        unsafe {
            buf.write_byte(0x12);
            buf.write_byte(0x23);
        }
        assert_eq!(orig, [0x0, 0x12, 0x23]);
    }

    #[test]
    #[should_panic]
    fn test_buf_empty_write_bit_buf() {
        let mut orig = [];
        let mut buf = buf(&mut orig);
        unsafe {
            buf.write_bit_buf(0);
        }
    }

    #[test]
    #[should_panic]
    fn test_buf_empty_write_byte() {
        let mut orig = [];
        let mut buf = buf(&mut orig);
        unsafe {
            buf.write_byte(0);
        }
    }

    #[test]
    #[should_panic]
    fn test_buf_write_bit_buf_overflow() {
        let mut orig = [0; 5];
        let mut buf = buf(&mut orig[2..]);
        unsafe {
            buf.write_bit_buf(0);
        }
    }

    #[test]
    #[should_panic]
    fn test_buf_write_byte_overflow() {
        let mut orig = [0; 3];
        let mut buf = buf(&mut orig[2..]);
        unsafe {
            buf.write_byte(0);
            buf.write_byte(1);
        }
    }
}
