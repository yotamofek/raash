//! FFI-friendly wrappers for callbacks from ffmpeg to a Rust encoder.

// TODO: catch panics, we can't unwind into C

use std::{mem, ptr::null_mut};

use ffi::codec::{frame::AVFrame, AVCodecContext, AVPacket};
use libc::c_int;

use super::{Encoder, PrivData};
use crate::PacketBuilder;

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
    let mut allocated = false;
    Enc::encode_frame(
        &mut *avctx,
        ctx,
        options,
        &*frame,
        PacketBuilder::new(avctx, avpkt, &mut allocated),
    );

    *got_packet_ptr = c_int::from(allocated);

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
