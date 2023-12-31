#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use libc::c_uchar;

static mut swb_size_128_96: [c_uchar; 12] = [4, 4, 4, 4, 4, 4, 8, 8, 8, 16, 28, 36];
static mut swb_size_128_64: [c_uchar; 12] = [4, 4, 4, 4, 4, 4, 8, 8, 8, 16, 28, 36];
static mut swb_size_128_48: [c_uchar; 14] = [4, 4, 4, 4, 4, 8, 8, 8, 12, 12, 12, 16, 16, 16];
static mut swb_size_128_24: [c_uchar; 15] = [4, 4, 4, 4, 4, 4, 4, 8, 8, 8, 12, 12, 16, 16, 20];
static mut swb_size_128_16: [c_uchar; 15] = [4, 4, 4, 4, 4, 4, 4, 4, 8, 8, 12, 12, 16, 20, 20];
static mut swb_size_128_8: [c_uchar; 15] = [4, 4, 4, 4, 4, 4, 4, 8, 8, 8, 8, 12, 16, 20, 20];
static mut swb_size_1024_96: [c_uchar; 41] = [
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 8, 8, 8, 8, 8, 12, 12, 12, 12, 12, 16, 16, 24, 28,
    36, 44, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
];
static mut swb_size_1024_64: [c_uchar; 47] = [
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 8, 8, 8, 8, 12, 12, 12, 16, 16, 16, 20, 24, 24, 28,
    36, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40,
];
static mut swb_size_1024_48: [c_uchar; 49] = [
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 8, 8, 8, 8, 8, 8, 8, 12, 12, 12, 12, 16, 16, 20, 20, 24, 24, 28,
    28, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 96,
];
static mut swb_size_1024_32: [c_uchar; 51] = [
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 8, 8, 8, 8, 8, 8, 8, 12, 12, 12, 12, 16, 16, 20, 20, 24, 24, 28,
    28, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
];
static mut swb_size_1024_24: [c_uchar; 47] = [
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 12, 12, 12, 12, 16, 16, 16, 20,
    20, 24, 24, 28, 28, 32, 36, 36, 40, 44, 48, 52, 52, 64, 64, 64, 64, 64,
];
static mut swb_size_1024_16: [c_uchar; 43] = [
    8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 12, 12, 12, 12, 12, 12, 12, 12, 12, 16, 16, 16, 16, 20, 20,
    20, 24, 24, 28, 28, 32, 36, 40, 40, 44, 48, 52, 56, 60, 64, 64, 64,
];
static mut swb_size_1024_8: [c_uchar; 40] = [
    12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 16, 16, 16, 16, 16, 16, 16, 20, 20, 20, 20,
    24, 24, 24, 28, 28, 32, 36, 36, 40, 44, 48, 52, 56, 60, 64, 80,
];

pub(crate) static mut ff_aac_swb_size_128: [*const c_uchar; 13] = unsafe {
    [
        swb_size_128_96.as_ptr(),
        swb_size_128_96.as_ptr(),
        swb_size_128_64.as_ptr(),
        swb_size_128_48.as_ptr(),
        swb_size_128_48.as_ptr(),
        swb_size_128_48.as_ptr(),
        swb_size_128_24.as_ptr(),
        swb_size_128_24.as_ptr(),
        swb_size_128_16.as_ptr(),
        swb_size_128_16.as_ptr(),
        swb_size_128_16.as_ptr(),
        swb_size_128_8.as_ptr(),
        swb_size_128_8.as_ptr(),
    ]
};

pub(crate) static mut ff_aac_swb_size_1024: [*const c_uchar; 13] = unsafe {
    [
        swb_size_1024_96.as_ptr(),
        swb_size_1024_96.as_ptr(),
        swb_size_1024_64.as_ptr(),
        swb_size_1024_48.as_ptr(),
        swb_size_1024_48.as_ptr(),
        swb_size_1024_32.as_ptr(),
        swb_size_1024_24.as_ptr(),
        swb_size_1024_24.as_ptr(),
        swb_size_1024_16.as_ptr(),
        swb_size_1024_16.as_ptr(),
        swb_size_1024_16.as_ptr(),
        swb_size_1024_8.as_ptr(),
        swb_size_1024_8.as_ptr(),
    ]
};
