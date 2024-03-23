use libc::c_uchar;

const SWB_SIZE_128_96: [c_uchar; 12] = [4, 4, 4, 4, 4, 4, 8, 8, 8, 16, 28, 36];
const SWB_SIZE_128_64: [c_uchar; 12] = [4, 4, 4, 4, 4, 4, 8, 8, 8, 16, 28, 36];
const SWB_SIZE_128_48: [c_uchar; 14] = [4, 4, 4, 4, 4, 8, 8, 8, 12, 12, 12, 16, 16, 16];
const SWB_SIZE_128_24: [c_uchar; 15] = [4, 4, 4, 4, 4, 4, 4, 8, 8, 8, 12, 12, 16, 16, 20];
const SWB_SIZE_128_16: [c_uchar; 15] = [4, 4, 4, 4, 4, 4, 4, 4, 8, 8, 12, 12, 16, 20, 20];
const SWB_SIZE_128_8: [c_uchar; 15] = [4, 4, 4, 4, 4, 4, 4, 8, 8, 8, 8, 12, 16, 20, 20];
const SWB_SIZE_1024_96: [c_uchar; 41] = [
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 8, 8, 8, 8, 8, 12, 12, 12, 12, 12, 16, 16, 24, 28,
    36, 44, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
];
const SWB_SIZE_1024_64: [c_uchar; 47] = [
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 8, 8, 8, 8, 12, 12, 12, 16, 16, 16, 20, 24, 24, 28,
    36, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40,
];
const SWB_SIZE_1024_48: [c_uchar; 49] = [
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 8, 8, 8, 8, 8, 8, 8, 12, 12, 12, 12, 16, 16, 20, 20, 24, 24, 28,
    28, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 96,
];
const SWB_SIZE_1024_32: [c_uchar; 51] = [
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 8, 8, 8, 8, 8, 8, 8, 12, 12, 12, 12, 16, 16, 20, 20, 24, 24, 28,
    28, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
];
const SWB_SIZE_1024_24: [c_uchar; 47] = [
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 12, 12, 12, 12, 16, 16, 16, 20,
    20, 24, 24, 28, 28, 32, 36, 36, 40, 44, 48, 52, 52, 64, 64, 64, 64, 64,
];
const SWB_SIZE_1024_16: [c_uchar; 43] = [
    8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 12, 12, 12, 12, 12, 12, 12, 12, 12, 16, 16, 16, 16, 20, 20,
    20, 24, 24, 28, 28, 32, 36, 40, 40, 44, 48, 52, 56, 60, 64, 64, 64,
];
const SWB_SIZE_1024_8: [c_uchar; 40] = [
    12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 16, 16, 16, 16, 16, 16, 16, 20, 20, 20, 20,
    24, 24, 24, 28, 28, 32, 36, 36, 40, 44, 48, 52, 56, 60, 64, 80,
];

pub(crate) const SWB_SIZE_128: [&[c_uchar]; 13] = {
    [
        &SWB_SIZE_128_96,
        &SWB_SIZE_128_96,
        &SWB_SIZE_128_64,
        &SWB_SIZE_128_48,
        &SWB_SIZE_128_48,
        &SWB_SIZE_128_48,
        &SWB_SIZE_128_24,
        &SWB_SIZE_128_24,
        &SWB_SIZE_128_16,
        &SWB_SIZE_128_16,
        &SWB_SIZE_128_16,
        &SWB_SIZE_128_8,
        &SWB_SIZE_128_8,
    ]
};

pub(crate) const SWB_SIZE_1024: [&[c_uchar]; 13] = {
    [
        &SWB_SIZE_1024_96,
        &SWB_SIZE_1024_96,
        &SWB_SIZE_1024_64,
        &SWB_SIZE_1024_48,
        &SWB_SIZE_1024_48,
        &SWB_SIZE_1024_32,
        &SWB_SIZE_1024_24,
        &SWB_SIZE_1024_24,
        &SWB_SIZE_1024_16,
        &SWB_SIZE_1024_16,
        &SWB_SIZE_1024_16,
        &SWB_SIZE_1024_8,
        &SWB_SIZE_1024_8,
    ]
};
