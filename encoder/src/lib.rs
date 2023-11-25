#![feature(const_option, const_ptr_is_null, inline_const)]

mod capabilities;

use std::{
    ffi::CStr,
    mem::{self, size_of},
    ptr::{null, null_mut},
};

use ffi::{
    class::{option::AVOption, AVClass, AVClassCategory},
    codec::{
        frame::AVFrame, AVCodec, AVCodecContext, AVCodecID, AVMediaType, AVPacket, AVSampleFormat,
        CodecCallback, FFCodec, FFCodecDefault, FFCodecType,
    },
};
use libc::{c_char, c_int, c_uint, c_void};

use self::capabilities::*;

const AVMEDIA_TYPE_AUDIO: AVMediaType = 1;
const AV_CLASS_CATEGORY_NA: AVClassCategory = 0;
const FF_CODEC_CAP_INIT_CLEANUP: c_uint = 1 << 1;
const FF_CODEC_CB_TYPE_ENCODE: FFCodecType = 3;

pub trait Class {
    const NAME: &'static CStr;
    const OPTIONS: &'static [AVOption];
}

pub enum GotPacket {
    Yes,
    No,
}

pub trait Encoder: Class {
    const NAME: &'static CStr;
    const LONG_NAME: &'static CStr;
    const ID: AVCodecID;
    /// Last element must be `0`.
    const SUPPORTED_SAMPLERATES: &'static [c_int];
    /// Last element must be `-1`.
    const SAMPLE_FMTS: &'static [AVSampleFormat];
    /// Last element must be [`FFCodecDefault::zero`].
    const DEFAULTS: &'static [FFCodecDefault];

    type Ctx;
    type Options;

    fn init(avctx: &mut AVCodecContext, options: &Self::Options) -> Box<Self::Ctx>;
    fn encode_frame(
        avctx: &mut AVCodecContext,
        ctx: &mut Self::Ctx,
        options: &Self::Options,
        avpkt: *mut AVPacket,
        frame: &AVFrame,
    ) -> GotPacket;
    fn close(avctx: &mut AVCodecContext, ctx: Box<Self::Ctx>);
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct PrivData<Enc: Encoder> {
    // `class` and `options` are populated by `avcodec_open2`:
    // https://github.com/FFmpeg/FFmpeg/blob/e9c93009fc34ca9dfcf0c6f2ed90ef1df298abf7/libavcodec/avcodec.c#L186C1-L189C14
    pub class: *mut AVClass,
    pub options: Enc::Options,

    pub ctx: *mut Enc::Ctx,
}

unsafe extern "C" fn av_default_item_name(ptr: *mut c_void) -> *const c_char {
    (**(ptr as *mut *mut AVClass)).class_name
}

pub const fn class<Cls: Class>() -> AVClass {
    AVClass {
        class_name: Cls::NAME.as_ptr(),
        item_name: Some(av_default_item_name),
        option: Cls::OPTIONS.as_ptr(),
        version: (58 << 16) | (32 << 8) | 100,
        log_level_offset_offset: 0,
        parent_log_context_offset: 0,
        category: AV_CLASS_CATEGORY_NA,
        get_category: None,
        query_ranges: None,
        child_next: None,
        child_class_iterate: None,
    }
}

unsafe extern "C" fn init<Enc: Encoder>(avctx: *mut AVCodecContext) -> c_int {
    let priv_data = (*avctx).priv_data as *mut PrivData<Enc>;
    debug_assert!((*priv_data).ctx.is_null());

    let ctx = Enc::init(&mut *avctx, &(*priv_data).options);
    (*priv_data).ctx = Box::into_raw(ctx);

    0
}

unsafe extern "C" fn encode_frame<Enc: Encoder>(
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

unsafe extern "C" fn close<Enc: Encoder>(avctx: *mut AVCodecContext) -> c_int {
    let priv_data = (*avctx).priv_data as *mut PrivData<Enc>;
    debug_assert!(!(*priv_data).ctx.is_null());

    let ctx = mem::replace(&mut (*priv_data).ctx, null_mut());
    let ctx = Box::from_raw(ctx);

    Enc::close(&mut *avctx, ctx);

    0
}

pub const fn encoder<Enc: Encoder>() -> FFCodec {
    assert!(*Enc::SUPPORTED_SAMPLERATES.last().unwrap() == 0);
    assert!(*Enc::SAMPLE_FMTS.last().unwrap() == -1);
    {
        let last_default = Enc::DEFAULTS.last().unwrap();
        // `PartialEq` is not const-implemented, so we have to do this manually
        assert!(last_default.key.is_null() && last_default.value.is_null());
    }

    // we have to use a const inline block to ensure `class` is promoted to a static
    // (otherwise this will create a dangling pointer)
    let class = &const { class::<Enc>() };

    FFCodec {
        p: AVCodec {
            name: <Enc as Encoder>::NAME.as_ptr(),
            long_name: <Enc as Encoder>::LONG_NAME.as_ptr(),
            type_0: AVMEDIA_TYPE_AUDIO,
            id: Enc::ID,
            capabilities: AV_CODEC_CAP_DR1 | AV_CODEC_CAP_DELAY | AV_CODEC_CAP_SMALL_LAST_FRAME,
            max_lowres: 0,
            supported_framerates: null(),
            pix_fmts: null(),
            supported_samplerates: Enc::SUPPORTED_SAMPLERATES.as_ptr(),
            sample_fmts: Enc::SAMPLE_FMTS.as_ptr(),
            channel_layouts: null(),
            priv_class: class,
            profiles: null(),
            wrapper_name: null(),
            ch_layouts: null(),
        },
        caps_internal_cb_type: {
            (FF_CODEC_CAP_INIT_CLEANUP | FF_CODEC_CB_TYPE_ENCODE << 29).to_le_bytes()
        },
        priv_data_size: size_of::<PrivData<Enc>>() as c_int,
        update_thread_context: None,
        update_thread_context_for_user: None,
        defaults: Enc::DEFAULTS.as_ptr(),
        init_static_data: None,
        init: Some(init::<Enc>),
        cb: CodecCallback {
            encode: Some(encode_frame::<Enc>),
        },
        close: Some(close::<Enc>),
        flush: None,
        bsfs: null(),
        hw_configs: null(),
        codec_tags: null(),
    }
}
