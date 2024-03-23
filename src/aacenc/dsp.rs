use ffi::codec::AVCodecContext;
use libc::{c_float, c_int, c_void};

use super::{avpriv_float_dsp_alloc, ctx::AACEncContext};
use crate::{avutil::tx::av_tx_init, types::AV_TX_FLOAT_MDCT};

#[cold]
/// Source: [libavcodec/aacenc.c](https://github.com/FFmpeg/FFmpeg/blob/2d9ed64859c9887d0504cd71dbd5b2c15e14251a/libavcodec/aacenc.c#L1204C4-L1221)
pub(super) unsafe fn init(mut avctx: *mut AVCodecContext, mut s: *mut AACEncContext) -> c_int {
    let mut ret: c_int = 0;
    let mut scale: c_float = 32768.0f32;

    (*s).fdsp = avpriv_float_dsp_alloc((*avctx).flags & (1) << 23);
    if ((*s).fdsp).is_null() {
        return -12;
    }
    ret = av_tx_init(
        &mut (*s).mdct1024,
        &mut (*s).mdct1024_fn,
        AV_TX_FLOAT_MDCT,
        0,
        1024,
        &mut scale as *mut c_float as *const c_void,
        0,
    );
    if ret < 0 {
        return ret;
    }
    ret = av_tx_init(
        &mut (*s).mdct128,
        &mut (*s).mdct128_fn,
        AV_TX_FLOAT_MDCT,
        0,
        128,
        &mut scale as *mut c_float as *const c_void,
        0,
    );
    if ret < 0 {
        return ret;
    }
    0
}
