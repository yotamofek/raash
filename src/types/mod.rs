use c2rust_bitfields::BitfieldStruct;

extern "C" {
    pub type AVDictionary;
    pub type AVBuffer;
    pub type AVCodecDescriptor;
    pub type AVCodecInternal;
    pub type AVCodecHWConfigInternal;
    pub type AVTXContext;
}

pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type intptr_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type AVChannel = libc::c_int;
pub const AV_CHAN_AMBISONIC_END: AVChannel = 2047;
pub const AV_CHAN_AMBISONIC_BASE: AVChannel = 1024;
pub const AV_CHAN_UNKNOWN: AVChannel = 768;
pub const AV_CHAN_UNUSED: AVChannel = 512;
pub const AV_CHAN_BOTTOM_FRONT_RIGHT: AVChannel = 40;
pub const AV_CHAN_BOTTOM_FRONT_LEFT: AVChannel = 39;
pub const AV_CHAN_BOTTOM_FRONT_CENTER: AVChannel = 38;
pub const AV_CHAN_TOP_SIDE_RIGHT: AVChannel = 37;
pub const AV_CHAN_TOP_SIDE_LEFT: AVChannel = 36;
pub const AV_CHAN_LOW_FREQUENCY_2: AVChannel = 35;
pub const AV_CHAN_SURROUND_DIRECT_RIGHT: AVChannel = 34;
pub const AV_CHAN_SURROUND_DIRECT_LEFT: AVChannel = 33;
pub const AV_CHAN_WIDE_RIGHT: AVChannel = 32;
pub const AV_CHAN_WIDE_LEFT: AVChannel = 31;
pub const AV_CHAN_STEREO_RIGHT: AVChannel = 30;
pub const AV_CHAN_STEREO_LEFT: AVChannel = 29;
pub const AV_CHAN_TOP_BACK_RIGHT: AVChannel = 17;
pub const AV_CHAN_TOP_BACK_CENTER: AVChannel = 16;
pub const AV_CHAN_TOP_BACK_LEFT: AVChannel = 15;
pub const AV_CHAN_TOP_FRONT_RIGHT: AVChannel = 14;
pub const AV_CHAN_TOP_FRONT_CENTER: AVChannel = 13;
pub const AV_CHAN_TOP_FRONT_LEFT: AVChannel = 12;
pub const AV_CHAN_TOP_CENTER: AVChannel = 11;
pub const AV_CHAN_SIDE_RIGHT: AVChannel = 10;
pub const AV_CHAN_SIDE_LEFT: AVChannel = 9;
pub const AV_CHAN_BACK_CENTER: AVChannel = 8;
pub const AV_CHAN_FRONT_RIGHT_OF_CENTER: AVChannel = 7;
pub const AV_CHAN_FRONT_LEFT_OF_CENTER: AVChannel = 6;
pub const AV_CHAN_BACK_RIGHT: AVChannel = 5;
pub const AV_CHAN_BACK_LEFT: AVChannel = 4;
pub const AV_CHAN_LOW_FREQUENCY: AVChannel = 3;
pub const AV_CHAN_FRONT_CENTER: AVChannel = 2;
pub const AV_CHAN_FRONT_RIGHT: AVChannel = 1;
pub const AV_CHAN_FRONT_LEFT: AVChannel = 0;
pub const AV_CHAN_NONE: AVChannel = -1;
pub type AVChannelOrder = libc::c_uint;
pub const AV_CHANNEL_ORDER_AMBISONIC: AVChannelOrder = 3;
pub const AV_CHANNEL_ORDER_CUSTOM: AVChannelOrder = 2;
pub const AV_CHANNEL_ORDER_NATIVE: AVChannelOrder = 1;
pub const AV_CHANNEL_ORDER_UNSPEC: AVChannelOrder = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVChannelCustom {
    pub id: AVChannel,
    pub name: [libc::c_char; 16],
    pub opaque: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVChannelLayout {
    pub order: AVChannelOrder,
    pub nb_channels: libc::c_int,
    pub u: C2RustUnnamed,
    pub opaque: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub mask: uint64_t,
    pub map: *mut AVChannelCustom,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVRational {
    pub num: libc::c_int,
    pub den: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFloatDSPContext {
    pub vector_fmul: Option<
        unsafe extern "C" fn(
            *mut libc::c_float,
            *const libc::c_float,
            *const libc::c_float,
            libc::c_int,
        ) -> (),
    >,
    pub vector_fmac_scalar: Option<
        unsafe extern "C" fn(
            *mut libc::c_float,
            *const libc::c_float,
            libc::c_float,
            libc::c_int,
        ) -> (),
    >,
    pub vector_dmac_scalar: Option<
        unsafe extern "C" fn(
            *mut libc::c_double,
            *const libc::c_double,
            libc::c_double,
            libc::c_int,
        ) -> (),
    >,
    pub vector_fmul_scalar: Option<
        unsafe extern "C" fn(
            *mut libc::c_float,
            *const libc::c_float,
            libc::c_float,
            libc::c_int,
        ) -> (),
    >,
    pub vector_dmul_scalar: Option<
        unsafe extern "C" fn(
            *mut libc::c_double,
            *const libc::c_double,
            libc::c_double,
            libc::c_int,
        ) -> (),
    >,
    pub vector_fmul_window: Option<
        unsafe extern "C" fn(
            *mut libc::c_float,
            *const libc::c_float,
            *const libc::c_float,
            *const libc::c_float,
            libc::c_int,
        ) -> (),
    >,
    pub vector_fmul_add: Option<
        unsafe extern "C" fn(
            *mut libc::c_float,
            *const libc::c_float,
            *const libc::c_float,
            *const libc::c_float,
            libc::c_int,
        ) -> (),
    >,
    pub vector_fmul_reverse: Option<
        unsafe extern "C" fn(
            *mut libc::c_float,
            *const libc::c_float,
            *const libc::c_float,
            libc::c_int,
        ) -> (),
    >,
    pub butterflies_float:
        Option<unsafe extern "C" fn(*mut libc::c_float, *mut libc::c_float, libc::c_int) -> ()>,
    pub scalarproduct_float: Option<
        unsafe extern "C" fn(
            *const libc::c_float,
            *const libc::c_float,
            libc::c_int,
        ) -> libc::c_float,
    >,
    pub vector_dmul: Option<
        unsafe extern "C" fn(
            *mut libc::c_double,
            *const libc::c_double,
            *const libc::c_double,
            libc::c_int,
        ) -> (),
    >,
}
pub type AVMediaType = libc::c_int;
pub const AVMEDIA_TYPE_NB: AVMediaType = 5;
pub const AVMEDIA_TYPE_ATTACHMENT: AVMediaType = 4;
pub const AVMEDIA_TYPE_SUBTITLE: AVMediaType = 3;
pub const AVMEDIA_TYPE_DATA: AVMediaType = 2;
pub const AVMEDIA_TYPE_AUDIO: AVMediaType = 1;
pub const AVMEDIA_TYPE_VIDEO: AVMediaType = 0;
pub const AVMEDIA_TYPE_UNKNOWN: AVMediaType = -1;
pub type AVPictureType = libc::c_uint;
pub const AV_PICTURE_TYPE_BI: AVPictureType = 7;
pub const AV_PICTURE_TYPE_SP: AVPictureType = 6;
pub const AV_PICTURE_TYPE_SI: AVPictureType = 5;
pub const AV_PICTURE_TYPE_S: AVPictureType = 4;
pub const AV_PICTURE_TYPE_B: AVPictureType = 3;
pub const AV_PICTURE_TYPE_P: AVPictureType = 2;
pub const AV_PICTURE_TYPE_I: AVPictureType = 1;
pub const AV_PICTURE_TYPE_NONE: AVPictureType = 0;
pub type ptrdiff_t = libc::c_long;
pub type AVPixelFormat = libc::c_int;
pub const AV_PIX_FMT_NB: AVPixelFormat = 228;
pub const AV_PIX_FMT_GBRAP14LE: AVPixelFormat = 227;
pub const AV_PIX_FMT_GBRAP14BE: AVPixelFormat = 226;
pub const AV_PIX_FMT_P412LE: AVPixelFormat = 225;
pub const AV_PIX_FMT_P412BE: AVPixelFormat = 224;
pub const AV_PIX_FMT_P212LE: AVPixelFormat = 223;
pub const AV_PIX_FMT_P212BE: AVPixelFormat = 222;
pub const AV_PIX_FMT_RGBAF32LE: AVPixelFormat = 221;
pub const AV_PIX_FMT_RGBAF32BE: AVPixelFormat = 220;
pub const AV_PIX_FMT_RGBF32LE: AVPixelFormat = 219;
pub const AV_PIX_FMT_RGBF32BE: AVPixelFormat = 218;
pub const AV_PIX_FMT_XV36LE: AVPixelFormat = 217;
pub const AV_PIX_FMT_XV36BE: AVPixelFormat = 216;
pub const AV_PIX_FMT_XV30LE: AVPixelFormat = 215;
pub const AV_PIX_FMT_XV30BE: AVPixelFormat = 214;
pub const AV_PIX_FMT_Y212LE: AVPixelFormat = 213;
pub const AV_PIX_FMT_Y212BE: AVPixelFormat = 212;
pub const AV_PIX_FMT_P012BE: AVPixelFormat = 211;
pub const AV_PIX_FMT_P012LE: AVPixelFormat = 210;
pub const AV_PIX_FMT_VUYX: AVPixelFormat = 209;
pub const AV_PIX_FMT_RGBAF16LE: AVPixelFormat = 208;
pub const AV_PIX_FMT_RGBAF16BE: AVPixelFormat = 207;
pub const AV_PIX_FMT_VUYA: AVPixelFormat = 206;
pub const AV_PIX_FMT_P416LE: AVPixelFormat = 205;
pub const AV_PIX_FMT_P416BE: AVPixelFormat = 204;
pub const AV_PIX_FMT_P216LE: AVPixelFormat = 203;
pub const AV_PIX_FMT_P216BE: AVPixelFormat = 202;
pub const AV_PIX_FMT_P410LE: AVPixelFormat = 201;
pub const AV_PIX_FMT_P410BE: AVPixelFormat = 200;
pub const AV_PIX_FMT_P210LE: AVPixelFormat = 199;
pub const AV_PIX_FMT_P210BE: AVPixelFormat = 198;
pub const AV_PIX_FMT_X2BGR10BE: AVPixelFormat = 197;
pub const AV_PIX_FMT_X2BGR10LE: AVPixelFormat = 196;
pub const AV_PIX_FMT_X2RGB10BE: AVPixelFormat = 195;
pub const AV_PIX_FMT_X2RGB10LE: AVPixelFormat = 194;
pub const AV_PIX_FMT_Y210LE: AVPixelFormat = 193;
pub const AV_PIX_FMT_Y210BE: AVPixelFormat = 192;
pub const AV_PIX_FMT_VULKAN: AVPixelFormat = 191;
pub const AV_PIX_FMT_NV42: AVPixelFormat = 190;
pub const AV_PIX_FMT_NV24: AVPixelFormat = 189;
pub const AV_PIX_FMT_YUVA444P12LE: AVPixelFormat = 188;
pub const AV_PIX_FMT_YUVA444P12BE: AVPixelFormat = 187;
pub const AV_PIX_FMT_YUVA422P12LE: AVPixelFormat = 186;
pub const AV_PIX_FMT_YUVA422P12BE: AVPixelFormat = 185;
pub const AV_PIX_FMT_GRAYF32LE: AVPixelFormat = 184;
pub const AV_PIX_FMT_GRAYF32BE: AVPixelFormat = 183;
pub const AV_PIX_FMT_GRAY14LE: AVPixelFormat = 182;
pub const AV_PIX_FMT_GRAY14BE: AVPixelFormat = 181;
pub const AV_PIX_FMT_OPENCL: AVPixelFormat = 180;
pub const AV_PIX_FMT_DRM_PRIME: AVPixelFormat = 179;
pub const AV_PIX_FMT_GBRAPF32LE: AVPixelFormat = 178;
pub const AV_PIX_FMT_GBRAPF32BE: AVPixelFormat = 177;
pub const AV_PIX_FMT_GBRPF32LE: AVPixelFormat = 176;
pub const AV_PIX_FMT_GBRPF32BE: AVPixelFormat = 175;
pub const AV_PIX_FMT_GRAY9LE: AVPixelFormat = 174;
pub const AV_PIX_FMT_GRAY9BE: AVPixelFormat = 173;
pub const AV_PIX_FMT_D3D11: AVPixelFormat = 172;
pub const AV_PIX_FMT_P016BE: AVPixelFormat = 171;
pub const AV_PIX_FMT_P016LE: AVPixelFormat = 170;
pub const AV_PIX_FMT_GRAY10LE: AVPixelFormat = 169;
pub const AV_PIX_FMT_GRAY10BE: AVPixelFormat = 168;
pub const AV_PIX_FMT_GRAY12LE: AVPixelFormat = 167;
pub const AV_PIX_FMT_GRAY12BE: AVPixelFormat = 166;
pub const AV_PIX_FMT_MEDIACODEC: AVPixelFormat = 165;
pub const AV_PIX_FMT_GBRAP10LE: AVPixelFormat = 164;
pub const AV_PIX_FMT_GBRAP10BE: AVPixelFormat = 163;
pub const AV_PIX_FMT_GBRAP12LE: AVPixelFormat = 162;
pub const AV_PIX_FMT_GBRAP12BE: AVPixelFormat = 161;
pub const AV_PIX_FMT_P010BE: AVPixelFormat = 160;
pub const AV_PIX_FMT_P010LE: AVPixelFormat = 159;
pub const AV_PIX_FMT_VIDEOTOOLBOX: AVPixelFormat = 158;
pub const AV_PIX_FMT_AYUV64BE: AVPixelFormat = 157;
pub const AV_PIX_FMT_AYUV64LE: AVPixelFormat = 156;
pub const AV_PIX_FMT_YUV440P12BE: AVPixelFormat = 155;
pub const AV_PIX_FMT_YUV440P12LE: AVPixelFormat = 154;
pub const AV_PIX_FMT_YUV440P10BE: AVPixelFormat = 153;
pub const AV_PIX_FMT_YUV440P10LE: AVPixelFormat = 152;
pub const AV_PIX_FMT_XVMC: AVPixelFormat = 151;
pub const AV_PIX_FMT_BAYER_GRBG16BE: AVPixelFormat = 150;
pub const AV_PIX_FMT_BAYER_GRBG16LE: AVPixelFormat = 149;
pub const AV_PIX_FMT_BAYER_GBRG16BE: AVPixelFormat = 148;
pub const AV_PIX_FMT_BAYER_GBRG16LE: AVPixelFormat = 147;
pub const AV_PIX_FMT_BAYER_RGGB16BE: AVPixelFormat = 146;
pub const AV_PIX_FMT_BAYER_RGGB16LE: AVPixelFormat = 145;
pub const AV_PIX_FMT_BAYER_BGGR16BE: AVPixelFormat = 144;
pub const AV_PIX_FMT_BAYER_BGGR16LE: AVPixelFormat = 143;
pub const AV_PIX_FMT_BAYER_GRBG8: AVPixelFormat = 142;
pub const AV_PIX_FMT_BAYER_GBRG8: AVPixelFormat = 141;
pub const AV_PIX_FMT_BAYER_RGGB8: AVPixelFormat = 140;
pub const AV_PIX_FMT_BAYER_BGGR8: AVPixelFormat = 139;
pub const AV_PIX_FMT_YUVJ411P: AVPixelFormat = 138;
pub const AV_PIX_FMT_GBRP14LE: AVPixelFormat = 137;
pub const AV_PIX_FMT_GBRP14BE: AVPixelFormat = 136;
pub const AV_PIX_FMT_GBRP12LE: AVPixelFormat = 135;
pub const AV_PIX_FMT_GBRP12BE: AVPixelFormat = 134;
pub const AV_PIX_FMT_YUV444P14LE: AVPixelFormat = 133;
pub const AV_PIX_FMT_YUV444P14BE: AVPixelFormat = 132;
pub const AV_PIX_FMT_YUV444P12LE: AVPixelFormat = 131;
pub const AV_PIX_FMT_YUV444P12BE: AVPixelFormat = 130;
pub const AV_PIX_FMT_YUV422P14LE: AVPixelFormat = 129;
pub const AV_PIX_FMT_YUV422P14BE: AVPixelFormat = 128;
pub const AV_PIX_FMT_YUV422P12LE: AVPixelFormat = 127;
pub const AV_PIX_FMT_YUV422P12BE: AVPixelFormat = 126;
pub const AV_PIX_FMT_YUV420P14LE: AVPixelFormat = 125;
pub const AV_PIX_FMT_YUV420P14BE: AVPixelFormat = 124;
pub const AV_PIX_FMT_YUV420P12LE: AVPixelFormat = 123;
pub const AV_PIX_FMT_YUV420P12BE: AVPixelFormat = 122;
pub const AV_PIX_FMT_BGR0: AVPixelFormat = 121;
pub const AV_PIX_FMT_0BGR: AVPixelFormat = 120;
pub const AV_PIX_FMT_RGB0: AVPixelFormat = 119;
pub const AV_PIX_FMT_0RGB: AVPixelFormat = 118;
pub const AV_PIX_FMT_CUDA: AVPixelFormat = 117;
pub const AV_PIX_FMT_D3D11VA_VLD: AVPixelFormat = 116;
pub const AV_PIX_FMT_MMAL: AVPixelFormat = 115;
pub const AV_PIX_FMT_QSV: AVPixelFormat = 114;
pub const AV_PIX_FMT_GBRAP16LE: AVPixelFormat = 113;
pub const AV_PIX_FMT_GBRAP16BE: AVPixelFormat = 112;
pub const AV_PIX_FMT_GBRAP: AVPixelFormat = 111;
pub const AV_PIX_FMT_YA16LE: AVPixelFormat = 110;
pub const AV_PIX_FMT_YA16BE: AVPixelFormat = 109;
pub const AV_PIX_FMT_YVYU422: AVPixelFormat = 108;
pub const AV_PIX_FMT_BGRA64LE: AVPixelFormat = 107;
pub const AV_PIX_FMT_BGRA64BE: AVPixelFormat = 106;
pub const AV_PIX_FMT_RGBA64LE: AVPixelFormat = 105;
pub const AV_PIX_FMT_RGBA64BE: AVPixelFormat = 104;
pub const AV_PIX_FMT_NV20BE: AVPixelFormat = 103;
pub const AV_PIX_FMT_NV20LE: AVPixelFormat = 102;
pub const AV_PIX_FMT_NV16: AVPixelFormat = 101;
pub const AV_PIX_FMT_XYZ12BE: AVPixelFormat = 100;
pub const AV_PIX_FMT_XYZ12LE: AVPixelFormat = 99;
pub const AV_PIX_FMT_VDPAU: AVPixelFormat = 98;
pub const AV_PIX_FMT_YUVA444P16LE: AVPixelFormat = 97;
pub const AV_PIX_FMT_YUVA444P16BE: AVPixelFormat = 96;
pub const AV_PIX_FMT_YUVA422P16LE: AVPixelFormat = 95;
pub const AV_PIX_FMT_YUVA422P16BE: AVPixelFormat = 94;
pub const AV_PIX_FMT_YUVA420P16LE: AVPixelFormat = 93;
pub const AV_PIX_FMT_YUVA420P16BE: AVPixelFormat = 92;
pub const AV_PIX_FMT_YUVA444P10LE: AVPixelFormat = 91;
pub const AV_PIX_FMT_YUVA444P10BE: AVPixelFormat = 90;
pub const AV_PIX_FMT_YUVA422P10LE: AVPixelFormat = 89;
pub const AV_PIX_FMT_YUVA422P10BE: AVPixelFormat = 88;
pub const AV_PIX_FMT_YUVA420P10LE: AVPixelFormat = 87;
pub const AV_PIX_FMT_YUVA420P10BE: AVPixelFormat = 86;
pub const AV_PIX_FMT_YUVA444P9LE: AVPixelFormat = 85;
pub const AV_PIX_FMT_YUVA444P9BE: AVPixelFormat = 84;
pub const AV_PIX_FMT_YUVA422P9LE: AVPixelFormat = 83;
pub const AV_PIX_FMT_YUVA422P9BE: AVPixelFormat = 82;
pub const AV_PIX_FMT_YUVA420P9LE: AVPixelFormat = 81;
pub const AV_PIX_FMT_YUVA420P9BE: AVPixelFormat = 80;
pub const AV_PIX_FMT_YUVA444P: AVPixelFormat = 79;
pub const AV_PIX_FMT_YUVA422P: AVPixelFormat = 78;
pub const AV_PIX_FMT_GBRP16LE: AVPixelFormat = 77;
pub const AV_PIX_FMT_GBRP16BE: AVPixelFormat = 76;
pub const AV_PIX_FMT_GBRP10LE: AVPixelFormat = 75;
pub const AV_PIX_FMT_GBRP10BE: AVPixelFormat = 74;
pub const AV_PIX_FMT_GBRP9LE: AVPixelFormat = 73;
pub const AV_PIX_FMT_GBRP9BE: AVPixelFormat = 72;
pub const AV_PIX_FMT_GBR24P: AVPixelFormat = 71;
pub const AV_PIX_FMT_GBRP: AVPixelFormat = 71;
pub const AV_PIX_FMT_YUV422P9LE: AVPixelFormat = 70;
pub const AV_PIX_FMT_YUV422P9BE: AVPixelFormat = 69;
pub const AV_PIX_FMT_YUV444P10LE: AVPixelFormat = 68;
pub const AV_PIX_FMT_YUV444P10BE: AVPixelFormat = 67;
pub const AV_PIX_FMT_YUV444P9LE: AVPixelFormat = 66;
pub const AV_PIX_FMT_YUV444P9BE: AVPixelFormat = 65;
pub const AV_PIX_FMT_YUV422P10LE: AVPixelFormat = 64;
pub const AV_PIX_FMT_YUV422P10BE: AVPixelFormat = 63;
pub const AV_PIX_FMT_YUV420P10LE: AVPixelFormat = 62;
pub const AV_PIX_FMT_YUV420P10BE: AVPixelFormat = 61;
pub const AV_PIX_FMT_YUV420P9LE: AVPixelFormat = 60;
pub const AV_PIX_FMT_YUV420P9BE: AVPixelFormat = 59;
pub const AV_PIX_FMT_BGR48LE: AVPixelFormat = 58;
pub const AV_PIX_FMT_BGR48BE: AVPixelFormat = 57;
pub const AV_PIX_FMT_GRAY8A: AVPixelFormat = 56;
pub const AV_PIX_FMT_Y400A: AVPixelFormat = 56;
pub const AV_PIX_FMT_YA8: AVPixelFormat = 56;
pub const AV_PIX_FMT_BGR444BE: AVPixelFormat = 55;
pub const AV_PIX_FMT_BGR444LE: AVPixelFormat = 54;
pub const AV_PIX_FMT_RGB444BE: AVPixelFormat = 53;
pub const AV_PIX_FMT_RGB444LE: AVPixelFormat = 52;
pub const AV_PIX_FMT_DXVA2_VLD: AVPixelFormat = 51;
pub const AV_PIX_FMT_YUV444P16BE: AVPixelFormat = 50;
pub const AV_PIX_FMT_YUV444P16LE: AVPixelFormat = 49;
pub const AV_PIX_FMT_YUV422P16BE: AVPixelFormat = 48;
pub const AV_PIX_FMT_YUV422P16LE: AVPixelFormat = 47;
pub const AV_PIX_FMT_YUV420P16BE: AVPixelFormat = 46;
pub const AV_PIX_FMT_YUV420P16LE: AVPixelFormat = 45;
pub const AV_PIX_FMT_VAAPI: AVPixelFormat = 44;
pub const AV_PIX_FMT_BGR555LE: AVPixelFormat = 43;
pub const AV_PIX_FMT_BGR555BE: AVPixelFormat = 42;
pub const AV_PIX_FMT_BGR565LE: AVPixelFormat = 41;
pub const AV_PIX_FMT_BGR565BE: AVPixelFormat = 40;
pub const AV_PIX_FMT_RGB555LE: AVPixelFormat = 39;
pub const AV_PIX_FMT_RGB555BE: AVPixelFormat = 38;
pub const AV_PIX_FMT_RGB565LE: AVPixelFormat = 37;
pub const AV_PIX_FMT_RGB565BE: AVPixelFormat = 36;
pub const AV_PIX_FMT_RGB48LE: AVPixelFormat = 35;
pub const AV_PIX_FMT_RGB48BE: AVPixelFormat = 34;
pub const AV_PIX_FMT_YUVA420P: AVPixelFormat = 33;
pub const AV_PIX_FMT_YUVJ440P: AVPixelFormat = 32;
pub const AV_PIX_FMT_YUV440P: AVPixelFormat = 31;
pub const AV_PIX_FMT_GRAY16LE: AVPixelFormat = 30;
pub const AV_PIX_FMT_GRAY16BE: AVPixelFormat = 29;
pub const AV_PIX_FMT_BGRA: AVPixelFormat = 28;
pub const AV_PIX_FMT_ABGR: AVPixelFormat = 27;
pub const AV_PIX_FMT_RGBA: AVPixelFormat = 26;
pub const AV_PIX_FMT_ARGB: AVPixelFormat = 25;
pub const AV_PIX_FMT_NV21: AVPixelFormat = 24;
pub const AV_PIX_FMT_NV12: AVPixelFormat = 23;
pub const AV_PIX_FMT_RGB4_BYTE: AVPixelFormat = 22;
pub const AV_PIX_FMT_RGB4: AVPixelFormat = 21;
pub const AV_PIX_FMT_RGB8: AVPixelFormat = 20;
pub const AV_PIX_FMT_BGR4_BYTE: AVPixelFormat = 19;
pub const AV_PIX_FMT_BGR4: AVPixelFormat = 18;
pub const AV_PIX_FMT_BGR8: AVPixelFormat = 17;
pub const AV_PIX_FMT_UYYVYY411: AVPixelFormat = 16;
pub const AV_PIX_FMT_UYVY422: AVPixelFormat = 15;
pub const AV_PIX_FMT_YUVJ444P: AVPixelFormat = 14;
pub const AV_PIX_FMT_YUVJ422P: AVPixelFormat = 13;
pub const AV_PIX_FMT_YUVJ420P: AVPixelFormat = 12;
pub const AV_PIX_FMT_PAL8: AVPixelFormat = 11;
pub const AV_PIX_FMT_MONOBLACK: AVPixelFormat = 10;
pub const AV_PIX_FMT_MONOWHITE: AVPixelFormat = 9;
pub const AV_PIX_FMT_GRAY8: AVPixelFormat = 8;
pub const AV_PIX_FMT_YUV411P: AVPixelFormat = 7;
pub const AV_PIX_FMT_YUV410P: AVPixelFormat = 6;
pub const AV_PIX_FMT_YUV444P: AVPixelFormat = 5;
pub const AV_PIX_FMT_YUV422P: AVPixelFormat = 4;
pub const AV_PIX_FMT_BGR24: AVPixelFormat = 3;
pub const AV_PIX_FMT_RGB24: AVPixelFormat = 2;
pub const AV_PIX_FMT_YUYV422: AVPixelFormat = 1;
pub const AV_PIX_FMT_YUV420P: AVPixelFormat = 0;
pub const AV_PIX_FMT_NONE: AVPixelFormat = -1;
pub type AVColorPrimaries = libc::c_uint;
pub const AVCOL_PRI_NB: AVColorPrimaries = 23;
pub const AVCOL_PRI_JEDEC_P22: AVColorPrimaries = 22;
pub const AVCOL_PRI_EBU3213: AVColorPrimaries = 22;
pub const AVCOL_PRI_SMPTE432: AVColorPrimaries = 12;
pub const AVCOL_PRI_SMPTE431: AVColorPrimaries = 11;
pub const AVCOL_PRI_SMPTEST428_1: AVColorPrimaries = 10;
pub const AVCOL_PRI_SMPTE428: AVColorPrimaries = 10;
pub const AVCOL_PRI_BT2020: AVColorPrimaries = 9;
pub const AVCOL_PRI_FILM: AVColorPrimaries = 8;
pub const AVCOL_PRI_SMPTE240M: AVColorPrimaries = 7;
pub const AVCOL_PRI_SMPTE170M: AVColorPrimaries = 6;
pub const AVCOL_PRI_BT470BG: AVColorPrimaries = 5;
pub const AVCOL_PRI_BT470M: AVColorPrimaries = 4;
pub const AVCOL_PRI_RESERVED: AVColorPrimaries = 3;
pub const AVCOL_PRI_UNSPECIFIED: AVColorPrimaries = 2;
pub const AVCOL_PRI_BT709: AVColorPrimaries = 1;
pub const AVCOL_PRI_RESERVED0: AVColorPrimaries = 0;
pub type AVColorTransferCharacteristic = libc::c_uint;
pub const AVCOL_TRC_NB: AVColorTransferCharacteristic = 19;
pub const AVCOL_TRC_ARIB_STD_B67: AVColorTransferCharacteristic = 18;
pub const AVCOL_TRC_SMPTEST428_1: AVColorTransferCharacteristic = 17;
pub const AVCOL_TRC_SMPTE428: AVColorTransferCharacteristic = 17;
pub const AVCOL_TRC_SMPTEST2084: AVColorTransferCharacteristic = 16;
pub const AVCOL_TRC_SMPTE2084: AVColorTransferCharacteristic = 16;
pub const AVCOL_TRC_BT2020_12: AVColorTransferCharacteristic = 15;
pub const AVCOL_TRC_BT2020_10: AVColorTransferCharacteristic = 14;
pub const AVCOL_TRC_IEC61966_2_1: AVColorTransferCharacteristic = 13;
pub const AVCOL_TRC_BT1361_ECG: AVColorTransferCharacteristic = 12;
pub const AVCOL_TRC_IEC61966_2_4: AVColorTransferCharacteristic = 11;
pub const AVCOL_TRC_LOG_SQRT: AVColorTransferCharacteristic = 10;
pub const AVCOL_TRC_LOG: AVColorTransferCharacteristic = 9;
pub const AVCOL_TRC_LINEAR: AVColorTransferCharacteristic = 8;
pub const AVCOL_TRC_SMPTE240M: AVColorTransferCharacteristic = 7;
pub const AVCOL_TRC_SMPTE170M: AVColorTransferCharacteristic = 6;
pub const AVCOL_TRC_GAMMA28: AVColorTransferCharacteristic = 5;
pub const AVCOL_TRC_GAMMA22: AVColorTransferCharacteristic = 4;
pub const AVCOL_TRC_RESERVED: AVColorTransferCharacteristic = 3;
pub const AVCOL_TRC_UNSPECIFIED: AVColorTransferCharacteristic = 2;
pub const AVCOL_TRC_BT709: AVColorTransferCharacteristic = 1;
pub const AVCOL_TRC_RESERVED0: AVColorTransferCharacteristic = 0;
pub type AVColorSpace = libc::c_uint;
pub const AVCOL_SPC_NB: AVColorSpace = 15;
pub const AVCOL_SPC_ICTCP: AVColorSpace = 14;
pub const AVCOL_SPC_CHROMA_DERIVED_CL: AVColorSpace = 13;
pub const AVCOL_SPC_CHROMA_DERIVED_NCL: AVColorSpace = 12;
pub const AVCOL_SPC_SMPTE2085: AVColorSpace = 11;
pub const AVCOL_SPC_BT2020_CL: AVColorSpace = 10;
pub const AVCOL_SPC_BT2020_NCL: AVColorSpace = 9;
pub const AVCOL_SPC_YCOCG: AVColorSpace = 8;
pub const AVCOL_SPC_YCGCO: AVColorSpace = 8;
pub const AVCOL_SPC_SMPTE240M: AVColorSpace = 7;
pub const AVCOL_SPC_SMPTE170M: AVColorSpace = 6;
pub const AVCOL_SPC_BT470BG: AVColorSpace = 5;
pub const AVCOL_SPC_FCC: AVColorSpace = 4;
pub const AVCOL_SPC_RESERVED: AVColorSpace = 3;
pub const AVCOL_SPC_UNSPECIFIED: AVColorSpace = 2;
pub const AVCOL_SPC_BT709: AVColorSpace = 1;
pub const AVCOL_SPC_RGB: AVColorSpace = 0;
pub type AVColorRange = libc::c_uint;
pub const AVCOL_RANGE_NB: AVColorRange = 3;
pub const AVCOL_RANGE_JPEG: AVColorRange = 2;
pub const AVCOL_RANGE_MPEG: AVColorRange = 1;
pub const AVCOL_RANGE_UNSPECIFIED: AVColorRange = 0;
pub type AVChromaLocation = libc::c_uint;
pub const AVCHROMA_LOC_NB: AVChromaLocation = 7;
pub const AVCHROMA_LOC_BOTTOM: AVChromaLocation = 6;
pub const AVCHROMA_LOC_BOTTOMLEFT: AVChromaLocation = 5;
pub const AVCHROMA_LOC_TOP: AVChromaLocation = 4;
pub const AVCHROMA_LOC_TOPLEFT: AVChromaLocation = 3;
pub const AVCHROMA_LOC_CENTER: AVChromaLocation = 2;
pub const AVCHROMA_LOC_LEFT: AVChromaLocation = 1;
pub const AVCHROMA_LOC_UNSPECIFIED: AVChromaLocation = 0;
pub type AVClassCategory = libc::c_uint;
pub const AV_CLASS_CATEGORY_NB: AVClassCategory = 46;
pub const AV_CLASS_CATEGORY_DEVICE_INPUT: AVClassCategory = 45;
pub const AV_CLASS_CATEGORY_DEVICE_OUTPUT: AVClassCategory = 44;
pub const AV_CLASS_CATEGORY_DEVICE_AUDIO_INPUT: AVClassCategory = 43;
pub const AV_CLASS_CATEGORY_DEVICE_AUDIO_OUTPUT: AVClassCategory = 42;
pub const AV_CLASS_CATEGORY_DEVICE_VIDEO_INPUT: AVClassCategory = 41;
pub const AV_CLASS_CATEGORY_DEVICE_VIDEO_OUTPUT: AVClassCategory = 40;
pub const AV_CLASS_CATEGORY_SWRESAMPLER: AVClassCategory = 10;
pub const AV_CLASS_CATEGORY_SWSCALER: AVClassCategory = 9;
pub const AV_CLASS_CATEGORY_BITSTREAM_FILTER: AVClassCategory = 8;
pub const AV_CLASS_CATEGORY_FILTER: AVClassCategory = 7;
pub const AV_CLASS_CATEGORY_DECODER: AVClassCategory = 6;
pub const AV_CLASS_CATEGORY_ENCODER: AVClassCategory = 5;
pub const AV_CLASS_CATEGORY_DEMUXER: AVClassCategory = 4;
pub const AV_CLASS_CATEGORY_MUXER: AVClassCategory = 3;
pub const AV_CLASS_CATEGORY_OUTPUT: AVClassCategory = 2;
pub const AV_CLASS_CATEGORY_INPUT: AVClassCategory = 1;
pub const AV_CLASS_CATEGORY_NA: AVClassCategory = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVOptionRanges {
    pub range: *mut *mut AVOptionRange,
    pub nb_ranges: libc::c_int,
    pub nb_components: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVOptionRange {
    pub str_0: *const libc::c_char,
    pub value_min: libc::c_double,
    pub value_max: libc::c_double,
    pub component_min: libc::c_double,
    pub component_max: libc::c_double,
    pub is_range: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVClass {
    pub class_name: *const libc::c_char,
    pub item_name: Option<unsafe extern "C" fn(*mut libc::c_void) -> *const libc::c_char>,
    pub option: *const AVOption,
    pub version: libc::c_int,
    pub log_level_offset_offset: libc::c_int,
    pub parent_log_context_offset: libc::c_int,
    pub category: AVClassCategory,
    pub get_category: Option<unsafe extern "C" fn(*mut libc::c_void) -> AVClassCategory>,
    pub query_ranges: Option<
        unsafe extern "C" fn(
            *mut *mut AVOptionRanges,
            *mut libc::c_void,
            *const libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub child_next:
        Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> *mut libc::c_void>,
    pub child_class_iterate: Option<unsafe extern "C" fn(*mut *mut libc::c_void) -> *const AVClass>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVOption {
    pub name: *const libc::c_char,
    pub help: *const libc::c_char,
    pub offset: libc::c_int,
    pub type_0: AVOptionType,
    pub default_val: C2RustUnnamed_0,
    pub min: libc::c_double,
    pub max: libc::c_double,
    pub flags: libc::c_int,
    pub unit: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub i64_0: int64_t,
    pub dbl: libc::c_double,
    pub str_0: *const libc::c_char,
    pub q: AVRational,
}
pub type AVOptionType = libc::c_uint;
pub const AV_OPT_TYPE_CHLAYOUT: AVOptionType = 19;
pub const AV_OPT_TYPE_BOOL: AVOptionType = 18;
pub const AV_OPT_TYPE_CHANNEL_LAYOUT: AVOptionType = 17;
pub const AV_OPT_TYPE_COLOR: AVOptionType = 16;
pub const AV_OPT_TYPE_DURATION: AVOptionType = 15;
pub const AV_OPT_TYPE_VIDEO_RATE: AVOptionType = 14;
pub const AV_OPT_TYPE_SAMPLE_FMT: AVOptionType = 13;
pub const AV_OPT_TYPE_PIXEL_FMT: AVOptionType = 12;
pub const AV_OPT_TYPE_IMAGE_SIZE: AVOptionType = 11;
pub const AV_OPT_TYPE_CONST: AVOptionType = 10;
pub const AV_OPT_TYPE_UINT64: AVOptionType = 9;
pub const AV_OPT_TYPE_DICT: AVOptionType = 8;
pub const AV_OPT_TYPE_BINARY: AVOptionType = 7;
pub const AV_OPT_TYPE_RATIONAL: AVOptionType = 6;
pub const AV_OPT_TYPE_STRING: AVOptionType = 5;
pub const AV_OPT_TYPE_FLOAT: AVOptionType = 4;
pub const AV_OPT_TYPE_DOUBLE: AVOptionType = 3;
pub const AV_OPT_TYPE_INT64: AVOptionType = 2;
pub const AV_OPT_TYPE_INT: AVOptionType = 1;
pub const AV_OPT_TYPE_FLAGS: AVOptionType = 0;
pub type AVSampleFormat = libc::c_int;
pub const AV_SAMPLE_FMT_NB: AVSampleFormat = 12;
pub const AV_SAMPLE_FMT_S64P: AVSampleFormat = 11;
pub const AV_SAMPLE_FMT_S64: AVSampleFormat = 10;
pub const AV_SAMPLE_FMT_DBLP: AVSampleFormat = 9;
pub const AV_SAMPLE_FMT_FLTP: AVSampleFormat = 8;
pub const AV_SAMPLE_FMT_S32P: AVSampleFormat = 7;
pub const AV_SAMPLE_FMT_S16P: AVSampleFormat = 6;
pub const AV_SAMPLE_FMT_U8P: AVSampleFormat = 5;
pub const AV_SAMPLE_FMT_DBL: AVSampleFormat = 4;
pub const AV_SAMPLE_FMT_FLT: AVSampleFormat = 3;
pub const AV_SAMPLE_FMT_S32: AVSampleFormat = 2;
pub const AV_SAMPLE_FMT_S16: AVSampleFormat = 1;
pub const AV_SAMPLE_FMT_U8: AVSampleFormat = 0;
pub const AV_SAMPLE_FMT_NONE: AVSampleFormat = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVBufferRef {
    pub buffer: *mut AVBuffer,
    pub data: *mut uint8_t,
    pub size: size_t,
}
pub type AVFrameSideDataType = libc::c_uint;
pub const AV_FRAME_DATA_VIDEO_HINT: AVFrameSideDataType = 27;
pub const AV_FRAME_DATA_AMBIENT_VIEWING_ENVIRONMENT: AVFrameSideDataType = 26;
pub const AV_FRAME_DATA_DYNAMIC_HDR_VIVID: AVFrameSideDataType = 25;
pub const AV_FRAME_DATA_DOVI_METADATA: AVFrameSideDataType = 24;
pub const AV_FRAME_DATA_DOVI_RPU_BUFFER: AVFrameSideDataType = 23;
pub const AV_FRAME_DATA_DETECTION_BBOXES: AVFrameSideDataType = 22;
pub const AV_FRAME_DATA_FILM_GRAIN_PARAMS: AVFrameSideDataType = 21;
pub const AV_FRAME_DATA_SEI_UNREGISTERED: AVFrameSideDataType = 20;
pub const AV_FRAME_DATA_VIDEO_ENC_PARAMS: AVFrameSideDataType = 19;
pub const AV_FRAME_DATA_REGIONS_OF_INTEREST: AVFrameSideDataType = 18;
pub const AV_FRAME_DATA_DYNAMIC_HDR_PLUS: AVFrameSideDataType = 17;
pub const AV_FRAME_DATA_S12M_TIMECODE: AVFrameSideDataType = 16;
pub const AV_FRAME_DATA_ICC_PROFILE: AVFrameSideDataType = 15;
pub const AV_FRAME_DATA_CONTENT_LIGHT_LEVEL: AVFrameSideDataType = 14;
pub const AV_FRAME_DATA_SPHERICAL: AVFrameSideDataType = 13;
pub const AV_FRAME_DATA_GOP_TIMECODE: AVFrameSideDataType = 12;
pub const AV_FRAME_DATA_MASTERING_DISPLAY_METADATA: AVFrameSideDataType = 11;
pub const AV_FRAME_DATA_AUDIO_SERVICE_TYPE: AVFrameSideDataType = 10;
pub const AV_FRAME_DATA_SKIP_SAMPLES: AVFrameSideDataType = 9;
pub const AV_FRAME_DATA_MOTION_VECTORS: AVFrameSideDataType = 8;
pub const AV_FRAME_DATA_AFD: AVFrameSideDataType = 7;
pub const AV_FRAME_DATA_DISPLAYMATRIX: AVFrameSideDataType = 6;
pub const AV_FRAME_DATA_REPLAYGAIN: AVFrameSideDataType = 5;
pub const AV_FRAME_DATA_DOWNMIX_INFO: AVFrameSideDataType = 4;
pub const AV_FRAME_DATA_MATRIXENCODING: AVFrameSideDataType = 3;
pub const AV_FRAME_DATA_STEREO3D: AVFrameSideDataType = 2;
pub const AV_FRAME_DATA_A53_CC: AVFrameSideDataType = 1;
pub const AV_FRAME_DATA_PANSCAN: AVFrameSideDataType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFrameSideData {
    pub type_0: AVFrameSideDataType,
    pub data: *mut uint8_t,
    pub size: size_t,
    pub metadata: *mut AVDictionary,
    pub buf: *mut AVBufferRef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFrame {
    pub data: [*mut uint8_t; 8],
    pub linesize: [libc::c_int; 8],
    pub extended_data: *mut *mut uint8_t,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub nb_samples: libc::c_int,
    pub format: libc::c_int,
    pub key_frame: libc::c_int,
    pub pict_type: AVPictureType,
    pub sample_aspect_ratio: AVRational,
    pub pts: int64_t,
    pub pkt_dts: int64_t,
    pub time_base: AVRational,
    pub coded_picture_number: libc::c_int,
    pub display_picture_number: libc::c_int,
    pub quality: libc::c_int,
    pub opaque: *mut libc::c_void,
    pub repeat_pict: libc::c_int,
    pub interlaced_frame: libc::c_int,
    pub top_field_first: libc::c_int,
    pub palette_has_changed: libc::c_int,
    pub reordered_opaque: int64_t,
    pub sample_rate: libc::c_int,
    pub channel_layout: uint64_t,
    pub buf: [*mut AVBufferRef; 8],
    pub extended_buf: *mut *mut AVBufferRef,
    pub nb_extended_buf: libc::c_int,
    pub side_data: *mut *mut AVFrameSideData,
    pub nb_side_data: libc::c_int,
    pub flags: libc::c_int,
    pub color_range: AVColorRange,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub colorspace: AVColorSpace,
    pub chroma_location: AVChromaLocation,
    pub best_effort_timestamp: int64_t,
    pub pkt_pos: int64_t,
    pub pkt_duration: int64_t,
    pub metadata: *mut AVDictionary,
    pub decode_error_flags: libc::c_int,
    pub channels: libc::c_int,
    pub pkt_size: libc::c_int,
    pub hw_frames_ctx: *mut AVBufferRef,
    pub opaque_ref: *mut AVBufferRef,
    pub crop_top: size_t,
    pub crop_bottom: size_t,
    pub crop_left: size_t,
    pub crop_right: size_t,
    pub private_ref: *mut AVBufferRef,
    pub ch_layout: AVChannelLayout,
    pub duration: int64_t,
}
pub type AVCodecID = libc::c_uint;
pub const AV_CODEC_ID_ANULL: AVCodecID = 135171;
pub const AV_CODEC_ID_VNULL: AVCodecID = 135170;
pub const AV_CODEC_ID_WRAPPED_AVFRAME: AVCodecID = 135169;
pub const AV_CODEC_ID_FFMETADATA: AVCodecID = 135168;
pub const AV_CODEC_ID_MPEG4SYSTEMS: AVCodecID = 131073;
pub const AV_CODEC_ID_MPEG2TS: AVCodecID = 131072;
pub const AV_CODEC_ID_PROBE: AVCodecID = 102400;
pub const AV_CODEC_ID_SMPTE_2038: AVCodecID = 98315;
pub const AV_CODEC_ID_BIN_DATA: AVCodecID = 98314;
pub const AV_CODEC_ID_TIMED_ID3: AVCodecID = 98313;
pub const AV_CODEC_ID_DVD_NAV: AVCodecID = 98312;
pub const AV_CODEC_ID_SMPTE_KLV: AVCodecID = 98311;
pub const AV_CODEC_ID_OTF: AVCodecID = 98310;
pub const AV_CODEC_ID_IDF: AVCodecID = 98309;
pub const AV_CODEC_ID_XBIN: AVCodecID = 98308;
pub const AV_CODEC_ID_BINTEXT: AVCodecID = 98307;
pub const AV_CODEC_ID_EPG: AVCodecID = 98306;
pub const AV_CODEC_ID_SCTE_35: AVCodecID = 98305;
pub const AV_CODEC_ID_TTF: AVCodecID = 98304;
pub const AV_CODEC_ID_FIRST_UNKNOWN: AVCodecID = 98304;
pub const AV_CODEC_ID_ARIB_CAPTION: AVCodecID = 94233;
pub const AV_CODEC_ID_TTML: AVCodecID = 94232;
pub const AV_CODEC_ID_HDMV_TEXT_SUBTITLE: AVCodecID = 94231;
pub const AV_CODEC_ID_ASS: AVCodecID = 94230;
pub const AV_CODEC_ID_PJS: AVCodecID = 94229;
pub const AV_CODEC_ID_VPLAYER: AVCodecID = 94228;
pub const AV_CODEC_ID_MPL2: AVCodecID = 94227;
pub const AV_CODEC_ID_WEBVTT: AVCodecID = 94226;
pub const AV_CODEC_ID_SUBRIP: AVCodecID = 94225;
pub const AV_CODEC_ID_SUBVIEWER: AVCodecID = 94224;
pub const AV_CODEC_ID_SUBVIEWER1: AVCodecID = 94223;
pub const AV_CODEC_ID_STL: AVCodecID = 94222;
pub const AV_CODEC_ID_REALTEXT: AVCodecID = 94221;
pub const AV_CODEC_ID_SAMI: AVCodecID = 94220;
pub const AV_CODEC_ID_JACOSUB: AVCodecID = 94219;
pub const AV_CODEC_ID_EIA_608: AVCodecID = 94218;
pub const AV_CODEC_ID_MICRODVD: AVCodecID = 94217;
pub const AV_CODEC_ID_SRT: AVCodecID = 94216;
pub const AV_CODEC_ID_DVB_TELETEXT: AVCodecID = 94215;
pub const AV_CODEC_ID_HDMV_PGS_SUBTITLE: AVCodecID = 94214;
pub const AV_CODEC_ID_MOV_TEXT: AVCodecID = 94213;
pub const AV_CODEC_ID_SSA: AVCodecID = 94212;
pub const AV_CODEC_ID_XSUB: AVCodecID = 94211;
pub const AV_CODEC_ID_TEXT: AVCodecID = 94210;
pub const AV_CODEC_ID_DVB_SUBTITLE: AVCodecID = 94209;
pub const AV_CODEC_ID_DVD_SUBTITLE: AVCodecID = 94208;
pub const AV_CODEC_ID_FIRST_SUBTITLE: AVCodecID = 94208;
pub const AV_CODEC_ID_OSQ: AVCodecID = 86120;
pub const AV_CODEC_ID_AC4: AVCodecID = 86119;
pub const AV_CODEC_ID_RKA: AVCodecID = 86118;
pub const AV_CODEC_ID_WAVARC: AVCodecID = 86117;
pub const AV_CODEC_ID_FTR: AVCodecID = 86116;
pub const AV_CODEC_ID_APAC: AVCodecID = 86115;
pub const AV_CODEC_ID_MISC4: AVCodecID = 86114;
pub const AV_CODEC_ID_BONK: AVCodecID = 86113;
pub const AV_CODEC_ID_DFPWM: AVCodecID = 86112;
pub const AV_CODEC_ID_MSNSIREN: AVCodecID = 86111;
pub const AV_CODEC_ID_FASTAUDIO: AVCodecID = 86110;
pub const AV_CODEC_ID_HCA: AVCodecID = 86109;
pub const AV_CODEC_ID_SIREN: AVCodecID = 86108;
pub const AV_CODEC_ID_MPEGH_3D_AUDIO: AVCodecID = 86107;
pub const AV_CODEC_ID_ACELP_KELVIN: AVCodecID = 86106;
pub const AV_CODEC_ID_HCOM: AVCodecID = 86105;
pub const AV_CODEC_ID_ATRAC9: AVCodecID = 86104;
pub const AV_CODEC_ID_SBC: AVCodecID = 86103;
pub const AV_CODEC_ID_APTX_HD: AVCodecID = 86102;
pub const AV_CODEC_ID_APTX: AVCodecID = 86101;
pub const AV_CODEC_ID_DOLBY_E: AVCodecID = 86100;
pub const AV_CODEC_ID_ATRAC3PAL: AVCodecID = 86099;
pub const AV_CODEC_ID_ATRAC3AL: AVCodecID = 86098;
pub const AV_CODEC_ID_DST: AVCodecID = 86097;
pub const AV_CODEC_ID_XMA2: AVCodecID = 86096;
pub const AV_CODEC_ID_XMA1: AVCodecID = 86095;
pub const AV_CODEC_ID_INTERPLAY_ACM: AVCodecID = 86094;
pub const AV_CODEC_ID_4GV: AVCodecID = 86093;
pub const AV_CODEC_ID_DSD_MSBF_PLANAR: AVCodecID = 86092;
pub const AV_CODEC_ID_DSD_LSBF_PLANAR: AVCodecID = 86091;
pub const AV_CODEC_ID_DSD_MSBF: AVCodecID = 86090;
pub const AV_CODEC_ID_DSD_LSBF: AVCodecID = 86089;
pub const AV_CODEC_ID_SMV: AVCodecID = 86088;
pub const AV_CODEC_ID_EVRC: AVCodecID = 86087;
pub const AV_CODEC_ID_SONIC_LS: AVCodecID = 86086;
pub const AV_CODEC_ID_SONIC: AVCodecID = 86085;
pub const AV_CODEC_ID_FFWAVESYNTH: AVCodecID = 86084;
pub const AV_CODEC_ID_CODEC2: AVCodecID = 86083;
pub const AV_CODEC_ID_DSS_SP: AVCodecID = 86082;
pub const AV_CODEC_ID_ON2AVC: AVCodecID = 86081;
pub const AV_CODEC_ID_PAF_AUDIO: AVCodecID = 86080;
pub const AV_CODEC_ID_METASOUND: AVCodecID = 86079;
pub const AV_CODEC_ID_TAK: AVCodecID = 86078;
pub const AV_CODEC_ID_COMFORT_NOISE: AVCodecID = 86077;
pub const AV_CODEC_ID_OPUS: AVCodecID = 86076;
pub const AV_CODEC_ID_ILBC: AVCodecID = 86075;
pub const AV_CODEC_ID_IAC: AVCodecID = 86074;
pub const AV_CODEC_ID_RALF: AVCodecID = 86073;
pub const AV_CODEC_ID_BMV_AUDIO: AVCodecID = 86072;
pub const AV_CODEC_ID_8SVX_FIB: AVCodecID = 86071;
pub const AV_CODEC_ID_8SVX_EXP: AVCodecID = 86070;
pub const AV_CODEC_ID_G729: AVCodecID = 86069;
pub const AV_CODEC_ID_G723_1: AVCodecID = 86068;
pub const AV_CODEC_ID_CELT: AVCodecID = 86067;
pub const AV_CODEC_ID_QDMC: AVCodecID = 86066;
pub const AV_CODEC_ID_AAC_LATM: AVCodecID = 86065;
pub const AV_CODEC_ID_BINKAUDIO_DCT: AVCodecID = 86064;
pub const AV_CODEC_ID_BINKAUDIO_RDFT: AVCodecID = 86063;
pub const AV_CODEC_ID_ATRAC1: AVCodecID = 86062;
pub const AV_CODEC_ID_MP4ALS: AVCodecID = 86061;
pub const AV_CODEC_ID_TRUEHD: AVCodecID = 86060;
pub const AV_CODEC_ID_TWINVQ: AVCodecID = 86059;
pub const AV_CODEC_ID_MP1: AVCodecID = 86058;
pub const AV_CODEC_ID_SIPR: AVCodecID = 86057;
pub const AV_CODEC_ID_EAC3: AVCodecID = 86056;
pub const AV_CODEC_ID_ATRAC3P: AVCodecID = 86055;
pub const AV_CODEC_ID_WMALOSSLESS: AVCodecID = 86054;
pub const AV_CODEC_ID_WMAPRO: AVCodecID = 86053;
pub const AV_CODEC_ID_WMAVOICE: AVCodecID = 86052;
pub const AV_CODEC_ID_SPEEX: AVCodecID = 86051;
pub const AV_CODEC_ID_MUSEPACK8: AVCodecID = 86050;
pub const AV_CODEC_ID_NELLYMOSER: AVCodecID = 86049;
pub const AV_CODEC_ID_APE: AVCodecID = 86048;
pub const AV_CODEC_ID_ATRAC3: AVCodecID = 86047;
pub const AV_CODEC_ID_GSM_MS: AVCodecID = 86046;
pub const AV_CODEC_ID_MLP: AVCodecID = 86045;
pub const AV_CODEC_ID_MUSEPACK7: AVCodecID = 86044;
pub const AV_CODEC_ID_IMC: AVCodecID = 86043;
pub const AV_CODEC_ID_DSICINAUDIO: AVCodecID = 86042;
pub const AV_CODEC_ID_WAVPACK: AVCodecID = 86041;
pub const AV_CODEC_ID_QCELP: AVCodecID = 86040;
pub const AV_CODEC_ID_SMACKAUDIO: AVCodecID = 86039;
pub const AV_CODEC_ID_TTA: AVCodecID = 86038;
pub const AV_CODEC_ID_TRUESPEECH: AVCodecID = 86037;
pub const AV_CODEC_ID_COOK: AVCodecID = 86036;
pub const AV_CODEC_ID_QDM2: AVCodecID = 86035;
pub const AV_CODEC_ID_GSM: AVCodecID = 86034;
pub const AV_CODEC_ID_WESTWOOD_SND1: AVCodecID = 86033;
pub const AV_CODEC_ID_ALAC: AVCodecID = 86032;
pub const AV_CODEC_ID_SHORTEN: AVCodecID = 86031;
pub const AV_CODEC_ID_MP3ON4: AVCodecID = 86030;
pub const AV_CODEC_ID_MP3ADU: AVCodecID = 86029;
pub const AV_CODEC_ID_FLAC: AVCodecID = 86028;
pub const AV_CODEC_ID_VMDAUDIO: AVCodecID = 86027;
pub const AV_CODEC_ID_MACE6: AVCodecID = 86026;
pub const AV_CODEC_ID_MACE3: AVCodecID = 86025;
pub const AV_CODEC_ID_WMAV2: AVCodecID = 86024;
pub const AV_CODEC_ID_WMAV1: AVCodecID = 86023;
pub const AV_CODEC_ID_DVAUDIO: AVCodecID = 86022;
pub const AV_CODEC_ID_VORBIS: AVCodecID = 86021;
pub const AV_CODEC_ID_DTS: AVCodecID = 86020;
pub const AV_CODEC_ID_AC3: AVCodecID = 86019;
pub const AV_CODEC_ID_AAC: AVCodecID = 86018;
pub const AV_CODEC_ID_MP3: AVCodecID = 86017;
pub const AV_CODEC_ID_MP2: AVCodecID = 86016;
pub const AV_CODEC_ID_CBD2_DPCM: AVCodecID = 81928;
pub const AV_CODEC_ID_WADY_DPCM: AVCodecID = 81927;
pub const AV_CODEC_ID_DERF_DPCM: AVCodecID = 81926;
pub const AV_CODEC_ID_GREMLIN_DPCM: AVCodecID = 81925;
pub const AV_CODEC_ID_SDX2_DPCM: AVCodecID = 81924;
pub const AV_CODEC_ID_SOL_DPCM: AVCodecID = 81923;
pub const AV_CODEC_ID_XAN_DPCM: AVCodecID = 81922;
pub const AV_CODEC_ID_INTERPLAY_DPCM: AVCodecID = 81921;
pub const AV_CODEC_ID_ROQ_DPCM: AVCodecID = 81920;
pub const AV_CODEC_ID_RA_288: AVCodecID = 77825;
pub const AV_CODEC_ID_RA_144: AVCodecID = 77824;
pub const AV_CODEC_ID_AMR_WB: AVCodecID = 73729;
pub const AV_CODEC_ID_AMR_NB: AVCodecID = 73728;
pub const AV_CODEC_ID_ADPCM_XMD: AVCodecID = 69683;
pub const AV_CODEC_ID_ADPCM_IMA_ACORN: AVCodecID = 69682;
pub const AV_CODEC_ID_ADPCM_IMA_MOFLEX: AVCodecID = 69681;
pub const AV_CODEC_ID_ADPCM_IMA_CUNNING: AVCodecID = 69680;
pub const AV_CODEC_ID_ADPCM_IMA_MTF: AVCodecID = 69679;
pub const AV_CODEC_ID_ADPCM_IMA_ALP: AVCodecID = 69678;
pub const AV_CODEC_ID_ADPCM_IMA_APM: AVCodecID = 69677;
pub const AV_CODEC_ID_ADPCM_ZORK: AVCodecID = 69676;
pub const AV_CODEC_ID_ADPCM_IMA_SSI: AVCodecID = 69675;
pub const AV_CODEC_ID_ADPCM_ARGO: AVCodecID = 69674;
pub const AV_CODEC_ID_ADPCM_AGM: AVCodecID = 69673;
pub const AV_CODEC_ID_ADPCM_MTAF: AVCodecID = 69672;
pub const AV_CODEC_ID_ADPCM_IMA_DAT4: AVCodecID = 69671;
pub const AV_CODEC_ID_ADPCM_AICA: AVCodecID = 69670;
pub const AV_CODEC_ID_ADPCM_PSX: AVCodecID = 69669;
pub const AV_CODEC_ID_ADPCM_THP_LE: AVCodecID = 69668;
pub const AV_CODEC_ID_ADPCM_G726LE: AVCodecID = 69667;
pub const AV_CODEC_ID_ADPCM_IMA_RAD: AVCodecID = 69666;
pub const AV_CODEC_ID_ADPCM_DTK: AVCodecID = 69665;
pub const AV_CODEC_ID_ADPCM_IMA_OKI: AVCodecID = 69664;
pub const AV_CODEC_ID_ADPCM_AFC: AVCodecID = 69663;
pub const AV_CODEC_ID_ADPCM_VIMA: AVCodecID = 69662;
pub const AV_CODEC_ID_ADPCM_IMA_APC: AVCodecID = 69661;
pub const AV_CODEC_ID_ADPCM_G722: AVCodecID = 69660;
pub const AV_CODEC_ID_ADPCM_IMA_ISS: AVCodecID = 69659;
pub const AV_CODEC_ID_ADPCM_EA_MAXIS_XA: AVCodecID = 69658;
pub const AV_CODEC_ID_ADPCM_EA_XAS: AVCodecID = 69657;
pub const AV_CODEC_ID_ADPCM_IMA_EA_EACS: AVCodecID = 69656;
pub const AV_CODEC_ID_ADPCM_IMA_EA_SEAD: AVCodecID = 69655;
pub const AV_CODEC_ID_ADPCM_EA_R2: AVCodecID = 69654;
pub const AV_CODEC_ID_ADPCM_EA_R3: AVCodecID = 69653;
pub const AV_CODEC_ID_ADPCM_EA_R1: AVCodecID = 69652;
pub const AV_CODEC_ID_ADPCM_IMA_AMV: AVCodecID = 69651;
pub const AV_CODEC_ID_ADPCM_THP: AVCodecID = 69650;
pub const AV_CODEC_ID_ADPCM_SBPRO_2: AVCodecID = 69649;
pub const AV_CODEC_ID_ADPCM_SBPRO_3: AVCodecID = 69648;
pub const AV_CODEC_ID_ADPCM_SBPRO_4: AVCodecID = 69647;
pub const AV_CODEC_ID_ADPCM_YAMAHA: AVCodecID = 69646;
pub const AV_CODEC_ID_ADPCM_SWF: AVCodecID = 69645;
pub const AV_CODEC_ID_ADPCM_CT: AVCodecID = 69644;
pub const AV_CODEC_ID_ADPCM_G726: AVCodecID = 69643;
pub const AV_CODEC_ID_ADPCM_EA: AVCodecID = 69642;
pub const AV_CODEC_ID_ADPCM_ADX: AVCodecID = 69641;
pub const AV_CODEC_ID_ADPCM_XA: AVCodecID = 69640;
pub const AV_CODEC_ID_ADPCM_4XM: AVCodecID = 69639;
pub const AV_CODEC_ID_ADPCM_MS: AVCodecID = 69638;
pub const AV_CODEC_ID_ADPCM_IMA_SMJPEG: AVCodecID = 69637;
pub const AV_CODEC_ID_ADPCM_IMA_WS: AVCodecID = 69636;
pub const AV_CODEC_ID_ADPCM_IMA_DK4: AVCodecID = 69635;
pub const AV_CODEC_ID_ADPCM_IMA_DK3: AVCodecID = 69634;
pub const AV_CODEC_ID_ADPCM_IMA_WAV: AVCodecID = 69633;
pub const AV_CODEC_ID_ADPCM_IMA_QT: AVCodecID = 69632;
pub const AV_CODEC_ID_PCM_SGA: AVCodecID = 65572;
pub const AV_CODEC_ID_PCM_VIDC: AVCodecID = 65571;
pub const AV_CODEC_ID_PCM_F24LE: AVCodecID = 65570;
pub const AV_CODEC_ID_PCM_F16LE: AVCodecID = 65569;
pub const AV_CODEC_ID_PCM_S64BE: AVCodecID = 65568;
pub const AV_CODEC_ID_PCM_S64LE: AVCodecID = 65567;
pub const AV_CODEC_ID_PCM_S16BE_PLANAR: AVCodecID = 65566;
pub const AV_CODEC_ID_PCM_S32LE_PLANAR: AVCodecID = 65565;
pub const AV_CODEC_ID_PCM_S24LE_PLANAR: AVCodecID = 65564;
pub const AV_CODEC_ID_PCM_S8_PLANAR: AVCodecID = 65563;
pub const AV_CODEC_ID_S302M: AVCodecID = 65562;
pub const AV_CODEC_ID_PCM_LXF: AVCodecID = 65561;
pub const AV_CODEC_ID_PCM_BLURAY: AVCodecID = 65560;
pub const AV_CODEC_ID_PCM_F64LE: AVCodecID = 65559;
pub const AV_CODEC_ID_PCM_F64BE: AVCodecID = 65558;
pub const AV_CODEC_ID_PCM_F32LE: AVCodecID = 65557;
pub const AV_CODEC_ID_PCM_F32BE: AVCodecID = 65556;
pub const AV_CODEC_ID_PCM_DVD: AVCodecID = 65555;
pub const AV_CODEC_ID_PCM_S16LE_PLANAR: AVCodecID = 65554;
pub const AV_CODEC_ID_PCM_ZORK: AVCodecID = 65553;
pub const AV_CODEC_ID_PCM_S24DAUD: AVCodecID = 65552;
pub const AV_CODEC_ID_PCM_U24BE: AVCodecID = 65551;
pub const AV_CODEC_ID_PCM_U24LE: AVCodecID = 65550;
pub const AV_CODEC_ID_PCM_S24BE: AVCodecID = 65549;
pub const AV_CODEC_ID_PCM_S24LE: AVCodecID = 65548;
pub const AV_CODEC_ID_PCM_U32BE: AVCodecID = 65547;
pub const AV_CODEC_ID_PCM_U32LE: AVCodecID = 65546;
pub const AV_CODEC_ID_PCM_S32BE: AVCodecID = 65545;
pub const AV_CODEC_ID_PCM_S32LE: AVCodecID = 65544;
pub const AV_CODEC_ID_PCM_ALAW: AVCodecID = 65543;
pub const AV_CODEC_ID_PCM_MULAW: AVCodecID = 65542;
pub const AV_CODEC_ID_PCM_U8: AVCodecID = 65541;
pub const AV_CODEC_ID_PCM_S8: AVCodecID = 65540;
pub const AV_CODEC_ID_PCM_U16BE: AVCodecID = 65539;
pub const AV_CODEC_ID_PCM_U16LE: AVCodecID = 65538;
pub const AV_CODEC_ID_PCM_S16BE: AVCodecID = 65537;
pub const AV_CODEC_ID_PCM_S16LE: AVCodecID = 65536;
pub const AV_CODEC_ID_FIRST_AUDIO: AVCodecID = 65536;
pub const AV_CODEC_ID_LEAD: AVCodecID = 270;
pub const AV_CODEC_ID_VMIX: AVCodecID = 269;
pub const AV_CODEC_ID_RTV1: AVCodecID = 268;
pub const AV_CODEC_ID_EVC: AVCodecID = 267;
pub const AV_CODEC_ID_PDV: AVCodecID = 266;
pub const AV_CODEC_ID_VQC: AVCodecID = 265;
pub const AV_CODEC_ID_MEDIA100: AVCodecID = 264;
pub const AV_CODEC_ID_WBMP: AVCodecID = 263;
pub const AV_CODEC_ID_RADIANCE_HDR: AVCodecID = 262;
pub const AV_CODEC_ID_PHM: AVCodecID = 261;
pub const AV_CODEC_ID_QOI: AVCodecID = 260;
pub const AV_CODEC_ID_JPEGXL: AVCodecID = 259;
pub const AV_CODEC_ID_VBN: AVCodecID = 258;
pub const AV_CODEC_ID_GEM: AVCodecID = 257;
pub const AV_CODEC_ID_SGA_VIDEO: AVCodecID = 256;
pub const AV_CODEC_ID_SIMBIOSIS_IMX: AVCodecID = 255;
pub const AV_CODEC_ID_CRI: AVCodecID = 254;
pub const AV_CODEC_ID_ARGO: AVCodecID = 253;
pub const AV_CODEC_ID_IPU: AVCodecID = 252;
pub const AV_CODEC_ID_PHOTOCD: AVCodecID = 251;
pub const AV_CODEC_ID_MOBICLIP: AVCodecID = 250;
pub const AV_CODEC_ID_PFM: AVCodecID = 249;
pub const AV_CODEC_ID_NOTCHLC: AVCodecID = 248;
pub const AV_CODEC_ID_MV30: AVCodecID = 247;
pub const AV_CODEC_ID_CDTOONS: AVCodecID = 246;
pub const AV_CODEC_ID_MVHA: AVCodecID = 245;
pub const AV_CODEC_ID_MVDV: AVCodecID = 244;
pub const AV_CODEC_ID_IMM5: AVCodecID = 243;
pub const AV_CODEC_ID_VP4: AVCodecID = 242;
pub const AV_CODEC_ID_LSCR: AVCodecID = 241;
pub const AV_CODEC_ID_AGM: AVCodecID = 240;
pub const AV_CODEC_ID_ARBC: AVCodecID = 239;
pub const AV_CODEC_ID_HYMT: AVCodecID = 238;
pub const AV_CODEC_ID_RASC: AVCodecID = 237;
pub const AV_CODEC_ID_WCMV: AVCodecID = 236;
pub const AV_CODEC_ID_MWSC: AVCodecID = 235;
pub const AV_CODEC_ID_PROSUMER: AVCodecID = 234;
pub const AV_CODEC_ID_IMM4: AVCodecID = 233;
pub const AV_CODEC_ID_FITS: AVCodecID = 232;
pub const AV_CODEC_ID_GDV: AVCodecID = 231;
pub const AV_CODEC_ID_SVG: AVCodecID = 230;
pub const AV_CODEC_ID_SRGC: AVCodecID = 229;
pub const AV_CODEC_ID_MSCC: AVCodecID = 228;
pub const AV_CODEC_ID_BITPACKED: AVCodecID = 227;
pub const AV_CODEC_ID_AV1: AVCodecID = 226;
pub const AV_CODEC_ID_XPM: AVCodecID = 225;
pub const AV_CODEC_ID_CLEARVIDEO: AVCodecID = 224;
pub const AV_CODEC_ID_SCPR: AVCodecID = 223;
pub const AV_CODEC_ID_FMVC: AVCodecID = 222;
pub const AV_CODEC_ID_SPEEDHQ: AVCodecID = 221;
pub const AV_CODEC_ID_PIXLET: AVCodecID = 220;
pub const AV_CODEC_ID_PSD: AVCodecID = 219;
pub const AV_CODEC_ID_YLC: AVCodecID = 218;
pub const AV_CODEC_ID_SHEERVIDEO: AVCodecID = 217;
pub const AV_CODEC_ID_MAGICYUV: AVCodecID = 216;
pub const AV_CODEC_ID_M101: AVCodecID = 215;
pub const AV_CODEC_ID_TRUEMOTION2RT: AVCodecID = 214;
pub const AV_CODEC_ID_CFHD: AVCodecID = 213;
pub const AV_CODEC_ID_DAALA: AVCodecID = 212;
pub const AV_CODEC_ID_APNG: AVCodecID = 211;
pub const AV_CODEC_ID_SMVJPEG: AVCodecID = 210;
pub const AV_CODEC_ID_SNOW: AVCodecID = 209;
pub const AV_CODEC_ID_XFACE: AVCodecID = 208;
pub const AV_CODEC_ID_CPIA: AVCodecID = 207;
pub const AV_CODEC_ID_AVRN: AVCodecID = 206;
pub const AV_CODEC_ID_YUV4: AVCodecID = 205;
pub const AV_CODEC_ID_V408: AVCodecID = 204;
pub const AV_CODEC_ID_V308: AVCodecID = 203;
pub const AV_CODEC_ID_TARGA_Y216: AVCodecID = 202;
pub const AV_CODEC_ID_AYUV: AVCodecID = 201;
pub const AV_CODEC_ID_AVUI: AVCodecID = 200;
pub const AV_CODEC_ID_012V: AVCodecID = 199;
pub const AV_CODEC_ID_AVRP: AVCodecID = 198;
pub const AV_CODEC_ID_Y41P: AVCodecID = 197;
pub const AV_CODEC_ID_VVC: AVCodecID = 196;
pub const AV_CODEC_ID_MSP2: AVCodecID = 195;
pub const AV_CODEC_ID_AVS3: AVCodecID = 194;
pub const AV_CODEC_ID_PGX: AVCodecID = 193;
pub const AV_CODEC_ID_AVS2: AVCodecID = 192;
pub const AV_CODEC_ID_RSCC: AVCodecID = 191;
pub const AV_CODEC_ID_SCREENPRESSO: AVCodecID = 190;
pub const AV_CODEC_ID_DXV: AVCodecID = 189;
pub const AV_CODEC_ID_DDS: AVCodecID = 188;
pub const AV_CODEC_ID_HAP: AVCodecID = 187;
pub const AV_CODEC_ID_HQ_HQA: AVCodecID = 186;
pub const AV_CODEC_ID_TDSC: AVCodecID = 185;
pub const AV_CODEC_ID_HQX: AVCodecID = 184;
pub const AV_CODEC_ID_MVC2: AVCodecID = 183;
pub const AV_CODEC_ID_MVC1: AVCodecID = 182;
pub const AV_CODEC_ID_SGIRLE: AVCodecID = 181;
pub const AV_CODEC_ID_SANM: AVCodecID = 180;
pub const AV_CODEC_ID_VP7: AVCodecID = 179;
pub const AV_CODEC_ID_EXR: AVCodecID = 178;
pub const AV_CODEC_ID_PAF_VIDEO: AVCodecID = 177;
pub const AV_CODEC_ID_BRENDER_PIX: AVCodecID = 176;
pub const AV_CODEC_ID_ALIAS_PIX: AVCodecID = 175;
pub const AV_CODEC_ID_FIC: AVCodecID = 174;
pub const AV_CODEC_ID_HEVC: AVCodecID = 173;
pub const AV_CODEC_ID_HNM4_VIDEO: AVCodecID = 172;
pub const AV_CODEC_ID_WEBP: AVCodecID = 171;
pub const AV_CODEC_ID_G2M: AVCodecID = 170;
pub const AV_CODEC_ID_ESCAPE130: AVCodecID = 169;
pub const AV_CODEC_ID_AIC: AVCodecID = 168;
pub const AV_CODEC_ID_VP9: AVCodecID = 167;
pub const AV_CODEC_ID_MSS2: AVCodecID = 166;
pub const AV_CODEC_ID_CLLC: AVCodecID = 165;
pub const AV_CODEC_ID_MTS2: AVCodecID = 164;
pub const AV_CODEC_ID_TSCC2: AVCodecID = 163;
pub const AV_CODEC_ID_MSA1: AVCodecID = 162;
pub const AV_CODEC_ID_MSS1: AVCodecID = 161;
pub const AV_CODEC_ID_ZEROCODEC: AVCodecID = 160;
pub const AV_CODEC_ID_XBM: AVCodecID = 159;
pub const AV_CODEC_ID_CDXL: AVCodecID = 158;
pub const AV_CODEC_ID_XWD: AVCodecID = 157;
pub const AV_CODEC_ID_V410: AVCodecID = 156;
pub const AV_CODEC_ID_DXTORY: AVCodecID = 155;
pub const AV_CODEC_ID_VBLE: AVCodecID = 154;
pub const AV_CODEC_ID_BMV_VIDEO: AVCodecID = 153;
pub const AV_CODEC_ID_UTVIDEO: AVCodecID = 152;
pub const AV_CODEC_ID_VC1IMAGE: AVCodecID = 151;
pub const AV_CODEC_ID_WMV3IMAGE: AVCodecID = 150;
pub const AV_CODEC_ID_DFA: AVCodecID = 149;
pub const AV_CODEC_ID_JV: AVCodecID = 148;
pub const AV_CODEC_ID_PRORES: AVCodecID = 147;
pub const AV_CODEC_ID_LAGARITH: AVCodecID = 146;
pub const AV_CODEC_ID_MXPEG: AVCodecID = 145;
pub const AV_CODEC_ID_R10K: AVCodecID = 144;
pub const AV_CODEC_ID_A64_MULTI5: AVCodecID = 143;
pub const AV_CODEC_ID_A64_MULTI: AVCodecID = 142;
pub const AV_CODEC_ID_ANSI: AVCodecID = 141;
pub const AV_CODEC_ID_PICTOR: AVCodecID = 140;
pub const AV_CODEC_ID_VP8: AVCodecID = 139;
pub const AV_CODEC_ID_YOP: AVCodecID = 138;
pub const AV_CODEC_ID_KGV1: AVCodecID = 137;
pub const AV_CODEC_ID_IFF_ILBM: AVCodecID = 136;
pub const AV_CODEC_ID_BINKVIDEO: AVCodecID = 135;
pub const AV_CODEC_ID_ANM: AVCodecID = 134;
pub const AV_CODEC_ID_R210: AVCodecID = 133;
pub const AV_CODEC_ID_CDGRAPHICS: AVCodecID = 132;
pub const AV_CODEC_ID_FLASHSV2: AVCodecID = 131;
pub const AV_CODEC_ID_FRWU: AVCodecID = 130;
pub const AV_CODEC_ID_MAD: AVCodecID = 129;
pub const AV_CODEC_ID_DPX: AVCodecID = 128;
pub const AV_CODEC_ID_V210: AVCodecID = 127;
pub const AV_CODEC_ID_TMV: AVCodecID = 126;
pub const AV_CODEC_ID_V210X: AVCodecID = 125;
pub const AV_CODEC_ID_AURA2: AVCodecID = 124;
pub const AV_CODEC_ID_AURA: AVCodecID = 123;
pub const AV_CODEC_ID_TQI: AVCodecID = 122;
pub const AV_CODEC_ID_TGQ: AVCodecID = 121;
pub const AV_CODEC_ID_TGV: AVCodecID = 120;
pub const AV_CODEC_ID_MOTIONPIXELS: AVCodecID = 119;
pub const AV_CODEC_ID_CMV: AVCodecID = 118;
pub const AV_CODEC_ID_BFI: AVCodecID = 117;
pub const AV_CODEC_ID_DIRAC: AVCodecID = 116;
pub const AV_CODEC_ID_ESCAPE124: AVCodecID = 115;
pub const AV_CODEC_ID_RL2: AVCodecID = 114;
pub const AV_CODEC_ID_MIMIC: AVCodecID = 113;
pub const AV_CODEC_ID_INDEO5: AVCodecID = 112;
pub const AV_CODEC_ID_INDEO4: AVCodecID = 111;
pub const AV_CODEC_ID_SUNRAST: AVCodecID = 110;
pub const AV_CODEC_ID_PCX: AVCodecID = 109;
pub const AV_CODEC_ID_VB: AVCodecID = 108;
pub const AV_CODEC_ID_AMV: AVCodecID = 107;
pub const AV_CODEC_ID_VP6A: AVCodecID = 106;
pub const AV_CODEC_ID_TXD: AVCodecID = 105;
pub const AV_CODEC_ID_PTX: AVCodecID = 104;
pub const AV_CODEC_ID_BETHSOFTVID: AVCodecID = 103;
pub const AV_CODEC_ID_C93: AVCodecID = 102;
pub const AV_CODEC_ID_SGI: AVCodecID = 101;
pub const AV_CODEC_ID_THP: AVCodecID = 100;
pub const AV_CODEC_ID_DNXHD: AVCodecID = 99;
pub const AV_CODEC_ID_DXA: AVCodecID = 98;
pub const AV_CODEC_ID_GIF: AVCodecID = 97;
pub const AV_CODEC_ID_TIFF: AVCodecID = 96;
pub const AV_CODEC_ID_TIERTEXSEQVIDEO: AVCodecID = 95;
pub const AV_CODEC_ID_DSICINVIDEO: AVCodecID = 94;
pub const AV_CODEC_ID_TARGA: AVCodecID = 93;
pub const AV_CODEC_ID_VP6F: AVCodecID = 92;
pub const AV_CODEC_ID_VP6: AVCodecID = 91;
pub const AV_CODEC_ID_VP5: AVCodecID = 90;
pub const AV_CODEC_ID_VMNC: AVCodecID = 89;
pub const AV_CODEC_ID_JPEG2000: AVCodecID = 88;
pub const AV_CODEC_ID_CAVS: AVCodecID = 87;
pub const AV_CODEC_ID_FLASHSV: AVCodecID = 86;
pub const AV_CODEC_ID_KMVC: AVCodecID = 85;
pub const AV_CODEC_ID_NUV: AVCodecID = 84;
pub const AV_CODEC_ID_SMACKVIDEO: AVCodecID = 83;
pub const AV_CODEC_ID_AVS: AVCodecID = 82;
pub const AV_CODEC_ID_ZMBV: AVCodecID = 81;
pub const AV_CODEC_ID_MMVIDEO: AVCodecID = 80;
pub const AV_CODEC_ID_CSCD: AVCodecID = 79;
pub const AV_CODEC_ID_BMP: AVCodecID = 78;
pub const AV_CODEC_ID_TRUEMOTION2: AVCodecID = 77;
pub const AV_CODEC_ID_FRAPS: AVCodecID = 76;
pub const AV_CODEC_ID_INDEO2: AVCodecID = 75;
pub const AV_CODEC_ID_AASC: AVCodecID = 74;
pub const AV_CODEC_ID_WNV1: AVCodecID = 73;
pub const AV_CODEC_ID_LOCO: AVCodecID = 72;
pub const AV_CODEC_ID_WMV3: AVCodecID = 71;
pub const AV_CODEC_ID_VC1: AVCodecID = 70;
pub const AV_CODEC_ID_RV40: AVCodecID = 69;
pub const AV_CODEC_ID_RV30: AVCodecID = 68;
pub const AV_CODEC_ID_FFVHUFF: AVCodecID = 67;
pub const AV_CODEC_ID_PAM: AVCodecID = 66;
pub const AV_CODEC_ID_PGMYUV: AVCodecID = 65;
pub const AV_CODEC_ID_PGM: AVCodecID = 64;
pub const AV_CODEC_ID_PBM: AVCodecID = 63;
pub const AV_CODEC_ID_PPM: AVCodecID = 62;
pub const AV_CODEC_ID_PNG: AVCodecID = 61;
pub const AV_CODEC_ID_QPEG: AVCodecID = 60;
pub const AV_CODEC_ID_VIXL: AVCodecID = 59;
pub const AV_CODEC_ID_QDRAW: AVCodecID = 58;
pub const AV_CODEC_ID_ULTI: AVCodecID = 57;
pub const AV_CODEC_ID_TSCC: AVCodecID = 56;
pub const AV_CODEC_ID_QTRLE: AVCodecID = 55;
pub const AV_CODEC_ID_ZLIB: AVCodecID = 54;
pub const AV_CODEC_ID_MSZH: AVCodecID = 53;
pub const AV_CODEC_ID_VMDVIDEO: AVCodecID = 52;
pub const AV_CODEC_ID_TRUEMOTION1: AVCodecID = 51;
pub const AV_CODEC_ID_FLIC: AVCodecID = 50;
pub const AV_CODEC_ID_SMC: AVCodecID = 49;
pub const AV_CODEC_ID_8BPS: AVCodecID = 48;
pub const AV_CODEC_ID_IDCIN: AVCodecID = 47;
pub const AV_CODEC_ID_MSVIDEO1: AVCodecID = 46;
pub const AV_CODEC_ID_MSRLE: AVCodecID = 45;
pub const AV_CODEC_ID_WS_VQA: AVCodecID = 44;
pub const AV_CODEC_ID_CINEPAK: AVCodecID = 43;
pub const AV_CODEC_ID_RPZA: AVCodecID = 42;
pub const AV_CODEC_ID_XAN_WC4: AVCodecID = 41;
pub const AV_CODEC_ID_XAN_WC3: AVCodecID = 40;
pub const AV_CODEC_ID_INTERPLAY_VIDEO: AVCodecID = 39;
pub const AV_CODEC_ID_ROQ: AVCodecID = 38;
pub const AV_CODEC_ID_MDEC: AVCodecID = 37;
pub const AV_CODEC_ID_CLJR: AVCodecID = 36;
pub const AV_CODEC_ID_VCR1: AVCodecID = 35;
pub const AV_CODEC_ID_4XM: AVCodecID = 34;
pub const AV_CODEC_ID_FFV1: AVCodecID = 33;
pub const AV_CODEC_ID_ASV2: AVCodecID = 32;
pub const AV_CODEC_ID_ASV1: AVCodecID = 31;
pub const AV_CODEC_ID_THEORA: AVCodecID = 30;
pub const AV_CODEC_ID_VP3: AVCodecID = 29;
pub const AV_CODEC_ID_INDEO3: AVCodecID = 28;
pub const AV_CODEC_ID_H264: AVCodecID = 27;
pub const AV_CODEC_ID_CYUV: AVCodecID = 26;
pub const AV_CODEC_ID_HUFFYUV: AVCodecID = 25;
pub const AV_CODEC_ID_DVVIDEO: AVCodecID = 24;
pub const AV_CODEC_ID_SVQ3: AVCodecID = 23;
pub const AV_CODEC_ID_SVQ1: AVCodecID = 22;
pub const AV_CODEC_ID_FLV1: AVCodecID = 21;
pub const AV_CODEC_ID_H263I: AVCodecID = 20;
pub const AV_CODEC_ID_H263P: AVCodecID = 19;
pub const AV_CODEC_ID_WMV2: AVCodecID = 18;
pub const AV_CODEC_ID_WMV1: AVCodecID = 17;
pub const AV_CODEC_ID_MSMPEG4V3: AVCodecID = 16;
pub const AV_CODEC_ID_MSMPEG4V2: AVCodecID = 15;
pub const AV_CODEC_ID_MSMPEG4V1: AVCodecID = 14;
pub const AV_CODEC_ID_RAWVIDEO: AVCodecID = 13;
pub const AV_CODEC_ID_MPEG4: AVCodecID = 12;
pub const AV_CODEC_ID_JPEGLS: AVCodecID = 11;
pub const AV_CODEC_ID_SP5X: AVCodecID = 10;
pub const AV_CODEC_ID_LJPEG: AVCodecID = 9;
pub const AV_CODEC_ID_MJPEGB: AVCodecID = 8;
pub const AV_CODEC_ID_MJPEG: AVCodecID = 7;
pub const AV_CODEC_ID_RV20: AVCodecID = 6;
pub const AV_CODEC_ID_RV10: AVCodecID = 5;
pub const AV_CODEC_ID_H263: AVCodecID = 4;
pub const AV_CODEC_ID_H261: AVCodecID = 3;
pub const AV_CODEC_ID_MPEG2VIDEO: AVCodecID = 2;
pub const AV_CODEC_ID_MPEG1VIDEO: AVCodecID = 1;
pub const AV_CODEC_ID_NONE: AVCodecID = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVProfile {
    pub profile: libc::c_int,
    pub name: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodec {
    pub name: *const libc::c_char,
    pub long_name: *const libc::c_char,
    pub type_0: AVMediaType,
    pub id: AVCodecID,
    pub capabilities: libc::c_int,
    pub max_lowres: uint8_t,
    pub supported_framerates: *const AVRational,
    pub pix_fmts: *const AVPixelFormat,
    pub supported_samplerates: *const libc::c_int,
    pub sample_fmts: *const AVSampleFormat,
    pub channel_layouts: *const uint64_t,
    pub priv_class: *const AVClass,
    pub profiles: *const AVProfile,
    pub wrapper_name: *const libc::c_char,
    pub ch_layouts: *const AVChannelLayout,
}
pub type AVFieldOrder = libc::c_uint;
pub const AV_FIELD_BT: AVFieldOrder = 5;
pub const AV_FIELD_TB: AVFieldOrder = 4;
pub const AV_FIELD_BB: AVFieldOrder = 3;
pub const AV_FIELD_TT: AVFieldOrder = 2;
pub const AV_FIELD_PROGRESSIVE: AVFieldOrder = 1;
pub const AV_FIELD_UNKNOWN: AVFieldOrder = 0;
pub type AVDiscard = libc::c_int;
pub const AVDISCARD_ALL: AVDiscard = 48;
pub const AVDISCARD_NONKEY: AVDiscard = 32;
pub const AVDISCARD_NONINTRA: AVDiscard = 24;
pub const AVDISCARD_BIDIR: AVDiscard = 16;
pub const AVDISCARD_NONREF: AVDiscard = 8;
pub const AVDISCARD_DEFAULT: AVDiscard = 0;
pub const AVDISCARD_NONE: AVDiscard = -16;
pub type AVAudioServiceType = libc::c_uint;
pub const AV_AUDIO_SERVICE_TYPE_NB: AVAudioServiceType = 9;
pub const AV_AUDIO_SERVICE_TYPE_KARAOKE: AVAudioServiceType = 8;
pub const AV_AUDIO_SERVICE_TYPE_VOICE_OVER: AVAudioServiceType = 7;
pub const AV_AUDIO_SERVICE_TYPE_EMERGENCY: AVAudioServiceType = 6;
pub const AV_AUDIO_SERVICE_TYPE_COMMENTARY: AVAudioServiceType = 5;
pub const AV_AUDIO_SERVICE_TYPE_DIALOGUE: AVAudioServiceType = 4;
pub const AV_AUDIO_SERVICE_TYPE_HEARING_IMPAIRED: AVAudioServiceType = 3;
pub const AV_AUDIO_SERVICE_TYPE_VISUALLY_IMPAIRED: AVAudioServiceType = 2;
pub const AV_AUDIO_SERVICE_TYPE_EFFECTS: AVAudioServiceType = 1;
pub const AV_AUDIO_SERVICE_TYPE_MAIN: AVAudioServiceType = 0;
pub type AVPacketSideDataType = libc::c_uint;
pub const AV_PKT_DATA_NB: AVPacketSideDataType = 32;
pub const AV_PKT_DATA_DYNAMIC_HDR10_PLUS: AVPacketSideDataType = 31;
pub const AV_PKT_DATA_S12M_TIMECODE: AVPacketSideDataType = 30;
pub const AV_PKT_DATA_DOVI_CONF: AVPacketSideDataType = 29;
pub const AV_PKT_DATA_ICC_PROFILE: AVPacketSideDataType = 28;
pub const AV_PKT_DATA_PRFT: AVPacketSideDataType = 27;
pub const AV_PKT_DATA_AFD: AVPacketSideDataType = 26;
pub const AV_PKT_DATA_ENCRYPTION_INFO: AVPacketSideDataType = 25;
pub const AV_PKT_DATA_ENCRYPTION_INIT_INFO: AVPacketSideDataType = 24;
pub const AV_PKT_DATA_A53_CC: AVPacketSideDataType = 23;
pub const AV_PKT_DATA_CONTENT_LIGHT_LEVEL: AVPacketSideDataType = 22;
pub const AV_PKT_DATA_SPHERICAL: AVPacketSideDataType = 21;
pub const AV_PKT_DATA_MASTERING_DISPLAY_METADATA: AVPacketSideDataType = 20;
pub const AV_PKT_DATA_MPEGTS_STREAM_ID: AVPacketSideDataType = 19;
pub const AV_PKT_DATA_METADATA_UPDATE: AVPacketSideDataType = 18;
pub const AV_PKT_DATA_WEBVTT_SETTINGS: AVPacketSideDataType = 17;
pub const AV_PKT_DATA_WEBVTT_IDENTIFIER: AVPacketSideDataType = 16;
pub const AV_PKT_DATA_MATROSKA_BLOCKADDITIONAL: AVPacketSideDataType = 15;
pub const AV_PKT_DATA_SUBTITLE_POSITION: AVPacketSideDataType = 14;
pub const AV_PKT_DATA_STRINGS_METADATA: AVPacketSideDataType = 13;
pub const AV_PKT_DATA_JP_DUALMONO: AVPacketSideDataType = 12;
pub const AV_PKT_DATA_SKIP_SAMPLES: AVPacketSideDataType = 11;
pub const AV_PKT_DATA_CPB_PROPERTIES: AVPacketSideDataType = 10;
pub const AV_PKT_DATA_FALLBACK_TRACK: AVPacketSideDataType = 9;
pub const AV_PKT_DATA_QUALITY_STATS: AVPacketSideDataType = 8;
pub const AV_PKT_DATA_AUDIO_SERVICE_TYPE: AVPacketSideDataType = 7;
pub const AV_PKT_DATA_STEREO3D: AVPacketSideDataType = 6;
pub const AV_PKT_DATA_DISPLAYMATRIX: AVPacketSideDataType = 5;
pub const AV_PKT_DATA_REPLAYGAIN: AVPacketSideDataType = 4;
pub const AV_PKT_DATA_H263_MB_INFO: AVPacketSideDataType = 3;
pub const AV_PKT_DATA_PARAM_CHANGE: AVPacketSideDataType = 2;
pub const AV_PKT_DATA_NEW_EXTRADATA: AVPacketSideDataType = 1;
pub const AV_PKT_DATA_PALETTE: AVPacketSideDataType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVPacketSideData {
    pub data: *mut uint8_t,
    pub size: size_t,
    pub type_0: AVPacketSideDataType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVPacket {
    pub buf: *mut AVBufferRef,
    pub pts: int64_t,
    pub dts: int64_t,
    pub data: *mut uint8_t,
    pub size: libc::c_int,
    pub stream_index: libc::c_int,
    pub flags: libc::c_int,
    pub side_data: *mut AVPacketSideData,
    pub side_data_elems: libc::c_int,
    pub duration: int64_t,
    pub pos: int64_t,
    pub opaque: *mut libc::c_void,
    pub opaque_ref: *mut AVBufferRef,
    pub time_base: AVRational,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RcOverride {
    pub start_frame: libc::c_int,
    pub end_frame: libc::c_int,
    pub qscale: libc::c_int,
    pub quality_factor: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecContext {
    pub av_class: *const AVClass,
    pub log_level_offset: libc::c_int,
    pub codec_type: AVMediaType,
    pub codec: *const AVCodec,
    pub codec_id: AVCodecID,
    pub codec_tag: libc::c_uint,
    pub priv_data: *mut libc::c_void,
    pub internal: *mut AVCodecInternal,
    pub opaque: *mut libc::c_void,
    pub bit_rate: int64_t,
    pub bit_rate_tolerance: libc::c_int,
    pub global_quality: libc::c_int,
    pub compression_level: libc::c_int,
    pub flags: libc::c_int,
    pub flags2: libc::c_int,
    pub extradata: *mut uint8_t,
    pub extradata_size: libc::c_int,
    pub time_base: AVRational,
    pub ticks_per_frame: libc::c_int,
    pub delay: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub coded_width: libc::c_int,
    pub coded_height: libc::c_int,
    pub gop_size: libc::c_int,
    pub pix_fmt: AVPixelFormat,
    pub draw_horiz_band: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *const AVFrame,
            *mut libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub get_format:
        Option<unsafe extern "C" fn(*mut AVCodecContext, *const AVPixelFormat) -> AVPixelFormat>,
    pub max_b_frames: libc::c_int,
    pub b_quant_factor: libc::c_float,
    pub b_quant_offset: libc::c_float,
    pub has_b_frames: libc::c_int,
    pub i_quant_factor: libc::c_float,
    pub i_quant_offset: libc::c_float,
    pub lumi_masking: libc::c_float,
    pub temporal_cplx_masking: libc::c_float,
    pub spatial_cplx_masking: libc::c_float,
    pub p_masking: libc::c_float,
    pub dark_masking: libc::c_float,
    pub slice_count: libc::c_int,
    pub slice_offset: *mut libc::c_int,
    pub sample_aspect_ratio: AVRational,
    pub me_cmp: libc::c_int,
    pub me_sub_cmp: libc::c_int,
    pub mb_cmp: libc::c_int,
    pub ildct_cmp: libc::c_int,
    pub dia_size: libc::c_int,
    pub last_predictor_count: libc::c_int,
    pub me_pre_cmp: libc::c_int,
    pub pre_dia_size: libc::c_int,
    pub me_subpel_quality: libc::c_int,
    pub me_range: libc::c_int,
    pub slice_flags: libc::c_int,
    pub mb_decision: libc::c_int,
    pub intra_matrix: *mut uint16_t,
    pub inter_matrix: *mut uint16_t,
    pub intra_dc_precision: libc::c_int,
    pub skip_top: libc::c_int,
    pub skip_bottom: libc::c_int,
    pub mb_lmin: libc::c_int,
    pub mb_lmax: libc::c_int,
    pub bidir_refine: libc::c_int,
    pub keyint_min: libc::c_int,
    pub refs: libc::c_int,
    pub mv0_threshold: libc::c_int,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub colorspace: AVColorSpace,
    pub color_range: AVColorRange,
    pub chroma_sample_location: AVChromaLocation,
    pub slices: libc::c_int,
    pub field_order: AVFieldOrder,
    pub sample_rate: libc::c_int,
    pub channels: libc::c_int,
    pub sample_fmt: AVSampleFormat,
    pub frame_size: libc::c_int,
    pub frame_number: libc::c_int,
    pub block_align: libc::c_int,
    pub cutoff: libc::c_int,
    pub channel_layout: uint64_t,
    pub request_channel_layout: uint64_t,
    pub audio_service_type: AVAudioServiceType,
    pub request_sample_fmt: AVSampleFormat,
    pub get_buffer2:
        Option<unsafe extern "C" fn(*mut AVCodecContext, *mut AVFrame, libc::c_int) -> libc::c_int>,
    pub qcompress: libc::c_float,
    pub qblur: libc::c_float,
    pub qmin: libc::c_int,
    pub qmax: libc::c_int,
    pub max_qdiff: libc::c_int,
    pub rc_buffer_size: libc::c_int,
    pub rc_override_count: libc::c_int,
    pub rc_override: *mut RcOverride,
    pub rc_max_rate: int64_t,
    pub rc_min_rate: int64_t,
    pub rc_max_available_vbv_use: libc::c_float,
    pub rc_min_vbv_overflow_use: libc::c_float,
    pub rc_initial_buffer_occupancy: libc::c_int,
    pub trellis: libc::c_int,
    pub stats_out: *mut libc::c_char,
    pub stats_in: *mut libc::c_char,
    pub workaround_bugs: libc::c_int,
    pub strict_std_compliance: libc::c_int,
    pub error_concealment: libc::c_int,
    pub debug: libc::c_int,
    pub err_recognition: libc::c_int,
    pub reordered_opaque: int64_t,
    pub hwaccel: *const AVHWAccel,
    pub hwaccel_context: *mut libc::c_void,
    pub error: [uint64_t; 8],
    pub dct_algo: libc::c_int,
    pub idct_algo: libc::c_int,
    pub bits_per_coded_sample: libc::c_int,
    pub bits_per_raw_sample: libc::c_int,
    pub lowres: libc::c_int,
    pub thread_count: libc::c_int,
    pub thread_type: libc::c_int,
    pub active_thread_type: libc::c_int,
    pub execute: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            Option<unsafe extern "C" fn(*mut AVCodecContext, *mut libc::c_void) -> libc::c_int>,
            *mut libc::c_void,
            *mut libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub execute2: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            Option<
                unsafe extern "C" fn(
                    *mut AVCodecContext,
                    *mut libc::c_void,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int,
            >,
            *mut libc::c_void,
            *mut libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub nsse_weight: libc::c_int,
    pub profile: libc::c_int,
    pub level: libc::c_int,
    pub skip_loop_filter: AVDiscard,
    pub skip_idct: AVDiscard,
    pub skip_frame: AVDiscard,
    pub subtitle_header: *mut uint8_t,
    pub subtitle_header_size: libc::c_int,
    pub initial_padding: libc::c_int,
    pub framerate: AVRational,
    pub sw_pix_fmt: AVPixelFormat,
    pub pkt_timebase: AVRational,
    pub codec_descriptor: *const AVCodecDescriptor,
    pub pts_correction_num_faulty_pts: int64_t,
    pub pts_correction_num_faulty_dts: int64_t,
    pub pts_correction_last_pts: int64_t,
    pub pts_correction_last_dts: int64_t,
    pub sub_charenc: *mut libc::c_char,
    pub sub_charenc_mode: libc::c_int,
    pub skip_alpha: libc::c_int,
    pub seek_preroll: libc::c_int,
    pub chroma_intra_matrix: *mut uint16_t,
    pub dump_separator: *mut uint8_t,
    pub codec_whitelist: *mut libc::c_char,
    pub properties: libc::c_uint,
    pub coded_side_data: *mut AVPacketSideData,
    pub nb_coded_side_data: libc::c_int,
    pub hw_frames_ctx: *mut AVBufferRef,
    pub trailing_padding: libc::c_int,
    pub max_pixels: int64_t,
    pub hw_device_ctx: *mut AVBufferRef,
    pub hwaccel_flags: libc::c_int,
    pub apply_cropping: libc::c_int,
    pub extra_hw_frames: libc::c_int,
    pub discard_damaged_percentage: libc::c_int,
    pub max_samples: int64_t,
    pub export_side_data: libc::c_int,
    pub get_encode_buffer: Option<
        unsafe extern "C" fn(*mut AVCodecContext, *mut AVPacket, libc::c_int) -> libc::c_int,
    >,
    pub ch_layout: AVChannelLayout,
    pub frame_num: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVHWAccel {
    pub name: *const libc::c_char,
    pub type_0: AVMediaType,
    pub id: AVCodecID,
    pub pix_fmt: AVPixelFormat,
    pub capabilities: libc::c_int,
}
pub type AVSubtitleType = libc::c_uint;
pub const SUBTITLE_ASS: AVSubtitleType = 3;
pub const SUBTITLE_TEXT: AVSubtitleType = 2;
pub const SUBTITLE_BITMAP: AVSubtitleType = 1;
pub const SUBTITLE_NONE: AVSubtitleType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVSubtitleRect {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub w: libc::c_int,
    pub h: libc::c_int,
    pub nb_colors: libc::c_int,
    pub data: [*mut uint8_t; 4],
    pub linesize: [libc::c_int; 4],
    pub type_0: AVSubtitleType,
    pub text: *mut libc::c_char,
    pub ass: *mut libc::c_char,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVSubtitle {
    pub format: uint16_t,
    pub start_display_time: uint32_t,
    pub end_display_time: uint32_t,
    pub num_rects: libc::c_uint,
    pub rects: *mut *mut AVSubtitleRect,
    pub pts: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFCodecDefault {
    pub key: *const libc::c_char,
    pub value: *const libc::c_char,
}
pub type FFCodecType = libc::c_uint;
pub const FF_CODEC_CB_TYPE_RECEIVE_PACKET: FFCodecType = 5;
pub const FF_CODEC_CB_TYPE_ENCODE_SUB: FFCodecType = 4;
pub const FF_CODEC_CB_TYPE_ENCODE: FFCodecType = 3;
pub const FF_CODEC_CB_TYPE_RECEIVE_FRAME: FFCodecType = 2;
pub const FF_CODEC_CB_TYPE_DECODE_SUB: FFCodecType = 1;
pub const FF_CODEC_CB_TYPE_DECODE: FFCodecType = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct FFCodec {
    pub p: AVCodec,
    #[bitfield(name = "caps_internal", ty = "libc::c_uint", bits = "0..=28")]
    #[bitfield(name = "cb_type", ty = "libc::c_uint", bits = "29..=31")]
    pub caps_internal_cb_type: [u8; 4],
    pub priv_data_size: libc::c_int,
    pub update_thread_context:
        Option<unsafe extern "C" fn(*mut AVCodecContext, *const AVCodecContext) -> libc::c_int>,
    pub update_thread_context_for_user:
        Option<unsafe extern "C" fn(*mut AVCodecContext, *const AVCodecContext) -> libc::c_int>,
    pub defaults: *const FFCodecDefault,
    pub init_static_data: Option<unsafe extern "C" fn(*mut FFCodec) -> ()>,
    pub init: Option<unsafe extern "C" fn(*mut AVCodecContext) -> libc::c_int>,
    pub cb: C2RustUnnamed_1,
    pub close: Option<unsafe extern "C" fn(*mut AVCodecContext) -> libc::c_int>,
    pub flush: Option<unsafe extern "C" fn(*mut AVCodecContext) -> ()>,
    pub bsfs: *const libc::c_char,
    pub hw_configs: *const *const AVCodecHWConfigInternal,
    pub codec_tags: *const uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub decode: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut AVFrame,
            *mut libc::c_int,
            *mut AVPacket,
        ) -> libc::c_int,
    >,
    pub decode_sub: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut AVSubtitle,
            *mut libc::c_int,
            *const AVPacket,
        ) -> libc::c_int,
    >,
    pub receive_frame:
        Option<unsafe extern "C" fn(*mut AVCodecContext, *mut AVFrame) -> libc::c_int>,
    pub encode: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut AVPacket,
            *const AVFrame,
            *mut libc::c_int,
        ) -> libc::c_int,
    >,
    pub encode_sub: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut uint8_t,
            libc::c_int,
            *const AVSubtitle,
        ) -> libc::c_int,
    >,
    pub receive_packet:
        Option<unsafe extern "C" fn(*mut AVCodecContext, *mut AVPacket) -> libc::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union unaligned_32 {
    pub l: uint32_t,
}
pub type BitBuf = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PutBitContext {
    pub bit_buf: BitBuf,
    pub bit_left: libc::c_int,
    pub buf: *mut uint8_t,
    pub buf_ptr: *mut uint8_t,
    pub buf_end: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MPEG4AudioConfig {
    pub object_type: libc::c_int,
    pub sampling_index: libc::c_int,
    pub sample_rate: libc::c_int,
    pub chan_config: libc::c_int,
    pub sbr: libc::c_int,
    pub ext_object_type: libc::c_int,
    pub ext_sampling_index: libc::c_int,
    pub ext_sample_rate: libc::c_int,
    pub ext_chan_config: libc::c_int,
    pub channels: libc::c_int,
    pub ps: libc::c_int,
    pub frame_length_short: libc::c_int,
}
pub type AudioObjectType = libc::c_uint;
pub const AOT_USAC: AudioObjectType = 45;
pub const AOT_LD_SURROUND: AudioObjectType = 44;
pub const AOT_SAOC: AudioObjectType = 43;
pub const AOT_USAC_NOSBR: AudioObjectType = 42;
pub const AOT_SMR_MAIN: AudioObjectType = 41;
pub const AOT_SMR_SIMPLE: AudioObjectType = 40;
pub const AOT_ER_AAC_ELD: AudioObjectType = 39;
pub const AOT_SLS_NON_CORE: AudioObjectType = 38;
pub const AOT_SLS: AudioObjectType = 37;
pub const AOT_ALS: AudioObjectType = 36;
pub const AOT_DST: AudioObjectType = 35;
pub const AOT_L3: AudioObjectType = 34;
pub const AOT_L2: AudioObjectType = 33;
pub const AOT_L1: AudioObjectType = 32;
pub const AOT_ESCAPE: AudioObjectType = 31;
pub const AOT_SURROUND: AudioObjectType = 30;
pub const AOT_PS: AudioObjectType = 29;
pub const AOT_SSC: AudioObjectType = 28;
pub const AOT_ER_PARAM: AudioObjectType = 27;
pub const AOT_ER_HILN: AudioObjectType = 26;
pub const AOT_ER_HVXC: AudioObjectType = 25;
pub const AOT_ER_CELP: AudioObjectType = 24;
pub const AOT_ER_AAC_LD: AudioObjectType = 23;
pub const AOT_ER_BSAC: AudioObjectType = 22;
pub const AOT_ER_TWINVQ: AudioObjectType = 21;
pub const AOT_ER_AAC_SCALABLE: AudioObjectType = 20;
pub const AOT_ER_AAC_LTP: AudioObjectType = 19;
pub const AOT_ER_AAC_LC: AudioObjectType = 17;
pub const AOT_SAFX: AudioObjectType = 16;
pub const AOT_MIDI: AudioObjectType = 15;
pub const AOT_WAVESYNTH: AudioObjectType = 14;
pub const AOT_MAINSYNTH: AudioObjectType = 13;
pub const AOT_TTSI: AudioObjectType = 12;
pub const AOT_HVXC: AudioObjectType = 9;
pub const AOT_CELP: AudioObjectType = 8;
pub const AOT_TWINVQ: AudioObjectType = 7;
pub const AOT_AAC_SCALABLE: AudioObjectType = 6;
pub const AOT_SBR: AudioObjectType = 5;
pub const AOT_AAC_LTP: AudioObjectType = 4;
pub const AOT_AAC_SSR: AudioObjectType = 3;
pub const AOT_AAC_LC: AudioObjectType = 2;
pub const AOT_AAC_MAIN: AudioObjectType = 1;
pub const AOT_NULL: AudioObjectType = 0;
pub type INTFLOAT = libc::c_float;
pub type AAC_FLOAT = libc::c_float;
pub type AAC_SIGNE = libc::c_uint;
pub type AVTXType = libc::c_uint;
pub const AV_TX_NB: AVTXType = 18;
pub const AV_TX_INT32_DST_I: AVTXType = 17;
pub const AV_TX_DOUBLE_DST_I: AVTXType = 16;
pub const AV_TX_FLOAT_DST_I: AVTXType = 15;
pub const AV_TX_INT32_DCT_I: AVTXType = 14;
pub const AV_TX_DOUBLE_DCT_I: AVTXType = 13;
pub const AV_TX_FLOAT_DCT_I: AVTXType = 12;
pub const AV_TX_INT32_DCT: AVTXType = 11;
pub const AV_TX_DOUBLE_DCT: AVTXType = 10;
pub const AV_TX_FLOAT_DCT: AVTXType = 9;
pub const AV_TX_INT32_RDFT: AVTXType = 8;
pub const AV_TX_DOUBLE_RDFT: AVTXType = 7;
pub const AV_TX_FLOAT_RDFT: AVTXType = 6;
pub const AV_TX_INT32_MDCT: AVTXType = 5;
pub const AV_TX_DOUBLE_MDCT: AVTXType = 3;
pub const AV_TX_FLOAT_MDCT: AVTXType = 1;
pub const AV_TX_INT32_FFT: AVTXType = 4;
pub const AV_TX_DOUBLE_FFT: AVTXType = 2;
pub const AV_TX_FLOAT_FFT: AVTXType = 0;
pub type av_tx_fn = Option<
    unsafe extern "C" fn(*mut AVTXContext, *mut libc::c_void, *mut libc::c_void, ptrdiff_t) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PSDSPContext {
    pub add_squares:
        Option<unsafe extern "C" fn(*mut INTFLOAT, *const [INTFLOAT; 2], libc::c_int) -> ()>,
    pub mul_pair_single: Option<
        unsafe extern "C" fn(
            *mut [INTFLOAT; 2],
            *mut [INTFLOAT; 2],
            *mut INTFLOAT,
            libc::c_int,
        ) -> (),
    >,
    pub hybrid_analysis: Option<
        unsafe extern "C" fn(
            *mut [INTFLOAT; 2],
            *mut [INTFLOAT; 2],
            *const [[INTFLOAT; 2]; 8],
            ptrdiff_t,
            libc::c_int,
        ) -> (),
    >,
    pub hybrid_analysis_ileave: Option<
        unsafe extern "C" fn(
            *mut [[INTFLOAT; 2]; 32],
            *mut [[INTFLOAT; 64]; 38],
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub hybrid_synthesis_deint: Option<
        unsafe extern "C" fn(
            *mut [[INTFLOAT; 64]; 38],
            *mut [[INTFLOAT; 2]; 32],
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub decorrelate: Option<
        unsafe extern "C" fn(
            *mut [INTFLOAT; 2],
            *mut [INTFLOAT; 2],
            *mut [[INTFLOAT; 2]; 37],
            *const INTFLOAT,
            *const [INTFLOAT; 2],
            *const INTFLOAT,
            INTFLOAT,
            libc::c_int,
        ) -> (),
    >,
    pub stereo_interpolate: [Option<
        unsafe extern "C" fn(
            *mut [INTFLOAT; 2],
            *mut [INTFLOAT; 2],
            *mut [INTFLOAT; 4],
            *mut [INTFLOAT; 4],
            libc::c_int,
        ) -> (),
    >; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PSCommonContext {
    pub start: libc::c_int,
    pub enable_iid: libc::c_int,
    pub iid_quant: libc::c_int,
    pub nr_iid_par: libc::c_int,
    pub nr_ipdopd_par: libc::c_int,
    pub enable_icc: libc::c_int,
    pub icc_mode: libc::c_int,
    pub nr_icc_par: libc::c_int,
    pub enable_ext: libc::c_int,
    pub frame_class: libc::c_int,
    pub num_env_old: libc::c_int,
    pub num_env: libc::c_int,
    pub enable_ipdopd: libc::c_int,
    pub border_position: [libc::c_int; 6],
    pub iid_par: [[int8_t; 34]; 5],
    pub icc_par: [[int8_t; 34]; 5],
    pub ipd_par: [[int8_t; 34]; 5],
    pub opd_par: [[int8_t; 34]; 5],
    pub is34bands: libc::c_int,
    pub is34bands_old: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PSContext {
    pub common: PSCommonContext,
    pub in_buf: [[[INTFLOAT; 2]; 44]; 5],
    pub delay: [[[INTFLOAT; 2]; 46]; 91],
    pub ap_delay: [[[[INTFLOAT; 2]; 37]; 3]; 50],
    pub peak_decay_nrg: [INTFLOAT; 34],
    pub power_smooth: [INTFLOAT; 34],
    pub peak_decay_diff_smooth: [INTFLOAT; 34],
    pub H11: [[[INTFLOAT; 34]; 6]; 2],
    pub H12: [[[INTFLOAT; 34]; 6]; 2],
    pub H21: [[[INTFLOAT; 34]; 6]; 2],
    pub H22: [[[INTFLOAT; 34]; 6]; 2],
    pub Lbuf: [[[INTFLOAT; 2]; 32]; 91],
    pub Rbuf: [[[INTFLOAT; 2]; 32]; 91],
    pub opd_hist: [int8_t; 34],
    pub ipd_hist: [int8_t; 34],
    pub dsp: PSDSPContext,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SBRDSPContext {
    pub sum64x5: Option<unsafe extern "C" fn(*mut INTFLOAT) -> ()>,
    pub sum_square: Option<unsafe extern "C" fn(*mut [INTFLOAT; 2], libc::c_int) -> AAC_FLOAT>,
    pub neg_odd_64: Option<unsafe extern "C" fn(*mut INTFLOAT) -> ()>,
    pub qmf_pre_shuffle: Option<unsafe extern "C" fn(*mut INTFLOAT) -> ()>,
    pub qmf_post_shuffle: Option<unsafe extern "C" fn(*mut [INTFLOAT; 2], *const INTFLOAT) -> ()>,
    pub qmf_deint_neg: Option<unsafe extern "C" fn(*mut INTFLOAT, *const INTFLOAT) -> ()>,
    pub qmf_deint_bfly:
        Option<unsafe extern "C" fn(*mut INTFLOAT, *const INTFLOAT, *const INTFLOAT) -> ()>,
    pub autocorrelate:
        Option<unsafe extern "C" fn(*const [INTFLOAT; 2], *mut [[AAC_FLOAT; 2]; 2]) -> ()>,
    pub hf_gen: Option<
        unsafe extern "C" fn(
            *mut [INTFLOAT; 2],
            *const [INTFLOAT; 2],
            *const INTFLOAT,
            *const INTFLOAT,
            INTFLOAT,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub hf_g_filt: Option<
        unsafe extern "C" fn(
            *mut [INTFLOAT; 2],
            *const [[INTFLOAT; 2]; 40],
            *const AAC_FLOAT,
            libc::c_int,
            intptr_t,
        ) -> (),
    >,
    pub hf_apply_noise: [Option<
        unsafe extern "C" fn(
            *mut [INTFLOAT; 2],
            *const AAC_FLOAT,
            *const AAC_FLOAT,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AACContext {
    pub class: *mut AVClass,
    pub avctx: *mut AVCodecContext,
    pub frame: *mut AVFrame,
    pub is_saved: libc::c_int,
    pub che_drc: DynamicRangeControl,
    pub che: [[*mut ChannelElement; 16]; 4],
    pub tag_che_map: [[*mut ChannelElement; 16]; 4],
    pub tags_mapped: libc::c_int,
    pub warned_remapping_once: libc::c_int,
    pub buf_mdct: [INTFLOAT; 1024],
    pub mdct120: *mut AVTXContext,
    pub mdct128: *mut AVTXContext,
    pub mdct480: *mut AVTXContext,
    pub mdct512: *mut AVTXContext,
    pub mdct960: *mut AVTXContext,
    pub mdct1024: *mut AVTXContext,
    pub mdct_ltp: *mut AVTXContext,
    pub mdct120_fn: av_tx_fn,
    pub mdct128_fn: av_tx_fn,
    pub mdct480_fn: av_tx_fn,
    pub mdct512_fn: av_tx_fn,
    pub mdct960_fn: av_tx_fn,
    pub mdct1024_fn: av_tx_fn,
    pub mdct_ltp_fn: av_tx_fn,
    pub fdsp: *mut AVFloatDSPContext,
    pub random_state: libc::c_int,
    pub output_element: [*mut SingleChannelElement; 64],
    pub force_dmono_mode: libc::c_int,
    pub dmono_mode: libc::c_int,
    pub output_channel_order: AACOutputChannelOrder,
    pub temp: [INTFLOAT; 128],
    pub oc: [OutputConfiguration; 2],
    pub warned_num_aac_frames: libc::c_int,
    pub warned_960_sbr: libc::c_int,
    pub warned_71_wide: libc::c_uint,
    pub warned_gain_control: libc::c_int,
    pub warned_he_aac_mono: libc::c_int,
    pub imdct_and_windowing:
        Option<unsafe extern "C" fn(*mut AACContext, *mut SingleChannelElement) -> ()>,
    pub apply_ltp: Option<unsafe extern "C" fn(*mut AACContext, *mut SingleChannelElement) -> ()>,
    pub apply_tns: Option<
        unsafe extern "C" fn(
            *mut INTFLOAT,
            *mut TemporalNoiseShaping,
            *mut IndividualChannelStream,
            libc::c_int,
        ) -> (),
    >,
    pub windowing_and_mdct_ltp: Option<
        unsafe extern "C" fn(
            *mut AACContext,
            *mut INTFLOAT,
            *mut INTFLOAT,
            *mut IndividualChannelStream,
        ) -> (),
    >,
    pub update_ltp: Option<unsafe extern "C" fn(*mut AACContext, *mut SingleChannelElement) -> ()>,
    pub vector_pow43: Option<unsafe extern "C" fn(*mut libc::c_int, libc::c_int) -> ()>,
    pub subband_scale: Option<
        unsafe extern "C" fn(
            *mut libc::c_int,
            *mut libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *mut libc::c_void,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SingleChannelElement {
    pub ics: IndividualChannelStream,
    pub tns: TemporalNoiseShaping,
    pub pulse: Pulse,
    pub band_type: [BandType; 128],
    pub band_alt: [BandType; 128],
    pub band_type_run_end: [libc::c_int; 120],
    pub sf: [INTFLOAT; 120],
    pub sf_idx: [libc::c_int; 128],
    pub zeroes: [uint8_t; 128],
    pub can_pns: [uint8_t; 128],
    pub is_ener: [libc::c_float; 128],
    pub pns_ener: [libc::c_float; 128],
    pub pcoeffs: [INTFLOAT; 1024],
    pub coeffs: [INTFLOAT; 1024],
    pub saved: [INTFLOAT; 1536],
    pub ret_buf: [INTFLOAT; 2048],
    pub ltp_state: [INTFLOAT; 3072],
    pub lcoeffs: [AAC_FLOAT; 1024],
    pub prcoeffs: [AAC_FLOAT; 1024],
    pub predictor_state: [PredictorState; 672],
    pub ret: *mut INTFLOAT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PredictorState {
    pub cor0: AAC_FLOAT,
    pub cor1: AAC_FLOAT,
    pub var0: AAC_FLOAT,
    pub var1: AAC_FLOAT,
    pub r0: AAC_FLOAT,
    pub r1: AAC_FLOAT,
    pub k1: AAC_FLOAT,
    pub x_est: AAC_FLOAT,
}
pub type BandType = libc::c_uint;
pub const INTENSITY_BT: BandType = 15;
pub const INTENSITY_BT2: BandType = 14;
pub const NOISE_BT: BandType = 13;
pub const RESERVED_BT: BandType = 12;
pub const ESC_BT: BandType = 11;
pub const FIRST_PAIR_BT: BandType = 5;
pub const ZERO_BT: BandType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Pulse {
    pub num_pulse: libc::c_int,
    pub start: libc::c_int,
    pub pos: [libc::c_int; 4],
    pub amp: [libc::c_int; 4],
}
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct TemporalNoiseShaping {
    pub present: libc::c_int,
    pub n_filt: [libc::c_int; 8],
    pub length: [[libc::c_int; 4]; 8],
    pub direction: [[libc::c_int; 4]; 8],
    pub order: [[libc::c_int; 4]; 8],
    pub coef_idx: [[[libc::c_int; 20]; 4]; 8],
    pub coef: [[[INTFLOAT; 20]; 4]; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IndividualChannelStream {
    pub max_sfb: uint8_t,
    pub window_sequence: [WindowSequence; 2],
    pub use_kb_window: [uint8_t; 2],
    pub num_window_groups: libc::c_int,
    pub group_len: [uint8_t; 8],
    pub ltp: LongTermPrediction,
    pub swb_offset: *const uint16_t,
    pub swb_sizes: *const uint8_t,
    pub num_swb: libc::c_int,
    pub num_windows: libc::c_int,
    pub tns_max_bands: libc::c_int,
    pub predictor_present: libc::c_int,
    pub predictor_initialized: libc::c_int,
    pub predictor_reset_group: libc::c_int,
    pub predictor_reset_count: [libc::c_int; 31],
    pub prediction_used: [uint8_t; 41],
    pub window_clipping: [uint8_t; 8],
    pub clip_avoidance_factor: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LongTermPrediction {
    pub present: int8_t,
    pub lag: int16_t,
    pub coef_idx: libc::c_int,
    pub coef: INTFLOAT,
    pub used: [int8_t; 40],
}

impl Default for LongTermPrediction {
    fn default() -> Self {
        Self {
            present: Default::default(),
            lag: Default::default(),
            coef_idx: Default::default(),
            coef: Default::default(),
            used: [0; 40],
        }
    }
}
pub type WindowSequence = libc::c_uint;
pub const LONG_STOP_SEQUENCE: WindowSequence = 3;
pub const EIGHT_SHORT_SEQUENCE: WindowSequence = 2;
pub const LONG_START_SEQUENCE: WindowSequence = 1;
pub const ONLY_LONG_SEQUENCE: WindowSequence = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OutputConfiguration {
    pub m4ac: MPEG4AudioConfig,
    pub layout_map: [[uint8_t; 3]; 64],
    pub layout_map_tags: libc::c_int,
    pub ch_layout: AVChannelLayout,
    pub status: OCStatus,
}
pub type OCStatus = libc::c_uint;
pub const OC_LOCKED: OCStatus = 4;
pub const OC_GLOBAL_HDR: OCStatus = 3;
pub const OC_TRIAL_FRAME: OCStatus = 2;
pub const OC_TRIAL_PCE: OCStatus = 1;
pub const OC_NONE: OCStatus = 0;
pub type AACOutputChannelOrder = libc::c_uint;
pub const CHANNEL_ORDER_CODED: AACOutputChannelOrder = 1;
pub const CHANNEL_ORDER_DEFAULT: AACOutputChannelOrder = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ChannelElement {
    pub present: libc::c_int,
    pub common_window: libc::c_int,
    pub ms_mode: libc::c_int,
    pub is_mode: uint8_t,
    pub ms_mask: [uint8_t; 128],
    pub is_mask: [uint8_t; 128],
    pub ch: [SingleChannelElement; 2],
    pub coup: ChannelCoupling,
    pub sbr: SpectralBandReplication,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SpectralBandReplication {
    pub sample_rate: libc::c_int,
    pub start: libc::c_int,
    pub ready_for_dequant: libc::c_int,
    pub id_aac: libc::c_int,
    pub reset: libc::c_int,
    pub spectrum_params: SpectrumParameters,
    pub bs_amp_res_header: libc::c_int,
    pub bs_limiter_bands: libc::c_uint,
    pub bs_limiter_gains: libc::c_uint,
    pub bs_interpol_freq: libc::c_uint,
    pub bs_smoothing_mode: libc::c_uint,
    pub bs_coupling: libc::c_uint,
    pub k: [AAC_SIGNE; 5],
    pub kx: [AAC_SIGNE; 2],
    pub m: [AAC_SIGNE; 2],
    pub kx_and_m_pushed: libc::c_uint,
    pub n_master: AAC_SIGNE,
    pub data: [SBRData; 2],
    pub ps: PSContext,
    pub n: [AAC_SIGNE; 2],
    pub n_q: AAC_SIGNE,
    pub n_lim: AAC_SIGNE,
    pub f_master: [uint16_t; 49],
    pub f_tablelow: [uint16_t; 25],
    pub f_tablehigh: [uint16_t; 49],
    pub f_tablenoise: [uint16_t; 6],
    pub f_tablelim: [uint16_t; 30],
    pub num_patches: AAC_SIGNE,
    pub patch_num_subbands: [uint8_t; 6],
    pub patch_start_subband: [uint8_t; 6],
    pub X_low: [[[INTFLOAT; 2]; 40]; 32],
    pub X_high: [[[INTFLOAT; 2]; 40]; 64],
    pub X: [[[[INTFLOAT; 64]; 38]; 2]; 2],
    pub alpha0: [[INTFLOAT; 2]; 64],
    pub alpha1: [[INTFLOAT; 2]; 64],
    pub e_origmapped: [[AAC_FLOAT; 48]; 7],
    pub q_mapped: [[AAC_FLOAT; 48]; 7],
    pub s_mapped: [[uint8_t; 48]; 7],
    pub e_curr: [[AAC_FLOAT; 48]; 7],
    pub q_m: [[AAC_FLOAT; 48]; 7],
    pub s_m: [[AAC_FLOAT; 48]; 7],
    pub gain: [[AAC_FLOAT; 48]; 7],
    pub qmf_filter_scratch: [[INTFLOAT; 64]; 5],
    pub mdct_ana: *mut AVTXContext,
    pub mdct_ana_fn: av_tx_fn,
    pub mdct: *mut AVTXContext,
    pub mdct_fn: av_tx_fn,
    pub dsp: SBRDSPContext,
    pub c: AACSBRContext,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AACSBRContext {
    pub sbr_lf_gen: Option<
        unsafe extern "C" fn(
            *mut AACContext,
            *mut SpectralBandReplication,
            *mut [[INTFLOAT; 2]; 40],
            *const [[[INTFLOAT; 2]; 32]; 32],
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub sbr_hf_assemble: Option<
        unsafe extern "C" fn(
            *mut [[INTFLOAT; 2]; 64],
            *const [[INTFLOAT; 2]; 40],
            *mut SpectralBandReplication,
            *mut SBRData,
            *const libc::c_int,
        ) -> (),
    >,
    pub sbr_x_gen: Option<
        unsafe extern "C" fn(
            *mut SpectralBandReplication,
            *mut [[INTFLOAT; 64]; 38],
            *const [[INTFLOAT; 2]; 64],
            *const [[INTFLOAT; 2]; 64],
            *const [[INTFLOAT; 2]; 40],
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub sbr_hf_inverse_filter: Option<
        unsafe extern "C" fn(
            *mut SBRDSPContext,
            *mut [INTFLOAT; 2],
            *mut [INTFLOAT; 2],
            *const [[INTFLOAT; 2]; 40],
            libc::c_int,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SBRData {
    pub bs_frame_class: libc::c_uint,
    pub bs_add_harmonic_flag: libc::c_uint,
    pub bs_num_env: AAC_SIGNE,
    pub bs_freq_res: [uint8_t; 7],
    pub bs_num_noise: AAC_SIGNE,
    pub bs_df_env: [uint8_t; 5],
    pub bs_df_noise: [uint8_t; 2],
    pub bs_invf_mode: [[uint8_t; 5]; 2],
    pub bs_add_harmonic: [uint8_t; 48],
    pub bs_amp_res: libc::c_uint,
    pub synthesis_filterbank_samples: [INTFLOAT; 2304],
    pub analysis_filterbank_samples: [INTFLOAT; 1312],
    pub synthesis_filterbank_samples_offset: libc::c_int,
    pub e_a: [libc::c_int; 2],
    pub bw_array: [INTFLOAT; 5],
    pub W: [[[[INTFLOAT; 2]; 32]; 32]; 2],
    pub Ypos: libc::c_int,
    pub Y: [[[[INTFLOAT; 2]; 64]; 38]; 2],
    pub g_temp: [[AAC_FLOAT; 48]; 42],
    pub q_temp: [[AAC_FLOAT; 48]; 42],
    pub s_indexmapped: [[uint8_t; 48]; 8],
    pub env_facs_q: [[uint8_t; 48]; 6],
    pub env_facs: [[AAC_FLOAT; 48]; 6],
    pub noise_facs_q: [[uint8_t; 5]; 3],
    pub noise_facs: [[AAC_FLOAT; 5]; 3],
    pub t_env: [uint8_t; 8],
    pub t_env_num_env_old: uint8_t,
    pub t_q: [uint8_t; 3],
    pub f_indexnoise: libc::c_uint,
    pub f_indexsine: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SpectrumParameters {
    pub bs_start_freq: uint8_t,
    pub bs_stop_freq: uint8_t,
    pub bs_xover_band: uint8_t,
    pub bs_freq_scale: uint8_t,
    pub bs_alter_scale: uint8_t,
    pub bs_noise_bands: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ChannelCoupling {
    pub coupling_point: CouplingPoint,
    pub num_coupled: libc::c_int,
    pub type_0: [RawDataBlockType; 8],
    pub id_select: [libc::c_int; 8],
    pub ch_select: [libc::c_int; 8],
    pub gain: [[INTFLOAT; 120]; 16],
}
pub type RawDataBlockType = libc::c_uint;
pub const TYPE_END: RawDataBlockType = 7;
pub const TYPE_FIL: RawDataBlockType = 6;
pub const TYPE_PCE: RawDataBlockType = 5;
pub const TYPE_DSE: RawDataBlockType = 4;
pub const TYPE_LFE: RawDataBlockType = 3;
pub const TYPE_CCE: RawDataBlockType = 2;
pub const TYPE_CPE: RawDataBlockType = 1;
pub const TYPE_SCE: RawDataBlockType = 0;
pub type CouplingPoint = libc::c_uint;
pub const AFTER_IMDCT: CouplingPoint = 3;
pub const BETWEEN_TNS_AND_IMDCT: CouplingPoint = 1;
pub const BEFORE_TNS: CouplingPoint = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DynamicRangeControl {
    pub pce_instance_tag: libc::c_int,
    pub dyn_rng_sgn: [libc::c_int; 17],
    pub dyn_rng_ctl: [libc::c_int; 17],
    pub exclude_mask: [libc::c_int; 64],
    pub band_incr: libc::c_int,
    pub interpolation_scheme: libc::c_int,
    pub band_top: [libc::c_int; 17],
    pub prog_ref_level: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioFrame {
    pub pts: int64_t,
    pub duration: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioFrameQueue {
    pub avctx: *mut AVCodecContext,
    pub remaining_delay: libc::c_int,
    pub remaining_samples: libc::c_int,
    pub frames: *mut AudioFrame,
    pub frame_count: libc::c_uint,
    pub frame_alloc: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFPsyBand {
    pub bits: libc::c_int,
    pub energy: libc::c_float,
    pub threshold: libc::c_float,
    pub spread: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFPsyChannel {
    pub psy_bands: [FFPsyBand; 128],
    pub entropy: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFPsyChannelGroup {
    pub ch: [*mut FFPsyChannel; 20],
    pub num_ch: uint8_t,
    pub coupling: [uint8_t; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFPsyWindowInfo {
    pub window_type: [libc::c_int; 3],
    pub window_shape: libc::c_int,
    pub num_windows: libc::c_int,
    pub grouping: [libc::c_int; 8],
    pub clipping: [libc::c_float; 8],
    pub window_sizes: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFPsyContext {
    pub avctx: *mut AVCodecContext,
    pub model: *const FFPsyModel,
    pub ch: *mut FFPsyChannel,
    pub group: *mut FFPsyChannelGroup,
    pub num_groups: libc::c_int,
    pub cutoff: libc::c_int,
    pub bands: *mut *mut uint8_t,
    pub num_bands: *mut libc::c_int,
    pub num_lens: libc::c_int,
    pub bitres: C2RustUnnamed_2,
    pub model_priv_data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub size: libc::c_int,
    pub bits: libc::c_int,
    pub alloc: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFPsyModel {
    pub name: *const libc::c_char,
    pub init: Option<unsafe extern "C" fn(*mut FFPsyContext) -> libc::c_int>,
    pub window: Option<
        unsafe extern "C" fn(
            *mut FFPsyContext,
            *const libc::c_float,
            *const libc::c_float,
            libc::c_int,
            libc::c_int,
        ) -> FFPsyWindowInfo,
    >,
    pub analyze: Option<
        unsafe extern "C" fn(
            *mut FFPsyContext,
            libc::c_int,
            *mut *const libc::c_float,
            *const FFPsyWindowInfo,
        ) -> (),
    >,
    pub end: Option<unsafe extern "C" fn(*mut FFPsyContext) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LLSModel {
    pub covariance: [[libc::c_double; 36]; 36],
    pub coeff: [[libc::c_double; 32]; 32],
    pub variance: [libc::c_double; 32],
    pub indep_count: libc::c_int,
    pub update_lls: Option<unsafe extern "C" fn(*mut LLSModel, *const libc::c_double) -> ()>,
    pub evaluate_lls: Option<
        unsafe extern "C" fn(*mut LLSModel, *const libc::c_double, libc::c_int) -> libc::c_double,
    >,
}
pub type FFLPCType = libc::c_int;
pub const FF_LPC_TYPE_NB: FFLPCType = 4;
pub const FF_LPC_TYPE_CHOLESKY: FFLPCType = 3;
pub const FF_LPC_TYPE_LEVINSON: FFLPCType = 2;
pub const FF_LPC_TYPE_FIXED: FFLPCType = 1;
pub const FF_LPC_TYPE_NONE: FFLPCType = 0;
pub const FF_LPC_TYPE_DEFAULT: FFLPCType = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LPCContext {
    pub blocksize: libc::c_int,
    pub max_order: libc::c_int,
    pub lpc_type: FFLPCType,
    pub windowed_buffer: *mut libc::c_double,
    pub windowed_samples: *mut libc::c_double,
    pub lpc_apply_welch_window:
        Option<unsafe extern "C" fn(*const int32_t, ptrdiff_t, *mut libc::c_double) -> ()>,
    pub lpc_compute_autocorr: Option<
        unsafe extern "C" fn(
            *const libc::c_double,
            ptrdiff_t,
            libc::c_int,
            *mut libc::c_double,
        ) -> (),
    >,
    pub lls_models: [LLSModel; 2],
}
pub type AACCoder = libc::c_uint;
pub const AAC_CODER_NB: AACCoder = 3;
pub const AAC_CODER_FAST: AACCoder = 2;
pub const AAC_CODER_TWOLOOP: AACCoder = 1;
pub const AAC_CODER_ANMR: AACCoder = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AACEncOptions {
    pub coder: libc::c_int,
    pub pns: libc::c_int,
    pub tns: libc::c_int,
    pub ltp: libc::c_int,
    pub pce: libc::c_int,
    pub pred: libc::c_int,
    pub mid_side: libc::c_int,
    pub intensity_stereo: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AACEncContext {
    pub av_class: *mut AVClass,
    pub options: AACEncOptions,
    pub pb: PutBitContext,
    pub mdct1024: *mut AVTXContext,
    pub mdct1024_fn: av_tx_fn,
    pub mdct128: *mut AVTXContext,
    pub mdct128_fn: av_tx_fn,
    pub fdsp: *mut AVFloatDSPContext,
    pub pce: AACPCEInfo,
    pub planar_samples: [*mut libc::c_float; 16],
    pub profile: libc::c_int,
    pub needs_pce: libc::c_int,
    pub lpc: LPCContext,
    pub samplerate_index: libc::c_int,
    pub channels: libc::c_int,
    pub reorder_map: *const uint8_t,
    pub chan_map: *const uint8_t,
    pub cpe: *mut ChannelElement,
    pub psy: FFPsyContext,
    pub psypp: *mut FFPsyPreprocessContext,
    pub coder: *const AACCoefficientsEncoder,
    pub cur_channel: libc::c_int,
    pub random_state: libc::c_int,
    pub lambda: libc::c_float,
    pub last_frame_pb_count: libc::c_int,
    pub lambda_sum: libc::c_float,
    pub lambda_count: libc::c_int,
    pub cur_type: RawDataBlockType,
    pub afq: AudioFrameQueue,
    pub qcoefs: [libc::c_int; 96],
    pub scoefs: [libc::c_float; 1024],
    pub quantize_band_cost_cache_generation: uint16_t,
    pub quantize_band_cost_cache: [[AACQuantizeBandCostCacheEntry; 128]; 256],
    pub abs_pow34:
        Option<unsafe extern "C" fn(*mut libc::c_float, *const libc::c_float, libc::c_int) -> ()>,
    pub quant_bands: Option<
        unsafe extern "C" fn(
            *mut libc::c_int,
            *const libc::c_float,
            *const libc::c_float,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_float,
            libc::c_float,
        ) -> (),
    >,
    pub buffer: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub samples: *mut libc::c_float,
}
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct AACQuantizeBandCostCacheEntry {
    pub rd: libc::c_float,
    pub energy: libc::c_float,
    pub bits: libc::c_int,
    pub cb: libc::c_char,
    pub rtz: libc::c_char,
    pub generation: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AACCoefficientsEncoder {
    pub search_for_quantizers: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut AACEncContext,
            *mut SingleChannelElement,
            libc::c_float,
        ) -> (),
    >,
    pub encode_window_bands_info: Option<
        unsafe extern "C" fn(
            *mut AACEncContext,
            *mut SingleChannelElement,
            libc::c_int,
            libc::c_int,
            libc::c_float,
        ) -> (),
    >,
    pub quantize_and_encode_band: Option<
        unsafe extern "C" fn(
            *mut AACEncContext,
            *mut PutBitContext,
            *const libc::c_float,
            *mut libc::c_float,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_float,
            libc::c_int,
        ) -> (),
    >,
    pub encode_tns_info:
        Option<unsafe extern "C" fn(*mut AACEncContext, *mut SingleChannelElement) -> ()>,
    pub encode_ltp_info: Option<
        unsafe extern "C" fn(*mut AACEncContext, *mut SingleChannelElement, libc::c_int) -> (),
    >,
    pub encode_main_pred:
        Option<unsafe extern "C" fn(*mut AACEncContext, *mut SingleChannelElement) -> ()>,
    pub adjust_common_pred:
        Option<unsafe extern "C" fn(*mut AACEncContext, *mut ChannelElement) -> ()>,
    pub adjust_common_ltp:
        Option<unsafe extern "C" fn(*mut AACEncContext, *mut ChannelElement) -> ()>,
    pub apply_main_pred:
        Option<unsafe extern "C" fn(*mut AACEncContext, *mut SingleChannelElement) -> ()>,
    pub apply_tns_filt:
        Option<unsafe extern "C" fn(*mut AACEncContext, *mut SingleChannelElement) -> ()>,
    pub update_ltp:
        Option<unsafe extern "C" fn(*mut AACEncContext, *mut SingleChannelElement) -> ()>,
    pub ltp_insert_new_frame: Option<unsafe extern "C" fn(*mut AACEncContext) -> ()>,
    pub set_special_band_scalefactors:
        Option<unsafe extern "C" fn(*mut AACEncContext, *mut SingleChannelElement) -> ()>,
    pub search_for_pns: Option<
        unsafe extern "C" fn(
            *mut AACEncContext,
            *mut AVCodecContext,
            *mut SingleChannelElement,
        ) -> (),
    >,
    pub mark_pns: Option<
        unsafe extern "C" fn(
            *mut AACEncContext,
            *mut AVCodecContext,
            *mut SingleChannelElement,
        ) -> (),
    >,
    pub search_for_tns:
        Option<unsafe extern "C" fn(*mut AACEncContext, *mut SingleChannelElement) -> ()>,
    pub search_for_ltp: Option<
        unsafe extern "C" fn(*mut AACEncContext, *mut SingleChannelElement, libc::c_int) -> (),
    >,
    pub search_for_ms: Option<unsafe extern "C" fn(*mut AACEncContext, *mut ChannelElement) -> ()>,
    pub search_for_is: Option<
        unsafe extern "C" fn(*mut AACEncContext, *mut AVCodecContext, *mut ChannelElement) -> (),
    >,
    pub search_for_pred:
        Option<unsafe extern "C" fn(*mut AACEncContext, *mut SingleChannelElement) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AACPCEInfo {
    pub layout: AVChannelLayout,
    pub num_ele: [libc::c_int; 4],
    pub pairing: [[libc::c_int; 8]; 3],
    pub index: [[libc::c_int; 8]; 4],
    pub config_map: [uint8_t; 16],
    pub reorder_map: [uint8_t; 16],
}

static mut run_value_bits_long: [uint8_t; 64] = [
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
];
static mut run_value_bits_short: [uint8_t; 16] = [
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
];
pub static mut run_value_bits: [*const uint8_t; 2] =
    unsafe { [run_value_bits_long.as_ptr(), run_value_bits_short.as_ptr()] };
pub static mut aac_cb_out_map: [uint8_t; 15] = [
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
];
static mut aac_cb_in_map: [uint8_t; 16] = [
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
];
pub static mut aac_cb_range: [uint8_t; 12] = [
    0 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    17 as libc::c_int as uint8_t,
];
pub static mut aac_cb_maxval: [uint8_t; 12] = [
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    16 as libc::c_int as uint8_t,
];
pub static mut aac_maxval_cb: [libc::c_uchar; 14] = [
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
];

pub type LPC_TYPE = libc::c_float;
pub type LPC_TYPE_U = libc::c_float;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AACISError {
    pub pass: libc::c_int,
    pub phase: libc::c_int,
    pub error: libc::c_float,
    pub dist1: libc::c_float,
    pub dist2: libc::c_float,
    pub ener01: libc::c_float,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFPsyPreprocessContext {
    pub avctx: *mut AVCodecContext,
    pub stereo_att: libc::c_float,
    pub fcoeffs: *mut FFIIRFilterCoeffs,
    pub fstate: *mut *mut FFIIRFilterState,
    pub fiir: FFIIRFilterContext,
}
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct FFIIRFilterContext {
    pub filter_flt: Option<
        unsafe extern "C" fn(
            *const FFIIRFilterCoeffs,
            *mut FFIIRFilterState,
            libc::c_int,
            *const libc::c_float,
            ptrdiff_t,
            *mut libc::c_float,
            ptrdiff_t,
        ) -> (),
    >,
}
pub type IIRFilterMode = libc::c_uint;
pub const FF_FILTER_MODE_BANDSTOP: IIRFilterMode = 3;
pub const FF_FILTER_MODE_BANDPASS: IIRFilterMode = 2;
pub const FF_FILTER_MODE_HIGHPASS: IIRFilterMode = 1;
pub const FF_FILTER_MODE_LOWPASS: IIRFilterMode = 0;
pub type IIRFilterType = libc::c_uint;
pub const FF_FILTER_TYPE_ELLIPTIC: IIRFilterType = 4;
pub const FF_FILTER_TYPE_CHEBYSHEV: IIRFilterType = 3;
pub const FF_FILTER_TYPE_BUTTERWORTH: IIRFilterType = 2;
pub const FF_FILTER_TYPE_BIQUAD: IIRFilterType = 1;
pub const FF_FILTER_TYPE_BESSEL: IIRFilterType = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFIIRFilterCoeffs {
    pub order: libc::c_int,
    pub gain: libc::c_float,
    pub cx: *mut libc::c_int,
    pub cy: *mut libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFIIRFilterState {
    pub x: [libc::c_float; 1],
}
