use std::{
    f64::consts::{FRAC_PI_2, PI},
    mem::size_of,
    ptr,
    sync::Once,
};

use ::libc;
use ffi::num::AVComplexInt32;
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_longlong, c_uchar, c_uint, c_ulong, c_ulonglong,
    c_void,
};

use super::{
    ff_tx_clear_ctx, ff_tx_decompose_length, ff_tx_gen_compound_mapping, ff_tx_gen_default_map,
    ff_tx_gen_inplace_map, ff_tx_gen_pfa_input_map, ff_tx_gen_ptwo_revtab, ff_tx_init_subtx,
};
use crate::types::*;
extern "C" {
    fn cos(_: c_double) -> c_double;
    fn sin(_: c_double) -> c_double;
    fn sqrt(_: c_double) -> c_double;
    fn fabs(_: c_double) -> c_double;
    fn llrintf(_: c_float) -> c_longlong;
    fn av_malloc_array(nmemb: c_ulong, size: c_ulong) -> *mut c_void;
    fn av_mallocz(size: c_ulong) -> *mut c_void;
    fn av_malloc(size: c_ulong) -> *mut c_void;
    fn memcpy(_: *mut c_void, _: *const c_void, _: c_ulong) -> *mut c_void;
}
pub type TXSample = c_int;
pub type TXUSample = c_uint;
pub type TXComplex = AVComplexInt32;

#[inline(always)]
unsafe extern "C" fn ff_ctz_c(v: c_int) -> c_int {
    static mut debruijn_ctz32: [c_uchar; 32] = [
        0 as c_int as c_uchar,
        1 as c_int as c_uchar,
        28 as c_int as c_uchar,
        2 as c_int as c_uchar,
        29 as c_int as c_uchar,
        14 as c_int as c_uchar,
        24 as c_int as c_uchar,
        3 as c_int as c_uchar,
        30 as c_int as c_uchar,
        22 as c_int as c_uchar,
        20 as c_int as c_uchar,
        15 as c_int as c_uchar,
        25 as c_int as c_uchar,
        17 as c_int as c_uchar,
        4 as c_int as c_uchar,
        8 as c_int as c_uchar,
        31 as c_int as c_uchar,
        27 as c_int as c_uchar,
        13 as c_int as c_uchar,
        23 as c_int as c_uchar,
        21 as c_int as c_uchar,
        19 as c_int as c_uchar,
        16 as c_int as c_uchar,
        7 as c_int as c_uchar,
        26 as c_int as c_uchar,
        12 as c_int as c_uchar,
        18 as c_int as c_uchar,
        6 as c_int as c_uchar,
        11 as c_int as c_uchar,
        5 as c_int as c_uchar,
        10 as c_int as c_uchar,
        9 as c_int as c_uchar,
    ];
    debruijn_ctz32[(((v & -v) as c_uint).wrapping_mul(0x77cb531 as c_uint) >> 27 as c_int) as usize]
        as c_int
}
#[inline(always)]
unsafe extern "C" fn av_clip64_c(a: c_long, amin: c_long, amax: c_long) -> c_long {
    if a < amin {
        amin
    } else if a > amax {
        return amax;
    } else {
        return a;
    }
}

pub static mut ff_tx_tab_8192_int32: [TXSample; 2049] = [0; 2049];

pub static mut ff_tx_tab_131072_int32: [TXSample; 32769] = [0; 32769];

pub static mut ff_tx_tab_1048576_int32: [TXSample; 262145] = [0; 262145];

pub static mut ff_tx_tab_65536_int32: [TXSample; 16385] = [0; 16385];

pub static mut ff_tx_tab_64_int32: [TXSample; 17] = [0; 17];

pub static mut ff_tx_tab_32768_int32: [TXSample; 8193] = [0; 8193];

pub static mut ff_tx_tab_2097152_int32: [TXSample; 524289] = [0; 524289];

pub static mut ff_tx_tab_16384_int32: [TXSample; 4097] = [0; 4097];

pub static mut ff_tx_tab_524288_int32: [TXSample; 131073] = [0; 131073];

pub static mut ff_tx_tab_262144_int32: [TXSample; 65537] = [0; 65537];

pub static mut ff_tx_tab_4096_int32: [TXSample; 1025] = [0; 1025];

pub static mut ff_tx_tab_2048_int32: [TXSample; 513] = [0; 513];

pub static mut ff_tx_tab_8_int32: [TXSample; 3] = [0; 3];

pub static mut ff_tx_tab_1024_int32: [TXSample; 257] = [0; 257];

pub static mut ff_tx_tab_16_int32: [TXSample; 5] = [0; 5];

pub static mut ff_tx_tab_512_int32: [TXSample; 129] = [0; 129];

pub static mut ff_tx_tab_32_int32: [TXSample; 9] = [0; 9];

pub static mut ff_tx_tab_256_int32: [TXSample; 65] = [0; 65];

pub static mut ff_tx_tab_128_int32: [TXSample; 33] = [0; 33];

pub static mut ff_tx_tab_53_int32: [TXSample; 12] = [0; 12];

pub static mut ff_tx_tab_7_int32: [TXSample; 6] = [0; 6];

pub static mut ff_tx_tab_9_int32: [TXSample; 8] = [0; 8];
#[cold]
unsafe extern "C" fn ff_tx_init_tab_2097152_int32() {
    let freq: c_double = 2. * PI / 2097152.;
    let mut tab: *mut TXSample = ff_tx_tab_2097152_int32.as_mut_ptr();
    let mut i: c_int = 0 as c_int;
    while i < 2097152 as c_int / 4 as c_int {
        let fresh0 = tab;
        tab = tab.offset(1);
        *fresh0 = av_clip64_c(
            llrintf((cos(i as c_double * freq) * 2147483648.0f64) as c_float) as c_long,
            (-(2147483647 as c_int) - 1 as c_int) as c_long,
            2147483647 as c_int as c_long,
        ) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as c_int;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_262144_int32() {
    let freq: c_double = 2. * PI / 262144.;
    let mut tab: *mut TXSample = ff_tx_tab_262144_int32.as_mut_ptr();
    let mut i: c_int = 0 as c_int;
    while i < 262144 as c_int / 4 as c_int {
        let fresh1 = tab;
        tab = tab.offset(1);
        *fresh1 = av_clip64_c(
            llrintf((cos(i as c_double * freq) * 2147483648.0f64) as c_float) as c_long,
            (-(2147483647 as c_int) - 1 as c_int) as c_long,
            2147483647 as c_int as c_long,
        ) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as c_int;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_16_int32() {
    let freq: c_double = 2. * PI / 16.;
    let mut tab: *mut TXSample = ff_tx_tab_16_int32.as_mut_ptr();
    let mut i: c_int = 0 as c_int;
    while i < 16 as c_int / 4 as c_int {
        let fresh2 = tab;
        tab = tab.offset(1);
        *fresh2 = av_clip64_c(
            llrintf((cos(i as c_double * freq) * 2147483648.0f64) as c_float) as c_long,
            (-(2147483647 as c_int) - 1 as c_int) as c_long,
            2147483647 as c_int as c_long,
        ) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as c_int;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_256_int32() {
    let freq: c_double = 2. * PI / 256.;
    let mut tab: *mut TXSample = ff_tx_tab_256_int32.as_mut_ptr();
    let mut i: c_int = 0 as c_int;
    while i < 256 as c_int / 4 as c_int {
        let fresh3 = tab;
        tab = tab.offset(1);
        *fresh3 = av_clip64_c(
            llrintf((cos(i as c_double * freq) * 2147483648.0f64) as c_float) as c_long,
            (-(2147483647 as c_int) - 1 as c_int) as c_long,
            2147483647 as c_int as c_long,
        ) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as c_int;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_8_int32() {
    let freq: c_double = 2. * PI / 8.;
    let mut tab: *mut TXSample = ff_tx_tab_8_int32.as_mut_ptr();
    let mut i: c_int = 0 as c_int;
    while i < 8 as c_int / 4 as c_int {
        let fresh4 = tab;
        tab = tab.offset(1);
        *fresh4 = av_clip64_c(
            llrintf((cos(i as c_double * freq) * 2147483648.0f64) as c_float) as c_long,
            (-(2147483647 as c_int) - 1 as c_int) as c_long,
            2147483647 as c_int as c_long,
        ) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as c_int;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_512_int32() {
    let freq: c_double = 2. * PI / 512.;
    let mut tab: *mut TXSample = ff_tx_tab_512_int32.as_mut_ptr();
    let mut i: c_int = 0 as c_int;
    while i < 512 as c_int / 4 as c_int {
        let fresh5 = tab;
        tab = tab.offset(1);
        *fresh5 = av_clip64_c(
            llrintf((cos(i as c_double * freq) * 2147483648.0f64) as c_float) as c_long,
            (-(2147483647 as c_int) - 1 as c_int) as c_long,
            2147483647 as c_int as c_long,
        ) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as c_int;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_32_int32() {
    let freq: c_double = 2. * PI / 32.;
    let mut tab: *mut TXSample = ff_tx_tab_32_int32.as_mut_ptr();
    let mut i: c_int = 0 as c_int;
    while i < 32 as c_int / 4 as c_int {
        let fresh6 = tab;
        tab = tab.offset(1);
        *fresh6 = av_clip64_c(
            llrintf((cos(i as c_double * freq) * 2147483648.0f64) as c_float) as c_long,
            (-(2147483647 as c_int) - 1 as c_int) as c_long,
            2147483647 as c_int as c_long,
        ) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as c_int;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_1024_int32() {
    let freq: c_double = 2. * PI / 1024.;
    let mut tab: *mut TXSample = ff_tx_tab_1024_int32.as_mut_ptr();
    let mut i: c_int = 0 as c_int;
    while i < 1024 as c_int / 4 as c_int {
        let fresh7 = tab;
        tab = tab.offset(1);
        *fresh7 = av_clip64_c(
            llrintf((cos(i as c_double * freq) * 2147483648.0f64) as c_float) as c_long,
            (-(2147483647 as c_int) - 1 as c_int) as c_long,
            2147483647 as c_int as c_long,
        ) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as c_int;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_2048_int32() {
    let freq: c_double = 2. * PI / 2048.;
    let mut tab: *mut TXSample = ff_tx_tab_2048_int32.as_mut_ptr();
    let mut i: c_int = 0 as c_int;
    while i < 2048 as c_int / 4 as c_int {
        let fresh8 = tab;
        tab = tab.offset(1);
        *fresh8 = av_clip64_c(
            llrintf((cos(i as c_double * freq) * 2147483648.0f64) as c_float) as c_long,
            (-(2147483647 as c_int) - 1 as c_int) as c_long,
            2147483647 as c_int as c_long,
        ) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as c_int;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_4096_int32() {
    let freq: c_double = 2. * PI / 4096.;
    let mut tab: *mut TXSample = ff_tx_tab_4096_int32.as_mut_ptr();
    let mut i: c_int = 0 as c_int;
    while i < 4096 as c_int / 4 as c_int {
        let fresh9 = tab;
        tab = tab.offset(1);
        *fresh9 = av_clip64_c(
            llrintf((cos(i as c_double * freq) * 2147483648.0f64) as c_float) as c_long,
            (-(2147483647 as c_int) - 1 as c_int) as c_long,
            2147483647 as c_int as c_long,
        ) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as c_int;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_8192_int32() {
    let freq: c_double = 2. * PI / 8192.;
    let mut tab: *mut TXSample = ff_tx_tab_8192_int32.as_mut_ptr();
    let mut i: c_int = 0 as c_int;
    while i < 8192 as c_int / 4 as c_int {
        let fresh10 = tab;
        tab = tab.offset(1);
        *fresh10 = av_clip64_c(
            llrintf((cos(i as c_double * freq) * 2147483648.0f64) as c_float) as c_long,
            (-(2147483647 as c_int) - 1 as c_int) as c_long,
            2147483647 as c_int as c_long,
        ) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as c_int;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_16384_int32() {
    let freq: c_double = 2. * PI / 16384.;
    let mut tab: *mut TXSample = ff_tx_tab_16384_int32.as_mut_ptr();
    let mut i: c_int = 0 as c_int;
    while i < 16384 as c_int / 4 as c_int {
        let fresh11 = tab;
        tab = tab.offset(1);
        *fresh11 = av_clip64_c(
            llrintf((cos(i as c_double * freq) * 2147483648.0f64) as c_float) as c_long,
            (-(2147483647 as c_int) - 1 as c_int) as c_long,
            2147483647 as c_int as c_long,
        ) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as c_int;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_32768_int32() {
    let freq: c_double = 2. * PI / 32768.;
    let mut tab: *mut TXSample = ff_tx_tab_32768_int32.as_mut_ptr();
    let mut i: c_int = 0 as c_int;
    while i < 32768 as c_int / 4 as c_int {
        let fresh12 = tab;
        tab = tab.offset(1);
        *fresh12 = av_clip64_c(
            llrintf((cos(i as c_double * freq) * 2147483648.0f64) as c_float) as c_long,
            (-(2147483647 as c_int) - 1 as c_int) as c_long,
            2147483647 as c_int as c_long,
        ) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as c_int;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_128_int32() {
    let freq: c_double = 2. * PI / 128.;
    let mut tab: *mut TXSample = ff_tx_tab_128_int32.as_mut_ptr();
    let mut i: c_int = 0 as c_int;
    while i < 128 as c_int / 4 as c_int {
        let fresh13 = tab;
        tab = tab.offset(1);
        *fresh13 = av_clip64_c(
            llrintf((cos(i as c_double * freq) * 2147483648.0f64) as c_float) as c_long,
            (-(2147483647 as c_int) - 1 as c_int) as c_long,
            2147483647 as c_int as c_long,
        ) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as c_int;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_131072_int32() {
    let freq: c_double = 2. * PI / 131072.;
    let mut tab: *mut TXSample = ff_tx_tab_131072_int32.as_mut_ptr();
    let mut i: c_int = 0 as c_int;
    while i < 131072 as c_int / 4 as c_int {
        let fresh14 = tab;
        tab = tab.offset(1);
        *fresh14 = av_clip64_c(
            llrintf((cos(i as c_double * freq) * 2147483648.0f64) as c_float) as c_long,
            (-(2147483647 as c_int) - 1 as c_int) as c_long,
            2147483647 as c_int as c_long,
        ) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as c_int;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_65536_int32() {
    let freq: c_double = 2. * PI / 65536.;
    let mut tab: *mut TXSample = ff_tx_tab_65536_int32.as_mut_ptr();
    let mut i: c_int = 0 as c_int;
    while i < 65536 as c_int / 4 as c_int {
        let fresh15 = tab;
        tab = tab.offset(1);
        *fresh15 = av_clip64_c(
            llrintf((cos(i as c_double * freq) * 2147483648.0f64) as c_float) as c_long,
            (-(2147483647 as c_int) - 1 as c_int) as c_long,
            2147483647 as c_int as c_long,
        ) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as c_int;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_524288_int32() {
    let freq: c_double = 2. * PI / 524288.;
    let mut tab: *mut TXSample = ff_tx_tab_524288_int32.as_mut_ptr();
    let mut i: c_int = 0 as c_int;
    while i < 524288 as c_int / 4 as c_int {
        let fresh16 = tab;
        tab = tab.offset(1);
        *fresh16 = av_clip64_c(
            llrintf((cos(i as c_double * freq) * 2147483648.0f64) as c_float) as c_long,
            (-(2147483647 as c_int) - 1 as c_int) as c_long,
            2147483647 as c_int as c_long,
        ) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as c_int;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_64_int32() {
    let freq: c_double = 2. * PI / 64.;
    let mut tab: *mut TXSample = ff_tx_tab_64_int32.as_mut_ptr();
    let mut i: c_int = 0 as c_int;
    while i < 64 as c_int / 4 as c_int {
        let fresh17 = tab;
        tab = tab.offset(1);
        *fresh17 = av_clip64_c(
            llrintf((cos(i as c_double * freq) * 2147483648.0f64) as c_float) as c_long,
            (-(2147483647 as c_int) - 1 as c_int) as c_long,
            2147483647 as c_int as c_long,
        ) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as c_int;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_1048576_int32() {
    let freq: c_double = 2. * PI / 1048576.;
    let mut tab: *mut TXSample = ff_tx_tab_1048576_int32.as_mut_ptr();
    let mut i: c_int = 0 as c_int;
    while i < 1048576 as c_int / 4 as c_int {
        let fresh18 = tab;
        tab = tab.offset(1);
        *fresh18 = av_clip64_c(
            llrintf((cos(i as c_double * freq) * 2147483648.0f64) as c_float) as c_long,
            (-(2147483647 as c_int) - 1 as c_int) as c_long,
            2147483647 as c_int as c_long,
        ) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as c_int;
}
static mut sr_tabs_init_funcs: [Option<unsafe extern "C" fn() -> ()>; 19] = unsafe {
    [
        Some(ff_tx_init_tab_8_int32 as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_16_int32 as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_32_int32 as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_64_int32 as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_128_int32 as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_256_int32 as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_512_int32 as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_1024_int32 as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_2048_int32 as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_4096_int32 as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_8192_int32 as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_16384_int32 as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_32768_int32 as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_65536_int32 as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_131072_int32 as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_262144_int32 as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_524288_int32 as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_1048576_int32 as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_2097152_int32 as unsafe extern "C" fn() -> ()),
    ]
};
static mut sr_tabs_init_once: [Once; 19] = [
    Once::new(),
    Once::new(),
    Once::new(),
    Once::new(),
    Once::new(),
    Once::new(),
    Once::new(),
    Once::new(),
    Once::new(),
    Once::new(),
    Once::new(),
    Once::new(),
    Once::new(),
    Once::new(),
    Once::new(),
    Once::new(),
    Once::new(),
    Once::new(),
    Once::new(),
];
#[cold]
unsafe extern "C" fn ff_tx_init_tab_53_int32() {
    ff_tx_tab_53_int32[0 as c_int as usize] = av_clip64_c(
        llrintf((cos(2. * PI / 5.) * 2147483648.0f64) as c_float) as c_long,
        (-(2147483647 as c_int) - 1 as c_int) as c_long,
        2147483647 as c_int as c_long,
    ) as TXSample;
    ff_tx_tab_53_int32[1 as c_int as usize] = av_clip64_c(
        llrintf((cos(2. * PI / 5.) * 2147483648.0f64) as c_float) as c_long,
        (-(2147483647 as c_int) - 1 as c_int) as c_long,
        2147483647 as c_int as c_long,
    ) as TXSample;
    ff_tx_tab_53_int32[2 as c_int as usize] = av_clip64_c(
        llrintf((cos(2. * PI / 10.) * 2147483648.0f64) as c_float) as c_long,
        (-(2147483647 as c_int) - 1 as c_int) as c_long,
        2147483647 as c_int as c_long,
    ) as TXSample;
    ff_tx_tab_53_int32[3 as c_int as usize] = av_clip64_c(
        llrintf((cos(2. * PI / 10.) * 2147483648.0f64) as c_float) as c_long,
        (-(2147483647 as c_int) - 1 as c_int) as c_long,
        2147483647 as c_int as c_long,
    ) as TXSample;
    ff_tx_tab_53_int32[4 as c_int as usize] = av_clip64_c(
        llrintf((sin(2. * PI / 5.) * 2147483648.0f64) as c_float) as c_long,
        (-(2147483647 as c_int) - 1 as c_int) as c_long,
        2147483647 as c_int as c_long,
    ) as TXSample;
    ff_tx_tab_53_int32[5 as c_int as usize] = av_clip64_c(
        llrintf((sin(2. * PI / 5.) * 2147483648.0f64) as c_float) as c_long,
        (-(2147483647 as c_int) - 1 as c_int) as c_long,
        2147483647 as c_int as c_long,
    ) as TXSample;
    ff_tx_tab_53_int32[6 as c_int as usize] = av_clip64_c(
        llrintf((sin(2. * PI / 10.) * 2147483648.0f64) as c_float) as c_long,
        (-(2147483647 as c_int) - 1 as c_int) as c_long,
        2147483647 as c_int as c_long,
    ) as TXSample;
    ff_tx_tab_53_int32[7 as c_int as usize] = av_clip64_c(
        llrintf((sin(2. * PI / 10.) * 2147483648.0f64) as c_float) as c_long,
        (-(2147483647 as c_int) - 1 as c_int) as c_long,
        2147483647 as c_int as c_long,
    ) as TXSample;
    ff_tx_tab_53_int32[8 as c_int as usize] = av_clip64_c(
        llrintf((cos(2. * PI / 12.) * 2147483648.0f64) as c_float) as c_long,
        (-(2147483647 as c_int) - 1 as c_int) as c_long,
        2147483647 as c_int as c_long,
    ) as TXSample;
    ff_tx_tab_53_int32[9 as c_int as usize] = av_clip64_c(
        llrintf((cos(2. * PI / 12.) * 2147483648.0f64) as c_float) as c_long,
        (-(2147483647 as c_int) - 1 as c_int) as c_long,
        2147483647 as c_int as c_long,
    ) as TXSample;
    ff_tx_tab_53_int32[10 as c_int as usize] = av_clip64_c(
        llrintf((cos(2. * PI / 6.) * 2147483648.0f64) as c_float) as c_long,
        (-(2147483647 as c_int) - 1 as c_int) as c_long,
        2147483647 as c_int as c_long,
    ) as TXSample;
    ff_tx_tab_53_int32[11 as c_int as usize] = av_clip64_c(
        llrintf((cos(8. * PI / 6.) * 2147483648.0f64) as c_float) as c_long,
        (-(2147483647 as c_int) - 1 as c_int) as c_long,
        2147483647 as c_int as c_long,
    ) as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_7_int32() {
    ff_tx_tab_7_int32[0 as c_int as usize] = av_clip64_c(
        llrintf((cos(2. * PI / 7.) * 2147483648.0f64) as c_float) as c_long,
        (-(2147483647 as c_int) - 1 as c_int) as c_long,
        2147483647 as c_int as c_long,
    ) as TXSample;
    ff_tx_tab_7_int32[1 as c_int as usize] = av_clip64_c(
        llrintf((sin(2. * PI / 7.) * 2147483648.0f64) as c_float) as c_long,
        (-(2147483647 as c_int) - 1 as c_int) as c_long,
        2147483647 as c_int as c_long,
    ) as TXSample;
    ff_tx_tab_7_int32[2 as c_int as usize] = av_clip64_c(
        llrintf((sin(2. * PI / 28.) * 2147483648.0f64) as c_float) as c_long,
        (-(2147483647 as c_int) - 1 as c_int) as c_long,
        2147483647 as c_int as c_long,
    ) as TXSample;
    ff_tx_tab_7_int32[3 as c_int as usize] = av_clip64_c(
        llrintf((cos(2. * PI / 28.) * 2147483648.0f64) as c_float) as c_long,
        (-(2147483647 as c_int) - 1 as c_int) as c_long,
        2147483647 as c_int as c_long,
    ) as TXSample;
    ff_tx_tab_7_int32[4 as c_int as usize] = av_clip64_c(
        llrintf((cos(2. * PI / 14.) * 2147483648.0f64) as c_float) as c_long,
        (-(2147483647 as c_int) - 1 as c_int) as c_long,
        2147483647 as c_int as c_long,
    ) as TXSample;
    ff_tx_tab_7_int32[5 as c_int as usize] = av_clip64_c(
        llrintf((sin(2. * PI / 14.) * 2147483648.0f64) as c_float) as c_long,
        (-(2147483647 as c_int) - 1 as c_int) as c_long,
        2147483647 as c_int as c_long,
    ) as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_9_int32() {
    ff_tx_tab_9_int32[0 as c_int as usize] = av_clip64_c(
        llrintf((cos(2. * PI / 3.) * 2147483648.0f64) as c_float) as c_long,
        (-(2147483647 as c_int) - 1 as c_int) as c_long,
        2147483647 as c_int as c_long,
    ) as TXSample;
    ff_tx_tab_9_int32[1 as c_int as usize] = av_clip64_c(
        llrintf((sin(2. * PI / 3.) * 2147483648.0f64) as c_float) as c_long,
        (-(2147483647 as c_int) - 1 as c_int) as c_long,
        2147483647 as c_int as c_long,
    ) as TXSample;
    ff_tx_tab_9_int32[2 as c_int as usize] = av_clip64_c(
        llrintf((cos(2. * PI / 9.) * 2147483648.0f64) as c_float) as c_long,
        (-(2147483647 as c_int) - 1 as c_int) as c_long,
        2147483647 as c_int as c_long,
    ) as TXSample;
    ff_tx_tab_9_int32[3 as c_int as usize] = av_clip64_c(
        llrintf((sin(2. * PI / 9.) * 2147483648.0f64) as c_float) as c_long,
        (-(2147483647 as c_int) - 1 as c_int) as c_long,
        2147483647 as c_int as c_long,
    ) as TXSample;
    ff_tx_tab_9_int32[4 as c_int as usize] = av_clip64_c(
        llrintf((cos(2. * PI / 36.) * 2147483648.0f64) as c_float) as c_long,
        (-(2147483647 as c_int) - 1 as c_int) as c_long,
        2147483647 as c_int as c_long,
    ) as TXSample;
    ff_tx_tab_9_int32[5 as c_int as usize] = av_clip64_c(
        llrintf((sin(2. * PI / 36.) * 2147483648.0f64) as c_float) as c_long,
        (-(2147483647 as c_int) - 1 as c_int) as c_long,
        2147483647 as c_int as c_long,
    ) as TXSample;
    ff_tx_tab_9_int32[6 as c_int as usize] =
        ff_tx_tab_9_int32[2 as c_int as usize] + ff_tx_tab_9_int32[5 as c_int as usize];
    ff_tx_tab_9_int32[7 as c_int as usize] =
        ff_tx_tab_9_int32[3 as c_int as usize] - ff_tx_tab_9_int32[4 as c_int as usize];
}
static mut nptwo_tabs_init_data: [FFTabInitData; 3] = unsafe {
    [
        {
            FFTabInitData {
                func: Some(ff_tx_init_tab_53_int32 as unsafe extern "C" fn() -> ()),
                factors: [15 as c_int, 5 as c_int, 3 as c_int, 0],
            }
        },
        {
            FFTabInitData {
                func: Some(ff_tx_init_tab_9_int32 as unsafe extern "C" fn() -> ()),
                factors: [9 as c_int, 0, 0, 0],
            }
        },
        {
            FFTabInitData {
                func: Some(ff_tx_init_tab_7_int32 as unsafe extern "C" fn() -> ()),
                factors: [7 as c_int, 0, 0, 0],
            }
        },
    ]
};
static mut nptwo_tabs_init_once: [Once; 3] = [Once::new(), Once::new(), Once::new()];

#[cold]
pub unsafe extern "C" fn ff_tx_init_tabs_int32(mut len: c_int) {
    let factor_2: c_int = ff_ctz_c(len);
    if factor_2 != 0 {
        let idx: c_int = factor_2 - 3 as c_int;
        let mut i: c_int = 0 as c_int;
        while i <= idx {
            sr_tabs_init_once[i as usize].call_once(|| sr_tabs_init_funcs[i as usize].unwrap()());
            i += 1;
            i;
        }
        len >>= factor_2;
    }
    let mut i_0: c_int = 0 as c_int;
    while (i_0 as c_ulong)
        < (size_of::<[FFTabInitData; 3]>() as c_ulong)
            .wrapping_div(size_of::<FFTabInitData>() as c_ulong)
    {
        let mut f: c_int = 0;
        let mut f_idx: c_int = 0 as c_int;
        if len <= 1 as c_int {
            return;
        }
        loop {
            let fresh19 = f_idx;
            f_idx += 1;
            f = nptwo_tabs_init_data[i_0 as usize].factors[fresh19 as usize];
            if !(f != 0) {
                break;
            }
            if f % len != 0 {
                continue;
            }
            nptwo_tabs_init_once[i_0 as usize]
                .call_once(|| nptwo_tabs_init_data[i_0 as usize].func.unwrap()());
            len /= f;
            break;
        }
        i_0 += 1;
        i_0;
    }
}
#[inline(always)]
unsafe extern "C" fn fft3(out: *mut TXComplex, in_0: *mut TXComplex, stride: ptrdiff_t) {
    let mut tmp: [TXComplex; 3] = [TXComplex { re: 0, im: 0 }; 3];
    let tab: *const TXSample = ff_tx_tab_53_int32.as_mut_ptr();
    let mut mtmp: [c_long; 4] = [0; 4];
    tmp[0 as c_int as usize] = *in_0.offset(0 as c_int as isize);
    tmp[1 as c_int as usize].re = ((*in_0.offset(1 as c_int as isize)).im as c_uint)
        .wrapping_sub((*in_0.offset(2 as c_int as isize)).im as c_uint)
        as c_int;
    tmp[2 as c_int as usize].im = ((*in_0.offset(1 as c_int as isize)).im as c_uint)
        .wrapping_add((*in_0.offset(2 as c_int as isize)).im as c_uint)
        as c_int;
    tmp[1 as c_int as usize].im = ((*in_0.offset(1 as c_int as isize)).re as c_uint)
        .wrapping_sub((*in_0.offset(2 as c_int as isize)).re as c_uint)
        as c_int;
    tmp[2 as c_int as usize].re = ((*in_0.offset(1 as c_int as isize)).re as c_uint)
        .wrapping_add((*in_0.offset(2 as c_int as isize)).re as c_uint)
        as c_int;
    (*out.offset((0 as c_int as c_long * stride) as isize)).re =
        (tmp[0 as c_int as usize].re as c_long + tmp[2 as c_int as usize].re as c_long) as c_int;
    (*out.offset((0 as c_int as c_long * stride) as isize)).im =
        (tmp[0 as c_int as usize].im as c_long + tmp[2 as c_int as usize].im as c_long) as c_int;
    mtmp[0 as c_int as usize] =
        *tab.offset(8 as c_int as isize) as c_long * tmp[1 as c_int as usize].re as c_long;
    mtmp[1 as c_int as usize] =
        *tab.offset(9 as c_int as isize) as c_long * tmp[1 as c_int as usize].im as c_long;
    mtmp[2 as c_int as usize] =
        *tab.offset(10 as c_int as isize) as c_long * tmp[2 as c_int as usize].re as c_long;
    mtmp[3 as c_int as usize] =
        *tab.offset(10 as c_int as isize) as c_long * tmp[2 as c_int as usize].im as c_long;
    (*out.offset((1 as c_int as c_long * stride) as isize)).re = (tmp[0 as c_int as usize].re
        as c_long
        - (mtmp[2 as c_int as usize] + mtmp[0 as c_int as usize] + 0x40000000 as c_int as c_long
            >> 31 as c_int))
        as c_int;
    (*out.offset((1 as c_int as c_long * stride) as isize)).im = (tmp[0 as c_int as usize].im
        as c_long
        - (mtmp[3 as c_int as usize] - mtmp[1 as c_int as usize] + 0x40000000 as c_int as c_long
            >> 31 as c_int))
        as c_int;
    (*out.offset((2 as c_int as c_long * stride) as isize)).re = (tmp[0 as c_int as usize].re
        as c_long
        - (mtmp[2 as c_int as usize] - mtmp[0 as c_int as usize] + 0x40000000 as c_int as c_long
            >> 31 as c_int))
        as c_int;
    (*out.offset((2 as c_int as c_long * stride) as isize)).im = (tmp[0 as c_int as usize].im
        as c_long
        - (mtmp[3 as c_int as usize] + mtmp[1 as c_int as usize] + 0x40000000 as c_int as c_long
            >> 31 as c_int))
        as c_int;
}
#[inline(always)]
unsafe extern "C" fn fft5(out: *mut TXComplex, in_0: *mut TXComplex, stride: ptrdiff_t) {
    let mut dc: TXComplex = TXComplex { re: 0, im: 0 };
    let mut z0: [TXComplex; 4] = [TXComplex { re: 0, im: 0 }; 4];
    let mut t: [TXComplex; 6] = [TXComplex { re: 0, im: 0 }; 6];
    let tab: *const TXSample = ff_tx_tab_53_int32.as_mut_ptr();
    dc = *in_0.offset(0 as c_int as isize);
    t[1 as c_int as usize].im = ((*in_0.offset(1 as c_int as isize)).re as c_uint)
        .wrapping_sub((*in_0.offset(4 as c_int as isize)).re as c_uint)
        as c_int;
    t[0 as c_int as usize].re = ((*in_0.offset(1 as c_int as isize)).re as c_uint)
        .wrapping_add((*in_0.offset(4 as c_int as isize)).re as c_uint)
        as c_int;
    t[1 as c_int as usize].re = ((*in_0.offset(1 as c_int as isize)).im as c_uint)
        .wrapping_sub((*in_0.offset(4 as c_int as isize)).im as c_uint)
        as c_int;
    t[0 as c_int as usize].im = ((*in_0.offset(1 as c_int as isize)).im as c_uint)
        .wrapping_add((*in_0.offset(4 as c_int as isize)).im as c_uint)
        as c_int;
    t[3 as c_int as usize].im = ((*in_0.offset(2 as c_int as isize)).re as c_uint)
        .wrapping_sub((*in_0.offset(3 as c_int as isize)).re as c_uint)
        as c_int;
    t[2 as c_int as usize].re = ((*in_0.offset(2 as c_int as isize)).re as c_uint)
        .wrapping_add((*in_0.offset(3 as c_int as isize)).re as c_uint)
        as c_int;
    t[3 as c_int as usize].re = ((*in_0.offset(2 as c_int as isize)).im as c_uint)
        .wrapping_sub((*in_0.offset(3 as c_int as isize)).im as c_uint)
        as c_int;
    t[2 as c_int as usize].im = ((*in_0.offset(2 as c_int as isize)).im as c_uint)
        .wrapping_add((*in_0.offset(3 as c_int as isize)).im as c_uint)
        as c_int;
    (*out.offset((0 as c_int as c_long * stride) as isize)).re = (dc.re as c_uint)
        .wrapping_add(t[0 as c_int as usize].re as TXUSample)
        .wrapping_add(t[2 as c_int as usize].re as c_uint)
        as c_int;
    (*out.offset((0 as c_int as c_long * stride) as isize)).im = (dc.im as c_uint)
        .wrapping_add(t[0 as c_int as usize].im as TXUSample)
        .wrapping_add(t[2 as c_int as usize].im as c_uint)
        as c_int;
    let mut accu: c_long = 0;
    accu = t[2 as c_int as usize].re as c_long * *tab.offset(0 as c_int as isize) as c_long;
    accu -= t[0 as c_int as usize].re as c_long * *tab.offset(2 as c_int as isize) as c_long;
    t[4 as c_int as usize].re = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
    accu = t[0 as c_int as usize].re as c_long * *tab.offset(0 as c_int as isize) as c_long;
    accu -= t[2 as c_int as usize].re as c_long * *tab.offset(2 as c_int as isize) as c_long;
    t[0 as c_int as usize].re = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
    let mut accu_0: c_long = 0;
    accu_0 = t[2 as c_int as usize].im as c_long * *tab.offset(0 as c_int as isize) as c_long;
    accu_0 -= t[0 as c_int as usize].im as c_long * *tab.offset(2 as c_int as isize) as c_long;
    t[4 as c_int as usize].im = (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
    accu_0 = t[0 as c_int as usize].im as c_long * *tab.offset(0 as c_int as isize) as c_long;
    accu_0 -= t[2 as c_int as usize].im as c_long * *tab.offset(2 as c_int as isize) as c_long;
    t[0 as c_int as usize].im = (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
    let mut accu_1: c_long = 0;
    accu_1 = t[3 as c_int as usize].re as c_long * *tab.offset(4 as c_int as isize) as c_long;
    accu_1 -= t[1 as c_int as usize].re as c_long * *tab.offset(6 as c_int as isize) as c_long;
    t[5 as c_int as usize].re = (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
    accu_1 = t[1 as c_int as usize].re as c_long * *tab.offset(4 as c_int as isize) as c_long;
    accu_1 += t[3 as c_int as usize].re as c_long * *tab.offset(6 as c_int as isize) as c_long;
    t[1 as c_int as usize].re = (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
    let mut accu_2: c_long = 0;
    accu_2 = t[3 as c_int as usize].im as c_long * *tab.offset(4 as c_int as isize) as c_long;
    accu_2 -= t[1 as c_int as usize].im as c_long * *tab.offset(6 as c_int as isize) as c_long;
    t[5 as c_int as usize].im = (accu_2 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
    accu_2 = t[1 as c_int as usize].im as c_long * *tab.offset(4 as c_int as isize) as c_long;
    accu_2 += t[3 as c_int as usize].im as c_long * *tab.offset(6 as c_int as isize) as c_long;
    t[1 as c_int as usize].im = (accu_2 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
    z0[0 as c_int as usize].re = (t[0 as c_int as usize].re as c_uint)
        .wrapping_sub(t[1 as c_int as usize].re as c_uint)
        as c_int;
    z0[3 as c_int as usize].re = (t[0 as c_int as usize].re as c_uint)
        .wrapping_add(t[1 as c_int as usize].re as c_uint)
        as c_int;
    z0[0 as c_int as usize].im = (t[0 as c_int as usize].im as c_uint)
        .wrapping_sub(t[1 as c_int as usize].im as c_uint)
        as c_int;
    z0[3 as c_int as usize].im = (t[0 as c_int as usize].im as c_uint)
        .wrapping_add(t[1 as c_int as usize].im as c_uint)
        as c_int;
    z0[2 as c_int as usize].re = (t[4 as c_int as usize].re as c_uint)
        .wrapping_sub(t[5 as c_int as usize].re as c_uint)
        as c_int;
    z0[1 as c_int as usize].re = (t[4 as c_int as usize].re as c_uint)
        .wrapping_add(t[5 as c_int as usize].re as c_uint)
        as c_int;
    z0[2 as c_int as usize].im = (t[4 as c_int as usize].im as c_uint)
        .wrapping_sub(t[5 as c_int as usize].im as c_uint)
        as c_int;
    z0[1 as c_int as usize].im = (t[4 as c_int as usize].im as c_uint)
        .wrapping_add(t[5 as c_int as usize].im as c_uint)
        as c_int;
    (*out.offset((1 as c_int as c_long * stride) as isize)).re =
        (dc.re as c_uint).wrapping_add(z0[3 as c_int as usize].re as TXUSample) as c_int;
    (*out.offset((1 as c_int as c_long * stride) as isize)).im =
        (dc.im as c_uint).wrapping_add(z0[0 as c_int as usize].im as TXUSample) as c_int;
    (*out.offset((2 as c_int as c_long * stride) as isize)).re =
        (dc.re as c_uint).wrapping_add(z0[2 as c_int as usize].re as TXUSample) as c_int;
    (*out.offset((2 as c_int as c_long * stride) as isize)).im =
        (dc.im as c_uint).wrapping_add(z0[1 as c_int as usize].im as TXUSample) as c_int;
    (*out.offset((3 as c_int as c_long * stride) as isize)).re =
        (dc.re as c_uint).wrapping_add(z0[1 as c_int as usize].re as TXUSample) as c_int;
    (*out.offset((3 as c_int as c_long * stride) as isize)).im =
        (dc.im as c_uint).wrapping_add(z0[2 as c_int as usize].im as TXUSample) as c_int;
    (*out.offset((4 as c_int as c_long * stride) as isize)).re =
        (dc.re as c_uint).wrapping_add(z0[0 as c_int as usize].re as TXUSample) as c_int;
    (*out.offset((4 as c_int as c_long * stride) as isize)).im =
        (dc.im as c_uint).wrapping_add(z0[3 as c_int as usize].im as TXUSample) as c_int;
}
#[inline(always)]
unsafe extern "C" fn fft5_m1(out: *mut TXComplex, in_0: *mut TXComplex, stride: ptrdiff_t) {
    let mut dc: TXComplex = TXComplex { re: 0, im: 0 };
    let mut z0: [TXComplex; 4] = [TXComplex { re: 0, im: 0 }; 4];
    let mut t: [TXComplex; 6] = [TXComplex { re: 0, im: 0 }; 6];
    let tab: *const TXSample = ff_tx_tab_53_int32.as_mut_ptr();
    dc = *in_0.offset(0 as c_int as isize);
    t[1 as c_int as usize].im = ((*in_0.offset(1 as c_int as isize)).re as c_uint)
        .wrapping_sub((*in_0.offset(4 as c_int as isize)).re as c_uint)
        as c_int;
    t[0 as c_int as usize].re = ((*in_0.offset(1 as c_int as isize)).re as c_uint)
        .wrapping_add((*in_0.offset(4 as c_int as isize)).re as c_uint)
        as c_int;
    t[1 as c_int as usize].re = ((*in_0.offset(1 as c_int as isize)).im as c_uint)
        .wrapping_sub((*in_0.offset(4 as c_int as isize)).im as c_uint)
        as c_int;
    t[0 as c_int as usize].im = ((*in_0.offset(1 as c_int as isize)).im as c_uint)
        .wrapping_add((*in_0.offset(4 as c_int as isize)).im as c_uint)
        as c_int;
    t[3 as c_int as usize].im = ((*in_0.offset(2 as c_int as isize)).re as c_uint)
        .wrapping_sub((*in_0.offset(3 as c_int as isize)).re as c_uint)
        as c_int;
    t[2 as c_int as usize].re = ((*in_0.offset(2 as c_int as isize)).re as c_uint)
        .wrapping_add((*in_0.offset(3 as c_int as isize)).re as c_uint)
        as c_int;
    t[3 as c_int as usize].re = ((*in_0.offset(2 as c_int as isize)).im as c_uint)
        .wrapping_sub((*in_0.offset(3 as c_int as isize)).im as c_uint)
        as c_int;
    t[2 as c_int as usize].im = ((*in_0.offset(2 as c_int as isize)).im as c_uint)
        .wrapping_add((*in_0.offset(3 as c_int as isize)).im as c_uint)
        as c_int;
    (*out.offset((0 as c_int as c_long * stride) as isize)).re = (dc.re as c_uint)
        .wrapping_add(t[0 as c_int as usize].re as TXUSample)
        .wrapping_add(t[2 as c_int as usize].re as c_uint)
        as c_int;
    (*out.offset((0 as c_int as c_long * stride) as isize)).im = (dc.im as c_uint)
        .wrapping_add(t[0 as c_int as usize].im as TXUSample)
        .wrapping_add(t[2 as c_int as usize].im as c_uint)
        as c_int;
    let mut accu: c_long = 0;
    accu = t[2 as c_int as usize].re as c_long * *tab.offset(0 as c_int as isize) as c_long;
    accu -= t[0 as c_int as usize].re as c_long * *tab.offset(2 as c_int as isize) as c_long;
    t[4 as c_int as usize].re = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
    accu = t[0 as c_int as usize].re as c_long * *tab.offset(0 as c_int as isize) as c_long;
    accu -= t[2 as c_int as usize].re as c_long * *tab.offset(2 as c_int as isize) as c_long;
    t[0 as c_int as usize].re = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
    let mut accu_0: c_long = 0;
    accu_0 = t[2 as c_int as usize].im as c_long * *tab.offset(0 as c_int as isize) as c_long;
    accu_0 -= t[0 as c_int as usize].im as c_long * *tab.offset(2 as c_int as isize) as c_long;
    t[4 as c_int as usize].im = (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
    accu_0 = t[0 as c_int as usize].im as c_long * *tab.offset(0 as c_int as isize) as c_long;
    accu_0 -= t[2 as c_int as usize].im as c_long * *tab.offset(2 as c_int as isize) as c_long;
    t[0 as c_int as usize].im = (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
    let mut accu_1: c_long = 0;
    accu_1 = t[3 as c_int as usize].re as c_long * *tab.offset(4 as c_int as isize) as c_long;
    accu_1 -= t[1 as c_int as usize].re as c_long * *tab.offset(6 as c_int as isize) as c_long;
    t[5 as c_int as usize].re = (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
    accu_1 = t[1 as c_int as usize].re as c_long * *tab.offset(4 as c_int as isize) as c_long;
    accu_1 += t[3 as c_int as usize].re as c_long * *tab.offset(6 as c_int as isize) as c_long;
    t[1 as c_int as usize].re = (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
    let mut accu_2: c_long = 0;
    accu_2 = t[3 as c_int as usize].im as c_long * *tab.offset(4 as c_int as isize) as c_long;
    accu_2 -= t[1 as c_int as usize].im as c_long * *tab.offset(6 as c_int as isize) as c_long;
    t[5 as c_int as usize].im = (accu_2 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
    accu_2 = t[1 as c_int as usize].im as c_long * *tab.offset(4 as c_int as isize) as c_long;
    accu_2 += t[3 as c_int as usize].im as c_long * *tab.offset(6 as c_int as isize) as c_long;
    t[1 as c_int as usize].im = (accu_2 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
    z0[0 as c_int as usize].re = (t[0 as c_int as usize].re as c_uint)
        .wrapping_sub(t[1 as c_int as usize].re as c_uint)
        as c_int;
    z0[3 as c_int as usize].re = (t[0 as c_int as usize].re as c_uint)
        .wrapping_add(t[1 as c_int as usize].re as c_uint)
        as c_int;
    z0[0 as c_int as usize].im = (t[0 as c_int as usize].im as c_uint)
        .wrapping_sub(t[1 as c_int as usize].im as c_uint)
        as c_int;
    z0[3 as c_int as usize].im = (t[0 as c_int as usize].im as c_uint)
        .wrapping_add(t[1 as c_int as usize].im as c_uint)
        as c_int;
    z0[2 as c_int as usize].re = (t[4 as c_int as usize].re as c_uint)
        .wrapping_sub(t[5 as c_int as usize].re as c_uint)
        as c_int;
    z0[1 as c_int as usize].re = (t[4 as c_int as usize].re as c_uint)
        .wrapping_add(t[5 as c_int as usize].re as c_uint)
        as c_int;
    z0[2 as c_int as usize].im = (t[4 as c_int as usize].im as c_uint)
        .wrapping_sub(t[5 as c_int as usize].im as c_uint)
        as c_int;
    z0[1 as c_int as usize].im = (t[4 as c_int as usize].im as c_uint)
        .wrapping_add(t[5 as c_int as usize].im as c_uint)
        as c_int;
    (*out.offset((6 as c_int as c_long * stride) as isize)).re =
        (dc.re as c_uint).wrapping_add(z0[3 as c_int as usize].re as TXUSample) as c_int;
    (*out.offset((6 as c_int as c_long * stride) as isize)).im =
        (dc.im as c_uint).wrapping_add(z0[0 as c_int as usize].im as TXUSample) as c_int;
    (*out.offset((12 as c_int as c_long * stride) as isize)).re =
        (dc.re as c_uint).wrapping_add(z0[2 as c_int as usize].re as TXUSample) as c_int;
    (*out.offset((12 as c_int as c_long * stride) as isize)).im =
        (dc.im as c_uint).wrapping_add(z0[1 as c_int as usize].im as TXUSample) as c_int;
    (*out.offset((3 as c_int as c_long * stride) as isize)).re =
        (dc.re as c_uint).wrapping_add(z0[1 as c_int as usize].re as TXUSample) as c_int;
    (*out.offset((3 as c_int as c_long * stride) as isize)).im =
        (dc.im as c_uint).wrapping_add(z0[2 as c_int as usize].im as TXUSample) as c_int;
    (*out.offset((9 as c_int as c_long * stride) as isize)).re =
        (dc.re as c_uint).wrapping_add(z0[0 as c_int as usize].re as TXUSample) as c_int;
    (*out.offset((9 as c_int as c_long * stride) as isize)).im =
        (dc.im as c_uint).wrapping_add(z0[3 as c_int as usize].im as TXUSample) as c_int;
}
#[inline(always)]
unsafe extern "C" fn fft5_m2(out: *mut TXComplex, in_0: *mut TXComplex, stride: ptrdiff_t) {
    let mut dc: TXComplex = TXComplex { re: 0, im: 0 };
    let mut z0: [TXComplex; 4] = [TXComplex { re: 0, im: 0 }; 4];
    let mut t: [TXComplex; 6] = [TXComplex { re: 0, im: 0 }; 6];
    let tab: *const TXSample = ff_tx_tab_53_int32.as_mut_ptr();
    dc = *in_0.offset(0 as c_int as isize);
    t[1 as c_int as usize].im = ((*in_0.offset(1 as c_int as isize)).re as c_uint)
        .wrapping_sub((*in_0.offset(4 as c_int as isize)).re as c_uint)
        as c_int;
    t[0 as c_int as usize].re = ((*in_0.offset(1 as c_int as isize)).re as c_uint)
        .wrapping_add((*in_0.offset(4 as c_int as isize)).re as c_uint)
        as c_int;
    t[1 as c_int as usize].re = ((*in_0.offset(1 as c_int as isize)).im as c_uint)
        .wrapping_sub((*in_0.offset(4 as c_int as isize)).im as c_uint)
        as c_int;
    t[0 as c_int as usize].im = ((*in_0.offset(1 as c_int as isize)).im as c_uint)
        .wrapping_add((*in_0.offset(4 as c_int as isize)).im as c_uint)
        as c_int;
    t[3 as c_int as usize].im = ((*in_0.offset(2 as c_int as isize)).re as c_uint)
        .wrapping_sub((*in_0.offset(3 as c_int as isize)).re as c_uint)
        as c_int;
    t[2 as c_int as usize].re = ((*in_0.offset(2 as c_int as isize)).re as c_uint)
        .wrapping_add((*in_0.offset(3 as c_int as isize)).re as c_uint)
        as c_int;
    t[3 as c_int as usize].re = ((*in_0.offset(2 as c_int as isize)).im as c_uint)
        .wrapping_sub((*in_0.offset(3 as c_int as isize)).im as c_uint)
        as c_int;
    t[2 as c_int as usize].im = ((*in_0.offset(2 as c_int as isize)).im as c_uint)
        .wrapping_add((*in_0.offset(3 as c_int as isize)).im as c_uint)
        as c_int;
    (*out.offset((10 as c_int as c_long * stride) as isize)).re = (dc.re as c_uint)
        .wrapping_add(t[0 as c_int as usize].re as TXUSample)
        .wrapping_add(t[2 as c_int as usize].re as c_uint)
        as c_int;
    (*out.offset((10 as c_int as c_long * stride) as isize)).im = (dc.im as c_uint)
        .wrapping_add(t[0 as c_int as usize].im as TXUSample)
        .wrapping_add(t[2 as c_int as usize].im as c_uint)
        as c_int;
    let mut accu: c_long = 0;
    accu = t[2 as c_int as usize].re as c_long * *tab.offset(0 as c_int as isize) as c_long;
    accu -= t[0 as c_int as usize].re as c_long * *tab.offset(2 as c_int as isize) as c_long;
    t[4 as c_int as usize].re = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
    accu = t[0 as c_int as usize].re as c_long * *tab.offset(0 as c_int as isize) as c_long;
    accu -= t[2 as c_int as usize].re as c_long * *tab.offset(2 as c_int as isize) as c_long;
    t[0 as c_int as usize].re = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
    let mut accu_0: c_long = 0;
    accu_0 = t[2 as c_int as usize].im as c_long * *tab.offset(0 as c_int as isize) as c_long;
    accu_0 -= t[0 as c_int as usize].im as c_long * *tab.offset(2 as c_int as isize) as c_long;
    t[4 as c_int as usize].im = (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
    accu_0 = t[0 as c_int as usize].im as c_long * *tab.offset(0 as c_int as isize) as c_long;
    accu_0 -= t[2 as c_int as usize].im as c_long * *tab.offset(2 as c_int as isize) as c_long;
    t[0 as c_int as usize].im = (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
    let mut accu_1: c_long = 0;
    accu_1 = t[3 as c_int as usize].re as c_long * *tab.offset(4 as c_int as isize) as c_long;
    accu_1 -= t[1 as c_int as usize].re as c_long * *tab.offset(6 as c_int as isize) as c_long;
    t[5 as c_int as usize].re = (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
    accu_1 = t[1 as c_int as usize].re as c_long * *tab.offset(4 as c_int as isize) as c_long;
    accu_1 += t[3 as c_int as usize].re as c_long * *tab.offset(6 as c_int as isize) as c_long;
    t[1 as c_int as usize].re = (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
    let mut accu_2: c_long = 0;
    accu_2 = t[3 as c_int as usize].im as c_long * *tab.offset(4 as c_int as isize) as c_long;
    accu_2 -= t[1 as c_int as usize].im as c_long * *tab.offset(6 as c_int as isize) as c_long;
    t[5 as c_int as usize].im = (accu_2 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
    accu_2 = t[1 as c_int as usize].im as c_long * *tab.offset(4 as c_int as isize) as c_long;
    accu_2 += t[3 as c_int as usize].im as c_long * *tab.offset(6 as c_int as isize) as c_long;
    t[1 as c_int as usize].im = (accu_2 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
    z0[0 as c_int as usize].re = (t[0 as c_int as usize].re as c_uint)
        .wrapping_sub(t[1 as c_int as usize].re as c_uint)
        as c_int;
    z0[3 as c_int as usize].re = (t[0 as c_int as usize].re as c_uint)
        .wrapping_add(t[1 as c_int as usize].re as c_uint)
        as c_int;
    z0[0 as c_int as usize].im = (t[0 as c_int as usize].im as c_uint)
        .wrapping_sub(t[1 as c_int as usize].im as c_uint)
        as c_int;
    z0[3 as c_int as usize].im = (t[0 as c_int as usize].im as c_uint)
        .wrapping_add(t[1 as c_int as usize].im as c_uint)
        as c_int;
    z0[2 as c_int as usize].re = (t[4 as c_int as usize].re as c_uint)
        .wrapping_sub(t[5 as c_int as usize].re as c_uint)
        as c_int;
    z0[1 as c_int as usize].re = (t[4 as c_int as usize].re as c_uint)
        .wrapping_add(t[5 as c_int as usize].re as c_uint)
        as c_int;
    z0[2 as c_int as usize].im = (t[4 as c_int as usize].im as c_uint)
        .wrapping_sub(t[5 as c_int as usize].im as c_uint)
        as c_int;
    z0[1 as c_int as usize].im = (t[4 as c_int as usize].im as c_uint)
        .wrapping_add(t[5 as c_int as usize].im as c_uint)
        as c_int;
    (*out.offset((1 as c_int as c_long * stride) as isize)).re =
        (dc.re as c_uint).wrapping_add(z0[3 as c_int as usize].re as TXUSample) as c_int;
    (*out.offset((1 as c_int as c_long * stride) as isize)).im =
        (dc.im as c_uint).wrapping_add(z0[0 as c_int as usize].im as TXUSample) as c_int;
    (*out.offset((7 as c_int as c_long * stride) as isize)).re =
        (dc.re as c_uint).wrapping_add(z0[2 as c_int as usize].re as TXUSample) as c_int;
    (*out.offset((7 as c_int as c_long * stride) as isize)).im =
        (dc.im as c_uint).wrapping_add(z0[1 as c_int as usize].im as TXUSample) as c_int;
    (*out.offset((13 as c_int as c_long * stride) as isize)).re =
        (dc.re as c_uint).wrapping_add(z0[1 as c_int as usize].re as TXUSample) as c_int;
    (*out.offset((13 as c_int as c_long * stride) as isize)).im =
        (dc.im as c_uint).wrapping_add(z0[2 as c_int as usize].im as TXUSample) as c_int;
    (*out.offset((4 as c_int as c_long * stride) as isize)).re =
        (dc.re as c_uint).wrapping_add(z0[0 as c_int as usize].re as TXUSample) as c_int;
    (*out.offset((4 as c_int as c_long * stride) as isize)).im =
        (dc.im as c_uint).wrapping_add(z0[3 as c_int as usize].im as TXUSample) as c_int;
}
#[inline(always)]
unsafe extern "C" fn fft5_m3(out: *mut TXComplex, in_0: *mut TXComplex, stride: ptrdiff_t) {
    let mut dc: TXComplex = TXComplex { re: 0, im: 0 };
    let mut z0: [TXComplex; 4] = [TXComplex { re: 0, im: 0 }; 4];
    let mut t: [TXComplex; 6] = [TXComplex { re: 0, im: 0 }; 6];
    let tab: *const TXSample = ff_tx_tab_53_int32.as_mut_ptr();
    dc = *in_0.offset(0 as c_int as isize);
    t[1 as c_int as usize].im = ((*in_0.offset(1 as c_int as isize)).re as c_uint)
        .wrapping_sub((*in_0.offset(4 as c_int as isize)).re as c_uint)
        as c_int;
    t[0 as c_int as usize].re = ((*in_0.offset(1 as c_int as isize)).re as c_uint)
        .wrapping_add((*in_0.offset(4 as c_int as isize)).re as c_uint)
        as c_int;
    t[1 as c_int as usize].re = ((*in_0.offset(1 as c_int as isize)).im as c_uint)
        .wrapping_sub((*in_0.offset(4 as c_int as isize)).im as c_uint)
        as c_int;
    t[0 as c_int as usize].im = ((*in_0.offset(1 as c_int as isize)).im as c_uint)
        .wrapping_add((*in_0.offset(4 as c_int as isize)).im as c_uint)
        as c_int;
    t[3 as c_int as usize].im = ((*in_0.offset(2 as c_int as isize)).re as c_uint)
        .wrapping_sub((*in_0.offset(3 as c_int as isize)).re as c_uint)
        as c_int;
    t[2 as c_int as usize].re = ((*in_0.offset(2 as c_int as isize)).re as c_uint)
        .wrapping_add((*in_0.offset(3 as c_int as isize)).re as c_uint)
        as c_int;
    t[3 as c_int as usize].re = ((*in_0.offset(2 as c_int as isize)).im as c_uint)
        .wrapping_sub((*in_0.offset(3 as c_int as isize)).im as c_uint)
        as c_int;
    t[2 as c_int as usize].im = ((*in_0.offset(2 as c_int as isize)).im as c_uint)
        .wrapping_add((*in_0.offset(3 as c_int as isize)).im as c_uint)
        as c_int;
    (*out.offset((5 as c_int as c_long * stride) as isize)).re = (dc.re as c_uint)
        .wrapping_add(t[0 as c_int as usize].re as TXUSample)
        .wrapping_add(t[2 as c_int as usize].re as c_uint)
        as c_int;
    (*out.offset((5 as c_int as c_long * stride) as isize)).im = (dc.im as c_uint)
        .wrapping_add(t[0 as c_int as usize].im as TXUSample)
        .wrapping_add(t[2 as c_int as usize].im as c_uint)
        as c_int;
    let mut accu: c_long = 0;
    accu = t[2 as c_int as usize].re as c_long * *tab.offset(0 as c_int as isize) as c_long;
    accu -= t[0 as c_int as usize].re as c_long * *tab.offset(2 as c_int as isize) as c_long;
    t[4 as c_int as usize].re = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
    accu = t[0 as c_int as usize].re as c_long * *tab.offset(0 as c_int as isize) as c_long;
    accu -= t[2 as c_int as usize].re as c_long * *tab.offset(2 as c_int as isize) as c_long;
    t[0 as c_int as usize].re = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
    let mut accu_0: c_long = 0;
    accu_0 = t[2 as c_int as usize].im as c_long * *tab.offset(0 as c_int as isize) as c_long;
    accu_0 -= t[0 as c_int as usize].im as c_long * *tab.offset(2 as c_int as isize) as c_long;
    t[4 as c_int as usize].im = (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
    accu_0 = t[0 as c_int as usize].im as c_long * *tab.offset(0 as c_int as isize) as c_long;
    accu_0 -= t[2 as c_int as usize].im as c_long * *tab.offset(2 as c_int as isize) as c_long;
    t[0 as c_int as usize].im = (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
    let mut accu_1: c_long = 0;
    accu_1 = t[3 as c_int as usize].re as c_long * *tab.offset(4 as c_int as isize) as c_long;
    accu_1 -= t[1 as c_int as usize].re as c_long * *tab.offset(6 as c_int as isize) as c_long;
    t[5 as c_int as usize].re = (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
    accu_1 = t[1 as c_int as usize].re as c_long * *tab.offset(4 as c_int as isize) as c_long;
    accu_1 += t[3 as c_int as usize].re as c_long * *tab.offset(6 as c_int as isize) as c_long;
    t[1 as c_int as usize].re = (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
    let mut accu_2: c_long = 0;
    accu_2 = t[3 as c_int as usize].im as c_long * *tab.offset(4 as c_int as isize) as c_long;
    accu_2 -= t[1 as c_int as usize].im as c_long * *tab.offset(6 as c_int as isize) as c_long;
    t[5 as c_int as usize].im = (accu_2 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
    accu_2 = t[1 as c_int as usize].im as c_long * *tab.offset(4 as c_int as isize) as c_long;
    accu_2 += t[3 as c_int as usize].im as c_long * *tab.offset(6 as c_int as isize) as c_long;
    t[1 as c_int as usize].im = (accu_2 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
    z0[0 as c_int as usize].re = (t[0 as c_int as usize].re as c_uint)
        .wrapping_sub(t[1 as c_int as usize].re as c_uint)
        as c_int;
    z0[3 as c_int as usize].re = (t[0 as c_int as usize].re as c_uint)
        .wrapping_add(t[1 as c_int as usize].re as c_uint)
        as c_int;
    z0[0 as c_int as usize].im = (t[0 as c_int as usize].im as c_uint)
        .wrapping_sub(t[1 as c_int as usize].im as c_uint)
        as c_int;
    z0[3 as c_int as usize].im = (t[0 as c_int as usize].im as c_uint)
        .wrapping_add(t[1 as c_int as usize].im as c_uint)
        as c_int;
    z0[2 as c_int as usize].re = (t[4 as c_int as usize].re as c_uint)
        .wrapping_sub(t[5 as c_int as usize].re as c_uint)
        as c_int;
    z0[1 as c_int as usize].re = (t[4 as c_int as usize].re as c_uint)
        .wrapping_add(t[5 as c_int as usize].re as c_uint)
        as c_int;
    z0[2 as c_int as usize].im = (t[4 as c_int as usize].im as c_uint)
        .wrapping_sub(t[5 as c_int as usize].im as c_uint)
        as c_int;
    z0[1 as c_int as usize].im = (t[4 as c_int as usize].im as c_uint)
        .wrapping_add(t[5 as c_int as usize].im as c_uint)
        as c_int;
    (*out.offset((11 as c_int as c_long * stride) as isize)).re =
        (dc.re as c_uint).wrapping_add(z0[3 as c_int as usize].re as TXUSample) as c_int;
    (*out.offset((11 as c_int as c_long * stride) as isize)).im =
        (dc.im as c_uint).wrapping_add(z0[0 as c_int as usize].im as TXUSample) as c_int;
    (*out.offset((2 as c_int as c_long * stride) as isize)).re =
        (dc.re as c_uint).wrapping_add(z0[2 as c_int as usize].re as TXUSample) as c_int;
    (*out.offset((2 as c_int as c_long * stride) as isize)).im =
        (dc.im as c_uint).wrapping_add(z0[1 as c_int as usize].im as TXUSample) as c_int;
    (*out.offset((8 as c_int as c_long * stride) as isize)).re =
        (dc.re as c_uint).wrapping_add(z0[1 as c_int as usize].re as TXUSample) as c_int;
    (*out.offset((8 as c_int as c_long * stride) as isize)).im =
        (dc.im as c_uint).wrapping_add(z0[2 as c_int as usize].im as TXUSample) as c_int;
    (*out.offset((14 as c_int as c_long * stride) as isize)).re =
        (dc.re as c_uint).wrapping_add(z0[0 as c_int as usize].re as TXUSample) as c_int;
    (*out.offset((14 as c_int as c_long * stride) as isize)).im =
        (dc.im as c_uint).wrapping_add(z0[3 as c_int as usize].im as TXUSample) as c_int;
}
#[inline(always)]
unsafe extern "C" fn fft7(out: *mut TXComplex, in_0: *mut TXComplex, stride: ptrdiff_t) {
    let mut dc: TXComplex = TXComplex { re: 0, im: 0 };
    let mut t: [TXComplex; 6] = [TXComplex { re: 0, im: 0 }; 6];
    let mut z: [TXComplex; 3] = [TXComplex { re: 0, im: 0 }; 3];
    let tab: *const TXComplex = ff_tx_tab_7_int32.as_mut_ptr() as *const TXComplex;
    let mut mtmp: [c_long; 12] = [0; 12];
    dc = *in_0.offset(0 as c_int as isize);
    t[1 as c_int as usize].re = ((*in_0.offset(1 as c_int as isize)).re as c_uint)
        .wrapping_sub((*in_0.offset(6 as c_int as isize)).re as c_uint)
        as c_int;
    t[0 as c_int as usize].re = ((*in_0.offset(1 as c_int as isize)).re as c_uint)
        .wrapping_add((*in_0.offset(6 as c_int as isize)).re as c_uint)
        as c_int;
    t[1 as c_int as usize].im = ((*in_0.offset(1 as c_int as isize)).im as c_uint)
        .wrapping_sub((*in_0.offset(6 as c_int as isize)).im as c_uint)
        as c_int;
    t[0 as c_int as usize].im = ((*in_0.offset(1 as c_int as isize)).im as c_uint)
        .wrapping_add((*in_0.offset(6 as c_int as isize)).im as c_uint)
        as c_int;
    t[3 as c_int as usize].re = ((*in_0.offset(2 as c_int as isize)).re as c_uint)
        .wrapping_sub((*in_0.offset(5 as c_int as isize)).re as c_uint)
        as c_int;
    t[2 as c_int as usize].re = ((*in_0.offset(2 as c_int as isize)).re as c_uint)
        .wrapping_add((*in_0.offset(5 as c_int as isize)).re as c_uint)
        as c_int;
    t[3 as c_int as usize].im = ((*in_0.offset(2 as c_int as isize)).im as c_uint)
        .wrapping_sub((*in_0.offset(5 as c_int as isize)).im as c_uint)
        as c_int;
    t[2 as c_int as usize].im = ((*in_0.offset(2 as c_int as isize)).im as c_uint)
        .wrapping_add((*in_0.offset(5 as c_int as isize)).im as c_uint)
        as c_int;
    t[5 as c_int as usize].re = ((*in_0.offset(3 as c_int as isize)).re as c_uint)
        .wrapping_sub((*in_0.offset(4 as c_int as isize)).re as c_uint)
        as c_int;
    t[4 as c_int as usize].re = ((*in_0.offset(3 as c_int as isize)).re as c_uint)
        .wrapping_add((*in_0.offset(4 as c_int as isize)).re as c_uint)
        as c_int;
    t[5 as c_int as usize].im = ((*in_0.offset(3 as c_int as isize)).im as c_uint)
        .wrapping_sub((*in_0.offset(4 as c_int as isize)).im as c_uint)
        as c_int;
    t[4 as c_int as usize].im = ((*in_0.offset(3 as c_int as isize)).im as c_uint)
        .wrapping_add((*in_0.offset(4 as c_int as isize)).im as c_uint)
        as c_int;
    (*out.offset((0 as c_int as c_long * stride) as isize)).re =
        dc.re + t[0 as c_int as usize].re + t[2 as c_int as usize].re + t[4 as c_int as usize].re;
    (*out.offset((0 as c_int as c_long * stride) as isize)).im =
        dc.im + t[0 as c_int as usize].im + t[2 as c_int as usize].im + t[4 as c_int as usize].im;
    mtmp[0 as c_int as usize] = (*tab.offset(0 as c_int as isize)).re as c_long
        * t[0 as c_int as usize].re as c_long
        - (*tab.offset(2 as c_int as isize)).re as c_long * t[4 as c_int as usize].re as c_long;
    mtmp[1 as c_int as usize] = (*tab.offset(0 as c_int as isize)).re as c_long
        * t[4 as c_int as usize].re as c_long
        - (*tab.offset(1 as c_int as isize)).re as c_long * t[0 as c_int as usize].re as c_long;
    mtmp[2 as c_int as usize] = (*tab.offset(0 as c_int as isize)).re as c_long
        * t[2 as c_int as usize].re as c_long
        - (*tab.offset(2 as c_int as isize)).re as c_long * t[0 as c_int as usize].re as c_long;
    mtmp[3 as c_int as usize] = (*tab.offset(0 as c_int as isize)).re as c_long
        * t[0 as c_int as usize].im as c_long
        - (*tab.offset(1 as c_int as isize)).re as c_long * t[2 as c_int as usize].im as c_long;
    mtmp[4 as c_int as usize] = (*tab.offset(0 as c_int as isize)).re as c_long
        * t[4 as c_int as usize].im as c_long
        - (*tab.offset(1 as c_int as isize)).re as c_long * t[0 as c_int as usize].im as c_long;
    mtmp[5 as c_int as usize] = (*tab.offset(0 as c_int as isize)).re as c_long
        * t[2 as c_int as usize].im as c_long
        - (*tab.offset(2 as c_int as isize)).re as c_long * t[0 as c_int as usize].im as c_long;
    mtmp[6 as c_int as usize] = (*tab.offset(2 as c_int as isize)).im as c_long
        * t[1 as c_int as usize].im as c_long
        + (*tab.offset(1 as c_int as isize)).im as c_long * t[5 as c_int as usize].im as c_long;
    mtmp[7 as c_int as usize] = (*tab.offset(0 as c_int as isize)).im as c_long
        * t[5 as c_int as usize].im as c_long
        + (*tab.offset(2 as c_int as isize)).im as c_long * t[3 as c_int as usize].im as c_long;
    mtmp[8 as c_int as usize] = (*tab.offset(2 as c_int as isize)).im as c_long
        * t[5 as c_int as usize].im as c_long
        + (*tab.offset(1 as c_int as isize)).im as c_long * t[3 as c_int as usize].im as c_long;
    mtmp[9 as c_int as usize] = (*tab.offset(0 as c_int as isize)).im as c_long
        * t[1 as c_int as usize].re as c_long
        + (*tab.offset(1 as c_int as isize)).im as c_long * t[3 as c_int as usize].re as c_long;
    mtmp[10 as c_int as usize] = (*tab.offset(2 as c_int as isize)).im as c_long
        * t[3 as c_int as usize].re as c_long
        + (*tab.offset(0 as c_int as isize)).im as c_long * t[5 as c_int as usize].re as c_long;
    mtmp[11 as c_int as usize] = (*tab.offset(2 as c_int as isize)).im as c_long
        * t[1 as c_int as usize].re as c_long
        + (*tab.offset(1 as c_int as isize)).im as c_long * t[5 as c_int as usize].re as c_long;
    z[0 as c_int as usize].re = (mtmp[0 as c_int as usize]
        - (*tab.offset(1 as c_int as isize)).re as c_long * t[2 as c_int as usize].re as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    z[1 as c_int as usize].re = (mtmp[1 as c_int as usize]
        - (*tab.offset(2 as c_int as isize)).re as c_long * t[2 as c_int as usize].re as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    z[2 as c_int as usize].re = (mtmp[2 as c_int as usize]
        - (*tab.offset(1 as c_int as isize)).re as c_long * t[4 as c_int as usize].re as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    z[0 as c_int as usize].im = (mtmp[3 as c_int as usize]
        - (*tab.offset(2 as c_int as isize)).re as c_long * t[4 as c_int as usize].im as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    z[1 as c_int as usize].im = (mtmp[4 as c_int as usize]
        - (*tab.offset(2 as c_int as isize)).re as c_long * t[2 as c_int as usize].im as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    z[2 as c_int as usize].im = (mtmp[5 as c_int as usize]
        - (*tab.offset(1 as c_int as isize)).re as c_long * t[4 as c_int as usize].im as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    t[0 as c_int as usize].re = (mtmp[6 as c_int as usize]
        - (*tab.offset(0 as c_int as isize)).im as c_long * t[3 as c_int as usize].im as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    t[2 as c_int as usize].re = (mtmp[7 as c_int as usize]
        - (*tab.offset(1 as c_int as isize)).im as c_long * t[1 as c_int as usize].im as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    t[4 as c_int as usize].re = (mtmp[8 as c_int as usize]
        + (*tab.offset(0 as c_int as isize)).im as c_long * t[1 as c_int as usize].im as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    t[0 as c_int as usize].im = (mtmp[9 as c_int as usize]
        + (*tab.offset(2 as c_int as isize)).im as c_long * t[5 as c_int as usize].re as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    t[2 as c_int as usize].im = (mtmp[10 as c_int as usize]
        - (*tab.offset(1 as c_int as isize)).im as c_long * t[1 as c_int as usize].re as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    t[4 as c_int as usize].im = (mtmp[11 as c_int as usize]
        - (*tab.offset(0 as c_int as isize)).im as c_long * t[3 as c_int as usize].re as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    t[1 as c_int as usize].re = (z[0 as c_int as usize].re as c_uint)
        .wrapping_sub(t[4 as c_int as usize].re as c_uint) as c_int;
    z[0 as c_int as usize].re = (z[0 as c_int as usize].re as c_uint)
        .wrapping_add(t[4 as c_int as usize].re as c_uint) as c_int;
    t[3 as c_int as usize].re = (z[1 as c_int as usize].re as c_uint)
        .wrapping_sub(t[2 as c_int as usize].re as c_uint) as c_int;
    z[1 as c_int as usize].re = (z[1 as c_int as usize].re as c_uint)
        .wrapping_add(t[2 as c_int as usize].re as c_uint) as c_int;
    t[5 as c_int as usize].re = (z[2 as c_int as usize].re as c_uint)
        .wrapping_sub(t[0 as c_int as usize].re as c_uint) as c_int;
    z[2 as c_int as usize].re = (z[2 as c_int as usize].re as c_uint)
        .wrapping_add(t[0 as c_int as usize].re as c_uint) as c_int;
    t[1 as c_int as usize].im = (z[0 as c_int as usize].im as c_uint)
        .wrapping_sub(t[0 as c_int as usize].im as c_uint) as c_int;
    z[0 as c_int as usize].im = (z[0 as c_int as usize].im as c_uint)
        .wrapping_add(t[0 as c_int as usize].im as c_uint) as c_int;
    t[3 as c_int as usize].im = (z[1 as c_int as usize].im as c_uint)
        .wrapping_sub(t[2 as c_int as usize].im as c_uint) as c_int;
    z[1 as c_int as usize].im = (z[1 as c_int as usize].im as c_uint)
        .wrapping_add(t[2 as c_int as usize].im as c_uint) as c_int;
    t[5 as c_int as usize].im = (z[2 as c_int as usize].im as c_uint)
        .wrapping_sub(t[4 as c_int as usize].im as c_uint) as c_int;
    z[2 as c_int as usize].im = (z[2 as c_int as usize].im as c_uint)
        .wrapping_add(t[4 as c_int as usize].im as c_uint) as c_int;
    (*out.offset((1 as c_int as c_long * stride) as isize)).re = dc.re + z[0 as c_int as usize].re;
    (*out.offset((1 as c_int as c_long * stride) as isize)).im = dc.im + t[1 as c_int as usize].im;
    (*out.offset((2 as c_int as c_long * stride) as isize)).re = dc.re + t[3 as c_int as usize].re;
    (*out.offset((2 as c_int as c_long * stride) as isize)).im = dc.im + z[1 as c_int as usize].im;
    (*out.offset((3 as c_int as c_long * stride) as isize)).re = dc.re + z[2 as c_int as usize].re;
    (*out.offset((3 as c_int as c_long * stride) as isize)).im = dc.im + t[5 as c_int as usize].im;
    (*out.offset((4 as c_int as c_long * stride) as isize)).re = dc.re + t[5 as c_int as usize].re;
    (*out.offset((4 as c_int as c_long * stride) as isize)).im = dc.im + z[2 as c_int as usize].im;
    (*out.offset((5 as c_int as c_long * stride) as isize)).re = dc.re + z[1 as c_int as usize].re;
    (*out.offset((5 as c_int as c_long * stride) as isize)).im = dc.im + t[3 as c_int as usize].im;
    (*out.offset((6 as c_int as c_long * stride) as isize)).re = dc.re + t[1 as c_int as usize].re;
    (*out.offset((6 as c_int as c_long * stride) as isize)).im = dc.im + z[0 as c_int as usize].im;
}
#[inline(always)]
unsafe extern "C" fn fft9(out: *mut TXComplex, in_0: *mut TXComplex, stride: ptrdiff_t) {
    let tab: *const TXComplex = ff_tx_tab_9_int32.as_mut_ptr() as *const TXComplex;
    let mut dc: TXComplex = TXComplex { re: 0, im: 0 };
    let mut t: [TXComplex; 16] = [TXComplex { re: 0, im: 0 }; 16];
    let mut w: [TXComplex; 4] = [TXComplex { re: 0, im: 0 }; 4];
    let mut x: [TXComplex; 5] = [TXComplex { re: 0, im: 0 }; 5];
    let mut y: [TXComplex; 5] = [TXComplex { re: 0, im: 0 }; 5];
    let mut z: [TXComplex; 2] = [TXComplex { re: 0, im: 0 }; 2];
    let mut mtmp: [c_long; 12] = [0; 12];
    dc = *in_0.offset(0 as c_int as isize);
    t[1 as c_int as usize].re = ((*in_0.offset(1 as c_int as isize)).re as c_uint)
        .wrapping_sub((*in_0.offset(8 as c_int as isize)).re as c_uint)
        as c_int;
    t[0 as c_int as usize].re = ((*in_0.offset(1 as c_int as isize)).re as c_uint)
        .wrapping_add((*in_0.offset(8 as c_int as isize)).re as c_uint)
        as c_int;
    t[1 as c_int as usize].im = ((*in_0.offset(1 as c_int as isize)).im as c_uint)
        .wrapping_sub((*in_0.offset(8 as c_int as isize)).im as c_uint)
        as c_int;
    t[0 as c_int as usize].im = ((*in_0.offset(1 as c_int as isize)).im as c_uint)
        .wrapping_add((*in_0.offset(8 as c_int as isize)).im as c_uint)
        as c_int;
    t[3 as c_int as usize].re = ((*in_0.offset(2 as c_int as isize)).re as c_uint)
        .wrapping_sub((*in_0.offset(7 as c_int as isize)).re as c_uint)
        as c_int;
    t[2 as c_int as usize].re = ((*in_0.offset(2 as c_int as isize)).re as c_uint)
        .wrapping_add((*in_0.offset(7 as c_int as isize)).re as c_uint)
        as c_int;
    t[3 as c_int as usize].im = ((*in_0.offset(2 as c_int as isize)).im as c_uint)
        .wrapping_sub((*in_0.offset(7 as c_int as isize)).im as c_uint)
        as c_int;
    t[2 as c_int as usize].im = ((*in_0.offset(2 as c_int as isize)).im as c_uint)
        .wrapping_add((*in_0.offset(7 as c_int as isize)).im as c_uint)
        as c_int;
    t[5 as c_int as usize].re = ((*in_0.offset(3 as c_int as isize)).re as c_uint)
        .wrapping_sub((*in_0.offset(6 as c_int as isize)).re as c_uint)
        as c_int;
    t[4 as c_int as usize].re = ((*in_0.offset(3 as c_int as isize)).re as c_uint)
        .wrapping_add((*in_0.offset(6 as c_int as isize)).re as c_uint)
        as c_int;
    t[5 as c_int as usize].im = ((*in_0.offset(3 as c_int as isize)).im as c_uint)
        .wrapping_sub((*in_0.offset(6 as c_int as isize)).im as c_uint)
        as c_int;
    t[4 as c_int as usize].im = ((*in_0.offset(3 as c_int as isize)).im as c_uint)
        .wrapping_add((*in_0.offset(6 as c_int as isize)).im as c_uint)
        as c_int;
    t[7 as c_int as usize].re = ((*in_0.offset(4 as c_int as isize)).re as c_uint)
        .wrapping_sub((*in_0.offset(5 as c_int as isize)).re as c_uint)
        as c_int;
    t[6 as c_int as usize].re = ((*in_0.offset(4 as c_int as isize)).re as c_uint)
        .wrapping_add((*in_0.offset(5 as c_int as isize)).re as c_uint)
        as c_int;
    t[7 as c_int as usize].im = ((*in_0.offset(4 as c_int as isize)).im as c_uint)
        .wrapping_sub((*in_0.offset(5 as c_int as isize)).im as c_uint)
        as c_int;
    t[6 as c_int as usize].im = ((*in_0.offset(4 as c_int as isize)).im as c_uint)
        .wrapping_add((*in_0.offset(5 as c_int as isize)).im as c_uint)
        as c_int;
    w[0 as c_int as usize].re = t[0 as c_int as usize].re - t[6 as c_int as usize].re;
    w[0 as c_int as usize].im = t[0 as c_int as usize].im - t[6 as c_int as usize].im;
    w[1 as c_int as usize].re = t[2 as c_int as usize].re - t[6 as c_int as usize].re;
    w[1 as c_int as usize].im = t[2 as c_int as usize].im - t[6 as c_int as usize].im;
    w[2 as c_int as usize].re = t[1 as c_int as usize].re - t[7 as c_int as usize].re;
    w[2 as c_int as usize].im = t[1 as c_int as usize].im - t[7 as c_int as usize].im;
    w[3 as c_int as usize].re = t[3 as c_int as usize].re + t[7 as c_int as usize].re;
    w[3 as c_int as usize].im = t[3 as c_int as usize].im + t[7 as c_int as usize].im;
    z[0 as c_int as usize].re = dc.re + t[4 as c_int as usize].re;
    z[0 as c_int as usize].im = dc.im + t[4 as c_int as usize].im;
    z[1 as c_int as usize].re =
        t[0 as c_int as usize].re + t[2 as c_int as usize].re + t[6 as c_int as usize].re;
    z[1 as c_int as usize].im =
        t[0 as c_int as usize].im + t[2 as c_int as usize].im + t[6 as c_int as usize].im;
    (*out.offset((0 as c_int as c_long * stride) as isize)).re =
        z[0 as c_int as usize].re + z[1 as c_int as usize].re;
    (*out.offset((0 as c_int as c_long * stride) as isize)).im =
        z[0 as c_int as usize].im + z[1 as c_int as usize].im;
    mtmp[0 as c_int as usize] = (t[1 as c_int as usize].re - t[3 as c_int as usize].re
        + t[7 as c_int as usize].re) as c_long;
    mtmp[1 as c_int as usize] = (t[1 as c_int as usize].im - t[3 as c_int as usize].im
        + t[7 as c_int as usize].im) as c_long;
    y[3 as c_int as usize].re = ((*tab.offset(0 as c_int as isize)).im as c_long
        * mtmp[0 as c_int as usize]
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    y[3 as c_int as usize].im = ((*tab.offset(0 as c_int as isize)).im as c_long
        * mtmp[1 as c_int as usize]
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    mtmp[0 as c_int as usize] = ((*tab.offset(0 as c_int as isize)).re as c_long
        * z[1 as c_int as usize].re as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int as c_long;
    mtmp[1 as c_int as usize] = ((*tab.offset(0 as c_int as isize)).re as c_long
        * z[1 as c_int as usize].im as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int as c_long;
    mtmp[2 as c_int as usize] = ((*tab.offset(0 as c_int as isize)).re as c_long
        * t[4 as c_int as usize].re as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int as c_long;
    mtmp[3 as c_int as usize] = ((*tab.offset(0 as c_int as isize)).re as c_long
        * t[4 as c_int as usize].im as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int as c_long;
    x[3 as c_int as usize].re = z[0 as c_int as usize].re + mtmp[0 as c_int as usize] as c_int;
    x[3 as c_int as usize].im = z[0 as c_int as usize].im + mtmp[1 as c_int as usize] as c_int;
    z[0 as c_int as usize].re =
        (*in_0.offset(0 as c_int as isize)).re + mtmp[2 as c_int as usize] as c_int;
    z[0 as c_int as usize].im =
        (*in_0.offset(0 as c_int as isize)).im + mtmp[3 as c_int as usize] as c_int;
    mtmp[0 as c_int as usize] =
        (*tab.offset(1 as c_int as isize)).re as c_long * w[0 as c_int as usize].re as c_long;
    mtmp[1 as c_int as usize] =
        (*tab.offset(1 as c_int as isize)).re as c_long * w[0 as c_int as usize].im as c_long;
    mtmp[2 as c_int as usize] =
        (*tab.offset(2 as c_int as isize)).im as c_long * w[0 as c_int as usize].re as c_long;
    mtmp[3 as c_int as usize] =
        (*tab.offset(2 as c_int as isize)).im as c_long * w[0 as c_int as usize].im as c_long;
    mtmp[4 as c_int as usize] =
        (*tab.offset(1 as c_int as isize)).im as c_long * w[2 as c_int as usize].re as c_long;
    mtmp[5 as c_int as usize] =
        (*tab.offset(1 as c_int as isize)).im as c_long * w[2 as c_int as usize].im as c_long;
    mtmp[6 as c_int as usize] =
        (*tab.offset(2 as c_int as isize)).re as c_long * w[2 as c_int as usize].re as c_long;
    mtmp[7 as c_int as usize] =
        (*tab.offset(2 as c_int as isize)).re as c_long * w[2 as c_int as usize].im as c_long;
    x[1 as c_int as usize].re = (mtmp[0 as c_int as usize]
        + (*tab.offset(2 as c_int as isize)).im as c_long * w[1 as c_int as usize].re as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    x[1 as c_int as usize].im = (mtmp[1 as c_int as usize]
        + (*tab.offset(2 as c_int as isize)).im as c_long * w[1 as c_int as usize].im as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    x[2 as c_int as usize].re = (mtmp[2 as c_int as usize]
        - (*tab.offset(3 as c_int as isize)).re as c_long * w[1 as c_int as usize].re as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    x[2 as c_int as usize].im = (mtmp[3 as c_int as usize]
        - (*tab.offset(3 as c_int as isize)).re as c_long * w[1 as c_int as usize].im as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    y[1 as c_int as usize].re = (mtmp[4 as c_int as usize]
        + (*tab.offset(2 as c_int as isize)).re as c_long * w[3 as c_int as usize].re as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    y[1 as c_int as usize].im = (mtmp[5 as c_int as usize]
        + (*tab.offset(2 as c_int as isize)).re as c_long * w[3 as c_int as usize].im as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    y[2 as c_int as usize].re = (mtmp[6 as c_int as usize]
        - (*tab.offset(3 as c_int as isize)).im as c_long * w[3 as c_int as usize].re as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    y[2 as c_int as usize].im = (mtmp[7 as c_int as usize]
        - (*tab.offset(3 as c_int as isize)).im as c_long * w[3 as c_int as usize].im as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    y[0 as c_int as usize].re = ((*tab.offset(0 as c_int as isize)).im as c_long
        * t[5 as c_int as usize].re as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    y[0 as c_int as usize].im = ((*tab.offset(0 as c_int as isize)).im as c_long
        * t[5 as c_int as usize].im as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    x[4 as c_int as usize].re = x[1 as c_int as usize].re + x[2 as c_int as usize].re;
    x[4 as c_int as usize].im = x[1 as c_int as usize].im + x[2 as c_int as usize].im;
    y[4 as c_int as usize].re = y[1 as c_int as usize].re - y[2 as c_int as usize].re;
    y[4 as c_int as usize].im = y[1 as c_int as usize].im - y[2 as c_int as usize].im;
    x[1 as c_int as usize].re += z[0 as c_int as usize].re;
    x[1 as c_int as usize].im += z[0 as c_int as usize].im;
    y[1 as c_int as usize].re += y[0 as c_int as usize].re;
    y[1 as c_int as usize].im += y[0 as c_int as usize].im;
    x[2 as c_int as usize].re += z[0 as c_int as usize].re;
    x[2 as c_int as usize].im += z[0 as c_int as usize].im;
    y[2 as c_int as usize].re -= y[0 as c_int as usize].re;
    y[2 as c_int as usize].im -= y[0 as c_int as usize].im;
    x[4 as c_int as usize].re = z[0 as c_int as usize].re - x[4 as c_int as usize].re;
    x[4 as c_int as usize].im = z[0 as c_int as usize].im - x[4 as c_int as usize].im;
    y[4 as c_int as usize].re = y[0 as c_int as usize].re - y[4 as c_int as usize].re;
    y[4 as c_int as usize].im = y[0 as c_int as usize].im - y[4 as c_int as usize].im;
    *out.offset((1 as c_int as c_long * stride) as isize) = {
        AVComplexInt32 {
            re: x[1 as c_int as usize].re + y[1 as c_int as usize].im,
            im: x[1 as c_int as usize].im - y[1 as c_int as usize].re,
        }
    };
    *out.offset((2 as c_int as c_long * stride) as isize) = {
        AVComplexInt32 {
            re: x[2 as c_int as usize].re + y[2 as c_int as usize].im,
            im: x[2 as c_int as usize].im - y[2 as c_int as usize].re,
        }
    };
    *out.offset((3 as c_int as c_long * stride) as isize) = {
        AVComplexInt32 {
            re: x[3 as c_int as usize].re + y[3 as c_int as usize].im,
            im: x[3 as c_int as usize].im - y[3 as c_int as usize].re,
        }
    };
    *out.offset((4 as c_int as c_long * stride) as isize) = {
        AVComplexInt32 {
            re: x[4 as c_int as usize].re + y[4 as c_int as usize].im,
            im: x[4 as c_int as usize].im - y[4 as c_int as usize].re,
        }
    };
    *out.offset((5 as c_int as c_long * stride) as isize) = {
        AVComplexInt32 {
            re: x[4 as c_int as usize].re - y[4 as c_int as usize].im,
            im: x[4 as c_int as usize].im + y[4 as c_int as usize].re,
        }
    };
    *out.offset((6 as c_int as c_long * stride) as isize) = {
        AVComplexInt32 {
            re: x[3 as c_int as usize].re - y[3 as c_int as usize].im,
            im: x[3 as c_int as usize].im + y[3 as c_int as usize].re,
        }
    };
    *out.offset((7 as c_int as c_long * stride) as isize) = {
        AVComplexInt32 {
            re: x[2 as c_int as usize].re - y[2 as c_int as usize].im,
            im: x[2 as c_int as usize].im + y[2 as c_int as usize].re,
        }
    };
    *out.offset((8 as c_int as c_long * stride) as isize) = {
        AVComplexInt32 {
            re: x[1 as c_int as usize].re - y[1 as c_int as usize].im,
            im: x[1 as c_int as usize].im + y[1 as c_int as usize].re,
        }
    };
}
#[inline(always)]
unsafe extern "C" fn fft15(out: *mut TXComplex, in_0: *mut TXComplex, stride: ptrdiff_t) {
    let mut tmp: [TXComplex; 15] = [TXComplex { re: 0, im: 0 }; 15];
    let mut i: c_int = 0 as c_int;
    while i < 5 as c_int {
        fft3(
            tmp.as_mut_ptr().offset(i as isize),
            in_0.offset((i * 3 as c_int) as isize),
            5 as c_int as ptrdiff_t,
        );
        i += 1;
        i;
    }
    fft5_m1(out, tmp.as_mut_ptr().offset(0 as c_int as isize), stride);
    fft5_m2(out, tmp.as_mut_ptr().offset(5 as c_int as isize), stride);
    fft5_m3(out, tmp.as_mut_ptr().offset(10 as c_int as isize), stride);
}
#[cold]
unsafe extern "C" fn ff_tx_fft_factor_init_int32_c(
    s: *mut AVTXContext,
    _cd: *const FFTXCodelet,
    flags: c_ulong,
    opts: *mut FFTXCodeletOptions,
    len: c_int,
    _inv: c_int,
    _scale: *const c_void,
) -> c_int {
    let mut ret: c_int = 0 as c_int;
    ff_tx_init_tabs_int32(len);
    if len == 15 as c_int {
        ret = ff_tx_gen_pfa_input_map(s, opts, 3 as c_int, 5 as c_int);
    } else if flags as c_ulonglong & (1 as c_ulonglong) << 61 as c_int != 0 {
        ret = ff_tx_gen_default_map(s, opts);
    }
    ret
}
static mut ff_tx_fft3_fwd_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft3_fwd_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft3_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: (AV_TX_INPLACE as c_int as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 59 as c_int) as c_ulong,
            factors: [3 as c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 3 as c_int,
            max_len: 3 as c_int,
            init: Some(
                ff_tx_fft_factor_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
unsafe extern "C" fn ff_tx_fft3_int32_c(
    _s: *mut AVTXContext,
    dst: *mut c_void,
    src: *mut c_void,
    stride: ptrdiff_t,
) {
    fft3(
        dst as *mut TXComplex,
        src as *mut TXComplex,
        (stride as c_ulong).wrapping_div(size_of::<TXComplex>() as c_ulong) as ptrdiff_t,
    );
}
static mut ff_tx_fft3_ns_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft3_ns_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft3_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: (AV_TX_INPLACE as c_int as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 61 as c_int) as c_ulong,
            factors: [3 as c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 3 as c_int,
            max_len: 3 as c_int,
            init: Some(
                ff_tx_fft_factor_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
static mut ff_tx_fft5_ns_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft5_ns_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft5_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: (AV_TX_INPLACE as c_int as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 61 as c_int) as c_ulong,
            factors: [5 as c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 5 as c_int,
            max_len: 5 as c_int,
            init: Some(
                ff_tx_fft_factor_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
static mut ff_tx_fft5_fwd_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft5_fwd_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft5_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: (AV_TX_INPLACE as c_int as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 59 as c_int) as c_ulong,
            factors: [5 as c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 5 as c_int,
            max_len: 5 as c_int,
            init: Some(
                ff_tx_fft_factor_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
unsafe extern "C" fn ff_tx_fft5_int32_c(
    _s: *mut AVTXContext,
    dst: *mut c_void,
    src: *mut c_void,
    stride: ptrdiff_t,
) {
    fft5(
        dst as *mut TXComplex,
        src as *mut TXComplex,
        (stride as c_ulong).wrapping_div(size_of::<TXComplex>() as c_ulong) as ptrdiff_t,
    );
}
unsafe extern "C" fn ff_tx_fft7_int32_c(
    _s: *mut AVTXContext,
    dst: *mut c_void,
    src: *mut c_void,
    stride: ptrdiff_t,
) {
    fft7(
        dst as *mut TXComplex,
        src as *mut TXComplex,
        (stride as c_ulong).wrapping_div(size_of::<TXComplex>() as c_ulong) as ptrdiff_t,
    );
}
static mut ff_tx_fft7_fwd_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft7_fwd_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft7_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: (AV_TX_INPLACE as c_int as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 59 as c_int) as c_ulong,
            factors: [7 as c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 7 as c_int,
            max_len: 7 as c_int,
            init: Some(
                ff_tx_fft_factor_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
static mut ff_tx_fft7_ns_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft7_ns_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft7_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: (AV_TX_INPLACE as c_int as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 61 as c_int) as c_ulong,
            factors: [7 as c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 7 as c_int,
            max_len: 7 as c_int,
            init: Some(
                ff_tx_fft_factor_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
static mut ff_tx_fft9_fwd_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft9_fwd_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft9_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: (AV_TX_INPLACE as c_int as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 59 as c_int) as c_ulong,
            factors: [9 as c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 9 as c_int,
            max_len: 9 as c_int,
            init: Some(
                ff_tx_fft_factor_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
static mut ff_tx_fft9_ns_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft9_ns_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft9_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: (AV_TX_INPLACE as c_int as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 61 as c_int) as c_ulong,
            factors: [9 as c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 9 as c_int,
            max_len: 9 as c_int,
            init: Some(
                ff_tx_fft_factor_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
unsafe extern "C" fn ff_tx_fft9_int32_c(
    _s: *mut AVTXContext,
    dst: *mut c_void,
    src: *mut c_void,
    stride: ptrdiff_t,
) {
    fft9(
        dst as *mut TXComplex,
        src as *mut TXComplex,
        (stride as c_ulong).wrapping_div(size_of::<TXComplex>() as c_ulong) as ptrdiff_t,
    );
}
static mut ff_tx_fft15_ns_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft15_ns_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft15_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: (AV_TX_INPLACE as c_int as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 61 as c_int) as c_ulong,
            factors: [15 as c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 15 as c_int,
            max_len: 15 as c_int,
            init: Some(
                ff_tx_fft_factor_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
unsafe extern "C" fn ff_tx_fft15_int32_c(
    _s: *mut AVTXContext,
    dst: *mut c_void,
    src: *mut c_void,
    stride: ptrdiff_t,
) {
    fft15(
        dst as *mut TXComplex,
        src as *mut TXComplex,
        (stride as c_ulong).wrapping_div(size_of::<TXComplex>() as c_ulong) as ptrdiff_t,
    );
}
#[inline]
unsafe extern "C" fn ff_tx_fft_sr_combine_int32_c(
    mut z: *mut TXComplex,
    mut cos_0: *const TXSample,
    len: c_int,
) {
    let o1: c_int = 2 as c_int * len;
    let o2: c_int = 4 as c_int * len;
    let o3: c_int = 6 as c_int * len;
    let mut wim: *const TXSample = cos_0.offset(o1 as isize).offset(-(7 as c_int as isize));
    let mut t1: TXUSample = 0;
    let mut t2: TXUSample = 0;
    let mut t3: TXUSample = 0;
    let mut t4: TXUSample = 0;
    let mut t5: TXUSample = 0;
    let mut t6: TXUSample = 0;
    let mut r0: TXUSample = 0;
    let mut i0: TXUSample = 0;
    let mut r1: TXUSample = 0;
    let mut i1: TXUSample = 0;
    let mut i: c_int = 0 as c_int;
    while i < len {
        let mut accu: c_long = 0;
        accu = *cos_0.offset(0 as c_int as isize) as c_long
            * (*z.offset((o2 + 0 as c_int) as isize)).re as c_long;
        accu -= -*wim.offset(7 as c_int as isize) as c_long
            * (*z.offset((o2 + 0 as c_int) as isize)).im as c_long;
        t1 = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
        accu = -*wim.offset(7 as c_int as isize) as c_long
            * (*z.offset((o2 + 0 as c_int) as isize)).re as c_long;
        accu += *cos_0.offset(0 as c_int as isize) as c_long
            * (*z.offset((o2 + 0 as c_int) as isize)).im as c_long;
        t2 = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
        let mut accu_0: c_long = 0;
        accu_0 = *cos_0.offset(0 as c_int as isize) as c_long
            * (*z.offset((o3 + 0 as c_int) as isize)).re as c_long;
        accu_0 -= *wim.offset(7 as c_int as isize) as c_long
            * (*z.offset((o3 + 0 as c_int) as isize)).im as c_long;
        t5 = (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
        accu_0 = *wim.offset(7 as c_int as isize) as c_long
            * (*z.offset((o3 + 0 as c_int) as isize)).re as c_long;
        accu_0 += *cos_0.offset(0 as c_int as isize) as c_long
            * (*z.offset((o3 + 0 as c_int) as isize)).im as c_long;
        t6 = (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
        r0 = (*z.offset(0 as c_int as isize)).re as TXUSample;
        i0 = (*z.offset(0 as c_int as isize)).im as TXUSample;
        r1 = (*z.offset((o1 + 0 as c_int) as isize)).re as TXUSample;
        i1 = (*z.offset((o1 + 0 as c_int) as isize)).im as TXUSample;
        t3 = t5.wrapping_sub(t1);
        t5 = t5.wrapping_add(t1);
        (*z.offset((o2 + 0 as c_int) as isize)).re = r0.wrapping_sub(t5) as c_int;
        (*z.offset(0 as c_int as isize)).re = r0.wrapping_add(t5) as c_int;
        (*z.offset((o3 + 0 as c_int) as isize)).im = i1.wrapping_sub(t3) as c_int;
        (*z.offset((o1 + 0 as c_int) as isize)).im = i1.wrapping_add(t3) as c_int;
        t4 = t2.wrapping_sub(t6);
        t6 = t2.wrapping_add(t6);
        (*z.offset((o3 + 0 as c_int) as isize)).re = r1.wrapping_sub(t4) as c_int;
        (*z.offset((o1 + 0 as c_int) as isize)).re = r1.wrapping_add(t4) as c_int;
        (*z.offset((o2 + 0 as c_int) as isize)).im = i0.wrapping_sub(t6) as c_int;
        (*z.offset(0 as c_int as isize)).im = i0.wrapping_add(t6) as c_int;
        let mut accu_1: c_long = 0;
        accu_1 = *cos_0.offset(2 as c_int as isize) as c_long
            * (*z.offset((o2 + 2 as c_int) as isize)).re as c_long;
        accu_1 -= -*wim.offset(5 as c_int as isize) as c_long
            * (*z.offset((o2 + 2 as c_int) as isize)).im as c_long;
        t1 = (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
        accu_1 = -*wim.offset(5 as c_int as isize) as c_long
            * (*z.offset((o2 + 2 as c_int) as isize)).re as c_long;
        accu_1 += *cos_0.offset(2 as c_int as isize) as c_long
            * (*z.offset((o2 + 2 as c_int) as isize)).im as c_long;
        t2 = (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
        let mut accu_2: c_long = 0;
        accu_2 = *cos_0.offset(2 as c_int as isize) as c_long
            * (*z.offset((o3 + 2 as c_int) as isize)).re as c_long;
        accu_2 -= *wim.offset(5 as c_int as isize) as c_long
            * (*z.offset((o3 + 2 as c_int) as isize)).im as c_long;
        t5 = (accu_2 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
        accu_2 = *wim.offset(5 as c_int as isize) as c_long
            * (*z.offset((o3 + 2 as c_int) as isize)).re as c_long;
        accu_2 += *cos_0.offset(2 as c_int as isize) as c_long
            * (*z.offset((o3 + 2 as c_int) as isize)).im as c_long;
        t6 = (accu_2 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
        r0 = (*z.offset(2 as c_int as isize)).re as TXUSample;
        i0 = (*z.offset(2 as c_int as isize)).im as TXUSample;
        r1 = (*z.offset((o1 + 2 as c_int) as isize)).re as TXUSample;
        i1 = (*z.offset((o1 + 2 as c_int) as isize)).im as TXUSample;
        t3 = t5.wrapping_sub(t1);
        t5 = t5.wrapping_add(t1);
        (*z.offset((o2 + 2 as c_int) as isize)).re = r0.wrapping_sub(t5) as c_int;
        (*z.offset(2 as c_int as isize)).re = r0.wrapping_add(t5) as c_int;
        (*z.offset((o3 + 2 as c_int) as isize)).im = i1.wrapping_sub(t3) as c_int;
        (*z.offset((o1 + 2 as c_int) as isize)).im = i1.wrapping_add(t3) as c_int;
        t4 = t2.wrapping_sub(t6);
        t6 = t2.wrapping_add(t6);
        (*z.offset((o3 + 2 as c_int) as isize)).re = r1.wrapping_sub(t4) as c_int;
        (*z.offset((o1 + 2 as c_int) as isize)).re = r1.wrapping_add(t4) as c_int;
        (*z.offset((o2 + 2 as c_int) as isize)).im = i0.wrapping_sub(t6) as c_int;
        (*z.offset(2 as c_int as isize)).im = i0.wrapping_add(t6) as c_int;
        let mut accu_3: c_long = 0;
        accu_3 = *cos_0.offset(4 as c_int as isize) as c_long
            * (*z.offset((o2 + 4 as c_int) as isize)).re as c_long;
        accu_3 -= -*wim.offset(3 as c_int as isize) as c_long
            * (*z.offset((o2 + 4 as c_int) as isize)).im as c_long;
        t1 = (accu_3 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
        accu_3 = -*wim.offset(3 as c_int as isize) as c_long
            * (*z.offset((o2 + 4 as c_int) as isize)).re as c_long;
        accu_3 += *cos_0.offset(4 as c_int as isize) as c_long
            * (*z.offset((o2 + 4 as c_int) as isize)).im as c_long;
        t2 = (accu_3 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
        let mut accu_4: c_long = 0;
        accu_4 = *cos_0.offset(4 as c_int as isize) as c_long
            * (*z.offset((o3 + 4 as c_int) as isize)).re as c_long;
        accu_4 -= *wim.offset(3 as c_int as isize) as c_long
            * (*z.offset((o3 + 4 as c_int) as isize)).im as c_long;
        t5 = (accu_4 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
        accu_4 = *wim.offset(3 as c_int as isize) as c_long
            * (*z.offset((o3 + 4 as c_int) as isize)).re as c_long;
        accu_4 += *cos_0.offset(4 as c_int as isize) as c_long
            * (*z.offset((o3 + 4 as c_int) as isize)).im as c_long;
        t6 = (accu_4 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
        r0 = (*z.offset(4 as c_int as isize)).re as TXUSample;
        i0 = (*z.offset(4 as c_int as isize)).im as TXUSample;
        r1 = (*z.offset((o1 + 4 as c_int) as isize)).re as TXUSample;
        i1 = (*z.offset((o1 + 4 as c_int) as isize)).im as TXUSample;
        t3 = t5.wrapping_sub(t1);
        t5 = t5.wrapping_add(t1);
        (*z.offset((o2 + 4 as c_int) as isize)).re = r0.wrapping_sub(t5) as c_int;
        (*z.offset(4 as c_int as isize)).re = r0.wrapping_add(t5) as c_int;
        (*z.offset((o3 + 4 as c_int) as isize)).im = i1.wrapping_sub(t3) as c_int;
        (*z.offset((o1 + 4 as c_int) as isize)).im = i1.wrapping_add(t3) as c_int;
        t4 = t2.wrapping_sub(t6);
        t6 = t2.wrapping_add(t6);
        (*z.offset((o3 + 4 as c_int) as isize)).re = r1.wrapping_sub(t4) as c_int;
        (*z.offset((o1 + 4 as c_int) as isize)).re = r1.wrapping_add(t4) as c_int;
        (*z.offset((o2 + 4 as c_int) as isize)).im = i0.wrapping_sub(t6) as c_int;
        (*z.offset(4 as c_int as isize)).im = i0.wrapping_add(t6) as c_int;
        let mut accu_5: c_long = 0;
        accu_5 = *cos_0.offset(6 as c_int as isize) as c_long
            * (*z.offset((o2 + 6 as c_int) as isize)).re as c_long;
        accu_5 -= -*wim.offset(1 as c_int as isize) as c_long
            * (*z.offset((o2 + 6 as c_int) as isize)).im as c_long;
        t1 = (accu_5 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
        accu_5 = -*wim.offset(1 as c_int as isize) as c_long
            * (*z.offset((o2 + 6 as c_int) as isize)).re as c_long;
        accu_5 += *cos_0.offset(6 as c_int as isize) as c_long
            * (*z.offset((o2 + 6 as c_int) as isize)).im as c_long;
        t2 = (accu_5 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
        let mut accu_6: c_long = 0;
        accu_6 = *cos_0.offset(6 as c_int as isize) as c_long
            * (*z.offset((o3 + 6 as c_int) as isize)).re as c_long;
        accu_6 -= *wim.offset(1 as c_int as isize) as c_long
            * (*z.offset((o3 + 6 as c_int) as isize)).im as c_long;
        t5 = (accu_6 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
        accu_6 = *wim.offset(1 as c_int as isize) as c_long
            * (*z.offset((o3 + 6 as c_int) as isize)).re as c_long;
        accu_6 += *cos_0.offset(6 as c_int as isize) as c_long
            * (*z.offset((o3 + 6 as c_int) as isize)).im as c_long;
        t6 = (accu_6 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
        r0 = (*z.offset(6 as c_int as isize)).re as TXUSample;
        i0 = (*z.offset(6 as c_int as isize)).im as TXUSample;
        r1 = (*z.offset((o1 + 6 as c_int) as isize)).re as TXUSample;
        i1 = (*z.offset((o1 + 6 as c_int) as isize)).im as TXUSample;
        t3 = t5.wrapping_sub(t1);
        t5 = t5.wrapping_add(t1);
        (*z.offset((o2 + 6 as c_int) as isize)).re = r0.wrapping_sub(t5) as c_int;
        (*z.offset(6 as c_int as isize)).re = r0.wrapping_add(t5) as c_int;
        (*z.offset((o3 + 6 as c_int) as isize)).im = i1.wrapping_sub(t3) as c_int;
        (*z.offset((o1 + 6 as c_int) as isize)).im = i1.wrapping_add(t3) as c_int;
        t4 = t2.wrapping_sub(t6);
        t6 = t2.wrapping_add(t6);
        (*z.offset((o3 + 6 as c_int) as isize)).re = r1.wrapping_sub(t4) as c_int;
        (*z.offset((o1 + 6 as c_int) as isize)).re = r1.wrapping_add(t4) as c_int;
        (*z.offset((o2 + 6 as c_int) as isize)).im = i0.wrapping_sub(t6) as c_int;
        (*z.offset(6 as c_int as isize)).im = i0.wrapping_add(t6) as c_int;
        let mut accu_7: c_long = 0;
        accu_7 = *cos_0.offset(1 as c_int as isize) as c_long
            * (*z.offset((o2 + 1 as c_int) as isize)).re as c_long;
        accu_7 -= -*wim.offset(6 as c_int as isize) as c_long
            * (*z.offset((o2 + 1 as c_int) as isize)).im as c_long;
        t1 = (accu_7 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
        accu_7 = -*wim.offset(6 as c_int as isize) as c_long
            * (*z.offset((o2 + 1 as c_int) as isize)).re as c_long;
        accu_7 += *cos_0.offset(1 as c_int as isize) as c_long
            * (*z.offset((o2 + 1 as c_int) as isize)).im as c_long;
        t2 = (accu_7 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
        let mut accu_8: c_long = 0;
        accu_8 = *cos_0.offset(1 as c_int as isize) as c_long
            * (*z.offset((o3 + 1 as c_int) as isize)).re as c_long;
        accu_8 -= *wim.offset(6 as c_int as isize) as c_long
            * (*z.offset((o3 + 1 as c_int) as isize)).im as c_long;
        t5 = (accu_8 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
        accu_8 = *wim.offset(6 as c_int as isize) as c_long
            * (*z.offset((o3 + 1 as c_int) as isize)).re as c_long;
        accu_8 += *cos_0.offset(1 as c_int as isize) as c_long
            * (*z.offset((o3 + 1 as c_int) as isize)).im as c_long;
        t6 = (accu_8 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
        r0 = (*z.offset(1 as c_int as isize)).re as TXUSample;
        i0 = (*z.offset(1 as c_int as isize)).im as TXUSample;
        r1 = (*z.offset((o1 + 1 as c_int) as isize)).re as TXUSample;
        i1 = (*z.offset((o1 + 1 as c_int) as isize)).im as TXUSample;
        t3 = t5.wrapping_sub(t1);
        t5 = t5.wrapping_add(t1);
        (*z.offset((o2 + 1 as c_int) as isize)).re = r0.wrapping_sub(t5) as c_int;
        (*z.offset(1 as c_int as isize)).re = r0.wrapping_add(t5) as c_int;
        (*z.offset((o3 + 1 as c_int) as isize)).im = i1.wrapping_sub(t3) as c_int;
        (*z.offset((o1 + 1 as c_int) as isize)).im = i1.wrapping_add(t3) as c_int;
        t4 = t2.wrapping_sub(t6);
        t6 = t2.wrapping_add(t6);
        (*z.offset((o3 + 1 as c_int) as isize)).re = r1.wrapping_sub(t4) as c_int;
        (*z.offset((o1 + 1 as c_int) as isize)).re = r1.wrapping_add(t4) as c_int;
        (*z.offset((o2 + 1 as c_int) as isize)).im = i0.wrapping_sub(t6) as c_int;
        (*z.offset(1 as c_int as isize)).im = i0.wrapping_add(t6) as c_int;
        let mut accu_9: c_long = 0;
        accu_9 = *cos_0.offset(3 as c_int as isize) as c_long
            * (*z.offset((o2 + 3 as c_int) as isize)).re as c_long;
        accu_9 -= -*wim.offset(4 as c_int as isize) as c_long
            * (*z.offset((o2 + 3 as c_int) as isize)).im as c_long;
        t1 = (accu_9 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
        accu_9 = -*wim.offset(4 as c_int as isize) as c_long
            * (*z.offset((o2 + 3 as c_int) as isize)).re as c_long;
        accu_9 += *cos_0.offset(3 as c_int as isize) as c_long
            * (*z.offset((o2 + 3 as c_int) as isize)).im as c_long;
        t2 = (accu_9 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
        let mut accu_10: c_long = 0;
        accu_10 = *cos_0.offset(3 as c_int as isize) as c_long
            * (*z.offset((o3 + 3 as c_int) as isize)).re as c_long;
        accu_10 -= *wim.offset(4 as c_int as isize) as c_long
            * (*z.offset((o3 + 3 as c_int) as isize)).im as c_long;
        t5 = (accu_10 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
        accu_10 = *wim.offset(4 as c_int as isize) as c_long
            * (*z.offset((o3 + 3 as c_int) as isize)).re as c_long;
        accu_10 += *cos_0.offset(3 as c_int as isize) as c_long
            * (*z.offset((o3 + 3 as c_int) as isize)).im as c_long;
        t6 = (accu_10 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
        r0 = (*z.offset(3 as c_int as isize)).re as TXUSample;
        i0 = (*z.offset(3 as c_int as isize)).im as TXUSample;
        r1 = (*z.offset((o1 + 3 as c_int) as isize)).re as TXUSample;
        i1 = (*z.offset((o1 + 3 as c_int) as isize)).im as TXUSample;
        t3 = t5.wrapping_sub(t1);
        t5 = t5.wrapping_add(t1);
        (*z.offset((o2 + 3 as c_int) as isize)).re = r0.wrapping_sub(t5) as c_int;
        (*z.offset(3 as c_int as isize)).re = r0.wrapping_add(t5) as c_int;
        (*z.offset((o3 + 3 as c_int) as isize)).im = i1.wrapping_sub(t3) as c_int;
        (*z.offset((o1 + 3 as c_int) as isize)).im = i1.wrapping_add(t3) as c_int;
        t4 = t2.wrapping_sub(t6);
        t6 = t2.wrapping_add(t6);
        (*z.offset((o3 + 3 as c_int) as isize)).re = r1.wrapping_sub(t4) as c_int;
        (*z.offset((o1 + 3 as c_int) as isize)).re = r1.wrapping_add(t4) as c_int;
        (*z.offset((o2 + 3 as c_int) as isize)).im = i0.wrapping_sub(t6) as c_int;
        (*z.offset(3 as c_int as isize)).im = i0.wrapping_add(t6) as c_int;
        let mut accu_11: c_long = 0;
        accu_11 = *cos_0.offset(5 as c_int as isize) as c_long
            * (*z.offset((o2 + 5 as c_int) as isize)).re as c_long;
        accu_11 -= -*wim.offset(2 as c_int as isize) as c_long
            * (*z.offset((o2 + 5 as c_int) as isize)).im as c_long;
        t1 = (accu_11 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
        accu_11 = -*wim.offset(2 as c_int as isize) as c_long
            * (*z.offset((o2 + 5 as c_int) as isize)).re as c_long;
        accu_11 += *cos_0.offset(5 as c_int as isize) as c_long
            * (*z.offset((o2 + 5 as c_int) as isize)).im as c_long;
        t2 = (accu_11 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
        let mut accu_12: c_long = 0;
        accu_12 = *cos_0.offset(5 as c_int as isize) as c_long
            * (*z.offset((o3 + 5 as c_int) as isize)).re as c_long;
        accu_12 -= *wim.offset(2 as c_int as isize) as c_long
            * (*z.offset((o3 + 5 as c_int) as isize)).im as c_long;
        t5 = (accu_12 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
        accu_12 = *wim.offset(2 as c_int as isize) as c_long
            * (*z.offset((o3 + 5 as c_int) as isize)).re as c_long;
        accu_12 += *cos_0.offset(5 as c_int as isize) as c_long
            * (*z.offset((o3 + 5 as c_int) as isize)).im as c_long;
        t6 = (accu_12 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
        r0 = (*z.offset(5 as c_int as isize)).re as TXUSample;
        i0 = (*z.offset(5 as c_int as isize)).im as TXUSample;
        r1 = (*z.offset((o1 + 5 as c_int) as isize)).re as TXUSample;
        i1 = (*z.offset((o1 + 5 as c_int) as isize)).im as TXUSample;
        t3 = t5.wrapping_sub(t1);
        t5 = t5.wrapping_add(t1);
        (*z.offset((o2 + 5 as c_int) as isize)).re = r0.wrapping_sub(t5) as c_int;
        (*z.offset(5 as c_int as isize)).re = r0.wrapping_add(t5) as c_int;
        (*z.offset((o3 + 5 as c_int) as isize)).im = i1.wrapping_sub(t3) as c_int;
        (*z.offset((o1 + 5 as c_int) as isize)).im = i1.wrapping_add(t3) as c_int;
        t4 = t2.wrapping_sub(t6);
        t6 = t2.wrapping_add(t6);
        (*z.offset((o3 + 5 as c_int) as isize)).re = r1.wrapping_sub(t4) as c_int;
        (*z.offset((o1 + 5 as c_int) as isize)).re = r1.wrapping_add(t4) as c_int;
        (*z.offset((o2 + 5 as c_int) as isize)).im = i0.wrapping_sub(t6) as c_int;
        (*z.offset(5 as c_int as isize)).im = i0.wrapping_add(t6) as c_int;
        let mut accu_13: c_long = 0;
        accu_13 = *cos_0.offset(7 as c_int as isize) as c_long
            * (*z.offset((o2 + 7 as c_int) as isize)).re as c_long;
        accu_13 -= -*wim.offset(0 as c_int as isize) as c_long
            * (*z.offset((o2 + 7 as c_int) as isize)).im as c_long;
        t1 = (accu_13 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
        accu_13 = -*wim.offset(0 as c_int as isize) as c_long
            * (*z.offset((o2 + 7 as c_int) as isize)).re as c_long;
        accu_13 += *cos_0.offset(7 as c_int as isize) as c_long
            * (*z.offset((o2 + 7 as c_int) as isize)).im as c_long;
        t2 = (accu_13 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
        let mut accu_14: c_long = 0;
        accu_14 = *cos_0.offset(7 as c_int as isize) as c_long
            * (*z.offset((o3 + 7 as c_int) as isize)).re as c_long;
        accu_14 -= *wim.offset(0 as c_int as isize) as c_long
            * (*z.offset((o3 + 7 as c_int) as isize)).im as c_long;
        t5 = (accu_14 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
        accu_14 = *wim.offset(0 as c_int as isize) as c_long
            * (*z.offset((o3 + 7 as c_int) as isize)).re as c_long;
        accu_14 += *cos_0.offset(7 as c_int as isize) as c_long
            * (*z.offset((o3 + 7 as c_int) as isize)).im as c_long;
        t6 = (accu_14 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
        r0 = (*z.offset(7 as c_int as isize)).re as TXUSample;
        i0 = (*z.offset(7 as c_int as isize)).im as TXUSample;
        r1 = (*z.offset((o1 + 7 as c_int) as isize)).re as TXUSample;
        i1 = (*z.offset((o1 + 7 as c_int) as isize)).im as TXUSample;
        t3 = t5.wrapping_sub(t1);
        t5 = t5.wrapping_add(t1);
        (*z.offset((o2 + 7 as c_int) as isize)).re = r0.wrapping_sub(t5) as c_int;
        (*z.offset(7 as c_int as isize)).re = r0.wrapping_add(t5) as c_int;
        (*z.offset((o3 + 7 as c_int) as isize)).im = i1.wrapping_sub(t3) as c_int;
        (*z.offset((o1 + 7 as c_int) as isize)).im = i1.wrapping_add(t3) as c_int;
        t4 = t2.wrapping_sub(t6);
        t6 = t2.wrapping_add(t6);
        (*z.offset((o3 + 7 as c_int) as isize)).re = r1.wrapping_sub(t4) as c_int;
        (*z.offset((o1 + 7 as c_int) as isize)).re = r1.wrapping_add(t4) as c_int;
        (*z.offset((o2 + 7 as c_int) as isize)).im = i0.wrapping_sub(t6) as c_int;
        (*z.offset(7 as c_int as isize)).im = i0.wrapping_add(t6) as c_int;
        z = z.offset((2 as c_int * 4 as c_int) as isize);
        cos_0 = cos_0.offset((2 as c_int * 4 as c_int) as isize);
        wim = wim.offset(-((2 as c_int * 4 as c_int) as isize));
        i += 4 as c_int;
    }
}
#[cold]
unsafe extern "C" fn ff_tx_fft_sr_codelet_init_int32_c(
    s: *mut AVTXContext,
    _cd: *const FFTXCodelet,
    _flags: c_ulong,
    opts: *mut FFTXCodeletOptions,
    len: c_int,
    _inv: c_int,
    _scale: *const c_void,
) -> c_int {
    ff_tx_init_tabs_int32(len);
    ff_tx_gen_ptwo_revtab(s, opts)
}
unsafe extern "C" fn ff_tx_fft2_ns_int32_c(
    _s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    _stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let mut tmp: TXComplex = TXComplex { re: 0, im: 0 };
    tmp.re = ((*src.offset(0 as c_int as isize)).re as c_uint)
        .wrapping_sub((*src.offset(1 as c_int as isize)).re as c_uint) as c_int;
    (*dst.offset(0 as c_int as isize)).re = ((*src.offset(0 as c_int as isize)).re as c_uint)
        .wrapping_add((*src.offset(1 as c_int as isize)).re as c_uint)
        as c_int;
    tmp.im = ((*src.offset(0 as c_int as isize)).im as c_uint)
        .wrapping_sub((*src.offset(1 as c_int as isize)).im as c_uint) as c_int;
    (*dst.offset(0 as c_int as isize)).im = ((*src.offset(0 as c_int as isize)).im as c_uint)
        .wrapping_add((*src.offset(1 as c_int as isize)).im as c_uint)
        as c_int;
    *dst.offset(1 as c_int as isize) = tmp;
}
unsafe extern "C" fn ff_tx_fft4_ns_int32_c(
    _s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    _stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let mut t1: TXSample = 0;
    let mut t2: TXSample = 0;
    let mut t3: TXSample = 0;
    let mut t4: TXSample = 0;
    let mut t5: TXSample = 0;
    let mut t6: TXSample = 0;
    let mut t7: TXSample = 0;
    let mut t8: TXSample = 0;
    t3 = ((*src.offset(0 as c_int as isize)).re as c_uint)
        .wrapping_sub((*src.offset(1 as c_int as isize)).re as c_uint) as TXSample;
    t1 = ((*src.offset(0 as c_int as isize)).re as c_uint)
        .wrapping_add((*src.offset(1 as c_int as isize)).re as c_uint) as TXSample;
    t8 = ((*src.offset(3 as c_int as isize)).re as c_uint)
        .wrapping_sub((*src.offset(2 as c_int as isize)).re as c_uint) as TXSample;
    t6 = ((*src.offset(3 as c_int as isize)).re as c_uint)
        .wrapping_add((*src.offset(2 as c_int as isize)).re as c_uint) as TXSample;
    (*dst.offset(2 as c_int as isize)).re = (t1 as c_uint).wrapping_sub(t6 as c_uint) as c_int;
    (*dst.offset(0 as c_int as isize)).re = (t1 as c_uint).wrapping_add(t6 as c_uint) as c_int;
    t4 = ((*src.offset(0 as c_int as isize)).im as c_uint)
        .wrapping_sub((*src.offset(1 as c_int as isize)).im as c_uint) as TXSample;
    t2 = ((*src.offset(0 as c_int as isize)).im as c_uint)
        .wrapping_add((*src.offset(1 as c_int as isize)).im as c_uint) as TXSample;
    t7 = ((*src.offset(2 as c_int as isize)).im as c_uint)
        .wrapping_sub((*src.offset(3 as c_int as isize)).im as c_uint) as TXSample;
    t5 = ((*src.offset(2 as c_int as isize)).im as c_uint)
        .wrapping_add((*src.offset(3 as c_int as isize)).im as c_uint) as TXSample;
    (*dst.offset(3 as c_int as isize)).im = (t4 as c_uint).wrapping_sub(t8 as c_uint) as c_int;
    (*dst.offset(1 as c_int as isize)).im = (t4 as c_uint).wrapping_add(t8 as c_uint) as c_int;
    (*dst.offset(3 as c_int as isize)).re = (t3 as c_uint).wrapping_sub(t7 as c_uint) as c_int;
    (*dst.offset(1 as c_int as isize)).re = (t3 as c_uint).wrapping_add(t7 as c_uint) as c_int;
    (*dst.offset(2 as c_int as isize)).im = (t2 as c_uint).wrapping_sub(t5 as c_uint) as c_int;
    (*dst.offset(0 as c_int as isize)).im = (t2 as c_uint).wrapping_add(t5 as c_uint) as c_int;
}
unsafe extern "C" fn ff_tx_fft8_ns_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let mut t1: TXUSample = 0;
    let mut t2: TXUSample = 0;
    let mut t3: TXUSample = 0;
    let mut t4: TXUSample = 0;
    let mut t5: TXUSample = 0;
    let mut t6: TXUSample = 0;
    let mut r0: TXUSample = 0;
    let mut i0: TXUSample = 0;
    let mut r1: TXUSample = 0;
    let mut i1: TXUSample = 0;
    let cos_0: TXSample = ff_tx_tab_8_int32[1 as c_int as usize];
    ff_tx_fft4_ns_int32_c(s, dst as *mut c_void, src as *mut c_void, stride);
    t1 = ((*src.offset(4 as c_int as isize)).re as c_uint)
        .wrapping_sub(-(*src.offset(5 as c_int as isize)).re as c_uint);
    (*dst.offset(5 as c_int as isize)).re = ((*src.offset(4 as c_int as isize)).re as c_uint)
        .wrapping_add(-(*src.offset(5 as c_int as isize)).re as c_uint)
        as c_int;
    t2 = ((*src.offset(4 as c_int as isize)).im as c_uint)
        .wrapping_sub(-(*src.offset(5 as c_int as isize)).im as c_uint);
    (*dst.offset(5 as c_int as isize)).im = ((*src.offset(4 as c_int as isize)).im as c_uint)
        .wrapping_add(-(*src.offset(5 as c_int as isize)).im as c_uint)
        as c_int;
    t5 = ((*src.offset(6 as c_int as isize)).re as c_uint)
        .wrapping_sub(-(*src.offset(7 as c_int as isize)).re as c_uint);
    (*dst.offset(7 as c_int as isize)).re = ((*src.offset(6 as c_int as isize)).re as c_uint)
        .wrapping_add(-(*src.offset(7 as c_int as isize)).re as c_uint)
        as c_int;
    t6 = ((*src.offset(6 as c_int as isize)).im as c_uint)
        .wrapping_sub(-(*src.offset(7 as c_int as isize)).im as c_uint);
    (*dst.offset(7 as c_int as isize)).im = ((*src.offset(6 as c_int as isize)).im as c_uint)
        .wrapping_add(-(*src.offset(7 as c_int as isize)).im as c_uint)
        as c_int;
    r0 = (*dst.offset(0 as c_int as isize)).re as TXUSample;
    i0 = (*dst.offset(0 as c_int as isize)).im as TXUSample;
    r1 = (*dst.offset(2 as c_int as isize)).re as TXUSample;
    i1 = (*dst.offset(2 as c_int as isize)).im as TXUSample;
    t3 = t5.wrapping_sub(t1);
    t5 = t5.wrapping_add(t1);
    (*dst.offset(4 as c_int as isize)).re = r0.wrapping_sub(t5) as c_int;
    (*dst.offset(0 as c_int as isize)).re = r0.wrapping_add(t5) as c_int;
    (*dst.offset(6 as c_int as isize)).im = i1.wrapping_sub(t3) as c_int;
    (*dst.offset(2 as c_int as isize)).im = i1.wrapping_add(t3) as c_int;
    t4 = t2.wrapping_sub(t6);
    t6 = t2.wrapping_add(t6);
    (*dst.offset(6 as c_int as isize)).re = r1.wrapping_sub(t4) as c_int;
    (*dst.offset(2 as c_int as isize)).re = r1.wrapping_add(t4) as c_int;
    (*dst.offset(4 as c_int as isize)).im = i0.wrapping_sub(t6) as c_int;
    (*dst.offset(0 as c_int as isize)).im = i0.wrapping_add(t6) as c_int;
    let mut accu: c_long = 0;
    accu = cos_0 as c_long * (*dst.offset(5 as c_int as isize)).re as c_long;
    accu -= -cos_0 as c_long * (*dst.offset(5 as c_int as isize)).im as c_long;
    t1 = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
    accu = -cos_0 as c_long * (*dst.offset(5 as c_int as isize)).re as c_long;
    accu += cos_0 as c_long * (*dst.offset(5 as c_int as isize)).im as c_long;
    t2 = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
    let mut accu_0: c_long = 0;
    accu_0 = cos_0 as c_long * (*dst.offset(7 as c_int as isize)).re as c_long;
    accu_0 -= cos_0 as c_long * (*dst.offset(7 as c_int as isize)).im as c_long;
    t5 = (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
    accu_0 = cos_0 as c_long * (*dst.offset(7 as c_int as isize)).re as c_long;
    accu_0 += cos_0 as c_long * (*dst.offset(7 as c_int as isize)).im as c_long;
    t6 = (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
    r0 = (*dst.offset(1 as c_int as isize)).re as TXUSample;
    i0 = (*dst.offset(1 as c_int as isize)).im as TXUSample;
    r1 = (*dst.offset(3 as c_int as isize)).re as TXUSample;
    i1 = (*dst.offset(3 as c_int as isize)).im as TXUSample;
    t3 = t5.wrapping_sub(t1);
    t5 = t5.wrapping_add(t1);
    (*dst.offset(5 as c_int as isize)).re = r0.wrapping_sub(t5) as c_int;
    (*dst.offset(1 as c_int as isize)).re = r0.wrapping_add(t5) as c_int;
    (*dst.offset(7 as c_int as isize)).im = i1.wrapping_sub(t3) as c_int;
    (*dst.offset(3 as c_int as isize)).im = i1.wrapping_add(t3) as c_int;
    t4 = t2.wrapping_sub(t6);
    t6 = t2.wrapping_add(t6);
    (*dst.offset(7 as c_int as isize)).re = r1.wrapping_sub(t4) as c_int;
    (*dst.offset(3 as c_int as isize)).re = r1.wrapping_add(t4) as c_int;
    (*dst.offset(5 as c_int as isize)).im = i0.wrapping_sub(t6) as c_int;
    (*dst.offset(1 as c_int as isize)).im = i0.wrapping_add(t6) as c_int;
}
unsafe extern "C" fn ff_tx_fft16_ns_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_16_int32.as_mut_ptr();
    let mut t1: TXUSample = 0;
    let mut t2: TXUSample = 0;
    let mut t3: TXUSample = 0;
    let mut t4: TXUSample = 0;
    let mut t5: TXUSample = 0;
    let mut t6: TXUSample = 0;
    let mut r0: TXUSample = 0;
    let mut i0: TXUSample = 0;
    let mut r1: TXUSample = 0;
    let mut i1: TXUSample = 0;
    let cos_16_1: TXSample = *cos_0.offset(1 as c_int as isize);
    let cos_16_2: TXSample = *cos_0.offset(2 as c_int as isize);
    let cos_16_3: TXSample = *cos_0.offset(3 as c_int as isize);
    ff_tx_fft8_ns_int32_c(
        s,
        dst.offset(0 as c_int as isize) as *mut c_void,
        src.offset(0 as c_int as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft4_ns_int32_c(
        s,
        dst.offset(8 as c_int as isize) as *mut c_void,
        src.offset(8 as c_int as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft4_ns_int32_c(
        s,
        dst.offset(12 as c_int as isize) as *mut c_void,
        src.offset(12 as c_int as isize) as *mut c_void,
        stride,
    );
    t1 = (*dst.offset(8 as c_int as isize)).re as TXUSample;
    t2 = (*dst.offset(8 as c_int as isize)).im as TXUSample;
    t5 = (*dst.offset(12 as c_int as isize)).re as TXUSample;
    t6 = (*dst.offset(12 as c_int as isize)).im as TXUSample;
    r0 = (*dst.offset(0 as c_int as isize)).re as TXUSample;
    i0 = (*dst.offset(0 as c_int as isize)).im as TXUSample;
    r1 = (*dst.offset(4 as c_int as isize)).re as TXUSample;
    i1 = (*dst.offset(4 as c_int as isize)).im as TXUSample;
    t3 = t5.wrapping_sub(t1);
    t5 = t5.wrapping_add(t1);
    (*dst.offset(8 as c_int as isize)).re = r0.wrapping_sub(t5) as c_int;
    (*dst.offset(0 as c_int as isize)).re = r0.wrapping_add(t5) as c_int;
    (*dst.offset(12 as c_int as isize)).im = i1.wrapping_sub(t3) as c_int;
    (*dst.offset(4 as c_int as isize)).im = i1.wrapping_add(t3) as c_int;
    t4 = t2.wrapping_sub(t6);
    t6 = t2.wrapping_add(t6);
    (*dst.offset(12 as c_int as isize)).re = r1.wrapping_sub(t4) as c_int;
    (*dst.offset(4 as c_int as isize)).re = r1.wrapping_add(t4) as c_int;
    (*dst.offset(8 as c_int as isize)).im = i0.wrapping_sub(t6) as c_int;
    (*dst.offset(0 as c_int as isize)).im = i0.wrapping_add(t6) as c_int;
    let mut accu: c_long = 0;
    accu = cos_16_2 as c_long * (*dst.offset(10 as c_int as isize)).re as c_long;
    accu -= -cos_16_2 as c_long * (*dst.offset(10 as c_int as isize)).im as c_long;
    t1 = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
    accu = -cos_16_2 as c_long * (*dst.offset(10 as c_int as isize)).re as c_long;
    accu += cos_16_2 as c_long * (*dst.offset(10 as c_int as isize)).im as c_long;
    t2 = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
    let mut accu_0: c_long = 0;
    accu_0 = cos_16_2 as c_long * (*dst.offset(14 as c_int as isize)).re as c_long;
    accu_0 -= cos_16_2 as c_long * (*dst.offset(14 as c_int as isize)).im as c_long;
    t5 = (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
    accu_0 = cos_16_2 as c_long * (*dst.offset(14 as c_int as isize)).re as c_long;
    accu_0 += cos_16_2 as c_long * (*dst.offset(14 as c_int as isize)).im as c_long;
    t6 = (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
    r0 = (*dst.offset(2 as c_int as isize)).re as TXUSample;
    i0 = (*dst.offset(2 as c_int as isize)).im as TXUSample;
    r1 = (*dst.offset(6 as c_int as isize)).re as TXUSample;
    i1 = (*dst.offset(6 as c_int as isize)).im as TXUSample;
    t3 = t5.wrapping_sub(t1);
    t5 = t5.wrapping_add(t1);
    (*dst.offset(10 as c_int as isize)).re = r0.wrapping_sub(t5) as c_int;
    (*dst.offset(2 as c_int as isize)).re = r0.wrapping_add(t5) as c_int;
    (*dst.offset(14 as c_int as isize)).im = i1.wrapping_sub(t3) as c_int;
    (*dst.offset(6 as c_int as isize)).im = i1.wrapping_add(t3) as c_int;
    t4 = t2.wrapping_sub(t6);
    t6 = t2.wrapping_add(t6);
    (*dst.offset(14 as c_int as isize)).re = r1.wrapping_sub(t4) as c_int;
    (*dst.offset(6 as c_int as isize)).re = r1.wrapping_add(t4) as c_int;
    (*dst.offset(10 as c_int as isize)).im = i0.wrapping_sub(t6) as c_int;
    (*dst.offset(2 as c_int as isize)).im = i0.wrapping_add(t6) as c_int;
    let mut accu_1: c_long = 0;
    accu_1 = cos_16_1 as c_long * (*dst.offset(9 as c_int as isize)).re as c_long;
    accu_1 -= -cos_16_3 as c_long * (*dst.offset(9 as c_int as isize)).im as c_long;
    t1 = (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
    accu_1 = -cos_16_3 as c_long * (*dst.offset(9 as c_int as isize)).re as c_long;
    accu_1 += cos_16_1 as c_long * (*dst.offset(9 as c_int as isize)).im as c_long;
    t2 = (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
    let mut accu_2: c_long = 0;
    accu_2 = cos_16_1 as c_long * (*dst.offset(13 as c_int as isize)).re as c_long;
    accu_2 -= cos_16_3 as c_long * (*dst.offset(13 as c_int as isize)).im as c_long;
    t5 = (accu_2 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
    accu_2 = cos_16_3 as c_long * (*dst.offset(13 as c_int as isize)).re as c_long;
    accu_2 += cos_16_1 as c_long * (*dst.offset(13 as c_int as isize)).im as c_long;
    t6 = (accu_2 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
    r0 = (*dst.offset(1 as c_int as isize)).re as TXUSample;
    i0 = (*dst.offset(1 as c_int as isize)).im as TXUSample;
    r1 = (*dst.offset(5 as c_int as isize)).re as TXUSample;
    i1 = (*dst.offset(5 as c_int as isize)).im as TXUSample;
    t3 = t5.wrapping_sub(t1);
    t5 = t5.wrapping_add(t1);
    (*dst.offset(9 as c_int as isize)).re = r0.wrapping_sub(t5) as c_int;
    (*dst.offset(1 as c_int as isize)).re = r0.wrapping_add(t5) as c_int;
    (*dst.offset(13 as c_int as isize)).im = i1.wrapping_sub(t3) as c_int;
    (*dst.offset(5 as c_int as isize)).im = i1.wrapping_add(t3) as c_int;
    t4 = t2.wrapping_sub(t6);
    t6 = t2.wrapping_add(t6);
    (*dst.offset(13 as c_int as isize)).re = r1.wrapping_sub(t4) as c_int;
    (*dst.offset(5 as c_int as isize)).re = r1.wrapping_add(t4) as c_int;
    (*dst.offset(9 as c_int as isize)).im = i0.wrapping_sub(t6) as c_int;
    (*dst.offset(1 as c_int as isize)).im = i0.wrapping_add(t6) as c_int;
    let mut accu_3: c_long = 0;
    accu_3 = cos_16_3 as c_long * (*dst.offset(11 as c_int as isize)).re as c_long;
    accu_3 -= -cos_16_1 as c_long * (*dst.offset(11 as c_int as isize)).im as c_long;
    t1 = (accu_3 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
    accu_3 = -cos_16_1 as c_long * (*dst.offset(11 as c_int as isize)).re as c_long;
    accu_3 += cos_16_3 as c_long * (*dst.offset(11 as c_int as isize)).im as c_long;
    t2 = (accu_3 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
    let mut accu_4: c_long = 0;
    accu_4 = cos_16_3 as c_long * (*dst.offset(15 as c_int as isize)).re as c_long;
    accu_4 -= cos_16_1 as c_long * (*dst.offset(15 as c_int as isize)).im as c_long;
    t5 = (accu_4 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
    accu_4 = cos_16_1 as c_long * (*dst.offset(15 as c_int as isize)).re as c_long;
    accu_4 += cos_16_3 as c_long * (*dst.offset(15 as c_int as isize)).im as c_long;
    t6 = (accu_4 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int as TXUSample;
    r0 = (*dst.offset(3 as c_int as isize)).re as TXUSample;
    i0 = (*dst.offset(3 as c_int as isize)).im as TXUSample;
    r1 = (*dst.offset(7 as c_int as isize)).re as TXUSample;
    i1 = (*dst.offset(7 as c_int as isize)).im as TXUSample;
    t3 = t5.wrapping_sub(t1);
    t5 = t5.wrapping_add(t1);
    (*dst.offset(11 as c_int as isize)).re = r0.wrapping_sub(t5) as c_int;
    (*dst.offset(3 as c_int as isize)).re = r0.wrapping_add(t5) as c_int;
    (*dst.offset(15 as c_int as isize)).im = i1.wrapping_sub(t3) as c_int;
    (*dst.offset(7 as c_int as isize)).im = i1.wrapping_add(t3) as c_int;
    t4 = t2.wrapping_sub(t6);
    t6 = t2.wrapping_add(t6);
    (*dst.offset(15 as c_int as isize)).re = r1.wrapping_sub(t4) as c_int;
    (*dst.offset(7 as c_int as isize)).re = r1.wrapping_add(t4) as c_int;
    (*dst.offset(11 as c_int as isize)).im = i0.wrapping_sub(t6) as c_int;
    (*dst.offset(3 as c_int as isize)).im = i0.wrapping_add(t6) as c_int;
}
static mut ff_tx_fft2_ns_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft2_ns_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft2_ns_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: ((1 as c_ulonglong) << 63 as c_int
                | AV_TX_INPLACE as c_int as c_ulonglong
                | AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 61 as c_int) as c_ulong,
            factors: [2 as c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 2 as c_int,
            max_len: 2 as c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
static mut ff_tx_fft4_ns_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft4_ns_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft4_ns_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: ((1 as c_ulonglong) << 63 as c_int
                | AV_TX_INPLACE as c_int as c_ulonglong
                | AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 61 as c_int) as c_ulong,
            factors: [2 as c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 4 as c_int,
            max_len: 4 as c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
static mut ff_tx_fft8_ns_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft8_ns_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft8_ns_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: ((1 as c_ulonglong) << 63 as c_int
                | AV_TX_INPLACE as c_int as c_ulonglong
                | AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 61 as c_int) as c_ulong,
            factors: [2 as c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 8 as c_int,
            max_len: 8 as c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
static mut ff_tx_fft16_ns_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft16_ns_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft16_ns_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: ((1 as c_ulonglong) << 63 as c_int
                | AV_TX_INPLACE as c_int as c_ulonglong
                | AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 61 as c_int) as c_ulong,
            factors: [2 as c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 16 as c_int,
            max_len: 16 as c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
static mut ff_tx_fft32_ns_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft32_ns_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft32_ns_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: ((1 as c_ulonglong) << 63 as c_int
                | AV_TX_INPLACE as c_int as c_ulonglong
                | AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 61 as c_int) as c_ulong,
            factors: [2 as c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 32 as c_int,
            max_len: 32 as c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
unsafe extern "C" fn ff_tx_fft32_ns_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_32_int32.as_mut_ptr();
    ff_tx_fft16_ns_int32_c(s, dst as *mut c_void, src as *mut c_void, stride);
    ff_tx_fft8_ns_int32_c(
        s,
        dst.offset((8 as c_int * 2 as c_int) as isize) as *mut c_void,
        src.offset((8 as c_int * 2 as c_int) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft8_ns_int32_c(
        s,
        dst.offset((8 as c_int * 3 as c_int) as isize) as *mut c_void,
        src.offset((8 as c_int * 3 as c_int) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft_sr_combine_int32_c(dst, cos_0, 8 as c_int >> 1 as c_int);
}
static mut ff_tx_fft64_ns_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft64_ns_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft64_ns_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: ((1 as c_ulonglong) << 63 as c_int
                | AV_TX_INPLACE as c_int as c_ulonglong
                | AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 61 as c_int) as c_ulong,
            factors: [2 as c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 64 as c_int,
            max_len: 64 as c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
unsafe extern "C" fn ff_tx_fft64_ns_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_64_int32.as_mut_ptr();
    ff_tx_fft32_ns_int32_c(s, dst as *mut c_void, src as *mut c_void, stride);
    ff_tx_fft16_ns_int32_c(
        s,
        dst.offset((16 as c_int * 2 as c_int) as isize) as *mut c_void,
        src.offset((16 as c_int * 2 as c_int) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft16_ns_int32_c(
        s,
        dst.offset((16 as c_int * 3 as c_int) as isize) as *mut c_void,
        src.offset((16 as c_int * 3 as c_int) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft_sr_combine_int32_c(dst, cos_0, 16 as c_int >> 1 as c_int);
}
unsafe extern "C" fn ff_tx_fft128_ns_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_128_int32.as_mut_ptr();
    ff_tx_fft64_ns_int32_c(s, dst as *mut c_void, src as *mut c_void, stride);
    ff_tx_fft32_ns_int32_c(
        s,
        dst.offset((32 as c_int * 2 as c_int) as isize) as *mut c_void,
        src.offset((32 as c_int * 2 as c_int) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft32_ns_int32_c(
        s,
        dst.offset((32 as c_int * 3 as c_int) as isize) as *mut c_void,
        src.offset((32 as c_int * 3 as c_int) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft_sr_combine_int32_c(dst, cos_0, 32 as c_int >> 1 as c_int);
}
static mut ff_tx_fft128_ns_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft128_ns_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft128_ns_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: ((1 as c_ulonglong) << 63 as c_int
                | AV_TX_INPLACE as c_int as c_ulonglong
                | AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 61 as c_int) as c_ulong,
            factors: [2 as c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 128 as c_int,
            max_len: 128 as c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
unsafe extern "C" fn ff_tx_fft256_ns_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_256_int32.as_mut_ptr();
    ff_tx_fft128_ns_int32_c(s, dst as *mut c_void, src as *mut c_void, stride);
    ff_tx_fft64_ns_int32_c(
        s,
        dst.offset((64 as c_int * 2 as c_int) as isize) as *mut c_void,
        src.offset((64 as c_int * 2 as c_int) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft64_ns_int32_c(
        s,
        dst.offset((64 as c_int * 3 as c_int) as isize) as *mut c_void,
        src.offset((64 as c_int * 3 as c_int) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft_sr_combine_int32_c(dst, cos_0, 64 as c_int >> 1 as c_int);
}
static mut ff_tx_fft256_ns_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft256_ns_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft256_ns_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: ((1 as c_ulonglong) << 63 as c_int
                | AV_TX_INPLACE as c_int as c_ulonglong
                | AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 61 as c_int) as c_ulong,
            factors: [2 as c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 256 as c_int,
            max_len: 256 as c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
unsafe extern "C" fn ff_tx_fft512_ns_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_512_int32.as_mut_ptr();
    ff_tx_fft256_ns_int32_c(s, dst as *mut c_void, src as *mut c_void, stride);
    ff_tx_fft128_ns_int32_c(
        s,
        dst.offset((128 as c_int * 2 as c_int) as isize) as *mut c_void,
        src.offset((128 as c_int * 2 as c_int) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft128_ns_int32_c(
        s,
        dst.offset((128 as c_int * 3 as c_int) as isize) as *mut c_void,
        src.offset((128 as c_int * 3 as c_int) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft_sr_combine_int32_c(dst, cos_0, 128 as c_int >> 1 as c_int);
}
static mut ff_tx_fft512_ns_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft512_ns_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft512_ns_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: ((1 as c_ulonglong) << 63 as c_int
                | AV_TX_INPLACE as c_int as c_ulonglong
                | AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 61 as c_int) as c_ulong,
            factors: [2 as c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 512 as c_int,
            max_len: 512 as c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
static mut ff_tx_fft1024_ns_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft1024_ns_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft1024_ns_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: ((1 as c_ulonglong) << 63 as c_int
                | AV_TX_INPLACE as c_int as c_ulonglong
                | AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 61 as c_int) as c_ulong,
            factors: [2 as c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 1024 as c_int,
            max_len: 1024 as c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
unsafe extern "C" fn ff_tx_fft1024_ns_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_1024_int32.as_mut_ptr();
    ff_tx_fft512_ns_int32_c(s, dst as *mut c_void, src as *mut c_void, stride);
    ff_tx_fft256_ns_int32_c(
        s,
        dst.offset((256 as c_int * 2 as c_int) as isize) as *mut c_void,
        src.offset((256 as c_int * 2 as c_int) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft256_ns_int32_c(
        s,
        dst.offset((256 as c_int * 3 as c_int) as isize) as *mut c_void,
        src.offset((256 as c_int * 3 as c_int) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft_sr_combine_int32_c(dst, cos_0, 256 as c_int >> 1 as c_int);
}
static mut ff_tx_fft2048_ns_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft2048_ns_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft2048_ns_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: ((1 as c_ulonglong) << 63 as c_int
                | AV_TX_INPLACE as c_int as c_ulonglong
                | AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 61 as c_int) as c_ulong,
            factors: [2 as c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 2048 as c_int,
            max_len: 2048 as c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
unsafe extern "C" fn ff_tx_fft2048_ns_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_2048_int32.as_mut_ptr();
    ff_tx_fft1024_ns_int32_c(s, dst as *mut c_void, src as *mut c_void, stride);
    ff_tx_fft512_ns_int32_c(
        s,
        dst.offset((512 as c_int * 2 as c_int) as isize) as *mut c_void,
        src.offset((512 as c_int * 2 as c_int) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft512_ns_int32_c(
        s,
        dst.offset((512 as c_int * 3 as c_int) as isize) as *mut c_void,
        src.offset((512 as c_int * 3 as c_int) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft_sr_combine_int32_c(dst, cos_0, 512 as c_int >> 1 as c_int);
}
unsafe extern "C" fn ff_tx_fft4096_ns_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_4096_int32.as_mut_ptr();
    ff_tx_fft2048_ns_int32_c(s, dst as *mut c_void, src as *mut c_void, stride);
    ff_tx_fft1024_ns_int32_c(
        s,
        dst.offset((1024 as c_int * 2 as c_int) as isize) as *mut c_void,
        src.offset((1024 as c_int * 2 as c_int) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft1024_ns_int32_c(
        s,
        dst.offset((1024 as c_int * 3 as c_int) as isize) as *mut c_void,
        src.offset((1024 as c_int * 3 as c_int) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft_sr_combine_int32_c(dst, cos_0, 1024 as c_int >> 1 as c_int);
}
static mut ff_tx_fft4096_ns_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft4096_ns_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft4096_ns_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: ((1 as c_ulonglong) << 63 as c_int
                | AV_TX_INPLACE as c_int as c_ulonglong
                | AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 61 as c_int) as c_ulong,
            factors: [2 as c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 4096 as c_int,
            max_len: 4096 as c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
static mut ff_tx_fft8192_ns_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft8192_ns_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft8192_ns_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: ((1 as c_ulonglong) << 63 as c_int
                | AV_TX_INPLACE as c_int as c_ulonglong
                | AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 61 as c_int) as c_ulong,
            factors: [2 as c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 8192 as c_int,
            max_len: 8192 as c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
unsafe extern "C" fn ff_tx_fft8192_ns_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_8192_int32.as_mut_ptr();
    ff_tx_fft4096_ns_int32_c(s, dst as *mut c_void, src as *mut c_void, stride);
    ff_tx_fft2048_ns_int32_c(
        s,
        dst.offset((2048 as c_int * 2 as c_int) as isize) as *mut c_void,
        src.offset((2048 as c_int * 2 as c_int) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft2048_ns_int32_c(
        s,
        dst.offset((2048 as c_int * 3 as c_int) as isize) as *mut c_void,
        src.offset((2048 as c_int * 3 as c_int) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft_sr_combine_int32_c(dst, cos_0, 2048 as c_int >> 1 as c_int);
}
unsafe extern "C" fn ff_tx_fft16384_ns_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_16384_int32.as_mut_ptr();
    ff_tx_fft8192_ns_int32_c(s, dst as *mut c_void, src as *mut c_void, stride);
    ff_tx_fft4096_ns_int32_c(
        s,
        dst.offset((4096 as c_int * 2 as c_int) as isize) as *mut c_void,
        src.offset((4096 as c_int * 2 as c_int) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft4096_ns_int32_c(
        s,
        dst.offset((4096 as c_int * 3 as c_int) as isize) as *mut c_void,
        src.offset((4096 as c_int * 3 as c_int) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft_sr_combine_int32_c(dst, cos_0, 4096 as c_int >> 1 as c_int);
}
static mut ff_tx_fft16384_ns_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft16384_ns_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft16384_ns_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: ((1 as c_ulonglong) << 63 as c_int
                | AV_TX_INPLACE as c_int as c_ulonglong
                | AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 61 as c_int) as c_ulong,
            factors: [2 as c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 16384 as c_int,
            max_len: 16384 as c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
static mut ff_tx_fft32768_ns_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft32768_ns_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft32768_ns_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: ((1 as c_ulonglong) << 63 as c_int
                | AV_TX_INPLACE as c_int as c_ulonglong
                | AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 61 as c_int) as c_ulong,
            factors: [2 as c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 32768 as c_int,
            max_len: 32768 as c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
unsafe extern "C" fn ff_tx_fft32768_ns_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_32768_int32.as_mut_ptr();
    ff_tx_fft16384_ns_int32_c(s, dst as *mut c_void, src as *mut c_void, stride);
    ff_tx_fft8192_ns_int32_c(
        s,
        dst.offset((8192 as c_int * 2 as c_int) as isize) as *mut c_void,
        src.offset((8192 as c_int * 2 as c_int) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft8192_ns_int32_c(
        s,
        dst.offset((8192 as c_int * 3 as c_int) as isize) as *mut c_void,
        src.offset((8192 as c_int * 3 as c_int) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft_sr_combine_int32_c(dst, cos_0, 8192 as c_int >> 1 as c_int);
}
unsafe extern "C" fn ff_tx_fft65536_ns_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_65536_int32.as_mut_ptr();
    ff_tx_fft32768_ns_int32_c(s, dst as *mut c_void, src as *mut c_void, stride);
    ff_tx_fft16384_ns_int32_c(
        s,
        dst.offset((16384 as c_int * 2 as c_int) as isize) as *mut c_void,
        src.offset((16384 as c_int * 2 as c_int) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft16384_ns_int32_c(
        s,
        dst.offset((16384 as c_int * 3 as c_int) as isize) as *mut c_void,
        src.offset((16384 as c_int * 3 as c_int) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft_sr_combine_int32_c(dst, cos_0, 16384 as c_int >> 1 as c_int);
}
static mut ff_tx_fft65536_ns_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft65536_ns_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft65536_ns_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: ((1 as c_ulonglong) << 63 as c_int
                | AV_TX_INPLACE as c_int as c_ulonglong
                | AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 61 as c_int) as c_ulong,
            factors: [2 as c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 65536 as c_int,
            max_len: 65536 as c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
unsafe extern "C" fn ff_tx_fft131072_ns_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_131072_int32.as_mut_ptr();
    ff_tx_fft65536_ns_int32_c(s, dst as *mut c_void, src as *mut c_void, stride);
    ff_tx_fft32768_ns_int32_c(
        s,
        dst.offset((32768 as c_int * 2 as c_int) as isize) as *mut c_void,
        src.offset((32768 as c_int * 2 as c_int) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft32768_ns_int32_c(
        s,
        dst.offset((32768 as c_int * 3 as c_int) as isize) as *mut c_void,
        src.offset((32768 as c_int * 3 as c_int) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft_sr_combine_int32_c(dst, cos_0, 32768 as c_int >> 1 as c_int);
}
static mut ff_tx_fft131072_ns_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft131072_ns_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft131072_ns_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: ((1 as c_ulonglong) << 63 as c_int
                | AV_TX_INPLACE as c_int as c_ulonglong
                | AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 61 as c_int) as c_ulong,
            factors: [2 as c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 131072 as c_int,
            max_len: 131072 as c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
unsafe extern "C" fn ff_tx_fft262144_ns_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_262144_int32.as_mut_ptr();
    ff_tx_fft131072_ns_int32_c(s, dst as *mut c_void, src as *mut c_void, stride);
    ff_tx_fft65536_ns_int32_c(
        s,
        dst.offset((65536 as c_int * 2 as c_int) as isize) as *mut c_void,
        src.offset((65536 as c_int * 2 as c_int) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft65536_ns_int32_c(
        s,
        dst.offset((65536 as c_int * 3 as c_int) as isize) as *mut c_void,
        src.offset((65536 as c_int * 3 as c_int) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft_sr_combine_int32_c(dst, cos_0, 65536 as c_int >> 1 as c_int);
}
static mut ff_tx_fft262144_ns_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft262144_ns_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft262144_ns_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: ((1 as c_ulonglong) << 63 as c_int
                | AV_TX_INPLACE as c_int as c_ulonglong
                | AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 61 as c_int) as c_ulong,
            factors: [2 as c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 262144 as c_int,
            max_len: 262144 as c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
static mut ff_tx_fft524288_ns_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft524288_ns_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft524288_ns_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: ((1 as c_ulonglong) << 63 as c_int
                | AV_TX_INPLACE as c_int as c_ulonglong
                | AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 61 as c_int) as c_ulong,
            factors: [2 as c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 524288 as c_int,
            max_len: 524288 as c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
unsafe extern "C" fn ff_tx_fft524288_ns_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_524288_int32.as_mut_ptr();
    ff_tx_fft262144_ns_int32_c(s, dst as *mut c_void, src as *mut c_void, stride);
    ff_tx_fft131072_ns_int32_c(
        s,
        dst.offset((131072 as c_int * 2 as c_int) as isize) as *mut c_void,
        src.offset((131072 as c_int * 2 as c_int) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft131072_ns_int32_c(
        s,
        dst.offset((131072 as c_int * 3 as c_int) as isize) as *mut c_void,
        src.offset((131072 as c_int * 3 as c_int) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft_sr_combine_int32_c(dst, cos_0, 131072 as c_int >> 1 as c_int);
}
unsafe extern "C" fn ff_tx_fft1048576_ns_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_1048576_int32.as_mut_ptr();
    ff_tx_fft524288_ns_int32_c(s, dst as *mut c_void, src as *mut c_void, stride);
    ff_tx_fft262144_ns_int32_c(
        s,
        dst.offset((262144 as c_int * 2 as c_int) as isize) as *mut c_void,
        src.offset((262144 as c_int * 2 as c_int) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft262144_ns_int32_c(
        s,
        dst.offset((262144 as c_int * 3 as c_int) as isize) as *mut c_void,
        src.offset((262144 as c_int * 3 as c_int) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft_sr_combine_int32_c(dst, cos_0, 262144 as c_int >> 1 as c_int);
}
static mut ff_tx_fft1048576_ns_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft1048576_ns_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft1048576_ns_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: ((1 as c_ulonglong) << 63 as c_int
                | AV_TX_INPLACE as c_int as c_ulonglong
                | AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 61 as c_int) as c_ulong,
            factors: [2 as c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 1048576 as c_int,
            max_len: 1048576 as c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
unsafe extern "C" fn ff_tx_fft2097152_ns_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_2097152_int32.as_mut_ptr();
    ff_tx_fft1048576_ns_int32_c(s, dst as *mut c_void, src as *mut c_void, stride);
    ff_tx_fft524288_ns_int32_c(
        s,
        dst.offset((524288 as c_int * 2 as c_int) as isize) as *mut c_void,
        src.offset((524288 as c_int * 2 as c_int) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft524288_ns_int32_c(
        s,
        dst.offset((524288 as c_int * 3 as c_int) as isize) as *mut c_void,
        src.offset((524288 as c_int * 3 as c_int) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft_sr_combine_int32_c(dst, cos_0, 524288 as c_int >> 1 as c_int);
}
static mut ff_tx_fft2097152_ns_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft2097152_ns_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft2097152_ns_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: ((1 as c_ulonglong) << 63 as c_int
                | AV_TX_INPLACE as c_int as c_ulonglong
                | AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 61 as c_int) as c_ulong,
            factors: [2 as c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 2097152 as c_int,
            max_len: 2097152 as c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
#[cold]
unsafe extern "C" fn ff_tx_fft_init_int32_c(
    s: *mut AVTXContext,
    _cd: *const FFTXCodelet,
    mut flags: c_ulong,
    _opts: *mut FFTXCodeletOptions,
    len: c_int,
    inv: c_int,
    scale: *const c_void,
) -> c_int {
    let mut ret: c_int = 0;
    let is_inplace: c_int = (flags & AV_TX_INPLACE as c_int as c_ulong != 0) as c_int;
    let mut sub_opts: FFTXCodeletOptions = {
        FFTXCodeletOptions {
            map_dir: (if is_inplace != 0 {
                FF_TX_MAP_SCATTER as c_int
            } else {
                FF_TX_MAP_GATHER as c_int
            }) as FFTXMapDirection,
        }
    };
    flags = (flags as c_ulonglong & !((1 as c_ulonglong) << 63 as c_int)) as c_ulong;
    flags |= AV_TX_INPLACE as c_int as c_ulong;
    flags = (flags as c_ulonglong | (1 as c_ulonglong) << 61 as c_int) as c_ulong;
    ret = ff_tx_init_subtx(s, AV_TX_INT32_FFT, flags, &mut sub_opts, len, inv, scale);
    if ret != 0 {
        return ret;
    }
    if is_inplace != 0 && {
        ret = ff_tx_gen_inplace_map(s, len);
        ret != 0
    } {
        return ret;
    }
    0 as c_int
}
#[cold]
unsafe extern "C" fn ff_tx_fft_inplace_small_init_int32_c(
    s: *mut AVTXContext,
    cd: *const FFTXCodelet,
    mut flags: c_ulong,
    opts: *mut FFTXCodeletOptions,
    len: c_int,
    inv: c_int,
    scale: *const c_void,
) -> c_int {
    (*s).tmp = AVTXNum {
        int32: av_malloc((len as c_ulong).wrapping_mul(size_of::<TXComplex>() as c_ulong))
            as *mut TXComplex,
    };
    if ((*s).tmp).int32.is_null() {
        return -(12 as c_int);
    }
    flags &= !(AV_TX_INPLACE as c_int) as c_ulong;
    ff_tx_fft_init_int32_c(s, cd, flags, opts, len, inv, scale)
}
unsafe extern "C" fn ff_tx_fft_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst1: *mut TXComplex = (if (*s).flags & AV_TX_INPLACE as c_int as c_ulong != 0 {
        (*s).tmp.int32 as *mut c_void
    } else {
        _dst
    }) as *mut TXComplex;
    let dst2: *mut TXComplex = _dst as *mut TXComplex;
    let map: *mut c_int = (*((*s).sub).offset(0 as c_int as isize)).map;
    let len: c_int = (*s).len;
    let mut i: c_int = 0 as c_int;
    while i < len {
        *dst1.offset(i as isize) = *src.offset(*map.offset(i as isize) as isize);
        i += 1;
        i;
    }
    ((*s).fn_0[0 as c_int as usize]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0 as c_int as isize),
        dst2 as *mut c_void,
        dst1 as *mut c_void,
        stride,
    );
}
unsafe extern "C" fn ff_tx_fft_inplace_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let mut tmp: TXComplex = TXComplex { re: 0, im: 0 };
    let map: *const c_int = (*(*s).sub).map;
    let mut inplace_idx: *const c_int = (*s).map;
    let mut src_idx: c_int = 0;
    let mut dst_idx: c_int = 0;
    let fresh20 = inplace_idx;
    inplace_idx = inplace_idx.offset(1);
    src_idx = *fresh20;
    loop {
        tmp = *src.offset(src_idx as isize);
        dst_idx = *map.offset(src_idx as isize);
        loop {
            let SWAP_tmp: TXComplex = *src.offset(dst_idx as isize);
            *src.offset(dst_idx as isize) = tmp;
            tmp = SWAP_tmp;
            dst_idx = *map.offset(dst_idx as isize);
            if !(dst_idx != src_idx) {
                break;
            }
        }
        *src.offset(dst_idx as isize) = tmp;
        let fresh21 = inplace_idx;
        inplace_idx = inplace_idx.offset(1);
        src_idx = *fresh21;
        if !(src_idx != 0) {
            break;
        }
    }
    ((*s).fn_0[0 as c_int as usize]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0 as c_int as isize),
        dst as *mut c_void,
        src as *mut c_void,
        stride,
    );
}
static mut ff_tx_fft_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: (AV_TX_UNALIGNED as c_int as c_ulonglong | (1 as c_ulonglong) << 63 as c_int)
                as c_ulong,
            factors: [-(1 as c_int), 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 2 as c_int,
            max_len: -(1 as c_int),
            init: Some(
                ff_tx_fft_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
static mut ff_tx_fft_inplace_small_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft_inplace_small_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: (AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | AV_TX_INPLACE as c_int as c_ulonglong) as c_ulong,
            factors: [-(1 as c_int), 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 2 as c_int,
            max_len: 65536 as c_int,
            init: Some(
                ff_tx_fft_inplace_small_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int - 256 as c_int,
        }
    }
};
static mut ff_tx_fft_inplace_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft_inplace_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft_inplace_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: (AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | AV_TX_INPLACE as c_int as c_ulonglong) as c_ulong,
            factors: [-(1 as c_int), 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 2 as c_int,
            max_len: -(1 as c_int),
            init: Some(
                ff_tx_fft_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int - 512 as c_int,
        }
    }
};
#[cold]
unsafe extern "C" fn ff_tx_fft_init_naive_small_int32_c(
    s: *mut AVTXContext,
    _cd: *const FFTXCodelet,
    _flags: c_ulong,
    _opts: *mut FFTXCodeletOptions,
    len: c_int,
    _inv: c_int,
    _scale: *const c_void,
) -> c_int {
    let phase: c_double = if (*s).inv != 0 {
        2.0f64 * PI / len as c_double
    } else {
        -2.0f64 * PI / len as c_double
    };
    (*s).exp = AVTXNum {
        int32: av_malloc(((len * len) as c_ulong).wrapping_mul(size_of::<TXComplex>() as c_ulong))
            as *mut TXComplex,
    };
    if ((*s).exp).int32.is_null() {
        return -(12 as c_int);
    }
    let mut i: c_int = 0 as c_int;
    while i < len {
        let mut j: c_int = 0 as c_int;
        while j < len {
            let factor: c_double = phase * i as c_double * j as c_double;
            *((*s).exp).int32.offset((i * j) as isize) = {
                AVComplexInt32 {
                    re: av_clip64_c(
                        llrintf((cos(factor) * 2147483648.0f64) as c_float) as c_long,
                        (-(2147483647 as c_int) - 1 as c_int) as c_long,
                        2147483647 as c_int as c_long,
                    ) as c_int,
                    im: av_clip64_c(
                        llrintf((sin(factor) * 2147483648.0f64) as c_float) as c_long,
                        (-(2147483647 as c_int) - 1 as c_int) as c_long,
                        2147483647 as c_int as c_long,
                    ) as c_int,
                }
            };
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    0 as c_int
}
unsafe extern "C" fn ff_tx_fft_naive_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let n: c_int = (*s).len;
    let phase: c_double = if (*s).inv != 0 {
        2.0f64 * PI / n as c_double
    } else {
        -2.0f64 * PI / n as c_double
    };
    stride = (stride as c_ulong).wrapping_div(size_of::<TXComplex>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: c_int = 0 as c_int;
    while i < n {
        let mut tmp: TXComplex = {
            AVComplexInt32 {
                re: 0 as c_int,
                im: 0,
            }
        };
        let mut j: c_int = 0 as c_int;
        while j < n {
            let factor: c_double = phase * i as c_double * j as c_double;
            let mult: TXComplex = {
                AVComplexInt32 {
                    re: av_clip64_c(
                        llrintf((cos(factor) * 2147483648.0f64) as c_float) as c_long,
                        (-(2147483647 as c_int) - 1 as c_int) as c_long,
                        2147483647 as c_int as c_long,
                    ) as c_int,
                    im: av_clip64_c(
                        llrintf((sin(factor) * 2147483648.0f64) as c_float) as c_long,
                        (-(2147483647 as c_int) - 1 as c_int) as c_long,
                        2147483647 as c_int as c_long,
                    ) as c_int,
                }
            };
            let mut res: TXComplex = TXComplex { re: 0, im: 0 };
            let mut accu: c_long = 0;
            accu = mult.re as c_long * (*src.offset(j as isize)).re as c_long;
            accu -= mult.im as c_long * (*src.offset(j as isize)).im as c_long;
            res.re = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
            accu = mult.im as c_long * (*src.offset(j as isize)).re as c_long;
            accu += mult.re as c_long * (*src.offset(j as isize)).im as c_long;
            res.im = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
            tmp.re += res.re;
            tmp.im += res.im;
            j += 1;
            j;
        }
        *dst.offset((i as c_long * stride) as isize) = tmp;
        i += 1;
        i;
    }
}
unsafe extern "C" fn ff_tx_fft_naive_small_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let n: c_int = (*s).len;
    stride = (stride as c_ulong).wrapping_div(size_of::<TXComplex>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: c_int = 0 as c_int;
    while i < n {
        let mut tmp: TXComplex = {
            AVComplexInt32 {
                re: 0 as c_int,
                im: 0,
            }
        };
        let mut j: c_int = 0 as c_int;
        while j < n {
            let mut res: TXComplex = TXComplex { re: 0, im: 0 };
            let mult: TXComplex = *((*s).exp).int32.offset((i * j) as isize);
            let mut accu: c_long = 0;
            accu = mult.re as c_long * (*src.offset(j as isize)).re as c_long;
            accu -= mult.im as c_long * (*src.offset(j as isize)).im as c_long;
            res.re = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
            accu = mult.im as c_long * (*src.offset(j as isize)).re as c_long;
            accu += mult.re as c_long * (*src.offset(j as isize)).im as c_long;
            res.im = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
            tmp.re += res.re;
            tmp.im += res.im;
            j += 1;
            j;
        }
        *dst.offset((i as c_long * stride) as isize) = tmp;
        i += 1;
        i;
    }
}
static mut ff_tx_fft_naive_small_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft_naive_small_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft_naive_small_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: (AV_TX_UNALIGNED as c_int as c_ulonglong | (1 as c_ulonglong) << 63 as c_int)
                as c_ulong,
            factors: [-(1 as c_int), 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 2 as c_int,
            max_len: 1024 as c_int,
            init: Some(
                ff_tx_fft_init_naive_small_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_MIN as c_int / 2 as c_int,
        }
    }
};
static mut ff_tx_fft_naive_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft_naive_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft_naive_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: (AV_TX_UNALIGNED as c_int as c_ulonglong | (1 as c_ulonglong) << 63 as c_int)
                as c_ulong,
            factors: [-(1 as c_int), 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 1 as c_int,
            min_len: 2 as c_int,
            max_len: -(1 as c_int),
            init: None,
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_MIN as c_int,
        }
    }
};
#[cold]
unsafe extern "C" fn ff_tx_fft_pfa_init_int32_c(
    s: *mut AVTXContext,
    _cd: *const FFTXCodelet,
    mut flags: c_ulong,
    opts: *mut FFTXCodeletOptions,
    len: c_int,
    inv: c_int,
    scale: *const c_void,
) -> c_int {
    let mut ret: c_int = 0;
    let mut tmp: *mut c_int = std::ptr::null_mut::<c_int>();
    let ps: c_int = (flags as c_ulonglong & (1 as c_ulonglong) << 61 as c_int) as c_int;
    let mut sub_opts: FFTXCodeletOptions = {
        FFTXCodeletOptions {
            map_dir: FF_TX_MAP_GATHER,
        }
    };
    let mut extra_tmp_len: c_ulong = 0 as c_int as c_ulong;
    let mut len_list: [c_int; 512] = [0; 512];
    ret = ff_tx_decompose_length(len_list.as_mut_ptr(), AV_TX_INT32_FFT, len, inv);
    if ret < 0 as c_int {
        return ret;
    }
    let mut current_block_30: u64;
    let mut i: c_int = 0 as c_int;
    's_17: while i < ret {
        let mut len1: c_int = len_list[i as usize];
        let mut len2: c_int = len / len1;
        if len2 & len2 - 1 as c_int != 0 {
            std::mem::swap(&mut len2, &mut len1);
        }
        ff_tx_clear_ctx(s);
        sub_opts.map_dir = FF_TX_MAP_GATHER;
        flags &= !(AV_TX_INPLACE as c_int) as c_ulong;
        flags = (flags as c_ulonglong | (1 as c_ulonglong) << 63 as c_int) as c_ulong;
        flags = (flags as c_ulonglong | (1 as c_ulonglong) << 61 as c_int) as c_ulong;
        ret = ff_tx_init_subtx(s, AV_TX_INT32_FFT, flags, &mut sub_opts, len1, inv, scale);
        if ret == -(12 as c_int) {
            return ret;
        } else {
            if ret < 0 as c_int {
                flags = (flags as c_ulonglong & !((1 as c_ulonglong) << 61 as c_int)) as c_ulong;
                ret = ff_tx_init_subtx(s, AV_TX_INT32_FFT, flags, &mut sub_opts, len1, inv, scale);
                if ret == -(12 as c_int) {
                    return ret;
                } else if ret < 0 as c_int {
                    current_block_30 = 10680521327981672866;
                } else {
                    current_block_30 = 26972500619410423;
                }
            } else {
                current_block_30 = 26972500619410423;
            }
            match current_block_30 {
                26972500619410423 => {
                    sub_opts.map_dir = FF_TX_MAP_SCATTER;
                    flags = (flags as c_ulonglong | (1 as c_ulonglong) << 61 as c_int) as c_ulong;
                    loop {
                        flags = (flags as c_ulonglong & !((1 as c_ulonglong) << 63 as c_int))
                            as c_ulong;
                        flags |= AV_TX_INPLACE as c_int as c_ulong;
                        ret = ff_tx_init_subtx(
                            s,
                            AV_TX_INT32_FFT,
                            flags,
                            &mut sub_opts,
                            len2,
                            inv,
                            scale,
                        );
                        if ret == -(12 as c_int) {
                            return ret;
                        } else {
                            if !(ret < 0 as c_int) {
                                break 's_17;
                            }
                            flags = (flags as c_ulonglong | (1 as c_ulonglong) << 63 as c_int)
                                as c_ulong;
                            flags &= !(AV_TX_INPLACE as c_int) as c_ulong;
                            ret = ff_tx_init_subtx(
                                s,
                                AV_TX_INT32_FFT,
                                flags,
                                &mut sub_opts,
                                len2,
                                inv,
                                scale,
                            );
                            if ret == -(12 as c_int) {
                                return ret;
                            } else {
                                if !(ret < 0 as c_int) {
                                    break 's_17;
                                }
                                if !(flags as c_ulonglong & (1 as c_ulonglong) << 61 as c_int != 0)
                                {
                                    break;
                                }
                                flags = (flags as c_ulonglong
                                    & !((1 as c_ulonglong) << 61 as c_int))
                                    as c_ulong;
                            }
                        }
                    }
                }
                _ => {}
            }
            i += 1;
            i;
        }
    }
    if ret < 0 as c_int {
        return ret;
    }
    ret = ff_tx_gen_compound_mapping(
        s,
        opts,
        0 as c_int,
        (*((*s).sub).offset(0 as c_int as isize)).len,
        (*((*s).sub).offset(1 as c_int as isize)).len,
    );
    if ret != 0 {
        return ret;
    }
    (*s).tmp = AVTXNum {
        int32: av_malloc((len as c_ulong).wrapping_mul(size_of::<TXComplex>() as c_ulong))
            as *mut TXComplex,
    };
    if ((*s).tmp).int32.is_null() {
        return -(12 as c_int);
    }
    tmp = (*s).tmp.int32 as *mut c_int;
    let mut k: c_int = 0 as c_int;
    while k < len {
        memcpy(
            tmp as *mut c_void,
            &mut *((*s).map).offset(k as isize) as *mut c_int as *const c_void,
            ((*((*s).sub).offset(0 as c_int as isize)).len as c_ulong)
                .wrapping_mul(size_of::<c_int>() as c_ulong),
        );
        let mut i_0: c_int = 0 as c_int;
        while i_0 < (*((*s).sub).offset(0 as c_int as isize)).len {
            *((*s).map).offset((k + i_0) as isize) = *tmp.offset(
                *((*((*s).sub).offset(0 as c_int as isize)).map).offset(i_0 as isize) as isize,
            );
            i_0 += 1;
            i_0;
        }
        k += (*((*s).sub).offset(0 as c_int as isize)).len;
    }
    if (*((*s).sub).offset(1 as c_int as isize)).flags & AV_TX_INPLACE as c_int as c_ulong == 0 {
        extra_tmp_len = len as c_ulong;
    } else if ps == 0 {
        extra_tmp_len = (*((*s).sub).offset(0 as c_int as isize)).len as c_ulong;
    }
    if extra_tmp_len != 0 && {
        (*s).exp = AVTXNum {
            int32: av_malloc(extra_tmp_len.wrapping_mul(size_of::<TXComplex>() as c_ulong))
                as *mut TXComplex,
        };
        ((*s).exp).int32.is_null()
    } {
        return -(12 as c_int);
    }
    0 as c_int
}
unsafe extern "C" fn ff_tx_fft_pfa_int32_c(
    s: *mut AVTXContext,
    mut _out: *mut c_void,
    mut _in: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let n: c_int = (*((*s).sub).offset(0 as c_int as isize)).len;
    let m: c_int = (*((*s).sub).offset(1 as c_int as isize)).len;
    let l: c_int = (*s).len;
    let in_map: *const c_int = (*s).map;
    let out_map: *const c_int = in_map.offset(l as isize);
    let sub_map: *const c_int = (*((*s).sub).offset(1 as c_int as isize)).map;
    let tmp1: *mut TXComplex =
        if (*((*s).sub).offset(1 as c_int as isize)).flags & AV_TX_INPLACE as c_int as c_ulong != 0
        {
            (*s).tmp.int32
        } else {
            (*s).exp.int32
        };
    let in_0: *mut TXComplex = _in as *mut TXComplex;
    let out: *mut TXComplex = _out as *mut TXComplex;
    stride = (stride as c_ulong).wrapping_div(size_of::<TXComplex>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: c_int = 0 as c_int;
    while i < m {
        let mut j: c_int = 0 as c_int;
        while j < n {
            *((*s).exp).int32.offset(j as isize) =
                *in_0.offset(*in_map.offset((i * n + j) as isize) as isize);
            j += 1;
            j;
        }
        ((*s).fn_0[0 as c_int as usize]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0 as c_int as isize),
            &mut *((*s).tmp)
                .int32
                .offset(*sub_map.offset(i as isize) as isize) as *mut TXComplex
                as *mut c_void,
            (*s).exp.int32 as *mut c_void,
            (m as c_ulong).wrapping_mul(size_of::<TXComplex>() as c_ulong) as ptrdiff_t,
        );
        i += 1;
        i;
    }
    let mut i_0: c_int = 0 as c_int;
    while i_0 < n {
        ((*s).fn_0[1 as c_int as usize]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(1 as c_int as isize),
            &mut *tmp1.offset((m * i_0) as isize) as *mut TXComplex as *mut c_void,
            &mut *((*s).tmp).int32.offset((m * i_0) as isize) as *mut TXComplex as *mut c_void,
            size_of::<TXComplex>() as c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: c_int = 0 as c_int;
    while i_1 < l {
        *out.offset((i_1 as c_long * stride) as isize) =
            *tmp1.offset(*out_map.offset(i_1 as isize) as isize);
        i_1 += 1;
        i_1;
    }
}
unsafe extern "C" fn ff_tx_fft_pfa_ns_int32_c(
    s: *mut AVTXContext,
    mut _out: *mut c_void,
    mut _in: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let n: c_int = (*((*s).sub).offset(0 as c_int as isize)).len;
    let m: c_int = (*((*s).sub).offset(1 as c_int as isize)).len;
    let l: c_int = (*s).len;
    let in_map: *const c_int = (*s).map;
    let out_map: *const c_int = in_map.offset(l as isize);
    let sub_map: *const c_int = (*((*s).sub).offset(1 as c_int as isize)).map;
    let tmp1: *mut TXComplex =
        if (*((*s).sub).offset(1 as c_int as isize)).flags & AV_TX_INPLACE as c_int as c_ulong != 0
        {
            (*s).tmp.int32
        } else {
            (*s).exp.int32
        };
    let in_0: *mut TXComplex = _in as *mut TXComplex;
    let out: *mut TXComplex = _out as *mut TXComplex;
    stride = (stride as c_ulong).wrapping_div(size_of::<TXComplex>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: c_int = 0 as c_int;
    while i < m {
        ((*s).fn_0[0 as c_int as usize]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0 as c_int as isize),
            &mut *((*s).tmp)
                .int32
                .offset(*sub_map.offset(i as isize) as isize) as *mut TXComplex
                as *mut c_void,
            &mut *in_0.offset((i * n) as isize) as *mut TXComplex as *mut c_void,
            (m as c_ulong).wrapping_mul(size_of::<TXComplex>() as c_ulong) as ptrdiff_t,
        );
        i += 1;
        i;
    }
    let mut i_0: c_int = 0 as c_int;
    while i_0 < n {
        ((*s).fn_0[1 as c_int as usize]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(1 as c_int as isize),
            &mut *tmp1.offset((m * i_0) as isize) as *mut TXComplex as *mut c_void,
            &mut *((*s).tmp).int32.offset((m * i_0) as isize) as *mut TXComplex as *mut c_void,
            size_of::<TXComplex>() as c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: c_int = 0 as c_int;
    while i_1 < l {
        *out.offset((i_1 as c_long * stride) as isize) =
            *tmp1.offset(*out_map.offset(i_1 as isize) as isize);
        i_1 += 1;
        i_1;
    }
}
static mut ff_tx_fft_pfa_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft_pfa_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft_pfa_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: ((AV_TX_UNALIGNED as c_int | AV_TX_INPLACE as c_int) as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int) as c_ulong,
            factors: [
                7 as c_int,
                5 as c_int,
                3 as c_int,
                2 as c_int,
                -(1 as c_int),
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
            nb_factors: 2 as c_int,
            min_len: 2 as c_int * 3 as c_int,
            max_len: -(1 as c_int),
            init: Some(
                ff_tx_fft_pfa_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
static mut ff_tx_fft_pfa_ns_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"fft_pfa_ns_int32_c".as_ptr(),
            function: Some(
                ff_tx_fft_pfa_ns_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_FFT,
            flags: ((AV_TX_UNALIGNED as c_int | AV_TX_INPLACE as c_int) as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | (1 as c_ulonglong) << 61 as c_int) as c_ulong,
            factors: [
                7 as c_int,
                5 as c_int,
                3 as c_int,
                2 as c_int,
                -(1 as c_int),
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
            nb_factors: 2 as c_int,
            min_len: 2 as c_int * 3 as c_int,
            max_len: -(1 as c_int),
            init: Some(
                ff_tx_fft_pfa_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
#[cold]
unsafe extern "C" fn ff_tx_mdct_naive_init_int32_c(
    s: *mut AVTXContext,
    _cd: *const FFTXCodelet,
    _flags: c_ulong,
    _opts: *mut FFTXCodeletOptions,
    _len: c_int,
    _inv: c_int,
    scale: *const c_void,
) -> c_int {
    (*s).scale_d = *(scale as *mut c_float) as c_double;
    (*s).scale_f = (*s).scale_d as c_float;
    0 as c_int
}
unsafe extern "C" fn ff_tx_mdct_naive_fwd_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let src: *mut TXSample = _src as *mut TXSample;
    let dst: *mut TXSample = _dst as *mut TXSample;
    let scale: c_double = (*s).scale_d;
    let len: c_int = (*s).len;
    let phase: c_double = PI / (4.0f64 * len as c_double);
    stride = (stride as c_ulong).wrapping_div(size_of::<TXSample>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: c_int = 0 as c_int;
    while i < len {
        let mut sum: c_double = 0.0f64;
        let mut j: c_int = 0 as c_int;
        while j < len * 2 as c_int {
            let a: c_int = (2 as c_int * j + 1 as c_int + len) * (2 as c_int * i + 1 as c_int);
            sum +=
                *src.offset(j as isize) as c_double / 2147483648.0f64 * cos(a as c_double * phase);
            j += 1;
            j;
        }
        *dst.offset((i as c_long * stride) as isize) = av_clip64_c(
            llrintf((sum * scale * 2147483648.0f64) as c_float) as c_long,
            (-(2147483647 as c_int) - 1 as c_int) as c_long,
            2147483647 as c_int as c_long,
        ) as TXSample;
        i += 1;
        i;
    }
}
unsafe extern "C" fn ff_tx_mdct_naive_inv_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let src: *mut TXSample = _src as *mut TXSample;
    let dst: *mut TXSample = _dst as *mut TXSample;
    let scale: c_double = (*s).scale_d;
    let len: c_int = (*s).len >> 1 as c_int;
    let len2: c_int = len * 2 as c_int;
    let phase: c_double = PI / (4.0f64 * len2 as c_double);
    stride = (stride as c_ulong).wrapping_div(size_of::<TXSample>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: c_int = 0 as c_int;
    while i < len {
        let mut sum_d: c_double = 0.0f64;
        let mut sum_u: c_double = 0.0f64;
        let i_d: c_double = phase * (4 as c_int * len - 2 as c_int * i - 1 as c_int) as c_double;
        let i_u: c_double = phase * (3 as c_int * len2 + 2 as c_int * i + 1 as c_int) as c_double;
        let mut j: c_int = 0 as c_int;
        while j < len2 {
            let a: c_double = (2 as c_int * j + 1 as c_int) as c_double;
            let a_d: c_double = cos(a * i_d);
            let a_u: c_double = cos(a * i_u);
            let val: c_double =
                *src.offset((j as c_long * stride) as isize) as c_double / 2147483648.0f64;
            sum_d += a_d * val;
            sum_u += a_u * val;
            j += 1;
            j;
        }
        *dst.offset((i + 0 as c_int) as isize) = av_clip64_c(
            llrintf((sum_d * scale * 2147483648.0f64) as c_float) as c_long,
            (-(2147483647 as c_int) - 1 as c_int) as c_long,
            2147483647 as c_int as c_long,
        ) as TXSample;
        *dst.offset((i + len) as isize) = av_clip64_c(
            llrintf((-sum_u * scale * 2147483648.0f64) as c_float) as c_long,
            (-(2147483647 as c_int) - 1 as c_int) as c_long,
            2147483647 as c_int as c_long,
        ) as TXSample;
        i += 1;
        i;
    }
}
static mut ff_tx_mdct_naive_fwd_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"mdct_naive_fwd_int32_c".as_ptr(),
            function: Some(
                ff_tx_mdct_naive_fwd_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_MDCT,
            flags: (AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | (1 as c_ulonglong) << 59 as c_int) as c_ulong,
            factors: [
                2 as c_int,
                -(1 as c_int),
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
            nb_factors: 2 as c_int,
            min_len: 2 as c_int,
            max_len: -(1 as c_int),
            init: Some(
                ff_tx_mdct_naive_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_MIN as c_int,
        }
    }
};
static mut ff_tx_mdct_naive_inv_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"mdct_naive_inv_int32_c".as_ptr(),
            function: Some(
                ff_tx_mdct_naive_inv_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_MDCT,
            flags: (AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | (1 as c_ulonglong) << 60 as c_int) as c_ulong,
            factors: [
                2 as c_int,
                -(1 as c_int),
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
            nb_factors: 2 as c_int,
            min_len: 2 as c_int,
            max_len: -(1 as c_int),
            init: Some(
                ff_tx_mdct_naive_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_MIN as c_int,
        }
    }
};
#[cold]
unsafe extern "C" fn ff_tx_mdct_init_int32_c(
    s: *mut AVTXContext,
    _cd: *const FFTXCodelet,
    mut flags: c_ulong,
    _opts: *mut FFTXCodeletOptions,
    len: c_int,
    inv: c_int,
    scale: *const c_void,
) -> c_int {
    let mut ret: c_int = 0;
    let mut sub_opts: FFTXCodeletOptions = {
        FFTXCodeletOptions {
            map_dir: (if inv == 0 {
                FF_TX_MAP_SCATTER as c_int
            } else {
                FF_TX_MAP_GATHER as c_int
            }) as FFTXMapDirection,
        }
    };
    (*s).scale_d = *(scale as *mut c_float) as c_double;
    (*s).scale_f = (*s).scale_d as c_float;
    flags = (flags as c_ulonglong & !((1 as c_ulonglong) << 63 as c_int)) as c_ulong;
    flags |= AV_TX_INPLACE as c_int as c_ulong;
    flags = (flags as c_ulonglong | (1 as c_ulonglong) << 61 as c_int) as c_ulong;
    ret = ff_tx_init_subtx(
        s,
        AV_TX_INT32_FFT,
        flags,
        &mut sub_opts,
        len >> 1 as c_int,
        inv,
        scale,
    );
    if ret != 0 {
        flags = (flags as c_ulonglong & !((1 as c_ulonglong) << 61 as c_int)) as c_ulong;
        ret = ff_tx_init_subtx(
            s,
            AV_TX_INT32_FFT,
            flags,
            &mut sub_opts,
            len >> 1 as c_int,
            inv,
            scale,
        );
        if ret != 0 {
            return ret;
        }
    }
    (*s).map =
        av_malloc(((len >> 1 as c_int) as c_ulong).wrapping_mul(size_of::<c_int>() as c_ulong))
            as *mut c_int;
    if ((*s).map).is_null() {
        return -(12 as c_int);
    }
    if (*((*s).sub).offset(0 as c_int as isize)).flags as c_ulonglong
        & (1 as c_ulonglong) << 61 as c_int
        != 0
    {
        memcpy(
            (*s).map as *mut c_void,
            (*(*s).sub).map as *const c_void,
            ((len >> 1 as c_int) as c_ulong).wrapping_mul(size_of::<c_int>() as c_ulong),
        );
    } else {
        let mut i: c_int = 0 as c_int;
        while i < len >> 1 as c_int {
            *((*s).map).offset(i as isize) = i;
            i += 1;
            i;
        }
    }
    ret = ff_tx_mdct_gen_exp_int32(
        s,
        if inv != 0 {
            (*s).map
        } else {
            std::ptr::null_mut::<c_int>()
        },
    );
    if ret != 0 {
        return ret;
    }
    if inv != 0 {
        let mut i_0: c_int = 0 as c_int;
        while i_0 < (*s).len >> 1 as c_int {
            *((*s).map).offset(i_0 as isize) <<= 1 as c_int;
            i_0 += 1;
            i_0;
        }
    }
    0 as c_int
}
unsafe extern "C" fn ff_tx_mdct_fwd_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let src: *mut TXSample = _src as *mut TXSample;
    let dst: *mut TXSample = _dst as *mut TXSample;
    let exp: *mut TXComplex = (*s).exp.int32;
    let mut tmp: TXComplex = TXComplex { re: 0, im: 0 };
    let z: *mut TXComplex = _dst as *mut TXComplex;
    let len2: c_int = (*s).len >> 1 as c_int;
    let len4: c_int = (*s).len >> 2 as c_int;
    let len3: c_int = len2 * 3 as c_int;
    let sub_map: *const c_int = (*s).map;
    stride = (stride as c_ulong).wrapping_div(size_of::<TXSample>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: c_int = 0 as c_int;
    while i < len2 {
        let k: c_int = 2 as c_int * i;
        let idx: c_int = *sub_map.offset(i as isize);
        if k < len2 {
            tmp.re = (-*src.offset((len2 + k) as isize) as c_uint)
                .wrapping_add(*src.offset((1 as c_int * len2 - 1 as c_int - k) as isize) as c_uint)
                .wrapping_add(32 as c_int as c_uint) as c_int
                >> 6 as c_int;
            tmp.im = (-*src.offset((len3 + k) as isize) as c_uint)
                .wrapping_add(-*src.offset((1 as c_int * len3 - 1 as c_int - k) as isize) as c_uint)
                .wrapping_add(32 as c_int as c_uint) as c_int
                >> 6 as c_int;
        } else {
            tmp.re = (-*src.offset((len2 + k) as isize) as c_uint)
                .wrapping_add(-*src.offset((5 as c_int * len2 - 1 as c_int - k) as isize) as c_uint)
                .wrapping_add(32 as c_int as c_uint) as c_int
                >> 6 as c_int;
            tmp.im = (*src.offset((-len2 + k) as isize) as c_uint)
                .wrapping_add(-*src.offset((1 as c_int * len3 - 1 as c_int - k) as isize) as c_uint)
                .wrapping_add(32 as c_int as c_uint) as c_int
                >> 6 as c_int;
        }
        let mut accu: c_long = 0;
        accu = (*exp.offset(i as isize)).re as c_long * tmp.re as c_long;
        accu -= (*exp.offset(i as isize)).im as c_long * tmp.im as c_long;
        (*z.offset(idx as isize)).im =
            (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        accu = (*exp.offset(i as isize)).im as c_long * tmp.re as c_long;
        accu += (*exp.offset(i as isize)).re as c_long * tmp.im as c_long;
        (*z.offset(idx as isize)).re =
            (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        i += 1;
        i;
    }
    ((*s).fn_0[0 as c_int as usize]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0 as c_int as isize),
        z as *mut c_void,
        z as *mut c_void,
        size_of::<TXComplex>() as c_ulong as ptrdiff_t,
    );
    let mut i_0: c_int = 0 as c_int;
    while i_0 < len4 {
        let i0: c_int = len4 + i_0;
        let i1: c_int = len4 - i_0 - 1 as c_int;
        let src1: TXComplex = {
            AVComplexInt32 {
                re: (*z.offset(i1 as isize)).re,
                im: (*z.offset(i1 as isize)).im,
            }
        };
        let src0: TXComplex = {
            AVComplexInt32 {
                re: (*z.offset(i0 as isize)).re,
                im: (*z.offset(i0 as isize)).im,
            }
        };
        let mut accu_0: c_long = 0;
        accu_0 = (*exp.offset(i0 as isize)).im as c_long * src0.re as c_long;
        accu_0 -= (*exp.offset(i0 as isize)).re as c_long * src0.im as c_long;
        *dst.offset(((2 as c_int * i1) as c_long * stride + stride) as isize) =
            (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        accu_0 = (*exp.offset(i0 as isize)).re as c_long * src0.re as c_long;
        accu_0 += (*exp.offset(i0 as isize)).im as c_long * src0.im as c_long;
        *dst.offset(((2 as c_int * i0) as c_long * stride) as isize) =
            (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        let mut accu_1: c_long = 0;
        accu_1 = (*exp.offset(i1 as isize)).im as c_long * src1.re as c_long;
        accu_1 -= (*exp.offset(i1 as isize)).re as c_long * src1.im as c_long;
        *dst.offset(((2 as c_int * i0) as c_long * stride + stride) as isize) =
            (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        accu_1 = (*exp.offset(i1 as isize)).re as c_long * src1.re as c_long;
        accu_1 += (*exp.offset(i1 as isize)).im as c_long * src1.im as c_long;
        *dst.offset(((2 as c_int * i1) as c_long * stride) as isize) =
            (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        i_0 += 1;
        i_0;
    }
}
unsafe extern "C" fn ff_tx_mdct_inv_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let z: *mut TXComplex = _dst as *mut TXComplex;
    let mut exp: *mut TXComplex = (*s).exp.int32;
    let src: *const TXSample = _src as *const TXSample;
    let mut in1: *const TXSample = std::ptr::null::<TXSample>();
    let mut in2: *const TXSample = std::ptr::null::<TXSample>();
    let len2: c_int = (*s).len >> 1 as c_int;
    let len4: c_int = (*s).len >> 2 as c_int;
    let sub_map: *const c_int = (*s).map;
    stride = (stride as c_ulong).wrapping_div(size_of::<TXSample>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    in1 = src;
    in2 = src.offset(((len2 * 2 as c_int - 1 as c_int) as c_long * stride) as isize);
    let mut i: c_int = 0 as c_int;
    while i < len2 {
        let k: c_int = *sub_map.offset(i as isize);
        let tmp: TXComplex = {
            AVComplexInt32 {
                re: *in2.offset((-k as c_long * stride) as isize),
                im: *in1.offset((k as c_long * stride) as isize),
            }
        };
        let mut accu: c_long = 0;
        accu = (*exp.offset(i as isize)).re as c_long * tmp.re as c_long;
        accu -= (*exp.offset(i as isize)).im as c_long * tmp.im as c_long;
        (*z.offset(i as isize)).re = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        accu = (*exp.offset(i as isize)).im as c_long * tmp.re as c_long;
        accu += (*exp.offset(i as isize)).re as c_long * tmp.im as c_long;
        (*z.offset(i as isize)).im = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        i += 1;
        i;
    }
    ((*s).fn_0[0 as c_int as usize]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0 as c_int as isize),
        z as *mut c_void,
        z as *mut c_void,
        size_of::<TXComplex>() as c_ulong as ptrdiff_t,
    );
    exp = exp.offset(len2 as isize);
    let mut i_0: c_int = 0 as c_int;
    while i_0 < len4 {
        let i0: c_int = len4 + i_0;
        let i1: c_int = len4 - i_0 - 1 as c_int;
        let src1: TXComplex = {
            AVComplexInt32 {
                re: (*z.offset(i1 as isize)).im,
                im: (*z.offset(i1 as isize)).re,
            }
        };
        let src0: TXComplex = {
            AVComplexInt32 {
                re: (*z.offset(i0 as isize)).im,
                im: (*z.offset(i0 as isize)).re,
            }
        };
        let mut accu_0: c_long = 0;
        accu_0 = (*exp.offset(i1 as isize)).im as c_long * src1.re as c_long;
        accu_0 -= (*exp.offset(i1 as isize)).re as c_long * src1.im as c_long;
        (*z.offset(i1 as isize)).re =
            (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        accu_0 = (*exp.offset(i1 as isize)).re as c_long * src1.re as c_long;
        accu_0 += (*exp.offset(i1 as isize)).im as c_long * src1.im as c_long;
        (*z.offset(i0 as isize)).im =
            (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        let mut accu_1: c_long = 0;
        accu_1 = (*exp.offset(i0 as isize)).im as c_long * src0.re as c_long;
        accu_1 -= (*exp.offset(i0 as isize)).re as c_long * src0.im as c_long;
        (*z.offset(i0 as isize)).re =
            (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        accu_1 = (*exp.offset(i0 as isize)).re as c_long * src0.re as c_long;
        accu_1 += (*exp.offset(i0 as isize)).im as c_long * src0.im as c_long;
        (*z.offset(i1 as isize)).im =
            (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        i_0 += 1;
        i_0;
    }
}
static mut ff_tx_mdct_fwd_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"mdct_fwd_int32_c".as_ptr(),
            function: Some(
                ff_tx_mdct_fwd_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_MDCT,
            flags: (AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | (1 as c_ulonglong) << 59 as c_int) as c_ulong,
            factors: [
                2 as c_int,
                -(1 as c_int),
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
            nb_factors: 2 as c_int,
            min_len: 2 as c_int,
            max_len: -(1 as c_int),
            init: Some(
                ff_tx_mdct_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
static mut ff_tx_mdct_inv_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"mdct_inv_int32_c".as_ptr(),
            function: Some(
                ff_tx_mdct_inv_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_MDCT,
            flags: (AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | (1 as c_ulonglong) << 60 as c_int) as c_ulong,
            factors: [
                2 as c_int,
                -(1 as c_int),
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
            nb_factors: 2 as c_int,
            min_len: 2 as c_int,
            max_len: -(1 as c_int),
            init: Some(
                ff_tx_mdct_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
#[cold]
unsafe extern "C" fn ff_tx_mdct_inv_full_init_int32_c(
    s: *mut AVTXContext,
    _cd: *const FFTXCodelet,
    mut flags: c_ulong,
    _opts: *mut FFTXCodeletOptions,
    len: c_int,
    _inv: c_int,
    scale: *const c_void,
) -> c_int {
    let mut ret: c_int = 0;
    (*s).scale_d = *(scale as *mut c_float) as c_double;
    (*s).scale_f = (*s).scale_d as c_float;
    flags &= !(AV_TX_FULL_IMDCT as c_int) as c_ulong;
    ret = ff_tx_init_subtx(
        s,
        AV_TX_INT32_MDCT,
        flags,
        std::ptr::null_mut::<FFTXCodeletOptions>(),
        len,
        1 as c_int,
        scale,
    );
    if ret != 0 {
        return ret;
    }
    0 as c_int
}
unsafe extern "C" fn ff_tx_mdct_inv_full_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let len: c_int = (*s).len << 1 as c_int;
    let len2: c_int = len >> 1 as c_int;
    let len4: c_int = len >> 2 as c_int;
    let dst: *mut TXSample = _dst as *mut TXSample;
    ((*s).fn_0[0 as c_int as usize]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0 as c_int as isize),
        dst.offset(len4 as isize) as *mut c_void,
        _src,
        stride,
    );
    stride = (stride as c_ulong).wrapping_div(size_of::<TXSample>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: c_int = 0 as c_int;
    while i < len4 {
        *dst.offset((i as c_long * stride) as isize) =
            -*dst.offset(((len2 - i - 1 as c_int) as c_long * stride) as isize);
        *dst.offset(((len - i - 1 as c_int) as c_long * stride) as isize) =
            *dst.offset(((len2 + i + 0 as c_int) as c_long * stride) as isize);
        i += 1;
        i;
    }
}
static mut ff_tx_mdct_inv_full_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"mdct_inv_full_int32_c".as_ptr(),
            function: Some(
                ff_tx_mdct_inv_full_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_MDCT,
            flags: ((AV_TX_UNALIGNED as c_int | AV_TX_INPLACE as c_int) as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | AV_TX_FULL_IMDCT as c_int as c_ulonglong) as c_ulong,
            factors: [
                2 as c_int,
                -(1 as c_int),
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
            nb_factors: 2 as c_int,
            min_len: 2 as c_int,
            max_len: -(1 as c_int),
            init: Some(
                ff_tx_mdct_inv_full_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
#[cold]
unsafe extern "C" fn ff_tx_mdct_pfa_init_int32_c(
    s: *mut AVTXContext,
    cd: *const FFTXCodelet,
    mut flags: c_ulong,
    opts: *mut FFTXCodeletOptions,
    mut len: c_int,
    inv: c_int,
    scale: *const c_void,
) -> c_int {
    let mut ret: c_int = 0;
    let mut sub_len: c_int = 0;
    let mut sub_opts: FFTXCodeletOptions = {
        FFTXCodeletOptions {
            map_dir: FF_TX_MAP_SCATTER,
        }
    };
    len >>= 1 as c_int;
    sub_len = len / (*cd).factors[0 as c_int as usize];
    (*s).scale_d = *(scale as *mut c_float) as c_double;
    (*s).scale_f = (*s).scale_d as c_float;
    flags = (flags as c_ulonglong & !((1 as c_ulonglong) << 63 as c_int)) as c_ulong;
    flags |= AV_TX_INPLACE as c_int as c_ulong;
    flags = (flags as c_ulonglong | (1 as c_ulonglong) << 61 as c_int) as c_ulong;
    ret = ff_tx_init_subtx(
        s,
        AV_TX_INT32_FFT,
        flags,
        &mut sub_opts,
        sub_len,
        inv,
        scale,
    );
    if ret != 0 {
        return ret;
    }
    ret = ff_tx_gen_compound_mapping(
        s,
        opts,
        (*s).inv,
        (*cd).factors[0 as c_int as usize],
        sub_len,
    );
    if ret != 0 {
        return ret;
    }
    if (*cd).factors[0 as c_int as usize] == 15 as c_int {
        let mut mtmp: [c_int; 15] = [0; 15];
        let mut k: c_int = 0 as c_int;
        while k < len {
            memcpy(
                mtmp.as_mut_ptr() as *mut c_void,
                &mut *((*s).map).offset(k as isize) as *mut c_int as *const c_void,
                ((3 as c_int * 5 as c_int) as c_ulong).wrapping_mul(size_of::<c_int>() as c_ulong),
            );
            let mut m: c_int = 0 as c_int;
            while m < 5 as c_int {
                let mut n: c_int = 0 as c_int;
                while n < 3 as c_int {
                    *((*s).map).offset((k + m * 3 as c_int + n) as isize) = mtmp
                        [((m * 3 as c_int + n * 5 as c_int) % (3 as c_int * 5 as c_int)) as usize];
                    n += 1;
                    n;
                }
                m += 1;
                m;
            }
            k += 3 as c_int * 5 as c_int;
        }
    }
    ret = ff_tx_mdct_gen_exp_int32(
        s,
        if inv != 0 {
            (*s).map
        } else {
            std::ptr::null_mut::<c_int>()
        },
    );
    if ret != 0 {
        return ret;
    }
    let mut i: c_int = 0 as c_int;
    while i < len {
        *((*s).map).offset(i as isize) <<= 1 as c_int;
        i += 1;
        i;
    }
    (*s).tmp = AVTXNum {
        int32: av_malloc((len as c_ulong).wrapping_mul(size_of::<TXComplex>() as c_ulong))
            as *mut TXComplex,
    };
    if ((*s).tmp).int32.is_null() {
        return -(12 as c_int);
    }
    ff_tx_init_tabs_int32(len / sub_len);
    0 as c_int
}
unsafe extern "C" fn ff_tx_mdct_pfa_3xM_inv_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let mut fft3in: [TXComplex; 3] = [TXComplex { re: 0, im: 0 }; 3];
    let z: *mut TXComplex = _dst as *mut TXComplex;
    let mut exp: *mut TXComplex = (*s).exp.int32;
    let src: *const TXSample = _src as *const TXSample;
    let mut in1: *const TXSample = std::ptr::null::<TXSample>();
    let mut in2: *const TXSample = std::ptr::null::<TXSample>();
    let len4: c_int = (*s).len >> 2 as c_int;
    let len2: c_int = (*s).len >> 1 as c_int;
    let m: c_int = (*(*s).sub).len;
    let mut in_map: *const c_int = (*s).map;
    let out_map: *const c_int = in_map.offset((3 as c_int * m) as isize);
    let mut sub_map: *const c_int = (*(*s).sub).map;
    stride = (stride as c_ulong).wrapping_div(size_of::<TXSample>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    in1 = src;
    in2 = src.offset(((3 as c_int * m * 2 as c_int - 1 as c_int) as c_long * stride) as isize);
    let mut i: c_int = 0 as c_int;
    while i < len2 {
        let mut j: c_int = 0 as c_int;
        while j < 3 as c_int {
            let k: c_int = *in_map.offset(j as isize);
            let tmp: TXComplex = {
                AVComplexInt32 {
                    re: *in2.offset((-k as c_long * stride) as isize),
                    im: *in1.offset((k as c_long * stride) as isize),
                }
            };
            let mut accu: c_long = 0;
            accu = (*exp.offset(j as isize)).re as c_long * tmp.re as c_long;
            accu -= (*exp.offset(j as isize)).im as c_long * tmp.im as c_long;
            fft3in[j as usize].re = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
            accu = (*exp.offset(j as isize)).im as c_long * tmp.re as c_long;
            accu += (*exp.offset(j as isize)).re as c_long * tmp.im as c_long;
            fft3in[j as usize].im = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
            j += 1;
            j;
        }
        let fresh22 = sub_map;
        sub_map = sub_map.offset(1);
        fft3(
            ((*s).tmp).int32.offset(*fresh22 as isize),
            fft3in.as_mut_ptr(),
            m as ptrdiff_t,
        );
        exp = exp.offset(3 as c_int as isize);
        in_map = in_map.offset(3 as c_int as isize);
        i += 3 as c_int;
    }
    let mut i_0: c_int = 0 as c_int;
    while i_0 < 3 as c_int {
        ((*s).fn_0[0 as c_int as usize]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0 as c_int as isize),
            ((*s).tmp).int32.offset((m * i_0) as isize) as *mut c_void,
            ((*s).tmp).int32.offset((m * i_0) as isize) as *mut c_void,
            size_of::<TXComplex>() as c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: c_int = 0 as c_int;
    while i_1 < len4 {
        let i0: c_int = len4 + i_1;
        let i1: c_int = len4 - i_1 - 1 as c_int;
        let s0: c_int = *out_map.offset(i0 as isize);
        let s1: c_int = *out_map.offset(i1 as isize);
        let src1: TXComplex = {
            AVComplexInt32 {
                re: (*((*s).tmp).int32.offset(s1 as isize)).im,
                im: (*((*s).tmp).int32.offset(s1 as isize)).re,
            }
        };
        let src0: TXComplex = {
            AVComplexInt32 {
                re: (*((*s).tmp).int32.offset(s0 as isize)).im,
                im: (*((*s).tmp).int32.offset(s0 as isize)).re,
            }
        };
        let mut accu_0: c_long = 0;
        accu_0 = (*exp.offset(i1 as isize)).im as c_long * src1.re as c_long;
        accu_0 -= (*exp.offset(i1 as isize)).re as c_long * src1.im as c_long;
        (*z.offset(i1 as isize)).re =
            (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        accu_0 = (*exp.offset(i1 as isize)).re as c_long * src1.re as c_long;
        accu_0 += (*exp.offset(i1 as isize)).im as c_long * src1.im as c_long;
        (*z.offset(i0 as isize)).im =
            (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        let mut accu_1: c_long = 0;
        accu_1 = (*exp.offset(i0 as isize)).im as c_long * src0.re as c_long;
        accu_1 -= (*exp.offset(i0 as isize)).re as c_long * src0.im as c_long;
        (*z.offset(i0 as isize)).re =
            (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        accu_1 = (*exp.offset(i0 as isize)).re as c_long * src0.re as c_long;
        accu_1 += (*exp.offset(i0 as isize)).im as c_long * src0.im as c_long;
        (*z.offset(i1 as isize)).im =
            (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        i_1 += 1;
        i_1;
    }
}
static mut ff_tx_mdct_pfa_3xM_inv_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"mdct_pfa_3xM_inv_int32_c".as_ptr(),
            function: Some(
                ff_tx_mdct_pfa_3xM_inv_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_MDCT,
            flags: (AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | (1 as c_ulonglong) << 60 as c_int) as c_ulong,
            factors: [
                3 as c_int,
                -(1 as c_int),
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
            nb_factors: 2 as c_int,
            min_len: 3 as c_int * 2 as c_int,
            max_len: -(1 as c_int),
            init: Some(
                ff_tx_mdct_pfa_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
unsafe extern "C" fn ff_tx_mdct_pfa_5xM_inv_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let mut fft5in: [TXComplex; 5] = [TXComplex { re: 0, im: 0 }; 5];
    let z: *mut TXComplex = _dst as *mut TXComplex;
    let mut exp: *mut TXComplex = (*s).exp.int32;
    let src: *const TXSample = _src as *const TXSample;
    let mut in1: *const TXSample = std::ptr::null::<TXSample>();
    let mut in2: *const TXSample = std::ptr::null::<TXSample>();
    let len4: c_int = (*s).len >> 2 as c_int;
    let len2: c_int = (*s).len >> 1 as c_int;
    let m: c_int = (*(*s).sub).len;
    let mut in_map: *const c_int = (*s).map;
    let out_map: *const c_int = in_map.offset((5 as c_int * m) as isize);
    let mut sub_map: *const c_int = (*(*s).sub).map;
    stride = (stride as c_ulong).wrapping_div(size_of::<TXSample>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    in1 = src;
    in2 = src.offset(((5 as c_int * m * 2 as c_int - 1 as c_int) as c_long * stride) as isize);
    let mut i: c_int = 0 as c_int;
    while i < len2 {
        let mut j: c_int = 0 as c_int;
        while j < 5 as c_int {
            let k: c_int = *in_map.offset(j as isize);
            let tmp: TXComplex = {
                AVComplexInt32 {
                    re: *in2.offset((-k as c_long * stride) as isize),
                    im: *in1.offset((k as c_long * stride) as isize),
                }
            };
            let mut accu: c_long = 0;
            accu = (*exp.offset(j as isize)).re as c_long * tmp.re as c_long;
            accu -= (*exp.offset(j as isize)).im as c_long * tmp.im as c_long;
            fft5in[j as usize].re = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
            accu = (*exp.offset(j as isize)).im as c_long * tmp.re as c_long;
            accu += (*exp.offset(j as isize)).re as c_long * tmp.im as c_long;
            fft5in[j as usize].im = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
            j += 1;
            j;
        }
        let fresh23 = sub_map;
        sub_map = sub_map.offset(1);
        fft5(
            ((*s).tmp).int32.offset(*fresh23 as isize),
            fft5in.as_mut_ptr(),
            m as ptrdiff_t,
        );
        exp = exp.offset(5 as c_int as isize);
        in_map = in_map.offset(5 as c_int as isize);
        i += 5 as c_int;
    }
    let mut i_0: c_int = 0 as c_int;
    while i_0 < 5 as c_int {
        ((*s).fn_0[0 as c_int as usize]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0 as c_int as isize),
            ((*s).tmp).int32.offset((m * i_0) as isize) as *mut c_void,
            ((*s).tmp).int32.offset((m * i_0) as isize) as *mut c_void,
            size_of::<TXComplex>() as c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: c_int = 0 as c_int;
    while i_1 < len4 {
        let i0: c_int = len4 + i_1;
        let i1: c_int = len4 - i_1 - 1 as c_int;
        let s0: c_int = *out_map.offset(i0 as isize);
        let s1: c_int = *out_map.offset(i1 as isize);
        let src1: TXComplex = {
            AVComplexInt32 {
                re: (*((*s).tmp).int32.offset(s1 as isize)).im,
                im: (*((*s).tmp).int32.offset(s1 as isize)).re,
            }
        };
        let src0: TXComplex = {
            AVComplexInt32 {
                re: (*((*s).tmp).int32.offset(s0 as isize)).im,
                im: (*((*s).tmp).int32.offset(s0 as isize)).re,
            }
        };
        let mut accu_0: c_long = 0;
        accu_0 = (*exp.offset(i1 as isize)).im as c_long * src1.re as c_long;
        accu_0 -= (*exp.offset(i1 as isize)).re as c_long * src1.im as c_long;
        (*z.offset(i1 as isize)).re =
            (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        accu_0 = (*exp.offset(i1 as isize)).re as c_long * src1.re as c_long;
        accu_0 += (*exp.offset(i1 as isize)).im as c_long * src1.im as c_long;
        (*z.offset(i0 as isize)).im =
            (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        let mut accu_1: c_long = 0;
        accu_1 = (*exp.offset(i0 as isize)).im as c_long * src0.re as c_long;
        accu_1 -= (*exp.offset(i0 as isize)).re as c_long * src0.im as c_long;
        (*z.offset(i0 as isize)).re =
            (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        accu_1 = (*exp.offset(i0 as isize)).re as c_long * src0.re as c_long;
        accu_1 += (*exp.offset(i0 as isize)).im as c_long * src0.im as c_long;
        (*z.offset(i1 as isize)).im =
            (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        i_1 += 1;
        i_1;
    }
}
static mut ff_tx_mdct_pfa_5xM_inv_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"mdct_pfa_5xM_inv_int32_c".as_ptr(),
            function: Some(
                ff_tx_mdct_pfa_5xM_inv_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_MDCT,
            flags: (AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | (1 as c_ulonglong) << 60 as c_int) as c_ulong,
            factors: [
                5 as c_int,
                -(1 as c_int),
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
            nb_factors: 2 as c_int,
            min_len: 5 as c_int * 2 as c_int,
            max_len: -(1 as c_int),
            init: Some(
                ff_tx_mdct_pfa_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
static mut ff_tx_mdct_pfa_7xM_inv_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"mdct_pfa_7xM_inv_int32_c".as_ptr(),
            function: Some(
                ff_tx_mdct_pfa_7xM_inv_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_MDCT,
            flags: (AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | (1 as c_ulonglong) << 60 as c_int) as c_ulong,
            factors: [
                7 as c_int,
                -(1 as c_int),
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
            nb_factors: 2 as c_int,
            min_len: 7 as c_int * 2 as c_int,
            max_len: -(1 as c_int),
            init: Some(
                ff_tx_mdct_pfa_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
unsafe extern "C" fn ff_tx_mdct_pfa_7xM_inv_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let mut fft7in: [TXComplex; 7] = [TXComplex { re: 0, im: 0 }; 7];
    let z: *mut TXComplex = _dst as *mut TXComplex;
    let mut exp: *mut TXComplex = (*s).exp.int32;
    let src: *const TXSample = _src as *const TXSample;
    let mut in1: *const TXSample = std::ptr::null::<TXSample>();
    let mut in2: *const TXSample = std::ptr::null::<TXSample>();
    let len4: c_int = (*s).len >> 2 as c_int;
    let len2: c_int = (*s).len >> 1 as c_int;
    let m: c_int = (*(*s).sub).len;
    let mut in_map: *const c_int = (*s).map;
    let out_map: *const c_int = in_map.offset((7 as c_int * m) as isize);
    let mut sub_map: *const c_int = (*(*s).sub).map;
    stride = (stride as c_ulong).wrapping_div(size_of::<TXSample>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    in1 = src;
    in2 = src.offset(((7 as c_int * m * 2 as c_int - 1 as c_int) as c_long * stride) as isize);
    let mut i: c_int = 0 as c_int;
    while i < len2 {
        let mut j: c_int = 0 as c_int;
        while j < 7 as c_int {
            let k: c_int = *in_map.offset(j as isize);
            let tmp: TXComplex = {
                AVComplexInt32 {
                    re: *in2.offset((-k as c_long * stride) as isize),
                    im: *in1.offset((k as c_long * stride) as isize),
                }
            };
            let mut accu: c_long = 0;
            accu = (*exp.offset(j as isize)).re as c_long * tmp.re as c_long;
            accu -= (*exp.offset(j as isize)).im as c_long * tmp.im as c_long;
            fft7in[j as usize].re = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
            accu = (*exp.offset(j as isize)).im as c_long * tmp.re as c_long;
            accu += (*exp.offset(j as isize)).re as c_long * tmp.im as c_long;
            fft7in[j as usize].im = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
            j += 1;
            j;
        }
        let fresh24 = sub_map;
        sub_map = sub_map.offset(1);
        fft7(
            ((*s).tmp).int32.offset(*fresh24 as isize),
            fft7in.as_mut_ptr(),
            m as ptrdiff_t,
        );
        exp = exp.offset(7 as c_int as isize);
        in_map = in_map.offset(7 as c_int as isize);
        i += 7 as c_int;
    }
    let mut i_0: c_int = 0 as c_int;
    while i_0 < 7 as c_int {
        ((*s).fn_0[0 as c_int as usize]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0 as c_int as isize),
            ((*s).tmp).int32.offset((m * i_0) as isize) as *mut c_void,
            ((*s).tmp).int32.offset((m * i_0) as isize) as *mut c_void,
            size_of::<TXComplex>() as c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: c_int = 0 as c_int;
    while i_1 < len4 {
        let i0: c_int = len4 + i_1;
        let i1: c_int = len4 - i_1 - 1 as c_int;
        let s0: c_int = *out_map.offset(i0 as isize);
        let s1: c_int = *out_map.offset(i1 as isize);
        let src1: TXComplex = {
            AVComplexInt32 {
                re: (*((*s).tmp).int32.offset(s1 as isize)).im,
                im: (*((*s).tmp).int32.offset(s1 as isize)).re,
            }
        };
        let src0: TXComplex = {
            AVComplexInt32 {
                re: (*((*s).tmp).int32.offset(s0 as isize)).im,
                im: (*((*s).tmp).int32.offset(s0 as isize)).re,
            }
        };
        let mut accu_0: c_long = 0;
        accu_0 = (*exp.offset(i1 as isize)).im as c_long * src1.re as c_long;
        accu_0 -= (*exp.offset(i1 as isize)).re as c_long * src1.im as c_long;
        (*z.offset(i1 as isize)).re =
            (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        accu_0 = (*exp.offset(i1 as isize)).re as c_long * src1.re as c_long;
        accu_0 += (*exp.offset(i1 as isize)).im as c_long * src1.im as c_long;
        (*z.offset(i0 as isize)).im =
            (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        let mut accu_1: c_long = 0;
        accu_1 = (*exp.offset(i0 as isize)).im as c_long * src0.re as c_long;
        accu_1 -= (*exp.offset(i0 as isize)).re as c_long * src0.im as c_long;
        (*z.offset(i0 as isize)).re =
            (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        accu_1 = (*exp.offset(i0 as isize)).re as c_long * src0.re as c_long;
        accu_1 += (*exp.offset(i0 as isize)).im as c_long * src0.im as c_long;
        (*z.offset(i1 as isize)).im =
            (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        i_1 += 1;
        i_1;
    }
}
static mut ff_tx_mdct_pfa_9xM_inv_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"mdct_pfa_9xM_inv_int32_c".as_ptr(),
            function: Some(
                ff_tx_mdct_pfa_9xM_inv_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_MDCT,
            flags: (AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | (1 as c_ulonglong) << 60 as c_int) as c_ulong,
            factors: [
                9 as c_int,
                -(1 as c_int),
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
            nb_factors: 2 as c_int,
            min_len: 9 as c_int * 2 as c_int,
            max_len: -(1 as c_int),
            init: Some(
                ff_tx_mdct_pfa_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
unsafe extern "C" fn ff_tx_mdct_pfa_9xM_inv_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let mut fft9in: [TXComplex; 9] = [TXComplex { re: 0, im: 0 }; 9];
    let z: *mut TXComplex = _dst as *mut TXComplex;
    let mut exp: *mut TXComplex = (*s).exp.int32;
    let src: *const TXSample = _src as *const TXSample;
    let mut in1: *const TXSample = std::ptr::null::<TXSample>();
    let mut in2: *const TXSample = std::ptr::null::<TXSample>();
    let len4: c_int = (*s).len >> 2 as c_int;
    let len2: c_int = (*s).len >> 1 as c_int;
    let m: c_int = (*(*s).sub).len;
    let mut in_map: *const c_int = (*s).map;
    let out_map: *const c_int = in_map.offset((9 as c_int * m) as isize);
    let mut sub_map: *const c_int = (*(*s).sub).map;
    stride = (stride as c_ulong).wrapping_div(size_of::<TXSample>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    in1 = src;
    in2 = src.offset(((9 as c_int * m * 2 as c_int - 1 as c_int) as c_long * stride) as isize);
    let mut i: c_int = 0 as c_int;
    while i < len2 {
        let mut j: c_int = 0 as c_int;
        while j < 9 as c_int {
            let k: c_int = *in_map.offset(j as isize);
            let tmp: TXComplex = {
                AVComplexInt32 {
                    re: *in2.offset((-k as c_long * stride) as isize),
                    im: *in1.offset((k as c_long * stride) as isize),
                }
            };
            let mut accu: c_long = 0;
            accu = (*exp.offset(j as isize)).re as c_long * tmp.re as c_long;
            accu -= (*exp.offset(j as isize)).im as c_long * tmp.im as c_long;
            fft9in[j as usize].re = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
            accu = (*exp.offset(j as isize)).im as c_long * tmp.re as c_long;
            accu += (*exp.offset(j as isize)).re as c_long * tmp.im as c_long;
            fft9in[j as usize].im = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
            j += 1;
            j;
        }
        let fresh25 = sub_map;
        sub_map = sub_map.offset(1);
        fft9(
            ((*s).tmp).int32.offset(*fresh25 as isize),
            fft9in.as_mut_ptr(),
            m as ptrdiff_t,
        );
        exp = exp.offset(9 as c_int as isize);
        in_map = in_map.offset(9 as c_int as isize);
        i += 9 as c_int;
    }
    let mut i_0: c_int = 0 as c_int;
    while i_0 < 9 as c_int {
        ((*s).fn_0[0 as c_int as usize]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0 as c_int as isize),
            ((*s).tmp).int32.offset((m * i_0) as isize) as *mut c_void,
            ((*s).tmp).int32.offset((m * i_0) as isize) as *mut c_void,
            size_of::<TXComplex>() as c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: c_int = 0 as c_int;
    while i_1 < len4 {
        let i0: c_int = len4 + i_1;
        let i1: c_int = len4 - i_1 - 1 as c_int;
        let s0: c_int = *out_map.offset(i0 as isize);
        let s1: c_int = *out_map.offset(i1 as isize);
        let src1: TXComplex = {
            AVComplexInt32 {
                re: (*((*s).tmp).int32.offset(s1 as isize)).im,
                im: (*((*s).tmp).int32.offset(s1 as isize)).re,
            }
        };
        let src0: TXComplex = {
            AVComplexInt32 {
                re: (*((*s).tmp).int32.offset(s0 as isize)).im,
                im: (*((*s).tmp).int32.offset(s0 as isize)).re,
            }
        };
        let mut accu_0: c_long = 0;
        accu_0 = (*exp.offset(i1 as isize)).im as c_long * src1.re as c_long;
        accu_0 -= (*exp.offset(i1 as isize)).re as c_long * src1.im as c_long;
        (*z.offset(i1 as isize)).re =
            (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        accu_0 = (*exp.offset(i1 as isize)).re as c_long * src1.re as c_long;
        accu_0 += (*exp.offset(i1 as isize)).im as c_long * src1.im as c_long;
        (*z.offset(i0 as isize)).im =
            (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        let mut accu_1: c_long = 0;
        accu_1 = (*exp.offset(i0 as isize)).im as c_long * src0.re as c_long;
        accu_1 -= (*exp.offset(i0 as isize)).re as c_long * src0.im as c_long;
        (*z.offset(i0 as isize)).re =
            (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        accu_1 = (*exp.offset(i0 as isize)).re as c_long * src0.re as c_long;
        accu_1 += (*exp.offset(i0 as isize)).im as c_long * src0.im as c_long;
        (*z.offset(i1 as isize)).im =
            (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        i_1 += 1;
        i_1;
    }
}
unsafe extern "C" fn ff_tx_mdct_pfa_15xM_inv_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let mut fft15in: [TXComplex; 15] = [TXComplex { re: 0, im: 0 }; 15];
    let z: *mut TXComplex = _dst as *mut TXComplex;
    let mut exp: *mut TXComplex = (*s).exp.int32;
    let src: *const TXSample = _src as *const TXSample;
    let mut in1: *const TXSample = std::ptr::null::<TXSample>();
    let mut in2: *const TXSample = std::ptr::null::<TXSample>();
    let len4: c_int = (*s).len >> 2 as c_int;
    let len2: c_int = (*s).len >> 1 as c_int;
    let m: c_int = (*(*s).sub).len;
    let mut in_map: *const c_int = (*s).map;
    let out_map: *const c_int = in_map.offset((15 as c_int * m) as isize);
    let mut sub_map: *const c_int = (*(*s).sub).map;
    stride = (stride as c_ulong).wrapping_div(size_of::<TXSample>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    in1 = src;
    in2 = src.offset(((15 as c_int * m * 2 as c_int - 1 as c_int) as c_long * stride) as isize);
    let mut i: c_int = 0 as c_int;
    while i < len2 {
        let mut j: c_int = 0 as c_int;
        while j < 15 as c_int {
            let k: c_int = *in_map.offset(j as isize);
            let tmp: TXComplex = {
                AVComplexInt32 {
                    re: *in2.offset((-k as c_long * stride) as isize),
                    im: *in1.offset((k as c_long * stride) as isize),
                }
            };
            let mut accu: c_long = 0;
            accu = (*exp.offset(j as isize)).re as c_long * tmp.re as c_long;
            accu -= (*exp.offset(j as isize)).im as c_long * tmp.im as c_long;
            fft15in[j as usize].re = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
            accu = (*exp.offset(j as isize)).im as c_long * tmp.re as c_long;
            accu += (*exp.offset(j as isize)).re as c_long * tmp.im as c_long;
            fft15in[j as usize].im = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
            j += 1;
            j;
        }
        let fresh26 = sub_map;
        sub_map = sub_map.offset(1);
        fft15(
            ((*s).tmp).int32.offset(*fresh26 as isize),
            fft15in.as_mut_ptr(),
            m as ptrdiff_t,
        );
        exp = exp.offset(15 as c_int as isize);
        in_map = in_map.offset(15 as c_int as isize);
        i += 15 as c_int;
    }
    let mut i_0: c_int = 0 as c_int;
    while i_0 < 15 as c_int {
        ((*s).fn_0[0 as c_int as usize]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0 as c_int as isize),
            ((*s).tmp).int32.offset((m * i_0) as isize) as *mut c_void,
            ((*s).tmp).int32.offset((m * i_0) as isize) as *mut c_void,
            size_of::<TXComplex>() as c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: c_int = 0 as c_int;
    while i_1 < len4 {
        let i0: c_int = len4 + i_1;
        let i1: c_int = len4 - i_1 - 1 as c_int;
        let s0: c_int = *out_map.offset(i0 as isize);
        let s1: c_int = *out_map.offset(i1 as isize);
        let src1: TXComplex = {
            AVComplexInt32 {
                re: (*((*s).tmp).int32.offset(s1 as isize)).im,
                im: (*((*s).tmp).int32.offset(s1 as isize)).re,
            }
        };
        let src0: TXComplex = {
            AVComplexInt32 {
                re: (*((*s).tmp).int32.offset(s0 as isize)).im,
                im: (*((*s).tmp).int32.offset(s0 as isize)).re,
            }
        };
        let mut accu_0: c_long = 0;
        accu_0 = (*exp.offset(i1 as isize)).im as c_long * src1.re as c_long;
        accu_0 -= (*exp.offset(i1 as isize)).re as c_long * src1.im as c_long;
        (*z.offset(i1 as isize)).re =
            (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        accu_0 = (*exp.offset(i1 as isize)).re as c_long * src1.re as c_long;
        accu_0 += (*exp.offset(i1 as isize)).im as c_long * src1.im as c_long;
        (*z.offset(i0 as isize)).im =
            (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        let mut accu_1: c_long = 0;
        accu_1 = (*exp.offset(i0 as isize)).im as c_long * src0.re as c_long;
        accu_1 -= (*exp.offset(i0 as isize)).re as c_long * src0.im as c_long;
        (*z.offset(i0 as isize)).re =
            (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        accu_1 = (*exp.offset(i0 as isize)).re as c_long * src0.re as c_long;
        accu_1 += (*exp.offset(i0 as isize)).im as c_long * src0.im as c_long;
        (*z.offset(i1 as isize)).im =
            (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        i_1 += 1;
        i_1;
    }
}
static mut ff_tx_mdct_pfa_15xM_inv_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"mdct_pfa_15xM_inv_int32_c".as_ptr(),
            function: Some(
                ff_tx_mdct_pfa_15xM_inv_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_MDCT,
            flags: (AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | (1 as c_ulonglong) << 60 as c_int) as c_ulong,
            factors: [
                15 as c_int,
                -(1 as c_int),
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
            nb_factors: 2 as c_int,
            min_len: 15 as c_int * 2 as c_int,
            max_len: -(1 as c_int),
            init: Some(
                ff_tx_mdct_pfa_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
unsafe extern "C" fn ff_tx_mdct_pfa_3xM_fwd_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let mut fft3in: [TXComplex; 3] = [TXComplex { re: 0, im: 0 }; 3];
    let src: *mut TXSample = _src as *mut TXSample;
    let dst: *mut TXSample = _dst as *mut TXSample;
    let exp: *mut TXComplex = (*s).exp.int32;
    let mut tmp: TXComplex = TXComplex { re: 0, im: 0 };
    let m: c_int = (*(*s).sub).len;
    let len4: c_int = 3 as c_int * m;
    let len3: c_int = len4 * 3 as c_int;
    let len8: c_int = (*s).len >> 2 as c_int;
    let in_map: *const c_int = (*s).map;
    let out_map: *const c_int = in_map.offset((3 as c_int * m) as isize);
    let sub_map: *const c_int = (*(*s).sub).map;
    stride = (stride as c_ulong).wrapping_div(size_of::<TXSample>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: c_int = 0 as c_int;
    while i < m {
        let mut j: c_int = 0 as c_int;
        while j < 3 as c_int {
            let k: c_int = *in_map.offset((i * 3 as c_int + j) as isize);
            if k < len4 {
                tmp.re = (-*src.offset((len4 + k) as isize) as c_uint)
                    .wrapping_add(
                        *src.offset((1 as c_int * len4 - 1 as c_int - k) as isize) as c_uint
                    )
                    .wrapping_add(32 as c_int as c_uint) as c_int
                    >> 6 as c_int;
                tmp.im = (-*src.offset((len3 + k) as isize) as c_uint)
                    .wrapping_add(
                        -*src.offset((1 as c_int * len3 - 1 as c_int - k) as isize) as c_uint
                    )
                    .wrapping_add(32 as c_int as c_uint) as c_int
                    >> 6 as c_int;
            } else {
                tmp.re = (-*src.offset((len4 + k) as isize) as c_uint)
                    .wrapping_add(
                        -*src.offset((5 as c_int * len4 - 1 as c_int - k) as isize) as c_uint
                    )
                    .wrapping_add(32 as c_int as c_uint) as c_int
                    >> 6 as c_int;
                tmp.im = (*src.offset((-len4 + k) as isize) as c_uint)
                    .wrapping_add(
                        -*src.offset((1 as c_int * len3 - 1 as c_int - k) as isize) as c_uint
                    )
                    .wrapping_add(32 as c_int as c_uint) as c_int
                    >> 6 as c_int;
            }
            let mut accu: c_long = 0;
            accu = (*exp.offset((k >> 1 as c_int) as isize)).re as c_long * tmp.re as c_long;
            accu -= (*exp.offset((k >> 1 as c_int) as isize)).im as c_long * tmp.im as c_long;
            fft3in[j as usize].im = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
            accu = (*exp.offset((k >> 1 as c_int) as isize)).im as c_long * tmp.re as c_long;
            accu += (*exp.offset((k >> 1 as c_int) as isize)).re as c_long * tmp.im as c_long;
            fft3in[j as usize].re = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
            j += 1;
            j;
        }
        fft3(
            ((*s).tmp)
                .int32
                .offset(*sub_map.offset(i as isize) as isize),
            fft3in.as_mut_ptr(),
            m as ptrdiff_t,
        );
        i += 1;
        i;
    }
    let mut i_0: c_int = 0 as c_int;
    while i_0 < 3 as c_int {
        ((*s).fn_0[0 as c_int as usize]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0 as c_int as isize),
            ((*s).tmp).int32.offset((m * i_0) as isize) as *mut c_void,
            ((*s).tmp).int32.offset((m * i_0) as isize) as *mut c_void,
            size_of::<TXComplex>() as c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: c_int = 0 as c_int;
    while i_1 < len8 {
        let i0: c_int = len8 + i_1;
        let i1: c_int = len8 - i_1 - 1 as c_int;
        let s0: c_int = *out_map.offset(i0 as isize);
        let s1: c_int = *out_map.offset(i1 as isize);
        let src1: TXComplex = {
            AVComplexInt32 {
                re: (*((*s).tmp).int32.offset(s1 as isize)).re,
                im: (*((*s).tmp).int32.offset(s1 as isize)).im,
            }
        };
        let src0: TXComplex = {
            AVComplexInt32 {
                re: (*((*s).tmp).int32.offset(s0 as isize)).re,
                im: (*((*s).tmp).int32.offset(s0 as isize)).im,
            }
        };
        let mut accu_0: c_long = 0;
        accu_0 = (*exp.offset(i0 as isize)).im as c_long * src0.re as c_long;
        accu_0 -= (*exp.offset(i0 as isize)).re as c_long * src0.im as c_long;
        *dst.offset(((2 as c_int * i1) as c_long * stride + stride) as isize) =
            (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        accu_0 = (*exp.offset(i0 as isize)).re as c_long * src0.re as c_long;
        accu_0 += (*exp.offset(i0 as isize)).im as c_long * src0.im as c_long;
        *dst.offset(((2 as c_int * i0) as c_long * stride) as isize) =
            (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        let mut accu_1: c_long = 0;
        accu_1 = (*exp.offset(i1 as isize)).im as c_long * src1.re as c_long;
        accu_1 -= (*exp.offset(i1 as isize)).re as c_long * src1.im as c_long;
        *dst.offset(((2 as c_int * i0) as c_long * stride + stride) as isize) =
            (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        accu_1 = (*exp.offset(i1 as isize)).re as c_long * src1.re as c_long;
        accu_1 += (*exp.offset(i1 as isize)).im as c_long * src1.im as c_long;
        *dst.offset(((2 as c_int * i1) as c_long * stride) as isize) =
            (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        i_1 += 1;
        i_1;
    }
}
static mut ff_tx_mdct_pfa_3xM_fwd_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"mdct_pfa_3xM_fwd_int32_c".as_ptr(),
            function: Some(
                ff_tx_mdct_pfa_3xM_fwd_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_MDCT,
            flags: (AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | (1 as c_ulonglong) << 59 as c_int) as c_ulong,
            factors: [
                3 as c_int,
                -(1 as c_int),
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
            nb_factors: 2 as c_int,
            min_len: 3 as c_int * 2 as c_int,
            max_len: -(1 as c_int),
            init: Some(
                ff_tx_mdct_pfa_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
unsafe extern "C" fn ff_tx_mdct_pfa_5xM_fwd_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let mut fft5in: [TXComplex; 5] = [TXComplex { re: 0, im: 0 }; 5];
    let src: *mut TXSample = _src as *mut TXSample;
    let dst: *mut TXSample = _dst as *mut TXSample;
    let exp: *mut TXComplex = (*s).exp.int32;
    let mut tmp: TXComplex = TXComplex { re: 0, im: 0 };
    let m: c_int = (*(*s).sub).len;
    let len4: c_int = 5 as c_int * m;
    let len3: c_int = len4 * 3 as c_int;
    let len8: c_int = (*s).len >> 2 as c_int;
    let in_map: *const c_int = (*s).map;
    let out_map: *const c_int = in_map.offset((5 as c_int * m) as isize);
    let sub_map: *const c_int = (*(*s).sub).map;
    stride = (stride as c_ulong).wrapping_div(size_of::<TXSample>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: c_int = 0 as c_int;
    while i < m {
        let mut j: c_int = 0 as c_int;
        while j < 5 as c_int {
            let k: c_int = *in_map.offset((i * 5 as c_int + j) as isize);
            if k < len4 {
                tmp.re = (-*src.offset((len4 + k) as isize) as c_uint)
                    .wrapping_add(
                        *src.offset((1 as c_int * len4 - 1 as c_int - k) as isize) as c_uint
                    )
                    .wrapping_add(32 as c_int as c_uint) as c_int
                    >> 6 as c_int;
                tmp.im = (-*src.offset((len3 + k) as isize) as c_uint)
                    .wrapping_add(
                        -*src.offset((1 as c_int * len3 - 1 as c_int - k) as isize) as c_uint
                    )
                    .wrapping_add(32 as c_int as c_uint) as c_int
                    >> 6 as c_int;
            } else {
                tmp.re = (-*src.offset((len4 + k) as isize) as c_uint)
                    .wrapping_add(
                        -*src.offset((5 as c_int * len4 - 1 as c_int - k) as isize) as c_uint
                    )
                    .wrapping_add(32 as c_int as c_uint) as c_int
                    >> 6 as c_int;
                tmp.im = (*src.offset((-len4 + k) as isize) as c_uint)
                    .wrapping_add(
                        -*src.offset((1 as c_int * len3 - 1 as c_int - k) as isize) as c_uint
                    )
                    .wrapping_add(32 as c_int as c_uint) as c_int
                    >> 6 as c_int;
            }
            let mut accu: c_long = 0;
            accu = (*exp.offset((k >> 1 as c_int) as isize)).re as c_long * tmp.re as c_long;
            accu -= (*exp.offset((k >> 1 as c_int) as isize)).im as c_long * tmp.im as c_long;
            fft5in[j as usize].im = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
            accu = (*exp.offset((k >> 1 as c_int) as isize)).im as c_long * tmp.re as c_long;
            accu += (*exp.offset((k >> 1 as c_int) as isize)).re as c_long * tmp.im as c_long;
            fft5in[j as usize].re = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
            j += 1;
            j;
        }
        fft5(
            ((*s).tmp)
                .int32
                .offset(*sub_map.offset(i as isize) as isize),
            fft5in.as_mut_ptr(),
            m as ptrdiff_t,
        );
        i += 1;
        i;
    }
    let mut i_0: c_int = 0 as c_int;
    while i_0 < 5 as c_int {
        ((*s).fn_0[0 as c_int as usize]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0 as c_int as isize),
            ((*s).tmp).int32.offset((m * i_0) as isize) as *mut c_void,
            ((*s).tmp).int32.offset((m * i_0) as isize) as *mut c_void,
            size_of::<TXComplex>() as c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: c_int = 0 as c_int;
    while i_1 < len8 {
        let i0: c_int = len8 + i_1;
        let i1: c_int = len8 - i_1 - 1 as c_int;
        let s0: c_int = *out_map.offset(i0 as isize);
        let s1: c_int = *out_map.offset(i1 as isize);
        let src1: TXComplex = {
            AVComplexInt32 {
                re: (*((*s).tmp).int32.offset(s1 as isize)).re,
                im: (*((*s).tmp).int32.offset(s1 as isize)).im,
            }
        };
        let src0: TXComplex = {
            AVComplexInt32 {
                re: (*((*s).tmp).int32.offset(s0 as isize)).re,
                im: (*((*s).tmp).int32.offset(s0 as isize)).im,
            }
        };
        let mut accu_0: c_long = 0;
        accu_0 = (*exp.offset(i0 as isize)).im as c_long * src0.re as c_long;
        accu_0 -= (*exp.offset(i0 as isize)).re as c_long * src0.im as c_long;
        *dst.offset(((2 as c_int * i1) as c_long * stride + stride) as isize) =
            (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        accu_0 = (*exp.offset(i0 as isize)).re as c_long * src0.re as c_long;
        accu_0 += (*exp.offset(i0 as isize)).im as c_long * src0.im as c_long;
        *dst.offset(((2 as c_int * i0) as c_long * stride) as isize) =
            (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        let mut accu_1: c_long = 0;
        accu_1 = (*exp.offset(i1 as isize)).im as c_long * src1.re as c_long;
        accu_1 -= (*exp.offset(i1 as isize)).re as c_long * src1.im as c_long;
        *dst.offset(((2 as c_int * i0) as c_long * stride + stride) as isize) =
            (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        accu_1 = (*exp.offset(i1 as isize)).re as c_long * src1.re as c_long;
        accu_1 += (*exp.offset(i1 as isize)).im as c_long * src1.im as c_long;
        *dst.offset(((2 as c_int * i1) as c_long * stride) as isize) =
            (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        i_1 += 1;
        i_1;
    }
}
static mut ff_tx_mdct_pfa_5xM_fwd_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"mdct_pfa_5xM_fwd_int32_c".as_ptr(),
            function: Some(
                ff_tx_mdct_pfa_5xM_fwd_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_MDCT,
            flags: (AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | (1 as c_ulonglong) << 59 as c_int) as c_ulong,
            factors: [
                5 as c_int,
                -(1 as c_int),
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
            nb_factors: 2 as c_int,
            min_len: 5 as c_int * 2 as c_int,
            max_len: -(1 as c_int),
            init: Some(
                ff_tx_mdct_pfa_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
unsafe extern "C" fn ff_tx_mdct_pfa_7xM_fwd_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let mut fft7in: [TXComplex; 7] = [TXComplex { re: 0, im: 0 }; 7];
    let src: *mut TXSample = _src as *mut TXSample;
    let dst: *mut TXSample = _dst as *mut TXSample;
    let exp: *mut TXComplex = (*s).exp.int32;
    let mut tmp: TXComplex = TXComplex { re: 0, im: 0 };
    let m: c_int = (*(*s).sub).len;
    let len4: c_int = 7 as c_int * m;
    let len3: c_int = len4 * 3 as c_int;
    let len8: c_int = (*s).len >> 2 as c_int;
    let in_map: *const c_int = (*s).map;
    let out_map: *const c_int = in_map.offset((7 as c_int * m) as isize);
    let sub_map: *const c_int = (*(*s).sub).map;
    stride = (stride as c_ulong).wrapping_div(size_of::<TXSample>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: c_int = 0 as c_int;
    while i < m {
        let mut j: c_int = 0 as c_int;
        while j < 7 as c_int {
            let k: c_int = *in_map.offset((i * 7 as c_int + j) as isize);
            if k < len4 {
                tmp.re = (-*src.offset((len4 + k) as isize) as c_uint)
                    .wrapping_add(
                        *src.offset((1 as c_int * len4 - 1 as c_int - k) as isize) as c_uint
                    )
                    .wrapping_add(32 as c_int as c_uint) as c_int
                    >> 6 as c_int;
                tmp.im = (-*src.offset((len3 + k) as isize) as c_uint)
                    .wrapping_add(
                        -*src.offset((1 as c_int * len3 - 1 as c_int - k) as isize) as c_uint
                    )
                    .wrapping_add(32 as c_int as c_uint) as c_int
                    >> 6 as c_int;
            } else {
                tmp.re = (-*src.offset((len4 + k) as isize) as c_uint)
                    .wrapping_add(
                        -*src.offset((5 as c_int * len4 - 1 as c_int - k) as isize) as c_uint
                    )
                    .wrapping_add(32 as c_int as c_uint) as c_int
                    >> 6 as c_int;
                tmp.im = (*src.offset((-len4 + k) as isize) as c_uint)
                    .wrapping_add(
                        -*src.offset((1 as c_int * len3 - 1 as c_int - k) as isize) as c_uint
                    )
                    .wrapping_add(32 as c_int as c_uint) as c_int
                    >> 6 as c_int;
            }
            let mut accu: c_long = 0;
            accu = (*exp.offset((k >> 1 as c_int) as isize)).re as c_long * tmp.re as c_long;
            accu -= (*exp.offset((k >> 1 as c_int) as isize)).im as c_long * tmp.im as c_long;
            fft7in[j as usize].im = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
            accu = (*exp.offset((k >> 1 as c_int) as isize)).im as c_long * tmp.re as c_long;
            accu += (*exp.offset((k >> 1 as c_int) as isize)).re as c_long * tmp.im as c_long;
            fft7in[j as usize].re = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
            j += 1;
            j;
        }
        fft7(
            ((*s).tmp)
                .int32
                .offset(*sub_map.offset(i as isize) as isize),
            fft7in.as_mut_ptr(),
            m as ptrdiff_t,
        );
        i += 1;
        i;
    }
    let mut i_0: c_int = 0 as c_int;
    while i_0 < 7 as c_int {
        ((*s).fn_0[0 as c_int as usize]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0 as c_int as isize),
            ((*s).tmp).int32.offset((m * i_0) as isize) as *mut c_void,
            ((*s).tmp).int32.offset((m * i_0) as isize) as *mut c_void,
            size_of::<TXComplex>() as c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: c_int = 0 as c_int;
    while i_1 < len8 {
        let i0: c_int = len8 + i_1;
        let i1: c_int = len8 - i_1 - 1 as c_int;
        let s0: c_int = *out_map.offset(i0 as isize);
        let s1: c_int = *out_map.offset(i1 as isize);
        let src1: TXComplex = {
            AVComplexInt32 {
                re: (*((*s).tmp).int32.offset(s1 as isize)).re,
                im: (*((*s).tmp).int32.offset(s1 as isize)).im,
            }
        };
        let src0: TXComplex = {
            AVComplexInt32 {
                re: (*((*s).tmp).int32.offset(s0 as isize)).re,
                im: (*((*s).tmp).int32.offset(s0 as isize)).im,
            }
        };
        let mut accu_0: c_long = 0;
        accu_0 = (*exp.offset(i0 as isize)).im as c_long * src0.re as c_long;
        accu_0 -= (*exp.offset(i0 as isize)).re as c_long * src0.im as c_long;
        *dst.offset(((2 as c_int * i1) as c_long * stride + stride) as isize) =
            (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        accu_0 = (*exp.offset(i0 as isize)).re as c_long * src0.re as c_long;
        accu_0 += (*exp.offset(i0 as isize)).im as c_long * src0.im as c_long;
        *dst.offset(((2 as c_int * i0) as c_long * stride) as isize) =
            (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        let mut accu_1: c_long = 0;
        accu_1 = (*exp.offset(i1 as isize)).im as c_long * src1.re as c_long;
        accu_1 -= (*exp.offset(i1 as isize)).re as c_long * src1.im as c_long;
        *dst.offset(((2 as c_int * i0) as c_long * stride + stride) as isize) =
            (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        accu_1 = (*exp.offset(i1 as isize)).re as c_long * src1.re as c_long;
        accu_1 += (*exp.offset(i1 as isize)).im as c_long * src1.im as c_long;
        *dst.offset(((2 as c_int * i1) as c_long * stride) as isize) =
            (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        i_1 += 1;
        i_1;
    }
}
static mut ff_tx_mdct_pfa_7xM_fwd_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"mdct_pfa_7xM_fwd_int32_c".as_ptr(),
            function: Some(
                ff_tx_mdct_pfa_7xM_fwd_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_MDCT,
            flags: (AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | (1 as c_ulonglong) << 59 as c_int) as c_ulong,
            factors: [
                7 as c_int,
                -(1 as c_int),
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
            nb_factors: 2 as c_int,
            min_len: 7 as c_int * 2 as c_int,
            max_len: -(1 as c_int),
            init: Some(
                ff_tx_mdct_pfa_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
unsafe extern "C" fn ff_tx_mdct_pfa_9xM_fwd_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let mut fft9in: [TXComplex; 9] = [TXComplex { re: 0, im: 0 }; 9];
    let src: *mut TXSample = _src as *mut TXSample;
    let dst: *mut TXSample = _dst as *mut TXSample;
    let exp: *mut TXComplex = (*s).exp.int32;
    let mut tmp: TXComplex = TXComplex { re: 0, im: 0 };
    let m: c_int = (*(*s).sub).len;
    let len4: c_int = 9 as c_int * m;
    let len3: c_int = len4 * 3 as c_int;
    let len8: c_int = (*s).len >> 2 as c_int;
    let in_map: *const c_int = (*s).map;
    let out_map: *const c_int = in_map.offset((9 as c_int * m) as isize);
    let sub_map: *const c_int = (*(*s).sub).map;
    stride = (stride as c_ulong).wrapping_div(size_of::<TXSample>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: c_int = 0 as c_int;
    while i < m {
        let mut j: c_int = 0 as c_int;
        while j < 9 as c_int {
            let k: c_int = *in_map.offset((i * 9 as c_int + j) as isize);
            if k < len4 {
                tmp.re = (-*src.offset((len4 + k) as isize) as c_uint)
                    .wrapping_add(
                        *src.offset((1 as c_int * len4 - 1 as c_int - k) as isize) as c_uint
                    )
                    .wrapping_add(32 as c_int as c_uint) as c_int
                    >> 6 as c_int;
                tmp.im = (-*src.offset((len3 + k) as isize) as c_uint)
                    .wrapping_add(
                        -*src.offset((1 as c_int * len3 - 1 as c_int - k) as isize) as c_uint
                    )
                    .wrapping_add(32 as c_int as c_uint) as c_int
                    >> 6 as c_int;
            } else {
                tmp.re = (-*src.offset((len4 + k) as isize) as c_uint)
                    .wrapping_add(
                        -*src.offset((5 as c_int * len4 - 1 as c_int - k) as isize) as c_uint
                    )
                    .wrapping_add(32 as c_int as c_uint) as c_int
                    >> 6 as c_int;
                tmp.im = (*src.offset((-len4 + k) as isize) as c_uint)
                    .wrapping_add(
                        -*src.offset((1 as c_int * len3 - 1 as c_int - k) as isize) as c_uint
                    )
                    .wrapping_add(32 as c_int as c_uint) as c_int
                    >> 6 as c_int;
            }
            let mut accu: c_long = 0;
            accu = (*exp.offset((k >> 1 as c_int) as isize)).re as c_long * tmp.re as c_long;
            accu -= (*exp.offset((k >> 1 as c_int) as isize)).im as c_long * tmp.im as c_long;
            fft9in[j as usize].im = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
            accu = (*exp.offset((k >> 1 as c_int) as isize)).im as c_long * tmp.re as c_long;
            accu += (*exp.offset((k >> 1 as c_int) as isize)).re as c_long * tmp.im as c_long;
            fft9in[j as usize].re = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
            j += 1;
            j;
        }
        fft9(
            ((*s).tmp)
                .int32
                .offset(*sub_map.offset(i as isize) as isize),
            fft9in.as_mut_ptr(),
            m as ptrdiff_t,
        );
        i += 1;
        i;
    }
    let mut i_0: c_int = 0 as c_int;
    while i_0 < 9 as c_int {
        ((*s).fn_0[0 as c_int as usize]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0 as c_int as isize),
            ((*s).tmp).int32.offset((m * i_0) as isize) as *mut c_void,
            ((*s).tmp).int32.offset((m * i_0) as isize) as *mut c_void,
            size_of::<TXComplex>() as c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: c_int = 0 as c_int;
    while i_1 < len8 {
        let i0: c_int = len8 + i_1;
        let i1: c_int = len8 - i_1 - 1 as c_int;
        let s0: c_int = *out_map.offset(i0 as isize);
        let s1: c_int = *out_map.offset(i1 as isize);
        let src1: TXComplex = {
            AVComplexInt32 {
                re: (*((*s).tmp).int32.offset(s1 as isize)).re,
                im: (*((*s).tmp).int32.offset(s1 as isize)).im,
            }
        };
        let src0: TXComplex = {
            AVComplexInt32 {
                re: (*((*s).tmp).int32.offset(s0 as isize)).re,
                im: (*((*s).tmp).int32.offset(s0 as isize)).im,
            }
        };
        let mut accu_0: c_long = 0;
        accu_0 = (*exp.offset(i0 as isize)).im as c_long * src0.re as c_long;
        accu_0 -= (*exp.offset(i0 as isize)).re as c_long * src0.im as c_long;
        *dst.offset(((2 as c_int * i1) as c_long * stride + stride) as isize) =
            (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        accu_0 = (*exp.offset(i0 as isize)).re as c_long * src0.re as c_long;
        accu_0 += (*exp.offset(i0 as isize)).im as c_long * src0.im as c_long;
        *dst.offset(((2 as c_int * i0) as c_long * stride) as isize) =
            (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        let mut accu_1: c_long = 0;
        accu_1 = (*exp.offset(i1 as isize)).im as c_long * src1.re as c_long;
        accu_1 -= (*exp.offset(i1 as isize)).re as c_long * src1.im as c_long;
        *dst.offset(((2 as c_int * i0) as c_long * stride + stride) as isize) =
            (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        accu_1 = (*exp.offset(i1 as isize)).re as c_long * src1.re as c_long;
        accu_1 += (*exp.offset(i1 as isize)).im as c_long * src1.im as c_long;
        *dst.offset(((2 as c_int * i1) as c_long * stride) as isize) =
            (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        i_1 += 1;
        i_1;
    }
}
static mut ff_tx_mdct_pfa_9xM_fwd_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"mdct_pfa_9xM_fwd_int32_c".as_ptr(),
            function: Some(
                ff_tx_mdct_pfa_9xM_fwd_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_MDCT,
            flags: (AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | (1 as c_ulonglong) << 59 as c_int) as c_ulong,
            factors: [
                9 as c_int,
                -(1 as c_int),
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
            nb_factors: 2 as c_int,
            min_len: 9 as c_int * 2 as c_int,
            max_len: -(1 as c_int),
            init: Some(
                ff_tx_mdct_pfa_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
unsafe extern "C" fn ff_tx_mdct_pfa_15xM_fwd_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let mut fft15in: [TXComplex; 15] = [TXComplex { re: 0, im: 0 }; 15];
    let src: *mut TXSample = _src as *mut TXSample;
    let dst: *mut TXSample = _dst as *mut TXSample;
    let exp: *mut TXComplex = (*s).exp.int32;
    let mut tmp: TXComplex = TXComplex { re: 0, im: 0 };
    let m: c_int = (*(*s).sub).len;
    let len4: c_int = 15 as c_int * m;
    let len3: c_int = len4 * 3 as c_int;
    let len8: c_int = (*s).len >> 2 as c_int;
    let in_map: *const c_int = (*s).map;
    let out_map: *const c_int = in_map.offset((15 as c_int * m) as isize);
    let sub_map: *const c_int = (*(*s).sub).map;
    stride = (stride as c_ulong).wrapping_div(size_of::<TXSample>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: c_int = 0 as c_int;
    while i < m {
        let mut j: c_int = 0 as c_int;
        while j < 15 as c_int {
            let k: c_int = *in_map.offset((i * 15 as c_int + j) as isize);
            if k < len4 {
                tmp.re = (-*src.offset((len4 + k) as isize) as c_uint)
                    .wrapping_add(
                        *src.offset((1 as c_int * len4 - 1 as c_int - k) as isize) as c_uint
                    )
                    .wrapping_add(32 as c_int as c_uint) as c_int
                    >> 6 as c_int;
                tmp.im = (-*src.offset((len3 + k) as isize) as c_uint)
                    .wrapping_add(
                        -*src.offset((1 as c_int * len3 - 1 as c_int - k) as isize) as c_uint
                    )
                    .wrapping_add(32 as c_int as c_uint) as c_int
                    >> 6 as c_int;
            } else {
                tmp.re = (-*src.offset((len4 + k) as isize) as c_uint)
                    .wrapping_add(
                        -*src.offset((5 as c_int * len4 - 1 as c_int - k) as isize) as c_uint
                    )
                    .wrapping_add(32 as c_int as c_uint) as c_int
                    >> 6 as c_int;
                tmp.im = (*src.offset((-len4 + k) as isize) as c_uint)
                    .wrapping_add(
                        -*src.offset((1 as c_int * len3 - 1 as c_int - k) as isize) as c_uint
                    )
                    .wrapping_add(32 as c_int as c_uint) as c_int
                    >> 6 as c_int;
            }
            let mut accu: c_long = 0;
            accu = (*exp.offset((k >> 1 as c_int) as isize)).re as c_long * tmp.re as c_long;
            accu -= (*exp.offset((k >> 1 as c_int) as isize)).im as c_long * tmp.im as c_long;
            fft15in[j as usize].im = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
            accu = (*exp.offset((k >> 1 as c_int) as isize)).im as c_long * tmp.re as c_long;
            accu += (*exp.offset((k >> 1 as c_int) as isize)).re as c_long * tmp.im as c_long;
            fft15in[j as usize].re = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
            j += 1;
            j;
        }
        fft15(
            ((*s).tmp)
                .int32
                .offset(*sub_map.offset(i as isize) as isize),
            fft15in.as_mut_ptr(),
            m as ptrdiff_t,
        );
        i += 1;
        i;
    }
    let mut i_0: c_int = 0 as c_int;
    while i_0 < 15 as c_int {
        ((*s).fn_0[0 as c_int as usize]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0 as c_int as isize),
            ((*s).tmp).int32.offset((m * i_0) as isize) as *mut c_void,
            ((*s).tmp).int32.offset((m * i_0) as isize) as *mut c_void,
            size_of::<TXComplex>() as c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: c_int = 0 as c_int;
    while i_1 < len8 {
        let i0: c_int = len8 + i_1;
        let i1: c_int = len8 - i_1 - 1 as c_int;
        let s0: c_int = *out_map.offset(i0 as isize);
        let s1: c_int = *out_map.offset(i1 as isize);
        let src1: TXComplex = {
            AVComplexInt32 {
                re: (*((*s).tmp).int32.offset(s1 as isize)).re,
                im: (*((*s).tmp).int32.offset(s1 as isize)).im,
            }
        };
        let src0: TXComplex = {
            AVComplexInt32 {
                re: (*((*s).tmp).int32.offset(s0 as isize)).re,
                im: (*((*s).tmp).int32.offset(s0 as isize)).im,
            }
        };
        let mut accu_0: c_long = 0;
        accu_0 = (*exp.offset(i0 as isize)).im as c_long * src0.re as c_long;
        accu_0 -= (*exp.offset(i0 as isize)).re as c_long * src0.im as c_long;
        *dst.offset(((2 as c_int * i1) as c_long * stride + stride) as isize) =
            (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        accu_0 = (*exp.offset(i0 as isize)).re as c_long * src0.re as c_long;
        accu_0 += (*exp.offset(i0 as isize)).im as c_long * src0.im as c_long;
        *dst.offset(((2 as c_int * i0) as c_long * stride) as isize) =
            (accu_0 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        let mut accu_1: c_long = 0;
        accu_1 = (*exp.offset(i1 as isize)).im as c_long * src1.re as c_long;
        accu_1 -= (*exp.offset(i1 as isize)).re as c_long * src1.im as c_long;
        *dst.offset(((2 as c_int * i0) as c_long * stride + stride) as isize) =
            (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        accu_1 = (*exp.offset(i1 as isize)).re as c_long * src1.re as c_long;
        accu_1 += (*exp.offset(i1 as isize)).im as c_long * src1.im as c_long;
        *dst.offset(((2 as c_int * i1) as c_long * stride) as isize) =
            (accu_1 + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        i_1 += 1;
        i_1;
    }
}
static mut ff_tx_mdct_pfa_15xM_fwd_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"mdct_pfa_15xM_fwd_int32_c".as_ptr(),
            function: Some(
                ff_tx_mdct_pfa_15xM_fwd_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_MDCT,
            flags: (AV_TX_UNALIGNED as c_int as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | (1 as c_ulonglong) << 59 as c_int) as c_ulong,
            factors: [
                15 as c_int,
                -(1 as c_int),
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
            nb_factors: 2 as c_int,
            min_len: 15 as c_int * 2 as c_int,
            max_len: -(1 as c_int),
            init: Some(
                ff_tx_mdct_pfa_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
#[cold]
unsafe extern "C" fn ff_tx_rdft_init_int32_c(
    s: *mut AVTXContext,
    _cd: *const FFTXCodelet,
    mut flags: c_ulong,
    _opts: *mut FFTXCodeletOptions,
    len: c_int,
    inv: c_int,
    scale: *const c_void,
) -> c_int {
    let mut ret: c_int = 0;
    let mut f: c_double = 0.;
    let mut m: c_double = 0.;
    let mut tab: *mut TXSample = std::ptr::null_mut::<TXSample>();
    let r2r: c_ulong = flags & AV_TX_REAL_TO_REAL as c_int as c_ulong;
    let len4: c_int = (len + 4 as c_int - 1 as c_int & !(4 as c_int - 1 as c_int)) / 4 as c_int;
    (*s).scale_d = *(scale as *mut c_float) as c_double;
    (*s).scale_f = (*s).scale_d as c_float;
    flags &= !(AV_TX_REAL_TO_REAL as c_int | AV_TX_REAL_TO_IMAGINARY as c_int) as c_ulong;
    ret = ff_tx_init_subtx(
        s,
        AV_TX_INT32_FFT,
        flags,
        std::ptr::null_mut::<FFTXCodeletOptions>(),
        len >> 1 as c_int,
        inv,
        scale,
    );
    if ret != 0 {
        return ret;
    }
    (*s).exp = AVTXNum {
        int32: av_mallocz(
            ((8 as c_int + 2 as c_int * len4) as c_ulong)
                .wrapping_mul(size_of::<TXComplex>() as c_ulong),
        ) as *mut TXComplex,
    };
    if ((*s).exp).int32.is_null() {
        return -(12 as c_int);
    }
    tab = (*s).exp.int32 as *mut TXSample;
    f = 2. * PI / len as c_double;
    m = if inv != 0 {
        2. * (*s).scale_d
    } else {
        (*s).scale_d
    };
    let fresh27 = tab;
    tab = tab.offset(1);
    *fresh27 = av_clip64_c(
        llrintf(((if inv != 0 { 0.5f64 } else { 1.0f64 }) * m * 2147483648.0f64) as c_float)
            as c_long,
        (-(2147483647 as c_int) - 1 as c_int) as c_long,
        2147483647 as c_int as c_long,
    ) as TXSample;
    let fresh28 = tab;
    tab = tab.offset(1);
    *fresh28 = av_clip64_c(
        llrintf(((if inv != 0 { 0.5f64 * m } else { 1.0f64 * m }) * 2147483648.0f64) as c_float)
            as c_long,
        (-(2147483647 as c_int) - 1 as c_int) as c_long,
        2147483647 as c_int as c_long,
    ) as TXSample;
    let fresh29 = tab;
    tab = tab.offset(1);
    *fresh29 = av_clip64_c(
        llrintf((m * 2147483648.0f64) as c_float) as c_long,
        (-(2147483647 as c_int) - 1 as c_int) as c_long,
        2147483647 as c_int as c_long,
    ) as TXSample;
    let fresh30 = tab;
    tab = tab.offset(1);
    *fresh30 = av_clip64_c(
        llrintf((-m * 2147483648.0f64) as c_float) as c_long,
        (-(2147483647 as c_int) - 1 as c_int) as c_long,
        2147483647 as c_int as c_long,
    ) as TXSample;
    let fresh31 = tab;
    tab = tab.offset(1);
    *fresh31 = av_clip64_c(
        llrintf(((0.5f64 - 0.0f64) * m * 2147483648.0f64) as c_float) as c_long,
        (-(2147483647 as c_int) - 1 as c_int) as c_long,
        2147483647 as c_int as c_long,
    ) as TXSample;
    if r2r != 0 {
        let fresh32 = tab;
        tab = tab.offset(1);
        *fresh32 = (1 as c_int as c_float / (*s).scale_f) as TXSample;
    } else {
        let fresh33 = tab;
        tab = tab.offset(1);
        *fresh33 = av_clip64_c(
            llrintf(((0.0f64 - 0.5f64) * m * 2147483648.0f64) as c_float) as c_long,
            (-(2147483647 as c_int) - 1 as c_int) as c_long,
            2147483647 as c_int as c_long,
        ) as TXSample;
    }
    let fresh34 = tab;
    tab = tab.offset(1);
    *fresh34 = av_clip64_c(
        llrintf(((0.5f64 - inv as c_double) * m * 2147483648.0f64) as c_float) as c_long,
        (-(2147483647 as c_int) - 1 as c_int) as c_long,
        2147483647 as c_int as c_long,
    ) as TXSample;
    let fresh35 = tab;
    tab = tab.offset(1);
    *fresh35 = av_clip64_c(
        llrintf((-(0.5f64 - inv as c_double) * m * 2147483648.0f64) as c_float) as c_long,
        (-(2147483647 as c_int) - 1 as c_int) as c_long,
        2147483647 as c_int as c_long,
    ) as TXSample;
    let mut i: c_int = 0 as c_int;
    while i < len4 {
        let fresh36 = tab;
        tab = tab.offset(1);
        *fresh36 = av_clip64_c(
            llrintf((cos(i as c_double * f) * 2147483648.0f64) as c_float) as c_long,
            (-(2147483647 as c_int) - 1 as c_int) as c_long,
            2147483647 as c_int as c_long,
        ) as TXSample;
        i += 1;
        i;
    }
    tab = ((*s).exp.int32 as *mut TXSample)
        .offset(len4 as isize)
        .offset(8 as c_int as isize);
    let mut i_0: c_int = 0 as c_int;
    while i_0 < len4 {
        let fresh37 = tab;
        tab = tab.offset(1);
        *fresh37 = (av_clip64_c(
            llrintf(
                (cos((len - i_0 * 4 as c_int) as c_double / 4.0f64 * f) * 2147483648.0f64)
                    as c_float,
            ) as c_long,
            (-(2147483647 as c_int) - 1 as c_int) as c_long,
            2147483647 as c_int as c_long,
        ) * (if inv != 0 { 1 as c_int } else { -(1 as c_int) }) as c_long)
            as TXSample;
        i_0 += 1;
        i_0;
    }
    0 as c_int
}
static mut ff_tx_rdft_r2c_def_int32_c: FFTXCodelet = FFTXCodelet {
    name: 0 as *const c_char,
    function: None,
    type_0: AV_TX_FLOAT_FFT,
    flags: 0,
    factors: [0; 16],
    nb_factors: 0,
    min_len: 0,
    max_len: 0,
    init: None,
    uninit: None,
    cpu_flags: 0,
    prio: 0,
};
unsafe extern "C" fn ff_tx_rdft_r2c_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    _stride: ptrdiff_t,
) {
    let len2: c_int = (*s).len >> 1 as c_int;
    let len4: c_int = (*s).len >> 2 as c_int;
    let fact: *const TXSample = (*s).exp.int32 as *mut c_void as *const TXSample;
    let tcos: *const TXSample = fact.offset(8 as c_int as isize);
    let tsin: *const TXSample = tcos.offset(len4 as isize);
    let data: *mut TXComplex = (if 0 as c_int != 0 { _src } else { _dst }) as *mut TXComplex;
    let mut t: [TXComplex; 3] = [TXComplex { re: 0, im: 0 }; 3];
    if 0 as c_int == 0 {
        ((*s).fn_0[0 as c_int as usize]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0 as c_int as isize),
            data as *mut c_void,
            _src,
            size_of::<TXComplex>() as c_ulong as ptrdiff_t,
        );
    } else {
        (*data.offset(0 as c_int as isize)).im = (*data.offset(len2 as isize)).re;
    }
    t[0 as c_int as usize].re = (*data.offset(0 as c_int as isize)).re;
    (*data.offset(0 as c_int as isize)).re =
        t[0 as c_int as usize].re + (*data.offset(0 as c_int as isize)).im;
    (*data.offset(0 as c_int as isize)).im =
        t[0 as c_int as usize].re - (*data.offset(0 as c_int as isize)).im;
    (*data.offset(0 as c_int as isize)).re = (*fact.offset(0 as c_int as isize) as c_long
        * (*data.offset(0 as c_int as isize)).re as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    (*data.offset(0 as c_int as isize)).im = (*fact.offset(1 as c_int as isize) as c_long
        * (*data.offset(0 as c_int as isize)).im as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    (*data.offset(len4 as isize)).re = (*fact.offset(2 as c_int as isize) as c_long
        * (*data.offset(len4 as isize)).re as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    (*data.offset(len4 as isize)).im = (*fact.offset(3 as c_int as isize) as c_long
        * (*data.offset(len4 as isize)).im as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    let mut i: c_int = 1 as c_int;
    while i < len4 {
        t[0 as c_int as usize].re = (*fact.offset(4 as c_int as isize) as c_long
            * ((*data.offset(i as isize)).re + (*data.offset((len2 - i) as isize)).re) as c_long
            + 0x40000000 as c_int as c_long
            >> 31 as c_int) as c_int;
        t[0 as c_int as usize].im = (*fact.offset(5 as c_int as isize) as c_long
            * ((*data.offset(i as isize)).im - (*data.offset((len2 - i) as isize)).im) as c_long
            + 0x40000000 as c_int as c_long
            >> 31 as c_int) as c_int;
        t[1 as c_int as usize].re = (*fact.offset(6 as c_int as isize) as c_long
            * ((*data.offset(i as isize)).im + (*data.offset((len2 - i) as isize)).im) as c_long
            + 0x40000000 as c_int as c_long
            >> 31 as c_int) as c_int;
        t[1 as c_int as usize].im = (*fact.offset(7 as c_int as isize) as c_long
            * ((*data.offset(i as isize)).re - (*data.offset((len2 - i) as isize)).re) as c_long
            + 0x40000000 as c_int as c_long
            >> 31 as c_int) as c_int;
        let mut accu: c_long = 0;
        accu = *tcos.offset(i as isize) as c_long * t[1 as c_int as usize].re as c_long;
        accu -= *tsin.offset(i as isize) as c_long * t[1 as c_int as usize].im as c_long;
        t[2 as c_int as usize].re = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        accu = *tsin.offset(i as isize) as c_long * t[1 as c_int as usize].re as c_long;
        accu += *tcos.offset(i as isize) as c_long * t[1 as c_int as usize].im as c_long;
        t[2 as c_int as usize].im = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        (*data.offset(i as isize)).re = t[0 as c_int as usize].re + t[2 as c_int as usize].re;
        (*data.offset(i as isize)).im = t[2 as c_int as usize].im - t[0 as c_int as usize].im;
        (*data.offset((len2 - i) as isize)).re =
            t[0 as c_int as usize].re - t[2 as c_int as usize].re;
        (*data.offset((len2 - i) as isize)).im =
            t[2 as c_int as usize].im + t[0 as c_int as usize].im;
        i += 1;
        i;
    }
    (*data.offset(len2 as isize)).re = (*data.offset(0 as c_int as isize)).im;
    let fresh38 = &mut (*data.offset(len2 as isize)).im;
    *fresh38 = 0 as c_int;
    (*data.offset(0 as c_int as isize)).im = *fresh38;
}
static mut ff_tx_rdft_c2r_def_int32_c: FFTXCodelet = FFTXCodelet {
    name: 0 as *const c_char,
    function: None,
    type_0: AV_TX_FLOAT_FFT,
    flags: 0,
    factors: [0; 16],
    nb_factors: 0,
    min_len: 0,
    max_len: 0,
    init: None,
    uninit: None,
    cpu_flags: 0,
    prio: 0,
};
unsafe extern "C" fn ff_tx_rdft_c2r_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    _stride: ptrdiff_t,
) {
    let len2: c_int = (*s).len >> 1 as c_int;
    let len4: c_int = (*s).len >> 2 as c_int;
    let fact: *const TXSample = (*s).exp.int32 as *mut c_void as *const TXSample;
    let tcos: *const TXSample = fact.offset(8 as c_int as isize);
    let tsin: *const TXSample = tcos.offset(len4 as isize);
    let data: *mut TXComplex = (if 1 as c_int != 0 { _src } else { _dst }) as *mut TXComplex;
    let mut t: [TXComplex; 3] = [TXComplex { re: 0, im: 0 }; 3];
    if 1 as c_int == 0 {
        ((*s).fn_0[0 as c_int as usize]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0 as c_int as isize),
            data as *mut c_void,
            _src,
            size_of::<TXComplex>() as c_ulong as ptrdiff_t,
        );
    } else {
        (*data.offset(0 as c_int as isize)).im = (*data.offset(len2 as isize)).re;
    }
    t[0 as c_int as usize].re = (*data.offset(0 as c_int as isize)).re;
    (*data.offset(0 as c_int as isize)).re =
        t[0 as c_int as usize].re + (*data.offset(0 as c_int as isize)).im;
    (*data.offset(0 as c_int as isize)).im =
        t[0 as c_int as usize].re - (*data.offset(0 as c_int as isize)).im;
    (*data.offset(0 as c_int as isize)).re = (*fact.offset(0 as c_int as isize) as c_long
        * (*data.offset(0 as c_int as isize)).re as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    (*data.offset(0 as c_int as isize)).im = (*fact.offset(1 as c_int as isize) as c_long
        * (*data.offset(0 as c_int as isize)).im as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    (*data.offset(len4 as isize)).re = (*fact.offset(2 as c_int as isize) as c_long
        * (*data.offset(len4 as isize)).re as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    (*data.offset(len4 as isize)).im = (*fact.offset(3 as c_int as isize) as c_long
        * (*data.offset(len4 as isize)).im as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    let mut i: c_int = 1 as c_int;
    while i < len4 {
        t[0 as c_int as usize].re = (*fact.offset(4 as c_int as isize) as c_long
            * ((*data.offset(i as isize)).re + (*data.offset((len2 - i) as isize)).re) as c_long
            + 0x40000000 as c_int as c_long
            >> 31 as c_int) as c_int;
        t[0 as c_int as usize].im = (*fact.offset(5 as c_int as isize) as c_long
            * ((*data.offset(i as isize)).im - (*data.offset((len2 - i) as isize)).im) as c_long
            + 0x40000000 as c_int as c_long
            >> 31 as c_int) as c_int;
        t[1 as c_int as usize].re = (*fact.offset(6 as c_int as isize) as c_long
            * ((*data.offset(i as isize)).im + (*data.offset((len2 - i) as isize)).im) as c_long
            + 0x40000000 as c_int as c_long
            >> 31 as c_int) as c_int;
        t[1 as c_int as usize].im = (*fact.offset(7 as c_int as isize) as c_long
            * ((*data.offset(i as isize)).re - (*data.offset((len2 - i) as isize)).re) as c_long
            + 0x40000000 as c_int as c_long
            >> 31 as c_int) as c_int;
        let mut accu: c_long = 0;
        accu = *tcos.offset(i as isize) as c_long * t[1 as c_int as usize].re as c_long;
        accu -= *tsin.offset(i as isize) as c_long * t[1 as c_int as usize].im as c_long;
        t[2 as c_int as usize].re = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        accu = *tsin.offset(i as isize) as c_long * t[1 as c_int as usize].re as c_long;
        accu += *tcos.offset(i as isize) as c_long * t[1 as c_int as usize].im as c_long;
        t[2 as c_int as usize].im = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        (*data.offset(i as isize)).re = t[0 as c_int as usize].re + t[2 as c_int as usize].re;
        (*data.offset(i as isize)).im = t[2 as c_int as usize].im - t[0 as c_int as usize].im;
        (*data.offset((len2 - i) as isize)).re =
            t[0 as c_int as usize].re - t[2 as c_int as usize].re;
        (*data.offset((len2 - i) as isize)).im =
            t[2 as c_int as usize].im + t[0 as c_int as usize].im;
        i += 1;
        i;
    }
    ((*s).fn_0[0 as c_int as usize]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0 as c_int as isize),
        _dst,
        data as *mut c_void,
        size_of::<TXComplex>() as c_ulong as ptrdiff_t,
    );
}
unsafe extern "C" fn ff_tx_rdft_r2r_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    _stride: ptrdiff_t,
) {
    let len: c_int = (*s).len;
    let len2: c_int = len >> 1 as c_int;
    let len4: c_int = len >> 2 as c_int;
    let aligned_len4: c_int =
        (len + 4 as c_int - 1 as c_int & !(4 as c_int - 1 as c_int)) / 4 as c_int;
    let fact: *const TXSample = (*s).exp.int32 as *mut c_void as *const TXSample;
    let tcos: *const TXSample = fact.offset(8 as c_int as isize);
    let tsin: *const TXSample = tcos.offset(aligned_len4 as isize);
    let data: *mut TXComplex = _dst as *mut TXComplex;
    let out: *mut TXSample = _dst as *mut TXSample;
    let mut tmp_dc: TXSample = 0;
    let mut tmp_mid: TXSample = 0;
    let mut tmp: [TXSample; 4] = [0; 4];
    let mut sf: TXComplex = TXComplex { re: 0, im: 0 };
    let mut sl: TXComplex = TXComplex { re: 0, im: 0 };
    ((*s).fn_0[0 as c_int as usize]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0 as c_int as isize),
        _dst,
        _src,
        size_of::<TXComplex>() as c_ulong as ptrdiff_t,
    );
    tmp_dc = (*data.offset(0 as c_int as isize)).re;
    (*data.offset(0 as c_int as isize)).re = tmp_dc + (*data.offset(0 as c_int as isize)).im;
    tmp_dc -= (*data.offset(0 as c_int as isize)).im;
    (*data.offset(0 as c_int as isize)).re = (*fact.offset(0 as c_int as isize) as c_long
        * (*data.offset(0 as c_int as isize)).re as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    tmp_dc = (*fact.offset(1 as c_int as isize) as c_long * tmp_dc as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as TXSample;
    (*data.offset(len4 as isize)).re = (*fact.offset(2 as c_int as isize) as c_long
        * (*data.offset(len4 as isize)).re as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    if 0 as c_int == 0 {
        (*data.offset(len4 as isize)).im = (*fact.offset(3 as c_int as isize) as c_long
            * (*data.offset(len4 as isize)).im as c_long
            + 0x40000000 as c_int as c_long
            >> 31 as c_int) as c_int;
    } else {
        sf = *data.offset(len4 as isize);
        sl = *data.offset((len4 + 1 as c_int) as isize);
        if AV_TX_REAL_TO_REAL as c_int == AV_TX_REAL_TO_REAL as c_int {
            tmp[0 as c_int as usize] = (*fact.offset(4 as c_int as isize) as c_long
                * (sf.re + sl.re) as c_long
                + 0x40000000 as c_int as c_long
                >> 31 as c_int) as TXSample;
        } else {
            tmp[0 as c_int as usize] = (*fact.offset(5 as c_int as isize) as c_long
                * (sf.im - sl.im) as c_long
                + 0x40000000 as c_int as c_long
                >> 31 as c_int) as TXSample;
        }
        tmp[1 as c_int as usize] = (*fact.offset(6 as c_int as isize) as c_long
            * (sf.im + sl.im) as c_long
            + 0x40000000 as c_int as c_long
            >> 31 as c_int) as TXSample;
        tmp[2 as c_int as usize] = (*fact.offset(7 as c_int as isize) as c_long
            * (sf.re - sl.re) as c_long
            + 0x40000000 as c_int as c_long
            >> 31 as c_int) as TXSample;
        if AV_TX_REAL_TO_REAL as c_int == AV_TX_REAL_TO_REAL as c_int {
            tmp[3 as c_int as usize] = tmp[1 as c_int as usize] * *tcos.offset(len4 as isize)
                - tmp[2 as c_int as usize] * *tsin.offset(len4 as isize);
            tmp_mid = tmp[0 as c_int as usize] - tmp[3 as c_int as usize];
        } else {
            tmp[3 as c_int as usize] = tmp[1 as c_int as usize] * *tsin.offset(len4 as isize)
                + tmp[2 as c_int as usize] * *tcos.offset(len4 as isize);
            tmp_mid = tmp[0 as c_int as usize] + tmp[3 as c_int as usize];
        }
    }
    let mut i: c_int = 1 as c_int;
    while i <= len4 {
        let mut tmp_0: [TXSample; 4] = [0; 4];
        let sf_0: TXComplex = *data.offset(i as isize);
        let sl_0: TXComplex = *data.offset((len2 - i) as isize);
        if AV_TX_REAL_TO_REAL as c_int == AV_TX_REAL_TO_REAL as c_int {
            tmp_0[0 as c_int as usize] = (*fact.offset(4 as c_int as isize) as c_long
                * (sf_0.re + sl_0.re) as c_long
                + 0x40000000 as c_int as c_long
                >> 31 as c_int) as TXSample;
        } else {
            tmp_0[0 as c_int as usize] = (*fact.offset(5 as c_int as isize) as c_long
                * (sf_0.im - sl_0.im) as c_long
                + 0x40000000 as c_int as c_long
                >> 31 as c_int) as TXSample;
        }
        tmp_0[1 as c_int as usize] = (*fact.offset(6 as c_int as isize) as c_long
            * (sf_0.im + sl_0.im) as c_long
            + 0x40000000 as c_int as c_long
            >> 31 as c_int) as TXSample;
        tmp_0[2 as c_int as usize] = (*fact.offset(7 as c_int as isize) as c_long
            * (sf_0.re - sl_0.re) as c_long
            + 0x40000000 as c_int as c_long
            >> 31 as c_int) as TXSample;
        if AV_TX_REAL_TO_REAL as c_int == AV_TX_REAL_TO_REAL as c_int {
            tmp_0[3 as c_int as usize] = tmp_0[1 as c_int as usize] * *tcos.offset(i as isize)
                - tmp_0[2 as c_int as usize] * *tsin.offset(i as isize);
            *out.offset(i as isize) = tmp_0[0 as c_int as usize] + tmp_0[3 as c_int as usize];
            *out.offset((len - i) as isize) =
                tmp_0[0 as c_int as usize] - tmp_0[3 as c_int as usize];
        } else {
            tmp_0[3 as c_int as usize] = tmp_0[1 as c_int as usize] * *tsin.offset(i as isize)
                + tmp_0[2 as c_int as usize] * *tcos.offset(i as isize);
            *out.offset((i - 1 as c_int) as isize) =
                tmp_0[3 as c_int as usize] - tmp_0[0 as c_int as usize];
            *out.offset((len - i - 1 as c_int) as isize) =
                tmp_0[0 as c_int as usize] + tmp_0[3 as c_int as usize];
        }
        i += 1;
        i;
    }
    let mut i_0: c_int = 1 as c_int;
    while i_0 < len4 + (AV_TX_REAL_TO_REAL as c_int == AV_TX_REAL_TO_IMAGINARY as c_int) as c_int {
        *out.offset((len2 - i_0) as isize) = *out.offset((len - i_0) as isize);
        i_0 += 1;
        i_0;
    }
    if AV_TX_REAL_TO_REAL as c_int == AV_TX_REAL_TO_REAL as c_int {
        *out.offset(len2 as isize) = tmp_dc;
    }
}
static mut ff_tx_rdft_r2r_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"rdft_r2r_int32_c".as_ptr(),
            function: Some(
                ff_tx_rdft_r2r_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_RDFT,
            flags: ((AV_TX_UNALIGNED as c_int
                | AV_TX_INPLACE as c_int
                | AV_TX_REAL_TO_REAL as c_int) as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | (1 as c_ulonglong) << 59 as c_int) as c_ulong,
            factors: [
                2 as c_int + 2 as c_int * (0 as c_int == 0) as c_int,
                -(1 as c_int),
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
            nb_factors: 2 as c_int,
            min_len: 2 as c_int + 2 as c_int * (0 as c_int == 0) as c_int,
            max_len: -(1 as c_int),
            init: Some(
                ff_tx_rdft_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
unsafe extern "C" fn ff_tx_rdft_r2r_mod2_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    _stride: ptrdiff_t,
) {
    let len: c_int = (*s).len;
    let len2: c_int = len >> 1 as c_int;
    let len4: c_int = len >> 2 as c_int;
    let aligned_len4: c_int =
        (len + 4 as c_int - 1 as c_int & !(4 as c_int - 1 as c_int)) / 4 as c_int;
    let fact: *const TXSample = (*s).exp.int32 as *mut c_void as *const TXSample;
    let tcos: *const TXSample = fact.offset(8 as c_int as isize);
    let tsin: *const TXSample = tcos.offset(aligned_len4 as isize);
    let data: *mut TXComplex = _dst as *mut TXComplex;
    let out: *mut TXSample = _dst as *mut TXSample;
    let mut tmp_dc: TXSample = 0;
    let mut tmp_mid: TXSample = 0;
    let mut tmp: [TXSample; 4] = [0; 4];
    let mut sf: TXComplex = TXComplex { re: 0, im: 0 };
    let mut sl: TXComplex = TXComplex { re: 0, im: 0 };
    ((*s).fn_0[0 as c_int as usize]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0 as c_int as isize),
        _dst,
        _src,
        size_of::<TXComplex>() as c_ulong as ptrdiff_t,
    );
    tmp_dc = (*data.offset(0 as c_int as isize)).re;
    (*data.offset(0 as c_int as isize)).re = tmp_dc + (*data.offset(0 as c_int as isize)).im;
    tmp_dc -= (*data.offset(0 as c_int as isize)).im;
    (*data.offset(0 as c_int as isize)).re = (*fact.offset(0 as c_int as isize) as c_long
        * (*data.offset(0 as c_int as isize)).re as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    tmp_dc = (*fact.offset(1 as c_int as isize) as c_long * tmp_dc as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as TXSample;
    (*data.offset(len4 as isize)).re = (*fact.offset(2 as c_int as isize) as c_long
        * (*data.offset(len4 as isize)).re as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    if 1 as c_int == 0 {
        (*data.offset(len4 as isize)).im = (*fact.offset(3 as c_int as isize) as c_long
            * (*data.offset(len4 as isize)).im as c_long
            + 0x40000000 as c_int as c_long
            >> 31 as c_int) as c_int;
    } else {
        sf = *data.offset(len4 as isize);
        sl = *data.offset((len4 + 1 as c_int) as isize);
        if AV_TX_REAL_TO_REAL as c_int == AV_TX_REAL_TO_REAL as c_int {
            tmp[0 as c_int as usize] = (*fact.offset(4 as c_int as isize) as c_long
                * (sf.re + sl.re) as c_long
                + 0x40000000 as c_int as c_long
                >> 31 as c_int) as TXSample;
        } else {
            tmp[0 as c_int as usize] = (*fact.offset(5 as c_int as isize) as c_long
                * (sf.im - sl.im) as c_long
                + 0x40000000 as c_int as c_long
                >> 31 as c_int) as TXSample;
        }
        tmp[1 as c_int as usize] = (*fact.offset(6 as c_int as isize) as c_long
            * (sf.im + sl.im) as c_long
            + 0x40000000 as c_int as c_long
            >> 31 as c_int) as TXSample;
        tmp[2 as c_int as usize] = (*fact.offset(7 as c_int as isize) as c_long
            * (sf.re - sl.re) as c_long
            + 0x40000000 as c_int as c_long
            >> 31 as c_int) as TXSample;
        if AV_TX_REAL_TO_REAL as c_int == AV_TX_REAL_TO_REAL as c_int {
            tmp[3 as c_int as usize] = tmp[1 as c_int as usize] * *tcos.offset(len4 as isize)
                - tmp[2 as c_int as usize] * *tsin.offset(len4 as isize);
            tmp_mid = tmp[0 as c_int as usize] - tmp[3 as c_int as usize];
        } else {
            tmp[3 as c_int as usize] = tmp[1 as c_int as usize] * *tsin.offset(len4 as isize)
                + tmp[2 as c_int as usize] * *tcos.offset(len4 as isize);
            tmp_mid = tmp[0 as c_int as usize] + tmp[3 as c_int as usize];
        }
    }
    let mut i: c_int = 1 as c_int;
    while i <= len4 {
        let mut tmp_0: [TXSample; 4] = [0; 4];
        let sf_0: TXComplex = *data.offset(i as isize);
        let sl_0: TXComplex = *data.offset((len2 - i) as isize);
        if AV_TX_REAL_TO_REAL as c_int == AV_TX_REAL_TO_REAL as c_int {
            tmp_0[0 as c_int as usize] = (*fact.offset(4 as c_int as isize) as c_long
                * (sf_0.re + sl_0.re) as c_long
                + 0x40000000 as c_int as c_long
                >> 31 as c_int) as TXSample;
        } else {
            tmp_0[0 as c_int as usize] = (*fact.offset(5 as c_int as isize) as c_long
                * (sf_0.im - sl_0.im) as c_long
                + 0x40000000 as c_int as c_long
                >> 31 as c_int) as TXSample;
        }
        tmp_0[1 as c_int as usize] = (*fact.offset(6 as c_int as isize) as c_long
            * (sf_0.im + sl_0.im) as c_long
            + 0x40000000 as c_int as c_long
            >> 31 as c_int) as TXSample;
        tmp_0[2 as c_int as usize] = (*fact.offset(7 as c_int as isize) as c_long
            * (sf_0.re - sl_0.re) as c_long
            + 0x40000000 as c_int as c_long
            >> 31 as c_int) as TXSample;
        if AV_TX_REAL_TO_REAL as c_int == AV_TX_REAL_TO_REAL as c_int {
            tmp_0[3 as c_int as usize] = tmp_0[1 as c_int as usize] * *tcos.offset(i as isize)
                - tmp_0[2 as c_int as usize] * *tsin.offset(i as isize);
            *out.offset(i as isize) = tmp_0[0 as c_int as usize] + tmp_0[3 as c_int as usize];
            *out.offset((len - i) as isize) =
                tmp_0[0 as c_int as usize] - tmp_0[3 as c_int as usize];
        } else {
            tmp_0[3 as c_int as usize] = tmp_0[1 as c_int as usize] * *tsin.offset(i as isize)
                + tmp_0[2 as c_int as usize] * *tcos.offset(i as isize);
            *out.offset((i - 1 as c_int) as isize) =
                tmp_0[3 as c_int as usize] - tmp_0[0 as c_int as usize];
            *out.offset((len - i - 1 as c_int) as isize) =
                tmp_0[0 as c_int as usize] + tmp_0[3 as c_int as usize];
        }
        i += 1;
        i;
    }
    let mut i_0: c_int = 1 as c_int;
    while i_0 < len4 + (AV_TX_REAL_TO_REAL as c_int == AV_TX_REAL_TO_IMAGINARY as c_int) as c_int {
        *out.offset((len2 - i_0) as isize) = *out.offset((len - i_0) as isize);
        i_0 += 1;
        i_0;
    }
    if AV_TX_REAL_TO_REAL as c_int == AV_TX_REAL_TO_REAL as c_int {
        *out.offset(len2 as isize) = tmp_dc;
        *out.offset((len4 + 1 as c_int) as isize) = tmp_mid * *fact.offset(5 as c_int as isize);
    } else {
        *out.offset(len4 as isize) = tmp_mid;
    };
}
static mut ff_tx_rdft_r2r_mod2_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"rdft_r2r_mod2_int32_c".as_ptr(),
            function: Some(
                ff_tx_rdft_r2r_mod2_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_RDFT,
            flags: ((AV_TX_UNALIGNED as c_int
                | AV_TX_INPLACE as c_int
                | AV_TX_REAL_TO_REAL as c_int) as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | (1 as c_ulonglong) << 59 as c_int) as c_ulong,
            factors: [
                2 as c_int + 2 as c_int * (1 as c_int == 0) as c_int,
                -(1 as c_int),
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
            nb_factors: 2 as c_int,
            min_len: 2 as c_int + 2 as c_int * (1 as c_int == 0) as c_int,
            max_len: -(1 as c_int),
            init: Some(
                ff_tx_rdft_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
static mut ff_tx_rdft_r2i_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"rdft_r2i_int32_c".as_ptr(),
            function: Some(
                ff_tx_rdft_r2i_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_RDFT,
            flags: ((AV_TX_UNALIGNED as c_int
                | AV_TX_INPLACE as c_int
                | AV_TX_REAL_TO_IMAGINARY as c_int) as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | (1 as c_ulonglong) << 59 as c_int) as c_ulong,
            factors: [
                2 as c_int + 2 as c_int * (0 as c_int == 0) as c_int,
                -(1 as c_int),
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
            nb_factors: 2 as c_int,
            min_len: 2 as c_int + 2 as c_int * (0 as c_int == 0) as c_int,
            max_len: -(1 as c_int),
            init: Some(
                ff_tx_rdft_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
unsafe extern "C" fn ff_tx_rdft_r2i_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    _stride: ptrdiff_t,
) {
    let len: c_int = (*s).len;
    let len2: c_int = len >> 1 as c_int;
    let len4: c_int = len >> 2 as c_int;
    let aligned_len4: c_int =
        (len + 4 as c_int - 1 as c_int & !(4 as c_int - 1 as c_int)) / 4 as c_int;
    let fact: *const TXSample = (*s).exp.int32 as *mut c_void as *const TXSample;
    let tcos: *const TXSample = fact.offset(8 as c_int as isize);
    let tsin: *const TXSample = tcos.offset(aligned_len4 as isize);
    let data: *mut TXComplex = _dst as *mut TXComplex;
    let out: *mut TXSample = _dst as *mut TXSample;
    let mut tmp_dc: TXSample = 0;
    let mut tmp_mid: TXSample = 0;
    let mut tmp: [TXSample; 4] = [0; 4];
    let mut sf: TXComplex = TXComplex { re: 0, im: 0 };
    let mut sl: TXComplex = TXComplex { re: 0, im: 0 };
    ((*s).fn_0[0 as c_int as usize]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0 as c_int as isize),
        _dst,
        _src,
        size_of::<TXComplex>() as c_ulong as ptrdiff_t,
    );
    tmp_dc = (*data.offset(0 as c_int as isize)).re;
    (*data.offset(0 as c_int as isize)).re = tmp_dc + (*data.offset(0 as c_int as isize)).im;
    tmp_dc -= (*data.offset(0 as c_int as isize)).im;
    (*data.offset(0 as c_int as isize)).re = (*fact.offset(0 as c_int as isize) as c_long
        * (*data.offset(0 as c_int as isize)).re as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    tmp_dc = (*fact.offset(1 as c_int as isize) as c_long * tmp_dc as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as TXSample;
    (*data.offset(len4 as isize)).re = (*fact.offset(2 as c_int as isize) as c_long
        * (*data.offset(len4 as isize)).re as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    if 0 as c_int == 0 {
        (*data.offset(len4 as isize)).im = (*fact.offset(3 as c_int as isize) as c_long
            * (*data.offset(len4 as isize)).im as c_long
            + 0x40000000 as c_int as c_long
            >> 31 as c_int) as c_int;
    } else {
        sf = *data.offset(len4 as isize);
        sl = *data.offset((len4 + 1 as c_int) as isize);
        if AV_TX_REAL_TO_IMAGINARY as c_int == AV_TX_REAL_TO_REAL as c_int {
            tmp[0 as c_int as usize] = (*fact.offset(4 as c_int as isize) as c_long
                * (sf.re + sl.re) as c_long
                + 0x40000000 as c_int as c_long
                >> 31 as c_int) as TXSample;
        } else {
            tmp[0 as c_int as usize] = (*fact.offset(5 as c_int as isize) as c_long
                * (sf.im - sl.im) as c_long
                + 0x40000000 as c_int as c_long
                >> 31 as c_int) as TXSample;
        }
        tmp[1 as c_int as usize] = (*fact.offset(6 as c_int as isize) as c_long
            * (sf.im + sl.im) as c_long
            + 0x40000000 as c_int as c_long
            >> 31 as c_int) as TXSample;
        tmp[2 as c_int as usize] = (*fact.offset(7 as c_int as isize) as c_long
            * (sf.re - sl.re) as c_long
            + 0x40000000 as c_int as c_long
            >> 31 as c_int) as TXSample;
        if AV_TX_REAL_TO_IMAGINARY as c_int == AV_TX_REAL_TO_REAL as c_int {
            tmp[3 as c_int as usize] = tmp[1 as c_int as usize] * *tcos.offset(len4 as isize)
                - tmp[2 as c_int as usize] * *tsin.offset(len4 as isize);
            tmp_mid = tmp[0 as c_int as usize] - tmp[3 as c_int as usize];
        } else {
            tmp[3 as c_int as usize] = tmp[1 as c_int as usize] * *tsin.offset(len4 as isize)
                + tmp[2 as c_int as usize] * *tcos.offset(len4 as isize);
            tmp_mid = tmp[0 as c_int as usize] + tmp[3 as c_int as usize];
        }
    }
    let mut i: c_int = 1 as c_int;
    while i <= len4 {
        let mut tmp_0: [TXSample; 4] = [0; 4];
        let sf_0: TXComplex = *data.offset(i as isize);
        let sl_0: TXComplex = *data.offset((len2 - i) as isize);
        if AV_TX_REAL_TO_IMAGINARY as c_int == AV_TX_REAL_TO_REAL as c_int {
            tmp_0[0 as c_int as usize] = (*fact.offset(4 as c_int as isize) as c_long
                * (sf_0.re + sl_0.re) as c_long
                + 0x40000000 as c_int as c_long
                >> 31 as c_int) as TXSample;
        } else {
            tmp_0[0 as c_int as usize] = (*fact.offset(5 as c_int as isize) as c_long
                * (sf_0.im - sl_0.im) as c_long
                + 0x40000000 as c_int as c_long
                >> 31 as c_int) as TXSample;
        }
        tmp_0[1 as c_int as usize] = (*fact.offset(6 as c_int as isize) as c_long
            * (sf_0.im + sl_0.im) as c_long
            + 0x40000000 as c_int as c_long
            >> 31 as c_int) as TXSample;
        tmp_0[2 as c_int as usize] = (*fact.offset(7 as c_int as isize) as c_long
            * (sf_0.re - sl_0.re) as c_long
            + 0x40000000 as c_int as c_long
            >> 31 as c_int) as TXSample;
        if AV_TX_REAL_TO_IMAGINARY as c_int == AV_TX_REAL_TO_REAL as c_int {
            tmp_0[3 as c_int as usize] = tmp_0[1 as c_int as usize] * *tcos.offset(i as isize)
                - tmp_0[2 as c_int as usize] * *tsin.offset(i as isize);
            *out.offset(i as isize) = tmp_0[0 as c_int as usize] + tmp_0[3 as c_int as usize];
            *out.offset((len - i) as isize) =
                tmp_0[0 as c_int as usize] - tmp_0[3 as c_int as usize];
        } else {
            tmp_0[3 as c_int as usize] = tmp_0[1 as c_int as usize] * *tsin.offset(i as isize)
                + tmp_0[2 as c_int as usize] * *tcos.offset(i as isize);
            *out.offset((i - 1 as c_int) as isize) =
                tmp_0[3 as c_int as usize] - tmp_0[0 as c_int as usize];
            *out.offset((len - i - 1 as c_int) as isize) =
                tmp_0[0 as c_int as usize] + tmp_0[3 as c_int as usize];
        }
        i += 1;
        i;
    }
    let mut i_0: c_int = 1 as c_int;
    while i_0
        < len4 + (AV_TX_REAL_TO_IMAGINARY as c_int == AV_TX_REAL_TO_IMAGINARY as c_int) as c_int
    {
        *out.offset((len2 - i_0) as isize) = *out.offset((len - i_0) as isize);
        i_0 += 1;
        i_0;
    }
    if AV_TX_REAL_TO_IMAGINARY as c_int == AV_TX_REAL_TO_REAL as c_int {
        *out.offset(len2 as isize) = tmp_dc;
    }
}
static mut ff_tx_rdft_r2i_mod2_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"rdft_r2i_mod2_int32_c".as_ptr(),
            function: Some(
                ff_tx_rdft_r2i_mod2_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_RDFT,
            flags: ((AV_TX_UNALIGNED as c_int
                | AV_TX_INPLACE as c_int
                | AV_TX_REAL_TO_IMAGINARY as c_int) as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | (1 as c_ulonglong) << 59 as c_int) as c_ulong,
            factors: [
                2 as c_int + 2 as c_int * (1 as c_int == 0) as c_int,
                -(1 as c_int),
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
            nb_factors: 2 as c_int,
            min_len: 2 as c_int + 2 as c_int * (1 as c_int == 0) as c_int,
            max_len: -(1 as c_int),
            init: Some(
                ff_tx_rdft_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
unsafe extern "C" fn ff_tx_rdft_r2i_mod2_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    _stride: ptrdiff_t,
) {
    let len: c_int = (*s).len;
    let len2: c_int = len >> 1 as c_int;
    let len4: c_int = len >> 2 as c_int;
    let aligned_len4: c_int =
        (len + 4 as c_int - 1 as c_int & !(4 as c_int - 1 as c_int)) / 4 as c_int;
    let fact: *const TXSample = (*s).exp.int32 as *mut c_void as *const TXSample;
    let tcos: *const TXSample = fact.offset(8 as c_int as isize);
    let tsin: *const TXSample = tcos.offset(aligned_len4 as isize);
    let data: *mut TXComplex = _dst as *mut TXComplex;
    let out: *mut TXSample = _dst as *mut TXSample;
    let mut tmp_dc: TXSample = 0;
    let mut tmp_mid: TXSample = 0;
    let mut tmp: [TXSample; 4] = [0; 4];
    let mut sf: TXComplex = TXComplex { re: 0, im: 0 };
    let mut sl: TXComplex = TXComplex { re: 0, im: 0 };
    ((*s).fn_0[0 as c_int as usize]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0 as c_int as isize),
        _dst,
        _src,
        size_of::<TXComplex>() as c_ulong as ptrdiff_t,
    );
    tmp_dc = (*data.offset(0 as c_int as isize)).re;
    (*data.offset(0 as c_int as isize)).re = tmp_dc + (*data.offset(0 as c_int as isize)).im;
    tmp_dc -= (*data.offset(0 as c_int as isize)).im;
    (*data.offset(0 as c_int as isize)).re = (*fact.offset(0 as c_int as isize) as c_long
        * (*data.offset(0 as c_int as isize)).re as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    tmp_dc = (*fact.offset(1 as c_int as isize) as c_long * tmp_dc as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as TXSample;
    (*data.offset(len4 as isize)).re = (*fact.offset(2 as c_int as isize) as c_long
        * (*data.offset(len4 as isize)).re as c_long
        + 0x40000000 as c_int as c_long
        >> 31 as c_int) as c_int;
    if 1 as c_int == 0 {
        (*data.offset(len4 as isize)).im = (*fact.offset(3 as c_int as isize) as c_long
            * (*data.offset(len4 as isize)).im as c_long
            + 0x40000000 as c_int as c_long
            >> 31 as c_int) as c_int;
    } else {
        sf = *data.offset(len4 as isize);
        sl = *data.offset((len4 + 1 as c_int) as isize);
        if AV_TX_REAL_TO_IMAGINARY as c_int == AV_TX_REAL_TO_REAL as c_int {
            tmp[0 as c_int as usize] = (*fact.offset(4 as c_int as isize) as c_long
                * (sf.re + sl.re) as c_long
                + 0x40000000 as c_int as c_long
                >> 31 as c_int) as TXSample;
        } else {
            tmp[0 as c_int as usize] = (*fact.offset(5 as c_int as isize) as c_long
                * (sf.im - sl.im) as c_long
                + 0x40000000 as c_int as c_long
                >> 31 as c_int) as TXSample;
        }
        tmp[1 as c_int as usize] = (*fact.offset(6 as c_int as isize) as c_long
            * (sf.im + sl.im) as c_long
            + 0x40000000 as c_int as c_long
            >> 31 as c_int) as TXSample;
        tmp[2 as c_int as usize] = (*fact.offset(7 as c_int as isize) as c_long
            * (sf.re - sl.re) as c_long
            + 0x40000000 as c_int as c_long
            >> 31 as c_int) as TXSample;
        if AV_TX_REAL_TO_IMAGINARY as c_int == AV_TX_REAL_TO_REAL as c_int {
            tmp[3 as c_int as usize] = tmp[1 as c_int as usize] * *tcos.offset(len4 as isize)
                - tmp[2 as c_int as usize] * *tsin.offset(len4 as isize);
            tmp_mid = tmp[0 as c_int as usize] - tmp[3 as c_int as usize];
        } else {
            tmp[3 as c_int as usize] = tmp[1 as c_int as usize] * *tsin.offset(len4 as isize)
                + tmp[2 as c_int as usize] * *tcos.offset(len4 as isize);
            tmp_mid = tmp[0 as c_int as usize] + tmp[3 as c_int as usize];
        }
    }
    let mut i: c_int = 1 as c_int;
    while i <= len4 {
        let mut tmp_0: [TXSample; 4] = [0; 4];
        let sf_0: TXComplex = *data.offset(i as isize);
        let sl_0: TXComplex = *data.offset((len2 - i) as isize);
        if AV_TX_REAL_TO_IMAGINARY as c_int == AV_TX_REAL_TO_REAL as c_int {
            tmp_0[0 as c_int as usize] = (*fact.offset(4 as c_int as isize) as c_long
                * (sf_0.re + sl_0.re) as c_long
                + 0x40000000 as c_int as c_long
                >> 31 as c_int) as TXSample;
        } else {
            tmp_0[0 as c_int as usize] = (*fact.offset(5 as c_int as isize) as c_long
                * (sf_0.im - sl_0.im) as c_long
                + 0x40000000 as c_int as c_long
                >> 31 as c_int) as TXSample;
        }
        tmp_0[1 as c_int as usize] = (*fact.offset(6 as c_int as isize) as c_long
            * (sf_0.im + sl_0.im) as c_long
            + 0x40000000 as c_int as c_long
            >> 31 as c_int) as TXSample;
        tmp_0[2 as c_int as usize] = (*fact.offset(7 as c_int as isize) as c_long
            * (sf_0.re - sl_0.re) as c_long
            + 0x40000000 as c_int as c_long
            >> 31 as c_int) as TXSample;
        if AV_TX_REAL_TO_IMAGINARY as c_int == AV_TX_REAL_TO_REAL as c_int {
            tmp_0[3 as c_int as usize] = tmp_0[1 as c_int as usize] * *tcos.offset(i as isize)
                - tmp_0[2 as c_int as usize] * *tsin.offset(i as isize);
            *out.offset(i as isize) = tmp_0[0 as c_int as usize] + tmp_0[3 as c_int as usize];
            *out.offset((len - i) as isize) =
                tmp_0[0 as c_int as usize] - tmp_0[3 as c_int as usize];
        } else {
            tmp_0[3 as c_int as usize] = tmp_0[1 as c_int as usize] * *tsin.offset(i as isize)
                + tmp_0[2 as c_int as usize] * *tcos.offset(i as isize);
            *out.offset((i - 1 as c_int) as isize) =
                tmp_0[3 as c_int as usize] - tmp_0[0 as c_int as usize];
            *out.offset((len - i - 1 as c_int) as isize) =
                tmp_0[0 as c_int as usize] + tmp_0[3 as c_int as usize];
        }
        i += 1;
        i;
    }
    let mut i_0: c_int = 1 as c_int;
    while i_0
        < len4 + (AV_TX_REAL_TO_IMAGINARY as c_int == AV_TX_REAL_TO_IMAGINARY as c_int) as c_int
    {
        *out.offset((len2 - i_0) as isize) = *out.offset((len - i_0) as isize);
        i_0 += 1;
        i_0;
    }
    if AV_TX_REAL_TO_IMAGINARY as c_int == AV_TX_REAL_TO_REAL as c_int {
        *out.offset(len2 as isize) = tmp_dc;
        *out.offset((len4 + 1 as c_int) as isize) = tmp_mid * *fact.offset(5 as c_int as isize);
    } else {
        *out.offset(len4 as isize) = tmp_mid;
    };
}
#[cold]
unsafe extern "C" fn ff_tx_dct_init_int32_c(
    s: *mut AVTXContext,
    _cd: *const FFTXCodelet,
    flags: c_ulong,
    _opts: *mut FFTXCodeletOptions,
    mut len: c_int,
    inv: c_int,
    scale: *const c_void,
) -> c_int {
    let mut ret: c_int = 0;
    let mut freq: c_double = 0.;
    let mut tab: *mut TXSample = std::ptr::null_mut::<TXSample>();
    let mut rsc: c_float = *(scale as *mut c_float);
    if inv != 0 {
        len *= 2 as c_int;
        (*s).len *= 2 as c_int;
        rsc = (rsc as c_double * 0.5f64) as c_float;
    }
    ret = ff_tx_init_subtx(
        s,
        AV_TX_INT32_RDFT,
        flags,
        std::ptr::null_mut::<FFTXCodeletOptions>(),
        len,
        inv,
        &mut rsc as *mut c_float as *const c_void,
    );
    if ret != 0 {
        return ret;
    }
    (*s).exp = AVTXNum {
        int32: av_malloc(
            ((len / 2 as c_int * 3 as c_int) as c_ulong)
                .wrapping_mul(size_of::<TXSample>() as c_ulong),
        ) as *mut TXComplex,
    };
    if ((*s).exp).int32.is_null() {
        return -(12 as c_int);
    }
    tab = (*s).exp.int32 as *mut TXSample;
    freq = PI / (len * 2 as c_int) as c_double;
    let mut i: c_int = 0 as c_int;
    while i < len {
        *tab.offset(i as isize) = av_clip64_c(
            llrintf(
                (cos(i as c_double * freq)
                    * ((inv == 0) as c_int + 1 as c_int) as c_double
                    * 2147483648.0f64) as c_float,
            ) as c_long,
            (-(2147483647 as c_int) - 1 as c_int) as c_long,
            2147483647 as c_int as c_long,
        ) as TXSample;
        i += 1;
        i;
    }
    if inv != 0 {
        let mut i_0: c_int = 0 as c_int;
        while i_0 < len / 2 as c_int {
            *tab.offset((len + i_0) as isize) = av_clip64_c(
                llrintf(
                    (0.5f64 / sin((2 as c_int * i_0 + 1 as c_int) as c_double * freq)
                        * 2147483648.0f64) as c_float,
                ) as c_long,
                (-(2147483647 as c_int) - 1 as c_int) as c_long,
                2147483647 as c_int as c_long,
            ) as TXSample;
            i_0 += 1;
            i_0;
        }
    } else {
        let mut i_1: c_int = 0 as c_int;
        while i_1 < len / 2 as c_int {
            *tab.offset((len + i_1) as isize) = av_clip64_c(
                llrintf(
                    (cos((len - 2 as c_int * i_1 - 1 as c_int) as c_double * freq)
                        * 2147483648.0f64) as c_float,
                ) as c_long,
                (-(2147483647 as c_int) - 1 as c_int) as c_long,
                2147483647 as c_int as c_long,
            ) as TXSample;
            i_1 += 1;
            i_1;
        }
    }
    0 as c_int
}
unsafe extern "C" fn ff_tx_dctII_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    _stride: ptrdiff_t,
) {
    let dst: *mut TXSample = _dst as *mut TXSample;
    let src: *mut TXSample = _src as *mut TXSample;
    let len: c_int = (*s).len;
    let len2: c_int = len >> 1 as c_int;
    let exp: *const TXSample = (*s).exp.int32 as *mut c_void as *const TXSample;
    let mut next: TXSample = 0;
    let mut tmp1: c_long = 0;
    let mut tmp2: c_long = 0;
    let mut i: c_int = 0 as c_int;
    while i < len2 {
        let in1: TXSample = *src.offset(i as isize);
        let in2: TXSample = *src.offset((len - i - 1 as c_int) as isize);
        let s_0: TXSample = *exp.offset((len + i) as isize);
        tmp1 = (in1 + in2) as c_long;
        tmp2 = (in1 - in2) as c_long;
        tmp1 >>= 1 as c_int;
        tmp2 *= s_0 as c_long;
        tmp2 = tmp2 + 0x40000000 as c_int as c_long >> 31 as c_int;
        *src.offset(i as isize) = (tmp1 + tmp2) as TXSample;
        *src.offset((len - i - 1 as c_int) as isize) = (tmp1 - tmp2) as TXSample;
        i += 1;
        i;
    }
    ((*s).fn_0[0 as c_int as usize]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0 as c_int as isize),
        dst as *mut c_void,
        src as *mut c_void,
        size_of::<TXComplex>() as c_ulong as ptrdiff_t,
    );
    next = *dst.offset(len as isize);
    let mut i_0: c_int = len - 2 as c_int;
    while i_0 > 0 as c_int {
        let mut tmp: TXSample = 0;
        let mut accu: c_long = 0;
        accu = *dst.offset((i_0 + 0 as c_int) as isize) as c_long
            * *exp.offset((len - i_0) as isize) as c_long;
        accu -= *dst.offset((i_0 + 1 as c_int) as isize) as c_long
            * *exp.offset(i_0 as isize) as c_long;
        tmp = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        accu = *dst.offset((i_0 + 1 as c_int) as isize) as c_long
            * *exp.offset((len - i_0) as isize) as c_long;
        accu += *dst.offset((i_0 + 0 as c_int) as isize) as c_long
            * *exp.offset(i_0 as isize) as c_long;
        *dst.offset(i_0 as isize) = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        *dst.offset((i_0 + 1 as c_int) as isize) = next;
        next += tmp;
        i_0 -= 2 as c_int;
    }
    tmp1 = *exp.offset(0 as c_int as isize) as c_long * *dst.offset(0 as c_int as isize) as c_long;
    *dst.offset(0 as c_int as isize) =
        (tmp1 + 0x40000000 as c_int as c_long >> 31 as c_int) as TXSample;
    *dst.offset(1 as c_int as isize) = next;
}
unsafe extern "C" fn ff_tx_dctIII_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    _stride: ptrdiff_t,
) {
    let dst: *mut TXSample = _dst as *mut TXSample;
    let src: *mut TXSample = _src as *mut TXSample;
    let len: c_int = (*s).len;
    let len2: c_int = len >> 1 as c_int;
    let exp: *const TXSample = (*s).exp.int32 as *mut c_void as *const TXSample;
    let mut tmp1: c_long = 0;
    let mut tmp2: c_long = *src.offset((len - 1 as c_int) as isize) as c_long;
    tmp2 = 2 as c_int as c_long * tmp2 + 0x40000000 as c_int as c_long >> 31 as c_int;
    *src.offset(len as isize) = tmp2 as TXSample;
    let mut i: c_int = len - 2 as c_int;
    while i >= 2 as c_int {
        let val1: TXSample = *src.offset((i - 0 as c_int) as isize);
        let val2: TXSample =
            *src.offset((i - 1 as c_int) as isize) - *src.offset((i + 1 as c_int) as isize);
        let mut accu: c_long = 0;
        accu = val1 as c_long * *exp.offset((len - i) as isize) as c_long;
        accu -= val2 as c_long * *exp.offset(i as isize) as c_long;
        *src.offset((i + 1 as c_int) as isize) =
            (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        accu = val2 as c_long * *exp.offset((len - i) as isize) as c_long;
        accu += val1 as c_long * *exp.offset(i as isize) as c_long;
        *src.offset(i as isize) = (accu + 0x40000000 as c_int as c_long >> 31 as c_int) as c_int;
        i -= 2 as c_int;
    }
    ((*s).fn_0[0 as c_int as usize]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0 as c_int as isize),
        dst as *mut c_void,
        src as *mut c_void,
        size_of::<c_float>() as c_ulong as ptrdiff_t,
    );
    let mut i_0: c_int = 0 as c_int;
    while i_0 < len2 {
        let in1: TXSample = *dst.offset(i_0 as isize);
        let in2: TXSample = *dst.offset((len - i_0 - 1 as c_int) as isize);
        let c: TXSample = *exp.offset((len + i_0) as isize);
        tmp1 = (in1 + in2) as c_long;
        tmp2 = (in1 - in2) as c_long;
        tmp2 *= c as c_long;
        tmp2 = tmp2 + 0x40000000 as c_int as c_long >> 31 as c_int;
        *dst.offset(i_0 as isize) = (tmp1 + tmp2) as TXSample;
        *dst.offset((len - i_0 - 1 as c_int) as isize) = (tmp1 - tmp2) as TXSample;
        i_0 += 1;
        i_0;
    }
}
static mut ff_tx_dctII_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"dctII_int32_c".as_ptr(),
            function: Some(
                ff_tx_dctII_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_DCT,
            flags: ((AV_TX_UNALIGNED as c_int | AV_TX_INPLACE as c_int) as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | (1 as c_ulonglong) << 59 as c_int) as c_ulong,
            factors: [
                2 as c_int,
                -(1 as c_int),
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
            nb_factors: 0,
            min_len: 2 as c_int,
            max_len: -(1 as c_int),
            init: Some(
                ff_tx_dct_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
static mut ff_tx_dctIII_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"dctIII_int32_c".as_ptr(),
            function: Some(
                ff_tx_dctIII_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_DCT,
            flags: ((AV_TX_UNALIGNED as c_int | AV_TX_INPLACE as c_int) as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | (1 as c_ulonglong) << 60 as c_int) as c_ulong,
            factors: [
                2 as c_int,
                -(1 as c_int),
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
            nb_factors: 0,
            min_len: 2 as c_int,
            max_len: -(1 as c_int),
            init: Some(
                ff_tx_dct_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
#[cold]
unsafe extern "C" fn ff_tx_dcstI_init_int32_c(
    s: *mut AVTXContext,
    cd: *const FFTXCodelet,
    mut flags: c_ulong,
    _opts: *mut FFTXCodeletOptions,
    mut len: c_int,
    inv: c_int,
    scale: *const c_void,
) -> c_int {
    let mut ret: c_int = 0;
    let mut rsc: c_float = *(scale as *mut c_float);
    if inv != 0 {
        len *= 2 as c_int;
        (*s).len *= 2 as c_int;
        rsc = (rsc as c_double * 0.5f64) as c_float;
    }
    flags |= (if (*cd).type_0 as c_uint == AV_TX_INT32_DCT_I as c_int as c_uint {
        AV_TX_REAL_TO_REAL as c_int
    } else {
        AV_TX_REAL_TO_IMAGINARY as c_int
    }) as c_ulong;
    ret = ff_tx_init_subtx(
        s,
        AV_TX_INT32_RDFT,
        flags,
        std::ptr::null_mut::<FFTXCodeletOptions>(),
        (len - 1 as c_int
            + 2 as c_int
                * ((*cd).type_0 as c_uint == AV_TX_INT32_DST_I as c_int as c_uint) as c_int)
            * 2 as c_int,
        0 as c_int,
        &mut rsc as *mut c_float as *const c_void,
    );
    if ret != 0 {
        return ret;
    }
    (*s).tmp = AVTXNum {
        int32: av_mallocz(
            (((len + 1 as c_int) * 2 as c_int) as c_ulong)
                .wrapping_mul(size_of::<TXSample>() as c_ulong),
        ) as *mut TXComplex,
    };
    if ((*s).tmp).int32.is_null() {
        return -(12 as c_int);
    }
    0 as c_int
}
unsafe extern "C" fn ff_tx_dctI_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let dst: *mut TXSample = _dst as *mut TXSample;
    let src: *mut TXSample = _src as *mut TXSample;
    let len: c_int = (*s).len - 1 as c_int;
    let tmp: *mut TXSample = (*s).tmp.int32 as *mut TXSample;
    stride = (stride as c_ulong).wrapping_div(size_of::<TXSample>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: c_int = 0 as c_int;
    while i < len {
        let fresh40 = &mut (*tmp.offset((2 as c_int * len - i) as isize));
        *fresh40 = *src.offset((i as c_long * stride) as isize);
        *tmp.offset(i as isize) = *fresh40;
        i += 1;
        i;
    }
    *tmp.offset(len as isize) = *src.offset((len as c_long * stride) as isize);
    ((*s).fn_0[0 as c_int as usize]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0 as c_int as isize),
        dst as *mut c_void,
        tmp as *mut c_void,
        size_of::<TXSample>() as c_ulong as ptrdiff_t,
    );
}
unsafe extern "C" fn ff_tx_dstI_int32_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let dst: *mut TXSample = _dst as *mut TXSample;
    let src: *mut TXSample = _src as *mut TXSample;
    let len: c_int = (*s).len + 1 as c_int;
    let tmp: *mut TXSample = (*s).tmp.int32 as *mut c_void as *mut TXSample;
    stride = (stride as c_ulong).wrapping_div(size_of::<TXSample>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    *tmp.offset(0 as c_int as isize) = 0 as c_int;
    let mut i: c_int = 1 as c_int;
    while i < len {
        let a: TXSample = *src.offset(((i - 1 as c_int) as c_long * stride) as isize);
        *tmp.offset(i as isize) = -a;
        *tmp.offset((2 as c_int * len - i) as isize) = a;
        i += 1;
        i;
    }
    *tmp.offset(len as isize) = 0 as c_int;
    ((*s).fn_0[0 as c_int as usize]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0 as c_int as isize),
        dst as *mut c_void,
        tmp as *mut c_void,
        size_of::<c_float>() as c_ulong as ptrdiff_t,
    );
}
static mut ff_tx_dctI_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"dctI_int32_c".as_ptr(),
            function: Some(
                ff_tx_dctI_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_DCT_I,
            flags: ((AV_TX_UNALIGNED as c_int | AV_TX_INPLACE as c_int) as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int) as c_ulong,
            factors: [
                2 as c_int,
                -(1 as c_int),
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
            nb_factors: 2 as c_int,
            min_len: 2 as c_int,
            max_len: -(1 as c_int),
            init: Some(
                ff_tx_dcstI_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};
static mut ff_tx_dstI_def_int32_c: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: c"dstI_int32_c".as_ptr(),
            function: Some(
                ff_tx_dstI_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_DST_I,
            flags: ((AV_TX_UNALIGNED as c_int | AV_TX_INPLACE as c_int) as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int) as c_ulong,
            factors: [
                2 as c_int,
                -(1 as c_int),
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
            nb_factors: 2 as c_int,
            min_len: 2 as c_int,
            max_len: -(1 as c_int),
            init: Some(
                ff_tx_dcstI_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    }
};

pub unsafe extern "C" fn ff_tx_mdct_gen_exp_int32(
    s: *mut AVTXContext,
    pre_tab: *mut c_int,
) -> c_int {
    let mut off: c_int = 0 as c_int;
    let len4: c_int = (*s).len >> 1 as c_int;
    let mut scale: c_double = (*s).scale_d;
    let theta: c_double =
        (if scale < 0. { len4 } else { 0 as c_int }) as c_double + 1.0f64 / 8.0f64;
    let alloc: c_ulong = (if !pre_tab.is_null() {
        2 as c_int * len4
    } else {
        len4
    }) as c_ulong;
    (*s).exp = AVTXNum {
        int32: av_malloc_array(alloc, size_of::<TXComplex>() as c_ulong) as *mut TXComplex,
    };
    if ((*s).exp).int32.is_null() {
        return -(12 as c_int);
    }
    scale = sqrt(fabs(scale));
    if !pre_tab.is_null() {
        off = len4;
    }
    let mut i: c_int = 0 as c_int;
    while i < len4 {
        let alpha: c_double = FRAC_PI_2 * (i as c_double + theta) / len4 as c_double;
        *((*s).exp).int32.offset((off + i) as isize) = {
            AVComplexInt32 {
                re: av_clip64_c(
                    llrintf((cos(alpha) * scale * 2147483648.0f64) as c_float) as c_long,
                    (-(2147483647 as c_int) - 1 as c_int) as c_long,
                    2147483647 as c_int as c_long,
                ) as c_int,
                im: av_clip64_c(
                    llrintf((sin(alpha) * scale * 2147483648.0f64) as c_float) as c_long,
                    (-(2147483647 as c_int) - 1 as c_int) as c_long,
                    2147483647 as c_int as c_long,
                ) as c_int,
            }
        };
        i += 1;
        i;
    }
    if !pre_tab.is_null() {
        let mut i_0: c_int = 0 as c_int;
        while i_0 < len4 {
            *((*s).exp).int32.offset(i_0 as isize) = *((*s).exp)
                .int32
                .offset((len4 + *pre_tab.offset(i_0 as isize)) as isize);
            i_0 += 1;
            i_0;
        }
    }
    0 as c_int
}

pub static mut ff_tx_codelet_list_int32_c: [*const FFTXCodelet; 63] = unsafe {
    [
        &ff_tx_fft2_ns_def_int32_c,
        &ff_tx_fft4_ns_def_int32_c,
        &ff_tx_fft8_ns_def_int32_c,
        &ff_tx_fft16_ns_def_int32_c,
        &ff_tx_fft32_ns_def_int32_c,
        &ff_tx_fft64_ns_def_int32_c,
        &ff_tx_fft128_ns_def_int32_c,
        &ff_tx_fft256_ns_def_int32_c,
        &ff_tx_fft512_ns_def_int32_c,
        &ff_tx_fft1024_ns_def_int32_c,
        &ff_tx_fft2048_ns_def_int32_c,
        &ff_tx_fft4096_ns_def_int32_c,
        &ff_tx_fft8192_ns_def_int32_c,
        &ff_tx_fft16384_ns_def_int32_c,
        &ff_tx_fft32768_ns_def_int32_c,
        &ff_tx_fft65536_ns_def_int32_c,
        &ff_tx_fft131072_ns_def_int32_c,
        &ff_tx_fft262144_ns_def_int32_c,
        &ff_tx_fft524288_ns_def_int32_c,
        &ff_tx_fft1048576_ns_def_int32_c,
        &ff_tx_fft2097152_ns_def_int32_c,
        &ff_tx_fft3_ns_def_int32_c,
        &ff_tx_fft5_ns_def_int32_c,
        &ff_tx_fft7_ns_def_int32_c,
        &ff_tx_fft9_ns_def_int32_c,
        &ff_tx_fft15_ns_def_int32_c,
        &ff_tx_fft3_fwd_def_int32_c,
        &ff_tx_fft5_fwd_def_int32_c,
        &ff_tx_fft7_fwd_def_int32_c,
        &ff_tx_fft9_fwd_def_int32_c,
        &ff_tx_fft_def_int32_c,
        &ff_tx_fft_inplace_def_int32_c,
        &ff_tx_fft_inplace_small_def_int32_c,
        &ff_tx_fft_pfa_def_int32_c,
        &ff_tx_fft_pfa_ns_def_int32_c,
        &ff_tx_fft_naive_def_int32_c,
        &ff_tx_fft_naive_small_def_int32_c,
        &ff_tx_mdct_fwd_def_int32_c,
        &ff_tx_mdct_inv_def_int32_c,
        &ff_tx_mdct_pfa_3xM_fwd_def_int32_c,
        &ff_tx_mdct_pfa_5xM_fwd_def_int32_c,
        &ff_tx_mdct_pfa_7xM_fwd_def_int32_c,
        &ff_tx_mdct_pfa_9xM_fwd_def_int32_c,
        &ff_tx_mdct_pfa_15xM_fwd_def_int32_c,
        &ff_tx_mdct_pfa_3xM_inv_def_int32_c,
        &ff_tx_mdct_pfa_5xM_inv_def_int32_c,
        &ff_tx_mdct_pfa_7xM_inv_def_int32_c,
        &ff_tx_mdct_pfa_9xM_inv_def_int32_c,
        &ff_tx_mdct_pfa_15xM_inv_def_int32_c,
        &ff_tx_mdct_naive_fwd_def_int32_c,
        &ff_tx_mdct_naive_inv_def_int32_c,
        &ff_tx_mdct_inv_full_def_int32_c,
        &ff_tx_rdft_r2c_def_int32_c,
        &ff_tx_rdft_r2r_def_int32_c,
        &ff_tx_rdft_r2r_mod2_def_int32_c,
        &ff_tx_rdft_r2i_def_int32_c,
        &ff_tx_rdft_r2i_mod2_def_int32_c,
        &ff_tx_rdft_c2r_def_int32_c,
        &ff_tx_dctII_def_int32_c,
        &ff_tx_dctIII_def_int32_c,
        &ff_tx_dctI_def_int32_c,
        &ff_tx_dstI_def_int32_c,
        ptr::null(),
    ]
};
unsafe extern "C" fn run_static_initializers() {
    ff_tx_rdft_r2c_def_int32_c = {
        FFTXCodelet {
            name: c"rdft_r2c_int32_c".as_ptr(),
            function: Some(
                ff_tx_rdft_r2c_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_RDFT,
            flags: ((AV_TX_UNALIGNED as c_int | AV_TX_INPLACE as c_int) as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | (if 0 as c_int != 0 {
                    (1 as c_ulonglong) << 60 as c_int
                } else {
                    (1 as c_ulonglong) << 59 as c_int
                })) as c_ulong,
            factors: [4, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 2,
            min_len: 4,
            max_len: -1,
            init: Some(
                ff_tx_rdft_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    };
    ff_tx_rdft_c2r_def_int32_c = {
        FFTXCodelet {
            name: c"rdft_c2r_int32_c".as_ptr(),
            function: Some(
                ff_tx_rdft_c2r_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_INT32_RDFT,
            flags: ((AV_TX_UNALIGNED as c_int | AV_TX_INPLACE as c_int) as c_ulonglong
                | (1 as c_ulonglong) << 63 as c_int
                | (if 1 as c_int != 0 {
                    (1 as c_ulonglong) << 60 as c_int
                } else {
                    (1 as c_ulonglong) << 59 as c_int
                })) as c_ulong,
            factors: [
                4 as c_int,
                -(1 as c_int),
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
            nb_factors: 2 as c_int,
            min_len: 4 as c_int,
            max_len: -(1 as c_int),
            init: Some(
                ff_tx_rdft_init_int32_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        c_ulong,
                        *mut FFTXCodeletOptions,
                        c_int,
                        c_int,
                        *const c_void,
                    ) -> c_int,
            ),
            uninit: None,
            cpu_flags: 0 as c_int,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
