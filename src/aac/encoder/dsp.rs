use ffi::codec::AVCodecContext;
use ffmpeg_src_macro::ffmpeg_src;
use libc::{c_float, c_int, c_void};

use super::ctx::AACEncContext;
use crate::{avutil::tx::av_tx_init, types::AV_TX_FLOAT_MDCT};

#[cold]
#[ffmpeg_src(file = "libavcodec/aacenc.c", lines = 1204..=1221, name = "dsp_init")]
pub(super) unsafe fn init(mut avctx: *mut AVCodecContext, mut s: *mut AACEncContext) -> c_int {
    let mut ret: c_int = 0;
    let mut scale: c_float = 32768.;

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
