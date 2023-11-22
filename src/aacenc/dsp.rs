use ffi::codec::AVCodecContext;
use libc::{c_float, c_int, c_void};

use super::{avpriv_float_dsp_alloc, ctx::AACEncContext};
use crate::{avutil::tx::av_tx_init, types::AV_TX_FLOAT_MDCT};

#[cold]
pub(super) unsafe fn init(mut avctx: *mut AVCodecContext, mut s: *mut AACEncContext) -> c_int {
    let mut ret: c_int = 0 as c_int;
    let mut scale: c_float = 32768.0f32;

    (*s).fdsp = avpriv_float_dsp_alloc((*avctx).flags & (1 as c_int) << 23 as c_int);
    if ((*s).fdsp).is_null() {
        return -(12 as c_int);
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
    if ret < 0 as c_int {
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
    if ret < 0 as c_int {
        return ret;
    }
    0 as c_int
}
