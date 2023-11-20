#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use std::{ffi::CStr, mem::size_of, ptr};

use libc::{
    c_char, c_double, c_float, c_int, c_long, c_schar, c_uchar, c_uint, c_ulong, c_ulonglong,
    c_ushort, c_void,
};

use crate::{
    aaccoder::ff_aac_coders,
    aacenctab::{
        ff_aac_swb_size_1024, ff_aac_swb_size_1024_len, ff_aac_swb_size_128,
        ff_aac_swb_size_128_len,
    },
    aactab::{
        ff_aac_float_common_init, ff_aac_kbd_long_1024, ff_aac_kbd_short_128, ff_aac_num_swb_1024,
        ff_aac_num_swb_128, ff_aac_scalefactor_bits, ff_aac_scalefactor_code, ff_swb_offset_1024,
        ff_swb_offset_128, ff_tns_max_bands_1024, ff_tns_max_bands_128,
    },
    audio_frame_queue::{ff_af_queue_add, ff_af_queue_close, ff_af_queue_init, ff_af_queue_remove},
    avutil::{
        log::av_default_item_name,
        tx::{av_tx_init, av_tx_uninit},
    },
    common::*,
    lpc::{ff_lpc_end, ff_lpc_init},
    mpeg4audio_sample_rates::ff_mpeg4audio_sample_rates,
    psymodel::{
        ff_psy_end, ff_psy_init, ff_psy_preprocess, ff_psy_preprocess_end, ff_psy_preprocess_init,
    },
    sinewin::{ff_sine_1024, ff_sine_128},
    types::*,
};

extern "C" {
    fn av_channel_layout_describe(
        channel_layout: *const AVChannelLayout,
        buf: *mut c_char,
        buf_size: c_ulong,
    ) -> c_int;
    fn av_channel_layout_compare(
        chl: *const AVChannelLayout,
        chl1: *const AVChannelLayout,
    ) -> c_int;
    fn avpriv_float_dsp_alloc(strict: c_int) -> *mut AVFloatDSPContext;
    fn av_mallocz(size: c_ulong) -> *mut c_void;
    fn av_calloc(nmemb: c_ulong, size: c_ulong) -> *mut c_void;
    fn av_freep(ptr: *mut c_void);
    fn av_log(avcl: *mut c_void, level: c_int, fmt: *const c_char, _: ...);
    fn ff_alloc_packet(avctx: *mut AVCodecContext, avpkt: *mut AVPacket, size: c_long) -> c_int;
}
#[inline(always)]
unsafe extern "C" fn av_clipf_c(mut a: c_float, mut amin: c_float, mut amax: c_float) -> c_float {
    if (if a > amin { a } else { amin }) > amax {
        amax
    } else if a > amin {
        a
    } else {
        amin
    }
}
static mut BUF_BITS: c_int = 0;
#[inline]
unsafe extern "C" fn init_put_bits(
    mut s: *mut PutBitContext,
    mut buffer: *mut c_uchar,
    mut buffer_size: c_int,
) {
    if buffer_size < 0 as c_int {
        buffer_size = 0 as c_int;
        buffer = ptr::null_mut::<c_uchar>();
    }
    (*s).buf = buffer;
    (*s).buf_end = ((*s).buf).offset(buffer_size as isize);
    (*s).buf_ptr = (*s).buf;
    (*s).bit_left = BUF_BITS;
    (*s).bit_buf = 0 as c_int as BitBuf;
}
#[inline]
unsafe extern "C" fn put_bits_count(mut s: *mut PutBitContext) -> c_int {
    (((*s).buf_ptr).offset_from((*s).buf) as c_long * 8 as c_int as c_long + BUF_BITS as c_long
        - (*s).bit_left as c_long) as c_int
}
#[inline]
unsafe extern "C" fn put_bytes_output(mut s: *const PutBitContext) -> c_int {
    ((*s).buf_ptr).offset_from((*s).buf) as c_long as c_int
}
#[inline]
unsafe extern "C" fn flush_put_bits(mut s: *mut PutBitContext) {
    if (*s).bit_left < BUF_BITS {
        (*s).bit_buf <<= (*s).bit_left;
    }
    while (*s).bit_left < BUF_BITS {
        assert!((*s).buf_ptr < (*s).buf_end);
        let fresh0 = (*s).buf_ptr;
        (*s).buf_ptr = ((*s).buf_ptr).offset(1);
        *fresh0 = ((*s).bit_buf >> BUF_BITS - 8 as c_int) as c_uchar;
        (*s).bit_buf <<= 8 as c_int;
        (*s).bit_left += 8 as c_int;
    }
    (*s).bit_left = BUF_BITS;
    (*s).bit_buf = 0 as c_int as BitBuf;
}
#[inline]
unsafe extern "C" fn put_bits_no_assert(
    mut s: *mut PutBitContext,
    mut n: c_int,
    mut value: BitBuf,
) {
    let mut bit_buf: BitBuf = 0;
    let mut bit_left: c_int = 0;
    bit_buf = (*s).bit_buf;
    bit_left = (*s).bit_left;
    if n < bit_left {
        bit_buf = bit_buf << n | value;
        bit_left -= n;
    } else {
        bit_buf <<= bit_left;
        bit_buf |= value >> n - bit_left;
        if ((*s).buf_end).offset_from((*s).buf_ptr) as c_long as c_ulong
            >= size_of::<BitBuf>() as c_ulong
        {
            (*((*s).buf_ptr as *mut unaligned_32)).l = bit_buf;
            (*s).buf_ptr = ((*s).buf_ptr).offset(size_of::<BitBuf>() as c_ulong as isize);
        } else {
            panic!("Internal error, put_bits buffer too small");
        }
        bit_left += BUF_BITS - n;
        bit_buf = value;
    }
    (*s).bit_buf = bit_buf;
    (*s).bit_left = bit_left;
}
#[inline]
unsafe extern "C" fn put_bits(mut s: *mut PutBitContext, mut n: c_int, mut value: BitBuf) {
    put_bits_no_assert(s, n, value);
}

#[inline]
unsafe extern "C" fn align_put_bits(mut s: *mut PutBitContext) {
    put_bits(s, (*s).bit_left & 7 as c_int, 0 as c_int as BitBuf);
}
static mut aac_normal_chan_layouts: [AVChannelLayout; 7] = [
    {
        AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 1 as c_int,
            u: C2RustUnnamed {
                mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_CENTER as c_int) as c_ulong,
            },
            opaque: 0 as *const c_void as *mut c_void,
        }
    },
    {
        AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 2 as c_int,
            u: C2RustUnnamed {
                mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_LEFT as c_int
                    | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT as c_int)
                    as c_ulong,
            },
            opaque: 0 as *const c_void as *mut c_void,
        }
    },
    {
        AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 3 as c_int,
            u: C2RustUnnamed {
                mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_LEFT as c_int
                    | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT as c_int
                    | (1 as c_ulonglong) << AV_CHAN_FRONT_CENTER as c_int)
                    as c_ulong,
            },
            opaque: 0 as *const c_void as *mut c_void,
        }
    },
    {
        AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 4 as c_int,
            u: C2RustUnnamed {
                mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_LEFT as c_int
                    | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT as c_int
                    | (1 as c_ulonglong) << AV_CHAN_FRONT_CENTER as c_int
                    | (1 as c_ulonglong) << AV_CHAN_BACK_CENTER as c_int)
                    as c_ulong,
            },
            opaque: 0 as *const c_void as *mut c_void,
        }
    },
    {
        AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 5 as c_int,
            u: C2RustUnnamed {
                mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_LEFT as c_int
                    | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT as c_int
                    | (1 as c_ulonglong) << AV_CHAN_FRONT_CENTER as c_int
                    | (1 as c_ulonglong) << AV_CHAN_BACK_LEFT as c_int
                    | (1 as c_ulonglong) << AV_CHAN_BACK_RIGHT as c_int)
                    as c_ulong,
            },
            opaque: 0 as *const c_void as *mut c_void,
        }
    },
    {
        AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 6 as c_int,
            u: C2RustUnnamed {
                mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_LEFT as c_int
                    | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT as c_int
                    | (1 as c_ulonglong) << AV_CHAN_FRONT_CENTER as c_int
                    | (1 as c_ulonglong) << AV_CHAN_BACK_LEFT as c_int
                    | (1 as c_ulonglong) << AV_CHAN_BACK_RIGHT as c_int
                    | (1 as c_ulonglong) << AV_CHAN_LOW_FREQUENCY as c_int)
                    as c_ulong,
            },
            opaque: 0 as *const c_void as *mut c_void,
        }
    },
    {
        AVChannelLayout {
            order: AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 8 as c_int,
            u: C2RustUnnamed {
                mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_LEFT as c_int
                    | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT as c_int
                    | (1 as c_ulonglong) << AV_CHAN_FRONT_CENTER as c_int
                    | (1 as c_ulonglong) << AV_CHAN_SIDE_LEFT as c_int
                    | (1 as c_ulonglong) << AV_CHAN_SIDE_RIGHT as c_int
                    | (1 as c_ulonglong) << AV_CHAN_LOW_FREQUENCY as c_int
                    | (1 as c_ulonglong) << AV_CHAN_BACK_LEFT as c_int
                    | (1 as c_ulonglong) << AV_CHAN_BACK_RIGHT as c_int)
                    as c_ulong,
            },
            opaque: 0 as *const c_void as *mut c_void,
        }
    },
];
static mut aac_chan_configs: [[c_uchar; 6]; 16] = [
    [
        1 as c_int as c_uchar,
        TYPE_SCE as c_int as c_uchar,
        0,
        0,
        0,
        0,
    ],
    [
        1 as c_int as c_uchar,
        TYPE_CPE as c_int as c_uchar,
        0,
        0,
        0,
        0,
    ],
    [
        2 as c_int as c_uchar,
        TYPE_SCE as c_int as c_uchar,
        TYPE_CPE as c_int as c_uchar,
        0,
        0,
        0,
    ],
    [
        3 as c_int as c_uchar,
        TYPE_SCE as c_int as c_uchar,
        TYPE_CPE as c_int as c_uchar,
        TYPE_SCE as c_int as c_uchar,
        0,
        0,
    ],
    [
        3 as c_int as c_uchar,
        TYPE_SCE as c_int as c_uchar,
        TYPE_CPE as c_int as c_uchar,
        TYPE_CPE as c_int as c_uchar,
        0,
        0,
    ],
    [
        4 as c_int as c_uchar,
        TYPE_SCE as c_int as c_uchar,
        TYPE_CPE as c_int as c_uchar,
        TYPE_CPE as c_int as c_uchar,
        TYPE_LFE as c_int as c_uchar,
        0,
    ],
    [0 as c_int as c_uchar, 0, 0, 0, 0, 0],
    [
        5 as c_int as c_uchar,
        TYPE_SCE as c_int as c_uchar,
        TYPE_CPE as c_int as c_uchar,
        TYPE_CPE as c_int as c_uchar,
        TYPE_CPE as c_int as c_uchar,
        TYPE_LFE as c_int as c_uchar,
    ],
    [0; 6],
    [0; 6],
    [0; 6],
    [0; 6],
    [0; 6],
    [0; 6],
    [0; 6],
    [0; 6],
];
static mut aac_chan_maps: [[c_uchar; 16]; 16] = [
    [
        0 as c_int as c_uchar,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    [
        0 as c_int as c_uchar,
        1 as c_int as c_uchar,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    [
        2 as c_int as c_uchar,
        0 as c_int as c_uchar,
        1 as c_int as c_uchar,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    [
        2 as c_int as c_uchar,
        0 as c_int as c_uchar,
        1 as c_int as c_uchar,
        3 as c_int as c_uchar,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    [
        2 as c_int as c_uchar,
        0 as c_int as c_uchar,
        1 as c_int as c_uchar,
        3 as c_int as c_uchar,
        4 as c_int as c_uchar,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    [
        2 as c_int as c_uchar,
        0 as c_int as c_uchar,
        1 as c_int as c_uchar,
        4 as c_int as c_uchar,
        5 as c_int as c_uchar,
        3 as c_int as c_uchar,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    [
        0 as c_int as c_uchar,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    [
        2 as c_int as c_uchar,
        0 as c_int as c_uchar,
        1 as c_int as c_uchar,
        6 as c_int as c_uchar,
        7 as c_int as c_uchar,
        4 as c_int as c_uchar,
        5 as c_int as c_uchar,
        3 as c_int as c_uchar,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    [0; 16],
    [0; 16],
    [0; 16],
    [0; 16],
    [0; 16],
    [0; 16],
    [0; 16],
    [0; 16],
];
static mut aacenc_profiles: [c_int; 4] = [0 as c_int, 1 as c_int, 3 as c_int, 128 as c_int];
#[inline]
unsafe extern "C" fn abs_pow34_v(mut out: *mut c_float, mut in_0: *const c_float, size: c_int) {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < size {
        let mut a: c_float = fabsf(*in_0.offset(i as isize));
        *out.offset(i as isize) = sqrtf(a * sqrtf(a));
        i += 1;
        i;
    }
}
#[inline]
unsafe extern "C" fn quantize_bands(
    mut out: *mut c_int,
    mut in_0: *const c_float,
    mut scaled: *const c_float,
    mut size: c_int,
    mut is_signed: c_int,
    mut maxval: c_int,
    Q34: c_float,
    rounding: c_float,
) {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < size {
        let mut qc: c_float = *scaled.offset(i as isize) * Q34;
        let mut tmp: c_int = (if qc + rounding > maxval as c_float {
            maxval as c_float
        } else {
            qc + rounding
        }) as c_int;
        if is_signed != 0 && *in_0.offset(i as isize) < 0.0f32 {
            tmp = -tmp;
        }
        *out.offset(i as isize) = tmp;
        i += 1;
        i;
    }
}
static mut aac_pce_configs: [AACPCEInfo; 29] = [
    {
        AACPCEInfo {
            layout: {
                AVChannelLayout {
                    order: AV_CHANNEL_ORDER_NATIVE,
                    nb_channels: 1 as c_int,
                    u: C2RustUnnamed {
                        mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_CENTER as c_int) as c_ulong,
                    },
                    opaque: 0 as *const c_void as *mut c_void,
                }
            },
            num_ele: [1 as c_int, 0 as c_int, 0 as c_int, 0 as c_int],
            pairing: [[0 as c_int, 0, 0, 0, 0, 0, 0, 0], [0; 8], [0; 8]],
            index: [[0 as c_int, 0, 0, 0, 0, 0, 0, 0], [0; 8], [0; 8], [0; 8]],
            config_map: [
                1 as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            reorder_map: [
                0 as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        }
    },
    {
        AACPCEInfo {
            layout: {
                AVChannelLayout {
                    order: AV_CHANNEL_ORDER_NATIVE,
                    nb_channels: 2 as c_int,
                    u: C2RustUnnamed {
                        mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT as c_int)
                            as c_ulong,
                    },
                    opaque: 0 as *const c_void as *mut c_void,
                }
            },
            num_ele: [1 as c_int, 0 as c_int, 0 as c_int, 0 as c_int],
            pairing: [[1 as c_int, 0, 0, 0, 0, 0, 0, 0], [0; 8], [0; 8]],
            index: [[0 as c_int, 0, 0, 0, 0, 0, 0, 0], [0; 8], [0; 8], [0; 8]],
            config_map: [
                1 as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            reorder_map: [
                0 as c_int as c_uchar,
                1 as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        }
    },
    {
        AACPCEInfo {
            layout: {
                AVChannelLayout {
                    order: AV_CHANNEL_ORDER_NATIVE,
                    nb_channels: 3 as c_int,
                    u: C2RustUnnamed {
                        mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_LOW_FREQUENCY as c_int)
                            as c_ulong,
                    },
                    opaque: 0 as *const c_void as *mut c_void,
                }
            },
            num_ele: [1 as c_int, 0 as c_int, 0 as c_int, 1 as c_int],
            pairing: [[1 as c_int, 0, 0, 0, 0, 0, 0, 0], [0; 8], [0; 8]],
            index: [
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
            ],
            config_map: [
                2 as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_LFE as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            reorder_map: [
                0 as c_int as c_uchar,
                1 as c_int as c_uchar,
                2 as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        }
    },
    {
        AACPCEInfo {
            layout: {
                AVChannelLayout {
                    order: AV_CHANNEL_ORDER_NATIVE,
                    nb_channels: 3 as c_int,
                    u: C2RustUnnamed {
                        mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_CENTER as c_int)
                            as c_ulong,
                    },
                    opaque: 0 as *const c_void as *mut c_void,
                }
            },
            num_ele: [1 as c_int, 0 as c_int, 1 as c_int, 0 as c_int],
            pairing: [
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
            ],
            index: [
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [0; 8],
            ],
            config_map: [
                2 as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            reorder_map: [
                0 as c_int as c_uchar,
                1 as c_int as c_uchar,
                2 as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        }
    },
    {
        AACPCEInfo {
            layout: {
                AVChannelLayout {
                    order: AV_CHANNEL_ORDER_NATIVE,
                    nb_channels: 3 as c_int,
                    u: C2RustUnnamed {
                        mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_CENTER as c_int)
                            as c_ulong,
                    },
                    opaque: 0 as *const c_void as *mut c_void,
                }
            },
            num_ele: [2 as c_int, 0 as c_int, 0 as c_int, 0 as c_int],
            pairing: [[1 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0], [0; 8], [0; 8]],
            index: [
                [0 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [0; 8],
                [0; 8],
                [0; 8],
            ],
            config_map: [
                2 as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            reorder_map: [
                0 as c_int as c_uchar,
                1 as c_int as c_uchar,
                2 as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        }
    },
    {
        AACPCEInfo {
            layout: {
                AVChannelLayout {
                    order: AV_CHANNEL_ORDER_NATIVE,
                    nb_channels: 4 as c_int,
                    u: C2RustUnnamed {
                        mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_CENTER as c_int
                            | (1 as c_ulonglong) << AV_CHAN_LOW_FREQUENCY as c_int)
                            as c_ulong,
                    },
                    opaque: 0 as *const c_void as *mut c_void,
                }
            },
            num_ele: [2 as c_int, 0 as c_int, 0 as c_int, 1 as c_int],
            pairing: [[1 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0], [0; 8], [0; 8]],
            index: [
                [0 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
            ],
            config_map: [
                3 as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                TYPE_LFE as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            reorder_map: [
                0 as c_int as c_uchar,
                1 as c_int as c_uchar,
                2 as c_int as c_uchar,
                3 as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        }
    },
    {
        AACPCEInfo {
            layout: {
                AVChannelLayout {
                    order: AV_CHANNEL_ORDER_NATIVE,
                    nb_channels: 4 as c_int,
                    u: C2RustUnnamed {
                        mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_CENTER as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_CENTER as c_int)
                            as c_ulong,
                    },
                    opaque: 0 as *const c_void as *mut c_void,
                }
            },
            num_ele: [2 as c_int, 0 as c_int, 1 as c_int, 0 as c_int],
            pairing: [
                [1 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
            ],
            index: [
                [0 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [0; 8],
            ],
            config_map: [
                3 as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            reorder_map: [
                0 as c_int as c_uchar,
                1 as c_int as c_uchar,
                2 as c_int as c_uchar,
                3 as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        }
    },
    {
        AACPCEInfo {
            layout: {
                AVChannelLayout {
                    order: AV_CHANNEL_ORDER_NATIVE,
                    nb_channels: 5 as c_int,
                    u: C2RustUnnamed {
                        mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_CENTER as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_CENTER as c_int
                            | (1 as c_ulonglong) << AV_CHAN_LOW_FREQUENCY as c_int)
                            as c_ulong,
                    },
                    opaque: 0 as *const c_void as *mut c_void,
                }
            },
            num_ele: [2 as c_int, 1 as c_int, 1 as c_int, 0 as c_int],
            pairing: [
                [1 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
            ],
            index: [
                [0 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [2 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
            ],
            config_map: [
                4 as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            reorder_map: [
                0 as c_int as c_uchar,
                1 as c_int as c_uchar,
                2 as c_int as c_uchar,
                3 as c_int as c_uchar,
                4 as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        }
    },
    {
        AACPCEInfo {
            layout: {
                AVChannelLayout {
                    order: AV_CHANNEL_ORDER_NATIVE,
                    nb_channels: 4 as c_int,
                    u: C2RustUnnamed {
                        mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_SIDE_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_SIDE_RIGHT as c_int)
                            as c_ulong,
                    },
                    opaque: 0 as *const c_void as *mut c_void,
                }
            },
            num_ele: [1 as c_int, 1 as c_int, 0 as c_int, 0 as c_int],
            pairing: [
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [0; 8],
            ],
            index: [
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [0; 8],
                [0; 8],
            ],
            config_map: [
                2 as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            reorder_map: [
                0 as c_int as c_uchar,
                1 as c_int as c_uchar,
                2 as c_int as c_uchar,
                3 as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        }
    },
    {
        AACPCEInfo {
            layout: {
                AVChannelLayout {
                    order: AV_CHANNEL_ORDER_NATIVE,
                    nb_channels: 4 as c_int,
                    u: C2RustUnnamed {
                        mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_RIGHT as c_int)
                            as c_ulong,
                    },
                    opaque: 0 as *const c_void as *mut c_void,
                }
            },
            num_ele: [1 as c_int, 0 as c_int, 1 as c_int, 0 as c_int],
            pairing: [
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
            ],
            index: [
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [0; 8],
            ],
            config_map: [
                2 as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            reorder_map: [
                0 as c_int as c_uchar,
                1 as c_int as c_uchar,
                2 as c_int as c_uchar,
                3 as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        }
    },
    {
        AACPCEInfo {
            layout: {
                AVChannelLayout {
                    order: AV_CHANNEL_ORDER_NATIVE,
                    nb_channels: 5 as c_int,
                    u: C2RustUnnamed {
                        mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_CENTER as c_int
                            | (1 as c_ulonglong) << AV_CHAN_SIDE_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_SIDE_RIGHT as c_int)
                            as c_ulong,
                    },
                    opaque: 0 as *const c_void as *mut c_void,
                }
            },
            num_ele: [2 as c_int, 1 as c_int, 0 as c_int, 0 as c_int],
            pairing: [
                [1 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [0; 8],
            ],
            index: [
                [0 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [0; 8],
                [0; 8],
            ],
            config_map: [
                3 as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            reorder_map: [
                0 as c_int as c_uchar,
                1 as c_int as c_uchar,
                2 as c_int as c_uchar,
                3 as c_int as c_uchar,
                4 as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        }
    },
    {
        AACPCEInfo {
            layout: {
                AVChannelLayout {
                    order: AV_CHANNEL_ORDER_NATIVE,
                    nb_channels: 6 as c_int,
                    u: C2RustUnnamed {
                        mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_CENTER as c_int
                            | (1 as c_ulonglong) << AV_CHAN_SIDE_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_SIDE_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_LOW_FREQUENCY as c_int)
                            as c_ulong,
                    },
                    opaque: 0 as *const c_void as *mut c_void,
                }
            },
            num_ele: [2 as c_int, 1 as c_int, 1 as c_int, 0 as c_int],
            pairing: [
                [1 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
            ],
            index: [
                [0 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [0; 8],
            ],
            config_map: [
                4 as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            reorder_map: [
                0 as c_int as c_uchar,
                1 as c_int as c_uchar,
                2 as c_int as c_uchar,
                3 as c_int as c_uchar,
                4 as c_int as c_uchar,
                5 as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        }
    },
    {
        AACPCEInfo {
            layout: {
                AVChannelLayout {
                    order: AV_CHANNEL_ORDER_NATIVE,
                    nb_channels: 5 as c_int,
                    u: C2RustUnnamed {
                        mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_CENTER as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_RIGHT as c_int)
                            as c_ulong,
                    },
                    opaque: 0 as *const c_void as *mut c_void,
                }
            },
            num_ele: [2 as c_int, 0 as c_int, 1 as c_int, 0 as c_int],
            pairing: [
                [1 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
            ],
            index: [
                [0 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [0; 8],
            ],
            config_map: [
                3 as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            reorder_map: [
                0 as c_int as c_uchar,
                1 as c_int as c_uchar,
                2 as c_int as c_uchar,
                3 as c_int as c_uchar,
                4 as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        }
    },
    {
        AACPCEInfo {
            layout: {
                AVChannelLayout {
                    order: AV_CHANNEL_ORDER_NATIVE,
                    nb_channels: 6 as c_int,
                    u: C2RustUnnamed {
                        mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_CENTER as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_LOW_FREQUENCY as c_int)
                            as c_ulong,
                    },
                    opaque: 0 as *const c_void as *mut c_void,
                }
            },
            num_ele: [2 as c_int, 1 as c_int, 1 as c_int, 0 as c_int],
            pairing: [
                [1 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
            ],
            index: [
                [0 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [0; 8],
            ],
            config_map: [
                4 as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            reorder_map: [
                0 as c_int as c_uchar,
                1 as c_int as c_uchar,
                2 as c_int as c_uchar,
                3 as c_int as c_uchar,
                4 as c_int as c_uchar,
                5 as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        }
    },
    {
        AACPCEInfo {
            layout: {
                AVChannelLayout {
                    order: AV_CHANNEL_ORDER_NATIVE,
                    nb_channels: 6 as c_int,
                    u: C2RustUnnamed {
                        mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_CENTER as c_int
                            | (1 as c_ulonglong) << AV_CHAN_SIDE_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_SIDE_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_CENTER as c_int)
                            as c_ulong,
                    },
                    opaque: 0 as *const c_void as *mut c_void,
                }
            },
            num_ele: [2 as c_int, 1 as c_int, 1 as c_int, 0 as c_int],
            pairing: [
                [1 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
            ],
            index: [
                [0 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [0; 8],
            ],
            config_map: [
                4 as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            reorder_map: [
                0 as c_int as c_uchar,
                1 as c_int as c_uchar,
                2 as c_int as c_uchar,
                3 as c_int as c_uchar,
                4 as c_int as c_uchar,
                5 as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        }
    },
    {
        AACPCEInfo {
            layout: {
                AVChannelLayout {
                    order: AV_CHANNEL_ORDER_NATIVE,
                    nb_channels: 6 as c_int,
                    u: C2RustUnnamed {
                        mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_SIDE_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_SIDE_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_LEFT_OF_CENTER as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT_OF_CENTER as c_int)
                            as c_ulong,
                    },
                    opaque: 0 as *const c_void as *mut c_void,
                }
            },
            num_ele: [2 as c_int, 1 as c_int, 0 as c_int, 0 as c_int],
            pairing: [
                [1 as c_int, 1 as c_int, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [0; 8],
            ],
            index: [
                [1 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [2 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [0; 8],
                [0; 8],
            ],
            config_map: [
                3 as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            reorder_map: [
                0 as c_int as c_uchar,
                1 as c_int as c_uchar,
                2 as c_int as c_uchar,
                3 as c_int as c_uchar,
                4 as c_int as c_uchar,
                5 as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        }
    },
    {
        AACPCEInfo {
            layout: {
                AVChannelLayout {
                    order: AV_CHANNEL_ORDER_NATIVE,
                    nb_channels: 6 as c_int,
                    u: C2RustUnnamed {
                        mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_CENTER as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_CENTER as c_int)
                            as c_ulong,
                    },
                    opaque: 0 as *const c_void as *mut c_void,
                }
            },
            num_ele: [2 as c_int, 0 as c_int, 2 as c_int, 0 as c_int],
            pairing: [
                [1 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
            ],
            index: [
                [0 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 1 as c_int, 0, 0, 0, 0, 0, 0],
                [0; 8],
            ],
            config_map: [
                4 as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            reorder_map: [
                0 as c_int as c_uchar,
                1 as c_int as c_uchar,
                2 as c_int as c_uchar,
                3 as c_int as c_uchar,
                4 as c_int as c_uchar,
                5 as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        }
    },
    {
        AACPCEInfo {
            layout: {
                AVChannelLayout {
                    order: AV_CHANNEL_ORDER_NATIVE,
                    nb_channels: 7 as c_int,
                    u: C2RustUnnamed {
                        mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_CENTER as c_int
                            | (1 as c_ulonglong) << AV_CHAN_SIDE_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_SIDE_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_LOW_FREQUENCY as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_CENTER as c_int)
                            as c_ulong,
                    },
                    opaque: 0 as *const c_void as *mut c_void,
                }
            },
            num_ele: [2 as c_int, 1 as c_int, 2 as c_int, 0 as c_int],
            pairing: [
                [1 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
            ],
            index: [
                [0 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 2 as c_int, 0, 0, 0, 0, 0, 0],
                [0; 8],
            ],
            config_map: [
                5 as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            reorder_map: [
                0 as c_int as c_uchar,
                1 as c_int as c_uchar,
                2 as c_int as c_uchar,
                3 as c_int as c_uchar,
                4 as c_int as c_uchar,
                5 as c_int as c_uchar,
                6 as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        }
    },
    {
        AACPCEInfo {
            layout: {
                AVChannelLayout {
                    order: AV_CHANNEL_ORDER_NATIVE,
                    nb_channels: 7 as c_int,
                    u: C2RustUnnamed {
                        mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_CENTER as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_LOW_FREQUENCY as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_CENTER as c_int)
                            as c_ulong,
                    },
                    opaque: 0 as *const c_void as *mut c_void,
                }
            },
            num_ele: [2 as c_int, 1 as c_int, 2 as c_int, 0 as c_int],
            pairing: [
                [1 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
            ],
            index: [
                [0 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 2 as c_int, 0, 0, 0, 0, 0, 0],
                [0; 8],
            ],
            config_map: [
                5 as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            reorder_map: [
                0 as c_int as c_uchar,
                1 as c_int as c_uchar,
                2 as c_int as c_uchar,
                3 as c_int as c_uchar,
                4 as c_int as c_uchar,
                5 as c_int as c_uchar,
                6 as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        }
    },
    {
        AACPCEInfo {
            layout: {
                AVChannelLayout {
                    order: AV_CHANNEL_ORDER_NATIVE,
                    nb_channels: 7 as c_int,
                    u: C2RustUnnamed {
                        mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_SIDE_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_SIDE_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_LEFT_OF_CENTER as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT_OF_CENTER as c_int
                            | (1 as c_ulonglong) << AV_CHAN_LOW_FREQUENCY as c_int)
                            as c_ulong,
                    },
                    opaque: 0 as *const c_void as *mut c_void,
                }
            },
            num_ele: [2 as c_int, 1 as c_int, 2 as c_int, 0 as c_int],
            pairing: [
                [1 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
            ],
            index: [
                [0 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 2 as c_int, 0, 0, 0, 0, 0, 0],
                [0; 8],
            ],
            config_map: [
                5 as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            reorder_map: [
                0 as c_int as c_uchar,
                1 as c_int as c_uchar,
                2 as c_int as c_uchar,
                3 as c_int as c_uchar,
                4 as c_int as c_uchar,
                5 as c_int as c_uchar,
                6 as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        }
    },
    {
        AACPCEInfo {
            layout: {
                AVChannelLayout {
                    order: AV_CHANNEL_ORDER_NATIVE,
                    nb_channels: 7 as c_int,
                    u: C2RustUnnamed {
                        mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_CENTER as c_int
                            | (1 as c_ulonglong) << AV_CHAN_SIDE_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_SIDE_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_RIGHT as c_int)
                            as c_ulong,
                    },
                    opaque: 0 as *const c_void as *mut c_void,
                }
            },
            num_ele: [2 as c_int, 1 as c_int, 1 as c_int, 0 as c_int],
            pairing: [
                [1 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
            ],
            index: [
                [0 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [2 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [0; 8],
            ],
            config_map: [
                4 as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            reorder_map: [
                0 as c_int as c_uchar,
                1 as c_int as c_uchar,
                2 as c_int as c_uchar,
                3 as c_int as c_uchar,
                4 as c_int as c_uchar,
                5 as c_int as c_uchar,
                6 as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        }
    },
    {
        AACPCEInfo {
            layout: {
                AVChannelLayout {
                    order: AV_CHANNEL_ORDER_NATIVE,
                    nb_channels: 7 as c_int,
                    u: C2RustUnnamed {
                        mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_CENTER as c_int
                            | (1 as c_ulonglong) << AV_CHAN_SIDE_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_SIDE_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_LEFT_OF_CENTER as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT_OF_CENTER as c_int)
                            as c_ulong,
                    },
                    opaque: 0 as *const c_void as *mut c_void,
                }
            },
            num_ele: [2 as c_int, 1 as c_int, 1 as c_int, 0 as c_int],
            pairing: [
                [1 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
            ],
            index: [
                [0 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [2 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [0; 8],
            ],
            config_map: [
                4 as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            reorder_map: [
                0 as c_int as c_uchar,
                1 as c_int as c_uchar,
                2 as c_int as c_uchar,
                3 as c_int as c_uchar,
                4 as c_int as c_uchar,
                5 as c_int as c_uchar,
                6 as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        }
    },
    {
        AACPCEInfo {
            layout: {
                AVChannelLayout {
                    order: AV_CHANNEL_ORDER_NATIVE,
                    nb_channels: 8 as c_int,
                    u: C2RustUnnamed {
                        mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_CENTER as c_int
                            | (1 as c_ulonglong) << AV_CHAN_SIDE_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_SIDE_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_LOW_FREQUENCY as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_RIGHT as c_int)
                            as c_ulong,
                    },
                    opaque: 0 as *const c_void as *mut c_void,
                }
            },
            num_ele: [2 as c_int, 1 as c_int, 2 as c_int, 0 as c_int],
            pairing: [
                [1 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 1 as c_int, 0, 0, 0, 0, 0, 0],
            ],
            index: [
                [0 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 2 as c_int, 0, 0, 0, 0, 0, 0],
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
            ],
            config_map: [
                5 as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            reorder_map: [
                0 as c_int as c_uchar,
                1 as c_int as c_uchar,
                2 as c_int as c_uchar,
                3 as c_int as c_uchar,
                4 as c_int as c_uchar,
                5 as c_int as c_uchar,
                6 as c_int as c_uchar,
                7 as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        }
    },
    {
        AACPCEInfo {
            layout: {
                AVChannelLayout {
                    order: AV_CHANNEL_ORDER_NATIVE,
                    nb_channels: 8 as c_int,
                    u: C2RustUnnamed {
                        mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_CENTER as c_int
                            | (1 as c_ulonglong) << AV_CHAN_SIDE_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_SIDE_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_LOW_FREQUENCY as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_LEFT_OF_CENTER as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT_OF_CENTER as c_int)
                            as c_ulong,
                    },
                    opaque: 0 as *const c_void as *mut c_void,
                }
            },
            num_ele: [2 as c_int, 1 as c_int, 2 as c_int, 0 as c_int],
            pairing: [
                [1 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 1 as c_int, 0, 0, 0, 0, 0, 0],
            ],
            index: [
                [0 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 2 as c_int, 0, 0, 0, 0, 0, 0],
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
            ],
            config_map: [
                5 as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            reorder_map: [
                0 as c_int as c_uchar,
                1 as c_int as c_uchar,
                2 as c_int as c_uchar,
                3 as c_int as c_uchar,
                4 as c_int as c_uchar,
                5 as c_int as c_uchar,
                6 as c_int as c_uchar,
                7 as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        }
    },
    {
        AACPCEInfo {
            layout: {
                AVChannelLayout {
                    order: AV_CHANNEL_ORDER_NATIVE,
                    nb_channels: 8 as c_int,
                    u: C2RustUnnamed {
                        mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_CENTER as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_LOW_FREQUENCY as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_LEFT_OF_CENTER as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT_OF_CENTER as c_int)
                            as c_ulong,
                    },
                    opaque: 0 as *const c_void as *mut c_void,
                }
            },
            num_ele: [2 as c_int, 1 as c_int, 2 as c_int, 0 as c_int],
            pairing: [
                [1 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 1 as c_int, 0, 0, 0, 0, 0, 0],
            ],
            index: [
                [0 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 2 as c_int, 0, 0, 0, 0, 0, 0],
                [0 as c_int, 0, 0, 0, 0, 0, 0, 0],
            ],
            config_map: [
                5 as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            reorder_map: [
                0 as c_int as c_uchar,
                1 as c_int as c_uchar,
                2 as c_int as c_uchar,
                3 as c_int as c_uchar,
                4 as c_int as c_uchar,
                5 as c_int as c_uchar,
                6 as c_int as c_uchar,
                7 as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        }
    },
    {
        AACPCEInfo {
            layout: {
                AVChannelLayout {
                    order: AV_CHANNEL_ORDER_NATIVE,
                    nb_channels: 8 as c_int,
                    u: C2RustUnnamed {
                        mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_CENTER as c_int
                            | (1 as c_ulonglong) << AV_CHAN_SIDE_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_SIDE_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_CENTER as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_RIGHT as c_int)
                            as c_ulong,
                    },
                    opaque: 0 as *const c_void as *mut c_void,
                }
            },
            num_ele: [2 as c_int, 1 as c_int, 2 as c_int, 0 as c_int],
            pairing: [
                [1 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
            ],
            index: [
                [0 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0, 0, 0, 0, 0, 0, 0],
                [2 as c_int, 1 as c_int, 0, 0, 0, 0, 0, 0],
                [0; 8],
            ],
            config_map: [
                5 as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            reorder_map: [
                0 as c_int as c_uchar,
                1 as c_int as c_uchar,
                2 as c_int as c_uchar,
                3 as c_int as c_uchar,
                4 as c_int as c_uchar,
                5 as c_int as c_uchar,
                6 as c_int as c_uchar,
                7 as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        }
    },
    {
        AACPCEInfo {
            layout: {
                AVChannelLayout {
                    order: AV_CHANNEL_ORDER_NATIVE,
                    nb_channels: 9 as c_int,
                    u: C2RustUnnamed {
                        mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_CENTER as c_int
                            | (1 as c_ulonglong) << AV_CHAN_SIDE_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_SIDE_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_CENTER as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_TOP_CENTER as c_int)
                            as c_ulong,
                    },
                    opaque: 0 as *const c_void as *mut c_void,
                }
            },
            num_ele: [2 as c_int, 2 as c_int, 2 as c_int, 0 as c_int],
            pairing: [
                [1 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
            ],
            index: [
                [0 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 1 as c_int, 0, 0, 0, 0, 0, 0],
                [2 as c_int, 2 as c_int, 0, 0, 0, 0, 0, 0],
                [0; 8],
            ],
            config_map: [
                6 as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            reorder_map: [
                0 as c_int as c_uchar,
                1 as c_int as c_uchar,
                2 as c_int as c_uchar,
                3 as c_int as c_uchar,
                4 as c_int as c_uchar,
                5 as c_int as c_uchar,
                6 as c_int as c_uchar,
                7 as c_int as c_uchar,
                8 as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        }
    },
    {
        AACPCEInfo {
            layout: {
                AVChannelLayout {
                    order: AV_CHANNEL_ORDER_NATIVE,
                    nb_channels: 10 as c_int,
                    u: C2RustUnnamed {
                        mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_SIDE_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_SIDE_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_LEFT_OF_CENTER as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT_OF_CENTER as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_CENTER as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_TOP_CENTER as c_int)
                            as c_ulong,
                    },
                    opaque: 0 as *const c_void as *mut c_void,
                }
            },
            num_ele: [2 as c_int, 2 as c_int, 2 as c_int, 0 as c_int],
            pairing: [
                [1 as c_int, 1 as c_int, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
            ],
            index: [
                [0 as c_int, 1 as c_int, 0, 0, 0, 0, 0, 0],
                [2 as c_int, 0 as c_int, 0, 0, 0, 0, 0, 0],
                [3 as c_int, 1 as c_int, 0, 0, 0, 0, 0, 0],
                [0; 8],
            ],
            config_map: [
                6 as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            reorder_map: [
                0 as c_int as c_uchar,
                1 as c_int as c_uchar,
                2 as c_int as c_uchar,
                3 as c_int as c_uchar,
                4 as c_int as c_uchar,
                5 as c_int as c_uchar,
                6 as c_int as c_uchar,
                7 as c_int as c_uchar,
                8 as c_int as c_uchar,
                9 as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        }
    },
    {
        AACPCEInfo {
            layout: {
                AVChannelLayout {
                    order: AV_CHANNEL_ORDER_NATIVE,
                    nb_channels: 16 as c_int,
                    u: C2RustUnnamed {
                        mask: ((1 as c_ulonglong) << AV_CHAN_FRONT_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_FRONT_CENTER as c_int
                            | (1 as c_ulonglong) << AV_CHAN_SIDE_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_SIDE_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_CENTER as c_int
                            | (1 as c_ulonglong) << AV_CHAN_BACK_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_WIDE_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_WIDE_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_TOP_BACK_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_TOP_BACK_RIGHT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_TOP_BACK_CENTER as c_int
                            | (1 as c_ulonglong) << AV_CHAN_TOP_FRONT_CENTER as c_int
                            | (1 as c_ulonglong) << AV_CHAN_TOP_FRONT_LEFT as c_int
                            | (1 as c_ulonglong) << AV_CHAN_TOP_FRONT_RIGHT as c_int)
                            as c_ulong,
                    },
                    opaque: 0 as *const c_void as *mut c_void,
                }
            },
            num_ele: [4 as c_int, 2 as c_int, 4 as c_int, 0 as c_int],
            pairing: [
                [1 as c_int, 0 as c_int, 1 as c_int, 0 as c_int, 0, 0, 0, 0],
                [1 as c_int, 1 as c_int, 0, 0, 0, 0, 0, 0],
                [1 as c_int, 0 as c_int, 1 as c_int, 0 as c_int, 0, 0, 0, 0],
            ],
            index: [
                [0 as c_int, 0 as c_int, 1 as c_int, 1 as c_int, 0, 0, 0, 0],
                [2 as c_int, 3 as c_int, 0, 0, 0, 0, 0, 0],
                [4 as c_int, 2 as c_int, 5 as c_int, 3 as c_int, 0, 0, 0, 0],
                [0; 8],
            ],
            config_map: [
                10 as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                TYPE_CPE as c_int as c_uchar,
                TYPE_SCE as c_int as c_uchar,
                0,
                0,
                0,
                0,
                0,
            ],
            reorder_map: [
                0 as c_int as c_uchar,
                1 as c_int as c_uchar,
                2 as c_int as c_uchar,
                3 as c_int as c_uchar,
                4 as c_int as c_uchar,
                5 as c_int as c_uchar,
                6 as c_int as c_uchar,
                7 as c_int as c_uchar,
                8 as c_int as c_uchar,
                9 as c_int as c_uchar,
                10 as c_int as c_uchar,
                11 as c_int as c_uchar,
                12 as c_int as c_uchar,
                13 as c_int as c_uchar,
                14 as c_int as c_uchar,
                15 as c_int as c_uchar,
            ],
        }
    },
];
unsafe extern "C" fn put_pce(mut pb: *mut PutBitContext, mut avctx: *mut AVCodecContext) {
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut s: *mut AACEncContext = (*avctx).priv_data as *mut AACEncContext;
    let mut pce: *mut AACPCEInfo = &mut (*s).pce;
    let bitexact: c_int = (*avctx).flags & (1 as c_int) << 23 as c_int;
    let mut aux_data = if bitexact != 0 {
        c"Lavc"
    } else {
        c"Lavc60.33.100"
    };
    put_bits(pb, 4 as c_int, 0 as c_int as BitBuf);
    put_bits(pb, 2 as c_int, (*avctx).profile as BitBuf);
    put_bits(pb, 4 as c_int, (*s).samplerate_index as BitBuf);
    put_bits(
        pb,
        4 as c_int,
        (*pce).num_ele[0 as c_int as usize] as BitBuf,
    );
    put_bits(
        pb,
        4 as c_int,
        (*pce).num_ele[1 as c_int as usize] as BitBuf,
    );
    put_bits(
        pb,
        4 as c_int,
        (*pce).num_ele[2 as c_int as usize] as BitBuf,
    );
    put_bits(
        pb,
        2 as c_int,
        (*pce).num_ele[3 as c_int as usize] as BitBuf,
    );
    put_bits(pb, 3 as c_int, 0 as c_int as BitBuf);
    put_bits(pb, 4 as c_int, 0 as c_int as BitBuf);
    put_bits(pb, 1 as c_int, 0 as c_int as BitBuf);
    put_bits(pb, 1 as c_int, 0 as c_int as BitBuf);
    put_bits(pb, 1 as c_int, 0 as c_int as BitBuf);
    i = 0 as c_int;
    while i < 4 as c_int {
        j = 0 as c_int;
        while j < (*pce).num_ele[i as usize] {
            if i < 3 as c_int {
                put_bits(
                    pb,
                    1 as c_int,
                    (*pce).pairing[i as usize][j as usize] as BitBuf,
                );
            }
            put_bits(
                pb,
                4 as c_int,
                (*pce).index[i as usize][j as usize] as BitBuf,
            );
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    align_put_bits(pb);
    put_bits(pb, 8 as c_int, aux_data.to_bytes().len() as BitBuf);
    for c in aux_data.to_bytes() {
        put_bits(pb, 8, *c as u32);
    }
}
unsafe extern "C" fn put_audio_specific_config(mut avctx: *mut AVCodecContext) -> c_int {
    let mut pb: PutBitContext = PutBitContext {
        bit_buf: 0,
        bit_left: 0,
        buf: ptr::null_mut::<c_uchar>(),
        buf_ptr: ptr::null_mut::<c_uchar>(),
        buf_end: ptr::null_mut::<c_uchar>(),
    };
    let mut s: *mut AACEncContext = (*avctx).priv_data as *mut AACEncContext;
    let mut channels: c_int = ((*s).needs_pce == 0) as c_int
        * ((*s).channels
            - (if (*s).channels == 8 as c_int {
                1 as c_int
            } else {
                0 as c_int
            }));
    let max_size: c_int = 32 as c_int;
    (*avctx).extradata = av_mallocz(max_size as c_ulong) as *mut c_uchar;
    if ((*avctx).extradata).is_null() {
        return -(12 as c_int);
    }
    init_put_bits(&mut pb, (*avctx).extradata, max_size);
    put_bits(&mut pb, 5 as c_int, ((*s).profile + 1 as c_int) as BitBuf);
    put_bits(&mut pb, 4 as c_int, (*s).samplerate_index as BitBuf);
    put_bits(&mut pb, 4 as c_int, channels as BitBuf);
    put_bits(&mut pb, 1 as c_int, 0 as c_int as BitBuf);
    put_bits(&mut pb, 1 as c_int, 0 as c_int as BitBuf);
    put_bits(&mut pb, 1 as c_int, 0 as c_int as BitBuf);
    if (*s).needs_pce != 0 {
        put_pce(&mut pb, avctx);
    }
    put_bits(&mut pb, 11 as c_int, 0x2b7 as c_int as BitBuf);
    put_bits(&mut pb, 5 as c_int, AOT_SBR as c_int as BitBuf);
    put_bits(&mut pb, 1 as c_int, 0 as c_int as BitBuf);
    flush_put_bits(&mut pb);
    (*avctx).extradata_size = put_bytes_output(&mut pb);
    0 as c_int
}
#[no_mangle]
pub unsafe extern "C" fn ff_quantize_band_cost_cache_init(mut s: *mut AACEncContext) {
    (*s).quantize_band_cost_cache_generation =
        ((*s).quantize_band_cost_cache_generation).wrapping_add(1);
    (*s).quantize_band_cost_cache_generation;
    if (*s).quantize_band_cost_cache_generation as c_int == 0 as c_int {
        (*s).quantize_band_cost_cache = [[AACQuantizeBandCostCacheEntry::default(); 128]; 256];
        (*s).quantize_band_cost_cache_generation = 1 as c_int as c_ushort;
    }
}
unsafe extern "C" fn apply_only_long_window(
    mut fdsp: *mut AVFloatDSPContext,
    mut sce: *mut SingleChannelElement,
    mut audio: *const c_float,
) {
    let mut lwindow: *const c_float = if (*sce).ics.use_kb_window[0 as c_int as usize] as c_int != 0
    {
        ff_aac_kbd_long_1024.as_mut_ptr()
    } else {
        ff_sine_1024.as_mut_ptr()
    };
    let mut pwindow: *const c_float = if (*sce).ics.use_kb_window[1 as c_int as usize] as c_int != 0
    {
        ff_aac_kbd_long_1024.as_mut_ptr()
    } else {
        ff_sine_1024.as_mut_ptr()
    };
    let mut out: *mut c_float = ((*sce).ret_buf).as_mut_ptr();
    ((*fdsp).vector_fmul).expect("non-null function pointer")(out, audio, lwindow, 1024 as c_int);
    ((*fdsp).vector_fmul_reverse).expect("non-null function pointer")(
        out.offset(1024 as c_int as isize),
        audio.offset(1024 as c_int as isize),
        pwindow,
        1024 as c_int,
    );
}
unsafe extern "C" fn apply_long_start_window(
    mut fdsp: *mut AVFloatDSPContext,
    mut sce: *mut SingleChannelElement,
    mut audio: *const c_float,
) {
    let mut lwindow: *const c_float = if (*sce).ics.use_kb_window[1 as c_int as usize] as c_int != 0
    {
        ff_aac_kbd_long_1024.as_mut_ptr()
    } else {
        ff_sine_1024.as_mut_ptr()
    };
    let mut swindow: *const c_float = if (*sce).ics.use_kb_window[0 as c_int as usize] as c_int != 0
    {
        ff_aac_kbd_short_128.as_mut_ptr()
    } else {
        ff_sine_128.as_mut_ptr()
    };
    let mut out: *mut c_float = ((*sce).ret_buf).as_mut_ptr();
    ((*fdsp).vector_fmul).expect("non-null function pointer")(out, audio, lwindow, 1024 as c_int);
    ptr::copy_nonoverlapping(
        audio.offset(1024 as c_int as isize),
        out.offset(1024 as c_int as isize),
        448,
    );
    ((*fdsp).vector_fmul_reverse).expect("non-null function pointer")(
        out.offset(1024 as c_int as isize)
            .offset(448 as c_int as isize),
        audio
            .offset(1024 as c_int as isize)
            .offset(448 as c_int as isize),
        swindow,
        128 as c_int,
    );
    ptr::write_bytes(
        out.offset(1024 as c_int as isize)
            .offset(576 as c_int as isize),
        0,
        448,
    );
}
unsafe extern "C" fn apply_long_stop_window(
    mut fdsp: *mut AVFloatDSPContext,
    mut sce: *mut SingleChannelElement,
    mut audio: *const c_float,
) {
    let mut lwindow: *const c_float = if (*sce).ics.use_kb_window[0 as c_int as usize] as c_int != 0
    {
        ff_aac_kbd_long_1024.as_mut_ptr()
    } else {
        ff_sine_1024.as_mut_ptr()
    };
    let mut swindow: *const c_float = if (*sce).ics.use_kb_window[1 as c_int as usize] as c_int != 0
    {
        ff_aac_kbd_short_128.as_mut_ptr()
    } else {
        ff_sine_128.as_mut_ptr()
    };
    let mut out: *mut c_float = ((*sce).ret_buf).as_mut_ptr();
    ptr::write_bytes(out, 0, 448);
    ((*fdsp).vector_fmul).expect("non-null function pointer")(
        out.offset(448 as c_int as isize),
        audio.offset(448 as c_int as isize),
        swindow,
        128 as c_int,
    );
    ptr::copy_nonoverlapping(
        audio.offset(576 as c_int as isize),
        out.offset(576 as c_int as isize),
        448,
    );
    ((*fdsp).vector_fmul_reverse).expect("non-null function pointer")(
        out.offset(1024 as c_int as isize),
        audio.offset(1024 as c_int as isize),
        lwindow,
        1024 as c_int,
    );
}
unsafe extern "C" fn apply_eight_short_window(
    mut fdsp: *mut AVFloatDSPContext,
    mut sce: *mut SingleChannelElement,
    mut audio: *const c_float,
) {
    let mut swindow: *const c_float = if (*sce).ics.use_kb_window[0 as c_int as usize] as c_int != 0
    {
        ff_aac_kbd_short_128.as_mut_ptr()
    } else {
        ff_sine_128.as_mut_ptr()
    };
    let mut pwindow: *const c_float = if (*sce).ics.use_kb_window[1 as c_int as usize] as c_int != 0
    {
        ff_aac_kbd_short_128.as_mut_ptr()
    } else {
        ff_sine_128.as_mut_ptr()
    };
    let mut in_0: *const c_float = audio.offset(448 as c_int as isize);
    let mut out: *mut c_float = ((*sce).ret_buf).as_mut_ptr();
    let mut w: c_int = 0;
    w = 0 as c_int;
    while w < 8 as c_int {
        ((*fdsp).vector_fmul).expect("non-null function pointer")(
            out,
            in_0,
            if w != 0 { pwindow } else { swindow },
            128 as c_int,
        );
        out = out.offset(128 as c_int as isize);
        in_0 = in_0.offset(128 as c_int as isize);
        ((*fdsp).vector_fmul_reverse).expect("non-null function pointer")(
            out,
            in_0,
            swindow,
            128 as c_int,
        );
        out = out.offset(128 as c_int as isize);
        w += 1;
        w;
    }
}
static mut apply_window: [Option<
    unsafe extern "C" fn(*mut AVFloatDSPContext, *mut SingleChannelElement, *const c_float) -> (),
>; 4] = unsafe {
    [
        Some(
            apply_only_long_window
                as unsafe extern "C" fn(
                    *mut AVFloatDSPContext,
                    *mut SingleChannelElement,
                    *const c_float,
                ) -> (),
        ),
        Some(
            apply_long_start_window
                as unsafe extern "C" fn(
                    *mut AVFloatDSPContext,
                    *mut SingleChannelElement,
                    *const c_float,
                ) -> (),
        ),
        Some(
            apply_eight_short_window
                as unsafe extern "C" fn(
                    *mut AVFloatDSPContext,
                    *mut SingleChannelElement,
                    *const c_float,
                ) -> (),
        ),
        Some(
            apply_long_stop_window
                as unsafe extern "C" fn(
                    *mut AVFloatDSPContext,
                    *mut SingleChannelElement,
                    *const c_float,
                ) -> (),
        ),
    ]
};
unsafe extern "C" fn apply_window_and_mdct(
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
    mut audio: *mut c_float,
) {
    let mut i: c_int = 0;
    let mut output: *mut c_float = ((*sce).ret_buf).as_mut_ptr();
    (apply_window[(*sce).ics.window_sequence[0 as c_int as usize] as usize])
        .expect("non-null function pointer")((*s).fdsp, sce, audio);
    if (*sce).ics.window_sequence[0 as c_int as usize] as c_uint
        != EIGHT_SHORT_SEQUENCE as c_int as c_uint
    {
        ((*s).mdct1024_fn).expect("non-null function pointer")(
            (*s).mdct1024,
            ((*sce).coeffs).as_mut_ptr() as *mut c_void,
            output as *mut c_void,
            size_of::<c_float>() as c_ulong as ptrdiff_t,
        );
    } else {
        i = 0 as c_int;
        while i < 1024 as c_int {
            ((*s).mdct128_fn).expect("non-null function pointer")(
                (*s).mdct128,
                &mut *((*sce).coeffs).as_mut_ptr().offset(i as isize) as *mut c_float
                    as *mut c_void,
                output.offset((i * 2 as c_int) as isize) as *mut c_void,
                size_of::<c_float>() as c_ulong as ptrdiff_t,
            );
            i += 128 as c_int;
        }
    }
    ptr::copy_nonoverlapping(audio.offset(1024 as c_int as isize), audio, 1024);
    (*sce).pcoeffs = (*sce).coeffs;
}
unsafe extern "C" fn put_ics_info(
    mut s: *mut AACEncContext,
    mut info: *mut IndividualChannelStream,
) {
    let mut w: c_int = 0;
    put_bits(&mut (*s).pb, 1 as c_int, 0 as c_int as BitBuf);
    put_bits(
        &mut (*s).pb,
        2 as c_int,
        (*info).window_sequence[0 as c_int as usize] as BitBuf,
    );
    put_bits(
        &mut (*s).pb,
        1 as c_int,
        (*info).use_kb_window[0 as c_int as usize] as BitBuf,
    );
    if (*info).window_sequence[0 as c_int as usize] as c_uint
        != EIGHT_SHORT_SEQUENCE as c_int as c_uint
    {
        put_bits(&mut (*s).pb, 6 as c_int, (*info).max_sfb as BitBuf);
        put_bits(
            &mut (*s).pb,
            1 as c_int,
            ((*info).predictor_present != 0) as c_int as BitBuf,
        );
    } else {
        put_bits(&mut (*s).pb, 4 as c_int, (*info).max_sfb as BitBuf);
        w = 1 as c_int;
        while w < 8 as c_int {
            put_bits(
                &mut (*s).pb,
                1 as c_int,
                ((*info).group_len[w as usize] == 0) as c_int as BitBuf,
            );
            w += 1;
            w;
        }
    };
}
unsafe extern "C" fn encode_ms_info(mut pb: *mut PutBitContext, mut cpe: *mut ChannelElement) {
    let mut i: c_int = 0;
    let mut w: c_int = 0;
    put_bits(pb, 2 as c_int, (*cpe).ms_mode as BitBuf);
    if (*cpe).ms_mode == 1 as c_int {
        w = 0 as c_int;
        while w < (*cpe).ch[0 as c_int as usize].ics.num_windows {
            i = 0 as c_int;
            while i < (*cpe).ch[0 as c_int as usize].ics.max_sfb as c_int {
                put_bits(
                    pb,
                    1 as c_int,
                    (*cpe).ms_mask[(w * 16 as c_int + i) as usize] as BitBuf,
                );
                i += 1;
                i;
            }
            w += (*cpe).ch[0 as c_int as usize].ics.group_len[w as usize] as c_int;
        }
    }
}
unsafe extern "C" fn adjust_frame_information(mut cpe: *mut ChannelElement, mut chans: c_int) {
    let mut i: c_int = 0;
    let mut w: c_int = 0;
    let mut w2: c_int = 0;
    let mut g: c_int = 0;
    let mut ch: c_int = 0;
    let mut maxsfb: c_int = 0;
    let mut cmaxsfb: c_int = 0;
    ch = 0 as c_int;
    while ch < chans {
        let mut ics: *mut IndividualChannelStream =
            &mut (*((*cpe).ch).as_mut_ptr().offset(ch as isize)).ics;
        maxsfb = 0 as c_int;
        (*cpe).ch[ch as usize].pulse.num_pulse = 0 as c_int;
        w = 0 as c_int;
        while w < (*ics).num_windows {
            w2 = 0 as c_int;
            while w2 < (*ics).group_len[w as usize] as c_int {
                cmaxsfb = (*ics).num_swb;
                while cmaxsfb > 0 as c_int
                    && (*cpe).ch[ch as usize].zeroes
                        [(w * 16 as c_int + cmaxsfb - 1 as c_int) as usize]
                        as c_int
                        != 0
                {
                    cmaxsfb -= 1;
                    cmaxsfb;
                }
                maxsfb = if maxsfb > cmaxsfb { maxsfb } else { cmaxsfb };
                w2 += 1;
                w2;
            }
            w += (*ics).group_len[w as usize] as c_int;
        }
        (*ics).max_sfb = maxsfb as c_uchar;
        w = 0 as c_int;
        while w < (*ics).num_windows {
            g = 0 as c_int;
            while g < (*ics).max_sfb as c_int {
                i = 1 as c_int;
                w2 = w;
                while w2 < w + (*ics).group_len[w as usize] as c_int {
                    if (*cpe).ch[ch as usize].zeroes[(w2 * 16 as c_int + g) as usize] == 0 {
                        i = 0 as c_int;
                        break;
                    } else {
                        w2 += 1;
                        w2;
                    }
                }
                (*cpe).ch[ch as usize].zeroes[(w * 16 as c_int + g) as usize] = i as c_uchar;
                g += 1;
                g;
            }
            w += (*ics).group_len[w as usize] as c_int;
        }
        ch += 1;
        ch;
    }
    if chans > 1 as c_int && (*cpe).common_window != 0 {
        let mut ics0: *mut IndividualChannelStream =
            &mut (*((*cpe).ch).as_mut_ptr().offset(0 as c_int as isize)).ics;
        let mut ics1: *mut IndividualChannelStream =
            &mut (*((*cpe).ch).as_mut_ptr().offset(1 as c_int as isize)).ics;
        let mut msc: c_int = 0 as c_int;
        (*ics0).max_sfb = (if (*ics0).max_sfb as c_int > (*ics1).max_sfb as c_int {
            (*ics0).max_sfb as c_int
        } else {
            (*ics1).max_sfb as c_int
        }) as c_uchar;
        (*ics1).max_sfb = (*ics0).max_sfb;
        w = 0 as c_int;
        while w < (*ics0).num_windows * 16 as c_int {
            i = 0 as c_int;
            while i < (*ics0).max_sfb as c_int {
                if (*cpe).ms_mask[(w + i) as usize] != 0 {
                    msc += 1;
                    msc;
                }
                i += 1;
                i;
            }
            w += 16 as c_int;
        }
        if msc == 0 as c_int || (*ics0).max_sfb as c_int == 0 as c_int {
            (*cpe).ms_mode = 0 as c_int;
        } else {
            (*cpe).ms_mode = if msc < (*ics0).max_sfb as c_int * (*ics0).num_windows {
                1 as c_int
            } else {
                2 as c_int
            };
        }
    }
}
unsafe extern "C" fn apply_intensity_stereo(mut cpe: *mut ChannelElement) {
    let mut w: c_int = 0;
    let mut w2: c_int = 0;
    let mut g: c_int = 0;
    let mut i: c_int = 0;
    let mut ics: *mut IndividualChannelStream =
        &mut (*((*cpe).ch).as_mut_ptr().offset(0 as c_int as isize)).ics;
    if (*cpe).common_window == 0 {
        return;
    }
    w = 0 as c_int;
    while w < (*ics).num_windows {
        w2 = 0 as c_int;
        while w2 < (*ics).group_len[w as usize] as c_int {
            let mut start: c_int = (w + w2) * 128 as c_int;
            g = 0 as c_int;
            while g < (*ics).num_swb {
                let mut p: c_int = (-(1 as c_int) as c_uint).wrapping_add(
                    (2 as c_int as c_uint).wrapping_mul(
                        ((*cpe).ch[1 as c_int as usize].band_type[(w * 16 as c_int + g) as usize]
                            as c_uint)
                            .wrapping_sub(14 as c_int as c_uint),
                    ),
                ) as c_int;
                let mut scale: c_float =
                    (*cpe).ch[0 as c_int as usize].is_ener[(w * 16 as c_int + g) as usize];
                if (*cpe).is_mask[(w * 16 as c_int + g) as usize] == 0 {
                    start += *((*ics).swb_sizes).offset(g as isize) as c_int;
                } else {
                    if (*cpe).ms_mask[(w * 16 as c_int + g) as usize] != 0 {
                        p *= -(1 as c_int);
                    }
                    i = 0 as c_int;
                    while i < *((*ics).swb_sizes).offset(g as isize) as c_int {
                        let mut sum: c_float = ((*cpe).ch[0 as c_int as usize].coeffs
                            [(start + i) as usize]
                            + p as c_float
                                * (*cpe).ch[1 as c_int as usize].coeffs[(start + i) as usize])
                            * scale;
                        (*cpe).ch[0 as c_int as usize].coeffs[(start + i) as usize] = sum;
                        (*cpe).ch[1 as c_int as usize].coeffs[(start + i) as usize] = 0.0f32;
                        i += 1;
                        i;
                    }
                    start += *((*ics).swb_sizes).offset(g as isize) as c_int;
                }
                g += 1;
                g;
            }
            w2 += 1;
            w2;
        }
        w += (*ics).group_len[w as usize] as c_int;
    }
}
unsafe extern "C" fn apply_mid_side_stereo(mut cpe: *mut ChannelElement) {
    let mut w: c_int = 0;
    let mut w2: c_int = 0;
    let mut g: c_int = 0;
    let mut i: c_int = 0;
    let mut ics: *mut IndividualChannelStream =
        &mut (*((*cpe).ch).as_mut_ptr().offset(0 as c_int as isize)).ics;
    if (*cpe).common_window == 0 {
        return;
    }
    w = 0 as c_int;
    while w < (*ics).num_windows {
        w2 = 0 as c_int;
        while w2 < (*ics).group_len[w as usize] as c_int {
            let mut start: c_int = (w + w2) * 128 as c_int;
            g = 0 as c_int;
            while g < (*ics).num_swb {
                if (*cpe).ms_mask[(w * 16 as c_int + g) as usize] == 0
                    || (*cpe).is_mask[(w * 16 as c_int + g) as usize] as c_int != 0
                    || (*cpe).ch[0 as c_int as usize].band_type[(w * 16 as c_int + g) as usize]
                        as c_uint
                        >= NOISE_BT as c_int as c_uint
                    || (*cpe).ch[1 as c_int as usize].band_type[(w * 16 as c_int + g) as usize]
                        as c_uint
                        >= NOISE_BT as c_int as c_uint
                {
                    start += *((*ics).swb_sizes).offset(g as isize) as c_int;
                } else {
                    i = 0 as c_int;
                    while i < *((*ics).swb_sizes).offset(g as isize) as c_int {
                        let mut L: c_float = ((*cpe).ch[0 as c_int as usize].coeffs
                            [(start + i) as usize]
                            + (*cpe).ch[1 as c_int as usize].coeffs[(start + i) as usize])
                            * 0.5f32;
                        let mut R: c_float =
                            L - (*cpe).ch[1 as c_int as usize].coeffs[(start + i) as usize];
                        (*cpe).ch[0 as c_int as usize].coeffs[(start + i) as usize] = L;
                        (*cpe).ch[1 as c_int as usize].coeffs[(start + i) as usize] = R;
                        i += 1;
                        i;
                    }
                    start += *((*ics).swb_sizes).offset(g as isize) as c_int;
                }
                g += 1;
                g;
            }
            w2 += 1;
            w2;
        }
        w += (*ics).group_len[w as usize] as c_int;
    }
}
unsafe extern "C" fn encode_band_info(
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
) {
    let mut w: c_int = 0;
    if ((*(*s).coder).set_special_band_scalefactors).is_some() {
        ((*(*s).coder).set_special_band_scalefactors).expect("non-null function pointer")(s, sce);
    }
    w = 0 as c_int;
    while w < (*sce).ics.num_windows {
        ((*(*s).coder).encode_window_bands_info).expect("non-null function pointer")(
            s,
            sce,
            w,
            (*sce).ics.group_len[w as usize] as c_int,
            (*s).lambda,
        );
        w += (*sce).ics.group_len[w as usize] as c_int;
    }
}
unsafe extern "C" fn encode_scale_factors(
    mut _avctx: *mut AVCodecContext,
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
) {
    let mut diff: c_int = 0;
    let mut off_sf: c_int = (*sce).sf_idx[0 as c_int as usize];
    let mut off_pns: c_int = (*sce).sf_idx[0 as c_int as usize] - 90 as c_int;
    let mut off_is: c_int = 0 as c_int;
    let mut noise_flag: c_int = 1 as c_int;
    let mut i: c_int = 0;
    let mut w: c_int = 0;
    w = 0 as c_int;
    while w < (*sce).ics.num_windows {
        let mut current_block_19: u64;
        i = 0 as c_int;
        while i < (*sce).ics.max_sfb as c_int {
            if (*sce).zeroes[(w * 16 as c_int + i) as usize] == 0 {
                if (*sce).band_type[(w * 16 as c_int + i) as usize] as c_uint
                    == NOISE_BT as c_int as c_uint
                {
                    diff = (*sce).sf_idx[(w * 16 as c_int + i) as usize] - off_pns;
                    off_pns = (*sce).sf_idx[(w * 16 as c_int + i) as usize];
                    let fresh1 = noise_flag;
                    noise_flag -= 1;
                    if fresh1 > 0 as c_int {
                        put_bits(&mut (*s).pb, 9 as c_int, (diff + 256 as c_int) as BitBuf);
                        current_block_19 = 10680521327981672866;
                    } else {
                        current_block_19 = 7976072742316086414;
                    }
                } else {
                    if (*sce).band_type[(w * 16 as c_int + i) as usize] as c_uint
                        == INTENSITY_BT as c_int as c_uint
                        || (*sce).band_type[(w * 16 as c_int + i) as usize] as c_uint
                            == INTENSITY_BT2 as c_int as c_uint
                    {
                        diff = (*sce).sf_idx[(w * 16 as c_int + i) as usize] - off_is;
                        off_is = (*sce).sf_idx[(w * 16 as c_int + i) as usize];
                    } else {
                        diff = (*sce).sf_idx[(w * 16 as c_int + i) as usize] - off_sf;
                        off_sf = (*sce).sf_idx[(w * 16 as c_int + i) as usize];
                    }
                    current_block_19 = 7976072742316086414;
                }
                match current_block_19 {
                    10680521327981672866 => {}
                    _ => {
                        diff += 60 as c_int;
                        assert!(diff >= 0 as c_int && diff <= 120 as c_int);
                        put_bits(
                            &mut (*s).pb,
                            ff_aac_scalefactor_bits[diff as usize] as c_int,
                            ff_aac_scalefactor_code[diff as usize],
                        );
                    }
                }
            }
            i += 1;
            i;
        }
        w += (*sce).ics.group_len[w as usize] as c_int;
    }
}
unsafe extern "C" fn encode_pulses(mut s: *mut AACEncContext, mut pulse: *mut Pulse) {
    let mut i: c_int = 0;
    put_bits(
        &mut (*s).pb,
        1 as c_int,
        ((*pulse).num_pulse != 0) as c_int as BitBuf,
    );
    if (*pulse).num_pulse == 0 {
        return;
    }
    put_bits(
        &mut (*s).pb,
        2 as c_int,
        ((*pulse).num_pulse - 1 as c_int) as BitBuf,
    );
    put_bits(&mut (*s).pb, 6 as c_int, (*pulse).start as BitBuf);
    i = 0 as c_int;
    while i < (*pulse).num_pulse {
        put_bits(&mut (*s).pb, 5 as c_int, (*pulse).pos[i as usize] as BitBuf);
        put_bits(&mut (*s).pb, 4 as c_int, (*pulse).amp[i as usize] as BitBuf);
        i += 1;
        i;
    }
}
unsafe extern "C" fn encode_spectral_coeffs(
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
) {
    let mut start: c_int = 0;
    let mut i: c_int = 0;
    let mut w: c_int = 0;
    let mut w2: c_int = 0;
    w = 0 as c_int;
    while w < (*sce).ics.num_windows {
        start = 0 as c_int;
        i = 0 as c_int;
        while i < (*sce).ics.max_sfb as c_int {
            if (*sce).zeroes[(w * 16 as c_int + i) as usize] != 0 {
                start += *((*sce).ics.swb_sizes).offset(i as isize) as c_int;
            } else {
                w2 = w;
                while w2 < w + (*sce).ics.group_len[w as usize] as c_int {
                    ((*(*s).coder).quantize_and_encode_band).expect("non-null function pointer")(
                        s,
                        &mut (*s).pb,
                        &mut *((*sce).coeffs)
                            .as_mut_ptr()
                            .offset((start + w2 * 128 as c_int) as isize),
                        ptr::null_mut::<c_float>(),
                        *((*sce).ics.swb_sizes).offset(i as isize) as c_int,
                        (*sce).sf_idx[(w * 16 as c_int + i) as usize],
                        (*sce).band_type[(w * 16 as c_int + i) as usize] as c_int,
                        (*s).lambda,
                        (*sce).ics.window_clipping[w as usize] as c_int,
                    );
                    w2 += 1;
                    w2;
                }
                start += *((*sce).ics.swb_sizes).offset(i as isize) as c_int;
            }
            i += 1;
            i;
        }
        w += (*sce).ics.group_len[w as usize] as c_int;
    }
}
unsafe extern "C" fn avoid_clipping(
    mut _s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
) {
    let mut start: c_int = 0;
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut w: c_int = 0;
    if (*sce).ics.clip_avoidance_factor < 1.0f32 {
        w = 0 as c_int;
        while w < (*sce).ics.num_windows {
            start = 0 as c_int;
            i = 0 as c_int;
            while i < (*sce).ics.max_sfb as c_int {
                let mut swb_coeffs: *mut c_float = &mut *((*sce).coeffs)
                    .as_mut_ptr()
                    .offset((start + w * 128 as c_int) as isize)
                    as *mut c_float;
                j = 0 as c_int;
                while j < *((*sce).ics.swb_sizes).offset(i as isize) as c_int {
                    *swb_coeffs.offset(j as isize) *= (*sce).ics.clip_avoidance_factor;
                    j += 1;
                    j;
                }
                start += *((*sce).ics.swb_sizes).offset(i as isize) as c_int;
                i += 1;
                i;
            }
            w += 1;
            w;
        }
    }
}
unsafe extern "C" fn encode_individual_channel(
    mut avctx: *mut AVCodecContext,
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
    mut common_window: c_int,
) -> c_int {
    put_bits(
        &mut (*s).pb,
        8 as c_int,
        (*sce).sf_idx[0 as c_int as usize] as BitBuf,
    );
    if common_window == 0 {
        put_ics_info(s, &mut (*sce).ics);
        if ((*(*s).coder).encode_main_pred).is_some() {
            ((*(*s).coder).encode_main_pred).expect("non-null function pointer")(s, sce);
        }
        if ((*(*s).coder).encode_ltp_info).is_some() {
            ((*(*s).coder).encode_ltp_info).expect("non-null function pointer")(s, sce, 0 as c_int);
        }
    }
    encode_band_info(s, sce);
    encode_scale_factors(avctx, s, sce);
    encode_pulses(s, &mut (*sce).pulse);
    put_bits(
        &mut (*s).pb,
        1 as c_int,
        ((*sce).tns.present != 0) as c_int as BitBuf,
    );
    if ((*(*s).coder).encode_tns_info).is_some() {
        ((*(*s).coder).encode_tns_info).expect("non-null function pointer")(s, sce);
    }
    put_bits(&mut (*s).pb, 1 as c_int, 0 as c_int as BitBuf);
    encode_spectral_coeffs(s, sce);
    0 as c_int
}
unsafe fn put_bitstream_info(mut s: *mut AACEncContext, mut name: &CStr) {
    let mut i: c_int = 0;
    let mut namelen: c_int = 0;
    let mut padbits: c_int = 0;
    namelen = name.to_bytes().len().wrapping_add(2) as c_int;
    put_bits(&mut (*s).pb, 3 as c_int, TYPE_FIL as c_int as BitBuf);
    put_bits(
        &mut (*s).pb,
        4 as c_int,
        (if namelen > 15 as c_int {
            15 as c_int
        } else {
            namelen
        }) as BitBuf,
    );
    if namelen >= 15 as c_int {
        put_bits(&mut (*s).pb, 8 as c_int, (namelen - 14 as c_int) as BitBuf);
    }
    put_bits(&mut (*s).pb, 4 as c_int, 0 as c_int as BitBuf);
    padbits = -put_bits_count(&mut (*s).pb) & 7 as c_int;
    align_put_bits(&mut (*s).pb);
    i = 0 as c_int;
    while i < namelen - 2 as c_int {
        put_bits(
            &mut (*s).pb,
            8 as c_int,
            name.to_bytes()[i as usize] as BitBuf,
        );
        i += 1;
        i;
    }
    put_bits(&mut (*s).pb, 12 as c_int - padbits, 0 as c_int as BitBuf);
}
unsafe extern "C" fn copy_input_samples(mut s: *mut AACEncContext, mut frame: *const AVFrame) {
    let mut ch: c_int = 0;
    let mut end: c_int = 2048 as c_int
        + (if !frame.is_null() {
            (*frame).nb_samples
        } else {
            0 as c_int
        });
    let mut channel_map: *const c_uchar = (*s).reorder_map;
    ch = 0 as c_int;
    while ch < (*s).channels {
        ptr::copy_nonoverlapping(
            &mut *(*((*s).planar_samples).as_mut_ptr().offset(ch as isize))
                .offset(2048 as c_int as isize) as *mut c_float,
            &mut *(*((*s).planar_samples).as_mut_ptr().offset(ch as isize))
                .offset(1024 as c_int as isize) as *mut c_float,
            1024,
        );
        if !frame.is_null() {
            ptr::copy_nonoverlapping(
                *((*frame).extended_data).offset(*channel_map.offset(ch as isize) as isize)
                    as *mut c_float,
                &mut *(*((*s).planar_samples).as_mut_ptr().offset(ch as isize))
                    .offset(2048 as c_int as isize),
                (*frame).nb_samples as usize,
            );
        }
        ptr::write_bytes(
            &mut *(*((*s).planar_samples).as_mut_ptr().offset(ch as isize)).offset(end as isize)
                as *mut c_float,
            0,
            (3072 as c_int - end) as usize,
        );
        ch += 1;
        ch;
    }
}
unsafe extern "C" fn aac_encode_frame(
    mut avctx: *mut AVCodecContext,
    mut avpkt: *mut AVPacket,
    mut frame: *const AVFrame,
    mut got_packet_ptr: *mut c_int,
) -> c_int {
    let mut s: *mut AACEncContext = (*avctx).priv_data as *mut AACEncContext;
    let mut samples: *mut *mut c_float = ((*s).planar_samples).as_mut_ptr();
    let mut samples2: *mut c_float = ptr::null_mut::<c_float>();
    let mut la: *mut c_float = ptr::null_mut::<c_float>();
    let mut overlap: *mut c_float = ptr::null_mut::<c_float>();
    let mut cpe: *mut ChannelElement = ptr::null_mut::<ChannelElement>();
    let mut sce: *mut SingleChannelElement = ptr::null_mut::<SingleChannelElement>();
    let mut ics: *mut IndividualChannelStream = ptr::null_mut::<IndividualChannelStream>();
    let mut i: c_int = 0;
    let mut its: c_int = 0;
    let mut ch: c_int = 0;
    let mut w: c_int = 0;
    let mut chans: c_int = 0;
    let mut tag: c_int = 0;
    let mut start_ch: c_int = 0;
    let mut ret: c_int = 0;
    let mut frame_bits: c_int = 0;
    let mut target_bits: c_int = 0;
    let mut rate_bits: c_int = 0;
    let mut too_many_bits: c_int = 0;
    let mut too_few_bits: c_int = 0;
    let mut ms_mode: c_int = 0 as c_int;
    let mut is_mode: c_int = 0 as c_int;
    let mut tns_mode: c_int = 0 as c_int;
    let mut pred_mode: c_int = 0 as c_int;
    let mut chan_el_counter: [c_int; 4] = [0; 4];
    let mut windows: [FFPsyWindowInfo; 16] = [FFPsyWindowInfo {
        window_type: [0; 3],
        window_shape: 0,
        num_windows: 0,
        grouping: [0; 8],
        clipping: [0.; 8],
        window_sizes: ptr::null_mut::<c_int>(),
    }; 16];
    if !frame.is_null() {
        ret = ff_af_queue_add(&mut (*s).afq, frame);
        if ret < 0 as c_int {
            return ret;
        }
    } else if (*s).afq.remaining_samples == 0
        || (*s).afq.frame_alloc == 0 && (*s).afq.frame_count == 0
    {
        return 0 as c_int;
    }
    copy_input_samples(s, frame);
    if !((*s).psypp).is_null() {
        ff_psy_preprocess(
            (*s).psypp,
            ((*s).planar_samples).as_mut_ptr(),
            (*s).channels,
        );
    }
    if (*avctx).frame_num == 0 {
        return 0 as c_int;
    }
    start_ch = 0 as c_int;
    i = 0 as c_int;
    while i < *((*s).chan_map).offset(0 as c_int as isize) as c_int {
        let mut wi: *mut FFPsyWindowInfo = windows.as_mut_ptr().offset(start_ch as isize);
        tag = *((*s).chan_map).offset((i + 1 as c_int) as isize) as c_int;
        chans = if tag == TYPE_CPE as c_int {
            2 as c_int
        } else {
            1 as c_int
        };
        cpe = &mut *((*s).cpe).offset(i as isize) as *mut ChannelElement;
        ch = 0 as c_int;
        while ch < chans {
            let mut k: c_int = 0;
            let mut clip_avoidance_factor: c_float = 0.;
            sce = &mut *((*cpe).ch).as_mut_ptr().offset(ch as isize) as *mut SingleChannelElement;
            ics = &mut (*sce).ics;
            (*s).cur_channel = start_ch + ch;
            overlap = &mut *(*samples.offset((*s).cur_channel as isize)).offset(0 as c_int as isize)
                as *mut c_float;
            samples2 = overlap.offset(1024 as c_int as isize);
            la = samples2.offset((448 as c_int + 64 as c_int) as isize);
            if frame.is_null() {
                la = ptr::null_mut::<c_float>();
            }
            if tag == TYPE_LFE as c_int {
                let fresh2 = &mut (*wi.offset(ch as isize)).window_type[1 as c_int as usize];
                *fresh2 = ONLY_LONG_SEQUENCE as c_int;
                (*wi.offset(ch as isize)).window_type[0 as c_int as usize] = *fresh2;
                (*wi.offset(ch as isize)).window_shape = 0 as c_int;
                (*wi.offset(ch as isize)).num_windows = 1 as c_int;
                (*wi.offset(ch as isize)).grouping[0 as c_int as usize] = 1 as c_int;
                (*wi.offset(ch as isize)).clipping[0 as c_int as usize] = 0 as c_int as c_float;
                (*ics).num_swb = if (*s).samplerate_index >= 8 as c_int {
                    1 as c_int
                } else {
                    3 as c_int
                };
            } else {
                *wi.offset(ch as isize) = ((*(*s).psy.model).window)
                    .expect("non-null function pointer")(
                    &mut (*s).psy,
                    samples2,
                    la,
                    (*s).cur_channel,
                    (*ics).window_sequence[0 as c_int as usize] as c_int,
                );
            }
            (*ics).window_sequence[1 as c_int as usize] =
                (*ics).window_sequence[0 as c_int as usize];
            (*ics).window_sequence[0 as c_int as usize] =
                (*wi.offset(ch as isize)).window_type[0 as c_int as usize] as WindowSequence;
            (*ics).use_kb_window[1 as c_int as usize] = (*ics).use_kb_window[0 as c_int as usize];
            (*ics).use_kb_window[0 as c_int as usize] =
                (*wi.offset(ch as isize)).window_shape as c_uchar;
            (*ics).num_windows = (*wi.offset(ch as isize)).num_windows;
            (*ics).swb_sizes =
                *((*s).psy.bands).offset(((*ics).num_windows == 8 as c_int) as c_int as isize);
            (*ics).num_swb = if tag == TYPE_LFE as c_int {
                (*ics).num_swb
            } else {
                *((*s).psy.num_bands).offset(((*ics).num_windows == 8 as c_int) as c_int as isize)
            };
            (*ics).max_sfb = (if (*ics).max_sfb as c_int > (*ics).num_swb {
                (*ics).num_swb
            } else {
                (*ics).max_sfb as c_int
            }) as c_uchar;
            (*ics).swb_offset = if (*wi.offset(ch as isize)).window_type[0 as c_int as usize]
                == EIGHT_SHORT_SEQUENCE as c_int
            {
                ff_swb_offset_128[(*s).samplerate_index as usize]
            } else {
                ff_swb_offset_1024[(*s).samplerate_index as usize]
            };
            (*ics).tns_max_bands = if (*wi.offset(ch as isize)).window_type[0 as c_int as usize]
                == EIGHT_SHORT_SEQUENCE as c_int
            {
                ff_tns_max_bands_128[(*s).samplerate_index as usize] as c_int
            } else {
                ff_tns_max_bands_1024[(*s).samplerate_index as usize] as c_int
            };
            w = 0 as c_int;
            while w < (*ics).num_windows {
                (*ics).group_len[w as usize] =
                    (*wi.offset(ch as isize)).grouping[w as usize] as c_uchar;
                w += 1;
                w;
            }
            clip_avoidance_factor = 0.0f32;
            w = 0 as c_int;
            while w < (*ics).num_windows {
                let mut wbuf: *const c_float = overlap.offset((w * 128 as c_int) as isize);
                let wlen: c_int = 2048 as c_int / (*ics).num_windows;
                let mut max: c_float = 0 as c_int as c_float;
                let mut j: c_int = 0;
                j = 0 as c_int;
                while j < wlen {
                    max = if max > fabsf(*wbuf.offset(j as isize)) {
                        max
                    } else {
                        fabsf(*wbuf.offset(j as isize))
                    };
                    j += 1;
                    j;
                }
                (*wi.offset(ch as isize)).clipping[w as usize] = max;
                w += 1;
                w;
            }
            w = 0 as c_int;
            while w < (*ics).num_windows {
                if (*wi.offset(ch as isize)).clipping[w as usize] > 0.95f32 {
                    (*ics).window_clipping[w as usize] = 1 as c_int as c_uchar;
                    clip_avoidance_factor =
                        if clip_avoidance_factor > (*wi.offset(ch as isize)).clipping[w as usize] {
                            clip_avoidance_factor
                        } else {
                            (*wi.offset(ch as isize)).clipping[w as usize]
                        };
                } else {
                    (*ics).window_clipping[w as usize] = 0 as c_int as c_uchar;
                }
                w += 1;
                w;
            }
            if clip_avoidance_factor > 0.95f32 {
                (*ics).clip_avoidance_factor = 0.95f32 / clip_avoidance_factor;
            } else {
                (*ics).clip_avoidance_factor = 1.0f32;
            }
            apply_window_and_mdct(s, sce, overlap);
            if (*s).options.ltp != 0 && ((*(*s).coder).update_ltp).is_some() {
                ((*(*s).coder).update_ltp).expect("non-null function pointer")(s, sce);
                (apply_window[(*sce).ics.window_sequence[0 as c_int as usize] as usize])
                    .expect("non-null function pointer")(
                    (*s).fdsp,
                    sce,
                    &mut *((*sce).ltp_state).as_mut_ptr().offset(0 as c_int as isize),
                );
                ((*s).mdct1024_fn).expect("non-null function pointer")(
                    (*s).mdct1024,
                    ((*sce).lcoeffs).as_mut_ptr() as *mut c_void,
                    ((*sce).ret_buf).as_mut_ptr() as *mut c_void,
                    size_of::<c_float>() as c_ulong as ptrdiff_t,
                );
            }
            k = 0 as c_int;
            while k < 1024 as c_int {
                if !(fabs((*cpe).ch[ch as usize].coeffs[k as usize] as c_double) < 1E16f64) {
                    av_log(
                        avctx as *mut c_void,
                        16 as c_int,
                        b"Input contains (near) NaN/+-Inf\n\0" as *const u8 as *const c_char,
                    );
                    return -(22 as c_int);
                }
                k += 1;
                k;
            }
            avoid_clipping(s, sce);
            ch += 1;
            ch;
        }
        start_ch += chans;
        i += 1;
        i;
    }
    ret = ff_alloc_packet(avctx, avpkt, (8192 as c_int * (*s).channels) as c_long);
    if ret < 0 as c_int {
        return ret;
    }
    its = 0 as c_int;
    frame_bits = its;
    loop {
        init_put_bits(&mut (*s).pb, (*avpkt).data, (*avpkt).size);
        if (*avctx).frame_num & 0xff as c_int as c_long == 1 as c_int as c_long
            && (*avctx).flags & (1 as c_int) << 23 as c_int == 0
        {
            put_bitstream_info(s, c"Lavc60.33.100");
        }
        start_ch = 0 as c_int;
        target_bits = 0 as c_int;
        chan_el_counter.fill(0);
        i = 0 as c_int;
        while i < *((*s).chan_map).offset(0 as c_int as isize) as c_int {
            let mut wi_0: *mut FFPsyWindowInfo = windows.as_mut_ptr().offset(start_ch as isize);
            let mut coeffs: [*const c_float; 2] = [ptr::null::<c_float>(); 2];
            tag = *((*s).chan_map).offset((i + 1 as c_int) as isize) as c_int;
            chans = if tag == TYPE_CPE as c_int {
                2 as c_int
            } else {
                1 as c_int
            };
            cpe = &mut *((*s).cpe).offset(i as isize) as *mut ChannelElement;
            (*cpe).common_window = 0 as c_int;
            (*cpe).is_mask.fill(0);
            (*cpe).ms_mask.fill(0);
            put_bits(&mut (*s).pb, 3 as c_int, tag as BitBuf);
            let fresh3 = chan_el_counter[tag as usize];
            chan_el_counter[tag as usize] += 1;
            put_bits(&mut (*s).pb, 4 as c_int, fresh3 as BitBuf);
            ch = 0 as c_int;
            while ch < chans {
                sce =
                    &mut *((*cpe).ch).as_mut_ptr().offset(ch as isize) as *mut SingleChannelElement;
                coeffs[ch as usize] = ((*sce).coeffs).as_mut_ptr();
                (*sce).ics.predictor_present = 0 as c_int;
                (*sce).ics.ltp.present = 0 as c_int as c_schar;
                (*sce).ics.ltp.used.fill(0);
                (*sce).ics.prediction_used.fill(0);
                (*sce).tns = TemporalNoiseShaping::default();
                w = 0 as c_int;
                while w < 128 as c_int {
                    if (*sce).band_type[w as usize] as c_uint > RESERVED_BT as c_int as c_uint {
                        (*sce).band_type[w as usize] = ZERO_BT;
                    }
                    w += 1;
                    w;
                }
                ch += 1;
                ch;
            }
            (*s).psy.bitres.alloc = -(1 as c_int);
            (*s).psy.bitres.bits = (*s).last_frame_pb_count / (*s).channels;
            ((*(*s).psy.model).analyze).expect("non-null function pointer")(
                &mut (*s).psy,
                start_ch,
                coeffs.as_mut_ptr(),
                wi_0,
            );
            if (*s).psy.bitres.alloc > 0 as c_int {
                target_bits = (target_bits as c_float
                    + (*s).psy.bitres.alloc as c_float
                        * ((*s).lambda
                            / (if (*avctx).global_quality != 0 {
                                (*avctx).global_quality
                            } else {
                                120 as c_int
                            }) as c_float)) as c_int;
                (*s).psy.bitres.alloc /= chans;
            }
            (*s).cur_type = tag as RawDataBlockType;
            ch = 0 as c_int;
            while ch < chans {
                (*s).cur_channel = start_ch + ch;
                if (*s).options.pns != 0 && ((*(*s).coder).mark_pns).is_some() {
                    ((*(*s).coder).mark_pns).expect("non-null function pointer")(
                        s,
                        avctx,
                        &mut *((*cpe).ch).as_mut_ptr().offset(ch as isize),
                    );
                }
                ((*(*s).coder).search_for_quantizers).expect("non-null function pointer")(
                    avctx,
                    s,
                    &mut *((*cpe).ch).as_mut_ptr().offset(ch as isize),
                    (*s).lambda,
                );
                ch += 1;
                ch;
            }
            if chans > 1 as c_int
                && (*wi_0.offset(0 as c_int as isize)).window_type[0 as c_int as usize]
                    == (*wi_0.offset(1 as c_int as isize)).window_type[0 as c_int as usize]
                && (*wi_0.offset(0 as c_int as isize)).window_shape
                    == (*wi_0.offset(1 as c_int as isize)).window_shape
            {
                (*cpe).common_window = 1 as c_int;
                w = 0 as c_int;
                while w < (*wi_0.offset(0 as c_int as isize)).num_windows {
                    if (*wi_0.offset(0 as c_int as isize)).grouping[w as usize]
                        != (*wi_0.offset(1 as c_int as isize)).grouping[w as usize]
                    {
                        (*cpe).common_window = 0 as c_int;
                        break;
                    } else {
                        w += 1;
                        w;
                    }
                }
            }
            ch = 0 as c_int;
            while ch < chans {
                sce =
                    &mut *((*cpe).ch).as_mut_ptr().offset(ch as isize) as *mut SingleChannelElement;
                (*s).cur_channel = start_ch + ch;
                if (*s).options.tns != 0 && ((*(*s).coder).search_for_tns).is_some() {
                    ((*(*s).coder).search_for_tns).expect("non-null function pointer")(s, sce);
                }
                if (*s).options.tns != 0 && ((*(*s).coder).apply_tns_filt).is_some() {
                    ((*(*s).coder).apply_tns_filt).expect("non-null function pointer")(s, sce);
                }
                if (*sce).tns.present != 0 {
                    tns_mode = 1 as c_int;
                }
                if (*s).options.pns != 0 && ((*(*s).coder).search_for_pns).is_some() {
                    ((*(*s).coder).search_for_pns).expect("non-null function pointer")(
                        s, avctx, sce,
                    );
                }
                ch += 1;
                ch;
            }
            (*s).cur_channel = start_ch;
            if (*s).options.intensity_stereo != 0 {
                if ((*(*s).coder).search_for_is).is_some() {
                    ((*(*s).coder).search_for_is).expect("non-null function pointer")(
                        s, avctx, cpe,
                    );
                }
                if (*cpe).is_mode != 0 {
                    is_mode = 1 as c_int;
                }
                apply_intensity_stereo(cpe);
            }
            if (*s).options.pred != 0 {
                ch = 0 as c_int;
                while ch < chans {
                    sce = &mut *((*cpe).ch).as_mut_ptr().offset(ch as isize)
                        as *mut SingleChannelElement;
                    (*s).cur_channel = start_ch + ch;
                    if (*s).options.pred != 0 && ((*(*s).coder).search_for_pred).is_some() {
                        ((*(*s).coder).search_for_pred).expect("non-null function pointer")(s, sce);
                    }
                    if (*cpe).ch[ch as usize].ics.predictor_present != 0 {
                        pred_mode = 1 as c_int;
                    }
                    ch += 1;
                    ch;
                }
                if ((*(*s).coder).adjust_common_pred).is_some() {
                    ((*(*s).coder).adjust_common_pred).expect("non-null function pointer")(s, cpe);
                }
                ch = 0 as c_int;
                while ch < chans {
                    sce = &mut *((*cpe).ch).as_mut_ptr().offset(ch as isize)
                        as *mut SingleChannelElement;
                    (*s).cur_channel = start_ch + ch;
                    if (*s).options.pred != 0 && ((*(*s).coder).apply_main_pred).is_some() {
                        ((*(*s).coder).apply_main_pred).expect("non-null function pointer")(s, sce);
                    }
                    ch += 1;
                    ch;
                }
                (*s).cur_channel = start_ch;
            }
            if (*s).options.mid_side != 0 {
                if (*s).options.mid_side == -(1 as c_int) && ((*(*s).coder).search_for_ms).is_some()
                {
                    ((*(*s).coder).search_for_ms).expect("non-null function pointer")(s, cpe);
                } else if (*cpe).common_window != 0 {
                    (*cpe).ms_mask.fill(1);
                }
                apply_mid_side_stereo(cpe);
            }
            adjust_frame_information(cpe, chans);
            if (*s).options.ltp != 0 {
                ch = 0 as c_int;
                while ch < chans {
                    sce = &mut *((*cpe).ch).as_mut_ptr().offset(ch as isize)
                        as *mut SingleChannelElement;
                    (*s).cur_channel = start_ch + ch;
                    if ((*(*s).coder).search_for_ltp).is_some() {
                        ((*(*s).coder).search_for_ltp).expect("non-null function pointer")(
                            s,
                            sce,
                            (*cpe).common_window,
                        );
                    }
                    if (*sce).ics.ltp.present != 0 {
                        pred_mode = 1 as c_int;
                    }
                    ch += 1;
                    ch;
                }
                (*s).cur_channel = start_ch;
                if ((*(*s).coder).adjust_common_ltp).is_some() {
                    ((*(*s).coder).adjust_common_ltp).expect("non-null function pointer")(s, cpe);
                }
            }
            if chans == 2 as c_int {
                put_bits(&mut (*s).pb, 1 as c_int, (*cpe).common_window as BitBuf);
                if (*cpe).common_window != 0 {
                    put_ics_info(
                        s,
                        &mut (*((*cpe).ch).as_mut_ptr().offset(0 as c_int as isize)).ics,
                    );
                    if ((*(*s).coder).encode_main_pred).is_some() {
                        ((*(*s).coder).encode_main_pred).expect("non-null function pointer")(
                            s,
                            &mut *((*cpe).ch).as_mut_ptr().offset(0 as c_int as isize),
                        );
                    }
                    if ((*(*s).coder).encode_ltp_info).is_some() {
                        ((*(*s).coder).encode_ltp_info).expect("non-null function pointer")(
                            s,
                            &mut *((*cpe).ch).as_mut_ptr().offset(0 as c_int as isize),
                            1 as c_int,
                        );
                    }
                    encode_ms_info(&mut (*s).pb, cpe);
                    if (*cpe).ms_mode != 0 {
                        ms_mode = 1 as c_int;
                    }
                }
            }
            ch = 0 as c_int;
            while ch < chans {
                (*s).cur_channel = start_ch + ch;
                encode_individual_channel(
                    avctx,
                    s,
                    &mut *((*cpe).ch).as_mut_ptr().offset(ch as isize),
                    (*cpe).common_window,
                );
                ch += 1;
                ch;
            }
            start_ch += chans;
            i += 1;
            i;
        }
        if (*avctx).flags & (1 as c_int) << 1 as c_int != 0 {
            break;
        }
        frame_bits = put_bits_count(&mut (*s).pb);
        rate_bits =
            ((*avctx).bit_rate * 1024 as c_int as c_long / (*avctx).sample_rate as c_long) as c_int;
        rate_bits = if rate_bits > 6144 as c_int * (*s).channels - 3 as c_int {
            6144 as c_int * (*s).channels - 3 as c_int
        } else {
            rate_bits
        };
        too_many_bits = if target_bits > rate_bits {
            target_bits
        } else {
            rate_bits
        };
        too_many_bits = if too_many_bits > 6144 as c_int * (*s).channels - 3 as c_int {
            6144 as c_int * (*s).channels - 3 as c_int
        } else {
            too_many_bits
        };
        too_few_bits = if (if rate_bits - rate_bits / 4 as c_int > target_bits {
            rate_bits - rate_bits / 4 as c_int
        } else {
            target_bits
        }) > too_many_bits
        {
            too_many_bits
        } else if rate_bits - rate_bits / 4 as c_int > target_bits {
            rate_bits - rate_bits / 4 as c_int
        } else {
            target_bits
        };
        if (*avctx).bit_rate_tolerance == 0 as c_int {
            if rate_bits < frame_bits {
                let mut ratio: c_float = rate_bits as c_float / frame_bits as c_float;
                (*s).lambda *= if 0.9f32 > ratio { ratio } else { 0.9f32 };
            } else {
                (*s).lambda = (if (*avctx).global_quality > 0 as c_int {
                    (*avctx).global_quality
                } else {
                    120 as c_int
                }) as c_float;
                break;
            }
        } else {
            too_few_bits = too_few_bits - too_few_bits / 8 as c_int;
            too_many_bits = too_many_bits + too_many_bits / 2 as c_int;
            if !(its == 0 as c_int
                || its < 5 as c_int && (frame_bits < too_few_bits || frame_bits > too_many_bits)
                || frame_bits >= 6144 as c_int * (*s).channels - 3 as c_int)
            {
                break;
            }
            let mut ratio_0: c_float = rate_bits as c_float / frame_bits as c_float;
            if frame_bits >= too_few_bits && frame_bits <= too_many_bits {
                ratio_0 = sqrtf(sqrtf(ratio_0));
                ratio_0 = av_clipf_c(ratio_0, 0.9f32, 1.1f32);
            } else {
                ratio_0 = sqrtf(ratio_0);
            }
            (*s).lambda = av_clipf_c((*s).lambda * ratio_0, 1.192_092_9e-7_f32, 65536.0f32);
            if ratio_0 > 0.9f32 && ratio_0 < 1.1f32 {
                break;
            }
            if is_mode != 0 || ms_mode != 0 || tns_mode != 0 || pred_mode != 0 {
                i = 0 as c_int;
                while i < *((*s).chan_map).offset(0 as c_int as isize) as c_int {
                    chans = if tag == TYPE_CPE as c_int {
                        2 as c_int
                    } else {
                        1 as c_int
                    };
                    cpe = &mut *((*s).cpe).offset(i as isize) as *mut ChannelElement;
                    ch = 0 as c_int;
                    while ch < chans {
                        (*cpe).ch[ch as usize].coeffs = (*cpe).ch[ch as usize].pcoeffs;
                        ch += 1;
                        ch;
                    }
                    i += 1;
                    i;
                }
            }
            its += 1;
            its;
        }
    }
    if (*s).options.ltp != 0 && ((*(*s).coder).ltp_insert_new_frame).is_some() {
        ((*(*s).coder).ltp_insert_new_frame).expect("non-null function pointer")(s);
    }
    put_bits(&mut (*s).pb, 3 as c_int, TYPE_END as c_int as BitBuf);
    flush_put_bits(&mut (*s).pb);
    (*s).last_frame_pb_count = put_bits_count(&mut (*s).pb);
    (*avpkt).size = put_bytes_output(&mut (*s).pb);
    (*s).lambda_sum += (*s).lambda;
    (*s).lambda_count += 1;
    (*s).lambda_count;
    ff_af_queue_remove(
        &mut (*s).afq,
        (*avctx).frame_size,
        &mut (*avpkt).pts,
        &mut (*avpkt).duration,
    );
    *got_packet_ptr = 1 as c_int;
    0 as c_int
}
#[cold]
unsafe extern "C" fn aac_encode_end(mut avctx: *mut AVCodecContext) -> c_int {
    let mut s: *mut AACEncContext = (*avctx).priv_data as *mut AACEncContext;
    av_log(
        avctx as *mut c_void,
        32 as c_int,
        b"Qavg: %.3f\n\0" as *const u8 as *const c_char,
        (if (*s).lambda_count != 0 {
            (*s).lambda_sum / (*s).lambda_count as c_float
        } else {
            ::core::f32::NAN
        }) as c_double,
    );
    av_tx_uninit(&mut (*s).mdct1024);
    av_tx_uninit(&mut (*s).mdct128);
    ff_psy_end(&mut (*s).psy);
    ff_lpc_end(&mut (*s).lpc);
    if !((*s).psypp).is_null() {
        ff_psy_preprocess_end((*s).psypp);
    }
    av_freep(&mut (*s).buffer.samples as *mut *mut c_float as *mut c_void);
    av_freep(&mut (*s).cpe as *mut *mut ChannelElement as *mut c_void);
    av_freep(&mut (*s).fdsp as *mut *mut AVFloatDSPContext as *mut c_void);
    ff_af_queue_close(&mut (*s).afq);
    0 as c_int
}
#[cold]
unsafe extern "C" fn dsp_init(mut avctx: *mut AVCodecContext, mut s: *mut AACEncContext) -> c_int {
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
        0 as c_int,
        1024 as c_int,
        &mut scale as *mut c_float as *const c_void,
        0 as c_int as c_ulong,
    );
    if ret < 0 as c_int {
        return ret;
    }
    ret = av_tx_init(
        &mut (*s).mdct128,
        &mut (*s).mdct128_fn,
        AV_TX_FLOAT_MDCT,
        0 as c_int,
        128 as c_int,
        &mut scale as *mut c_float as *const c_void,
        0 as c_int as c_ulong,
    );
    if ret < 0 as c_int {
        return ret;
    }
    0 as c_int
}
#[cold]
unsafe extern "C" fn alloc_buffers(
    mut _avctx: *mut AVCodecContext,
    mut s: *mut AACEncContext,
) -> c_int {
    let mut ch: c_int = 0;
    (*s).buffer.samples = av_calloc(
        ((*s).channels * 3 as c_int * 1024 as c_int) as c_ulong,
        size_of::<c_float>() as c_ulong,
    ) as *mut c_float;
    if ((*s).buffer.samples).is_null() || {
        (*s).cpe = av_calloc(
            *((*s).chan_map).offset(0 as c_int as isize) as c_ulong,
            size_of::<ChannelElement>() as c_ulong,
        ) as *mut ChannelElement;
        ((*s).cpe).is_null()
    } {
        return -(12 as c_int);
    }
    ch = 0 as c_int;
    while ch < (*s).channels {
        (*s).planar_samples[ch as usize] =
            ((*s).buffer.samples).offset((3 as c_int * 1024 as c_int * ch) as isize);
        ch += 1;
        ch;
    }
    0 as c_int
}
#[cold]
unsafe extern "C" fn aac_encode_init(mut avctx: *mut AVCodecContext) -> c_int {
    let mut s: *mut AACEncContext = (*avctx).priv_data as *mut AACEncContext;
    let mut i: c_int = 0;
    let mut ret: c_int = 0 as c_int;
    let mut sizes: [*const c_uchar; 2] = [ptr::null::<c_uchar>(); 2];
    let mut grouping: [c_uchar; 16] = [0; 16];
    let mut lengths: [c_int; 2] = [0; 2];
    (*s).last_frame_pb_count = 0 as c_int;
    (*avctx).frame_size = 1024 as c_int;
    (*avctx).initial_padding = 1024 as c_int;
    (*s).lambda = (if (*avctx).global_quality > 0 as c_int {
        (*avctx).global_quality
    } else {
        120 as c_int
    }) as c_float;
    (*s).channels = (*avctx).ch_layout.nb_channels;
    (*s).needs_pce = 1 as c_int;
    i = 0 as c_int;
    while (i as c_ulong)
        < (size_of::<[AVChannelLayout; 7]>() as c_ulong)
            .wrapping_div(size_of::<AVChannelLayout>() as c_ulong)
    {
        if av_channel_layout_compare(
            &mut (*avctx).ch_layout,
            &*aac_normal_chan_layouts.as_ptr().offset(i as isize),
        ) == 0
        {
            (*s).needs_pce = (*s).options.pce;
            break;
        } else {
            i += 1;
            i;
        }
    }
    if (*s).needs_pce != 0 {
        let mut buf: [c_char; 64] = [0; 64];
        i = 0 as c_int;
        while (i as c_ulong)
            < (size_of::<[AACPCEInfo; 29]>() as c_ulong)
                .wrapping_div(size_of::<AACPCEInfo>() as c_ulong)
        {
            if av_channel_layout_compare(
                &mut (*avctx).ch_layout,
                &(*aac_pce_configs.as_ptr().offset(i as isize)).layout,
            ) == 0
            {
                break;
            }
            i += 1;
            i;
        }
        av_channel_layout_describe(
            &mut (*avctx).ch_layout,
            buf.as_mut_ptr(),
            size_of::<[c_char; 64]>() as c_ulong,
        );
        if i as c_ulong
            == (size_of::<[AACPCEInfo; 29]>() as c_ulong)
                .wrapping_div(size_of::<AACPCEInfo>() as c_ulong)
        {
            av_log(
                avctx as *mut c_void,
                16 as c_int,
                b"Unsupported channel layout \"%s\"\n\0" as *const u8 as *const c_char,
                buf.as_mut_ptr(),
            );
            return -(22 as c_int);
        }
        av_log(
            avctx as *mut c_void,
            32 as c_int,
            b"Using a PCE to encode channel layout \"%s\"\n\0" as *const u8 as *const c_char,
            buf.as_mut_ptr(),
        );
        (*s).pce = aac_pce_configs[i as usize];
        (*s).reorder_map = ((*s).pce.reorder_map).as_mut_ptr();
        (*s).chan_map = ((*s).pce.config_map).as_mut_ptr();
    } else {
        (*s).reorder_map = (aac_chan_maps[((*s).channels - 1 as c_int) as usize]).as_ptr();
        (*s).chan_map = (aac_chan_configs[((*s).channels - 1 as c_int) as usize]).as_ptr();
    }
    if (*avctx).bit_rate == 0 {
        i = 1 as c_int;
        while i <= *((*s).chan_map).offset(0 as c_int as isize) as c_int {
            (*avctx).bit_rate +=
                (if *((*s).chan_map).offset(i as isize) as c_int == TYPE_CPE as c_int {
                    128000 as c_int
                } else if *((*s).chan_map).offset(i as isize) as c_int == TYPE_LFE as c_int {
                    16000 as c_int
                } else {
                    69000 as c_int
                }) as c_long;
            i += 1;
            i;
        }
    }
    i = 0 as c_int;
    while i < 16 as c_int {
        if (*avctx).sample_rate == ff_mpeg4audio_sample_rates[i as usize] {
            break;
        }
        i += 1;
        i;
    }
    (*s).samplerate_index = i;
    if (*s).samplerate_index == 16 as c_int
        || (*s).samplerate_index >= ff_aac_swb_size_1024_len
        || (*s).samplerate_index >= ff_aac_swb_size_128_len
    {
        av_log(
            avctx as *mut c_void,
            16 as c_int,
            b"Unsupported sample rate %d\n\0" as *const u8 as *const c_char,
            (*avctx).sample_rate,
        );
        return -(22 as c_int);
    }
    if 1024.0f64 * (*avctx).bit_rate as c_double / (*avctx).sample_rate as c_double
        > (6144 as c_int * (*s).channels) as c_double
    {
        av_log(
            avctx as *mut c_void,
            24 as c_int,
            b"Too many bits %f > %d per frame requested, clamping to max\n\0" as *const u8
                as *const c_char,
            1024.0f64 * (*avctx).bit_rate as c_double / (*avctx).sample_rate as c_double,
            6144 as c_int * (*s).channels,
        );
    }
    (*avctx).bit_rate = (if (6144 as c_int * (*s).channels) as c_double / 1024.0f64
        * (*avctx).sample_rate as c_double
        > (*avctx).bit_rate as c_double
    {
        (*avctx).bit_rate as c_double
    } else {
        (6144 as c_int * (*s).channels) as c_double / 1024.0f64 * (*avctx).sample_rate as c_double
    }) as c_long;
    (*avctx).profile = if (*avctx).profile == -(99 as c_int) {
        1 as c_int
    } else {
        (*avctx).profile
    };
    i = 0 as c_int;
    while (i as c_ulong)
        < (size_of::<[c_int; 4]>() as c_ulong).wrapping_div(size_of::<c_int>() as c_ulong)
    {
        if (*avctx).profile == aacenc_profiles[i as usize] {
            break;
        }
        i += 1;
        i;
    }
    if (*avctx).profile == 128 as c_int {
        (*avctx).profile = 1 as c_int;
        if (*s).options.pred != 0 {
            av_log(
                avctx as *mut c_void,
                16 as c_int,
                b"Main prediction unavailable in the \"mpeg2_aac_low\" profile\n\0" as *const u8
                    as *const c_char,
            );
            return -(22 as c_int);
        }
        if (*s).options.ltp != 0 {
            av_log(
                avctx as *mut c_void,
                16 as c_int,
                b"LTP prediction unavailable in the \"mpeg2_aac_low\" profile\n\0" as *const u8
                    as *const c_char,
            );
            return -(22 as c_int);
        }
        if (*s).options.pns != 0 {
            av_log(
                avctx as *mut c_void,
                24 as c_int,
                b"PNS unavailable in the \"mpeg2_aac_low\" profile, turning off\n\0" as *const u8
                    as *const c_char,
            );
        }
        (*s).options.pns = 0 as c_int;
    } else if (*avctx).profile == 3 as c_int {
        (*s).options.ltp = 1 as c_int;
        if (*s).options.pred != 0 {
            av_log(
                avctx as *mut c_void,
                16 as c_int,
                b"Main prediction unavailable in the \"aac_ltp\" profile\n\0" as *const u8
                    as *const c_char,
            );
            return -(22 as c_int);
        }
    } else if (*avctx).profile == 0 as c_int {
        (*s).options.pred = 1 as c_int;
        if (*s).options.ltp != 0 {
            av_log(
                avctx as *mut c_void,
                16 as c_int,
                b"LTP prediction unavailable in the \"aac_main\" profile\n\0" as *const u8
                    as *const c_char,
            );
            return -(22 as c_int);
        }
    } else if (*s).options.ltp != 0 {
        (*avctx).profile = 3 as c_int;
        av_log(
            avctx as *mut c_void,
            24 as c_int,
            b"Chainging profile to \"aac_ltp\"\n\0" as *const u8 as *const c_char,
        );
        if (*s).options.pred != 0 {
            av_log(
                avctx as *mut c_void,
                16 as c_int,
                b"Main prediction unavailable in the \"aac_ltp\" profile\n\0" as *const u8
                    as *const c_char,
            );
            return -(22 as c_int);
        }
    } else if (*s).options.pred != 0 {
        (*avctx).profile = 0 as c_int;
        av_log(
            avctx as *mut c_void,
            24 as c_int,
            b"Chainging profile to \"aac_main\"\n\0" as *const u8 as *const c_char,
        );
        if (*s).options.ltp != 0 {
            av_log(
                avctx as *mut c_void,
                16 as c_int,
                b"LTP prediction unavailable in the \"aac_main\" profile\n\0" as *const u8
                    as *const c_char,
            );
            return -(22 as c_int);
        }
    }
    (*s).profile = (*avctx).profile;
    (*s).coder = &*ff_aac_coders.as_ptr().offset((*s).options.coder as isize)
        as *const AACCoefficientsEncoder;
    if (*s).options.coder == AAC_CODER_ANMR as c_int {
        if (*avctx).strict_std_compliance > -(2 as c_int) {
            av_log(
                avctx as *mut c_void,
                16 as c_int,
                b"The ANMR coder is considered experimental, add -strict -2 to enable!\n\0"
                    as *const u8 as *const c_char,
            );
            return -(22 as c_int);
        }
        (*s).options.intensity_stereo = 0 as c_int;
        (*s).options.pns = 0 as c_int;
    }
    if (*s).options.ltp != 0 && (*avctx).strict_std_compliance > -(2 as c_int) {
        av_log(
            avctx as *mut c_void,
            16 as c_int,
            b"The LPT profile requires experimental compliance, add -strict -2 to enable!\n\0"
                as *const u8 as *const c_char,
        );
        return -(22 as c_int);
    }
    if (*s).channels > 3 as c_int {
        (*s).options.mid_side = 0 as c_int;
    }
    ff_aac_float_common_init();
    ret = dsp_init(avctx, s);
    if ret < 0 as c_int {
        return ret;
    }
    ret = alloc_buffers(avctx, s);
    if ret < 0 as c_int {
        return ret;
    }
    ret = put_audio_specific_config(avctx);
    if ret != 0 {
        return ret;
    }
    sizes[0 as c_int as usize] = *ff_aac_swb_size_1024
        .as_ptr()
        .offset((*s).samplerate_index as isize);
    sizes[1 as c_int as usize] = *ff_aac_swb_size_128
        .as_ptr()
        .offset((*s).samplerate_index as isize);
    lengths[0 as c_int as usize] = *ff_aac_num_swb_1024
        .as_ptr()
        .offset((*s).samplerate_index as isize) as c_int;
    lengths[1 as c_int as usize] = *ff_aac_num_swb_128
        .as_ptr()
        .offset((*s).samplerate_index as isize) as c_int;
    i = 0 as c_int;
    while i < *((*s).chan_map).offset(0 as c_int as isize) as c_int {
        grouping[i as usize] = (*((*s).chan_map).offset((i + 1 as c_int) as isize) as c_int
            == TYPE_CPE as c_int) as c_int as c_uchar;
        i += 1;
        i;
    }
    ret = ff_psy_init(
        &mut (*s).psy,
        avctx,
        2 as c_int,
        sizes.as_mut_ptr(),
        lengths.as_mut_ptr(),
        *((*s).chan_map).offset(0 as c_int as isize) as c_int,
        grouping.as_mut_ptr(),
    );
    if ret < 0 as c_int {
        return ret;
    }
    (*s).psypp = ff_psy_preprocess_init(avctx);
    ff_lpc_init(
        &mut (*s).lpc,
        2 as c_int * (*avctx).frame_size,
        20 as c_int,
        FF_LPC_TYPE_LEVINSON,
    );
    (*s).random_state = 0x1f2e3d4c as c_int;
    (*s).abs_pow34 = Some(abs_pow34_v);
    (*s).quant_bands = Some(quantize_bands);
    ff_af_queue_init(avctx, &mut (*s).afq);
    0 as c_int
}
static mut aacenc_options: [AVOption; 22] = [
    {
        AVOption {
            name: c"aac_coder".as_ptr(),
            help: c"Coding algorithm".as_ptr(),
            offset: 8 as c_ulong as c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed_0 {
                i64_0: AAC_CODER_TWOLOOP as c_int as c_long,
            },
            min: 0 as c_int as c_double,
            max: (AAC_CODER_NB as c_int - 1 as c_int) as c_double,
            flags: 1 as c_int | 8 as c_int,
            unit: c"coder".as_ptr(),
        }
    },
    {
        AVOption {
            name: c"anmr".as_ptr(),
            help: c"ANMR method".as_ptr(),
            offset: 0 as c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed_0 {
                i64_0: AAC_CODER_ANMR as c_int as c_long,
            },
            min: (-(2147483647 as c_int) - 1 as c_int) as c_double,
            max: 2147483647 as c_int as c_double,
            flags: 1 as c_int | 8 as c_int,
            unit: c"coder".as_ptr(),
        }
    },
    {
        AVOption {
            name: c"twoloop".as_ptr(),
            help: c"Two loop searching method".as_ptr(),
            offset: 0 as c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed_0 {
                i64_0: AAC_CODER_TWOLOOP as c_int as c_long,
            },
            min: (-(2147483647 as c_int) - 1 as c_int) as c_double,
            max: 2147483647 as c_int as c_double,
            flags: 1 as c_int | 8 as c_int,
            unit: c"coder".as_ptr(),
        }
    },
    {
        AVOption {
            name: c"fast".as_ptr(),
            help: c"Default fast search".as_ptr(),
            offset: 0 as c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed_0 {
                i64_0: AAC_CODER_FAST as c_int as c_long,
            },
            min: (-(2147483647 as c_int) - 1 as c_int) as c_double,
            max: 2147483647 as c_int as c_double,
            flags: 1 as c_int | 8 as c_int,
            unit: c"coder".as_ptr(),
        }
    },
    {
        AVOption {
            name: c"aac_ms".as_ptr(),
            help: c"Force M/S stereo coding".as_ptr(),
            offset: 32 as c_ulong as c_int,
            type_0: AV_OPT_TYPE_BOOL,
            default_val: C2RustUnnamed_0 {
                i64_0: -(1 as c_int) as c_long,
            },
            min: -(1 as c_int) as c_double,
            max: 1 as c_int as c_double,
            flags: 1 as c_int | 8 as c_int,
            unit: 0 as *const c_char,
        }
    },
    {
        AVOption {
            name: c"aac_is".as_ptr(),
            help: c"Intensity stereo coding".as_ptr(),
            offset: 36 as c_ulong as c_int,
            type_0: AV_OPT_TYPE_BOOL,
            default_val: C2RustUnnamed_0 {
                i64_0: 1 as c_int as c_long,
            },
            min: -(1 as c_int) as c_double,
            max: 1 as c_int as c_double,
            flags: 1 as c_int | 8 as c_int,
            unit: 0 as *const c_char,
        }
    },
    {
        AVOption {
            name: c"aac_pns".as_ptr(),
            help: c"Perceptual noise substitution".as_ptr(),
            offset: 12 as c_ulong as c_int,
            type_0: AV_OPT_TYPE_BOOL,
            default_val: C2RustUnnamed_0 {
                i64_0: 1 as c_int as c_long,
            },
            min: -(1 as c_int) as c_double,
            max: 1 as c_int as c_double,
            flags: 1 as c_int | 8 as c_int,
            unit: 0 as *const c_char,
        }
    },
    {
        AVOption {
            name: c"aac_tns".as_ptr(),
            help: c"Temporal noise shaping".as_ptr(),
            offset: 16 as c_ulong as c_int,
            type_0: AV_OPT_TYPE_BOOL,
            default_val: C2RustUnnamed_0 {
                i64_0: 1 as c_int as c_long,
            },
            min: -(1 as c_int) as c_double,
            max: 1 as c_int as c_double,
            flags: 1 as c_int | 8 as c_int,
            unit: 0 as *const c_char,
        }
    },
    {
        AVOption {
            name: c"aac_ltp".as_ptr(),
            help: c"Long term prediction".as_ptr(),
            offset: 20 as c_ulong as c_int,
            type_0: AV_OPT_TYPE_BOOL,
            default_val: C2RustUnnamed_0 {
                i64_0: 0 as c_int as c_long,
            },
            min: -(1 as c_int) as c_double,
            max: 1 as c_int as c_double,
            flags: 1 as c_int | 8 as c_int,
            unit: 0 as *const c_char,
        }
    },
    {
        AVOption {
            name: c"aac_pred".as_ptr(),
            help: c"AAC-Main prediction".as_ptr(),
            offset: 28 as c_ulong as c_int,
            type_0: AV_OPT_TYPE_BOOL,
            default_val: C2RustUnnamed_0 {
                i64_0: 0 as c_int as c_long,
            },
            min: -(1 as c_int) as c_double,
            max: 1 as c_int as c_double,
            flags: 1 as c_int | 8 as c_int,
            unit: 0 as *const c_char,
        }
    },
    {
        AVOption {
            name: c"aac_pce".as_ptr(),
            help: c"Forces the use of PCEs".as_ptr(),
            offset: 24 as c_ulong as c_int,
            type_0: AV_OPT_TYPE_BOOL,
            default_val: C2RustUnnamed_0 {
                i64_0: 0 as c_int as c_long,
            },
            min: -(1 as c_int) as c_double,
            max: 1 as c_int as c_double,
            flags: 1 as c_int | 8 as c_int,
            unit: 0 as *const c_char,
        }
    },
    {
        AVOption {
            name: c"aac_main".as_ptr(),
            help: 0 as *const c_char,
            offset: 0 as c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed_0 {
                i64_0: 0 as c_int as c_long,
            },
            min: (-(2147483647 as c_int) - 1 as c_int) as c_double,
            max: 2147483647 as c_int as c_double,
            flags: 1 as c_int | 8 as c_int,
            unit: c"avctx.profile".as_ptr(),
        }
    },
    {
        AVOption {
            name: c"aac_low".as_ptr(),
            help: 0 as *const c_char,
            offset: 0 as c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed_0 {
                i64_0: 1 as c_int as c_long,
            },
            min: (-(2147483647 as c_int) - 1 as c_int) as c_double,
            max: 2147483647 as c_int as c_double,
            flags: 1 as c_int | 8 as c_int,
            unit: c"avctx.profile".as_ptr(),
        }
    },
    {
        AVOption {
            name: c"aac_ssr".as_ptr(),
            help: 0 as *const c_char,
            offset: 0 as c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed_0 {
                i64_0: 2 as c_int as c_long,
            },
            min: (-(2147483647 as c_int) - 1 as c_int) as c_double,
            max: 2147483647 as c_int as c_double,
            flags: 1 as c_int | 8 as c_int,
            unit: c"avctx.profile".as_ptr(),
        }
    },
    {
        AVOption {
            name: c"aac_ltp".as_ptr(),
            help: 0 as *const c_char,
            offset: 0 as c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed_0 {
                i64_0: 3 as c_int as c_long,
            },
            min: (-(2147483647 as c_int) - 1 as c_int) as c_double,
            max: 2147483647 as c_int as c_double,
            flags: 1 as c_int | 8 as c_int,
            unit: c"avctx.profile".as_ptr(),
        }
    },
    {
        AVOption {
            name: c"aac_he".as_ptr(),
            help: 0 as *const c_char,
            offset: 0 as c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed_0 {
                i64_0: 4 as c_int as c_long,
            },
            min: (-(2147483647 as c_int) - 1 as c_int) as c_double,
            max: 2147483647 as c_int as c_double,
            flags: 1 as c_int | 8 as c_int,
            unit: c"avctx.profile".as_ptr(),
        }
    },
    {
        AVOption {
            name: c"aac_he_v2".as_ptr(),
            help: 0 as *const c_char,
            offset: 0 as c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed_0 {
                i64_0: 28 as c_int as c_long,
            },
            min: (-(2147483647 as c_int) - 1 as c_int) as c_double,
            max: 2147483647 as c_int as c_double,
            flags: 1 as c_int | 8 as c_int,
            unit: c"avctx.profile".as_ptr(),
        }
    },
    {
        AVOption {
            name: c"aac_ld".as_ptr(),
            help: 0 as *const c_char,
            offset: 0 as c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed_0 {
                i64_0: 22 as c_int as c_long,
            },
            min: (-(2147483647 as c_int) - 1 as c_int) as c_double,
            max: 2147483647 as c_int as c_double,
            flags: 1 as c_int | 8 as c_int,
            unit: c"avctx.profile".as_ptr(),
        }
    },
    {
        AVOption {
            name: c"aac_eld".as_ptr(),
            help: 0 as *const c_char,
            offset: 0 as c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed_0 {
                i64_0: 38 as c_int as c_long,
            },
            min: (-(2147483647 as c_int) - 1 as c_int) as c_double,
            max: 2147483647 as c_int as c_double,
            flags: 1 as c_int | 8 as c_int,
            unit: c"avctx.profile".as_ptr(),
        }
    },
    {
        AVOption {
            name: c"mpeg2_aac_low".as_ptr(),
            help: 0 as *const c_char,
            offset: 0 as c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed_0 {
                i64_0: 128 as c_int as c_long,
            },
            min: (-(2147483647 as c_int) - 1 as c_int) as c_double,
            max: 2147483647 as c_int as c_double,
            flags: 1 as c_int | 8 as c_int,
            unit: c"avctx.profile".as_ptr(),
        }
    },
    {
        AVOption {
            name: c"mpeg2_aac_he".as_ptr(),
            help: 0 as *const c_char,
            offset: 0 as c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed_0 {
                i64_0: 131 as c_int as c_long,
            },
            min: (-(2147483647 as c_int) - 1 as c_int) as c_double,
            max: 2147483647 as c_int as c_double,
            flags: 1 as c_int | 8 as c_int,
            unit: c"avctx.profile".as_ptr(),
        }
    },
    {
        AVOption {
            name: 0 as *const c_char,
            help: 0 as *const c_char,
            offset: 0,
            type_0: AV_OPT_TYPE_FLAGS,
            default_val: C2RustUnnamed_0 { i64_0: 0 },
            min: 0.,
            max: 0.,
            flags: 0,
            unit: 0 as *const c_char,
        }
    },
];
static mut aacenc_class: AVClass = unsafe {
    {
        AVClass {
            class_name: c"AAC encoder".as_ptr(),
            item_name: Some(av_default_item_name),
            option: aacenc_options.as_ptr(),
            version: (58 as c_int) << 16 as c_int | (32 as c_int) << 8 as c_int | 100 as c_int,
            log_level_offset_offset: 0,
            parent_log_context_offset: 0,
            category: AV_CLASS_CATEGORY_NA,
            get_category: None,
            query_ranges: None,
            child_next: None,
            child_class_iterate: None,
        }
    }
};
static mut aac_encode_defaults: [FFCodecDefault; 2] = [
    {
        FFCodecDefault {
            key: b"b\0" as *const u8 as *const c_char,
            value: b"0\0" as *const u8 as *const c_char,
        }
    },
    {
        FFCodecDefault {
            key: 0 as *const c_char,
            value: 0 as *const c_char,
        }
    },
];
#[no_mangle]
pub static mut ff_aac_encoder: FFCodec = FFCodec {
    p: AVCodec {
        name: 0 as *const c_char,
        long_name: 0 as *const c_char,
        type_0: AVMEDIA_TYPE_VIDEO,
        id: AV_CODEC_ID_NONE,
        capabilities: 0,
        max_lowres: 0,
        supported_framerates: 0 as *const AVRational,
        pix_fmts: 0 as *const AVPixelFormat,
        supported_samplerates: 0 as *const c_int,
        sample_fmts: 0 as *const AVSampleFormat,
        channel_layouts: 0 as *const c_ulong,
        priv_class: 0 as *const AVClass,
        profiles: 0 as *const AVProfile,
        wrapper_name: 0 as *const c_char,
        ch_layouts: 0 as *const AVChannelLayout,
    },
    caps_internal_cb_type: [0; 4],
    priv_data_size: 0,
    update_thread_context: None,
    update_thread_context_for_user: None,
    defaults: 0 as *const FFCodecDefault,
    init_static_data: None,
    init: None,
    cb: C2RustUnnamed_1 { decode: None },
    close: None,
    flush: None,
    bsfs: 0 as *const c_char,
    hw_configs: 0 as *const *const AVCodecHWConfigInternal,
    codec_tags: 0 as *const c_uint,
};
unsafe extern "C" fn run_static_initializers() {
    BUF_BITS = (8 as c_int as c_ulong).wrapping_mul(size_of::<BitBuf>() as c_ulong) as c_int;
    ff_aac_encoder = {
        let mut init = FFCodec {
            caps_internal_cb_type: [0; 4],
            p: {
                AVCodec {
                    name: c"aac".as_ptr(),
                    long_name: c"AAC (Advanced Audio Coding)".as_ptr(),
                    type_0: AVMEDIA_TYPE_AUDIO,
                    id: AV_CODEC_ID_AAC,
                    capabilities: (1 as c_int) << 1 as c_int
                        | (1 as c_int) << 5 as c_int
                        | (1 as c_int) << 6 as c_int,
                    max_lowres: 0,
                    supported_framerates: ptr::null::<AVRational>(),
                    pix_fmts: ptr::null::<AVPixelFormat>(),
                    supported_samplerates: ff_mpeg4audio_sample_rates.as_ptr(),
                    sample_fmts: [8, -1].as_ptr(),
                    channel_layouts: ptr::null::<c_ulong>(),
                    priv_class: &aacenc_class,
                    profiles: ptr::null::<AVProfile>(),
                    wrapper_name: ptr::null::<c_char>(),
                    ch_layouts: ptr::null::<AVChannelLayout>(),
                }
            },
            priv_data_size: size_of::<AACEncContext>() as c_ulong as c_int,
            update_thread_context: None,
            update_thread_context_for_user: None,
            defaults: aac_encode_defaults.as_ptr(),
            init_static_data: None,
            init: Some(aac_encode_init),
            cb: C2RustUnnamed_1 {
                encode: Some(aac_encode_frame),
            },
            close: Some(aac_encode_end),
            flush: None,
            bsfs: ptr::null::<c_char>(),
            hw_configs: ptr::null::<*const AVCodecHWConfigInternal>(),
            codec_tags: ptr::null::<c_uint>(),
        };
        init.set_caps_internal(((1 as c_int) << 1 as c_int) as c_uint);
        init.set_cb_type(FF_CODEC_CB_TYPE_ENCODE as c_int as c_uint);
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
