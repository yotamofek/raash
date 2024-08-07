use ffmpeg_src_macro::ffmpeg_src;
use libc::c_uchar;

#[ffmpeg_src(file = "libavcodec/aacenctab.c", lines = 91..=97, name = "ff_aac_swb_size_128")]
pub(crate) const SWB_SIZE_128: [&[c_uchar]; 13] = {
    const _96: [c_uchar; 12] = [4, 4, 4, 4, 4, 4, 8, 8, 8, 16, 28, 36];
    const _64: [c_uchar; 12] = [4, 4, 4, 4, 4, 4, 8, 8, 8, 16, 28, 36];
    const _48: [c_uchar; 14] = [4, 4, 4, 4, 4, 8, 8, 8, 12, 12, 12, 16, 16, 16];
    const _24: [c_uchar; 15] = [4, 4, 4, 4, 4, 4, 4, 8, 8, 8, 12, 12, 16, 16, 20];
    const _16: [c_uchar; 15] = [4, 4, 4, 4, 4, 4, 4, 4, 8, 8, 12, 12, 16, 20, 20];
    const _8: [c_uchar; 15] = [4, 4, 4, 4, 4, 4, 4, 8, 8, 8, 8, 12, 16, 20, 20];

    [
        &_96, &_96, &_64, &_48, &_48, &_48, &_24, &_24, &_16, &_16, &_16, &_8, &_8,
    ]
};

#[ffmpeg_src(file = "libavcodec/aacenctab.c", lines = 99..=105, name = "ff_aac_swb_size_1024")]
pub(crate) const SWB_SIZE_1024: [&[c_uchar]; 13] = {
    const _96: [c_uchar; 41] = [
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 8, 8, 8, 8, 8, 12, 12, 12, 12, 12, 16, 16, 24,
        28, 36, 44, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    ];
    const _64: [c_uchar; 47] = [
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 8, 8, 8, 8, 12, 12, 12, 16, 16, 16, 20, 24, 24,
        28, 36, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40,
    ];
    const _48: [c_uchar; 49] = [
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 8, 8, 8, 8, 8, 8, 8, 12, 12, 12, 12, 16, 16, 20, 20, 24, 24,
        28, 28, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 96,
    ];
    const _32: [c_uchar; 51] = [
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 8, 8, 8, 8, 8, 8, 8, 12, 12, 12, 12, 16, 16, 20, 20, 24, 24,
        28, 28, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32,
    ];
    const _24: [c_uchar; 47] = [
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 12, 12, 12, 12, 16, 16, 16,
        20, 20, 24, 24, 28, 28, 32, 36, 36, 40, 44, 48, 52, 52, 64, 64, 64, 64, 64,
    ];
    const _16: [c_uchar; 43] = [
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 12, 12, 12, 12, 12, 12, 12, 12, 12, 16, 16, 16, 16, 20,
        20, 20, 24, 24, 28, 28, 32, 36, 40, 40, 44, 48, 52, 56, 60, 64, 64, 64,
    ];
    const _8: [c_uchar; 40] = [
        12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 16, 16, 16, 16, 16, 16, 16, 20, 20, 20,
        20, 24, 24, 24, 28, 28, 32, 36, 36, 40, 44, 48, 52, 56, 60, 64, 80,
    ];

    [
        &_96, &_96, &_64, &_48, &_48, &_32, &_24, &_24, &_16, &_16, &_16, &_8, &_8,
    ]
};
