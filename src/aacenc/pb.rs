use std::{mem::size_of, ptr};

use libc::{c_int, c_long, c_uchar, c_ulong};

use crate::types::{unaligned_32, BitBuf, PutBitContext};

const BUF_BITS: c_int = size_of::<BitBuf>() as c_int * 8;

#[inline]
pub(super) unsafe extern "C" fn init_put_bits(
    mut s: *mut PutBitContext,
    mut buffer: *mut c_uchar,
    mut buffer_size: c_int,
) {
    if buffer_size < 0 as c_int {
        buffer_size = 0 as c_int;
        buffer = ptr::null_mut::<c_uchar>();
    }
    (*s).buf = buffer;
    (*s).buf_end = ((*s).buf).offset(buffer_size as isize);
    (*s).buf_ptr = (*s).buf;
    (*s).bit_left = BUF_BITS;
    (*s).bit_buf = 0 as c_int as BitBuf;
}

#[inline]
pub(super) unsafe extern "C" fn put_bits_count(mut s: *mut PutBitContext) -> c_int {
    (((*s).buf_ptr).offset_from((*s).buf) as c_long * 8 as c_int as c_long + BUF_BITS as c_long
        - (*s).bit_left as c_long) as c_int
}

#[inline]
pub(super) unsafe extern "C" fn put_bytes_output(mut s: *const PutBitContext) -> c_int {
    ((*s).buf_ptr).offset_from((*s).buf) as c_long as c_int
}

#[inline]
pub(super) unsafe extern "C" fn flush_put_bits(mut s: *mut PutBitContext) {
    if (*s).bit_left < BUF_BITS {
        (*s).bit_buf <<= (*s).bit_left;
    }
    while (*s).bit_left < BUF_BITS {
        assert!((*s).buf_ptr < (*s).buf_end);
        let fresh0 = (*s).buf_ptr;
        (*s).buf_ptr = ((*s).buf_ptr).offset(1);
        *fresh0 = ((*s).bit_buf >> BUF_BITS - 8 as c_int) as c_uchar;
        (*s).bit_buf <<= 8 as c_int;
        (*s).bit_left += 8 as c_int;
    }
    (*s).bit_left = BUF_BITS;
    (*s).bit_buf = 0 as c_int as BitBuf;
}

#[inline]
unsafe extern "C" fn put_bits_no_assert(
    mut s: *mut PutBitContext,
    mut n: c_int,
    mut value: BitBuf,
) {
    let mut bit_buf: BitBuf = 0;
    let mut bit_left: c_int = 0;
    bit_buf = (*s).bit_buf;
    bit_left = (*s).bit_left;
    if n < bit_left {
        bit_buf = bit_buf << n | value;
        bit_left -= n;
    } else {
        bit_buf <<= bit_left;
        bit_buf |= value >> n - bit_left;
        if ((*s).buf_end).offset_from((*s).buf_ptr) as c_long as c_ulong
            >= size_of::<BitBuf>() as c_ulong
        {
            (*((*s).buf_ptr as *mut unaligned_32)).l = bit_buf.swap_bytes();
            (*s).buf_ptr = ((*s).buf_ptr).offset(size_of::<BitBuf>() as c_ulong as isize);
        } else {
            panic!("Internal error, put_bits buffer too small");
        }
        bit_left += BUF_BITS - n;
        bit_buf = value;
    }
    (*s).bit_buf = bit_buf;
    (*s).bit_left = bit_left;
}

#[inline]
pub(super) unsafe extern "C" fn put_bits(
    mut s: *mut PutBitContext,
    mut n: c_int,
    mut value: BitBuf,
) {
    put_bits_no_assert(s, n, value);
}

#[inline]
pub(super) unsafe extern "C" fn align_put_bits(mut s: *mut PutBitContext) {
    put_bits(s, (*s).bit_left & 7 as c_int, 0 as c_int as BitBuf);
}
