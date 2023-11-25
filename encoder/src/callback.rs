//! FFI-friendly wrappers for callbacks from ffmpeg to a Rust encoder.

use std::{mem, ptr::null_mut};

use ffi::codec::{frame::AVFrame, AVCodecContext, AVPacket};
use libc::c_int;

use super::{Encoder, PrivData};
use crate::GotPacket;

pub(super) unsafe extern "C" fn init<Enc: Encoder>(avctx: *mut AVCodecContext) -> c_int {
    let priv_data = (*avctx).priv_data as *mut PrivData<Enc>;
    debug_assert!((*priv_data).ctx.is_null());

    let ctx = Enc::init(&mut *avctx, &(*priv_data).options);
    (*priv_data).ctx = Box::into_raw(ctx);

    0
}

pub(super) unsafe extern "C" fn encode_frame<Enc: Encoder>(
    avctx: *mut AVCodecContext,
    avpkt: *mut AVPacket,
    frame: *const AVFrame,
    got_packet_ptr: *mut c_int,
) -> c_int {
    let priv_data = (*avctx).priv_data as *mut PrivData<Enc>;
    debug_assert!(!(*priv_data).ctx.is_null());

    let ctx = &mut *(*priv_data).ctx;
    let options = &(*priv_data).options;
    let got_packet = Enc::encode_frame(&mut *avctx, ctx, options, avpkt, &*frame);

    *got_packet_ptr = c_int::from(matches!(got_packet, GotPacket::Yes));

    0
}

pub(super) unsafe extern "C" fn close<Enc: Encoder>(avctx: *mut AVCodecContext) -> c_int {
    let priv_data = (*avctx).priv_data as *mut PrivData<Enc>;
    debug_assert!(!(*priv_data).ctx.is_null());

    let ctx = mem::replace(&mut (*priv_data).ctx, null_mut());
    let ctx = Box::from_raw(ctx);

    Enc::close(&mut *avctx, ctx);

    0
}