use ffi::codec::AVCodecContext;
use libc::{c_float, c_int};

use super::{encode_window_bands_info, quantizers, search_for_quantizers_fast, trellis};
use crate::{aacenc::ctx::AACEncContext, types::SingleChannelElement};

pub(crate) trait CoeffsEncoder {
    unsafe fn search_for_quantizers(
        &self,
        _: *mut AVCodecContext,
        _: *mut AACEncContext,
        _: *mut SingleChannelElement,
        _: c_float,
    );

    unsafe fn encode_window_bands_info(
        &self,
        _: *mut AACEncContext,
        _: *mut SingleChannelElement,
        _: c_int,
        _: c_int,
        _: c_float,
    );
}

pub(crate) struct TwoLoop;
pub(crate) struct Fast;

impl CoeffsEncoder for TwoLoop {
    unsafe fn search_for_quantizers(
        &self,
        avctx: *mut AVCodecContext,
        s: *mut AACEncContext,
        sce: *mut SingleChannelElement,
        lambda: c_float,
    ) {
        quantizers::twoloop::search(avctx, s, sce, lambda)
    }

    unsafe fn encode_window_bands_info(
        &self,
        s: *mut AACEncContext,
        sce: *mut SingleChannelElement,
        win: c_int,
        group_len: c_int,
        lambda: c_float,
    ) {
        trellis::codebook_rate(s, sce, win, group_len, lambda)
    }
}

impl CoeffsEncoder for Fast {
    unsafe fn search_for_quantizers(
        &self,
        avctx: *mut AVCodecContext,
        s: *mut AACEncContext,
        sce: *mut SingleChannelElement,
        lambda: c_float,
    ) {
        search_for_quantizers_fast(avctx, s, sce, lambda)
    }

    unsafe fn encode_window_bands_info(
        &self,
        s: *mut AACEncContext,
        sce: *mut SingleChannelElement,
        win: c_int,
        group_len: c_int,
        lambda: c_float,
    ) {
        trellis::codebook_rate(s, sce, win, group_len, lambda)
    }
}
