#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type AVOptionRanges;
    pub type AVOption;
    pub type AVBuffer;
    pub type AVDictionary;
    pub type AVCodecDescriptor;
    pub type AVCodecInternal;
    fn atanf(_: libc::c_float) -> libc::c_float;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn exp2f(_: libc::c_float) -> libc::c_float;
    fn exp2(_: libc::c_double) -> libc::c_double;
    fn log2f(_: libc::c_float) -> libc::c_float;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn fabsf(_: libc::c_float) -> libc::c_float;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn av_mallocz(size: size_t) -> *mut libc::c_void;
    fn av_calloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn av_freep(ptr: *mut libc::c_void);
    fn ff_psy_find_group(
        ctx: *mut FFPsyContext,
        channel: libc::c_int,
    ) -> *mut FFPsyChannelGroup;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVRational {
    pub num: libc::c_int,
    pub den: libc::c_int,
}
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
pub type size_t = libc::c_ulong;
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
pub struct AVClass {
    pub class_name: *const libc::c_char,
    pub item_name: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> *const libc::c_char,
    >,
    pub option: *const AVOption,
    pub version: libc::c_int,
    pub log_level_offset_offset: libc::c_int,
    pub parent_log_context_offset: libc::c_int,
    pub category: AVClassCategory,
    pub get_category: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> AVClassCategory,
    >,
    pub query_ranges: Option::<
        unsafe extern "C" fn(
            *mut *mut AVOptionRanges,
            *mut libc::c_void,
            *const libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub child_next: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> *mut libc::c_void,
    >,
    pub child_class_iterate: Option::<
        unsafe extern "C" fn(*mut *mut libc::c_void) -> *const AVClass,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVBufferRef {
    pub buffer: *mut AVBuffer,
    pub data: *mut uint8_t,
    pub size: size_t,
}
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
    pub draw_horiz_band: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *const AVFrame,
            *mut libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub get_format: Option::<
        unsafe extern "C" fn(*mut AVCodecContext, *const AVPixelFormat) -> AVPixelFormat,
    >,
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
    pub get_buffer2: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut AVFrame,
            libc::c_int,
        ) -> libc::c_int,
    >,
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
    pub execute: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            Option::<
                unsafe extern "C" fn(
                    *mut AVCodecContext,
                    *mut libc::c_void,
                ) -> libc::c_int,
            >,
            *mut libc::c_void,
            *mut libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub execute2: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            Option::<
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
    pub get_encode_buffer: Option::<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut AVPacket,
            libc::c_int,
        ) -> libc::c_int,
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
pub type WindowSequence = libc::c_uint;
pub const LONG_STOP_SEQUENCE: WindowSequence = 3;
pub const EIGHT_SHORT_SEQUENCE: WindowSequence = 2;
pub const LONG_START_SEQUENCE: WindowSequence = 1;
pub const ONLY_LONG_SEQUENCE: WindowSequence = 0;
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
    pub bitres: C2RustUnnamed_0,
    pub model_priv_data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub size: libc::c_int,
    pub bits: libc::c_int,
    pub alloc: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFPsyModel {
    pub name: *const libc::c_char,
    pub init: Option::<unsafe extern "C" fn(*mut FFPsyContext) -> libc::c_int>,
    pub window: Option::<
        unsafe extern "C" fn(
            *mut FFPsyContext,
            *const libc::c_float,
            *const libc::c_float,
            libc::c_int,
            libc::c_int,
        ) -> FFPsyWindowInfo,
    >,
    pub analyze: Option::<
        unsafe extern "C" fn(
            *mut FFPsyContext,
            libc::c_int,
            *mut *const libc::c_float,
            *const FFPsyWindowInfo,
        ) -> (),
    >,
    pub end: Option::<unsafe extern "C" fn(*mut FFPsyContext) -> ()>,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const PSY_3GPP_AH_ACTIVE: C2RustUnnamed_1 = 2;
pub const PSY_3GPP_AH_INACTIVE: C2RustUnnamed_1 = 1;
pub const PSY_3GPP_AH_NONE: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AacPsyBand {
    pub energy: libc::c_float,
    pub thr: libc::c_float,
    pub thr_quiet: libc::c_float,
    pub nz_lines: libc::c_float,
    pub active_lines: libc::c_float,
    pub pe: libc::c_float,
    pub pe_const: libc::c_float,
    pub norm_fac: libc::c_float,
    pub avoid_holes: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AacPsyChannel {
    pub band: [AacPsyBand; 128],
    pub prev_band: [AacPsyBand; 128],
    pub win_energy: libc::c_float,
    pub iir_state: [libc::c_float; 2],
    pub next_grouping: uint8_t,
    pub next_window_seq: WindowSequence,
    pub attack_threshold: libc::c_float,
    pub prev_energy_subshort: [libc::c_float; 24],
    pub prev_attack: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AacPsyCoeffs {
    pub ath: libc::c_float,
    pub barks: libc::c_float,
    pub spread_low: [libc::c_float; 2],
    pub spread_hi: [libc::c_float; 2],
    pub min_snr: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AacPsyContext {
    pub chan_bitrate: libc::c_int,
    pub frame_bits: libc::c_int,
    pub fill_level: libc::c_int,
    pub pe: C2RustUnnamed_2,
    pub psy_coef: [[AacPsyCoeffs; 64]; 2],
    pub ch: *mut AacPsyChannel,
    pub global_quality: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub min: libc::c_float,
    pub max: libc::c_float,
    pub previous: libc::c_float,
    pub correction: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PsyLamePreset {
    pub quality: libc::c_int,
    pub st_lrm: libc::c_float,
}
#[inline(always)]
unsafe extern "C" fn ff_exp10(mut x: libc::c_double) -> libc::c_double {
    return exp2(3.32192809488736234787f64 * x);
}
#[inline(always)]
unsafe extern "C" fn av_clip_c(
    mut a: libc::c_int,
    mut amin: libc::c_int,
    mut amax: libc::c_int,
) -> libc::c_int {
    if a < amin { return amin } else if a > amax { return amax } else { return a };
}
#[inline(always)]
unsafe extern "C" fn av_clipf_c(
    mut a: libc::c_float,
    mut amin: libc::c_float,
    mut amax: libc::c_float,
) -> libc::c_float {
    return if (if a > amin { a } else { amin }) > amax {
        amax
    } else if a > amin {
        a
    } else {
        amin
    };
}
static mut psy_abr_map: [PsyLamePreset; 13] = [
    {
        let mut init = PsyLamePreset {
            quality: 8 as libc::c_int,
            st_lrm: 6.60f64 as libc::c_float,
        };
        init
    },
    {
        let mut init = PsyLamePreset {
            quality: 16 as libc::c_int,
            st_lrm: 6.60f64 as libc::c_float,
        };
        init
    },
    {
        let mut init = PsyLamePreset {
            quality: 24 as libc::c_int,
            st_lrm: 6.60f64 as libc::c_float,
        };
        init
    },
    {
        let mut init = PsyLamePreset {
            quality: 32 as libc::c_int,
            st_lrm: 6.60f64 as libc::c_float,
        };
        init
    },
    {
        let mut init = PsyLamePreset {
            quality: 40 as libc::c_int,
            st_lrm: 6.60f64 as libc::c_float,
        };
        init
    },
    {
        let mut init = PsyLamePreset {
            quality: 48 as libc::c_int,
            st_lrm: 6.60f64 as libc::c_float,
        };
        init
    },
    {
        let mut init = PsyLamePreset {
            quality: 56 as libc::c_int,
            st_lrm: 6.60f64 as libc::c_float,
        };
        init
    },
    {
        let mut init = PsyLamePreset {
            quality: 64 as libc::c_int,
            st_lrm: 6.40f64 as libc::c_float,
        };
        init
    },
    {
        let mut init = PsyLamePreset {
            quality: 80 as libc::c_int,
            st_lrm: 6.00f64 as libc::c_float,
        };
        init
    },
    {
        let mut init = PsyLamePreset {
            quality: 96 as libc::c_int,
            st_lrm: 5.60f64 as libc::c_float,
        };
        init
    },
    {
        let mut init = PsyLamePreset {
            quality: 112 as libc::c_int,
            st_lrm: 5.20f64 as libc::c_float,
        };
        init
    },
    {
        let mut init = PsyLamePreset {
            quality: 128 as libc::c_int,
            st_lrm: 5.20f64 as libc::c_float,
        };
        init
    },
    {
        let mut init = PsyLamePreset {
            quality: 160 as libc::c_int,
            st_lrm: 5.20f64 as libc::c_float,
        };
        init
    },
];
static mut psy_vbr_map: [PsyLamePreset; 11] = [
    {
        let mut init = PsyLamePreset {
            quality: 0 as libc::c_int,
            st_lrm: 4.20f64 as libc::c_float,
        };
        init
    },
    {
        let mut init = PsyLamePreset {
            quality: 1 as libc::c_int,
            st_lrm: 4.20f64 as libc::c_float,
        };
        init
    },
    {
        let mut init = PsyLamePreset {
            quality: 2 as libc::c_int,
            st_lrm: 4.20f64 as libc::c_float,
        };
        init
    },
    {
        let mut init = PsyLamePreset {
            quality: 3 as libc::c_int,
            st_lrm: 4.20f64 as libc::c_float,
        };
        init
    },
    {
        let mut init = PsyLamePreset {
            quality: 4 as libc::c_int,
            st_lrm: 4.20f64 as libc::c_float,
        };
        init
    },
    {
        let mut init = PsyLamePreset {
            quality: 5 as libc::c_int,
            st_lrm: 4.20f64 as libc::c_float,
        };
        init
    },
    {
        let mut init = PsyLamePreset {
            quality: 6 as libc::c_int,
            st_lrm: 4.20f64 as libc::c_float,
        };
        init
    },
    {
        let mut init = PsyLamePreset {
            quality: 7 as libc::c_int,
            st_lrm: 4.20f64 as libc::c_float,
        };
        init
    },
    {
        let mut init = PsyLamePreset {
            quality: 8 as libc::c_int,
            st_lrm: 4.20f64 as libc::c_float,
        };
        init
    },
    {
        let mut init = PsyLamePreset {
            quality: 9 as libc::c_int,
            st_lrm: 4.20f64 as libc::c_float,
        };
        init
    },
    {
        let mut init = PsyLamePreset {
            quality: 10 as libc::c_int,
            st_lrm: 4.20f64 as libc::c_float,
        };
        init
    },
];
static mut psy_fir_coeffs: [libc::c_float; 10] = [
    (-8.65163e-18f64 * 2 as libc::c_int as libc::c_double) as libc::c_float,
    (-0.00851586f64 * 2 as libc::c_int as libc::c_double) as libc::c_float,
    (-6.74764e-18f64 * 2 as libc::c_int as libc::c_double) as libc::c_float,
    (0.0209036f64 * 2 as libc::c_int as libc::c_double) as libc::c_float,
    (-3.36639e-17f64 * 2 as libc::c_int as libc::c_double) as libc::c_float,
    (-0.0438162f64 * 2 as libc::c_int as libc::c_double) as libc::c_float,
    (-1.54175e-17f64 * 2 as libc::c_int as libc::c_double) as libc::c_float,
    (0.0931738f64 * 2 as libc::c_int as libc::c_double) as libc::c_float,
    (-5.52212e-17f64 * 2 as libc::c_int as libc::c_double) as libc::c_float,
    (-0.313819f64 * 2 as libc::c_int as libc::c_double) as libc::c_float,
];
unsafe extern "C" fn lame_calc_attack_threshold(
    mut bitrate: libc::c_int,
) -> libc::c_float {
    let mut lower_range: libc::c_int = 12 as libc::c_int;
    let mut upper_range: libc::c_int = 12 as libc::c_int;
    let mut lower_range_kbps: libc::c_int = psy_abr_map[12 as libc::c_int as usize]
        .quality;
    let mut upper_range_kbps: libc::c_int = psy_abr_map[12 as libc::c_int as usize]
        .quality;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < 13 as libc::c_int {
        if (if bitrate > psy_abr_map[i as usize].quality {
            bitrate
        } else {
            psy_abr_map[i as usize].quality
        }) != bitrate
        {
            upper_range = i;
            upper_range_kbps = psy_abr_map[i as usize].quality;
            lower_range = i - 1 as libc::c_int;
            lower_range_kbps = psy_abr_map[(i - 1 as libc::c_int) as usize].quality;
            break;
        } else {
            i += 1;
            i;
        }
    }
    if upper_range_kbps - bitrate > bitrate - lower_range_kbps {
        return psy_abr_map[lower_range as usize].st_lrm;
    }
    return psy_abr_map[upper_range as usize].st_lrm;
}
#[cold]
unsafe extern "C" fn lame_window_init(
    mut ctx: *mut AacPsyContext,
    mut avctx: *mut AVCodecContext,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*avctx).ch_layout.nb_channels {
        let mut pch: *mut AacPsyChannel = &mut *((*ctx).ch).offset(i as isize)
            as *mut AacPsyChannel;
        if (*avctx).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
            (*pch)
                .attack_threshold = psy_vbr_map[av_clip_c(
                    (*avctx).global_quality / 118 as libc::c_int,
                    0 as libc::c_int,
                    10 as libc::c_int,
                ) as usize]
                .st_lrm;
        } else {
            (*pch)
                .attack_threshold = lame_calc_attack_threshold(
                ((*avctx).bit_rate / (*avctx).ch_layout.nb_channels as libc::c_long
                    / 1000 as libc::c_int as libc::c_long) as libc::c_int,
            );
        }
        j = 0 as libc::c_int;
        while j < 8 as libc::c_int * 3 as libc::c_int {
            (*pch).prev_energy_subshort[j as usize] = 10.0f32;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[cold]
unsafe extern "C" fn calc_bark(mut f: libc::c_float) -> libc::c_float {
    return 13.3f32 * atanf(0.00076f32 * f)
        + 3.5f32 * atanf(f / 7500.0f32 * (f / 7500.0f32));
}
#[cold]
unsafe extern "C" fn ath(mut f: libc::c_float, mut add: libc::c_float) -> libc::c_float {
    f /= 1000.0f32;
    return (3.64f64 * pow(f as libc::c_double, -0.8f64)
        - 6.8f64
            * exp(
                -0.6f64 * (f as libc::c_double - 3.4f64) * (f as libc::c_double - 3.4f64),
            )
        + 6.0f64
            * exp(
                -0.15f64 * (f as libc::c_double - 8.7f64)
                    * (f as libc::c_double - 8.7f64),
            )
        + (0.6f64 + 0.04f64 * add as libc::c_double) * 0.001f64 * f as libc::c_double
            * f as libc::c_double * f as libc::c_double * f as libc::c_double)
        as libc::c_float;
}
#[cold]
unsafe extern "C" fn psy_3gpp_init(mut ctx: *mut FFPsyContext) -> libc::c_int {
    let mut pctx: *mut AacPsyContext = 0 as *mut AacPsyContext;
    let mut bark: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut prev: libc::c_float = 0.;
    let mut minscale: libc::c_float = 0.;
    let mut minath: libc::c_float = 0.;
    let mut minsnr: libc::c_float = 0.;
    let mut pe_min: libc::c_float = 0.;
    let mut chan_bitrate: libc::c_int = ((*(*ctx).avctx).bit_rate as libc::c_float
        / (if (*(*ctx).avctx).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
            2.0f32
        } else {
            (*(*ctx).avctx).ch_layout.nb_channels as libc::c_float
        })) as libc::c_int;
    let bandwidth: libc::c_int = (if (*ctx).cutoff != 0 {
        (*ctx).cutoff as libc::c_long
    } else if (*(*ctx).avctx).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        ((*(*ctx).avctx).sample_rate / 2 as libc::c_int) as libc::c_long
    } else if (*(*ctx).avctx).bit_rate != 0 {
        if (if (if (if (if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        })
            > 3000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 4 as libc::c_int as libc::c_long
        {
            3000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 4 as libc::c_int as libc::c_long
        } else {
            (if (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
                > (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    * 15 as libc::c_int as libc::c_long
                    / 32 as libc::c_int as libc::c_long
                    - 5500 as libc::c_int as libc::c_long
            {
                (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 5 as libc::c_int as libc::c_long
            } else {
                (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    * 15 as libc::c_int as libc::c_long
                    / 32 as libc::c_int as libc::c_long
                    - 5500 as libc::c_int as libc::c_long
            })
        })
            > 12000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 16 as libc::c_int as libc::c_long
        {
            12000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 16 as libc::c_int as libc::c_long
        } else {
            (if (if (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
                > (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    * 15 as libc::c_int as libc::c_long
                    / 32 as libc::c_int as libc::c_long
                    - 5500 as libc::c_int as libc::c_long
            {
                (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 5 as libc::c_int as libc::c_long
            } else {
                (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    * 15 as libc::c_int as libc::c_long
                    / 32 as libc::c_int as libc::c_long
                    - 5500 as libc::c_int as libc::c_long
            })
                > 3000 as libc::c_int as libc::c_long
                    + (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        / 4 as libc::c_int as libc::c_long
            {
                3000 as libc::c_int as libc::c_long
                    + (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        / 4 as libc::c_int as libc::c_long
            } else {
                (if (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 5 as libc::c_int as libc::c_long
                    > (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        * 15 as libc::c_int as libc::c_long
                        / 32 as libc::c_int as libc::c_long
                        - 5500 as libc::c_int as libc::c_long
                {
                    (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        / 5 as libc::c_int as libc::c_long
                } else {
                    (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        * 15 as libc::c_int as libc::c_long
                        / 32 as libc::c_int as libc::c_long
                        - 5500 as libc::c_int as libc::c_long
                })
            })
        }) > 22000 as libc::c_int as libc::c_long
        {
            22000 as libc::c_int as libc::c_long
        } else {
            (if (if (if (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
                > (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    * 15 as libc::c_int as libc::c_long
                    / 32 as libc::c_int as libc::c_long
                    - 5500 as libc::c_int as libc::c_long
            {
                (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 5 as libc::c_int as libc::c_long
            } else {
                (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    * 15 as libc::c_int as libc::c_long
                    / 32 as libc::c_int as libc::c_long
                    - 5500 as libc::c_int as libc::c_long
            })
                > 3000 as libc::c_int as libc::c_long
                    + (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        / 4 as libc::c_int as libc::c_long
            {
                3000 as libc::c_int as libc::c_long
                    + (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        / 4 as libc::c_int as libc::c_long
            } else {
                (if (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 5 as libc::c_int as libc::c_long
                    > (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        * 15 as libc::c_int as libc::c_long
                        / 32 as libc::c_int as libc::c_long
                        - 5500 as libc::c_int as libc::c_long
                {
                    (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        / 5 as libc::c_int as libc::c_long
                } else {
                    (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        * 15 as libc::c_int as libc::c_long
                        / 32 as libc::c_int as libc::c_long
                        - 5500 as libc::c_int as libc::c_long
                })
            })
                > 12000 as libc::c_int as libc::c_long
                    + (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        / 16 as libc::c_int as libc::c_long
            {
                12000 as libc::c_int as libc::c_long
                    + (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        / 16 as libc::c_int as libc::c_long
            } else {
                (if (if (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 5 as libc::c_int as libc::c_long
                    > (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        * 15 as libc::c_int as libc::c_long
                        / 32 as libc::c_int as libc::c_long
                        - 5500 as libc::c_int as libc::c_long
                {
                    (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        / 5 as libc::c_int as libc::c_long
                } else {
                    (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        * 15 as libc::c_int as libc::c_long
                        / 32 as libc::c_int as libc::c_long
                        - 5500 as libc::c_int as libc::c_long
                })
                    > 3000 as libc::c_int as libc::c_long
                        + (*(*ctx).avctx).bit_rate
                            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                            / 4 as libc::c_int as libc::c_long
                {
                    3000 as libc::c_int as libc::c_long
                        + (*(*ctx).avctx).bit_rate
                            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                            / 4 as libc::c_int as libc::c_long
                } else {
                    (if (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        / 5 as libc::c_int as libc::c_long
                        > (*(*ctx).avctx).bit_rate
                            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                            * 15 as libc::c_int as libc::c_long
                            / 32 as libc::c_int as libc::c_long
                            - 5500 as libc::c_int as libc::c_long
                    {
                        (*(*ctx).avctx).bit_rate
                            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                            / 5 as libc::c_int as libc::c_long
                    } else {
                        (*(*ctx).avctx).bit_rate
                            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                            * 15 as libc::c_int as libc::c_long
                            / 32 as libc::c_int as libc::c_long
                            - 5500 as libc::c_int as libc::c_long
                    })
                })
            })
        }) > ((*(*ctx).avctx).sample_rate / 2 as libc::c_int) as libc::c_long
        {
            ((*(*ctx).avctx).sample_rate / 2 as libc::c_int) as libc::c_long
        } else if (if (if (if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        })
            > 3000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 4 as libc::c_int as libc::c_long
        {
            3000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 4 as libc::c_int as libc::c_long
        } else {
            (if (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
                > (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    * 15 as libc::c_int as libc::c_long
                    / 32 as libc::c_int as libc::c_long
                    - 5500 as libc::c_int as libc::c_long
            {
                (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 5 as libc::c_int as libc::c_long
            } else {
                (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    * 15 as libc::c_int as libc::c_long
                    / 32 as libc::c_int as libc::c_long
                    - 5500 as libc::c_int as libc::c_long
            })
        })
            > 12000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 16 as libc::c_int as libc::c_long
        {
            12000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 16 as libc::c_int as libc::c_long
        } else {
            (if (if (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
                > (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    * 15 as libc::c_int as libc::c_long
                    / 32 as libc::c_int as libc::c_long
                    - 5500 as libc::c_int as libc::c_long
            {
                (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 5 as libc::c_int as libc::c_long
            } else {
                (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    * 15 as libc::c_int as libc::c_long
                    / 32 as libc::c_int as libc::c_long
                    - 5500 as libc::c_int as libc::c_long
            })
                > 3000 as libc::c_int as libc::c_long
                    + (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        / 4 as libc::c_int as libc::c_long
            {
                3000 as libc::c_int as libc::c_long
                    + (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        / 4 as libc::c_int as libc::c_long
            } else {
                (if (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 5 as libc::c_int as libc::c_long
                    > (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        * 15 as libc::c_int as libc::c_long
                        / 32 as libc::c_int as libc::c_long
                        - 5500 as libc::c_int as libc::c_long
                {
                    (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        / 5 as libc::c_int as libc::c_long
                } else {
                    (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        * 15 as libc::c_int as libc::c_long
                        / 32 as libc::c_int as libc::c_long
                        - 5500 as libc::c_int as libc::c_long
                })
            })
        }) > 22000 as libc::c_int as libc::c_long
        {
            22000 as libc::c_int as libc::c_long
        } else if (if (if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        })
            > 3000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 4 as libc::c_int as libc::c_long
        {
            3000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 4 as libc::c_int as libc::c_long
        } else {
            (if (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
                > (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    * 15 as libc::c_int as libc::c_long
                    / 32 as libc::c_int as libc::c_long
                    - 5500 as libc::c_int as libc::c_long
            {
                (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 5 as libc::c_int as libc::c_long
            } else {
                (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    * 15 as libc::c_int as libc::c_long
                    / 32 as libc::c_int as libc::c_long
                    - 5500 as libc::c_int as libc::c_long
            })
        })
            > 12000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 16 as libc::c_int as libc::c_long
        {
            12000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 16 as libc::c_int as libc::c_long
        } else if (if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        })
            > 3000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 4 as libc::c_int as libc::c_long
        {
            3000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 4 as libc::c_int as libc::c_long
        } else if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        }
    } else {
        ((*(*ctx).avctx).sample_rate / 2 as libc::c_int) as libc::c_long
    }) as libc::c_int;
    let num_bark: libc::c_float = calc_bark(bandwidth as libc::c_float);
    if bandwidth <= 0 as libc::c_int {
        return -(22 as libc::c_int);
    }
    (*ctx)
        .model_priv_data = av_mallocz(
        ::core::mem::size_of::<AacPsyContext>() as libc::c_ulong,
    );
    if ((*ctx).model_priv_data).is_null() {
        return -(12 as libc::c_int);
    }
    pctx = (*ctx).model_priv_data as *mut AacPsyContext;
    (*pctx)
        .global_quality = (if (*(*ctx).avctx).global_quality != 0 {
        (*(*ctx).avctx).global_quality
    } else {
        120 as libc::c_int
    }) as libc::c_float * 0.01f32;
    if (*(*ctx).avctx).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        chan_bitrate = (chan_bitrate as libc::c_double / 120.0f64
            * (if (*(*ctx).avctx).global_quality != 0 {
                (*(*ctx).avctx).global_quality
            } else {
                120 as libc::c_int
            }) as libc::c_double) as libc::c_int;
    }
    (*pctx).chan_bitrate = chan_bitrate;
    (*pctx)
        .frame_bits = if 2560 as libc::c_int
        > chan_bitrate * 1024 as libc::c_int / (*(*ctx).avctx).sample_rate
    {
        chan_bitrate * 1024 as libc::c_int / (*(*ctx).avctx).sample_rate
    } else {
        2560 as libc::c_int
    };
    (*pctx)
        .pe
        .min = 8.0f32 * 1024 as libc::c_int as libc::c_float * bandwidth as libc::c_float
        / ((*(*ctx).avctx).sample_rate as libc::c_float * 2.0f32);
    (*pctx)
        .pe
        .max = 12.0f32 * 1024 as libc::c_int as libc::c_float
        * bandwidth as libc::c_float
        / ((*(*ctx).avctx).sample_rate as libc::c_float * 2.0f32);
    (*ctx).bitres.size = 6144 as libc::c_int - (*pctx).frame_bits;
    (*ctx).bitres.size -= (*ctx).bitres.size % 8 as libc::c_int;
    (*pctx).fill_level = (*ctx).bitres.size;
    minath = ath(
        (3410 as libc::c_int as libc::c_double
            - 0.733f64 * 4 as libc::c_int as libc::c_double) as libc::c_float,
        4 as libc::c_int as libc::c_float,
    );
    j = 0 as libc::c_int;
    while j < 2 as libc::c_int {
        let mut coeffs: *mut AacPsyCoeffs = ((*pctx).psy_coef[j as usize]).as_mut_ptr();
        let mut band_sizes: *const uint8_t = *((*ctx).bands).offset(j as isize);
        let mut line_to_frequency: libc::c_float = (*(*ctx).avctx).sample_rate
            as libc::c_float / (if j != 0 { 256.0f32 } else { 2048.0f32 });
        let mut avg_chan_bits: libc::c_float = chan_bitrate as libc::c_float
            * (if j != 0 { 128.0f32 } else { 1024.0f32 })
            / (*(*ctx).avctx).sample_rate as libc::c_float;
        let mut bark_pe: libc::c_float = 0.024f32 * (avg_chan_bits * 1.18f32) / num_bark;
        let mut en_spread_low: libc::c_float = if j != 0 { 2.0f32 } else { 3.0f32 };
        let mut en_spread_hi: libc::c_float = if j != 0
            || chan_bitrate as libc::c_float <= 22.0f32
        {
            1.5f32
        } else {
            2.0f32
        };
        i = 0 as libc::c_int;
        prev = 0.0f64 as libc::c_float;
        g = 0 as libc::c_int;
        while g < *((*ctx).num_bands).offset(j as isize) {
            i += *band_sizes.offset(g as isize) as libc::c_int;
            bark = calc_bark(
                (i - 1 as libc::c_int) as libc::c_float * line_to_frequency,
            );
            (*coeffs.offset(g as isize))
                .barks = ((bark + prev) as libc::c_double / 2.0f64) as libc::c_float;
            prev = bark;
            g += 1;
            g;
        }
        g = 0 as libc::c_int;
        while g < *((*ctx).num_bands).offset(j as isize) - 1 as libc::c_int {
            let mut coeff: *mut AacPsyCoeffs = &mut *coeffs.offset(g as isize)
                as *mut AacPsyCoeffs;
            let mut bark_width: libc::c_float = (*coeffs
                .offset((g + 1 as libc::c_int) as isize))
                .barks - (*coeffs).barks;
            (*coeff)
                .spread_low[0 as libc::c_int
                as usize] = ff_exp10((-bark_width * 3.0f32) as libc::c_double)
                as libc::c_float;
            (*coeff)
                .spread_hi[0 as libc::c_int
                as usize] = ff_exp10((-bark_width * 1.5f32) as libc::c_double)
                as libc::c_float;
            (*coeff)
                .spread_low[1 as libc::c_int
                as usize] = ff_exp10((-bark_width * en_spread_low) as libc::c_double)
                as libc::c_float;
            (*coeff)
                .spread_hi[1 as libc::c_int
                as usize] = ff_exp10((-bark_width * en_spread_hi) as libc::c_double)
                as libc::c_float;
            pe_min = bark_pe * bark_width;
            minsnr = (exp2(
                (pe_min / *band_sizes.offset(g as isize) as libc::c_int as libc::c_float)
                    as libc::c_double,
            ) - 1.5f32 as libc::c_double) as libc::c_float;
            (*coeff)
                .min_snr = av_clipf_c(1.0f32 / minsnr, 3.1622776e-3f32, 7.9432821e-1f32);
            g += 1;
            g;
        }
        start = 0 as libc::c_int;
        g = 0 as libc::c_int;
        while g < *((*ctx).num_bands).offset(j as isize) {
            minscale = ath(
                start as libc::c_float * line_to_frequency,
                4 as libc::c_int as libc::c_float,
            );
            i = 1 as libc::c_int;
            while i < *band_sizes.offset(g as isize) as libc::c_int {
                minscale = if minscale
                    > ath(
                        (start + i) as libc::c_float * line_to_frequency,
                        4 as libc::c_int as libc::c_float,
                    )
                {
                    ath(
                        (start + i) as libc::c_float * line_to_frequency,
                        4 as libc::c_int as libc::c_float,
                    )
                } else {
                    minscale
                };
                i += 1;
                i;
            }
            (*coeffs.offset(g as isize)).ath = minscale - minath;
            start += *band_sizes.offset(g as isize) as libc::c_int;
            g += 1;
            g;
        }
        j += 1;
        j;
    }
    (*pctx)
        .ch = av_calloc(
        (*(*ctx).avctx).ch_layout.nb_channels as size_t,
        ::core::mem::size_of::<AacPsyChannel>() as libc::c_ulong,
    ) as *mut AacPsyChannel;
    if ((*pctx).ch).is_null() {
        av_freep(
            &mut (*ctx).model_priv_data as *mut *mut libc::c_void as *mut libc::c_void,
        );
        return -(12 as libc::c_int);
    }
    lame_window_init(pctx, (*ctx).avctx);
    return 0 as libc::c_int;
}
static mut window_grouping: [uint8_t; 9] = [
    0xb6 as libc::c_int as uint8_t,
    0x6c as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xb2 as libc::c_int as uint8_t,
    0x66 as libc::c_int as uint8_t,
    0xc6 as libc::c_int as uint8_t,
    0x96 as libc::c_int as uint8_t,
    0x36 as libc::c_int as uint8_t,
    0x36 as libc::c_int as uint8_t,
];
unsafe extern "C" fn calc_bit_demand(
    mut ctx: *mut AacPsyContext,
    mut pe: libc::c_float,
    mut bits: libc::c_int,
    mut size: libc::c_int,
    mut short_window: libc::c_int,
) -> libc::c_int {
    let bitsave_slope: libc::c_float = if short_window != 0 {
        -0.36363637f32
    } else {
        -0.46666667f32
    };
    let bitsave_add: libc::c_float = if short_window != 0 {
        -0.75f32
    } else {
        -0.84285712f32
    };
    let bitspend_slope: libc::c_float = if short_window != 0 {
        0.81818181f32
    } else {
        0.66666669f32
    };
    let bitspend_add: libc::c_float = if short_window != 0 {
        -0.26111111f32
    } else {
        -0.35f32
    };
    let clip_low: libc::c_float = if short_window != 0 { 0.2f32 } else { 0.2f32 };
    let clip_high: libc::c_float = if short_window != 0 { 0.75f32 } else { 0.95f32 };
    let mut clipped_pe: libc::c_float = 0.;
    let mut bit_save: libc::c_float = 0.;
    let mut bit_spend: libc::c_float = 0.;
    let mut bit_factor: libc::c_float = 0.;
    let mut fill_level: libc::c_float = 0.;
    let mut forgetful_min_pe: libc::c_float = 0.;
    (*ctx).fill_level += (*ctx).frame_bits - bits;
    (*ctx).fill_level = av_clip_c((*ctx).fill_level, 0 as libc::c_int, size);
    fill_level = av_clipf_c(
        (*ctx).fill_level as libc::c_float / size as libc::c_float,
        clip_low,
        clip_high,
    );
    clipped_pe = av_clipf_c(pe, (*ctx).pe.min, (*ctx).pe.max);
    bit_save = (fill_level + bitsave_add) * bitsave_slope;
    bit_spend = (fill_level + bitspend_add) * bitspend_slope;
    bit_factor = 1.0f32 - bit_save
        + (bit_spend - bit_save) / ((*ctx).pe.max - (*ctx).pe.min)
            * (clipped_pe - (*ctx).pe.min);
    (*ctx).pe.max = if pe > (*ctx).pe.max { pe } else { (*ctx).pe.max };
    forgetful_min_pe = ((*ctx).pe.min * 511 as libc::c_int as libc::c_float
        + (if (*ctx).pe.min > pe * (pe / (*ctx).pe.max) {
            (*ctx).pe.min
        } else {
            pe * (pe / (*ctx).pe.max)
        })) / (511 as libc::c_int + 1 as libc::c_int) as libc::c_float;
    (*ctx).pe.min = if pe > forgetful_min_pe { forgetful_min_pe } else { pe };
    return (if (*ctx).frame_bits as libc::c_float * bit_factor
        > (if (*ctx).frame_bits + size - bits > (*ctx).frame_bits / 8 as libc::c_int {
            (*ctx).frame_bits + size - bits
        } else {
            (*ctx).frame_bits / 8 as libc::c_int
        }) as libc::c_float
    {
        (if (*ctx).frame_bits + size - bits > (*ctx).frame_bits / 8 as libc::c_int {
            (*ctx).frame_bits + size - bits
        } else {
            (*ctx).frame_bits / 8 as libc::c_int
        }) as libc::c_float
    } else {
        (*ctx).frame_bits as libc::c_float * bit_factor
    }) as libc::c_int;
}
unsafe extern "C" fn calc_pe_3gpp(mut band: *mut AacPsyBand) -> libc::c_float {
    let mut pe: libc::c_float = 0.;
    let mut a: libc::c_float = 0.;
    (*band).pe = 0.0f32;
    (*band).pe_const = 0.0f32;
    (*band).active_lines = 0.0f32;
    if (*band).energy > (*band).thr {
        a = log2f((*band).energy);
        pe = a - log2f((*band).thr);
        (*band).active_lines = (*band).nz_lines;
        if pe < 3.0f32 {
            pe = pe * 0.55935729f32 + 1.3219281f32;
            a = a * 0.55935729f32 + 1.3219281f32;
            (*band).active_lines *= 0.55935729f32;
        }
        (*band).pe = pe * (*band).nz_lines;
        (*band).pe_const = a * (*band).nz_lines;
    }
    return (*band).pe;
}
unsafe extern "C" fn calc_reduction_3gpp(
    mut a: libc::c_float,
    mut desired_pe: libc::c_float,
    mut pe: libc::c_float,
    mut active_lines: libc::c_float,
) -> libc::c_float {
    let mut thr_avg: libc::c_float = 0.;
    let mut reduction: libc::c_float = 0.;
    if active_lines as libc::c_double == 0.0f64 {
        return 0 as libc::c_int as libc::c_float;
    }
    thr_avg = exp2f((a - pe) / (4.0f32 * active_lines));
    reduction = exp2f((a - desired_pe) / (4.0f32 * active_lines)) - thr_avg;
    return if reduction > 0.0f32 { reduction } else { 0.0f32 };
}
unsafe extern "C" fn calc_reduced_thr_3gpp(
    mut band: *mut AacPsyBand,
    mut min_snr: libc::c_float,
    mut reduction: libc::c_float,
) -> libc::c_float {
    let mut thr: libc::c_float = (*band).thr;
    if (*band).energy > thr {
        thr = sqrtf(thr);
        thr = sqrtf(thr) + reduction;
        thr *= thr;
        thr *= thr;
        if thr > (*band).energy * min_snr
            && (*band).avoid_holes != PSY_3GPP_AH_NONE as libc::c_int
        {
            thr = if (*band).thr > (*band).energy * min_snr {
                (*band).thr
            } else {
                (*band).energy * min_snr
            };
            (*band).avoid_holes = PSY_3GPP_AH_ACTIVE as libc::c_int;
        }
    }
    return thr;
}
unsafe extern "C" fn calc_thr_3gpp(
    mut wi: *const FFPsyWindowInfo,
    num_bands: libc::c_int,
    mut pch: *mut AacPsyChannel,
    mut band_sizes: *const uint8_t,
    mut coefs: *const libc::c_float,
    cutoff: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut start: libc::c_int = 0 as libc::c_int;
    let mut wstart: libc::c_int = 0 as libc::c_int;
    w = 0 as libc::c_int;
    while w < (*wi).num_windows * 16 as libc::c_int {
        wstart = 0 as libc::c_int;
        g = 0 as libc::c_int;
        while g < num_bands {
            let mut band: *mut AacPsyBand = &mut *((*pch).band)
                .as_mut_ptr()
                .offset((w + g) as isize) as *mut AacPsyBand;
            let mut form_factor: libc::c_float = 0.0f32;
            let mut Temp: libc::c_float = 0.;
            (*band).energy = 0.0f32;
            if wstart < cutoff {
                i = 0 as libc::c_int;
                while i < *band_sizes.offset(g as isize) as libc::c_int {
                    (*band).energy
                        += *coefs.offset((start + i) as isize)
                            * *coefs.offset((start + i) as isize);
                    form_factor
                        += sqrtf(
                            fabs(*coefs.offset((start + i) as isize) as libc::c_double)
                                as libc::c_float,
                        );
                    i += 1;
                    i;
                }
            }
            Temp = if (*band).energy > 0 as libc::c_int as libc::c_float {
                sqrtf(*band_sizes.offset(g as isize) as libc::c_float / (*band).energy)
            } else {
                0 as libc::c_int as libc::c_float
            };
            (*band).thr = (*band).energy * 0.001258925f32;
            (*band).nz_lines = form_factor * sqrtf(Temp);
            start += *band_sizes.offset(g as isize) as libc::c_int;
            wstart += *band_sizes.offset(g as isize) as libc::c_int;
            g += 1;
            g;
        }
        w += 16 as libc::c_int;
    }
}
unsafe extern "C" fn psy_hp_filter(
    mut firbuf: *const libc::c_float,
    mut hpfsmpl: *mut libc::c_float,
    mut psy_fir_coeffs_0: *const libc::c_float,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 1024 as libc::c_int {
        let mut sum1: libc::c_float = 0.;
        let mut sum2: libc::c_float = 0.;
        sum1 = *firbuf
            .offset(
                (i + (21 as libc::c_int - 1 as libc::c_int) / 2 as libc::c_int) as isize,
            );
        sum2 = 0.0f64 as libc::c_float;
        j = 0 as libc::c_int;
        while j
            < (21 as libc::c_int - 1 as libc::c_int) / 2 as libc::c_int
                - 1 as libc::c_int
        {
            sum1
                += *psy_fir_coeffs_0.offset(j as isize)
                    * (*firbuf.offset((i + j) as isize)
                        + *firbuf.offset((i + 21 as libc::c_int - j) as isize));
            sum2
                += *psy_fir_coeffs_0.offset((j + 1 as libc::c_int) as isize)
                    * (*firbuf.offset((i + j + 1 as libc::c_int) as isize)
                        + *firbuf
                            .offset(
                                (i + 21 as libc::c_int - j - 1 as libc::c_int) as isize,
                            ));
            j += 2 as libc::c_int;
        }
        *hpfsmpl.offset(i as isize) = (sum1 + sum2) * 32768.0f32;
        i += 1;
        i;
    }
}
unsafe extern "C" fn psy_3gpp_analyze_channel(
    mut ctx: *mut FFPsyContext,
    mut channel: libc::c_int,
    mut coefs: *const libc::c_float,
    mut wi: *const FFPsyWindowInfo,
) {
    let mut pctx: *mut AacPsyContext = (*ctx).model_priv_data as *mut AacPsyContext;
    let mut pch: *mut AacPsyChannel = &mut *((*pctx).ch).offset(channel as isize)
        as *mut AacPsyChannel;
    let mut i: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut desired_bits: libc::c_float = 0.;
    let mut desired_pe: libc::c_float = 0.;
    let mut delta_pe: libc::c_float = 0.;
    let mut reduction: libc::c_float = ::core::f32::NAN;
    let mut spread_en: [libc::c_float; 128] = [
        0 as libc::c_int as libc::c_float,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ];
    let mut a: libc::c_float = 0.0f32;
    let mut active_lines: libc::c_float = 0.0f32;
    let mut norm_fac: libc::c_float = 0.0f32;
    let mut pe: libc::c_float = if (*pctx).chan_bitrate > 32000 as libc::c_int {
        0.0f32
    } else if 50.0f32
        > 100.0f32 - (*pctx).chan_bitrate as libc::c_float * 100.0f32 / 32000.0f32
    {
        50.0f32
    } else {
        100.0f32 - (*pctx).chan_bitrate as libc::c_float * 100.0f32 / 32000.0f32
    };
    let num_bands: libc::c_int = *((*ctx).num_bands)
        .offset(((*wi).num_windows == 8 as libc::c_int) as libc::c_int as isize);
    let mut band_sizes: *const uint8_t = *((*ctx).bands)
        .offset(((*wi).num_windows == 8 as libc::c_int) as libc::c_int as isize);
    let mut coeffs: *mut AacPsyCoeffs = ((*pctx)
        .psy_coef[((*wi).num_windows == 8 as libc::c_int) as libc::c_int as usize])
        .as_mut_ptr();
    let avoid_hole_thr: libc::c_float = if (*wi).num_windows == 8 as libc::c_int {
        0.63f32
    } else {
        0.5f32
    };
    let bandwidth: libc::c_int = (if (*ctx).cutoff != 0 {
        (*ctx).cutoff as libc::c_long
    } else if (*(*ctx).avctx).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        ((*(*ctx).avctx).sample_rate / 2 as libc::c_int) as libc::c_long
    } else if (*(*ctx).avctx).bit_rate != 0 {
        if (if (if (if (if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        })
            > 3000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 4 as libc::c_int as libc::c_long
        {
            3000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 4 as libc::c_int as libc::c_long
        } else {
            (if (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
                > (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    * 15 as libc::c_int as libc::c_long
                    / 32 as libc::c_int as libc::c_long
                    - 5500 as libc::c_int as libc::c_long
            {
                (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 5 as libc::c_int as libc::c_long
            } else {
                (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    * 15 as libc::c_int as libc::c_long
                    / 32 as libc::c_int as libc::c_long
                    - 5500 as libc::c_int as libc::c_long
            })
        })
            > 12000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 16 as libc::c_int as libc::c_long
        {
            12000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 16 as libc::c_int as libc::c_long
        } else {
            (if (if (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
                > (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    * 15 as libc::c_int as libc::c_long
                    / 32 as libc::c_int as libc::c_long
                    - 5500 as libc::c_int as libc::c_long
            {
                (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 5 as libc::c_int as libc::c_long
            } else {
                (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    * 15 as libc::c_int as libc::c_long
                    / 32 as libc::c_int as libc::c_long
                    - 5500 as libc::c_int as libc::c_long
            })
                > 3000 as libc::c_int as libc::c_long
                    + (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        / 4 as libc::c_int as libc::c_long
            {
                3000 as libc::c_int as libc::c_long
                    + (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        / 4 as libc::c_int as libc::c_long
            } else {
                (if (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 5 as libc::c_int as libc::c_long
                    > (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        * 15 as libc::c_int as libc::c_long
                        / 32 as libc::c_int as libc::c_long
                        - 5500 as libc::c_int as libc::c_long
                {
                    (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        / 5 as libc::c_int as libc::c_long
                } else {
                    (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        * 15 as libc::c_int as libc::c_long
                        / 32 as libc::c_int as libc::c_long
                        - 5500 as libc::c_int as libc::c_long
                })
            })
        }) > 22000 as libc::c_int as libc::c_long
        {
            22000 as libc::c_int as libc::c_long
        } else {
            (if (if (if (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
                > (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    * 15 as libc::c_int as libc::c_long
                    / 32 as libc::c_int as libc::c_long
                    - 5500 as libc::c_int as libc::c_long
            {
                (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 5 as libc::c_int as libc::c_long
            } else {
                (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    * 15 as libc::c_int as libc::c_long
                    / 32 as libc::c_int as libc::c_long
                    - 5500 as libc::c_int as libc::c_long
            })
                > 3000 as libc::c_int as libc::c_long
                    + (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        / 4 as libc::c_int as libc::c_long
            {
                3000 as libc::c_int as libc::c_long
                    + (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        / 4 as libc::c_int as libc::c_long
            } else {
                (if (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 5 as libc::c_int as libc::c_long
                    > (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        * 15 as libc::c_int as libc::c_long
                        / 32 as libc::c_int as libc::c_long
                        - 5500 as libc::c_int as libc::c_long
                {
                    (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        / 5 as libc::c_int as libc::c_long
                } else {
                    (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        * 15 as libc::c_int as libc::c_long
                        / 32 as libc::c_int as libc::c_long
                        - 5500 as libc::c_int as libc::c_long
                })
            })
                > 12000 as libc::c_int as libc::c_long
                    + (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        / 16 as libc::c_int as libc::c_long
            {
                12000 as libc::c_int as libc::c_long
                    + (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        / 16 as libc::c_int as libc::c_long
            } else {
                (if (if (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 5 as libc::c_int as libc::c_long
                    > (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        * 15 as libc::c_int as libc::c_long
                        / 32 as libc::c_int as libc::c_long
                        - 5500 as libc::c_int as libc::c_long
                {
                    (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        / 5 as libc::c_int as libc::c_long
                } else {
                    (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        * 15 as libc::c_int as libc::c_long
                        / 32 as libc::c_int as libc::c_long
                        - 5500 as libc::c_int as libc::c_long
                })
                    > 3000 as libc::c_int as libc::c_long
                        + (*(*ctx).avctx).bit_rate
                            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                            / 4 as libc::c_int as libc::c_long
                {
                    3000 as libc::c_int as libc::c_long
                        + (*(*ctx).avctx).bit_rate
                            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                            / 4 as libc::c_int as libc::c_long
                } else {
                    (if (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        / 5 as libc::c_int as libc::c_long
                        > (*(*ctx).avctx).bit_rate
                            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                            * 15 as libc::c_int as libc::c_long
                            / 32 as libc::c_int as libc::c_long
                            - 5500 as libc::c_int as libc::c_long
                    {
                        (*(*ctx).avctx).bit_rate
                            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                            / 5 as libc::c_int as libc::c_long
                    } else {
                        (*(*ctx).avctx).bit_rate
                            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                            * 15 as libc::c_int as libc::c_long
                            / 32 as libc::c_int as libc::c_long
                            - 5500 as libc::c_int as libc::c_long
                    })
                })
            })
        }) > ((*(*ctx).avctx).sample_rate / 2 as libc::c_int) as libc::c_long
        {
            ((*(*ctx).avctx).sample_rate / 2 as libc::c_int) as libc::c_long
        } else if (if (if (if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        })
            > 3000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 4 as libc::c_int as libc::c_long
        {
            3000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 4 as libc::c_int as libc::c_long
        } else {
            (if (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
                > (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    * 15 as libc::c_int as libc::c_long
                    / 32 as libc::c_int as libc::c_long
                    - 5500 as libc::c_int as libc::c_long
            {
                (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 5 as libc::c_int as libc::c_long
            } else {
                (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    * 15 as libc::c_int as libc::c_long
                    / 32 as libc::c_int as libc::c_long
                    - 5500 as libc::c_int as libc::c_long
            })
        })
            > 12000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 16 as libc::c_int as libc::c_long
        {
            12000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 16 as libc::c_int as libc::c_long
        } else {
            (if (if (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
                > (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    * 15 as libc::c_int as libc::c_long
                    / 32 as libc::c_int as libc::c_long
                    - 5500 as libc::c_int as libc::c_long
            {
                (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 5 as libc::c_int as libc::c_long
            } else {
                (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    * 15 as libc::c_int as libc::c_long
                    / 32 as libc::c_int as libc::c_long
                    - 5500 as libc::c_int as libc::c_long
            })
                > 3000 as libc::c_int as libc::c_long
                    + (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        / 4 as libc::c_int as libc::c_long
            {
                3000 as libc::c_int as libc::c_long
                    + (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        / 4 as libc::c_int as libc::c_long
            } else {
                (if (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 5 as libc::c_int as libc::c_long
                    > (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        * 15 as libc::c_int as libc::c_long
                        / 32 as libc::c_int as libc::c_long
                        - 5500 as libc::c_int as libc::c_long
                {
                    (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        / 5 as libc::c_int as libc::c_long
                } else {
                    (*(*ctx).avctx).bit_rate
                        / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                        * 15 as libc::c_int as libc::c_long
                        / 32 as libc::c_int as libc::c_long
                        - 5500 as libc::c_int as libc::c_long
                })
            })
        }) > 22000 as libc::c_int as libc::c_long
        {
            22000 as libc::c_int as libc::c_long
        } else if (if (if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        })
            > 3000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 4 as libc::c_int as libc::c_long
        {
            3000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 4 as libc::c_int as libc::c_long
        } else {
            (if (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
                > (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    * 15 as libc::c_int as libc::c_long
                    / 32 as libc::c_int as libc::c_long
                    - 5500 as libc::c_int as libc::c_long
            {
                (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 5 as libc::c_int as libc::c_long
            } else {
                (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    * 15 as libc::c_int as libc::c_long
                    / 32 as libc::c_int as libc::c_long
                    - 5500 as libc::c_int as libc::c_long
            })
        })
            > 12000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 16 as libc::c_int as libc::c_long
        {
            12000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 16 as libc::c_int as libc::c_long
        } else if (if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        })
            > 3000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 4 as libc::c_int as libc::c_long
        {
            3000 as libc::c_int as libc::c_long
                + (*(*ctx).avctx).bit_rate
                    / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                    / 4 as libc::c_int as libc::c_long
        } else if (*(*ctx).avctx).bit_rate
            / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
            / 5 as libc::c_int as libc::c_long
            > (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                / 5 as libc::c_int as libc::c_long
        } else {
            (*(*ctx).avctx).bit_rate
                / (*(*ctx).avctx).ch_layout.nb_channels as libc::c_long
                * 15 as libc::c_int as libc::c_long / 32 as libc::c_int as libc::c_long
                - 5500 as libc::c_int as libc::c_long
        }
    } else {
        ((*(*ctx).avctx).sample_rate / 2 as libc::c_int) as libc::c_long
    }) as libc::c_int;
    let cutoff: libc::c_int = bandwidth * 2048 as libc::c_int / (*wi).num_windows
        / (*(*ctx).avctx).sample_rate;
    calc_thr_3gpp(wi, num_bands, pch, band_sizes, coefs, cutoff);
    w = 0 as libc::c_int;
    while w < (*wi).num_windows * 16 as libc::c_int {
        let mut bands: *mut AacPsyBand = &mut *((*pch).band)
            .as_mut_ptr()
            .offset(w as isize) as *mut AacPsyBand;
        spread_en[0 as libc::c_int
            as usize] = (*bands.offset(0 as libc::c_int as isize)).energy;
        g = 1 as libc::c_int;
        while g < num_bands {
            (*bands.offset(g as isize))
                .thr = if (*bands.offset(g as isize)).thr
                > (*bands.offset((g - 1 as libc::c_int) as isize)).thr
                    * (*coeffs.offset(g as isize)).spread_hi[0 as libc::c_int as usize]
            {
                (*bands.offset(g as isize)).thr
            } else {
                (*bands.offset((g - 1 as libc::c_int) as isize)).thr
                    * (*coeffs.offset(g as isize)).spread_hi[0 as libc::c_int as usize]
            };
            spread_en[(w + g)
                as usize] = if (*bands.offset(g as isize)).energy
                > spread_en[(w + g - 1 as libc::c_int) as usize]
                    * (*coeffs.offset(g as isize)).spread_hi[1 as libc::c_int as usize]
            {
                (*bands.offset(g as isize)).energy
            } else {
                spread_en[(w + g - 1 as libc::c_int) as usize]
                    * (*coeffs.offset(g as isize)).spread_hi[1 as libc::c_int as usize]
            };
            g += 1;
            g;
        }
        g = num_bands - 2 as libc::c_int;
        while g >= 0 as libc::c_int {
            (*bands.offset(g as isize))
                .thr = if (*bands.offset(g as isize)).thr
                > (*bands.offset((g + 1 as libc::c_int) as isize)).thr
                    * (*coeffs.offset(g as isize)).spread_low[0 as libc::c_int as usize]
            {
                (*bands.offset(g as isize)).thr
            } else {
                (*bands.offset((g + 1 as libc::c_int) as isize)).thr
                    * (*coeffs.offset(g as isize)).spread_low[0 as libc::c_int as usize]
            };
            spread_en[(w + g)
                as usize] = if spread_en[(w + g) as usize]
                > spread_en[(w + g + 1 as libc::c_int) as usize]
                    * (*coeffs.offset(g as isize)).spread_low[1 as libc::c_int as usize]
            {
                spread_en[(w + g) as usize]
            } else {
                spread_en[(w + g + 1 as libc::c_int) as usize]
                    * (*coeffs.offset(g as isize)).spread_low[1 as libc::c_int as usize]
            };
            g -= 1;
            g;
        }
        g = 0 as libc::c_int;
        while g < num_bands {
            let mut band: *mut AacPsyBand = &mut *bands.offset(g as isize)
                as *mut AacPsyBand;
            (*band)
                .thr = if (*band).thr > (*coeffs.offset(g as isize)).ath {
                (*band).thr
            } else {
                (*coeffs.offset(g as isize)).ath
            };
            (*band).thr_quiet = (*band).thr;
            if !((*wi).window_type[0 as libc::c_int as usize]
                == LONG_STOP_SEQUENCE as libc::c_int
                || w == 0
                    && (*wi).window_type[1 as libc::c_int as usize]
                        == LONG_START_SEQUENCE as libc::c_int)
            {
                (*band)
                    .thr = if 0.01f32 * (*band).thr
                    > (if (*band).thr
                        > 2.0f32 * (*pch).prev_band[(w + g) as usize].thr_quiet
                    {
                        2.0f32 * (*pch).prev_band[(w + g) as usize].thr_quiet
                    } else {
                        (*band).thr
                    })
                {
                    0.01f32 * (*band).thr
                } else if (*band).thr
                    > 2.0f32 * (*pch).prev_band[(w + g) as usize].thr_quiet
                {
                    2.0f32 * (*pch).prev_band[(w + g) as usize].thr_quiet
                } else {
                    (*band).thr
                };
            }
            pe += calc_pe_3gpp(band);
            a += (*band).pe_const;
            active_lines += (*band).active_lines;
            if spread_en[(w + g) as usize] * avoid_hole_thr > (*band).energy
                || (*coeffs.offset(g as isize)).min_snr > 1.0f32
            {
                (*band).avoid_holes = PSY_3GPP_AH_NONE as libc::c_int;
            } else {
                (*band).avoid_holes = PSY_3GPP_AH_INACTIVE as libc::c_int;
            }
            g += 1;
            g;
        }
        w += 16 as libc::c_int;
    }
    (*((*ctx).ch).offset(channel as isize)).entropy = pe;
    if (*(*ctx).avctx).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        desired_pe = pe
            * (if (*(*ctx).avctx).global_quality != 0 {
                (*(*ctx).avctx).global_quality
            } else {
                120 as libc::c_int
            }) as libc::c_float
            / (2 as libc::c_int as libc::c_float * 2.5f32 * 120.0f32);
        desired_bits = if 2560 as libc::c_int as libc::c_float > desired_pe / 1.18f32 {
            desired_pe / 1.18f32
        } else {
            2560 as libc::c_int as libc::c_float
        };
        desired_pe = desired_bits * 1.18f32;
        if (*ctx).bitres.bits > 0 as libc::c_int {
            desired_bits = if 2560 as libc::c_int as libc::c_float > desired_pe / 1.18f32
            {
                desired_pe / 1.18f32
            } else {
                2560 as libc::c_int as libc::c_float
            };
            desired_pe = desired_bits * 1.18f32;
        }
        (*pctx).pe.max = if pe > (*pctx).pe.max { pe } else { (*pctx).pe.max };
        (*pctx).pe.min = if pe > (*pctx).pe.min { (*pctx).pe.min } else { pe };
    } else {
        desired_bits = calc_bit_demand(
            pctx,
            pe,
            (*ctx).bitres.bits,
            (*ctx).bitres.size,
            ((*wi).num_windows == 8 as libc::c_int) as libc::c_int,
        ) as libc::c_float;
        desired_pe = desired_bits * 1.18f32;
        if (*ctx).bitres.bits > 0 as libc::c_int {
            desired_pe
                *= av_clipf_c(
                    (*pctx).pe.previous
                        / ((*ctx).bitres.bits as libc::c_float * 1.18f32),
                    0.85f32,
                    1.15f32,
                );
        }
    }
    (*pctx).pe.previous = desired_bits * 1.18f32;
    (*ctx).bitres.alloc = desired_bits as libc::c_int;
    if desired_pe < pe {
        w = 0 as libc::c_int;
        while w < (*wi).num_windows * 16 as libc::c_int {
            reduction = calc_reduction_3gpp(a, desired_pe, pe, active_lines);
            pe = 0.0f32;
            a = 0.0f32;
            active_lines = 0.0f32;
            g = 0 as libc::c_int;
            while g < num_bands {
                let mut band_0: *mut AacPsyBand = &mut *((*pch).band)
                    .as_mut_ptr()
                    .offset((w + g) as isize) as *mut AacPsyBand;
                (*band_0)
                    .thr = calc_reduced_thr_3gpp(
                    band_0,
                    (*coeffs.offset(g as isize)).min_snr,
                    reduction,
                );
                pe += calc_pe_3gpp(band_0);
                a += (*band_0).pe_const;
                active_lines += (*band_0).active_lines;
                g += 1;
                g;
            }
            w += 16 as libc::c_int;
        }
        i = 0 as libc::c_int;
        while i < 2 as libc::c_int {
            let mut pe_no_ah: libc::c_float = 0.0f32;
            let mut desired_pe_no_ah: libc::c_float = 0.;
            a = 0.0f32;
            active_lines = a;
            w = 0 as libc::c_int;
            while w < (*wi).num_windows * 16 as libc::c_int {
                g = 0 as libc::c_int;
                while g < num_bands {
                    let mut band_1: *mut AacPsyBand = &mut *((*pch).band)
                        .as_mut_ptr()
                        .offset((w + g) as isize) as *mut AacPsyBand;
                    if (*band_1).avoid_holes != PSY_3GPP_AH_ACTIVE as libc::c_int {
                        pe_no_ah += (*band_1).pe;
                        a += (*band_1).pe_const;
                        active_lines += (*band_1).active_lines;
                    }
                    g += 1;
                    g;
                }
                w += 16 as libc::c_int;
            }
            desired_pe_no_ah = if desired_pe - (pe - pe_no_ah) > 0.0f32 {
                desired_pe - (pe - pe_no_ah)
            } else {
                0.0f32
            };
            if active_lines > 0.0f32 {
                reduction = calc_reduction_3gpp(
                    a,
                    desired_pe_no_ah,
                    pe_no_ah,
                    active_lines,
                );
            }
            pe = 0.0f32;
            w = 0 as libc::c_int;
            while w < (*wi).num_windows * 16 as libc::c_int {
                g = 0 as libc::c_int;
                while g < num_bands {
                    let mut band_2: *mut AacPsyBand = &mut *((*pch).band)
                        .as_mut_ptr()
                        .offset((w + g) as isize) as *mut AacPsyBand;
                    if active_lines > 0.0f32 {
                        (*band_2)
                            .thr = calc_reduced_thr_3gpp(
                            band_2,
                            (*coeffs.offset(g as isize)).min_snr,
                            reduction,
                        );
                    }
                    pe += calc_pe_3gpp(band_2);
                    if (*band_2).thr > 0.0f32 {
                        (*band_2).norm_fac = (*band_2).active_lines / (*band_2).thr;
                    } else {
                        (*band_2).norm_fac = 0.0f32;
                    }
                    norm_fac += (*band_2).norm_fac;
                    g += 1;
                    g;
                }
                w += 16 as libc::c_int;
            }
            delta_pe = desired_pe - pe;
            if fabs(delta_pe as libc::c_double)
                > (0.05f32 * desired_pe) as libc::c_double
            {
                break;
            }
            i += 1;
            i;
        }
        if pe < 1.15f32 * desired_pe {
            norm_fac = if norm_fac != 0. {
                1.0f32 / norm_fac
            } else {
                0 as libc::c_int as libc::c_float
            };
            w = 0 as libc::c_int;
            while w < (*wi).num_windows * 16 as libc::c_int {
                g = 0 as libc::c_int;
                while g < num_bands {
                    let mut band_3: *mut AacPsyBand = &mut *((*pch).band)
                        .as_mut_ptr()
                        .offset((w + g) as isize) as *mut AacPsyBand;
                    if (*band_3).active_lines > 0.5f32 {
                        let mut delta_sfb_pe: libc::c_float = (*band_3).norm_fac
                            * norm_fac * delta_pe;
                        let mut thr: libc::c_float = (*band_3).thr;
                        thr *= exp2f(delta_sfb_pe / (*band_3).active_lines);
                        if thr > (*coeffs.offset(g as isize)).min_snr * (*band_3).energy
                            && (*band_3).avoid_holes
                                == PSY_3GPP_AH_INACTIVE as libc::c_int
                        {
                            thr = if (*band_3).thr
                                > (*coeffs.offset(g as isize)).min_snr * (*band_3).energy
                            {
                                (*band_3).thr
                            } else {
                                (*coeffs.offset(g as isize)).min_snr * (*band_3).energy
                            };
                        }
                        (*band_3).thr = thr;
                    }
                    g += 1;
                    g;
                }
                w += 16 as libc::c_int;
            }
        } else {
            g = num_bands;
            while pe > desired_pe
                && {
                    let fresh0 = g;
                    g = g - 1;
                    fresh0 != 0
                }
            {
                w = 0 as libc::c_int;
                while w < (*wi).num_windows * 16 as libc::c_int {
                    let mut band_4: *mut AacPsyBand = &mut *((*pch).band)
                        .as_mut_ptr()
                        .offset((w + g) as isize) as *mut AacPsyBand;
                    if (*band_4).avoid_holes != PSY_3GPP_AH_NONE as libc::c_int
                        && (*coeffs.offset(g as isize)).min_snr < 7.9432821e-1f32
                    {
                        (*coeffs.offset(g as isize)).min_snr = 7.9432821e-1f32;
                        (*band_4).thr = (*band_4).energy * 7.9432821e-1f32;
                        pe += (*band_4).active_lines * 1.5f32 - (*band_4).pe;
                    }
                    w += 16 as libc::c_int;
                }
            }
        }
    }
    w = 0 as libc::c_int;
    while w < (*wi).num_windows * 16 as libc::c_int {
        g = 0 as libc::c_int;
        while g < num_bands {
            let mut band_5: *mut AacPsyBand = &mut *((*pch).band)
                .as_mut_ptr()
                .offset((w + g) as isize) as *mut AacPsyBand;
            let mut psy_band: *mut FFPsyBand = &mut *((*((*ctx).ch)
                .offset(channel as isize))
                .psy_bands)
                .as_mut_ptr()
                .offset((w + g) as isize) as *mut FFPsyBand;
            (*psy_band).threshold = (*band_5).thr;
            (*psy_band).energy = (*band_5).energy;
            (*psy_band)
                .spread = (*band_5).active_lines * 2.0f32
                / *band_sizes.offset(g as isize) as libc::c_int as libc::c_float;
            (*psy_band).bits = ((*band_5).pe / 1.18f32) as libc::c_int;
            g += 1;
            g;
        }
        w += 16 as libc::c_int;
    }
    memcpy(
        ((*pch).prev_band).as_mut_ptr() as *mut libc::c_void,
        ((*pch).band).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[AacPsyBand; 128]>() as libc::c_ulong,
    );
}
unsafe extern "C" fn psy_3gpp_analyze(
    mut ctx: *mut FFPsyContext,
    mut channel: libc::c_int,
    mut coeffs: *mut *const libc::c_float,
    mut wi: *const FFPsyWindowInfo,
) {
    let mut ch: libc::c_int = 0;
    let mut group: *mut FFPsyChannelGroup = ff_psy_find_group(ctx, channel);
    ch = 0 as libc::c_int;
    while ch < (*group).num_ch as libc::c_int {
        psy_3gpp_analyze_channel(
            ctx,
            channel + ch,
            *coeffs.offset(ch as isize),
            &*wi.offset(ch as isize),
        );
        ch += 1;
        ch;
    }
}
#[cold]
unsafe extern "C" fn psy_3gpp_end(mut apc: *mut FFPsyContext) {
    let mut pctx: *mut AacPsyContext = (*apc).model_priv_data as *mut AacPsyContext;
    if !pctx.is_null() {
        av_freep(&mut (*pctx).ch as *mut *mut AacPsyChannel as *mut libc::c_void);
    }
    av_freep(&mut (*apc).model_priv_data as *mut *mut libc::c_void as *mut libc::c_void);
}
unsafe extern "C" fn lame_apply_block_type(
    mut ctx: *mut AacPsyChannel,
    mut wi: *mut FFPsyWindowInfo,
    mut uselongblock: libc::c_int,
) {
    let mut blocktype: libc::c_int = ONLY_LONG_SEQUENCE as libc::c_int;
    if uselongblock != 0 {
        if (*ctx).next_window_seq as libc::c_uint
            == EIGHT_SHORT_SEQUENCE as libc::c_int as libc::c_uint
        {
            blocktype = LONG_STOP_SEQUENCE as libc::c_int;
        }
    } else {
        blocktype = EIGHT_SHORT_SEQUENCE as libc::c_int;
        if (*ctx).next_window_seq as libc::c_uint
            == ONLY_LONG_SEQUENCE as libc::c_int as libc::c_uint
        {
            (*ctx).next_window_seq = LONG_START_SEQUENCE;
        }
        if (*ctx).next_window_seq as libc::c_uint
            == LONG_STOP_SEQUENCE as libc::c_int as libc::c_uint
        {
            (*ctx).next_window_seq = EIGHT_SHORT_SEQUENCE;
        }
    }
    (*wi).window_type[0 as libc::c_int as usize] = (*ctx).next_window_seq as libc::c_int;
    (*ctx).next_window_seq = blocktype as WindowSequence;
}
unsafe extern "C" fn psy_lame_window(
    mut ctx: *mut FFPsyContext,
    mut audio: *const libc::c_float,
    mut la: *const libc::c_float,
    mut channel: libc::c_int,
    mut prev_type: libc::c_int,
) -> FFPsyWindowInfo {
    let mut pctx: *mut AacPsyContext = (*ctx).model_priv_data as *mut AacPsyContext;
    let mut pch: *mut AacPsyChannel = &mut *((*pctx).ch).offset(channel as isize)
        as *mut AacPsyChannel;
    let mut grouping: libc::c_int = 0 as libc::c_int;
    let mut uselongblock: libc::c_int = 1 as libc::c_int;
    let mut attacks: [libc::c_int; 9] = [0 as libc::c_int, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut i: libc::c_int = 0;
    let mut wi: FFPsyWindowInfo = {
        let mut init = FFPsyWindowInfo {
            window_type: [0 as libc::c_int, 0, 0],
            window_shape: 0,
            num_windows: 0,
            grouping: [0; 8],
            clipping: [0.; 8],
            window_sizes: 0 as *mut libc::c_int,
        };
        init
    };
    if !la.is_null() {
        let mut hpfsmpl: [libc::c_float; 1024] = [0.; 1024];
        let mut pf: *const libc::c_float = hpfsmpl.as_mut_ptr();
        let mut attack_intensity: [libc::c_float; 27] = [0.; 27];
        let mut energy_subshort: [libc::c_float; 27] = [0.; 27];
        let mut energy_short: [libc::c_float; 9] = [
            0 as libc::c_int as libc::c_float,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
        ];
        let mut firbuf: *const libc::c_float = la
            .offset(
                (128 as libc::c_int / 4 as libc::c_int - 21 as libc::c_int) as isize,
            );
        let mut att_sum: libc::c_int = 0 as libc::c_int;
        psy_hp_filter(firbuf, hpfsmpl.as_mut_ptr(), psy_fir_coeffs.as_ptr());
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            energy_subshort[i
                as usize] = (*pch)
                .prev_energy_subshort[(i
                + (8 as libc::c_int - 1 as libc::c_int) * 3 as libc::c_int) as usize];
            attack_intensity[i
                as usize] = energy_subshort[i as usize]
                / (*pch)
                    .prev_energy_subshort[(i
                    + ((8 as libc::c_int - 2 as libc::c_int) * 3 as libc::c_int
                        + 1 as libc::c_int)) as usize];
            energy_short[0 as libc::c_int as usize] += energy_subshort[i as usize];
            i += 1;
            i;
        }
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int * 3 as libc::c_int {
            let pfe: *const libc::c_float = pf
                .offset(
                    (1024 as libc::c_int / (8 as libc::c_int * 3 as libc::c_int))
                        as isize,
                );
            let mut p: libc::c_float = 1.0f32;
            while pf < pfe {
                p = if p > fabsf(*pf) { p } else { fabsf(*pf) };
                pf = pf.offset(1);
                pf;
            }
            energy_subshort[(i + 3 as libc::c_int) as usize] = p;
            (*pch)
                .prev_energy_subshort[i
                as usize] = energy_subshort[(i + 3 as libc::c_int) as usize];
            energy_short[(1 as libc::c_int + i / 3 as libc::c_int) as usize] += p;
            if p > energy_subshort[(i + 1 as libc::c_int) as usize] {
                p = p / energy_subshort[(i + 1 as libc::c_int) as usize];
            } else if energy_subshort[(i + 1 as libc::c_int) as usize] > p * 10.0f32 {
                p = energy_subshort[(i + 1 as libc::c_int) as usize] / (p * 10.0f32);
            } else {
                p = 0.0f64 as libc::c_float;
            }
            attack_intensity[(i + 3 as libc::c_int) as usize] = p;
            i += 1;
            i;
        }
        i = 0 as libc::c_int;
        while i < (8 as libc::c_int + 1 as libc::c_int) * 3 as libc::c_int {
            if attacks[(i / 3 as libc::c_int) as usize] == 0 {
                if attack_intensity[i as usize] > (*pch).attack_threshold {
                    attacks[(i / 3 as libc::c_int)
                        as usize] = i % 3 as libc::c_int + 1 as libc::c_int;
                }
            }
            i += 1;
            i;
        }
        i = 1 as libc::c_int;
        while i < 8 as libc::c_int + 1 as libc::c_int {
            let u: libc::c_float = energy_short[(i - 1 as libc::c_int) as usize];
            let v: libc::c_float = energy_short[i as usize];
            let m: libc::c_float = if u > v { u } else { v };
            if m < 40000 as libc::c_int as libc::c_float {
                if u < 1.7f32 * v && v < 1.7f32 * u {
                    if i == 1 as libc::c_int
                        && attacks[0 as libc::c_int as usize] < attacks[i as usize]
                    {
                        attacks[0 as libc::c_int as usize] = 0 as libc::c_int;
                    }
                    attacks[i as usize] = 0 as libc::c_int;
                }
            }
            att_sum += attacks[i as usize];
            i += 1;
            i;
        }
        if attacks[0 as libc::c_int as usize] <= (*pch).prev_attack {
            attacks[0 as libc::c_int as usize] = 0 as libc::c_int;
        }
        att_sum += attacks[0 as libc::c_int as usize];
        if (*pch).prev_attack == 3 as libc::c_int || att_sum != 0 {
            uselongblock = 0 as libc::c_int;
            i = 1 as libc::c_int;
            while i < 8 as libc::c_int + 1 as libc::c_int {
                if attacks[i as usize] != 0
                    && attacks[(i - 1 as libc::c_int) as usize] != 0
                {
                    attacks[i as usize] = 0 as libc::c_int;
                }
                i += 1;
                i;
            }
        }
    } else {
        uselongblock = !(prev_type == EIGHT_SHORT_SEQUENCE as libc::c_int)
            as libc::c_int;
    }
    lame_apply_block_type(pch, &mut wi, uselongblock);
    wi.window_type[1 as libc::c_int as usize] = prev_type;
    if wi.window_type[0 as libc::c_int as usize] != EIGHT_SHORT_SEQUENCE as libc::c_int {
        wi.num_windows = 1 as libc::c_int;
        wi.grouping[0 as libc::c_int as usize] = 1 as libc::c_int;
        if wi.window_type[0 as libc::c_int as usize]
            == LONG_START_SEQUENCE as libc::c_int
        {
            wi.window_shape = 0 as libc::c_int;
        } else {
            wi.window_shape = 1 as libc::c_int;
        }
    } else {
        let mut lastgrp: libc::c_int = 0 as libc::c_int;
        wi.num_windows = 8 as libc::c_int;
        wi.window_shape = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            if (*pch).next_grouping as libc::c_int >> i & 1 as libc::c_int == 0 {
                lastgrp = i;
            }
            wi.grouping[lastgrp as usize] += 1;
            wi.grouping[lastgrp as usize];
            i += 1;
            i;
        }
    }
    i = 0 as libc::c_int;
    while i < 9 as libc::c_int {
        if attacks[i as usize] != 0 {
            grouping = i;
            break;
        } else {
            i += 1;
            i;
        }
    }
    (*pch).next_grouping = window_grouping[grouping as usize];
    (*pch).prev_attack = attacks[8 as libc::c_int as usize];
    return wi;
}
#[no_mangle]
pub static mut ff_aac_psy_model: FFPsyModel = unsafe {
    {
        let mut init = FFPsyModel {
            name: b"3GPP TS 26.403-inspired model\0" as *const u8 as *const libc::c_char,
            init: Some(
                psy_3gpp_init as unsafe extern "C" fn(*mut FFPsyContext) -> libc::c_int,
            ),
            window: Some(
                psy_lame_window
                    as unsafe extern "C" fn(
                        *mut FFPsyContext,
                        *const libc::c_float,
                        *const libc::c_float,
                        libc::c_int,
                        libc::c_int,
                    ) -> FFPsyWindowInfo,
            ),
            analyze: Some(
                psy_3gpp_analyze
                    as unsafe extern "C" fn(
                        *mut FFPsyContext,
                        libc::c_int,
                        *mut *const libc::c_float,
                        *const FFPsyWindowInfo,
                    ) -> (),
            ),
            end: Some(psy_3gpp_end as unsafe extern "C" fn(*mut FFPsyContext) -> ()),
        };
        init
    }
};
