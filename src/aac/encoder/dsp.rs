use std::ptr::{addr_of, addr_of_mut};

use ffmpeg_src_macro::ffmpeg_src;
use libc::{c_float, c_int};

use super::ctx::AACEncContext;
use crate::{avutil::tx::av_tx_init, types::AV_TX_FLOAT_MDCT};

#[cold]
#[ffmpeg_src(file = "libavcodec/aacenc.c", lines = 1204..=1221, name = "dsp_init")]
pub(super) unsafe fn init(s: &mut AACEncContext) -> Result<(), c_int> {
    let scale: c_float = 32768.;

    let ret = av_tx_init(
        addr_of_mut!(s.mdct.mdct1024),
        addr_of_mut!(s.mdct.mdct1024_fn),
        AV_TX_FLOAT_MDCT,
        0,
        1024,
        addr_of!(scale).cast(),
        0,
    );
    if ret < 0 {
        return Err(ret);
    }

    let ret = av_tx_init(
        addr_of_mut!(s.mdct.mdct128),
        addr_of_mut!(s.mdct.mdct128_fn),
        AV_TX_FLOAT_MDCT,
        0,
        128,
        addr_of!(scale).cast(),
        0,
    );
    if ret < 0 {
        return Err(ret);
    }

    Ok(())
}
