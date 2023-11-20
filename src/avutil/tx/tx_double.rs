use std::sync::Once;

use crate::common::*;
use crate::types::*;

use ::libc;
extern "C" {
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn av_malloc_array(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn av_mallocz(size: size_t) -> *mut libc::c_void;
    fn av_malloc(size: size_t) -> *mut libc::c_void;
    fn ff_tx_gen_inplace_map(s: *mut AVTXContext, len: libc::c_int) -> libc::c_int;
    fn ff_tx_gen_ptwo_revtab(s: *mut AVTXContext, opts: *mut FFTXCodeletOptions) -> libc::c_int;
    fn ff_tx_gen_compound_mapping(
        s: *mut AVTXContext,
        opts: *mut FFTXCodeletOptions,
        inv: libc::c_int,
        n: libc::c_int,
        m: libc::c_int,
    ) -> libc::c_int;
    fn ff_tx_gen_default_map(s: *mut AVTXContext, opts: *mut FFTXCodeletOptions) -> libc::c_int;
    fn ff_tx_decompose_length(
        dst: *mut libc::c_int,
        type_0: AVTXType,
        len: libc::c_int,
        inv: libc::c_int,
    ) -> libc::c_int;
    fn ff_tx_clear_ctx(s: *mut AVTXContext);
    fn ff_tx_init_subtx(
        s: *mut AVTXContext,
        type_0: AVTXType,
        flags: uint64_t,
        opts: *mut FFTXCodeletOptions,
        len: libc::c_int,
        inv: libc::c_int,
        scale: *const libc::c_void,
    ) -> libc::c_int;
    fn ff_tx_gen_pfa_input_map(
        s: *mut AVTXContext,
        opts: *mut FFTXCodeletOptions,
        d1: libc::c_int,
        d2: libc::c_int,
    ) -> libc::c_int;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type TXComplex = AVComplexDouble;
pub type TXSample = libc::c_double;
pub type TXUSample = libc::c_double;

#[inline(always)]
unsafe extern "C" fn ff_ctz_c(mut v: libc::c_int) -> libc::c_int {
    static mut debruijn_ctz32: [uint8_t; 32] = [
        0 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        28 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        29 as libc::c_int as uint8_t,
        14 as libc::c_int as uint8_t,
        24 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        30 as libc::c_int as uint8_t,
        22 as libc::c_int as uint8_t,
        20 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        25 as libc::c_int as uint8_t,
        17 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        31 as libc::c_int as uint8_t,
        27 as libc::c_int as uint8_t,
        13 as libc::c_int as uint8_t,
        23 as libc::c_int as uint8_t,
        21 as libc::c_int as uint8_t,
        19 as libc::c_int as uint8_t,
        16 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        26 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        18 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
    ];
    return debruijn_ctz32[(((v & -v) as libc::c_uint).wrapping_mul(0x77cb531 as libc::c_uint)
        >> 27 as libc::c_int) as usize] as libc::c_int;
}
#[no_mangle]
pub static mut ff_tx_tab_2048_double: [TXSample; 513] = [0.; 513];
#[no_mangle]
pub static mut ff_tx_tab_512_double: [TXSample; 129] = [0.; 129];
#[no_mangle]
pub static mut ff_tx_tab_65536_double: [TXSample; 16385] = [0.; 16385];
#[no_mangle]
pub static mut ff_tx_tab_524288_double: [TXSample; 131073] = [0.; 131073];
#[no_mangle]
pub static mut ff_tx_tab_32768_double: [TXSample; 8193] = [0.; 8193];
#[no_mangle]
pub static mut ff_tx_tab_64_double: [TXSample; 17] = [0.; 17];
#[no_mangle]
pub static mut ff_tx_tab_16384_double: [TXSample; 4097] = [0.; 4097];
#[no_mangle]
pub static mut ff_tx_tab_8_double: [TXSample; 3] = [0.; 3];
#[no_mangle]
pub static mut ff_tx_tab_8192_double: [TXSample; 2049] = [0.; 2049];
#[no_mangle]
pub static mut ff_tx_tab_128_double: [TXSample; 33] = [0.; 33];
#[no_mangle]
pub static mut ff_tx_tab_4096_double: [TXSample; 1025] = [0.; 1025];
#[no_mangle]
pub static mut ff_tx_tab_131072_double: [TXSample; 32769] = [0.; 32769];
#[no_mangle]
pub static mut ff_tx_tab_2097152_double: [TXSample; 524289] = [0.; 524289];
#[no_mangle]
pub static mut ff_tx_tab_1048576_double: [TXSample; 262145] = [0.; 262145];
#[no_mangle]
pub static mut ff_tx_tab_1024_double: [TXSample; 257] = [0.; 257];
#[no_mangle]
pub static mut ff_tx_tab_262144_double: [TXSample; 65537] = [0.; 65537];
#[no_mangle]
pub static mut ff_tx_tab_32_double: [TXSample; 9] = [0.; 9];
#[no_mangle]
pub static mut ff_tx_tab_16_double: [TXSample; 5] = [0.; 5];
#[no_mangle]
pub static mut ff_tx_tab_256_double: [TXSample; 65] = [0.; 65];
#[no_mangle]
pub static mut ff_tx_tab_53_double: [TXSample; 12] = [0.; 12];
#[no_mangle]
pub static mut ff_tx_tab_7_double: [TXSample; 6] = [0.; 6];
#[no_mangle]
pub static mut ff_tx_tab_9_double: [TXSample; 8] = [0.; 8];
#[cold]
unsafe extern "C" fn ff_tx_init_tab_262144_double() {
    let mut freq: libc::c_double = 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64
        / 262144 as libc::c_int as libc::c_double;
    let mut tab: *mut TXSample = ff_tx_tab_262144_double.as_mut_ptr();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 262144 as libc::c_int / 4 as libc::c_int {
        let fresh0 = tab;
        tab = tab.offset(1);
        *fresh0 = cos(i as libc::c_double * freq);
        i += 1;
        i;
    }
    *tab = 0 as libc::c_int as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_4096_double() {
    let mut freq: libc::c_double = 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64
        / 4096 as libc::c_int as libc::c_double;
    let mut tab: *mut TXSample = ff_tx_tab_4096_double.as_mut_ptr();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4096 as libc::c_int / 4 as libc::c_int {
        let fresh1 = tab;
        tab = tab.offset(1);
        *fresh1 = cos(i as libc::c_double * freq);
        i += 1;
        i;
    }
    *tab = 0 as libc::c_int as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_256_double() {
    let mut freq: libc::c_double = 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64
        / 256 as libc::c_int as libc::c_double;
    let mut tab: *mut TXSample = ff_tx_tab_256_double.as_mut_ptr();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 256 as libc::c_int / 4 as libc::c_int {
        let fresh2 = tab;
        tab = tab.offset(1);
        *fresh2 = cos(i as libc::c_double * freq);
        i += 1;
        i;
    }
    *tab = 0 as libc::c_int as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_128_double() {
    let mut freq: libc::c_double = 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64
        / 128 as libc::c_int as libc::c_double;
    let mut tab: *mut TXSample = ff_tx_tab_128_double.as_mut_ptr();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 128 as libc::c_int / 4 as libc::c_int {
        let fresh3 = tab;
        tab = tab.offset(1);
        *fresh3 = cos(i as libc::c_double * freq);
        i += 1;
        i;
    }
    *tab = 0 as libc::c_int as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_64_double() {
    let mut freq: libc::c_double = 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64
        / 64 as libc::c_int as libc::c_double;
    let mut tab: *mut TXSample = ff_tx_tab_64_double.as_mut_ptr();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 64 as libc::c_int / 4 as libc::c_int {
        let fresh4 = tab;
        tab = tab.offset(1);
        *fresh4 = cos(i as libc::c_double * freq);
        i += 1;
        i;
    }
    *tab = 0 as libc::c_int as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_32_double() {
    let mut freq: libc::c_double = 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64
        / 32 as libc::c_int as libc::c_double;
    let mut tab: *mut TXSample = ff_tx_tab_32_double.as_mut_ptr();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 32 as libc::c_int / 4 as libc::c_int {
        let fresh5 = tab;
        tab = tab.offset(1);
        *fresh5 = cos(i as libc::c_double * freq);
        i += 1;
        i;
    }
    *tab = 0 as libc::c_int as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_16_double() {
    let mut freq: libc::c_double = 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64
        / 16 as libc::c_int as libc::c_double;
    let mut tab: *mut TXSample = ff_tx_tab_16_double.as_mut_ptr();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 16 as libc::c_int / 4 as libc::c_int {
        let fresh6 = tab;
        tab = tab.offset(1);
        *fresh6 = cos(i as libc::c_double * freq);
        i += 1;
        i;
    }
    *tab = 0 as libc::c_int as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_8_double() {
    let mut freq: libc::c_double = 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64
        / 8 as libc::c_int as libc::c_double;
    let mut tab: *mut TXSample = ff_tx_tab_8_double.as_mut_ptr();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 8 as libc::c_int / 4 as libc::c_int {
        let fresh7 = tab;
        tab = tab.offset(1);
        *fresh7 = cos(i as libc::c_double * freq);
        i += 1;
        i;
    }
    *tab = 0 as libc::c_int as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_16384_double() {
    let mut freq: libc::c_double = 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64
        / 16384 as libc::c_int as libc::c_double;
    let mut tab: *mut TXSample = ff_tx_tab_16384_double.as_mut_ptr();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 16384 as libc::c_int / 4 as libc::c_int {
        let fresh8 = tab;
        tab = tab.offset(1);
        *fresh8 = cos(i as libc::c_double * freq);
        i += 1;
        i;
    }
    *tab = 0 as libc::c_int as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_8192_double() {
    let mut freq: libc::c_double = 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64
        / 8192 as libc::c_int as libc::c_double;
    let mut tab: *mut TXSample = ff_tx_tab_8192_double.as_mut_ptr();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 8192 as libc::c_int / 4 as libc::c_int {
        let fresh9 = tab;
        tab = tab.offset(1);
        *fresh9 = cos(i as libc::c_double * freq);
        i += 1;
        i;
    }
    *tab = 0 as libc::c_int as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_131072_double() {
    let mut freq: libc::c_double = 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64
        / 131072 as libc::c_int as libc::c_double;
    let mut tab: *mut TXSample = ff_tx_tab_131072_double.as_mut_ptr();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 131072 as libc::c_int / 4 as libc::c_int {
        let fresh10 = tab;
        tab = tab.offset(1);
        *fresh10 = cos(i as libc::c_double * freq);
        i += 1;
        i;
    }
    *tab = 0 as libc::c_int as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_65536_double() {
    let mut freq: libc::c_double = 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64
        / 65536 as libc::c_int as libc::c_double;
    let mut tab: *mut TXSample = ff_tx_tab_65536_double.as_mut_ptr();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 65536 as libc::c_int / 4 as libc::c_int {
        let fresh11 = tab;
        tab = tab.offset(1);
        *fresh11 = cos(i as libc::c_double * freq);
        i += 1;
        i;
    }
    *tab = 0 as libc::c_int as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_524288_double() {
    let mut freq: libc::c_double = 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64
        / 524288 as libc::c_int as libc::c_double;
    let mut tab: *mut TXSample = ff_tx_tab_524288_double.as_mut_ptr();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 524288 as libc::c_int / 4 as libc::c_int {
        let fresh12 = tab;
        tab = tab.offset(1);
        *fresh12 = cos(i as libc::c_double * freq);
        i += 1;
        i;
    }
    *tab = 0 as libc::c_int as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_32768_double() {
    let mut freq: libc::c_double = 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64
        / 32768 as libc::c_int as libc::c_double;
    let mut tab: *mut TXSample = ff_tx_tab_32768_double.as_mut_ptr();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 32768 as libc::c_int / 4 as libc::c_int {
        let fresh13 = tab;
        tab = tab.offset(1);
        *fresh13 = cos(i as libc::c_double * freq);
        i += 1;
        i;
    }
    *tab = 0 as libc::c_int as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_1048576_double() {
    let mut freq: libc::c_double = 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64
        / 1048576 as libc::c_int as libc::c_double;
    let mut tab: *mut TXSample = ff_tx_tab_1048576_double.as_mut_ptr();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 1048576 as libc::c_int / 4 as libc::c_int {
        let fresh14 = tab;
        tab = tab.offset(1);
        *fresh14 = cos(i as libc::c_double * freq);
        i += 1;
        i;
    }
    *tab = 0 as libc::c_int as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_1024_double() {
    let mut freq: libc::c_double = 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64
        / 1024 as libc::c_int as libc::c_double;
    let mut tab: *mut TXSample = ff_tx_tab_1024_double.as_mut_ptr();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 1024 as libc::c_int / 4 as libc::c_int {
        let fresh15 = tab;
        tab = tab.offset(1);
        *fresh15 = cos(i as libc::c_double * freq);
        i += 1;
        i;
    }
    *tab = 0 as libc::c_int as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_2097152_double() {
    let mut freq: libc::c_double = 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64
        / 2097152 as libc::c_int as libc::c_double;
    let mut tab: *mut TXSample = ff_tx_tab_2097152_double.as_mut_ptr();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 2097152 as libc::c_int / 4 as libc::c_int {
        let fresh16 = tab;
        tab = tab.offset(1);
        *fresh16 = cos(i as libc::c_double * freq);
        i += 1;
        i;
    }
    *tab = 0 as libc::c_int as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_512_double() {
    let mut freq: libc::c_double = 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64
        / 512 as libc::c_int as libc::c_double;
    let mut tab: *mut TXSample = ff_tx_tab_512_double.as_mut_ptr();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 512 as libc::c_int / 4 as libc::c_int {
        let fresh17 = tab;
        tab = tab.offset(1);
        *fresh17 = cos(i as libc::c_double * freq);
        i += 1;
        i;
    }
    *tab = 0 as libc::c_int as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_2048_double() {
    let mut freq: libc::c_double = 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64
        / 2048 as libc::c_int as libc::c_double;
    let mut tab: *mut TXSample = ff_tx_tab_2048_double.as_mut_ptr();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 2048 as libc::c_int / 4 as libc::c_int {
        let fresh18 = tab;
        tab = tab.offset(1);
        *fresh18 = cos(i as libc::c_double * freq);
        i += 1;
        i;
    }
    *tab = 0 as libc::c_int as TXSample;
}
static mut sr_tabs_init_funcs: [Option<unsafe extern "C" fn() -> ()>; 19] = unsafe {
    [
        Some(ff_tx_init_tab_8_double as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_16_double as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_32_double as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_64_double as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_128_double as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_256_double as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_512_double as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_1024_double as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_2048_double as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_4096_double as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_8192_double as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_16384_double as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_32768_double as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_65536_double as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_131072_double as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_262144_double as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_524288_double as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_1048576_double as unsafe extern "C" fn() -> ()),
        Some(ff_tx_init_tab_2097152_double as unsafe extern "C" fn() -> ()),
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
unsafe extern "C" fn ff_tx_init_tab_53_double() {
    ff_tx_tab_53_double[0 as libc::c_int as usize] = cos(2 as libc::c_int as libc::c_double
        * 3.14159265358979323846f64
        / 5 as libc::c_int as libc::c_double);
    ff_tx_tab_53_double[1 as libc::c_int as usize] = cos(2 as libc::c_int as libc::c_double
        * 3.14159265358979323846f64
        / 5 as libc::c_int as libc::c_double);
    ff_tx_tab_53_double[2 as libc::c_int as usize] = cos(2 as libc::c_int as libc::c_double
        * 3.14159265358979323846f64
        / 10 as libc::c_int as libc::c_double);
    ff_tx_tab_53_double[3 as libc::c_int as usize] = cos(2 as libc::c_int as libc::c_double
        * 3.14159265358979323846f64
        / 10 as libc::c_int as libc::c_double);
    ff_tx_tab_53_double[4 as libc::c_int as usize] = sin(2 as libc::c_int as libc::c_double
        * 3.14159265358979323846f64
        / 5 as libc::c_int as libc::c_double);
    ff_tx_tab_53_double[5 as libc::c_int as usize] = sin(2 as libc::c_int as libc::c_double
        * 3.14159265358979323846f64
        / 5 as libc::c_int as libc::c_double);
    ff_tx_tab_53_double[6 as libc::c_int as usize] = sin(2 as libc::c_int as libc::c_double
        * 3.14159265358979323846f64
        / 10 as libc::c_int as libc::c_double);
    ff_tx_tab_53_double[7 as libc::c_int as usize] = sin(2 as libc::c_int as libc::c_double
        * 3.14159265358979323846f64
        / 10 as libc::c_int as libc::c_double);
    ff_tx_tab_53_double[8 as libc::c_int as usize] = cos(2 as libc::c_int as libc::c_double
        * 3.14159265358979323846f64
        / 12 as libc::c_int as libc::c_double);
    ff_tx_tab_53_double[9 as libc::c_int as usize] = cos(2 as libc::c_int as libc::c_double
        * 3.14159265358979323846f64
        / 12 as libc::c_int as libc::c_double);
    ff_tx_tab_53_double[10 as libc::c_int as usize] = cos(2 as libc::c_int as libc::c_double
        * 3.14159265358979323846f64
        / 6 as libc::c_int as libc::c_double);
    ff_tx_tab_53_double[11 as libc::c_int as usize] = cos(8 as libc::c_int as libc::c_double
        * 3.14159265358979323846f64
        / 6 as libc::c_int as libc::c_double);
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_7_double() {
    ff_tx_tab_7_double[0 as libc::c_int as usize] = cos(2 as libc::c_int as libc::c_double
        * 3.14159265358979323846f64
        / 7 as libc::c_int as libc::c_double);
    ff_tx_tab_7_double[1 as libc::c_int as usize] = sin(2 as libc::c_int as libc::c_double
        * 3.14159265358979323846f64
        / 7 as libc::c_int as libc::c_double);
    ff_tx_tab_7_double[2 as libc::c_int as usize] = sin(2 as libc::c_int as libc::c_double
        * 3.14159265358979323846f64
        / 28 as libc::c_int as libc::c_double);
    ff_tx_tab_7_double[3 as libc::c_int as usize] = cos(2 as libc::c_int as libc::c_double
        * 3.14159265358979323846f64
        / 28 as libc::c_int as libc::c_double);
    ff_tx_tab_7_double[4 as libc::c_int as usize] = cos(2 as libc::c_int as libc::c_double
        * 3.14159265358979323846f64
        / 14 as libc::c_int as libc::c_double);
    ff_tx_tab_7_double[5 as libc::c_int as usize] = sin(2 as libc::c_int as libc::c_double
        * 3.14159265358979323846f64
        / 14 as libc::c_int as libc::c_double);
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_9_double() {
    ff_tx_tab_9_double[0 as libc::c_int as usize] = cos(2 as libc::c_int as libc::c_double
        * 3.14159265358979323846f64
        / 3 as libc::c_int as libc::c_double);
    ff_tx_tab_9_double[1 as libc::c_int as usize] = sin(2 as libc::c_int as libc::c_double
        * 3.14159265358979323846f64
        / 3 as libc::c_int as libc::c_double);
    ff_tx_tab_9_double[2 as libc::c_int as usize] = cos(2 as libc::c_int as libc::c_double
        * 3.14159265358979323846f64
        / 9 as libc::c_int as libc::c_double);
    ff_tx_tab_9_double[3 as libc::c_int as usize] = sin(2 as libc::c_int as libc::c_double
        * 3.14159265358979323846f64
        / 9 as libc::c_int as libc::c_double);
    ff_tx_tab_9_double[4 as libc::c_int as usize] = cos(2 as libc::c_int as libc::c_double
        * 3.14159265358979323846f64
        / 36 as libc::c_int as libc::c_double);
    ff_tx_tab_9_double[5 as libc::c_int as usize] = sin(2 as libc::c_int as libc::c_double
        * 3.14159265358979323846f64
        / 36 as libc::c_int as libc::c_double);
    ff_tx_tab_9_double[6 as libc::c_int as usize] = ff_tx_tab_9_double[2 as libc::c_int as usize]
        + ff_tx_tab_9_double[5 as libc::c_int as usize];
    ff_tx_tab_9_double[7 as libc::c_int as usize] = ff_tx_tab_9_double[3 as libc::c_int as usize]
        - ff_tx_tab_9_double[4 as libc::c_int as usize];
}
static mut nptwo_tabs_init_data: [FFTabInitData; 3] = unsafe {
    [
        {
            let mut init = FFTabInitData {
                func: Some(ff_tx_init_tab_53_double as unsafe extern "C" fn() -> ()),
                factors: [15 as libc::c_int, 5 as libc::c_int, 3 as libc::c_int, 0],
            };
            init
        },
        {
            let mut init = FFTabInitData {
                func: Some(ff_tx_init_tab_9_double as unsafe extern "C" fn() -> ()),
                factors: [9 as libc::c_int, 0, 0, 0],
            };
            init
        },
        {
            let mut init = FFTabInitData {
                func: Some(ff_tx_init_tab_7_double as unsafe extern "C" fn() -> ()),
                factors: [7 as libc::c_int, 0, 0, 0],
            };
            init
        },
    ]
};
static mut nptwo_tabs_init_once: [Once; 3] = [Once::new(), Once::new(), Once::new()];
#[no_mangle]
#[cold]
pub unsafe extern "C" fn ff_tx_init_tabs_double(mut len: libc::c_int) {
    let mut factor_2: libc::c_int = ff_ctz_c(len);
    if factor_2 != 0 {
        let mut idx: libc::c_int = factor_2 - 3 as libc::c_int;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i <= idx {
            sr_tabs_init_once[i as usize].call_once(|| sr_tabs_init_funcs[i as usize].unwrap()());
            i += 1;
            i;
        }
        len >>= factor_2;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while (i_0 as libc::c_ulong)
        < (::core::mem::size_of::<[FFTabInitData; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<FFTabInitData>() as libc::c_ulong)
    {
        let mut f: libc::c_int = 0;
        let mut f_idx: libc::c_int = 0 as libc::c_int;
        if len <= 1 as libc::c_int {
            return;
        }
        loop {
            let fresh19 = f_idx;
            f_idx = f_idx + 1;
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
unsafe extern "C" fn fft3(
    mut out: *mut TXComplex,
    mut in_0: *mut TXComplex,
    mut stride: ptrdiff_t,
) {
    let mut tmp: [TXComplex; 3] = [TXComplex { re: 0., im: 0. }; 3];
    let mut tab: *const TXSample = ff_tx_tab_53_double.as_mut_ptr();
    tmp[0 as libc::c_int as usize] = *in_0.offset(0 as libc::c_int as isize);
    tmp[1 as libc::c_int as usize].re =
        (*in_0.offset(1 as libc::c_int as isize)).im - (*in_0.offset(2 as libc::c_int as isize)).im;
    tmp[2 as libc::c_int as usize].im =
        (*in_0.offset(1 as libc::c_int as isize)).im + (*in_0.offset(2 as libc::c_int as isize)).im;
    tmp[1 as libc::c_int as usize].im =
        (*in_0.offset(1 as libc::c_int as isize)).re - (*in_0.offset(2 as libc::c_int as isize)).re;
    tmp[2 as libc::c_int as usize].re =
        (*in_0.offset(1 as libc::c_int as isize)).re + (*in_0.offset(2 as libc::c_int as isize)).re;
    (*out.offset((0 as libc::c_int as libc::c_long * stride) as isize)).re =
        tmp[0 as libc::c_int as usize].re + tmp[2 as libc::c_int as usize].re;
    (*out.offset((0 as libc::c_int as libc::c_long * stride) as isize)).im =
        tmp[0 as libc::c_int as usize].im + tmp[2 as libc::c_int as usize].im;
    tmp[1 as libc::c_int as usize].re =
        *tab.offset(8 as libc::c_int as isize) * tmp[1 as libc::c_int as usize].re;
    tmp[1 as libc::c_int as usize].im =
        *tab.offset(9 as libc::c_int as isize) * tmp[1 as libc::c_int as usize].im;
    tmp[2 as libc::c_int as usize].re =
        *tab.offset(10 as libc::c_int as isize) * tmp[2 as libc::c_int as usize].re;
    tmp[2 as libc::c_int as usize].im =
        *tab.offset(10 as libc::c_int as isize) * tmp[2 as libc::c_int as usize].im;
    (*out.offset((1 as libc::c_int as libc::c_long * stride) as isize)).re =
        tmp[0 as libc::c_int as usize].re - tmp[2 as libc::c_int as usize].re
            + tmp[1 as libc::c_int as usize].re;
    (*out.offset((1 as libc::c_int as libc::c_long * stride) as isize)).im =
        tmp[0 as libc::c_int as usize].im
            - tmp[2 as libc::c_int as usize].im
            - tmp[1 as libc::c_int as usize].im;
    (*out.offset((2 as libc::c_int as libc::c_long * stride) as isize)).re =
        tmp[0 as libc::c_int as usize].re
            - tmp[2 as libc::c_int as usize].re
            - tmp[1 as libc::c_int as usize].re;
    (*out.offset((2 as libc::c_int as libc::c_long * stride) as isize)).im =
        tmp[0 as libc::c_int as usize].im - tmp[2 as libc::c_int as usize].im
            + tmp[1 as libc::c_int as usize].im;
}
#[inline(always)]
unsafe extern "C" fn fft5(
    mut out: *mut TXComplex,
    mut in_0: *mut TXComplex,
    mut stride: ptrdiff_t,
) {
    let mut dc: TXComplex = TXComplex { re: 0., im: 0. };
    let mut z0: [TXComplex; 4] = [TXComplex { re: 0., im: 0. }; 4];
    let mut t: [TXComplex; 6] = [TXComplex { re: 0., im: 0. }; 6];
    let mut tab: *const TXSample = ff_tx_tab_53_double.as_mut_ptr();
    dc = *in_0.offset(0 as libc::c_int as isize);
    t[1 as libc::c_int as usize].im =
        (*in_0.offset(1 as libc::c_int as isize)).re - (*in_0.offset(4 as libc::c_int as isize)).re;
    t[0 as libc::c_int as usize].re =
        (*in_0.offset(1 as libc::c_int as isize)).re + (*in_0.offset(4 as libc::c_int as isize)).re;
    t[1 as libc::c_int as usize].re =
        (*in_0.offset(1 as libc::c_int as isize)).im - (*in_0.offset(4 as libc::c_int as isize)).im;
    t[0 as libc::c_int as usize].im =
        (*in_0.offset(1 as libc::c_int as isize)).im + (*in_0.offset(4 as libc::c_int as isize)).im;
    t[3 as libc::c_int as usize].im =
        (*in_0.offset(2 as libc::c_int as isize)).re - (*in_0.offset(3 as libc::c_int as isize)).re;
    t[2 as libc::c_int as usize].re =
        (*in_0.offset(2 as libc::c_int as isize)).re + (*in_0.offset(3 as libc::c_int as isize)).re;
    t[3 as libc::c_int as usize].re =
        (*in_0.offset(2 as libc::c_int as isize)).im - (*in_0.offset(3 as libc::c_int as isize)).im;
    t[2 as libc::c_int as usize].im =
        (*in_0.offset(2 as libc::c_int as isize)).im + (*in_0.offset(3 as libc::c_int as isize)).im;
    (*out.offset((0 as libc::c_int as libc::c_long * stride) as isize)).re =
        dc.re + t[0 as libc::c_int as usize].re + t[2 as libc::c_int as usize].re;
    (*out.offset((0 as libc::c_int as libc::c_long * stride) as isize)).im =
        dc.im + t[0 as libc::c_int as usize].im + t[2 as libc::c_int as usize].im;
    t[4 as libc::c_int as usize].re = *tab.offset(0 as libc::c_int as isize)
        * t[2 as libc::c_int as usize].re
        - *tab.offset(2 as libc::c_int as isize) * t[0 as libc::c_int as usize].re;
    t[0 as libc::c_int as usize].re = *tab.offset(0 as libc::c_int as isize)
        * t[0 as libc::c_int as usize].re
        - *tab.offset(2 as libc::c_int as isize) * t[2 as libc::c_int as usize].re;
    t[4 as libc::c_int as usize].im = *tab.offset(0 as libc::c_int as isize)
        * t[2 as libc::c_int as usize].im
        - *tab.offset(2 as libc::c_int as isize) * t[0 as libc::c_int as usize].im;
    t[0 as libc::c_int as usize].im = *tab.offset(0 as libc::c_int as isize)
        * t[0 as libc::c_int as usize].im
        - *tab.offset(2 as libc::c_int as isize) * t[2 as libc::c_int as usize].im;
    t[5 as libc::c_int as usize].re = *tab.offset(4 as libc::c_int as isize)
        * t[3 as libc::c_int as usize].re
        - *tab.offset(6 as libc::c_int as isize) * t[1 as libc::c_int as usize].re;
    t[1 as libc::c_int as usize].re = *tab.offset(4 as libc::c_int as isize)
        * t[1 as libc::c_int as usize].re
        + *tab.offset(6 as libc::c_int as isize) * t[3 as libc::c_int as usize].re;
    t[5 as libc::c_int as usize].im = *tab.offset(4 as libc::c_int as isize)
        * t[3 as libc::c_int as usize].im
        - *tab.offset(6 as libc::c_int as isize) * t[1 as libc::c_int as usize].im;
    t[1 as libc::c_int as usize].im = *tab.offset(4 as libc::c_int as isize)
        * t[1 as libc::c_int as usize].im
        + *tab.offset(6 as libc::c_int as isize) * t[3 as libc::c_int as usize].im;
    z0[0 as libc::c_int as usize].re =
        t[0 as libc::c_int as usize].re - t[1 as libc::c_int as usize].re;
    z0[3 as libc::c_int as usize].re =
        t[0 as libc::c_int as usize].re + t[1 as libc::c_int as usize].re;
    z0[0 as libc::c_int as usize].im =
        t[0 as libc::c_int as usize].im - t[1 as libc::c_int as usize].im;
    z0[3 as libc::c_int as usize].im =
        t[0 as libc::c_int as usize].im + t[1 as libc::c_int as usize].im;
    z0[2 as libc::c_int as usize].re =
        t[4 as libc::c_int as usize].re - t[5 as libc::c_int as usize].re;
    z0[1 as libc::c_int as usize].re =
        t[4 as libc::c_int as usize].re + t[5 as libc::c_int as usize].re;
    z0[2 as libc::c_int as usize].im =
        t[4 as libc::c_int as usize].im - t[5 as libc::c_int as usize].im;
    z0[1 as libc::c_int as usize].im =
        t[4 as libc::c_int as usize].im + t[5 as libc::c_int as usize].im;
    (*out.offset((1 as libc::c_int as libc::c_long * stride) as isize)).re =
        dc.re + z0[3 as libc::c_int as usize].re;
    (*out.offset((1 as libc::c_int as libc::c_long * stride) as isize)).im =
        dc.im + z0[0 as libc::c_int as usize].im;
    (*out.offset((2 as libc::c_int as libc::c_long * stride) as isize)).re =
        dc.re + z0[2 as libc::c_int as usize].re;
    (*out.offset((2 as libc::c_int as libc::c_long * stride) as isize)).im =
        dc.im + z0[1 as libc::c_int as usize].im;
    (*out.offset((3 as libc::c_int as libc::c_long * stride) as isize)).re =
        dc.re + z0[1 as libc::c_int as usize].re;
    (*out.offset((3 as libc::c_int as libc::c_long * stride) as isize)).im =
        dc.im + z0[2 as libc::c_int as usize].im;
    (*out.offset((4 as libc::c_int as libc::c_long * stride) as isize)).re =
        dc.re + z0[0 as libc::c_int as usize].re;
    (*out.offset((4 as libc::c_int as libc::c_long * stride) as isize)).im =
        dc.im + z0[3 as libc::c_int as usize].im;
}
#[inline(always)]
unsafe extern "C" fn fft5_m1(
    mut out: *mut TXComplex,
    mut in_0: *mut TXComplex,
    mut stride: ptrdiff_t,
) {
    let mut dc: TXComplex = TXComplex { re: 0., im: 0. };
    let mut z0: [TXComplex; 4] = [TXComplex { re: 0., im: 0. }; 4];
    let mut t: [TXComplex; 6] = [TXComplex { re: 0., im: 0. }; 6];
    let mut tab: *const TXSample = ff_tx_tab_53_double.as_mut_ptr();
    dc = *in_0.offset(0 as libc::c_int as isize);
    t[1 as libc::c_int as usize].im =
        (*in_0.offset(1 as libc::c_int as isize)).re - (*in_0.offset(4 as libc::c_int as isize)).re;
    t[0 as libc::c_int as usize].re =
        (*in_0.offset(1 as libc::c_int as isize)).re + (*in_0.offset(4 as libc::c_int as isize)).re;
    t[1 as libc::c_int as usize].re =
        (*in_0.offset(1 as libc::c_int as isize)).im - (*in_0.offset(4 as libc::c_int as isize)).im;
    t[0 as libc::c_int as usize].im =
        (*in_0.offset(1 as libc::c_int as isize)).im + (*in_0.offset(4 as libc::c_int as isize)).im;
    t[3 as libc::c_int as usize].im =
        (*in_0.offset(2 as libc::c_int as isize)).re - (*in_0.offset(3 as libc::c_int as isize)).re;
    t[2 as libc::c_int as usize].re =
        (*in_0.offset(2 as libc::c_int as isize)).re + (*in_0.offset(3 as libc::c_int as isize)).re;
    t[3 as libc::c_int as usize].re =
        (*in_0.offset(2 as libc::c_int as isize)).im - (*in_0.offset(3 as libc::c_int as isize)).im;
    t[2 as libc::c_int as usize].im =
        (*in_0.offset(2 as libc::c_int as isize)).im + (*in_0.offset(3 as libc::c_int as isize)).im;
    (*out.offset((0 as libc::c_int as libc::c_long * stride) as isize)).re =
        dc.re + t[0 as libc::c_int as usize].re + t[2 as libc::c_int as usize].re;
    (*out.offset((0 as libc::c_int as libc::c_long * stride) as isize)).im =
        dc.im + t[0 as libc::c_int as usize].im + t[2 as libc::c_int as usize].im;
    t[4 as libc::c_int as usize].re = *tab.offset(0 as libc::c_int as isize)
        * t[2 as libc::c_int as usize].re
        - *tab.offset(2 as libc::c_int as isize) * t[0 as libc::c_int as usize].re;
    t[0 as libc::c_int as usize].re = *tab.offset(0 as libc::c_int as isize)
        * t[0 as libc::c_int as usize].re
        - *tab.offset(2 as libc::c_int as isize) * t[2 as libc::c_int as usize].re;
    t[4 as libc::c_int as usize].im = *tab.offset(0 as libc::c_int as isize)
        * t[2 as libc::c_int as usize].im
        - *tab.offset(2 as libc::c_int as isize) * t[0 as libc::c_int as usize].im;
    t[0 as libc::c_int as usize].im = *tab.offset(0 as libc::c_int as isize)
        * t[0 as libc::c_int as usize].im
        - *tab.offset(2 as libc::c_int as isize) * t[2 as libc::c_int as usize].im;
    t[5 as libc::c_int as usize].re = *tab.offset(4 as libc::c_int as isize)
        * t[3 as libc::c_int as usize].re
        - *tab.offset(6 as libc::c_int as isize) * t[1 as libc::c_int as usize].re;
    t[1 as libc::c_int as usize].re = *tab.offset(4 as libc::c_int as isize)
        * t[1 as libc::c_int as usize].re
        + *tab.offset(6 as libc::c_int as isize) * t[3 as libc::c_int as usize].re;
    t[5 as libc::c_int as usize].im = *tab.offset(4 as libc::c_int as isize)
        * t[3 as libc::c_int as usize].im
        - *tab.offset(6 as libc::c_int as isize) * t[1 as libc::c_int as usize].im;
    t[1 as libc::c_int as usize].im = *tab.offset(4 as libc::c_int as isize)
        * t[1 as libc::c_int as usize].im
        + *tab.offset(6 as libc::c_int as isize) * t[3 as libc::c_int as usize].im;
    z0[0 as libc::c_int as usize].re =
        t[0 as libc::c_int as usize].re - t[1 as libc::c_int as usize].re;
    z0[3 as libc::c_int as usize].re =
        t[0 as libc::c_int as usize].re + t[1 as libc::c_int as usize].re;
    z0[0 as libc::c_int as usize].im =
        t[0 as libc::c_int as usize].im - t[1 as libc::c_int as usize].im;
    z0[3 as libc::c_int as usize].im =
        t[0 as libc::c_int as usize].im + t[1 as libc::c_int as usize].im;
    z0[2 as libc::c_int as usize].re =
        t[4 as libc::c_int as usize].re - t[5 as libc::c_int as usize].re;
    z0[1 as libc::c_int as usize].re =
        t[4 as libc::c_int as usize].re + t[5 as libc::c_int as usize].re;
    z0[2 as libc::c_int as usize].im =
        t[4 as libc::c_int as usize].im - t[5 as libc::c_int as usize].im;
    z0[1 as libc::c_int as usize].im =
        t[4 as libc::c_int as usize].im + t[5 as libc::c_int as usize].im;
    (*out.offset((6 as libc::c_int as libc::c_long * stride) as isize)).re =
        dc.re + z0[3 as libc::c_int as usize].re;
    (*out.offset((6 as libc::c_int as libc::c_long * stride) as isize)).im =
        dc.im + z0[0 as libc::c_int as usize].im;
    (*out.offset((12 as libc::c_int as libc::c_long * stride) as isize)).re =
        dc.re + z0[2 as libc::c_int as usize].re;
    (*out.offset((12 as libc::c_int as libc::c_long * stride) as isize)).im =
        dc.im + z0[1 as libc::c_int as usize].im;
    (*out.offset((3 as libc::c_int as libc::c_long * stride) as isize)).re =
        dc.re + z0[1 as libc::c_int as usize].re;
    (*out.offset((3 as libc::c_int as libc::c_long * stride) as isize)).im =
        dc.im + z0[2 as libc::c_int as usize].im;
    (*out.offset((9 as libc::c_int as libc::c_long * stride) as isize)).re =
        dc.re + z0[0 as libc::c_int as usize].re;
    (*out.offset((9 as libc::c_int as libc::c_long * stride) as isize)).im =
        dc.im + z0[3 as libc::c_int as usize].im;
}
#[inline(always)]
unsafe extern "C" fn fft5_m2(
    mut out: *mut TXComplex,
    mut in_0: *mut TXComplex,
    mut stride: ptrdiff_t,
) {
    let mut dc: TXComplex = TXComplex { re: 0., im: 0. };
    let mut z0: [TXComplex; 4] = [TXComplex { re: 0., im: 0. }; 4];
    let mut t: [TXComplex; 6] = [TXComplex { re: 0., im: 0. }; 6];
    let mut tab: *const TXSample = ff_tx_tab_53_double.as_mut_ptr();
    dc = *in_0.offset(0 as libc::c_int as isize);
    t[1 as libc::c_int as usize].im =
        (*in_0.offset(1 as libc::c_int as isize)).re - (*in_0.offset(4 as libc::c_int as isize)).re;
    t[0 as libc::c_int as usize].re =
        (*in_0.offset(1 as libc::c_int as isize)).re + (*in_0.offset(4 as libc::c_int as isize)).re;
    t[1 as libc::c_int as usize].re =
        (*in_0.offset(1 as libc::c_int as isize)).im - (*in_0.offset(4 as libc::c_int as isize)).im;
    t[0 as libc::c_int as usize].im =
        (*in_0.offset(1 as libc::c_int as isize)).im + (*in_0.offset(4 as libc::c_int as isize)).im;
    t[3 as libc::c_int as usize].im =
        (*in_0.offset(2 as libc::c_int as isize)).re - (*in_0.offset(3 as libc::c_int as isize)).re;
    t[2 as libc::c_int as usize].re =
        (*in_0.offset(2 as libc::c_int as isize)).re + (*in_0.offset(3 as libc::c_int as isize)).re;
    t[3 as libc::c_int as usize].re =
        (*in_0.offset(2 as libc::c_int as isize)).im - (*in_0.offset(3 as libc::c_int as isize)).im;
    t[2 as libc::c_int as usize].im =
        (*in_0.offset(2 as libc::c_int as isize)).im + (*in_0.offset(3 as libc::c_int as isize)).im;
    (*out.offset((10 as libc::c_int as libc::c_long * stride) as isize)).re =
        dc.re + t[0 as libc::c_int as usize].re + t[2 as libc::c_int as usize].re;
    (*out.offset((10 as libc::c_int as libc::c_long * stride) as isize)).im =
        dc.im + t[0 as libc::c_int as usize].im + t[2 as libc::c_int as usize].im;
    t[4 as libc::c_int as usize].re = *tab.offset(0 as libc::c_int as isize)
        * t[2 as libc::c_int as usize].re
        - *tab.offset(2 as libc::c_int as isize) * t[0 as libc::c_int as usize].re;
    t[0 as libc::c_int as usize].re = *tab.offset(0 as libc::c_int as isize)
        * t[0 as libc::c_int as usize].re
        - *tab.offset(2 as libc::c_int as isize) * t[2 as libc::c_int as usize].re;
    t[4 as libc::c_int as usize].im = *tab.offset(0 as libc::c_int as isize)
        * t[2 as libc::c_int as usize].im
        - *tab.offset(2 as libc::c_int as isize) * t[0 as libc::c_int as usize].im;
    t[0 as libc::c_int as usize].im = *tab.offset(0 as libc::c_int as isize)
        * t[0 as libc::c_int as usize].im
        - *tab.offset(2 as libc::c_int as isize) * t[2 as libc::c_int as usize].im;
    t[5 as libc::c_int as usize].re = *tab.offset(4 as libc::c_int as isize)
        * t[3 as libc::c_int as usize].re
        - *tab.offset(6 as libc::c_int as isize) * t[1 as libc::c_int as usize].re;
    t[1 as libc::c_int as usize].re = *tab.offset(4 as libc::c_int as isize)
        * t[1 as libc::c_int as usize].re
        + *tab.offset(6 as libc::c_int as isize) * t[3 as libc::c_int as usize].re;
    t[5 as libc::c_int as usize].im = *tab.offset(4 as libc::c_int as isize)
        * t[3 as libc::c_int as usize].im
        - *tab.offset(6 as libc::c_int as isize) * t[1 as libc::c_int as usize].im;
    t[1 as libc::c_int as usize].im = *tab.offset(4 as libc::c_int as isize)
        * t[1 as libc::c_int as usize].im
        + *tab.offset(6 as libc::c_int as isize) * t[3 as libc::c_int as usize].im;
    z0[0 as libc::c_int as usize].re =
        t[0 as libc::c_int as usize].re - t[1 as libc::c_int as usize].re;
    z0[3 as libc::c_int as usize].re =
        t[0 as libc::c_int as usize].re + t[1 as libc::c_int as usize].re;
    z0[0 as libc::c_int as usize].im =
        t[0 as libc::c_int as usize].im - t[1 as libc::c_int as usize].im;
    z0[3 as libc::c_int as usize].im =
        t[0 as libc::c_int as usize].im + t[1 as libc::c_int as usize].im;
    z0[2 as libc::c_int as usize].re =
        t[4 as libc::c_int as usize].re - t[5 as libc::c_int as usize].re;
    z0[1 as libc::c_int as usize].re =
        t[4 as libc::c_int as usize].re + t[5 as libc::c_int as usize].re;
    z0[2 as libc::c_int as usize].im =
        t[4 as libc::c_int as usize].im - t[5 as libc::c_int as usize].im;
    z0[1 as libc::c_int as usize].im =
        t[4 as libc::c_int as usize].im + t[5 as libc::c_int as usize].im;
    (*out.offset((1 as libc::c_int as libc::c_long * stride) as isize)).re =
        dc.re + z0[3 as libc::c_int as usize].re;
    (*out.offset((1 as libc::c_int as libc::c_long * stride) as isize)).im =
        dc.im + z0[0 as libc::c_int as usize].im;
    (*out.offset((7 as libc::c_int as libc::c_long * stride) as isize)).re =
        dc.re + z0[2 as libc::c_int as usize].re;
    (*out.offset((7 as libc::c_int as libc::c_long * stride) as isize)).im =
        dc.im + z0[1 as libc::c_int as usize].im;
    (*out.offset((13 as libc::c_int as libc::c_long * stride) as isize)).re =
        dc.re + z0[1 as libc::c_int as usize].re;
    (*out.offset((13 as libc::c_int as libc::c_long * stride) as isize)).im =
        dc.im + z0[2 as libc::c_int as usize].im;
    (*out.offset((4 as libc::c_int as libc::c_long * stride) as isize)).re =
        dc.re + z0[0 as libc::c_int as usize].re;
    (*out.offset((4 as libc::c_int as libc::c_long * stride) as isize)).im =
        dc.im + z0[3 as libc::c_int as usize].im;
}
#[inline(always)]
unsafe extern "C" fn fft5_m3(
    mut out: *mut TXComplex,
    mut in_0: *mut TXComplex,
    mut stride: ptrdiff_t,
) {
    let mut dc: TXComplex = TXComplex { re: 0., im: 0. };
    let mut z0: [TXComplex; 4] = [TXComplex { re: 0., im: 0. }; 4];
    let mut t: [TXComplex; 6] = [TXComplex { re: 0., im: 0. }; 6];
    let mut tab: *const TXSample = ff_tx_tab_53_double.as_mut_ptr();
    dc = *in_0.offset(0 as libc::c_int as isize);
    t[1 as libc::c_int as usize].im =
        (*in_0.offset(1 as libc::c_int as isize)).re - (*in_0.offset(4 as libc::c_int as isize)).re;
    t[0 as libc::c_int as usize].re =
        (*in_0.offset(1 as libc::c_int as isize)).re + (*in_0.offset(4 as libc::c_int as isize)).re;
    t[1 as libc::c_int as usize].re =
        (*in_0.offset(1 as libc::c_int as isize)).im - (*in_0.offset(4 as libc::c_int as isize)).im;
    t[0 as libc::c_int as usize].im =
        (*in_0.offset(1 as libc::c_int as isize)).im + (*in_0.offset(4 as libc::c_int as isize)).im;
    t[3 as libc::c_int as usize].im =
        (*in_0.offset(2 as libc::c_int as isize)).re - (*in_0.offset(3 as libc::c_int as isize)).re;
    t[2 as libc::c_int as usize].re =
        (*in_0.offset(2 as libc::c_int as isize)).re + (*in_0.offset(3 as libc::c_int as isize)).re;
    t[3 as libc::c_int as usize].re =
        (*in_0.offset(2 as libc::c_int as isize)).im - (*in_0.offset(3 as libc::c_int as isize)).im;
    t[2 as libc::c_int as usize].im =
        (*in_0.offset(2 as libc::c_int as isize)).im + (*in_0.offset(3 as libc::c_int as isize)).im;
    (*out.offset((5 as libc::c_int as libc::c_long * stride) as isize)).re =
        dc.re + t[0 as libc::c_int as usize].re + t[2 as libc::c_int as usize].re;
    (*out.offset((5 as libc::c_int as libc::c_long * stride) as isize)).im =
        dc.im + t[0 as libc::c_int as usize].im + t[2 as libc::c_int as usize].im;
    t[4 as libc::c_int as usize].re = *tab.offset(0 as libc::c_int as isize)
        * t[2 as libc::c_int as usize].re
        - *tab.offset(2 as libc::c_int as isize) * t[0 as libc::c_int as usize].re;
    t[0 as libc::c_int as usize].re = *tab.offset(0 as libc::c_int as isize)
        * t[0 as libc::c_int as usize].re
        - *tab.offset(2 as libc::c_int as isize) * t[2 as libc::c_int as usize].re;
    t[4 as libc::c_int as usize].im = *tab.offset(0 as libc::c_int as isize)
        * t[2 as libc::c_int as usize].im
        - *tab.offset(2 as libc::c_int as isize) * t[0 as libc::c_int as usize].im;
    t[0 as libc::c_int as usize].im = *tab.offset(0 as libc::c_int as isize)
        * t[0 as libc::c_int as usize].im
        - *tab.offset(2 as libc::c_int as isize) * t[2 as libc::c_int as usize].im;
    t[5 as libc::c_int as usize].re = *tab.offset(4 as libc::c_int as isize)
        * t[3 as libc::c_int as usize].re
        - *tab.offset(6 as libc::c_int as isize) * t[1 as libc::c_int as usize].re;
    t[1 as libc::c_int as usize].re = *tab.offset(4 as libc::c_int as isize)
        * t[1 as libc::c_int as usize].re
        + *tab.offset(6 as libc::c_int as isize) * t[3 as libc::c_int as usize].re;
    t[5 as libc::c_int as usize].im = *tab.offset(4 as libc::c_int as isize)
        * t[3 as libc::c_int as usize].im
        - *tab.offset(6 as libc::c_int as isize) * t[1 as libc::c_int as usize].im;
    t[1 as libc::c_int as usize].im = *tab.offset(4 as libc::c_int as isize)
        * t[1 as libc::c_int as usize].im
        + *tab.offset(6 as libc::c_int as isize) * t[3 as libc::c_int as usize].im;
    z0[0 as libc::c_int as usize].re =
        t[0 as libc::c_int as usize].re - t[1 as libc::c_int as usize].re;
    z0[3 as libc::c_int as usize].re =
        t[0 as libc::c_int as usize].re + t[1 as libc::c_int as usize].re;
    z0[0 as libc::c_int as usize].im =
        t[0 as libc::c_int as usize].im - t[1 as libc::c_int as usize].im;
    z0[3 as libc::c_int as usize].im =
        t[0 as libc::c_int as usize].im + t[1 as libc::c_int as usize].im;
    z0[2 as libc::c_int as usize].re =
        t[4 as libc::c_int as usize].re - t[5 as libc::c_int as usize].re;
    z0[1 as libc::c_int as usize].re =
        t[4 as libc::c_int as usize].re + t[5 as libc::c_int as usize].re;
    z0[2 as libc::c_int as usize].im =
        t[4 as libc::c_int as usize].im - t[5 as libc::c_int as usize].im;
    z0[1 as libc::c_int as usize].im =
        t[4 as libc::c_int as usize].im + t[5 as libc::c_int as usize].im;
    (*out.offset((11 as libc::c_int as libc::c_long * stride) as isize)).re =
        dc.re + z0[3 as libc::c_int as usize].re;
    (*out.offset((11 as libc::c_int as libc::c_long * stride) as isize)).im =
        dc.im + z0[0 as libc::c_int as usize].im;
    (*out.offset((2 as libc::c_int as libc::c_long * stride) as isize)).re =
        dc.re + z0[2 as libc::c_int as usize].re;
    (*out.offset((2 as libc::c_int as libc::c_long * stride) as isize)).im =
        dc.im + z0[1 as libc::c_int as usize].im;
    (*out.offset((8 as libc::c_int as libc::c_long * stride) as isize)).re =
        dc.re + z0[1 as libc::c_int as usize].re;
    (*out.offset((8 as libc::c_int as libc::c_long * stride) as isize)).im =
        dc.im + z0[2 as libc::c_int as usize].im;
    (*out.offset((14 as libc::c_int as libc::c_long * stride) as isize)).re =
        dc.re + z0[0 as libc::c_int as usize].re;
    (*out.offset((14 as libc::c_int as libc::c_long * stride) as isize)).im =
        dc.im + z0[3 as libc::c_int as usize].im;
}
#[inline(always)]
unsafe extern "C" fn fft7(
    mut out: *mut TXComplex,
    mut in_0: *mut TXComplex,
    mut stride: ptrdiff_t,
) {
    let mut dc: TXComplex = TXComplex { re: 0., im: 0. };
    let mut t: [TXComplex; 6] = [TXComplex { re: 0., im: 0. }; 6];
    let mut z: [TXComplex; 3] = [TXComplex { re: 0., im: 0. }; 3];
    let mut tab: *const TXComplex = ff_tx_tab_7_double.as_mut_ptr() as *const TXComplex;
    dc = *in_0.offset(0 as libc::c_int as isize);
    t[1 as libc::c_int as usize].re =
        (*in_0.offset(1 as libc::c_int as isize)).re - (*in_0.offset(6 as libc::c_int as isize)).re;
    t[0 as libc::c_int as usize].re =
        (*in_0.offset(1 as libc::c_int as isize)).re + (*in_0.offset(6 as libc::c_int as isize)).re;
    t[1 as libc::c_int as usize].im =
        (*in_0.offset(1 as libc::c_int as isize)).im - (*in_0.offset(6 as libc::c_int as isize)).im;
    t[0 as libc::c_int as usize].im =
        (*in_0.offset(1 as libc::c_int as isize)).im + (*in_0.offset(6 as libc::c_int as isize)).im;
    t[3 as libc::c_int as usize].re =
        (*in_0.offset(2 as libc::c_int as isize)).re - (*in_0.offset(5 as libc::c_int as isize)).re;
    t[2 as libc::c_int as usize].re =
        (*in_0.offset(2 as libc::c_int as isize)).re + (*in_0.offset(5 as libc::c_int as isize)).re;
    t[3 as libc::c_int as usize].im =
        (*in_0.offset(2 as libc::c_int as isize)).im - (*in_0.offset(5 as libc::c_int as isize)).im;
    t[2 as libc::c_int as usize].im =
        (*in_0.offset(2 as libc::c_int as isize)).im + (*in_0.offset(5 as libc::c_int as isize)).im;
    t[5 as libc::c_int as usize].re =
        (*in_0.offset(3 as libc::c_int as isize)).re - (*in_0.offset(4 as libc::c_int as isize)).re;
    t[4 as libc::c_int as usize].re =
        (*in_0.offset(3 as libc::c_int as isize)).re + (*in_0.offset(4 as libc::c_int as isize)).re;
    t[5 as libc::c_int as usize].im =
        (*in_0.offset(3 as libc::c_int as isize)).im - (*in_0.offset(4 as libc::c_int as isize)).im;
    t[4 as libc::c_int as usize].im =
        (*in_0.offset(3 as libc::c_int as isize)).im + (*in_0.offset(4 as libc::c_int as isize)).im;
    (*out.offset((0 as libc::c_int as libc::c_long * stride) as isize)).re = dc.re
        + t[0 as libc::c_int as usize].re
        + t[2 as libc::c_int as usize].re
        + t[4 as libc::c_int as usize].re;
    (*out.offset((0 as libc::c_int as libc::c_long * stride) as isize)).im = dc.im
        + t[0 as libc::c_int as usize].im
        + t[2 as libc::c_int as usize].im
        + t[4 as libc::c_int as usize].im;
    z[0 as libc::c_int as usize].re = (*tab.offset(0 as libc::c_int as isize)).re
        * t[0 as libc::c_int as usize].re
        - (*tab.offset(2 as libc::c_int as isize)).re * t[4 as libc::c_int as usize].re
        - (*tab.offset(1 as libc::c_int as isize)).re * t[2 as libc::c_int as usize].re;
    z[1 as libc::c_int as usize].re = (*tab.offset(0 as libc::c_int as isize)).re
        * t[4 as libc::c_int as usize].re
        - (*tab.offset(1 as libc::c_int as isize)).re * t[0 as libc::c_int as usize].re
        - (*tab.offset(2 as libc::c_int as isize)).re * t[2 as libc::c_int as usize].re;
    z[2 as libc::c_int as usize].re = (*tab.offset(0 as libc::c_int as isize)).re
        * t[2 as libc::c_int as usize].re
        - (*tab.offset(2 as libc::c_int as isize)).re * t[0 as libc::c_int as usize].re
        - (*tab.offset(1 as libc::c_int as isize)).re * t[4 as libc::c_int as usize].re;
    z[0 as libc::c_int as usize].im = (*tab.offset(0 as libc::c_int as isize)).re
        * t[0 as libc::c_int as usize].im
        - (*tab.offset(1 as libc::c_int as isize)).re * t[2 as libc::c_int as usize].im
        - (*tab.offset(2 as libc::c_int as isize)).re * t[4 as libc::c_int as usize].im;
    z[1 as libc::c_int as usize].im = (*tab.offset(0 as libc::c_int as isize)).re
        * t[4 as libc::c_int as usize].im
        - (*tab.offset(1 as libc::c_int as isize)).re * t[0 as libc::c_int as usize].im
        - (*tab.offset(2 as libc::c_int as isize)).re * t[2 as libc::c_int as usize].im;
    z[2 as libc::c_int as usize].im = (*tab.offset(0 as libc::c_int as isize)).re
        * t[2 as libc::c_int as usize].im
        - (*tab.offset(2 as libc::c_int as isize)).re * t[0 as libc::c_int as usize].im
        - (*tab.offset(1 as libc::c_int as isize)).re * t[4 as libc::c_int as usize].im;
    t[0 as libc::c_int as usize].re = (*tab.offset(2 as libc::c_int as isize)).im
        * t[1 as libc::c_int as usize].im
        + (*tab.offset(1 as libc::c_int as isize)).im * t[5 as libc::c_int as usize].im
        - (*tab.offset(0 as libc::c_int as isize)).im * t[3 as libc::c_int as usize].im;
    t[2 as libc::c_int as usize].re = (*tab.offset(0 as libc::c_int as isize)).im
        * t[5 as libc::c_int as usize].im
        + (*tab.offset(2 as libc::c_int as isize)).im * t[3 as libc::c_int as usize].im
        - (*tab.offset(1 as libc::c_int as isize)).im * t[1 as libc::c_int as usize].im;
    t[4 as libc::c_int as usize].re = (*tab.offset(2 as libc::c_int as isize)).im
        * t[5 as libc::c_int as usize].im
        + (*tab.offset(1 as libc::c_int as isize)).im * t[3 as libc::c_int as usize].im
        + (*tab.offset(0 as libc::c_int as isize)).im * t[1 as libc::c_int as usize].im;
    t[0 as libc::c_int as usize].im = (*tab.offset(0 as libc::c_int as isize)).im
        * t[1 as libc::c_int as usize].re
        + (*tab.offset(1 as libc::c_int as isize)).im * t[3 as libc::c_int as usize].re
        + (*tab.offset(2 as libc::c_int as isize)).im * t[5 as libc::c_int as usize].re;
    t[2 as libc::c_int as usize].im = (*tab.offset(2 as libc::c_int as isize)).im
        * t[3 as libc::c_int as usize].re
        + (*tab.offset(0 as libc::c_int as isize)).im * t[5 as libc::c_int as usize].re
        - (*tab.offset(1 as libc::c_int as isize)).im * t[1 as libc::c_int as usize].re;
    t[4 as libc::c_int as usize].im = (*tab.offset(2 as libc::c_int as isize)).im
        * t[1 as libc::c_int as usize].re
        + (*tab.offset(1 as libc::c_int as isize)).im * t[5 as libc::c_int as usize].re
        - (*tab.offset(0 as libc::c_int as isize)).im * t[3 as libc::c_int as usize].re;
    t[1 as libc::c_int as usize].re =
        z[0 as libc::c_int as usize].re - t[4 as libc::c_int as usize].re;
    z[0 as libc::c_int as usize].re =
        z[0 as libc::c_int as usize].re + t[4 as libc::c_int as usize].re;
    t[3 as libc::c_int as usize].re =
        z[1 as libc::c_int as usize].re - t[2 as libc::c_int as usize].re;
    z[1 as libc::c_int as usize].re =
        z[1 as libc::c_int as usize].re + t[2 as libc::c_int as usize].re;
    t[5 as libc::c_int as usize].re =
        z[2 as libc::c_int as usize].re - t[0 as libc::c_int as usize].re;
    z[2 as libc::c_int as usize].re =
        z[2 as libc::c_int as usize].re + t[0 as libc::c_int as usize].re;
    t[1 as libc::c_int as usize].im =
        z[0 as libc::c_int as usize].im - t[0 as libc::c_int as usize].im;
    z[0 as libc::c_int as usize].im =
        z[0 as libc::c_int as usize].im + t[0 as libc::c_int as usize].im;
    t[3 as libc::c_int as usize].im =
        z[1 as libc::c_int as usize].im - t[2 as libc::c_int as usize].im;
    z[1 as libc::c_int as usize].im =
        z[1 as libc::c_int as usize].im + t[2 as libc::c_int as usize].im;
    t[5 as libc::c_int as usize].im =
        z[2 as libc::c_int as usize].im - t[4 as libc::c_int as usize].im;
    z[2 as libc::c_int as usize].im =
        z[2 as libc::c_int as usize].im + t[4 as libc::c_int as usize].im;
    (*out.offset((1 as libc::c_int as libc::c_long * stride) as isize)).re =
        dc.re + z[0 as libc::c_int as usize].re;
    (*out.offset((1 as libc::c_int as libc::c_long * stride) as isize)).im =
        dc.im + t[1 as libc::c_int as usize].im;
    (*out.offset((2 as libc::c_int as libc::c_long * stride) as isize)).re =
        dc.re + t[3 as libc::c_int as usize].re;
    (*out.offset((2 as libc::c_int as libc::c_long * stride) as isize)).im =
        dc.im + z[1 as libc::c_int as usize].im;
    (*out.offset((3 as libc::c_int as libc::c_long * stride) as isize)).re =
        dc.re + z[2 as libc::c_int as usize].re;
    (*out.offset((3 as libc::c_int as libc::c_long * stride) as isize)).im =
        dc.im + t[5 as libc::c_int as usize].im;
    (*out.offset((4 as libc::c_int as libc::c_long * stride) as isize)).re =
        dc.re + t[5 as libc::c_int as usize].re;
    (*out.offset((4 as libc::c_int as libc::c_long * stride) as isize)).im =
        dc.im + z[2 as libc::c_int as usize].im;
    (*out.offset((5 as libc::c_int as libc::c_long * stride) as isize)).re =
        dc.re + z[1 as libc::c_int as usize].re;
    (*out.offset((5 as libc::c_int as libc::c_long * stride) as isize)).im =
        dc.im + t[3 as libc::c_int as usize].im;
    (*out.offset((6 as libc::c_int as libc::c_long * stride) as isize)).re =
        dc.re + t[1 as libc::c_int as usize].re;
    (*out.offset((6 as libc::c_int as libc::c_long * stride) as isize)).im =
        dc.im + z[0 as libc::c_int as usize].im;
}
#[inline(always)]
unsafe extern "C" fn fft9(
    mut out: *mut TXComplex,
    mut in_0: *mut TXComplex,
    mut stride: ptrdiff_t,
) {
    let mut tab: *const TXComplex = ff_tx_tab_9_double.as_mut_ptr() as *const TXComplex;
    let mut dc: TXComplex = TXComplex { re: 0., im: 0. };
    let mut t: [TXComplex; 16] = [TXComplex { re: 0., im: 0. }; 16];
    let mut w: [TXComplex; 4] = [TXComplex { re: 0., im: 0. }; 4];
    let mut x: [TXComplex; 5] = [TXComplex { re: 0., im: 0. }; 5];
    let mut y: [TXComplex; 5] = [TXComplex { re: 0., im: 0. }; 5];
    let mut z: [TXComplex; 2] = [TXComplex { re: 0., im: 0. }; 2];
    dc = *in_0.offset(0 as libc::c_int as isize);
    t[1 as libc::c_int as usize].re =
        (*in_0.offset(1 as libc::c_int as isize)).re - (*in_0.offset(8 as libc::c_int as isize)).re;
    t[0 as libc::c_int as usize].re =
        (*in_0.offset(1 as libc::c_int as isize)).re + (*in_0.offset(8 as libc::c_int as isize)).re;
    t[1 as libc::c_int as usize].im =
        (*in_0.offset(1 as libc::c_int as isize)).im - (*in_0.offset(8 as libc::c_int as isize)).im;
    t[0 as libc::c_int as usize].im =
        (*in_0.offset(1 as libc::c_int as isize)).im + (*in_0.offset(8 as libc::c_int as isize)).im;
    t[3 as libc::c_int as usize].re =
        (*in_0.offset(2 as libc::c_int as isize)).re - (*in_0.offset(7 as libc::c_int as isize)).re;
    t[2 as libc::c_int as usize].re =
        (*in_0.offset(2 as libc::c_int as isize)).re + (*in_0.offset(7 as libc::c_int as isize)).re;
    t[3 as libc::c_int as usize].im =
        (*in_0.offset(2 as libc::c_int as isize)).im - (*in_0.offset(7 as libc::c_int as isize)).im;
    t[2 as libc::c_int as usize].im =
        (*in_0.offset(2 as libc::c_int as isize)).im + (*in_0.offset(7 as libc::c_int as isize)).im;
    t[5 as libc::c_int as usize].re =
        (*in_0.offset(3 as libc::c_int as isize)).re - (*in_0.offset(6 as libc::c_int as isize)).re;
    t[4 as libc::c_int as usize].re =
        (*in_0.offset(3 as libc::c_int as isize)).re + (*in_0.offset(6 as libc::c_int as isize)).re;
    t[5 as libc::c_int as usize].im =
        (*in_0.offset(3 as libc::c_int as isize)).im - (*in_0.offset(6 as libc::c_int as isize)).im;
    t[4 as libc::c_int as usize].im =
        (*in_0.offset(3 as libc::c_int as isize)).im + (*in_0.offset(6 as libc::c_int as isize)).im;
    t[7 as libc::c_int as usize].re =
        (*in_0.offset(4 as libc::c_int as isize)).re - (*in_0.offset(5 as libc::c_int as isize)).re;
    t[6 as libc::c_int as usize].re =
        (*in_0.offset(4 as libc::c_int as isize)).re + (*in_0.offset(5 as libc::c_int as isize)).re;
    t[7 as libc::c_int as usize].im =
        (*in_0.offset(4 as libc::c_int as isize)).im - (*in_0.offset(5 as libc::c_int as isize)).im;
    t[6 as libc::c_int as usize].im =
        (*in_0.offset(4 as libc::c_int as isize)).im + (*in_0.offset(5 as libc::c_int as isize)).im;
    w[0 as libc::c_int as usize].re =
        t[0 as libc::c_int as usize].re - t[6 as libc::c_int as usize].re;
    w[0 as libc::c_int as usize].im =
        t[0 as libc::c_int as usize].im - t[6 as libc::c_int as usize].im;
    w[1 as libc::c_int as usize].re =
        t[2 as libc::c_int as usize].re - t[6 as libc::c_int as usize].re;
    w[1 as libc::c_int as usize].im =
        t[2 as libc::c_int as usize].im - t[6 as libc::c_int as usize].im;
    w[2 as libc::c_int as usize].re =
        t[1 as libc::c_int as usize].re - t[7 as libc::c_int as usize].re;
    w[2 as libc::c_int as usize].im =
        t[1 as libc::c_int as usize].im - t[7 as libc::c_int as usize].im;
    w[3 as libc::c_int as usize].re =
        t[3 as libc::c_int as usize].re + t[7 as libc::c_int as usize].re;
    w[3 as libc::c_int as usize].im =
        t[3 as libc::c_int as usize].im + t[7 as libc::c_int as usize].im;
    z[0 as libc::c_int as usize].re = dc.re + t[4 as libc::c_int as usize].re;
    z[0 as libc::c_int as usize].im = dc.im + t[4 as libc::c_int as usize].im;
    z[1 as libc::c_int as usize].re = t[0 as libc::c_int as usize].re
        + t[2 as libc::c_int as usize].re
        + t[6 as libc::c_int as usize].re;
    z[1 as libc::c_int as usize].im = t[0 as libc::c_int as usize].im
        + t[2 as libc::c_int as usize].im
        + t[6 as libc::c_int as usize].im;
    (*out.offset((0 as libc::c_int as libc::c_long * stride) as isize)).re =
        z[0 as libc::c_int as usize].re + z[1 as libc::c_int as usize].re;
    (*out.offset((0 as libc::c_int as libc::c_long * stride) as isize)).im =
        z[0 as libc::c_int as usize].im + z[1 as libc::c_int as usize].im;
    y[3 as libc::c_int as usize].re = (*tab.offset(0 as libc::c_int as isize)).im
        * (t[1 as libc::c_int as usize].re - t[3 as libc::c_int as usize].re
            + t[7 as libc::c_int as usize].re);
    y[3 as libc::c_int as usize].im = (*tab.offset(0 as libc::c_int as isize)).im
        * (t[1 as libc::c_int as usize].im - t[3 as libc::c_int as usize].im
            + t[7 as libc::c_int as usize].im);
    x[3 as libc::c_int as usize].re = z[0 as libc::c_int as usize].re
        + (*tab.offset(0 as libc::c_int as isize)).re * z[1 as libc::c_int as usize].re;
    x[3 as libc::c_int as usize].im = z[0 as libc::c_int as usize].im
        + (*tab.offset(0 as libc::c_int as isize)).re * z[1 as libc::c_int as usize].im;
    z[0 as libc::c_int as usize].re =
        dc.re + (*tab.offset(0 as libc::c_int as isize)).re * t[4 as libc::c_int as usize].re;
    z[0 as libc::c_int as usize].im =
        dc.im + (*tab.offset(0 as libc::c_int as isize)).re * t[4 as libc::c_int as usize].im;
    x[1 as libc::c_int as usize].re = (*tab.offset(1 as libc::c_int as isize)).re
        * w[0 as libc::c_int as usize].re
        + (*tab.offset(2 as libc::c_int as isize)).im * w[1 as libc::c_int as usize].re;
    x[1 as libc::c_int as usize].im = (*tab.offset(1 as libc::c_int as isize)).re
        * w[0 as libc::c_int as usize].im
        + (*tab.offset(2 as libc::c_int as isize)).im * w[1 as libc::c_int as usize].im;
    x[2 as libc::c_int as usize].re = (*tab.offset(2 as libc::c_int as isize)).im
        * w[0 as libc::c_int as usize].re
        - (*tab.offset(3 as libc::c_int as isize)).re * w[1 as libc::c_int as usize].re;
    x[2 as libc::c_int as usize].im = (*tab.offset(2 as libc::c_int as isize)).im
        * w[0 as libc::c_int as usize].im
        - (*tab.offset(3 as libc::c_int as isize)).re * w[1 as libc::c_int as usize].im;
    y[1 as libc::c_int as usize].re = (*tab.offset(1 as libc::c_int as isize)).im
        * w[2 as libc::c_int as usize].re
        + (*tab.offset(2 as libc::c_int as isize)).re * w[3 as libc::c_int as usize].re;
    y[1 as libc::c_int as usize].im = (*tab.offset(1 as libc::c_int as isize)).im
        * w[2 as libc::c_int as usize].im
        + (*tab.offset(2 as libc::c_int as isize)).re * w[3 as libc::c_int as usize].im;
    y[2 as libc::c_int as usize].re = (*tab.offset(2 as libc::c_int as isize)).re
        * w[2 as libc::c_int as usize].re
        - (*tab.offset(3 as libc::c_int as isize)).im * w[3 as libc::c_int as usize].re;
    y[2 as libc::c_int as usize].im = (*tab.offset(2 as libc::c_int as isize)).re
        * w[2 as libc::c_int as usize].im
        - (*tab.offset(3 as libc::c_int as isize)).im * w[3 as libc::c_int as usize].im;
    y[0 as libc::c_int as usize].re =
        (*tab.offset(0 as libc::c_int as isize)).im * t[5 as libc::c_int as usize].re;
    y[0 as libc::c_int as usize].im =
        (*tab.offset(0 as libc::c_int as isize)).im * t[5 as libc::c_int as usize].im;
    x[4 as libc::c_int as usize].re =
        x[1 as libc::c_int as usize].re + x[2 as libc::c_int as usize].re;
    x[4 as libc::c_int as usize].im =
        x[1 as libc::c_int as usize].im + x[2 as libc::c_int as usize].im;
    y[4 as libc::c_int as usize].re =
        y[1 as libc::c_int as usize].re - y[2 as libc::c_int as usize].re;
    y[4 as libc::c_int as usize].im =
        y[1 as libc::c_int as usize].im - y[2 as libc::c_int as usize].im;
    x[1 as libc::c_int as usize].re =
        z[0 as libc::c_int as usize].re + x[1 as libc::c_int as usize].re;
    x[1 as libc::c_int as usize].im =
        z[0 as libc::c_int as usize].im + x[1 as libc::c_int as usize].im;
    y[1 as libc::c_int as usize].re =
        y[0 as libc::c_int as usize].re + y[1 as libc::c_int as usize].re;
    y[1 as libc::c_int as usize].im =
        y[0 as libc::c_int as usize].im + y[1 as libc::c_int as usize].im;
    x[2 as libc::c_int as usize].re =
        z[0 as libc::c_int as usize].re + x[2 as libc::c_int as usize].re;
    x[2 as libc::c_int as usize].im =
        z[0 as libc::c_int as usize].im + x[2 as libc::c_int as usize].im;
    y[2 as libc::c_int as usize].re =
        y[2 as libc::c_int as usize].re - y[0 as libc::c_int as usize].re;
    y[2 as libc::c_int as usize].im =
        y[2 as libc::c_int as usize].im - y[0 as libc::c_int as usize].im;
    x[4 as libc::c_int as usize].re =
        z[0 as libc::c_int as usize].re - x[4 as libc::c_int as usize].re;
    x[4 as libc::c_int as usize].im =
        z[0 as libc::c_int as usize].im - x[4 as libc::c_int as usize].im;
    y[4 as libc::c_int as usize].re =
        y[0 as libc::c_int as usize].re - y[4 as libc::c_int as usize].re;
    y[4 as libc::c_int as usize].im =
        y[0 as libc::c_int as usize].im - y[4 as libc::c_int as usize].im;
    *out.offset((1 as libc::c_int as libc::c_long * stride) as isize) = {
        let mut init = AVComplexDouble {
            re: x[1 as libc::c_int as usize].re + y[1 as libc::c_int as usize].im,
            im: x[1 as libc::c_int as usize].im - y[1 as libc::c_int as usize].re,
        };
        init
    };
    *out.offset((2 as libc::c_int as libc::c_long * stride) as isize) = {
        let mut init = AVComplexDouble {
            re: x[2 as libc::c_int as usize].re + y[2 as libc::c_int as usize].im,
            im: x[2 as libc::c_int as usize].im - y[2 as libc::c_int as usize].re,
        };
        init
    };
    *out.offset((3 as libc::c_int as libc::c_long * stride) as isize) = {
        let mut init = AVComplexDouble {
            re: x[3 as libc::c_int as usize].re + y[3 as libc::c_int as usize].im,
            im: x[3 as libc::c_int as usize].im - y[3 as libc::c_int as usize].re,
        };
        init
    };
    *out.offset((4 as libc::c_int as libc::c_long * stride) as isize) = {
        let mut init = AVComplexDouble {
            re: x[4 as libc::c_int as usize].re + y[4 as libc::c_int as usize].im,
            im: x[4 as libc::c_int as usize].im - y[4 as libc::c_int as usize].re,
        };
        init
    };
    *out.offset((5 as libc::c_int as libc::c_long * stride) as isize) = {
        let mut init = AVComplexDouble {
            re: x[4 as libc::c_int as usize].re - y[4 as libc::c_int as usize].im,
            im: x[4 as libc::c_int as usize].im + y[4 as libc::c_int as usize].re,
        };
        init
    };
    *out.offset((6 as libc::c_int as libc::c_long * stride) as isize) = {
        let mut init = AVComplexDouble {
            re: x[3 as libc::c_int as usize].re - y[3 as libc::c_int as usize].im,
            im: x[3 as libc::c_int as usize].im + y[3 as libc::c_int as usize].re,
        };
        init
    };
    *out.offset((7 as libc::c_int as libc::c_long * stride) as isize) = {
        let mut init = AVComplexDouble {
            re: x[2 as libc::c_int as usize].re - y[2 as libc::c_int as usize].im,
            im: x[2 as libc::c_int as usize].im + y[2 as libc::c_int as usize].re,
        };
        init
    };
    *out.offset((8 as libc::c_int as libc::c_long * stride) as isize) = {
        let mut init = AVComplexDouble {
            re: x[1 as libc::c_int as usize].re - y[1 as libc::c_int as usize].im,
            im: x[1 as libc::c_int as usize].im + y[1 as libc::c_int as usize].re,
        };
        init
    };
}
#[inline(always)]
unsafe extern "C" fn fft15(
    mut out: *mut TXComplex,
    mut in_0: *mut TXComplex,
    mut stride: ptrdiff_t,
) {
    let mut tmp: [TXComplex; 15] = [TXComplex { re: 0., im: 0. }; 15];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        fft3(
            tmp.as_mut_ptr().offset(i as isize),
            in_0.offset((i * 3 as libc::c_int) as isize),
            5 as libc::c_int as ptrdiff_t,
        );
        i += 1;
        i;
    }
    fft5_m1(
        out,
        tmp.as_mut_ptr().offset(0 as libc::c_int as isize),
        stride,
    );
    fft5_m2(
        out,
        tmp.as_mut_ptr().offset(5 as libc::c_int as isize),
        stride,
    );
    fft5_m3(
        out,
        tmp.as_mut_ptr().offset(10 as libc::c_int as isize),
        stride,
    );
}
#[cold]
unsafe extern "C" fn ff_tx_fft_factor_init_double_c(
    mut s: *mut AVTXContext,
    mut cd: *const FFTXCodelet,
    mut flags: uint64_t,
    mut opts: *mut FFTXCodeletOptions,
    mut len: libc::c_int,
    mut inv: libc::c_int,
    mut scale: *const libc::c_void,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    ff_tx_init_tabs_double(len);
    if len == 15 as libc::c_int {
        ret = ff_tx_gen_pfa_input_map(s, opts, 3 as libc::c_int, 5 as libc::c_int);
    } else if flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 61 as libc::c_int != 0 {
        ret = ff_tx_gen_default_map(s, opts);
    }
    return ret;
}
static mut ff_tx_fft3_fwd_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft3_fwd_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft3_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: (AV_TX_INPLACE as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 59 as libc::c_int) as uint64_t,
            factors: [
                3 as libc::c_int,
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 3 as libc::c_int,
            max_len: 3 as libc::c_int,
            init: Some(
                ff_tx_fft_factor_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
static mut ff_tx_fft3_ns_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft3_ns_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft3_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: (AV_TX_INPLACE as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 61 as libc::c_int) as uint64_t,
            factors: [
                3 as libc::c_int,
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 3 as libc::c_int,
            max_len: 3 as libc::c_int,
            init: Some(
                ff_tx_fft_factor_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn ff_tx_fft3_double_c(
    mut s: *mut AVTXContext,
    mut dst: *mut libc::c_void,
    mut src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    fft3(
        dst as *mut TXComplex,
        src as *mut TXComplex,
        (stride as libc::c_ulong).wrapping_div(::core::mem::size_of::<TXComplex>() as libc::c_ulong)
            as ptrdiff_t,
    );
}
static mut ff_tx_fft5_ns_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft5_ns_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft5_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: (AV_TX_INPLACE as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 61 as libc::c_int) as uint64_t,
            factors: [
                5 as libc::c_int,
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 5 as libc::c_int,
            max_len: 5 as libc::c_int,
            init: Some(
                ff_tx_fft_factor_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
static mut ff_tx_fft5_fwd_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft5_fwd_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft5_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: (AV_TX_INPLACE as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 59 as libc::c_int) as uint64_t,
            factors: [
                5 as libc::c_int,
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 5 as libc::c_int,
            max_len: 5 as libc::c_int,
            init: Some(
                ff_tx_fft_factor_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn ff_tx_fft5_double_c(
    mut s: *mut AVTXContext,
    mut dst: *mut libc::c_void,
    mut src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    fft5(
        dst as *mut TXComplex,
        src as *mut TXComplex,
        (stride as libc::c_ulong).wrapping_div(::core::mem::size_of::<TXComplex>() as libc::c_ulong)
            as ptrdiff_t,
    );
}
static mut ff_tx_fft7_fwd_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft7_fwd_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft7_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: (AV_TX_INPLACE as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 59 as libc::c_int) as uint64_t,
            factors: [
                7 as libc::c_int,
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 7 as libc::c_int,
            max_len: 7 as libc::c_int,
            init: Some(
                ff_tx_fft_factor_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn ff_tx_fft7_double_c(
    mut s: *mut AVTXContext,
    mut dst: *mut libc::c_void,
    mut src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    fft7(
        dst as *mut TXComplex,
        src as *mut TXComplex,
        (stride as libc::c_ulong).wrapping_div(::core::mem::size_of::<TXComplex>() as libc::c_ulong)
            as ptrdiff_t,
    );
}
static mut ff_tx_fft7_ns_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft7_ns_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft7_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: (AV_TX_INPLACE as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 61 as libc::c_int) as uint64_t,
            factors: [
                7 as libc::c_int,
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 7 as libc::c_int,
            max_len: 7 as libc::c_int,
            init: Some(
                ff_tx_fft_factor_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
static mut ff_tx_fft9_fwd_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft9_fwd_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft9_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: (AV_TX_INPLACE as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 59 as libc::c_int) as uint64_t,
            factors: [
                9 as libc::c_int,
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 9 as libc::c_int,
            max_len: 9 as libc::c_int,
            init: Some(
                ff_tx_fft_factor_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
static mut ff_tx_fft9_ns_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft9_ns_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft9_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: (AV_TX_INPLACE as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 61 as libc::c_int) as uint64_t,
            factors: [
                9 as libc::c_int,
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 9 as libc::c_int,
            max_len: 9 as libc::c_int,
            init: Some(
                ff_tx_fft_factor_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn ff_tx_fft9_double_c(
    mut s: *mut AVTXContext,
    mut dst: *mut libc::c_void,
    mut src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    fft9(
        dst as *mut TXComplex,
        src as *mut TXComplex,
        (stride as libc::c_ulong).wrapping_div(::core::mem::size_of::<TXComplex>() as libc::c_ulong)
            as ptrdiff_t,
    );
}
static mut ff_tx_fft15_ns_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft15_ns_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft15_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: (AV_TX_INPLACE as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 61 as libc::c_int) as uint64_t,
            factors: [
                15 as libc::c_int,
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 15 as libc::c_int,
            max_len: 15 as libc::c_int,
            init: Some(
                ff_tx_fft_factor_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn ff_tx_fft15_double_c(
    mut s: *mut AVTXContext,
    mut dst: *mut libc::c_void,
    mut src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    fft15(
        dst as *mut TXComplex,
        src as *mut TXComplex,
        (stride as libc::c_ulong).wrapping_div(::core::mem::size_of::<TXComplex>() as libc::c_ulong)
            as ptrdiff_t,
    );
}
#[inline]
unsafe extern "C" fn ff_tx_fft_sr_combine_double_c(
    mut z: *mut TXComplex,
    mut cos_0: *const TXSample,
    mut len: libc::c_int,
) {
    let mut o1: libc::c_int = 2 as libc::c_int * len;
    let mut o2: libc::c_int = 4 as libc::c_int * len;
    let mut o3: libc::c_int = 6 as libc::c_int * len;
    let mut wim: *const TXSample = cos_0
        .offset(o1 as isize)
        .offset(-(7 as libc::c_int as isize));
    let mut t1: TXUSample = 0.;
    let mut t2: TXUSample = 0.;
    let mut t3: TXUSample = 0.;
    let mut t4: TXUSample = 0.;
    let mut t5: TXUSample = 0.;
    let mut t6: TXUSample = 0.;
    let mut r0: TXUSample = 0.;
    let mut i0: TXUSample = 0.;
    let mut r1: TXUSample = 0.;
    let mut i1: TXUSample = 0.;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < len {
        t1 = (*z.offset((o2 + 0 as libc::c_int) as isize)).re
            * *cos_0.offset(0 as libc::c_int as isize)
            - (*z.offset((o2 + 0 as libc::c_int) as isize)).im
                * -*wim.offset(7 as libc::c_int as isize);
        t2 = (*z.offset((o2 + 0 as libc::c_int) as isize)).re
            * -*wim.offset(7 as libc::c_int as isize)
            + (*z.offset((o2 + 0 as libc::c_int) as isize)).im
                * *cos_0.offset(0 as libc::c_int as isize);
        t5 = (*z.offset((o3 + 0 as libc::c_int) as isize)).re
            * *cos_0.offset(0 as libc::c_int as isize)
            - (*z.offset((o3 + 0 as libc::c_int) as isize)).im
                * *wim.offset(7 as libc::c_int as isize);
        t6 = (*z.offset((o3 + 0 as libc::c_int) as isize)).re
            * *wim.offset(7 as libc::c_int as isize)
            + (*z.offset((o3 + 0 as libc::c_int) as isize)).im
                * *cos_0.offset(0 as libc::c_int as isize);
        r0 = (*z.offset(0 as libc::c_int as isize)).re;
        i0 = (*z.offset(0 as libc::c_int as isize)).im;
        r1 = (*z.offset((o1 + 0 as libc::c_int) as isize)).re;
        i1 = (*z.offset((o1 + 0 as libc::c_int) as isize)).im;
        t3 = t5 - t1;
        t5 = t5 + t1;
        (*z.offset((o2 + 0 as libc::c_int) as isize)).re = r0 - t5;
        (*z.offset(0 as libc::c_int as isize)).re = r0 + t5;
        (*z.offset((o3 + 0 as libc::c_int) as isize)).im = i1 - t3;
        (*z.offset((o1 + 0 as libc::c_int) as isize)).im = i1 + t3;
        t4 = t2 - t6;
        t6 = t2 + t6;
        (*z.offset((o3 + 0 as libc::c_int) as isize)).re = r1 - t4;
        (*z.offset((o1 + 0 as libc::c_int) as isize)).re = r1 + t4;
        (*z.offset((o2 + 0 as libc::c_int) as isize)).im = i0 - t6;
        (*z.offset(0 as libc::c_int as isize)).im = i0 + t6;
        t1 = (*z.offset((o2 + 2 as libc::c_int) as isize)).re
            * *cos_0.offset(2 as libc::c_int as isize)
            - (*z.offset((o2 + 2 as libc::c_int) as isize)).im
                * -*wim.offset(5 as libc::c_int as isize);
        t2 = (*z.offset((o2 + 2 as libc::c_int) as isize)).re
            * -*wim.offset(5 as libc::c_int as isize)
            + (*z.offset((o2 + 2 as libc::c_int) as isize)).im
                * *cos_0.offset(2 as libc::c_int as isize);
        t5 = (*z.offset((o3 + 2 as libc::c_int) as isize)).re
            * *cos_0.offset(2 as libc::c_int as isize)
            - (*z.offset((o3 + 2 as libc::c_int) as isize)).im
                * *wim.offset(5 as libc::c_int as isize);
        t6 = (*z.offset((o3 + 2 as libc::c_int) as isize)).re
            * *wim.offset(5 as libc::c_int as isize)
            + (*z.offset((o3 + 2 as libc::c_int) as isize)).im
                * *cos_0.offset(2 as libc::c_int as isize);
        r0 = (*z.offset(2 as libc::c_int as isize)).re;
        i0 = (*z.offset(2 as libc::c_int as isize)).im;
        r1 = (*z.offset((o1 + 2 as libc::c_int) as isize)).re;
        i1 = (*z.offset((o1 + 2 as libc::c_int) as isize)).im;
        t3 = t5 - t1;
        t5 = t5 + t1;
        (*z.offset((o2 + 2 as libc::c_int) as isize)).re = r0 - t5;
        (*z.offset(2 as libc::c_int as isize)).re = r0 + t5;
        (*z.offset((o3 + 2 as libc::c_int) as isize)).im = i1 - t3;
        (*z.offset((o1 + 2 as libc::c_int) as isize)).im = i1 + t3;
        t4 = t2 - t6;
        t6 = t2 + t6;
        (*z.offset((o3 + 2 as libc::c_int) as isize)).re = r1 - t4;
        (*z.offset((o1 + 2 as libc::c_int) as isize)).re = r1 + t4;
        (*z.offset((o2 + 2 as libc::c_int) as isize)).im = i0 - t6;
        (*z.offset(2 as libc::c_int as isize)).im = i0 + t6;
        t1 = (*z.offset((o2 + 4 as libc::c_int) as isize)).re
            * *cos_0.offset(4 as libc::c_int as isize)
            - (*z.offset((o2 + 4 as libc::c_int) as isize)).im
                * -*wim.offset(3 as libc::c_int as isize);
        t2 = (*z.offset((o2 + 4 as libc::c_int) as isize)).re
            * -*wim.offset(3 as libc::c_int as isize)
            + (*z.offset((o2 + 4 as libc::c_int) as isize)).im
                * *cos_0.offset(4 as libc::c_int as isize);
        t5 = (*z.offset((o3 + 4 as libc::c_int) as isize)).re
            * *cos_0.offset(4 as libc::c_int as isize)
            - (*z.offset((o3 + 4 as libc::c_int) as isize)).im
                * *wim.offset(3 as libc::c_int as isize);
        t6 = (*z.offset((o3 + 4 as libc::c_int) as isize)).re
            * *wim.offset(3 as libc::c_int as isize)
            + (*z.offset((o3 + 4 as libc::c_int) as isize)).im
                * *cos_0.offset(4 as libc::c_int as isize);
        r0 = (*z.offset(4 as libc::c_int as isize)).re;
        i0 = (*z.offset(4 as libc::c_int as isize)).im;
        r1 = (*z.offset((o1 + 4 as libc::c_int) as isize)).re;
        i1 = (*z.offset((o1 + 4 as libc::c_int) as isize)).im;
        t3 = t5 - t1;
        t5 = t5 + t1;
        (*z.offset((o2 + 4 as libc::c_int) as isize)).re = r0 - t5;
        (*z.offset(4 as libc::c_int as isize)).re = r0 + t5;
        (*z.offset((o3 + 4 as libc::c_int) as isize)).im = i1 - t3;
        (*z.offset((o1 + 4 as libc::c_int) as isize)).im = i1 + t3;
        t4 = t2 - t6;
        t6 = t2 + t6;
        (*z.offset((o3 + 4 as libc::c_int) as isize)).re = r1 - t4;
        (*z.offset((o1 + 4 as libc::c_int) as isize)).re = r1 + t4;
        (*z.offset((o2 + 4 as libc::c_int) as isize)).im = i0 - t6;
        (*z.offset(4 as libc::c_int as isize)).im = i0 + t6;
        t1 = (*z.offset((o2 + 6 as libc::c_int) as isize)).re
            * *cos_0.offset(6 as libc::c_int as isize)
            - (*z.offset((o2 + 6 as libc::c_int) as isize)).im
                * -*wim.offset(1 as libc::c_int as isize);
        t2 = (*z.offset((o2 + 6 as libc::c_int) as isize)).re
            * -*wim.offset(1 as libc::c_int as isize)
            + (*z.offset((o2 + 6 as libc::c_int) as isize)).im
                * *cos_0.offset(6 as libc::c_int as isize);
        t5 = (*z.offset((o3 + 6 as libc::c_int) as isize)).re
            * *cos_0.offset(6 as libc::c_int as isize)
            - (*z.offset((o3 + 6 as libc::c_int) as isize)).im
                * *wim.offset(1 as libc::c_int as isize);
        t6 = (*z.offset((o3 + 6 as libc::c_int) as isize)).re
            * *wim.offset(1 as libc::c_int as isize)
            + (*z.offset((o3 + 6 as libc::c_int) as isize)).im
                * *cos_0.offset(6 as libc::c_int as isize);
        r0 = (*z.offset(6 as libc::c_int as isize)).re;
        i0 = (*z.offset(6 as libc::c_int as isize)).im;
        r1 = (*z.offset((o1 + 6 as libc::c_int) as isize)).re;
        i1 = (*z.offset((o1 + 6 as libc::c_int) as isize)).im;
        t3 = t5 - t1;
        t5 = t5 + t1;
        (*z.offset((o2 + 6 as libc::c_int) as isize)).re = r0 - t5;
        (*z.offset(6 as libc::c_int as isize)).re = r0 + t5;
        (*z.offset((o3 + 6 as libc::c_int) as isize)).im = i1 - t3;
        (*z.offset((o1 + 6 as libc::c_int) as isize)).im = i1 + t3;
        t4 = t2 - t6;
        t6 = t2 + t6;
        (*z.offset((o3 + 6 as libc::c_int) as isize)).re = r1 - t4;
        (*z.offset((o1 + 6 as libc::c_int) as isize)).re = r1 + t4;
        (*z.offset((o2 + 6 as libc::c_int) as isize)).im = i0 - t6;
        (*z.offset(6 as libc::c_int as isize)).im = i0 + t6;
        t1 = (*z.offset((o2 + 1 as libc::c_int) as isize)).re
            * *cos_0.offset(1 as libc::c_int as isize)
            - (*z.offset((o2 + 1 as libc::c_int) as isize)).im
                * -*wim.offset(6 as libc::c_int as isize);
        t2 = (*z.offset((o2 + 1 as libc::c_int) as isize)).re
            * -*wim.offset(6 as libc::c_int as isize)
            + (*z.offset((o2 + 1 as libc::c_int) as isize)).im
                * *cos_0.offset(1 as libc::c_int as isize);
        t5 = (*z.offset((o3 + 1 as libc::c_int) as isize)).re
            * *cos_0.offset(1 as libc::c_int as isize)
            - (*z.offset((o3 + 1 as libc::c_int) as isize)).im
                * *wim.offset(6 as libc::c_int as isize);
        t6 = (*z.offset((o3 + 1 as libc::c_int) as isize)).re
            * *wim.offset(6 as libc::c_int as isize)
            + (*z.offset((o3 + 1 as libc::c_int) as isize)).im
                * *cos_0.offset(1 as libc::c_int as isize);
        r0 = (*z.offset(1 as libc::c_int as isize)).re;
        i0 = (*z.offset(1 as libc::c_int as isize)).im;
        r1 = (*z.offset((o1 + 1 as libc::c_int) as isize)).re;
        i1 = (*z.offset((o1 + 1 as libc::c_int) as isize)).im;
        t3 = t5 - t1;
        t5 = t5 + t1;
        (*z.offset((o2 + 1 as libc::c_int) as isize)).re = r0 - t5;
        (*z.offset(1 as libc::c_int as isize)).re = r0 + t5;
        (*z.offset((o3 + 1 as libc::c_int) as isize)).im = i1 - t3;
        (*z.offset((o1 + 1 as libc::c_int) as isize)).im = i1 + t3;
        t4 = t2 - t6;
        t6 = t2 + t6;
        (*z.offset((o3 + 1 as libc::c_int) as isize)).re = r1 - t4;
        (*z.offset((o1 + 1 as libc::c_int) as isize)).re = r1 + t4;
        (*z.offset((o2 + 1 as libc::c_int) as isize)).im = i0 - t6;
        (*z.offset(1 as libc::c_int as isize)).im = i0 + t6;
        t1 = (*z.offset((o2 + 3 as libc::c_int) as isize)).re
            * *cos_0.offset(3 as libc::c_int as isize)
            - (*z.offset((o2 + 3 as libc::c_int) as isize)).im
                * -*wim.offset(4 as libc::c_int as isize);
        t2 = (*z.offset((o2 + 3 as libc::c_int) as isize)).re
            * -*wim.offset(4 as libc::c_int as isize)
            + (*z.offset((o2 + 3 as libc::c_int) as isize)).im
                * *cos_0.offset(3 as libc::c_int as isize);
        t5 = (*z.offset((o3 + 3 as libc::c_int) as isize)).re
            * *cos_0.offset(3 as libc::c_int as isize)
            - (*z.offset((o3 + 3 as libc::c_int) as isize)).im
                * *wim.offset(4 as libc::c_int as isize);
        t6 = (*z.offset((o3 + 3 as libc::c_int) as isize)).re
            * *wim.offset(4 as libc::c_int as isize)
            + (*z.offset((o3 + 3 as libc::c_int) as isize)).im
                * *cos_0.offset(3 as libc::c_int as isize);
        r0 = (*z.offset(3 as libc::c_int as isize)).re;
        i0 = (*z.offset(3 as libc::c_int as isize)).im;
        r1 = (*z.offset((o1 + 3 as libc::c_int) as isize)).re;
        i1 = (*z.offset((o1 + 3 as libc::c_int) as isize)).im;
        t3 = t5 - t1;
        t5 = t5 + t1;
        (*z.offset((o2 + 3 as libc::c_int) as isize)).re = r0 - t5;
        (*z.offset(3 as libc::c_int as isize)).re = r0 + t5;
        (*z.offset((o3 + 3 as libc::c_int) as isize)).im = i1 - t3;
        (*z.offset((o1 + 3 as libc::c_int) as isize)).im = i1 + t3;
        t4 = t2 - t6;
        t6 = t2 + t6;
        (*z.offset((o3 + 3 as libc::c_int) as isize)).re = r1 - t4;
        (*z.offset((o1 + 3 as libc::c_int) as isize)).re = r1 + t4;
        (*z.offset((o2 + 3 as libc::c_int) as isize)).im = i0 - t6;
        (*z.offset(3 as libc::c_int as isize)).im = i0 + t6;
        t1 = (*z.offset((o2 + 5 as libc::c_int) as isize)).re
            * *cos_0.offset(5 as libc::c_int as isize)
            - (*z.offset((o2 + 5 as libc::c_int) as isize)).im
                * -*wim.offset(2 as libc::c_int as isize);
        t2 = (*z.offset((o2 + 5 as libc::c_int) as isize)).re
            * -*wim.offset(2 as libc::c_int as isize)
            + (*z.offset((o2 + 5 as libc::c_int) as isize)).im
                * *cos_0.offset(5 as libc::c_int as isize);
        t5 = (*z.offset((o3 + 5 as libc::c_int) as isize)).re
            * *cos_0.offset(5 as libc::c_int as isize)
            - (*z.offset((o3 + 5 as libc::c_int) as isize)).im
                * *wim.offset(2 as libc::c_int as isize);
        t6 = (*z.offset((o3 + 5 as libc::c_int) as isize)).re
            * *wim.offset(2 as libc::c_int as isize)
            + (*z.offset((o3 + 5 as libc::c_int) as isize)).im
                * *cos_0.offset(5 as libc::c_int as isize);
        r0 = (*z.offset(5 as libc::c_int as isize)).re;
        i0 = (*z.offset(5 as libc::c_int as isize)).im;
        r1 = (*z.offset((o1 + 5 as libc::c_int) as isize)).re;
        i1 = (*z.offset((o1 + 5 as libc::c_int) as isize)).im;
        t3 = t5 - t1;
        t5 = t5 + t1;
        (*z.offset((o2 + 5 as libc::c_int) as isize)).re = r0 - t5;
        (*z.offset(5 as libc::c_int as isize)).re = r0 + t5;
        (*z.offset((o3 + 5 as libc::c_int) as isize)).im = i1 - t3;
        (*z.offset((o1 + 5 as libc::c_int) as isize)).im = i1 + t3;
        t4 = t2 - t6;
        t6 = t2 + t6;
        (*z.offset((o3 + 5 as libc::c_int) as isize)).re = r1 - t4;
        (*z.offset((o1 + 5 as libc::c_int) as isize)).re = r1 + t4;
        (*z.offset((o2 + 5 as libc::c_int) as isize)).im = i0 - t6;
        (*z.offset(5 as libc::c_int as isize)).im = i0 + t6;
        t1 = (*z.offset((o2 + 7 as libc::c_int) as isize)).re
            * *cos_0.offset(7 as libc::c_int as isize)
            - (*z.offset((o2 + 7 as libc::c_int) as isize)).im
                * -*wim.offset(0 as libc::c_int as isize);
        t2 = (*z.offset((o2 + 7 as libc::c_int) as isize)).re
            * -*wim.offset(0 as libc::c_int as isize)
            + (*z.offset((o2 + 7 as libc::c_int) as isize)).im
                * *cos_0.offset(7 as libc::c_int as isize);
        t5 = (*z.offset((o3 + 7 as libc::c_int) as isize)).re
            * *cos_0.offset(7 as libc::c_int as isize)
            - (*z.offset((o3 + 7 as libc::c_int) as isize)).im
                * *wim.offset(0 as libc::c_int as isize);
        t6 = (*z.offset((o3 + 7 as libc::c_int) as isize)).re
            * *wim.offset(0 as libc::c_int as isize)
            + (*z.offset((o3 + 7 as libc::c_int) as isize)).im
                * *cos_0.offset(7 as libc::c_int as isize);
        r0 = (*z.offset(7 as libc::c_int as isize)).re;
        i0 = (*z.offset(7 as libc::c_int as isize)).im;
        r1 = (*z.offset((o1 + 7 as libc::c_int) as isize)).re;
        i1 = (*z.offset((o1 + 7 as libc::c_int) as isize)).im;
        t3 = t5 - t1;
        t5 = t5 + t1;
        (*z.offset((o2 + 7 as libc::c_int) as isize)).re = r0 - t5;
        (*z.offset(7 as libc::c_int as isize)).re = r0 + t5;
        (*z.offset((o3 + 7 as libc::c_int) as isize)).im = i1 - t3;
        (*z.offset((o1 + 7 as libc::c_int) as isize)).im = i1 + t3;
        t4 = t2 - t6;
        t6 = t2 + t6;
        (*z.offset((o3 + 7 as libc::c_int) as isize)).re = r1 - t4;
        (*z.offset((o1 + 7 as libc::c_int) as isize)).re = r1 + t4;
        (*z.offset((o2 + 7 as libc::c_int) as isize)).im = i0 - t6;
        (*z.offset(7 as libc::c_int as isize)).im = i0 + t6;
        z = z.offset((2 as libc::c_int * 4 as libc::c_int) as isize);
        cos_0 = cos_0.offset((2 as libc::c_int * 4 as libc::c_int) as isize);
        wim = wim.offset(-((2 as libc::c_int * 4 as libc::c_int) as isize));
        i += 4 as libc::c_int;
    }
}
#[cold]
unsafe extern "C" fn ff_tx_fft_sr_codelet_init_double_c(
    mut s: *mut AVTXContext,
    mut cd: *const FFTXCodelet,
    mut flags: uint64_t,
    mut opts: *mut FFTXCodeletOptions,
    mut len: libc::c_int,
    mut inv: libc::c_int,
    mut scale: *const libc::c_void,
) -> libc::c_int {
    ff_tx_init_tabs_double(len);
    return ff_tx_gen_ptwo_revtab(s, opts);
}
unsafe extern "C" fn ff_tx_fft2_ns_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut src: *mut TXComplex = _src as *mut TXComplex;
    let mut dst: *mut TXComplex = _dst as *mut TXComplex;
    let mut tmp: TXComplex = TXComplex { re: 0., im: 0. };
    tmp.re =
        (*src.offset(0 as libc::c_int as isize)).re - (*src.offset(1 as libc::c_int as isize)).re;
    (*dst.offset(0 as libc::c_int as isize)).re =
        (*src.offset(0 as libc::c_int as isize)).re + (*src.offset(1 as libc::c_int as isize)).re;
    tmp.im =
        (*src.offset(0 as libc::c_int as isize)).im - (*src.offset(1 as libc::c_int as isize)).im;
    (*dst.offset(0 as libc::c_int as isize)).im =
        (*src.offset(0 as libc::c_int as isize)).im + (*src.offset(1 as libc::c_int as isize)).im;
    *dst.offset(1 as libc::c_int as isize) = tmp;
}
unsafe extern "C" fn ff_tx_fft4_ns_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut src: *mut TXComplex = _src as *mut TXComplex;
    let mut dst: *mut TXComplex = _dst as *mut TXComplex;
    let mut t1: TXSample = 0.;
    let mut t2: TXSample = 0.;
    let mut t3: TXSample = 0.;
    let mut t4: TXSample = 0.;
    let mut t5: TXSample = 0.;
    let mut t6: TXSample = 0.;
    let mut t7: TXSample = 0.;
    let mut t8: TXSample = 0.;
    t3 = (*src.offset(0 as libc::c_int as isize)).re - (*src.offset(1 as libc::c_int as isize)).re;
    t1 = (*src.offset(0 as libc::c_int as isize)).re + (*src.offset(1 as libc::c_int as isize)).re;
    t8 = (*src.offset(3 as libc::c_int as isize)).re - (*src.offset(2 as libc::c_int as isize)).re;
    t6 = (*src.offset(3 as libc::c_int as isize)).re + (*src.offset(2 as libc::c_int as isize)).re;
    (*dst.offset(2 as libc::c_int as isize)).re = t1 - t6;
    (*dst.offset(0 as libc::c_int as isize)).re = t1 + t6;
    t4 = (*src.offset(0 as libc::c_int as isize)).im - (*src.offset(1 as libc::c_int as isize)).im;
    t2 = (*src.offset(0 as libc::c_int as isize)).im + (*src.offset(1 as libc::c_int as isize)).im;
    t7 = (*src.offset(2 as libc::c_int as isize)).im - (*src.offset(3 as libc::c_int as isize)).im;
    t5 = (*src.offset(2 as libc::c_int as isize)).im + (*src.offset(3 as libc::c_int as isize)).im;
    (*dst.offset(3 as libc::c_int as isize)).im = t4 - t8;
    (*dst.offset(1 as libc::c_int as isize)).im = t4 + t8;
    (*dst.offset(3 as libc::c_int as isize)).re = t3 - t7;
    (*dst.offset(1 as libc::c_int as isize)).re = t3 + t7;
    (*dst.offset(2 as libc::c_int as isize)).im = t2 - t5;
    (*dst.offset(0 as libc::c_int as isize)).im = t2 + t5;
}
unsafe extern "C" fn ff_tx_fft8_ns_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut src: *mut TXComplex = _src as *mut TXComplex;
    let mut dst: *mut TXComplex = _dst as *mut TXComplex;
    let mut t1: TXUSample = 0.;
    let mut t2: TXUSample = 0.;
    let mut t3: TXUSample = 0.;
    let mut t4: TXUSample = 0.;
    let mut t5: TXUSample = 0.;
    let mut t6: TXUSample = 0.;
    let mut r0: TXUSample = 0.;
    let mut i0: TXUSample = 0.;
    let mut r1: TXUSample = 0.;
    let mut i1: TXUSample = 0.;
    let cos_0: TXSample = ff_tx_tab_8_double[1 as libc::c_int as usize];
    ff_tx_fft4_ns_double_c(
        s,
        dst as *mut libc::c_void,
        src as *mut libc::c_void,
        stride,
    );
    t1 = (*src.offset(4 as libc::c_int as isize)).re - -(*src.offset(5 as libc::c_int as isize)).re;
    (*dst.offset(5 as libc::c_int as isize)).re =
        (*src.offset(4 as libc::c_int as isize)).re + -(*src.offset(5 as libc::c_int as isize)).re;
    t2 = (*src.offset(4 as libc::c_int as isize)).im - -(*src.offset(5 as libc::c_int as isize)).im;
    (*dst.offset(5 as libc::c_int as isize)).im =
        (*src.offset(4 as libc::c_int as isize)).im + -(*src.offset(5 as libc::c_int as isize)).im;
    t5 = (*src.offset(6 as libc::c_int as isize)).re - -(*src.offset(7 as libc::c_int as isize)).re;
    (*dst.offset(7 as libc::c_int as isize)).re =
        (*src.offset(6 as libc::c_int as isize)).re + -(*src.offset(7 as libc::c_int as isize)).re;
    t6 = (*src.offset(6 as libc::c_int as isize)).im - -(*src.offset(7 as libc::c_int as isize)).im;
    (*dst.offset(7 as libc::c_int as isize)).im =
        (*src.offset(6 as libc::c_int as isize)).im + -(*src.offset(7 as libc::c_int as isize)).im;
    r0 = (*dst.offset(0 as libc::c_int as isize)).re;
    i0 = (*dst.offset(0 as libc::c_int as isize)).im;
    r1 = (*dst.offset(2 as libc::c_int as isize)).re;
    i1 = (*dst.offset(2 as libc::c_int as isize)).im;
    t3 = t5 - t1;
    t5 = t5 + t1;
    (*dst.offset(4 as libc::c_int as isize)).re = r0 - t5;
    (*dst.offset(0 as libc::c_int as isize)).re = r0 + t5;
    (*dst.offset(6 as libc::c_int as isize)).im = i1 - t3;
    (*dst.offset(2 as libc::c_int as isize)).im = i1 + t3;
    t4 = t2 - t6;
    t6 = t2 + t6;
    (*dst.offset(6 as libc::c_int as isize)).re = r1 - t4;
    (*dst.offset(2 as libc::c_int as isize)).re = r1 + t4;
    (*dst.offset(4 as libc::c_int as isize)).im = i0 - t6;
    (*dst.offset(0 as libc::c_int as isize)).im = i0 + t6;
    t1 = (*dst.offset(5 as libc::c_int as isize)).re * cos_0
        - (*dst.offset(5 as libc::c_int as isize)).im * -cos_0;
    t2 = (*dst.offset(5 as libc::c_int as isize)).re * -cos_0
        + (*dst.offset(5 as libc::c_int as isize)).im * cos_0;
    t5 = (*dst.offset(7 as libc::c_int as isize)).re * cos_0
        - (*dst.offset(7 as libc::c_int as isize)).im * cos_0;
    t6 = (*dst.offset(7 as libc::c_int as isize)).re * cos_0
        + (*dst.offset(7 as libc::c_int as isize)).im * cos_0;
    r0 = (*dst.offset(1 as libc::c_int as isize)).re;
    i0 = (*dst.offset(1 as libc::c_int as isize)).im;
    r1 = (*dst.offset(3 as libc::c_int as isize)).re;
    i1 = (*dst.offset(3 as libc::c_int as isize)).im;
    t3 = t5 - t1;
    t5 = t5 + t1;
    (*dst.offset(5 as libc::c_int as isize)).re = r0 - t5;
    (*dst.offset(1 as libc::c_int as isize)).re = r0 + t5;
    (*dst.offset(7 as libc::c_int as isize)).im = i1 - t3;
    (*dst.offset(3 as libc::c_int as isize)).im = i1 + t3;
    t4 = t2 - t6;
    t6 = t2 + t6;
    (*dst.offset(7 as libc::c_int as isize)).re = r1 - t4;
    (*dst.offset(3 as libc::c_int as isize)).re = r1 + t4;
    (*dst.offset(5 as libc::c_int as isize)).im = i0 - t6;
    (*dst.offset(1 as libc::c_int as isize)).im = i0 + t6;
}
unsafe extern "C" fn ff_tx_fft16_ns_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut src: *mut TXComplex = _src as *mut TXComplex;
    let mut dst: *mut TXComplex = _dst as *mut TXComplex;
    let mut cos_0: *const TXSample = ff_tx_tab_16_double.as_mut_ptr();
    let mut t1: TXUSample = 0.;
    let mut t2: TXUSample = 0.;
    let mut t3: TXUSample = 0.;
    let mut t4: TXUSample = 0.;
    let mut t5: TXUSample = 0.;
    let mut t6: TXUSample = 0.;
    let mut r0: TXUSample = 0.;
    let mut i0: TXUSample = 0.;
    let mut r1: TXUSample = 0.;
    let mut i1: TXUSample = 0.;
    let mut cos_16_1: TXSample = *cos_0.offset(1 as libc::c_int as isize);
    let mut cos_16_2: TXSample = *cos_0.offset(2 as libc::c_int as isize);
    let mut cos_16_3: TXSample = *cos_0.offset(3 as libc::c_int as isize);
    ff_tx_fft8_ns_double_c(
        s,
        dst.offset(0 as libc::c_int as isize) as *mut libc::c_void,
        src.offset(0 as libc::c_int as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft4_ns_double_c(
        s,
        dst.offset(8 as libc::c_int as isize) as *mut libc::c_void,
        src.offset(8 as libc::c_int as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft4_ns_double_c(
        s,
        dst.offset(12 as libc::c_int as isize) as *mut libc::c_void,
        src.offset(12 as libc::c_int as isize) as *mut libc::c_void,
        stride,
    );
    t1 = (*dst.offset(8 as libc::c_int as isize)).re;
    t2 = (*dst.offset(8 as libc::c_int as isize)).im;
    t5 = (*dst.offset(12 as libc::c_int as isize)).re;
    t6 = (*dst.offset(12 as libc::c_int as isize)).im;
    r0 = (*dst.offset(0 as libc::c_int as isize)).re;
    i0 = (*dst.offset(0 as libc::c_int as isize)).im;
    r1 = (*dst.offset(4 as libc::c_int as isize)).re;
    i1 = (*dst.offset(4 as libc::c_int as isize)).im;
    t3 = t5 - t1;
    t5 = t5 + t1;
    (*dst.offset(8 as libc::c_int as isize)).re = r0 - t5;
    (*dst.offset(0 as libc::c_int as isize)).re = r0 + t5;
    (*dst.offset(12 as libc::c_int as isize)).im = i1 - t3;
    (*dst.offset(4 as libc::c_int as isize)).im = i1 + t3;
    t4 = t2 - t6;
    t6 = t2 + t6;
    (*dst.offset(12 as libc::c_int as isize)).re = r1 - t4;
    (*dst.offset(4 as libc::c_int as isize)).re = r1 + t4;
    (*dst.offset(8 as libc::c_int as isize)).im = i0 - t6;
    (*dst.offset(0 as libc::c_int as isize)).im = i0 + t6;
    t1 = (*dst.offset(10 as libc::c_int as isize)).re * cos_16_2
        - (*dst.offset(10 as libc::c_int as isize)).im * -cos_16_2;
    t2 = (*dst.offset(10 as libc::c_int as isize)).re * -cos_16_2
        + (*dst.offset(10 as libc::c_int as isize)).im * cos_16_2;
    t5 = (*dst.offset(14 as libc::c_int as isize)).re * cos_16_2
        - (*dst.offset(14 as libc::c_int as isize)).im * cos_16_2;
    t6 = (*dst.offset(14 as libc::c_int as isize)).re * cos_16_2
        + (*dst.offset(14 as libc::c_int as isize)).im * cos_16_2;
    r0 = (*dst.offset(2 as libc::c_int as isize)).re;
    i0 = (*dst.offset(2 as libc::c_int as isize)).im;
    r1 = (*dst.offset(6 as libc::c_int as isize)).re;
    i1 = (*dst.offset(6 as libc::c_int as isize)).im;
    t3 = t5 - t1;
    t5 = t5 + t1;
    (*dst.offset(10 as libc::c_int as isize)).re = r0 - t5;
    (*dst.offset(2 as libc::c_int as isize)).re = r0 + t5;
    (*dst.offset(14 as libc::c_int as isize)).im = i1 - t3;
    (*dst.offset(6 as libc::c_int as isize)).im = i1 + t3;
    t4 = t2 - t6;
    t6 = t2 + t6;
    (*dst.offset(14 as libc::c_int as isize)).re = r1 - t4;
    (*dst.offset(6 as libc::c_int as isize)).re = r1 + t4;
    (*dst.offset(10 as libc::c_int as isize)).im = i0 - t6;
    (*dst.offset(2 as libc::c_int as isize)).im = i0 + t6;
    t1 = (*dst.offset(9 as libc::c_int as isize)).re * cos_16_1
        - (*dst.offset(9 as libc::c_int as isize)).im * -cos_16_3;
    t2 = (*dst.offset(9 as libc::c_int as isize)).re * -cos_16_3
        + (*dst.offset(9 as libc::c_int as isize)).im * cos_16_1;
    t5 = (*dst.offset(13 as libc::c_int as isize)).re * cos_16_1
        - (*dst.offset(13 as libc::c_int as isize)).im * cos_16_3;
    t6 = (*dst.offset(13 as libc::c_int as isize)).re * cos_16_3
        + (*dst.offset(13 as libc::c_int as isize)).im * cos_16_1;
    r0 = (*dst.offset(1 as libc::c_int as isize)).re;
    i0 = (*dst.offset(1 as libc::c_int as isize)).im;
    r1 = (*dst.offset(5 as libc::c_int as isize)).re;
    i1 = (*dst.offset(5 as libc::c_int as isize)).im;
    t3 = t5 - t1;
    t5 = t5 + t1;
    (*dst.offset(9 as libc::c_int as isize)).re = r0 - t5;
    (*dst.offset(1 as libc::c_int as isize)).re = r0 + t5;
    (*dst.offset(13 as libc::c_int as isize)).im = i1 - t3;
    (*dst.offset(5 as libc::c_int as isize)).im = i1 + t3;
    t4 = t2 - t6;
    t6 = t2 + t6;
    (*dst.offset(13 as libc::c_int as isize)).re = r1 - t4;
    (*dst.offset(5 as libc::c_int as isize)).re = r1 + t4;
    (*dst.offset(9 as libc::c_int as isize)).im = i0 - t6;
    (*dst.offset(1 as libc::c_int as isize)).im = i0 + t6;
    t1 = (*dst.offset(11 as libc::c_int as isize)).re * cos_16_3
        - (*dst.offset(11 as libc::c_int as isize)).im * -cos_16_1;
    t2 = (*dst.offset(11 as libc::c_int as isize)).re * -cos_16_1
        + (*dst.offset(11 as libc::c_int as isize)).im * cos_16_3;
    t5 = (*dst.offset(15 as libc::c_int as isize)).re * cos_16_3
        - (*dst.offset(15 as libc::c_int as isize)).im * cos_16_1;
    t6 = (*dst.offset(15 as libc::c_int as isize)).re * cos_16_1
        + (*dst.offset(15 as libc::c_int as isize)).im * cos_16_3;
    r0 = (*dst.offset(3 as libc::c_int as isize)).re;
    i0 = (*dst.offset(3 as libc::c_int as isize)).im;
    r1 = (*dst.offset(7 as libc::c_int as isize)).re;
    i1 = (*dst.offset(7 as libc::c_int as isize)).im;
    t3 = t5 - t1;
    t5 = t5 + t1;
    (*dst.offset(11 as libc::c_int as isize)).re = r0 - t5;
    (*dst.offset(3 as libc::c_int as isize)).re = r0 + t5;
    (*dst.offset(15 as libc::c_int as isize)).im = i1 - t3;
    (*dst.offset(7 as libc::c_int as isize)).im = i1 + t3;
    t4 = t2 - t6;
    t6 = t2 + t6;
    (*dst.offset(15 as libc::c_int as isize)).re = r1 - t4;
    (*dst.offset(7 as libc::c_int as isize)).re = r1 + t4;
    (*dst.offset(11 as libc::c_int as isize)).im = i0 - t6;
    (*dst.offset(3 as libc::c_int as isize)).im = i0 + t6;
}
static mut ff_tx_fft2_ns_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft2_ns_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft2_ns_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: ((1 as libc::c_ulonglong) << 63 as libc::c_int
                | AV_TX_INPLACE as libc::c_int as libc::c_ulonglong
                | AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 61 as libc::c_int) as uint64_t,
            factors: [
                2 as libc::c_int,
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 2 as libc::c_int,
            max_len: 2 as libc::c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
static mut ff_tx_fft4_ns_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft4_ns_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft4_ns_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: ((1 as libc::c_ulonglong) << 63 as libc::c_int
                | AV_TX_INPLACE as libc::c_int as libc::c_ulonglong
                | AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 61 as libc::c_int) as uint64_t,
            factors: [
                2 as libc::c_int,
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 4 as libc::c_int,
            max_len: 4 as libc::c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
static mut ff_tx_fft8_ns_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft8_ns_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft8_ns_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: ((1 as libc::c_ulonglong) << 63 as libc::c_int
                | AV_TX_INPLACE as libc::c_int as libc::c_ulonglong
                | AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 61 as libc::c_int) as uint64_t,
            factors: [
                2 as libc::c_int,
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 8 as libc::c_int,
            max_len: 8 as libc::c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
static mut ff_tx_fft16_ns_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft16_ns_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft16_ns_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: ((1 as libc::c_ulonglong) << 63 as libc::c_int
                | AV_TX_INPLACE as libc::c_int as libc::c_ulonglong
                | AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 61 as libc::c_int) as uint64_t,
            factors: [
                2 as libc::c_int,
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 16 as libc::c_int,
            max_len: 16 as libc::c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
static mut ff_tx_fft32_ns_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft32_ns_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft32_ns_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: ((1 as libc::c_ulonglong) << 63 as libc::c_int
                | AV_TX_INPLACE as libc::c_int as libc::c_ulonglong
                | AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 61 as libc::c_int) as uint64_t,
            factors: [
                2 as libc::c_int,
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 32 as libc::c_int,
            max_len: 32 as libc::c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn ff_tx_fft32_ns_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut src: *mut TXComplex = _src as *mut TXComplex;
    let mut dst: *mut TXComplex = _dst as *mut TXComplex;
    let mut cos_0: *const TXSample = ff_tx_tab_32_double.as_mut_ptr();
    ff_tx_fft16_ns_double_c(
        s,
        dst as *mut libc::c_void,
        src as *mut libc::c_void,
        stride,
    );
    ff_tx_fft8_ns_double_c(
        s,
        dst.offset((8 as libc::c_int * 2 as libc::c_int) as isize) as *mut libc::c_void,
        src.offset((8 as libc::c_int * 2 as libc::c_int) as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft8_ns_double_c(
        s,
        dst.offset((8 as libc::c_int * 3 as libc::c_int) as isize) as *mut libc::c_void,
        src.offset((8 as libc::c_int * 3 as libc::c_int) as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft_sr_combine_double_c(dst, cos_0, 8 as libc::c_int >> 1 as libc::c_int);
}
static mut ff_tx_fft64_ns_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft64_ns_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft64_ns_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: ((1 as libc::c_ulonglong) << 63 as libc::c_int
                | AV_TX_INPLACE as libc::c_int as libc::c_ulonglong
                | AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 61 as libc::c_int) as uint64_t,
            factors: [
                2 as libc::c_int,
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 64 as libc::c_int,
            max_len: 64 as libc::c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn ff_tx_fft64_ns_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut src: *mut TXComplex = _src as *mut TXComplex;
    let mut dst: *mut TXComplex = _dst as *mut TXComplex;
    let mut cos_0: *const TXSample = ff_tx_tab_64_double.as_mut_ptr();
    ff_tx_fft32_ns_double_c(
        s,
        dst as *mut libc::c_void,
        src as *mut libc::c_void,
        stride,
    );
    ff_tx_fft16_ns_double_c(
        s,
        dst.offset((16 as libc::c_int * 2 as libc::c_int) as isize) as *mut libc::c_void,
        src.offset((16 as libc::c_int * 2 as libc::c_int) as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft16_ns_double_c(
        s,
        dst.offset((16 as libc::c_int * 3 as libc::c_int) as isize) as *mut libc::c_void,
        src.offset((16 as libc::c_int * 3 as libc::c_int) as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft_sr_combine_double_c(dst, cos_0, 16 as libc::c_int >> 1 as libc::c_int);
}
unsafe extern "C" fn ff_tx_fft128_ns_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut src: *mut TXComplex = _src as *mut TXComplex;
    let mut dst: *mut TXComplex = _dst as *mut TXComplex;
    let mut cos_0: *const TXSample = ff_tx_tab_128_double.as_mut_ptr();
    ff_tx_fft64_ns_double_c(
        s,
        dst as *mut libc::c_void,
        src as *mut libc::c_void,
        stride,
    );
    ff_tx_fft32_ns_double_c(
        s,
        dst.offset((32 as libc::c_int * 2 as libc::c_int) as isize) as *mut libc::c_void,
        src.offset((32 as libc::c_int * 2 as libc::c_int) as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft32_ns_double_c(
        s,
        dst.offset((32 as libc::c_int * 3 as libc::c_int) as isize) as *mut libc::c_void,
        src.offset((32 as libc::c_int * 3 as libc::c_int) as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft_sr_combine_double_c(dst, cos_0, 32 as libc::c_int >> 1 as libc::c_int);
}
static mut ff_tx_fft128_ns_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft128_ns_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft128_ns_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: ((1 as libc::c_ulonglong) << 63 as libc::c_int
                | AV_TX_INPLACE as libc::c_int as libc::c_ulonglong
                | AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 61 as libc::c_int) as uint64_t,
            factors: [
                2 as libc::c_int,
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 128 as libc::c_int,
            max_len: 128 as libc::c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn ff_tx_fft256_ns_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut src: *mut TXComplex = _src as *mut TXComplex;
    let mut dst: *mut TXComplex = _dst as *mut TXComplex;
    let mut cos_0: *const TXSample = ff_tx_tab_256_double.as_mut_ptr();
    ff_tx_fft128_ns_double_c(
        s,
        dst as *mut libc::c_void,
        src as *mut libc::c_void,
        stride,
    );
    ff_tx_fft64_ns_double_c(
        s,
        dst.offset((64 as libc::c_int * 2 as libc::c_int) as isize) as *mut libc::c_void,
        src.offset((64 as libc::c_int * 2 as libc::c_int) as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft64_ns_double_c(
        s,
        dst.offset((64 as libc::c_int * 3 as libc::c_int) as isize) as *mut libc::c_void,
        src.offset((64 as libc::c_int * 3 as libc::c_int) as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft_sr_combine_double_c(dst, cos_0, 64 as libc::c_int >> 1 as libc::c_int);
}
static mut ff_tx_fft256_ns_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft256_ns_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft256_ns_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: ((1 as libc::c_ulonglong) << 63 as libc::c_int
                | AV_TX_INPLACE as libc::c_int as libc::c_ulonglong
                | AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 61 as libc::c_int) as uint64_t,
            factors: [
                2 as libc::c_int,
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 256 as libc::c_int,
            max_len: 256 as libc::c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn ff_tx_fft512_ns_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut src: *mut TXComplex = _src as *mut TXComplex;
    let mut dst: *mut TXComplex = _dst as *mut TXComplex;
    let mut cos_0: *const TXSample = ff_tx_tab_512_double.as_mut_ptr();
    ff_tx_fft256_ns_double_c(
        s,
        dst as *mut libc::c_void,
        src as *mut libc::c_void,
        stride,
    );
    ff_tx_fft128_ns_double_c(
        s,
        dst.offset((128 as libc::c_int * 2 as libc::c_int) as isize) as *mut libc::c_void,
        src.offset((128 as libc::c_int * 2 as libc::c_int) as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft128_ns_double_c(
        s,
        dst.offset((128 as libc::c_int * 3 as libc::c_int) as isize) as *mut libc::c_void,
        src.offset((128 as libc::c_int * 3 as libc::c_int) as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft_sr_combine_double_c(dst, cos_0, 128 as libc::c_int >> 1 as libc::c_int);
}
static mut ff_tx_fft512_ns_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft512_ns_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft512_ns_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: ((1 as libc::c_ulonglong) << 63 as libc::c_int
                | AV_TX_INPLACE as libc::c_int as libc::c_ulonglong
                | AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 61 as libc::c_int) as uint64_t,
            factors: [
                2 as libc::c_int,
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 512 as libc::c_int,
            max_len: 512 as libc::c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
static mut ff_tx_fft1024_ns_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft1024_ns_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft1024_ns_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: ((1 as libc::c_ulonglong) << 63 as libc::c_int
                | AV_TX_INPLACE as libc::c_int as libc::c_ulonglong
                | AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 61 as libc::c_int) as uint64_t,
            factors: [
                2 as libc::c_int,
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 1024 as libc::c_int,
            max_len: 1024 as libc::c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn ff_tx_fft1024_ns_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut src: *mut TXComplex = _src as *mut TXComplex;
    let mut dst: *mut TXComplex = _dst as *mut TXComplex;
    let mut cos_0: *const TXSample = ff_tx_tab_1024_double.as_mut_ptr();
    ff_tx_fft512_ns_double_c(
        s,
        dst as *mut libc::c_void,
        src as *mut libc::c_void,
        stride,
    );
    ff_tx_fft256_ns_double_c(
        s,
        dst.offset((256 as libc::c_int * 2 as libc::c_int) as isize) as *mut libc::c_void,
        src.offset((256 as libc::c_int * 2 as libc::c_int) as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft256_ns_double_c(
        s,
        dst.offset((256 as libc::c_int * 3 as libc::c_int) as isize) as *mut libc::c_void,
        src.offset((256 as libc::c_int * 3 as libc::c_int) as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft_sr_combine_double_c(dst, cos_0, 256 as libc::c_int >> 1 as libc::c_int);
}
static mut ff_tx_fft2048_ns_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft2048_ns_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft2048_ns_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: ((1 as libc::c_ulonglong) << 63 as libc::c_int
                | AV_TX_INPLACE as libc::c_int as libc::c_ulonglong
                | AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 61 as libc::c_int) as uint64_t,
            factors: [
                2 as libc::c_int,
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 2048 as libc::c_int,
            max_len: 2048 as libc::c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn ff_tx_fft2048_ns_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut src: *mut TXComplex = _src as *mut TXComplex;
    let mut dst: *mut TXComplex = _dst as *mut TXComplex;
    let mut cos_0: *const TXSample = ff_tx_tab_2048_double.as_mut_ptr();
    ff_tx_fft1024_ns_double_c(
        s,
        dst as *mut libc::c_void,
        src as *mut libc::c_void,
        stride,
    );
    ff_tx_fft512_ns_double_c(
        s,
        dst.offset((512 as libc::c_int * 2 as libc::c_int) as isize) as *mut libc::c_void,
        src.offset((512 as libc::c_int * 2 as libc::c_int) as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft512_ns_double_c(
        s,
        dst.offset((512 as libc::c_int * 3 as libc::c_int) as isize) as *mut libc::c_void,
        src.offset((512 as libc::c_int * 3 as libc::c_int) as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft_sr_combine_double_c(dst, cos_0, 512 as libc::c_int >> 1 as libc::c_int);
}
unsafe extern "C" fn ff_tx_fft4096_ns_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut src: *mut TXComplex = _src as *mut TXComplex;
    let mut dst: *mut TXComplex = _dst as *mut TXComplex;
    let mut cos_0: *const TXSample = ff_tx_tab_4096_double.as_mut_ptr();
    ff_tx_fft2048_ns_double_c(
        s,
        dst as *mut libc::c_void,
        src as *mut libc::c_void,
        stride,
    );
    ff_tx_fft1024_ns_double_c(
        s,
        dst.offset((1024 as libc::c_int * 2 as libc::c_int) as isize) as *mut libc::c_void,
        src.offset((1024 as libc::c_int * 2 as libc::c_int) as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft1024_ns_double_c(
        s,
        dst.offset((1024 as libc::c_int * 3 as libc::c_int) as isize) as *mut libc::c_void,
        src.offset((1024 as libc::c_int * 3 as libc::c_int) as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft_sr_combine_double_c(dst, cos_0, 1024 as libc::c_int >> 1 as libc::c_int);
}
static mut ff_tx_fft4096_ns_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft4096_ns_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft4096_ns_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: ((1 as libc::c_ulonglong) << 63 as libc::c_int
                | AV_TX_INPLACE as libc::c_int as libc::c_ulonglong
                | AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 61 as libc::c_int) as uint64_t,
            factors: [
                2 as libc::c_int,
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 4096 as libc::c_int,
            max_len: 4096 as libc::c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
static mut ff_tx_fft8192_ns_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft8192_ns_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft8192_ns_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: ((1 as libc::c_ulonglong) << 63 as libc::c_int
                | AV_TX_INPLACE as libc::c_int as libc::c_ulonglong
                | AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 61 as libc::c_int) as uint64_t,
            factors: [
                2 as libc::c_int,
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 8192 as libc::c_int,
            max_len: 8192 as libc::c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn ff_tx_fft8192_ns_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut src: *mut TXComplex = _src as *mut TXComplex;
    let mut dst: *mut TXComplex = _dst as *mut TXComplex;
    let mut cos_0: *const TXSample = ff_tx_tab_8192_double.as_mut_ptr();
    ff_tx_fft4096_ns_double_c(
        s,
        dst as *mut libc::c_void,
        src as *mut libc::c_void,
        stride,
    );
    ff_tx_fft2048_ns_double_c(
        s,
        dst.offset((2048 as libc::c_int * 2 as libc::c_int) as isize) as *mut libc::c_void,
        src.offset((2048 as libc::c_int * 2 as libc::c_int) as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft2048_ns_double_c(
        s,
        dst.offset((2048 as libc::c_int * 3 as libc::c_int) as isize) as *mut libc::c_void,
        src.offset((2048 as libc::c_int * 3 as libc::c_int) as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft_sr_combine_double_c(dst, cos_0, 2048 as libc::c_int >> 1 as libc::c_int);
}
unsafe extern "C" fn ff_tx_fft16384_ns_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut src: *mut TXComplex = _src as *mut TXComplex;
    let mut dst: *mut TXComplex = _dst as *mut TXComplex;
    let mut cos_0: *const TXSample = ff_tx_tab_16384_double.as_mut_ptr();
    ff_tx_fft8192_ns_double_c(
        s,
        dst as *mut libc::c_void,
        src as *mut libc::c_void,
        stride,
    );
    ff_tx_fft4096_ns_double_c(
        s,
        dst.offset((4096 as libc::c_int * 2 as libc::c_int) as isize) as *mut libc::c_void,
        src.offset((4096 as libc::c_int * 2 as libc::c_int) as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft4096_ns_double_c(
        s,
        dst.offset((4096 as libc::c_int * 3 as libc::c_int) as isize) as *mut libc::c_void,
        src.offset((4096 as libc::c_int * 3 as libc::c_int) as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft_sr_combine_double_c(dst, cos_0, 4096 as libc::c_int >> 1 as libc::c_int);
}
static mut ff_tx_fft16384_ns_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft16384_ns_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft16384_ns_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: ((1 as libc::c_ulonglong) << 63 as libc::c_int
                | AV_TX_INPLACE as libc::c_int as libc::c_ulonglong
                | AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 61 as libc::c_int) as uint64_t,
            factors: [
                2 as libc::c_int,
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 16384 as libc::c_int,
            max_len: 16384 as libc::c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
static mut ff_tx_fft32768_ns_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft32768_ns_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft32768_ns_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: ((1 as libc::c_ulonglong) << 63 as libc::c_int
                | AV_TX_INPLACE as libc::c_int as libc::c_ulonglong
                | AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 61 as libc::c_int) as uint64_t,
            factors: [
                2 as libc::c_int,
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 32768 as libc::c_int,
            max_len: 32768 as libc::c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn ff_tx_fft32768_ns_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut src: *mut TXComplex = _src as *mut TXComplex;
    let mut dst: *mut TXComplex = _dst as *mut TXComplex;
    let mut cos_0: *const TXSample = ff_tx_tab_32768_double.as_mut_ptr();
    ff_tx_fft16384_ns_double_c(
        s,
        dst as *mut libc::c_void,
        src as *mut libc::c_void,
        stride,
    );
    ff_tx_fft8192_ns_double_c(
        s,
        dst.offset((8192 as libc::c_int * 2 as libc::c_int) as isize) as *mut libc::c_void,
        src.offset((8192 as libc::c_int * 2 as libc::c_int) as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft8192_ns_double_c(
        s,
        dst.offset((8192 as libc::c_int * 3 as libc::c_int) as isize) as *mut libc::c_void,
        src.offset((8192 as libc::c_int * 3 as libc::c_int) as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft_sr_combine_double_c(dst, cos_0, 8192 as libc::c_int >> 1 as libc::c_int);
}
unsafe extern "C" fn ff_tx_fft65536_ns_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut src: *mut TXComplex = _src as *mut TXComplex;
    let mut dst: *mut TXComplex = _dst as *mut TXComplex;
    let mut cos_0: *const TXSample = ff_tx_tab_65536_double.as_mut_ptr();
    ff_tx_fft32768_ns_double_c(
        s,
        dst as *mut libc::c_void,
        src as *mut libc::c_void,
        stride,
    );
    ff_tx_fft16384_ns_double_c(
        s,
        dst.offset((16384 as libc::c_int * 2 as libc::c_int) as isize) as *mut libc::c_void,
        src.offset((16384 as libc::c_int * 2 as libc::c_int) as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft16384_ns_double_c(
        s,
        dst.offset((16384 as libc::c_int * 3 as libc::c_int) as isize) as *mut libc::c_void,
        src.offset((16384 as libc::c_int * 3 as libc::c_int) as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft_sr_combine_double_c(dst, cos_0, 16384 as libc::c_int >> 1 as libc::c_int);
}
static mut ff_tx_fft65536_ns_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft65536_ns_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft65536_ns_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: ((1 as libc::c_ulonglong) << 63 as libc::c_int
                | AV_TX_INPLACE as libc::c_int as libc::c_ulonglong
                | AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 61 as libc::c_int) as uint64_t,
            factors: [
                2 as libc::c_int,
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 65536 as libc::c_int,
            max_len: 65536 as libc::c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn ff_tx_fft131072_ns_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut src: *mut TXComplex = _src as *mut TXComplex;
    let mut dst: *mut TXComplex = _dst as *mut TXComplex;
    let mut cos_0: *const TXSample = ff_tx_tab_131072_double.as_mut_ptr();
    ff_tx_fft65536_ns_double_c(
        s,
        dst as *mut libc::c_void,
        src as *mut libc::c_void,
        stride,
    );
    ff_tx_fft32768_ns_double_c(
        s,
        dst.offset((32768 as libc::c_int * 2 as libc::c_int) as isize) as *mut libc::c_void,
        src.offset((32768 as libc::c_int * 2 as libc::c_int) as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft32768_ns_double_c(
        s,
        dst.offset((32768 as libc::c_int * 3 as libc::c_int) as isize) as *mut libc::c_void,
        src.offset((32768 as libc::c_int * 3 as libc::c_int) as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft_sr_combine_double_c(dst, cos_0, 32768 as libc::c_int >> 1 as libc::c_int);
}
static mut ff_tx_fft131072_ns_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft131072_ns_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft131072_ns_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: ((1 as libc::c_ulonglong) << 63 as libc::c_int
                | AV_TX_INPLACE as libc::c_int as libc::c_ulonglong
                | AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 61 as libc::c_int) as uint64_t,
            factors: [
                2 as libc::c_int,
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 131072 as libc::c_int,
            max_len: 131072 as libc::c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn ff_tx_fft262144_ns_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut src: *mut TXComplex = _src as *mut TXComplex;
    let mut dst: *mut TXComplex = _dst as *mut TXComplex;
    let mut cos_0: *const TXSample = ff_tx_tab_262144_double.as_mut_ptr();
    ff_tx_fft131072_ns_double_c(
        s,
        dst as *mut libc::c_void,
        src as *mut libc::c_void,
        stride,
    );
    ff_tx_fft65536_ns_double_c(
        s,
        dst.offset((65536 as libc::c_int * 2 as libc::c_int) as isize) as *mut libc::c_void,
        src.offset((65536 as libc::c_int * 2 as libc::c_int) as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft65536_ns_double_c(
        s,
        dst.offset((65536 as libc::c_int * 3 as libc::c_int) as isize) as *mut libc::c_void,
        src.offset((65536 as libc::c_int * 3 as libc::c_int) as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft_sr_combine_double_c(dst, cos_0, 65536 as libc::c_int >> 1 as libc::c_int);
}
static mut ff_tx_fft262144_ns_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft262144_ns_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft262144_ns_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: ((1 as libc::c_ulonglong) << 63 as libc::c_int
                | AV_TX_INPLACE as libc::c_int as libc::c_ulonglong
                | AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 61 as libc::c_int) as uint64_t,
            factors: [
                2 as libc::c_int,
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 262144 as libc::c_int,
            max_len: 262144 as libc::c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
static mut ff_tx_fft524288_ns_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft524288_ns_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft524288_ns_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: ((1 as libc::c_ulonglong) << 63 as libc::c_int
                | AV_TX_INPLACE as libc::c_int as libc::c_ulonglong
                | AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 61 as libc::c_int) as uint64_t,
            factors: [
                2 as libc::c_int,
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 524288 as libc::c_int,
            max_len: 524288 as libc::c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn ff_tx_fft524288_ns_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut src: *mut TXComplex = _src as *mut TXComplex;
    let mut dst: *mut TXComplex = _dst as *mut TXComplex;
    let mut cos_0: *const TXSample = ff_tx_tab_524288_double.as_mut_ptr();
    ff_tx_fft262144_ns_double_c(
        s,
        dst as *mut libc::c_void,
        src as *mut libc::c_void,
        stride,
    );
    ff_tx_fft131072_ns_double_c(
        s,
        dst.offset((131072 as libc::c_int * 2 as libc::c_int) as isize) as *mut libc::c_void,
        src.offset((131072 as libc::c_int * 2 as libc::c_int) as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft131072_ns_double_c(
        s,
        dst.offset((131072 as libc::c_int * 3 as libc::c_int) as isize) as *mut libc::c_void,
        src.offset((131072 as libc::c_int * 3 as libc::c_int) as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft_sr_combine_double_c(dst, cos_0, 131072 as libc::c_int >> 1 as libc::c_int);
}
unsafe extern "C" fn ff_tx_fft1048576_ns_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut src: *mut TXComplex = _src as *mut TXComplex;
    let mut dst: *mut TXComplex = _dst as *mut TXComplex;
    let mut cos_0: *const TXSample = ff_tx_tab_1048576_double.as_mut_ptr();
    ff_tx_fft524288_ns_double_c(
        s,
        dst as *mut libc::c_void,
        src as *mut libc::c_void,
        stride,
    );
    ff_tx_fft262144_ns_double_c(
        s,
        dst.offset((262144 as libc::c_int * 2 as libc::c_int) as isize) as *mut libc::c_void,
        src.offset((262144 as libc::c_int * 2 as libc::c_int) as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft262144_ns_double_c(
        s,
        dst.offset((262144 as libc::c_int * 3 as libc::c_int) as isize) as *mut libc::c_void,
        src.offset((262144 as libc::c_int * 3 as libc::c_int) as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft_sr_combine_double_c(dst, cos_0, 262144 as libc::c_int >> 1 as libc::c_int);
}
static mut ff_tx_fft1048576_ns_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft1048576_ns_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft1048576_ns_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: ((1 as libc::c_ulonglong) << 63 as libc::c_int
                | AV_TX_INPLACE as libc::c_int as libc::c_ulonglong
                | AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 61 as libc::c_int) as uint64_t,
            factors: [
                2 as libc::c_int,
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 1048576 as libc::c_int,
            max_len: 1048576 as libc::c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn ff_tx_fft2097152_ns_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut src: *mut TXComplex = _src as *mut TXComplex;
    let mut dst: *mut TXComplex = _dst as *mut TXComplex;
    let mut cos_0: *const TXSample = ff_tx_tab_2097152_double.as_mut_ptr();
    ff_tx_fft1048576_ns_double_c(
        s,
        dst as *mut libc::c_void,
        src as *mut libc::c_void,
        stride,
    );
    ff_tx_fft524288_ns_double_c(
        s,
        dst.offset((524288 as libc::c_int * 2 as libc::c_int) as isize) as *mut libc::c_void,
        src.offset((524288 as libc::c_int * 2 as libc::c_int) as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft524288_ns_double_c(
        s,
        dst.offset((524288 as libc::c_int * 3 as libc::c_int) as isize) as *mut libc::c_void,
        src.offset((524288 as libc::c_int * 3 as libc::c_int) as isize) as *mut libc::c_void,
        stride,
    );
    ff_tx_fft_sr_combine_double_c(dst, cos_0, 524288 as libc::c_int >> 1 as libc::c_int);
}
static mut ff_tx_fft2097152_ns_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft2097152_ns_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft2097152_ns_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: ((1 as libc::c_ulonglong) << 63 as libc::c_int
                | AV_TX_INPLACE as libc::c_int as libc::c_ulonglong
                | AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 61 as libc::c_int) as uint64_t,
            factors: [
                2 as libc::c_int,
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 2097152 as libc::c_int,
            max_len: 2097152 as libc::c_int,
            init: Some(
                ff_tx_fft_sr_codelet_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
#[cold]
unsafe extern "C" fn ff_tx_fft_init_double_c(
    mut s: *mut AVTXContext,
    mut cd: *const FFTXCodelet,
    mut flags: uint64_t,
    mut opts: *mut FFTXCodeletOptions,
    mut len: libc::c_int,
    mut inv: libc::c_int,
    mut scale: *const libc::c_void,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut is_inplace: libc::c_int =
        (flags & AV_TX_INPLACE as libc::c_int as libc::c_ulong != 0) as libc::c_int;
    let mut sub_opts: FFTXCodeletOptions = {
        let mut init = FFTXCodeletOptions {
            map_dir: (if is_inplace != 0 {
                FF_TX_MAP_SCATTER as libc::c_int
            } else {
                FF_TX_MAP_GATHER as libc::c_int
            }) as FFTXMapDirection,
        };
        init
    };
    flags =
        (flags as libc::c_ulonglong & !((1 as libc::c_ulonglong) << 63 as libc::c_int)) as uint64_t;
    flags |= AV_TX_INPLACE as libc::c_int as libc::c_ulong;
    flags =
        (flags as libc::c_ulonglong | (1 as libc::c_ulonglong) << 61 as libc::c_int) as uint64_t;
    ret = ff_tx_init_subtx(s, AV_TX_DOUBLE_FFT, flags, &mut sub_opts, len, inv, scale);
    if ret != 0 {
        return ret;
    }
    if is_inplace != 0 && {
        ret = ff_tx_gen_inplace_map(s, len);
        ret != 0
    } {
        return ret;
    }
    return 0 as libc::c_int;
}
#[cold]
unsafe extern "C" fn ff_tx_fft_inplace_small_init_double_c(
    mut s: *mut AVTXContext,
    mut cd: *const FFTXCodelet,
    mut flags: uint64_t,
    mut opts: *mut FFTXCodeletOptions,
    mut len: libc::c_int,
    mut inv: libc::c_int,
    mut scale: *const libc::c_void,
) -> libc::c_int {
    (*s).tmp = AVTXNum {
        double: av_malloc(
            (len as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<TXComplex>() as libc::c_ulong),
        ) as *mut TXComplex,
    };
    if ((*s).tmp).double.is_null() {
        return -(12 as libc::c_int);
    }
    flags &= !(AV_TX_INPLACE as libc::c_int) as libc::c_ulong;
    return ff_tx_fft_init_double_c(s, cd, flags, opts, len, inv, scale);
}
unsafe extern "C" fn ff_tx_fft_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut src: *mut TXComplex = _src as *mut TXComplex;
    let mut dst1: *mut TXComplex =
        (if (*s).flags & AV_TX_INPLACE as libc::c_int as libc::c_ulong != 0 {
            (*s).tmp.double as *mut libc::c_void
        } else {
            _dst
        }) as *mut TXComplex;
    let mut dst2: *mut TXComplex = _dst as *mut TXComplex;
    let mut map: *mut libc::c_int = (*((*s).sub).offset(0 as libc::c_int as isize)).map;
    let mut len: libc::c_int = (*s).len;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < len {
        *dst1.offset(i as isize) = *src.offset(*map.offset(i as isize) as isize);
        i += 1;
        i;
    }
    ((*s).fn_0[0 as libc::c_int as usize]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0 as libc::c_int as isize),
        dst2 as *mut libc::c_void,
        dst1 as *mut libc::c_void,
        stride,
    );
}
unsafe extern "C" fn ff_tx_fft_inplace_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut src: *mut TXComplex = _src as *mut TXComplex;
    let mut dst: *mut TXComplex = _dst as *mut TXComplex;
    let mut tmp: TXComplex = TXComplex { re: 0., im: 0. };
    let mut map: *const libc::c_int = (*(*s).sub).map;
    let mut inplace_idx: *const libc::c_int = (*s).map;
    let mut src_idx: libc::c_int = 0;
    let mut dst_idx: libc::c_int = 0;
    let fresh20 = inplace_idx;
    inplace_idx = inplace_idx.offset(1);
    src_idx = *fresh20;
    loop {
        tmp = *src.offset(src_idx as isize);
        dst_idx = *map.offset(src_idx as isize);
        loop {
            let mut SWAP_tmp: TXComplex = *src.offset(dst_idx as isize);
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
    ((*s).fn_0[0 as libc::c_int as usize]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0 as libc::c_int as isize),
        dst as *mut libc::c_void,
        src as *mut libc::c_void,
        stride,
    );
}
static mut ff_tx_fft_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: (AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int) as uint64_t,
            factors: [
                -(1 as libc::c_int),
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 2 as libc::c_int,
            max_len: -(1 as libc::c_int),
            init: Some(
                ff_tx_fft_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
static mut ff_tx_fft_inplace_small_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft_inplace_small_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: (AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | AV_TX_INPLACE as libc::c_int as libc::c_ulonglong) as uint64_t,
            factors: [
                -(1 as libc::c_int),
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 2 as libc::c_int,
            max_len: 65536 as libc::c_int,
            init: Some(
                ff_tx_fft_inplace_small_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int - 256 as libc::c_int,
        };
        init
    }
};
static mut ff_tx_fft_inplace_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft_inplace_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft_inplace_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: (AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | AV_TX_INPLACE as libc::c_int as libc::c_ulonglong) as uint64_t,
            factors: [
                -(1 as libc::c_int),
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 2 as libc::c_int,
            max_len: -(1 as libc::c_int),
            init: Some(
                ff_tx_fft_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int - 512 as libc::c_int,
        };
        init
    }
};
#[cold]
unsafe extern "C" fn ff_tx_fft_init_naive_small_double_c(
    mut s: *mut AVTXContext,
    mut cd: *const FFTXCodelet,
    mut flags: uint64_t,
    mut opts: *mut FFTXCodeletOptions,
    mut len: libc::c_int,
    mut inv: libc::c_int,
    mut scale: *const libc::c_void,
) -> libc::c_int {
    let phase: libc::c_double = if (*s).inv != 0 {
        2.0f64 * 3.14159265358979323846f64 / len as libc::c_double
    } else {
        -2.0f64 * 3.14159265358979323846f64 / len as libc::c_double
    };
    (*s).exp = AVTXNum {
        double: av_malloc(
            ((len * len) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<TXComplex>() as libc::c_ulong),
        ) as *mut TXComplex,
    };
    if ((*s).exp).double.is_null() {
        return -(12 as libc::c_int);
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < len {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < len {
            let factor: libc::c_double = phase * i as libc::c_double * j as libc::c_double;
            *(((*s).exp).double.offset((i * j) as isize)) = {
                let mut init = AVComplexDouble {
                    re: cos(factor),
                    im: sin(factor),
                };
                init
            };
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ff_tx_fft_naive_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut src: *mut TXComplex = _src as *mut TXComplex;
    let mut dst: *mut TXComplex = _dst as *mut TXComplex;
    let n: libc::c_int = (*s).len;
    let mut phase: libc::c_double = if (*s).inv != 0 {
        2.0f64 * 3.14159265358979323846f64 / n as libc::c_double
    } else {
        -2.0f64 * 3.14159265358979323846f64 / n as libc::c_double
    };
    stride = (stride as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<TXComplex>() as libc::c_ulong)
        as ptrdiff_t as ptrdiff_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        let mut tmp: TXComplex = {
            let mut init = AVComplexDouble {
                re: 0 as libc::c_int as libc::c_double,
                im: 0.,
            };
            init
        };
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < n {
            let factor: libc::c_double = phase * i as libc::c_double * j as libc::c_double;
            let mult: TXComplex = {
                let mut init = AVComplexDouble {
                    re: cos(factor),
                    im: sin(factor),
                };
                init
            };
            let mut res: TXComplex = TXComplex { re: 0., im: 0. };
            res.re =
                (*src.offset(j as isize)).re * mult.re - (*src.offset(j as isize)).im * mult.im;
            res.im =
                (*src.offset(j as isize)).re * mult.im + (*src.offset(j as isize)).im * mult.re;
            tmp.re += res.re;
            tmp.im += res.im;
            j += 1;
            j;
        }
        *dst.offset((i as libc::c_long * stride) as isize) = tmp;
        i += 1;
        i;
    }
}
unsafe extern "C" fn ff_tx_fft_naive_small_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut src: *mut TXComplex = _src as *mut TXComplex;
    let mut dst: *mut TXComplex = _dst as *mut TXComplex;
    let n: libc::c_int = (*s).len;
    stride = (stride as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<TXComplex>() as libc::c_ulong)
        as ptrdiff_t as ptrdiff_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        let mut tmp: TXComplex = {
            let mut init = AVComplexDouble {
                re: 0 as libc::c_int as libc::c_double,
                im: 0.,
            };
            init
        };
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < n {
            let mut res: TXComplex = TXComplex { re: 0., im: 0. };
            let mult: TXComplex = *(((*s).exp).double.offset((i * j) as isize) as *const TXComplex);
            res.re =
                (*src.offset(j as isize)).re * mult.re - (*src.offset(j as isize)).im * mult.im;
            res.im =
                (*src.offset(j as isize)).re * mult.im + (*src.offset(j as isize)).im * mult.re;
            tmp.re += res.re;
            tmp.im += res.im;
            j += 1;
            j;
        }
        *dst.offset((i as libc::c_long * stride) as isize) = tmp;
        i += 1;
        i;
    }
}
static mut ff_tx_fft_naive_small_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft_naive_small_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft_naive_small_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: (AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int) as uint64_t,
            factors: [
                -(1 as libc::c_int),
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 2 as libc::c_int,
            max_len: 1024 as libc::c_int,
            init: Some(
                ff_tx_fft_init_naive_small_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_MIN as libc::c_int / 2 as libc::c_int,
        };
        init
    }
};
static mut ff_tx_fft_naive_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft_naive_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft_naive_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: (AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int) as uint64_t,
            factors: [
                -(1 as libc::c_int),
                0,
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 1 as libc::c_int,
            min_len: 2 as libc::c_int,
            max_len: -(1 as libc::c_int),
            init: None,
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_MIN as libc::c_int,
        };
        init
    }
};
#[cold]
unsafe extern "C" fn ff_tx_fft_pfa_init_double_c(
    mut s: *mut AVTXContext,
    mut cd: *const FFTXCodelet,
    mut flags: uint64_t,
    mut opts: *mut FFTXCodeletOptions,
    mut len: libc::c_int,
    mut inv: libc::c_int,
    mut scale: *const libc::c_void,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ps: libc::c_int =
        (flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 61 as libc::c_int) as libc::c_int;
    let mut sub_opts: FFTXCodeletOptions = {
        let mut init = FFTXCodeletOptions {
            map_dir: FF_TX_MAP_GATHER,
        };
        init
    };
    let mut extra_tmp_len: size_t = 0 as libc::c_int as size_t;
    let mut len_list: [libc::c_int; 512] = [0; 512];
    ret = ff_tx_decompose_length(len_list.as_mut_ptr(), AV_TX_DOUBLE_FFT, len, inv);
    if ret < 0 as libc::c_int {
        return ret;
    }
    let mut current_block_30: u64;
    let mut i: libc::c_int = 0 as libc::c_int;
    's_17: while i < ret {
        let mut len1: libc::c_int = len_list[i as usize];
        let mut len2: libc::c_int = len / len1;
        if len2 & len2 - 1 as libc::c_int != 0 {
            let mut SWAP_tmp: libc::c_int = len2;
            len2 = len1;
            len1 = SWAP_tmp;
        }
        ff_tx_clear_ctx(s);
        sub_opts.map_dir = FF_TX_MAP_GATHER;
        flags &= !(AV_TX_INPLACE as libc::c_int) as libc::c_ulong;
        flags = (flags as libc::c_ulonglong | (1 as libc::c_ulonglong) << 63 as libc::c_int)
            as uint64_t;
        flags = (flags as libc::c_ulonglong | (1 as libc::c_ulonglong) << 61 as libc::c_int)
            as uint64_t;
        ret = ff_tx_init_subtx(s, AV_TX_DOUBLE_FFT, flags, &mut sub_opts, len1, inv, scale);
        if ret == -(12 as libc::c_int) {
            return ret;
        } else {
            if ret < 0 as libc::c_int {
                flags = (flags as libc::c_ulonglong
                    & !((1 as libc::c_ulonglong) << 61 as libc::c_int))
                    as uint64_t;
                ret = ff_tx_init_subtx(s, AV_TX_DOUBLE_FFT, flags, &mut sub_opts, len1, inv, scale);
                if ret == -(12 as libc::c_int) {
                    return ret;
                } else if ret < 0 as libc::c_int {
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
                    flags = (flags as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << 61 as libc::c_int)
                        as uint64_t;
                    loop {
                        flags = (flags as libc::c_ulonglong
                            & !((1 as libc::c_ulonglong) << 63 as libc::c_int))
                            as uint64_t;
                        flags |= AV_TX_INPLACE as libc::c_int as libc::c_ulong;
                        ret = ff_tx_init_subtx(
                            s,
                            AV_TX_DOUBLE_FFT,
                            flags,
                            &mut sub_opts,
                            len2,
                            inv,
                            scale,
                        );
                        if ret == -(12 as libc::c_int) {
                            return ret;
                        } else {
                            if !(ret < 0 as libc::c_int) {
                                break 's_17;
                            }
                            flags = (flags as libc::c_ulonglong
                                | (1 as libc::c_ulonglong) << 63 as libc::c_int)
                                as uint64_t;
                            flags &= !(AV_TX_INPLACE as libc::c_int) as libc::c_ulong;
                            ret = ff_tx_init_subtx(
                                s,
                                AV_TX_DOUBLE_FFT,
                                flags,
                                &mut sub_opts,
                                len2,
                                inv,
                                scale,
                            );
                            if ret == -(12 as libc::c_int) {
                                return ret;
                            } else {
                                if !(ret < 0 as libc::c_int) {
                                    break 's_17;
                                }
                                if !(flags as libc::c_ulonglong
                                    & (1 as libc::c_ulonglong) << 61 as libc::c_int
                                    != 0)
                                {
                                    break;
                                }
                                flags = (flags as libc::c_ulonglong
                                    & !((1 as libc::c_ulonglong) << 61 as libc::c_int))
                                    as uint64_t;
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
    if ret < 0 as libc::c_int {
        return ret;
    }
    ret = ff_tx_gen_compound_mapping(
        s,
        opts,
        0 as libc::c_int,
        (*((*s).sub).offset(0 as libc::c_int as isize)).len,
        (*((*s).sub).offset(1 as libc::c_int as isize)).len,
    );
    if ret != 0 {
        return ret;
    }
    (*s).tmp = AVTXNum {
        double: av_malloc(
            (len as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<TXComplex>() as libc::c_ulong),
        ) as *mut TXComplex,
    };
    if ((*s).tmp).double.is_null() {
        return -(12 as libc::c_int);
    }
    tmp = (*s).tmp.double as *mut libc::c_int;
    let mut k: libc::c_int = 0 as libc::c_int;
    while k < len {
        memcpy(
            tmp as *mut libc::c_void,
            &mut *((*s).map).offset(k as isize) as *mut libc::c_int as *const libc::c_void,
            ((*((*s).sub).offset(0 as libc::c_int as isize)).len as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < (*((*s).sub).offset(0 as libc::c_int as isize)).len {
            *((*s).map).offset((k + i_0) as isize) = *tmp.offset(
                *((*((*s).sub).offset(0 as libc::c_int as isize)).map).offset(i_0 as isize)
                    as isize,
            );
            i_0 += 1;
            i_0;
        }
        k += (*((*s).sub).offset(0 as libc::c_int as isize)).len;
    }
    if (*((*s).sub).offset(1 as libc::c_int as isize)).flags
        & AV_TX_INPLACE as libc::c_int as libc::c_ulong
        == 0
    {
        extra_tmp_len = len as size_t;
    } else if ps == 0 {
        extra_tmp_len = (*((*s).sub).offset(0 as libc::c_int as isize)).len as size_t;
    }
    if extra_tmp_len != 0 && {
        (*s).exp = AVTXNum {
            double: av_malloc(
                extra_tmp_len.wrapping_mul(::core::mem::size_of::<TXComplex>() as libc::c_ulong),
            ) as *mut TXComplex,
        };
        ((*s).exp).double.is_null()
    } {
        return -(12 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ff_tx_fft_pfa_double_c(
    mut s: *mut AVTXContext,
    mut _out: *mut libc::c_void,
    mut _in: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let n: libc::c_int = (*((*s).sub).offset(0 as libc::c_int as isize)).len;
    let m: libc::c_int = (*((*s).sub).offset(1 as libc::c_int as isize)).len;
    let l: libc::c_int = (*s).len;
    let mut in_map: *const libc::c_int = (*s).map;
    let mut out_map: *const libc::c_int = in_map.offset(l as isize);
    let mut sub_map: *const libc::c_int = (*((*s).sub).offset(1 as libc::c_int as isize)).map;
    let mut tmp1: *mut TXComplex = if (*((*s).sub).offset(1 as libc::c_int as isize)).flags
        & AV_TX_INPLACE as libc::c_int as libc::c_ulong
        != 0
    {
        (*s).tmp.double
    } else {
        (*s).exp.double
    };
    let mut in_0: *mut TXComplex = _in as *mut TXComplex;
    let mut out: *mut TXComplex = _out as *mut TXComplex;
    stride = (stride as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<TXComplex>() as libc::c_ulong)
        as ptrdiff_t as ptrdiff_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < m {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < n {
            *(((*s).exp).double.offset(j as isize) as *mut AVComplexDouble) =
                *in_0.offset(*in_map.offset((i * n + j) as isize) as isize);
            j += 1;
            j;
        }
        ((*s).fn_0[0 as libc::c_int as usize]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0 as libc::c_int as isize),
            &mut *((*s).tmp)
                .double
                .offset(*sub_map.offset(i as isize) as isize) as *mut TXComplex
                as *mut libc::c_void,
            (*s).exp.double as *mut libc::c_void,
            (m as libc::c_ulong).wrapping_mul(::core::mem::size_of::<TXComplex>() as libc::c_ulong)
                as ptrdiff_t,
        );
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < n {
        ((*s).fn_0[1 as libc::c_int as usize]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(1 as libc::c_int as isize),
            &mut *tmp1.offset((m * i_0) as isize) as *mut TXComplex as *mut libc::c_void,
            &mut *((*s).tmp).double.offset((m * i_0) as isize) as *mut TXComplex
                as *mut libc::c_void,
            ::core::mem::size_of::<TXComplex>() as libc::c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < l {
        *out.offset((i_1 as libc::c_long * stride) as isize) =
            *tmp1.offset(*out_map.offset(i_1 as isize) as isize);
        i_1 += 1;
        i_1;
    }
}
unsafe extern "C" fn ff_tx_fft_pfa_ns_double_c(
    mut s: *mut AVTXContext,
    mut _out: *mut libc::c_void,
    mut _in: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let n: libc::c_int = (*((*s).sub).offset(0 as libc::c_int as isize)).len;
    let m: libc::c_int = (*((*s).sub).offset(1 as libc::c_int as isize)).len;
    let l: libc::c_int = (*s).len;
    let mut in_map: *const libc::c_int = (*s).map;
    let mut out_map: *const libc::c_int = in_map.offset(l as isize);
    let mut sub_map: *const libc::c_int = (*((*s).sub).offset(1 as libc::c_int as isize)).map;
    let mut tmp1: *mut TXComplex = if (*((*s).sub).offset(1 as libc::c_int as isize)).flags
        & AV_TX_INPLACE as libc::c_int as libc::c_ulong
        != 0
    {
        (*s).tmp.double
    } else {
        (*s).exp.double
    };
    let mut in_0: *mut TXComplex = _in as *mut TXComplex;
    let mut out: *mut TXComplex = _out as *mut TXComplex;
    stride = (stride as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<TXComplex>() as libc::c_ulong)
        as ptrdiff_t as ptrdiff_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < m {
        ((*s).fn_0[0 as libc::c_int as usize]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0 as libc::c_int as isize),
            &mut *((*s).tmp)
                .double
                .offset(*sub_map.offset(i as isize) as isize) as *mut TXComplex
                as *mut libc::c_void,
            &mut *in_0.offset((i * n) as isize) as *mut TXComplex as *mut libc::c_void,
            (m as libc::c_ulong).wrapping_mul(::core::mem::size_of::<TXComplex>() as libc::c_ulong)
                as ptrdiff_t,
        );
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < n {
        ((*s).fn_0[1 as libc::c_int as usize]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(1 as libc::c_int as isize),
            &mut *tmp1.offset((m * i_0) as isize) as *mut TXComplex as *mut libc::c_void,
            &mut *((*s).tmp).double.offset((m * i_0) as isize) as *mut TXComplex
                as *mut libc::c_void,
            ::core::mem::size_of::<TXComplex>() as libc::c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < l {
        *out.offset((i_1 as libc::c_long * stride) as isize) =
            *tmp1.offset(*out_map.offset(i_1 as isize) as isize);
        i_1 += 1;
        i_1;
    }
}
static mut ff_tx_fft_pfa_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft_pfa_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft_pfa_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: ((AV_TX_UNALIGNED as libc::c_int | AV_TX_INPLACE as libc::c_int)
                as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int) as uint64_t,
            factors: [
                7 as libc::c_int,
                5 as libc::c_int,
                3 as libc::c_int,
                2 as libc::c_int,
                -(1 as libc::c_int),
                0,
                0,
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
            nb_factors: 2 as libc::c_int,
            min_len: 2 as libc::c_int * 3 as libc::c_int,
            max_len: -(1 as libc::c_int),
            init: Some(
                ff_tx_fft_pfa_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
static mut ff_tx_fft_pfa_ns_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"fft_pfa_ns_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_fft_pfa_ns_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_FFT,
            flags: ((AV_TX_UNALIGNED as libc::c_int | AV_TX_INPLACE as libc::c_int)
                as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | (1 as libc::c_ulonglong) << 61 as libc::c_int) as uint64_t,
            factors: [
                7 as libc::c_int,
                5 as libc::c_int,
                3 as libc::c_int,
                2 as libc::c_int,
                -(1 as libc::c_int),
                0,
                0,
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
            nb_factors: 2 as libc::c_int,
            min_len: 2 as libc::c_int * 3 as libc::c_int,
            max_len: -(1 as libc::c_int),
            init: Some(
                ff_tx_fft_pfa_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
#[cold]
unsafe extern "C" fn ff_tx_mdct_naive_init_double_c(
    mut s: *mut AVTXContext,
    mut cd: *const FFTXCodelet,
    mut flags: uint64_t,
    mut opts: *mut FFTXCodeletOptions,
    mut len: libc::c_int,
    mut inv: libc::c_int,
    mut scale: *const libc::c_void,
) -> libc::c_int {
    (*s).scale_d = *(scale as *mut libc::c_double);
    (*s).scale_f = (*s).scale_d as libc::c_float;
    return 0 as libc::c_int;
}
unsafe extern "C" fn ff_tx_mdct_naive_fwd_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut src: *mut TXSample = _src as *mut TXSample;
    let mut dst: *mut TXSample = _dst as *mut TXSample;
    let mut scale: libc::c_double = (*s).scale_d;
    let mut len: libc::c_int = (*s).len;
    let phase: libc::c_double = 3.14159265358979323846f64 / (4.0f64 * len as libc::c_double);
    stride = (stride as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<TXSample>() as libc::c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < len {
        let mut sum: libc::c_double = 0.0f64;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < len * 2 as libc::c_int {
            let mut a: libc::c_int = (2 as libc::c_int * j + 1 as libc::c_int + len)
                * (2 as libc::c_int * i + 1 as libc::c_int);
            sum += *src.offset(j as isize) * cos(a as libc::c_double * phase);
            j += 1;
            j;
        }
        *dst.offset((i as libc::c_long * stride) as isize) = sum * scale;
        i += 1;
        i;
    }
}
unsafe extern "C" fn ff_tx_mdct_naive_inv_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut src: *mut TXSample = _src as *mut TXSample;
    let mut dst: *mut TXSample = _dst as *mut TXSample;
    let mut scale: libc::c_double = (*s).scale_d;
    let mut len: libc::c_int = (*s).len >> 1 as libc::c_int;
    let mut len2: libc::c_int = len * 2 as libc::c_int;
    let phase: libc::c_double = 3.14159265358979323846f64 / (4.0f64 * len2 as libc::c_double);
    stride = (stride as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<TXSample>() as libc::c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < len {
        let mut sum_d: libc::c_double = 0.0f64;
        let mut sum_u: libc::c_double = 0.0f64;
        let mut i_d: libc::c_double = phase
            * (4 as libc::c_int * len - 2 as libc::c_int * i - 1 as libc::c_int) as libc::c_double;
        let mut i_u: libc::c_double = phase
            * (3 as libc::c_int * len2 + 2 as libc::c_int * i + 1 as libc::c_int) as libc::c_double;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < len2 {
            let mut a: libc::c_double = (2 as libc::c_int * j + 1 as libc::c_int) as libc::c_double;
            let mut a_d: libc::c_double = cos(a * i_d);
            let mut a_u: libc::c_double = cos(a * i_u);
            let mut val: libc::c_double = *src.offset((j as libc::c_long * stride) as isize);
            sum_d += a_d * val;
            sum_u += a_u * val;
            j += 1;
            j;
        }
        *dst.offset((i + 0 as libc::c_int) as isize) = sum_d * scale;
        *dst.offset((i + len) as isize) = -sum_u * scale;
        i += 1;
        i;
    }
}
static mut ff_tx_mdct_naive_fwd_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"mdct_naive_fwd_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_mdct_naive_fwd_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_MDCT,
            flags: (AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | (1 as libc::c_ulonglong) << 59 as libc::c_int) as uint64_t,
            factors: [
                2 as libc::c_int,
                -(1 as libc::c_int),
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 2 as libc::c_int,
            min_len: 2 as libc::c_int,
            max_len: -(1 as libc::c_int),
            init: Some(
                ff_tx_mdct_naive_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_MIN as libc::c_int,
        };
        init
    }
};
static mut ff_tx_mdct_naive_inv_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"mdct_naive_inv_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_mdct_naive_inv_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_MDCT,
            flags: (AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | (1 as libc::c_ulonglong) << 60 as libc::c_int) as uint64_t,
            factors: [
                2 as libc::c_int,
                -(1 as libc::c_int),
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 2 as libc::c_int,
            min_len: 2 as libc::c_int,
            max_len: -(1 as libc::c_int),
            init: Some(
                ff_tx_mdct_naive_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_MIN as libc::c_int,
        };
        init
    }
};
#[cold]
unsafe extern "C" fn ff_tx_mdct_init_double_c(
    mut s: *mut AVTXContext,
    mut cd: *const FFTXCodelet,
    mut flags: uint64_t,
    mut opts: *mut FFTXCodeletOptions,
    mut len: libc::c_int,
    mut inv: libc::c_int,
    mut scale: *const libc::c_void,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut sub_opts: FFTXCodeletOptions = {
        let mut init = FFTXCodeletOptions {
            map_dir: (if inv == 0 {
                FF_TX_MAP_SCATTER as libc::c_int
            } else {
                FF_TX_MAP_GATHER as libc::c_int
            }) as FFTXMapDirection,
        };
        init
    };
    (*s).scale_d = *(scale as *mut libc::c_double);
    (*s).scale_f = (*s).scale_d as libc::c_float;
    flags =
        (flags as libc::c_ulonglong & !((1 as libc::c_ulonglong) << 63 as libc::c_int)) as uint64_t;
    flags |= AV_TX_INPLACE as libc::c_int as libc::c_ulong;
    flags =
        (flags as libc::c_ulonglong | (1 as libc::c_ulonglong) << 61 as libc::c_int) as uint64_t;
    ret = ff_tx_init_subtx(
        s,
        AV_TX_DOUBLE_FFT,
        flags,
        &mut sub_opts,
        len >> 1 as libc::c_int,
        inv,
        scale,
    );
    if ret != 0 {
        flags = (flags as libc::c_ulonglong & !((1 as libc::c_ulonglong) << 61 as libc::c_int))
            as uint64_t;
        ret = ff_tx_init_subtx(
            s,
            AV_TX_DOUBLE_FFT,
            flags,
            &mut sub_opts,
            len >> 1 as libc::c_int,
            inv,
            scale,
        );
        if ret != 0 {
            return ret;
        }
    }
    (*s).map = av_malloc(
        ((len >> 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if ((*s).map).is_null() {
        return -(12 as libc::c_int);
    }
    if (*((*s).sub).offset(0 as libc::c_int as isize)).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 61 as libc::c_int
        != 0
    {
        memcpy(
            (*s).map as *mut libc::c_void,
            (*(*s).sub).map as *const libc::c_void,
            ((len >> 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
    } else {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < len >> 1 as libc::c_int {
            *((*s).map).offset(i as isize) = i;
            i += 1;
            i;
        }
    }
    ret = ff_tx_mdct_gen_exp_double(
        s,
        if inv != 0 {
            (*s).map
        } else {
            0 as *mut libc::c_int
        },
    );
    if ret != 0 {
        return ret;
    }
    if inv != 0 {
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < (*s).len >> 1 as libc::c_int {
            *((*s).map).offset(i_0 as isize) <<= 1 as libc::c_int;
            i_0 += 1;
            i_0;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ff_tx_mdct_fwd_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut src: *mut TXSample = _src as *mut TXSample;
    let mut dst: *mut TXSample = _dst as *mut TXSample;
    let mut exp: *mut TXComplex = (*s).exp.double;
    let mut tmp: TXComplex = TXComplex { re: 0., im: 0. };
    let mut z: *mut TXComplex = _dst as *mut TXComplex;
    let len2: libc::c_int = (*s).len >> 1 as libc::c_int;
    let len4: libc::c_int = (*s).len >> 2 as libc::c_int;
    let len3: libc::c_int = len2 * 3 as libc::c_int;
    let mut sub_map: *const libc::c_int = (*s).map;
    stride = (stride as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<TXSample>() as libc::c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < len2 {
        let k: libc::c_int = 2 as libc::c_int * i;
        let idx: libc::c_int = *sub_map.offset(i as isize);
        if k < len2 {
            tmp.re = -*src.offset((len2 + k) as isize)
                + *src.offset((1 as libc::c_int * len2 - 1 as libc::c_int - k) as isize);
            tmp.im = -*src.offset((len3 + k) as isize)
                + -*src.offset((1 as libc::c_int * len3 - 1 as libc::c_int - k) as isize);
        } else {
            tmp.re = -*src.offset((len2 + k) as isize)
                + -*src.offset((5 as libc::c_int * len2 - 1 as libc::c_int - k) as isize);
            tmp.im = *src.offset((-len2 + k) as isize)
                + -*src.offset((1 as libc::c_int * len3 - 1 as libc::c_int - k) as isize);
        }
        (*z.offset(idx as isize)).im =
            tmp.re * (*exp.offset(i as isize)).re - tmp.im * (*exp.offset(i as isize)).im;
        (*z.offset(idx as isize)).re =
            tmp.re * (*exp.offset(i as isize)).im + tmp.im * (*exp.offset(i as isize)).re;
        i += 1;
        i;
    }
    ((*s).fn_0[0 as libc::c_int as usize]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0 as libc::c_int as isize),
        z as *mut libc::c_void,
        z as *mut libc::c_void,
        ::core::mem::size_of::<TXComplex>() as libc::c_ulong as ptrdiff_t,
    );
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < len4 {
        let i0: libc::c_int = len4 + i_0;
        let i1: libc::c_int = len4 - i_0 - 1 as libc::c_int;
        let mut src1: TXComplex = {
            let mut init = AVComplexDouble {
                re: (*z.offset(i1 as isize)).re,
                im: (*z.offset(i1 as isize)).im,
            };
            init
        };
        let mut src0: TXComplex = {
            let mut init = AVComplexDouble {
                re: (*z.offset(i0 as isize)).re,
                im: (*z.offset(i0 as isize)).im,
            };
            init
        };
        *dst.offset(((2 as libc::c_int * i1) as libc::c_long * stride + stride) as isize) =
            src0.re * (*exp.offset(i0 as isize)).im - src0.im * (*exp.offset(i0 as isize)).re;
        *dst.offset(((2 as libc::c_int * i0) as libc::c_long * stride) as isize) =
            src0.re * (*exp.offset(i0 as isize)).re + src0.im * (*exp.offset(i0 as isize)).im;
        *dst.offset(((2 as libc::c_int * i0) as libc::c_long * stride + stride) as isize) =
            src1.re * (*exp.offset(i1 as isize)).im - src1.im * (*exp.offset(i1 as isize)).re;
        *dst.offset(((2 as libc::c_int * i1) as libc::c_long * stride) as isize) =
            src1.re * (*exp.offset(i1 as isize)).re + src1.im * (*exp.offset(i1 as isize)).im;
        i_0 += 1;
        i_0;
    }
}
unsafe extern "C" fn ff_tx_mdct_inv_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut z: *mut TXComplex = _dst as *mut TXComplex;
    let mut exp: *mut TXComplex = (*s).exp.double;
    let mut src: *const TXSample = _src as *const TXSample;
    let mut in1: *const TXSample = 0 as *const TXSample;
    let mut in2: *const TXSample = 0 as *const TXSample;
    let len2: libc::c_int = (*s).len >> 1 as libc::c_int;
    let len4: libc::c_int = (*s).len >> 2 as libc::c_int;
    let mut sub_map: *const libc::c_int = (*s).map;
    stride = (stride as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<TXSample>() as libc::c_ulong) as ptrdiff_t
        as ptrdiff_t;
    in1 = src;
    in2 = src
        .offset(((len2 * 2 as libc::c_int - 1 as libc::c_int) as libc::c_long * stride) as isize);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < len2 {
        let mut k: libc::c_int = *sub_map.offset(i as isize);
        let mut tmp: TXComplex = {
            let mut init = AVComplexDouble {
                re: *in2.offset((-k as libc::c_long * stride) as isize),
                im: *in1.offset((k as libc::c_long * stride) as isize),
            };
            init
        };
        (*z.offset(i as isize)).re =
            tmp.re * (*exp.offset(i as isize)).re - tmp.im * (*exp.offset(i as isize)).im;
        (*z.offset(i as isize)).im =
            tmp.re * (*exp.offset(i as isize)).im + tmp.im * (*exp.offset(i as isize)).re;
        i += 1;
        i;
    }
    ((*s).fn_0[0 as libc::c_int as usize]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0 as libc::c_int as isize),
        z as *mut libc::c_void,
        z as *mut libc::c_void,
        ::core::mem::size_of::<TXComplex>() as libc::c_ulong as ptrdiff_t,
    );
    exp = exp.offset(len2 as isize);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < len4 {
        let i0: libc::c_int = len4 + i_0;
        let i1: libc::c_int = len4 - i_0 - 1 as libc::c_int;
        let mut src1: TXComplex = {
            let mut init = AVComplexDouble {
                re: (*z.offset(i1 as isize)).im,
                im: (*z.offset(i1 as isize)).re,
            };
            init
        };
        let mut src0: TXComplex = {
            let mut init = AVComplexDouble {
                re: (*z.offset(i0 as isize)).im,
                im: (*z.offset(i0 as isize)).re,
            };
            init
        };
        (*z.offset(i1 as isize)).re =
            src1.re * (*exp.offset(i1 as isize)).im - src1.im * (*exp.offset(i1 as isize)).re;
        (*z.offset(i0 as isize)).im =
            src1.re * (*exp.offset(i1 as isize)).re + src1.im * (*exp.offset(i1 as isize)).im;
        (*z.offset(i0 as isize)).re =
            src0.re * (*exp.offset(i0 as isize)).im - src0.im * (*exp.offset(i0 as isize)).re;
        (*z.offset(i1 as isize)).im =
            src0.re * (*exp.offset(i0 as isize)).re + src0.im * (*exp.offset(i0 as isize)).im;
        i_0 += 1;
        i_0;
    }
}
static mut ff_tx_mdct_fwd_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"mdct_fwd_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_mdct_fwd_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_MDCT,
            flags: (AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | (1 as libc::c_ulonglong) << 59 as libc::c_int) as uint64_t,
            factors: [
                2 as libc::c_int,
                -(1 as libc::c_int),
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 2 as libc::c_int,
            min_len: 2 as libc::c_int,
            max_len: -(1 as libc::c_int),
            init: Some(
                ff_tx_mdct_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
static mut ff_tx_mdct_inv_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"mdct_inv_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_mdct_inv_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_MDCT,
            flags: (AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | (1 as libc::c_ulonglong) << 60 as libc::c_int) as uint64_t,
            factors: [
                2 as libc::c_int,
                -(1 as libc::c_int),
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 2 as libc::c_int,
            min_len: 2 as libc::c_int,
            max_len: -(1 as libc::c_int),
            init: Some(
                ff_tx_mdct_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
#[cold]
unsafe extern "C" fn ff_tx_mdct_inv_full_init_double_c(
    mut s: *mut AVTXContext,
    mut cd: *const FFTXCodelet,
    mut flags: uint64_t,
    mut opts: *mut FFTXCodeletOptions,
    mut len: libc::c_int,
    mut inv: libc::c_int,
    mut scale: *const libc::c_void,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    (*s).scale_d = *(scale as *mut libc::c_double);
    (*s).scale_f = (*s).scale_d as libc::c_float;
    flags &= !(AV_TX_FULL_IMDCT as libc::c_int) as libc::c_ulong;
    ret = ff_tx_init_subtx(
        s,
        AV_TX_DOUBLE_MDCT,
        flags,
        0 as *mut FFTXCodeletOptions,
        len,
        1 as libc::c_int,
        scale,
    );
    if ret != 0 {
        return ret;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ff_tx_mdct_inv_full_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut len: libc::c_int = (*s).len << 1 as libc::c_int;
    let mut len2: libc::c_int = len >> 1 as libc::c_int;
    let mut len4: libc::c_int = len >> 2 as libc::c_int;
    let mut dst: *mut TXSample = _dst as *mut TXSample;
    ((*s).fn_0[0 as libc::c_int as usize]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0 as libc::c_int as isize),
        dst.offset(len4 as isize) as *mut libc::c_void,
        _src,
        stride,
    );
    stride = (stride as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<TXSample>() as libc::c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < len4 {
        *dst.offset((i as libc::c_long * stride) as isize) =
            -*dst.offset(((len2 - i - 1 as libc::c_int) as libc::c_long * stride) as isize);
        *dst.offset(((len - i - 1 as libc::c_int) as libc::c_long * stride) as isize) =
            *dst.offset(((len2 + i + 0 as libc::c_int) as libc::c_long * stride) as isize);
        i += 1;
        i;
    }
}
static mut ff_tx_mdct_inv_full_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"mdct_inv_full_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_mdct_inv_full_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_MDCT,
            flags: ((AV_TX_UNALIGNED as libc::c_int | AV_TX_INPLACE as libc::c_int)
                as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | AV_TX_FULL_IMDCT as libc::c_int as libc::c_ulonglong)
                as uint64_t,
            factors: [
                2 as libc::c_int,
                -(1 as libc::c_int),
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 2 as libc::c_int,
            min_len: 2 as libc::c_int,
            max_len: -(1 as libc::c_int),
            init: Some(
                ff_tx_mdct_inv_full_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
#[cold]
unsafe extern "C" fn ff_tx_mdct_pfa_init_double_c(
    mut s: *mut AVTXContext,
    mut cd: *const FFTXCodelet,
    mut flags: uint64_t,
    mut opts: *mut FFTXCodeletOptions,
    mut len: libc::c_int,
    mut inv: libc::c_int,
    mut scale: *const libc::c_void,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut sub_len: libc::c_int = 0;
    let mut sub_opts: FFTXCodeletOptions = {
        let mut init = FFTXCodeletOptions {
            map_dir: FF_TX_MAP_SCATTER,
        };
        init
    };
    len >>= 1 as libc::c_int;
    sub_len = len / (*cd).factors[0 as libc::c_int as usize];
    (*s).scale_d = *(scale as *mut libc::c_double);
    (*s).scale_f = (*s).scale_d as libc::c_float;
    flags =
        (flags as libc::c_ulonglong & !((1 as libc::c_ulonglong) << 63 as libc::c_int)) as uint64_t;
    flags |= AV_TX_INPLACE as libc::c_int as libc::c_ulong;
    flags =
        (flags as libc::c_ulonglong | (1 as libc::c_ulonglong) << 61 as libc::c_int) as uint64_t;
    ret = ff_tx_init_subtx(
        s,
        AV_TX_DOUBLE_FFT,
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
        (*cd).factors[0 as libc::c_int as usize],
        sub_len,
    );
    if ret != 0 {
        return ret;
    }
    if (*cd).factors[0 as libc::c_int as usize] == 15 as libc::c_int {
        let mut mtmp: [libc::c_int; 15] = [0; 15];
        let mut k: libc::c_int = 0 as libc::c_int;
        while k < len {
            memcpy(
                mtmp.as_mut_ptr() as *mut libc::c_void,
                &mut *((*s).map).offset(k as isize) as *mut libc::c_int as *const libc::c_void,
                ((3 as libc::c_int * 5 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
            let mut m: libc::c_int = 0 as libc::c_int;
            while m < 5 as libc::c_int {
                let mut n: libc::c_int = 0 as libc::c_int;
                while n < 3 as libc::c_int {
                    *((*s).map).offset((k + m * 3 as libc::c_int + n) as isize) =
                        mtmp[((m * 3 as libc::c_int + n * 5 as libc::c_int)
                            % (3 as libc::c_int * 5 as libc::c_int))
                            as usize];
                    n += 1;
                    n;
                }
                m += 1;
                m;
            }
            k += 3 as libc::c_int * 5 as libc::c_int;
        }
    }
    ret = ff_tx_mdct_gen_exp_double(
        s,
        if inv != 0 {
            (*s).map
        } else {
            0 as *mut libc::c_int
        },
    );
    if ret != 0 {
        return ret;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < len {
        *((*s).map).offset(i as isize) <<= 1 as libc::c_int;
        i += 1;
        i;
    }
    (*s).tmp = AVTXNum {
        double: av_malloc(
            (len as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<TXComplex>() as libc::c_ulong),
        ) as *mut TXComplex,
    };
    if ((*s).tmp).double.is_null() {
        return -(12 as libc::c_int);
    }
    ff_tx_init_tabs_double(len / sub_len);
    return 0 as libc::c_int;
}
unsafe extern "C" fn ff_tx_mdct_pfa_3xM_inv_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut fft3in: [TXComplex; 3] = [TXComplex { re: 0., im: 0. }; 3];
    let mut z: *mut TXComplex = _dst as *mut TXComplex;
    let mut exp: *mut TXComplex = (*s).exp.double;
    let mut src: *const TXSample = _src as *const TXSample;
    let mut in1: *const TXSample = 0 as *const TXSample;
    let mut in2: *const TXSample = 0 as *const TXSample;
    let len4: libc::c_int = (*s).len >> 2 as libc::c_int;
    let len2: libc::c_int = (*s).len >> 1 as libc::c_int;
    let m: libc::c_int = (*(*s).sub).len;
    let mut in_map: *const libc::c_int = (*s).map;
    let mut out_map: *const libc::c_int = in_map.offset((3 as libc::c_int * m) as isize);
    let mut sub_map: *const libc::c_int = (*(*s).sub).map;
    stride = (stride as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<TXSample>() as libc::c_ulong) as ptrdiff_t
        as ptrdiff_t;
    in1 = src;
    in2 = src.offset(
        ((3 as libc::c_int * m * 2 as libc::c_int - 1 as libc::c_int) as libc::c_long * stride)
            as isize,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < len2 {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            let k: libc::c_int = *in_map.offset(j as isize);
            let mut tmp: TXComplex = {
                let mut init = AVComplexDouble {
                    re: *in2.offset((-k as libc::c_long * stride) as isize),
                    im: *in1.offset((k as libc::c_long * stride) as isize),
                };
                init
            };
            fft3in[j as usize].re =
                tmp.re * (*exp.offset(j as isize)).re - tmp.im * (*exp.offset(j as isize)).im;
            fft3in[j as usize].im =
                tmp.re * (*exp.offset(j as isize)).im + tmp.im * (*exp.offset(j as isize)).re;
            j += 1;
            j;
        }
        let fresh22 = sub_map;
        sub_map = sub_map.offset(1);
        fft3(
            ((*s).tmp).double.offset(*fresh22 as isize),
            fft3in.as_mut_ptr(),
            m as ptrdiff_t,
        );
        exp = exp.offset(3 as libc::c_int as isize);
        in_map = in_map.offset(3 as libc::c_int as isize);
        i += 3 as libc::c_int;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 3 as libc::c_int {
        ((*s).fn_0[0 as libc::c_int as usize]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0 as libc::c_int as isize),
            ((*s).tmp).double.offset((m * i_0) as isize) as *mut libc::c_void,
            ((*s).tmp).double.offset((m * i_0) as isize) as *mut libc::c_void,
            ::core::mem::size_of::<TXComplex>() as libc::c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < len4 {
        let i0: libc::c_int = len4 + i_1;
        let i1: libc::c_int = len4 - i_1 - 1 as libc::c_int;
        let s0: libc::c_int = *out_map.offset(i0 as isize);
        let s1: libc::c_int = *out_map.offset(i1 as isize);
        let mut src1: TXComplex = {
            let mut init = AVComplexDouble {
                re: (*((*s).tmp).double.offset(s1 as isize)).im,
                im: (*((*s).tmp).double.offset(s1 as isize)).re,
            };
            init
        };
        let mut src0: TXComplex = {
            let mut init = AVComplexDouble {
                re: (*((*s).tmp).double.offset(s0 as isize)).im,
                im: (*((*s).tmp).double.offset(s0 as isize)).re,
            };
            init
        };
        (*z.offset(i1 as isize)).re =
            src1.re * (*exp.offset(i1 as isize)).im - src1.im * (*exp.offset(i1 as isize)).re;
        (*z.offset(i0 as isize)).im =
            src1.re * (*exp.offset(i1 as isize)).re + src1.im * (*exp.offset(i1 as isize)).im;
        (*z.offset(i0 as isize)).re =
            src0.re * (*exp.offset(i0 as isize)).im - src0.im * (*exp.offset(i0 as isize)).re;
        (*z.offset(i1 as isize)).im =
            src0.re * (*exp.offset(i0 as isize)).re + src0.im * (*exp.offset(i0 as isize)).im;
        i_1 += 1;
        i_1;
    }
}
static mut ff_tx_mdct_pfa_3xM_inv_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"mdct_pfa_3xM_inv_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_mdct_pfa_3xM_inv_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_MDCT,
            flags: (AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | (1 as libc::c_ulonglong) << 60 as libc::c_int) as uint64_t,
            factors: [
                3 as libc::c_int,
                -(1 as libc::c_int),
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 2 as libc::c_int,
            min_len: 3 as libc::c_int * 2 as libc::c_int,
            max_len: -(1 as libc::c_int),
            init: Some(
                ff_tx_mdct_pfa_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn ff_tx_mdct_pfa_5xM_inv_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut fft5in: [TXComplex; 5] = [TXComplex { re: 0., im: 0. }; 5];
    let mut z: *mut TXComplex = _dst as *mut TXComplex;
    let mut exp: *mut TXComplex = (*s).exp.double;
    let mut src: *const TXSample = _src as *const TXSample;
    let mut in1: *const TXSample = 0 as *const TXSample;
    let mut in2: *const TXSample = 0 as *const TXSample;
    let len4: libc::c_int = (*s).len >> 2 as libc::c_int;
    let len2: libc::c_int = (*s).len >> 1 as libc::c_int;
    let m: libc::c_int = (*(*s).sub).len;
    let mut in_map: *const libc::c_int = (*s).map;
    let mut out_map: *const libc::c_int = in_map.offset((5 as libc::c_int * m) as isize);
    let mut sub_map: *const libc::c_int = (*(*s).sub).map;
    stride = (stride as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<TXSample>() as libc::c_ulong) as ptrdiff_t
        as ptrdiff_t;
    in1 = src;
    in2 = src.offset(
        ((5 as libc::c_int * m * 2 as libc::c_int - 1 as libc::c_int) as libc::c_long * stride)
            as isize,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < len2 {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 5 as libc::c_int {
            let k: libc::c_int = *in_map.offset(j as isize);
            let mut tmp: TXComplex = {
                let mut init = AVComplexDouble {
                    re: *in2.offset((-k as libc::c_long * stride) as isize),
                    im: *in1.offset((k as libc::c_long * stride) as isize),
                };
                init
            };
            fft5in[j as usize].re =
                tmp.re * (*exp.offset(j as isize)).re - tmp.im * (*exp.offset(j as isize)).im;
            fft5in[j as usize].im =
                tmp.re * (*exp.offset(j as isize)).im + tmp.im * (*exp.offset(j as isize)).re;
            j += 1;
            j;
        }
        let fresh23 = sub_map;
        sub_map = sub_map.offset(1);
        fft5(
            ((*s).tmp).double.offset(*fresh23 as isize),
            fft5in.as_mut_ptr(),
            m as ptrdiff_t,
        );
        exp = exp.offset(5 as libc::c_int as isize);
        in_map = in_map.offset(5 as libc::c_int as isize);
        i += 5 as libc::c_int;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 5 as libc::c_int {
        ((*s).fn_0[0 as libc::c_int as usize]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0 as libc::c_int as isize),
            ((*s).tmp).double.offset((m * i_0) as isize) as *mut libc::c_void,
            ((*s).tmp).double.offset((m * i_0) as isize) as *mut libc::c_void,
            ::core::mem::size_of::<TXComplex>() as libc::c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < len4 {
        let i0: libc::c_int = len4 + i_1;
        let i1: libc::c_int = len4 - i_1 - 1 as libc::c_int;
        let s0: libc::c_int = *out_map.offset(i0 as isize);
        let s1: libc::c_int = *out_map.offset(i1 as isize);
        let mut src1: TXComplex = {
            let mut init = AVComplexDouble {
                re: (*((*s).tmp).double.offset(s1 as isize)).im,
                im: (*((*s).tmp).double.offset(s1 as isize)).re,
            };
            init
        };
        let mut src0: TXComplex = {
            let mut init = AVComplexDouble {
                re: (*((*s).tmp).double.offset(s0 as isize)).im,
                im: (*((*s).tmp).double.offset(s0 as isize)).re,
            };
            init
        };
        (*z.offset(i1 as isize)).re =
            src1.re * (*exp.offset(i1 as isize)).im - src1.im * (*exp.offset(i1 as isize)).re;
        (*z.offset(i0 as isize)).im =
            src1.re * (*exp.offset(i1 as isize)).re + src1.im * (*exp.offset(i1 as isize)).im;
        (*z.offset(i0 as isize)).re =
            src0.re * (*exp.offset(i0 as isize)).im - src0.im * (*exp.offset(i0 as isize)).re;
        (*z.offset(i1 as isize)).im =
            src0.re * (*exp.offset(i0 as isize)).re + src0.im * (*exp.offset(i0 as isize)).im;
        i_1 += 1;
        i_1;
    }
}
static mut ff_tx_mdct_pfa_5xM_inv_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"mdct_pfa_5xM_inv_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_mdct_pfa_5xM_inv_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_MDCT,
            flags: (AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | (1 as libc::c_ulonglong) << 60 as libc::c_int) as uint64_t,
            factors: [
                5 as libc::c_int,
                -(1 as libc::c_int),
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 2 as libc::c_int,
            min_len: 5 as libc::c_int * 2 as libc::c_int,
            max_len: -(1 as libc::c_int),
            init: Some(
                ff_tx_mdct_pfa_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
static mut ff_tx_mdct_pfa_7xM_inv_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"mdct_pfa_7xM_inv_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_mdct_pfa_7xM_inv_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_MDCT,
            flags: (AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | (1 as libc::c_ulonglong) << 60 as libc::c_int) as uint64_t,
            factors: [
                7 as libc::c_int,
                -(1 as libc::c_int),
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 2 as libc::c_int,
            min_len: 7 as libc::c_int * 2 as libc::c_int,
            max_len: -(1 as libc::c_int),
            init: Some(
                ff_tx_mdct_pfa_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn ff_tx_mdct_pfa_7xM_inv_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut fft7in: [TXComplex; 7] = [TXComplex { re: 0., im: 0. }; 7];
    let mut z: *mut TXComplex = _dst as *mut TXComplex;
    let mut exp: *mut TXComplex = (*s).exp.double;
    let mut src: *const TXSample = _src as *const TXSample;
    let mut in1: *const TXSample = 0 as *const TXSample;
    let mut in2: *const TXSample = 0 as *const TXSample;
    let len4: libc::c_int = (*s).len >> 2 as libc::c_int;
    let len2: libc::c_int = (*s).len >> 1 as libc::c_int;
    let m: libc::c_int = (*(*s).sub).len;
    let mut in_map: *const libc::c_int = (*s).map;
    let mut out_map: *const libc::c_int = in_map.offset((7 as libc::c_int * m) as isize);
    let mut sub_map: *const libc::c_int = (*(*s).sub).map;
    stride = (stride as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<TXSample>() as libc::c_ulong) as ptrdiff_t
        as ptrdiff_t;
    in1 = src;
    in2 = src.offset(
        ((7 as libc::c_int * m * 2 as libc::c_int - 1 as libc::c_int) as libc::c_long * stride)
            as isize,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < len2 {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 7 as libc::c_int {
            let k: libc::c_int = *in_map.offset(j as isize);
            let mut tmp: TXComplex = {
                let mut init = AVComplexDouble {
                    re: *in2.offset((-k as libc::c_long * stride) as isize),
                    im: *in1.offset((k as libc::c_long * stride) as isize),
                };
                init
            };
            fft7in[j as usize].re =
                tmp.re * (*exp.offset(j as isize)).re - tmp.im * (*exp.offset(j as isize)).im;
            fft7in[j as usize].im =
                tmp.re * (*exp.offset(j as isize)).im + tmp.im * (*exp.offset(j as isize)).re;
            j += 1;
            j;
        }
        let fresh24 = sub_map;
        sub_map = sub_map.offset(1);
        fft7(
            ((*s).tmp).double.offset(*fresh24 as isize),
            fft7in.as_mut_ptr(),
            m as ptrdiff_t,
        );
        exp = exp.offset(7 as libc::c_int as isize);
        in_map = in_map.offset(7 as libc::c_int as isize);
        i += 7 as libc::c_int;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 7 as libc::c_int {
        ((*s).fn_0[0 as libc::c_int as usize]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0 as libc::c_int as isize),
            ((*s).tmp).double.offset((m * i_0) as isize) as *mut libc::c_void,
            ((*s).tmp).double.offset((m * i_0) as isize) as *mut libc::c_void,
            ::core::mem::size_of::<TXComplex>() as libc::c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < len4 {
        let i0: libc::c_int = len4 + i_1;
        let i1: libc::c_int = len4 - i_1 - 1 as libc::c_int;
        let s0: libc::c_int = *out_map.offset(i0 as isize);
        let s1: libc::c_int = *out_map.offset(i1 as isize);
        let mut src1: TXComplex = {
            let mut init = AVComplexDouble {
                re: (*((*s).tmp).double.offset(s1 as isize)).im,
                im: (*((*s).tmp).double.offset(s1 as isize)).re,
            };
            init
        };
        let mut src0: TXComplex = {
            let mut init = AVComplexDouble {
                re: (*((*s).tmp).double.offset(s0 as isize)).im,
                im: (*((*s).tmp).double.offset(s0 as isize)).re,
            };
            init
        };
        (*z.offset(i1 as isize)).re =
            src1.re * (*exp.offset(i1 as isize)).im - src1.im * (*exp.offset(i1 as isize)).re;
        (*z.offset(i0 as isize)).im =
            src1.re * (*exp.offset(i1 as isize)).re + src1.im * (*exp.offset(i1 as isize)).im;
        (*z.offset(i0 as isize)).re =
            src0.re * (*exp.offset(i0 as isize)).im - src0.im * (*exp.offset(i0 as isize)).re;
        (*z.offset(i1 as isize)).im =
            src0.re * (*exp.offset(i0 as isize)).re + src0.im * (*exp.offset(i0 as isize)).im;
        i_1 += 1;
        i_1;
    }
}
unsafe extern "C" fn ff_tx_mdct_pfa_9xM_inv_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut fft9in: [TXComplex; 9] = [TXComplex { re: 0., im: 0. }; 9];
    let mut z: *mut TXComplex = _dst as *mut TXComplex;
    let mut exp: *mut TXComplex = (*s).exp.double;
    let mut src: *const TXSample = _src as *const TXSample;
    let mut in1: *const TXSample = 0 as *const TXSample;
    let mut in2: *const TXSample = 0 as *const TXSample;
    let len4: libc::c_int = (*s).len >> 2 as libc::c_int;
    let len2: libc::c_int = (*s).len >> 1 as libc::c_int;
    let m: libc::c_int = (*(*s).sub).len;
    let mut in_map: *const libc::c_int = (*s).map;
    let mut out_map: *const libc::c_int = in_map.offset((9 as libc::c_int * m) as isize);
    let mut sub_map: *const libc::c_int = (*(*s).sub).map;
    stride = (stride as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<TXSample>() as libc::c_ulong) as ptrdiff_t
        as ptrdiff_t;
    in1 = src;
    in2 = src.offset(
        ((9 as libc::c_int * m * 2 as libc::c_int - 1 as libc::c_int) as libc::c_long * stride)
            as isize,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < len2 {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 9 as libc::c_int {
            let k: libc::c_int = *in_map.offset(j as isize);
            let mut tmp: TXComplex = {
                let mut init = AVComplexDouble {
                    re: *in2.offset((-k as libc::c_long * stride) as isize),
                    im: *in1.offset((k as libc::c_long * stride) as isize),
                };
                init
            };
            fft9in[j as usize].re =
                tmp.re * (*exp.offset(j as isize)).re - tmp.im * (*exp.offset(j as isize)).im;
            fft9in[j as usize].im =
                tmp.re * (*exp.offset(j as isize)).im + tmp.im * (*exp.offset(j as isize)).re;
            j += 1;
            j;
        }
        let fresh25 = sub_map;
        sub_map = sub_map.offset(1);
        fft9(
            ((*s).tmp).double.offset(*fresh25 as isize),
            fft9in.as_mut_ptr(),
            m as ptrdiff_t,
        );
        exp = exp.offset(9 as libc::c_int as isize);
        in_map = in_map.offset(9 as libc::c_int as isize);
        i += 9 as libc::c_int;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 9 as libc::c_int {
        ((*s).fn_0[0 as libc::c_int as usize]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0 as libc::c_int as isize),
            ((*s).tmp).double.offset((m * i_0) as isize) as *mut libc::c_void,
            ((*s).tmp).double.offset((m * i_0) as isize) as *mut libc::c_void,
            ::core::mem::size_of::<TXComplex>() as libc::c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < len4 {
        let i0: libc::c_int = len4 + i_1;
        let i1: libc::c_int = len4 - i_1 - 1 as libc::c_int;
        let s0: libc::c_int = *out_map.offset(i0 as isize);
        let s1: libc::c_int = *out_map.offset(i1 as isize);
        let mut src1: TXComplex = {
            let mut init = AVComplexDouble {
                re: (*((*s).tmp).double.offset(s1 as isize)).im,
                im: (*((*s).tmp).double.offset(s1 as isize)).re,
            };
            init
        };
        let mut src0: TXComplex = {
            let mut init = AVComplexDouble {
                re: (*((*s).tmp).double.offset(s0 as isize)).im,
                im: (*((*s).tmp).double.offset(s0 as isize)).re,
            };
            init
        };
        (*z.offset(i1 as isize)).re =
            src1.re * (*exp.offset(i1 as isize)).im - src1.im * (*exp.offset(i1 as isize)).re;
        (*z.offset(i0 as isize)).im =
            src1.re * (*exp.offset(i1 as isize)).re + src1.im * (*exp.offset(i1 as isize)).im;
        (*z.offset(i0 as isize)).re =
            src0.re * (*exp.offset(i0 as isize)).im - src0.im * (*exp.offset(i0 as isize)).re;
        (*z.offset(i1 as isize)).im =
            src0.re * (*exp.offset(i0 as isize)).re + src0.im * (*exp.offset(i0 as isize)).im;
        i_1 += 1;
        i_1;
    }
}
static mut ff_tx_mdct_pfa_9xM_inv_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"mdct_pfa_9xM_inv_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_mdct_pfa_9xM_inv_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_MDCT,
            flags: (AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | (1 as libc::c_ulonglong) << 60 as libc::c_int) as uint64_t,
            factors: [
                9 as libc::c_int,
                -(1 as libc::c_int),
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 2 as libc::c_int,
            min_len: 9 as libc::c_int * 2 as libc::c_int,
            max_len: -(1 as libc::c_int),
            init: Some(
                ff_tx_mdct_pfa_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn ff_tx_mdct_pfa_15xM_inv_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut fft15in: [TXComplex; 15] = [TXComplex { re: 0., im: 0. }; 15];
    let mut z: *mut TXComplex = _dst as *mut TXComplex;
    let mut exp: *mut TXComplex = (*s).exp.double;
    let mut src: *const TXSample = _src as *const TXSample;
    let mut in1: *const TXSample = 0 as *const TXSample;
    let mut in2: *const TXSample = 0 as *const TXSample;
    let len4: libc::c_int = (*s).len >> 2 as libc::c_int;
    let len2: libc::c_int = (*s).len >> 1 as libc::c_int;
    let m: libc::c_int = (*(*s).sub).len;
    let mut in_map: *const libc::c_int = (*s).map;
    let mut out_map: *const libc::c_int = in_map.offset((15 as libc::c_int * m) as isize);
    let mut sub_map: *const libc::c_int = (*(*s).sub).map;
    stride = (stride as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<TXSample>() as libc::c_ulong) as ptrdiff_t
        as ptrdiff_t;
    in1 = src;
    in2 = src.offset(
        ((15 as libc::c_int * m * 2 as libc::c_int - 1 as libc::c_int) as libc::c_long * stride)
            as isize,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < len2 {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 15 as libc::c_int {
            let k: libc::c_int = *in_map.offset(j as isize);
            let mut tmp: TXComplex = {
                let mut init = AVComplexDouble {
                    re: *in2.offset((-k as libc::c_long * stride) as isize),
                    im: *in1.offset((k as libc::c_long * stride) as isize),
                };
                init
            };
            fft15in[j as usize].re =
                tmp.re * (*exp.offset(j as isize)).re - tmp.im * (*exp.offset(j as isize)).im;
            fft15in[j as usize].im =
                tmp.re * (*exp.offset(j as isize)).im + tmp.im * (*exp.offset(j as isize)).re;
            j += 1;
            j;
        }
        let fresh26 = sub_map;
        sub_map = sub_map.offset(1);
        fft15(
            ((*s).tmp).double.offset(*fresh26 as isize),
            fft15in.as_mut_ptr(),
            m as ptrdiff_t,
        );
        exp = exp.offset(15 as libc::c_int as isize);
        in_map = in_map.offset(15 as libc::c_int as isize);
        i += 15 as libc::c_int;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 15 as libc::c_int {
        ((*s).fn_0[0 as libc::c_int as usize]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0 as libc::c_int as isize),
            ((*s).tmp).double.offset((m * i_0) as isize) as *mut libc::c_void,
            ((*s).tmp).double.offset((m * i_0) as isize) as *mut libc::c_void,
            ::core::mem::size_of::<TXComplex>() as libc::c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < len4 {
        let i0: libc::c_int = len4 + i_1;
        let i1: libc::c_int = len4 - i_1 - 1 as libc::c_int;
        let s0: libc::c_int = *out_map.offset(i0 as isize);
        let s1: libc::c_int = *out_map.offset(i1 as isize);
        let mut src1: TXComplex = {
            let mut init = AVComplexDouble {
                re: (*((*s).tmp).double.offset(s1 as isize)).im,
                im: (*((*s).tmp).double.offset(s1 as isize)).re,
            };
            init
        };
        let mut src0: TXComplex = {
            let mut init = AVComplexDouble {
                re: (*((*s).tmp).double.offset(s0 as isize)).im,
                im: (*((*s).tmp).double.offset(s0 as isize)).re,
            };
            init
        };
        (*z.offset(i1 as isize)).re =
            src1.re * (*exp.offset(i1 as isize)).im - src1.im * (*exp.offset(i1 as isize)).re;
        (*z.offset(i0 as isize)).im =
            src1.re * (*exp.offset(i1 as isize)).re + src1.im * (*exp.offset(i1 as isize)).im;
        (*z.offset(i0 as isize)).re =
            src0.re * (*exp.offset(i0 as isize)).im - src0.im * (*exp.offset(i0 as isize)).re;
        (*z.offset(i1 as isize)).im =
            src0.re * (*exp.offset(i0 as isize)).re + src0.im * (*exp.offset(i0 as isize)).im;
        i_1 += 1;
        i_1;
    }
}
static mut ff_tx_mdct_pfa_15xM_inv_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"mdct_pfa_15xM_inv_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_mdct_pfa_15xM_inv_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_MDCT,
            flags: (AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | (1 as libc::c_ulonglong) << 60 as libc::c_int) as uint64_t,
            factors: [
                15 as libc::c_int,
                -(1 as libc::c_int),
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 2 as libc::c_int,
            min_len: 15 as libc::c_int * 2 as libc::c_int,
            max_len: -(1 as libc::c_int),
            init: Some(
                ff_tx_mdct_pfa_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn ff_tx_mdct_pfa_3xM_fwd_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut fft3in: [TXComplex; 3] = [TXComplex { re: 0., im: 0. }; 3];
    let mut src: *mut TXSample = _src as *mut TXSample;
    let mut dst: *mut TXSample = _dst as *mut TXSample;
    let mut exp: *mut TXComplex = (*s).exp.double;
    let mut tmp: TXComplex = TXComplex { re: 0., im: 0. };
    let m: libc::c_int = (*(*s).sub).len;
    let len4: libc::c_int = 3 as libc::c_int * m;
    let len3: libc::c_int = len4 * 3 as libc::c_int;
    let len8: libc::c_int = (*s).len >> 2 as libc::c_int;
    let mut in_map: *const libc::c_int = (*s).map;
    let mut out_map: *const libc::c_int = in_map.offset((3 as libc::c_int * m) as isize);
    let mut sub_map: *const libc::c_int = (*(*s).sub).map;
    stride = (stride as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<TXSample>() as libc::c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < m {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            let k: libc::c_int = *in_map.offset((i * 3 as libc::c_int + j) as isize);
            if k < len4 {
                tmp.re = -*src.offset((len4 + k) as isize)
                    + *src.offset((1 as libc::c_int * len4 - 1 as libc::c_int - k) as isize);
                tmp.im = -*src.offset((len3 + k) as isize)
                    + -*src.offset((1 as libc::c_int * len3 - 1 as libc::c_int - k) as isize);
            } else {
                tmp.re = -*src.offset((len4 + k) as isize)
                    + -*src.offset((5 as libc::c_int * len4 - 1 as libc::c_int - k) as isize);
                tmp.im = *src.offset((-len4 + k) as isize)
                    + -*src.offset((1 as libc::c_int * len3 - 1 as libc::c_int - k) as isize);
            }
            fft3in[j as usize].im = tmp.re * (*exp.offset((k >> 1 as libc::c_int) as isize)).re
                - tmp.im * (*exp.offset((k >> 1 as libc::c_int) as isize)).im;
            fft3in[j as usize].re = tmp.re * (*exp.offset((k >> 1 as libc::c_int) as isize)).im
                + tmp.im * (*exp.offset((k >> 1 as libc::c_int) as isize)).re;
            j += 1;
            j;
        }
        fft3(
            ((*s).tmp)
                .double
                .offset(*sub_map.offset(i as isize) as isize),
            fft3in.as_mut_ptr(),
            m as ptrdiff_t,
        );
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 3 as libc::c_int {
        ((*s).fn_0[0 as libc::c_int as usize]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0 as libc::c_int as isize),
            ((*s).tmp).double.offset((m * i_0) as isize) as *mut libc::c_void,
            ((*s).tmp).double.offset((m * i_0) as isize) as *mut libc::c_void,
            ::core::mem::size_of::<TXComplex>() as libc::c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < len8 {
        let i0: libc::c_int = len8 + i_1;
        let i1: libc::c_int = len8 - i_1 - 1 as libc::c_int;
        let s0: libc::c_int = *out_map.offset(i0 as isize);
        let s1: libc::c_int = *out_map.offset(i1 as isize);
        let mut src1: TXComplex = {
            let mut init = AVComplexDouble {
                re: (*((*s).tmp).double.offset(s1 as isize)).re,
                im: (*((*s).tmp).double.offset(s1 as isize)).im,
            };
            init
        };
        let mut src0: TXComplex = {
            let mut init = AVComplexDouble {
                re: (*((*s).tmp).double.offset(s0 as isize)).re,
                im: (*((*s).tmp).double.offset(s0 as isize)).im,
            };
            init
        };
        *dst.offset(((2 as libc::c_int * i1) as libc::c_long * stride + stride) as isize) =
            src0.re * (*exp.offset(i0 as isize)).im - src0.im * (*exp.offset(i0 as isize)).re;
        *dst.offset(((2 as libc::c_int * i0) as libc::c_long * stride) as isize) =
            src0.re * (*exp.offset(i0 as isize)).re + src0.im * (*exp.offset(i0 as isize)).im;
        *dst.offset(((2 as libc::c_int * i0) as libc::c_long * stride + stride) as isize) =
            src1.re * (*exp.offset(i1 as isize)).im - src1.im * (*exp.offset(i1 as isize)).re;
        *dst.offset(((2 as libc::c_int * i1) as libc::c_long * stride) as isize) =
            src1.re * (*exp.offset(i1 as isize)).re + src1.im * (*exp.offset(i1 as isize)).im;
        i_1 += 1;
        i_1;
    }
}
static mut ff_tx_mdct_pfa_3xM_fwd_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"mdct_pfa_3xM_fwd_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_mdct_pfa_3xM_fwd_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_MDCT,
            flags: (AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | (1 as libc::c_ulonglong) << 59 as libc::c_int) as uint64_t,
            factors: [
                3 as libc::c_int,
                -(1 as libc::c_int),
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 2 as libc::c_int,
            min_len: 3 as libc::c_int * 2 as libc::c_int,
            max_len: -(1 as libc::c_int),
            init: Some(
                ff_tx_mdct_pfa_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn ff_tx_mdct_pfa_5xM_fwd_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut fft5in: [TXComplex; 5] = [TXComplex { re: 0., im: 0. }; 5];
    let mut src: *mut TXSample = _src as *mut TXSample;
    let mut dst: *mut TXSample = _dst as *mut TXSample;
    let mut exp: *mut TXComplex = (*s).exp.double;
    let mut tmp: TXComplex = TXComplex { re: 0., im: 0. };
    let m: libc::c_int = (*(*s).sub).len;
    let len4: libc::c_int = 5 as libc::c_int * m;
    let len3: libc::c_int = len4 * 3 as libc::c_int;
    let len8: libc::c_int = (*s).len >> 2 as libc::c_int;
    let mut in_map: *const libc::c_int = (*s).map;
    let mut out_map: *const libc::c_int = in_map.offset((5 as libc::c_int * m) as isize);
    let mut sub_map: *const libc::c_int = (*(*s).sub).map;
    stride = (stride as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<TXSample>() as libc::c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < m {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 5 as libc::c_int {
            let k: libc::c_int = *in_map.offset((i * 5 as libc::c_int + j) as isize);
            if k < len4 {
                tmp.re = -*src.offset((len4 + k) as isize)
                    + *src.offset((1 as libc::c_int * len4 - 1 as libc::c_int - k) as isize);
                tmp.im = -*src.offset((len3 + k) as isize)
                    + -*src.offset((1 as libc::c_int * len3 - 1 as libc::c_int - k) as isize);
            } else {
                tmp.re = -*src.offset((len4 + k) as isize)
                    + -*src.offset((5 as libc::c_int * len4 - 1 as libc::c_int - k) as isize);
                tmp.im = *src.offset((-len4 + k) as isize)
                    + -*src.offset((1 as libc::c_int * len3 - 1 as libc::c_int - k) as isize);
            }
            fft5in[j as usize].im = tmp.re * (*exp.offset((k >> 1 as libc::c_int) as isize)).re
                - tmp.im * (*exp.offset((k >> 1 as libc::c_int) as isize)).im;
            fft5in[j as usize].re = tmp.re * (*exp.offset((k >> 1 as libc::c_int) as isize)).im
                + tmp.im * (*exp.offset((k >> 1 as libc::c_int) as isize)).re;
            j += 1;
            j;
        }
        fft5(
            ((*s).tmp)
                .double
                .offset(*sub_map.offset(i as isize) as isize),
            fft5in.as_mut_ptr(),
            m as ptrdiff_t,
        );
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 5 as libc::c_int {
        ((*s).fn_0[0 as libc::c_int as usize]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0 as libc::c_int as isize),
            ((*s).tmp).double.offset((m * i_0) as isize) as *mut libc::c_void,
            ((*s).tmp).double.offset((m * i_0) as isize) as *mut libc::c_void,
            ::core::mem::size_of::<TXComplex>() as libc::c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < len8 {
        let i0: libc::c_int = len8 + i_1;
        let i1: libc::c_int = len8 - i_1 - 1 as libc::c_int;
        let s0: libc::c_int = *out_map.offset(i0 as isize);
        let s1: libc::c_int = *out_map.offset(i1 as isize);
        let mut src1: TXComplex = {
            let mut init = AVComplexDouble {
                re: (*((*s).tmp).double.offset(s1 as isize)).re,
                im: (*((*s).tmp).double.offset(s1 as isize)).im,
            };
            init
        };
        let mut src0: TXComplex = {
            let mut init = AVComplexDouble {
                re: (*((*s).tmp).double.offset(s0 as isize)).re,
                im: (*((*s).tmp).double.offset(s0 as isize)).im,
            };
            init
        };
        *dst.offset(((2 as libc::c_int * i1) as libc::c_long * stride + stride) as isize) =
            src0.re * (*exp.offset(i0 as isize)).im - src0.im * (*exp.offset(i0 as isize)).re;
        *dst.offset(((2 as libc::c_int * i0) as libc::c_long * stride) as isize) =
            src0.re * (*exp.offset(i0 as isize)).re + src0.im * (*exp.offset(i0 as isize)).im;
        *dst.offset(((2 as libc::c_int * i0) as libc::c_long * stride + stride) as isize) =
            src1.re * (*exp.offset(i1 as isize)).im - src1.im * (*exp.offset(i1 as isize)).re;
        *dst.offset(((2 as libc::c_int * i1) as libc::c_long * stride) as isize) =
            src1.re * (*exp.offset(i1 as isize)).re + src1.im * (*exp.offset(i1 as isize)).im;
        i_1 += 1;
        i_1;
    }
}
static mut ff_tx_mdct_pfa_5xM_fwd_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"mdct_pfa_5xM_fwd_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_mdct_pfa_5xM_fwd_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_MDCT,
            flags: (AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | (1 as libc::c_ulonglong) << 59 as libc::c_int) as uint64_t,
            factors: [
                5 as libc::c_int,
                -(1 as libc::c_int),
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 2 as libc::c_int,
            min_len: 5 as libc::c_int * 2 as libc::c_int,
            max_len: -(1 as libc::c_int),
            init: Some(
                ff_tx_mdct_pfa_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn ff_tx_mdct_pfa_7xM_fwd_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut fft7in: [TXComplex; 7] = [TXComplex { re: 0., im: 0. }; 7];
    let mut src: *mut TXSample = _src as *mut TXSample;
    let mut dst: *mut TXSample = _dst as *mut TXSample;
    let mut exp: *mut TXComplex = (*s).exp.double;
    let mut tmp: TXComplex = TXComplex { re: 0., im: 0. };
    let m: libc::c_int = (*(*s).sub).len;
    let len4: libc::c_int = 7 as libc::c_int * m;
    let len3: libc::c_int = len4 * 3 as libc::c_int;
    let len8: libc::c_int = (*s).len >> 2 as libc::c_int;
    let mut in_map: *const libc::c_int = (*s).map;
    let mut out_map: *const libc::c_int = in_map.offset((7 as libc::c_int * m) as isize);
    let mut sub_map: *const libc::c_int = (*(*s).sub).map;
    stride = (stride as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<TXSample>() as libc::c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < m {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 7 as libc::c_int {
            let k: libc::c_int = *in_map.offset((i * 7 as libc::c_int + j) as isize);
            if k < len4 {
                tmp.re = -*src.offset((len4 + k) as isize)
                    + *src.offset((1 as libc::c_int * len4 - 1 as libc::c_int - k) as isize);
                tmp.im = -*src.offset((len3 + k) as isize)
                    + -*src.offset((1 as libc::c_int * len3 - 1 as libc::c_int - k) as isize);
            } else {
                tmp.re = -*src.offset((len4 + k) as isize)
                    + -*src.offset((5 as libc::c_int * len4 - 1 as libc::c_int - k) as isize);
                tmp.im = *src.offset((-len4 + k) as isize)
                    + -*src.offset((1 as libc::c_int * len3 - 1 as libc::c_int - k) as isize);
            }
            fft7in[j as usize].im = tmp.re * (*exp.offset((k >> 1 as libc::c_int) as isize)).re
                - tmp.im * (*exp.offset((k >> 1 as libc::c_int) as isize)).im;
            fft7in[j as usize].re = tmp.re * (*exp.offset((k >> 1 as libc::c_int) as isize)).im
                + tmp.im * (*exp.offset((k >> 1 as libc::c_int) as isize)).re;
            j += 1;
            j;
        }
        fft7(
            ((*s).tmp)
                .double
                .offset(*sub_map.offset(i as isize) as isize),
            fft7in.as_mut_ptr(),
            m as ptrdiff_t,
        );
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 7 as libc::c_int {
        ((*s).fn_0[0 as libc::c_int as usize]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0 as libc::c_int as isize),
            ((*s).tmp).double.offset((m * i_0) as isize) as *mut libc::c_void,
            ((*s).tmp).double.offset((m * i_0) as isize) as *mut libc::c_void,
            ::core::mem::size_of::<TXComplex>() as libc::c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < len8 {
        let i0: libc::c_int = len8 + i_1;
        let i1: libc::c_int = len8 - i_1 - 1 as libc::c_int;
        let s0: libc::c_int = *out_map.offset(i0 as isize);
        let s1: libc::c_int = *out_map.offset(i1 as isize);
        let mut src1: TXComplex = {
            let mut init = AVComplexDouble {
                re: (*((*s).tmp).double.offset(s1 as isize)).re,
                im: (*((*s).tmp).double.offset(s1 as isize)).im,
            };
            init
        };
        let mut src0: TXComplex = {
            let mut init = AVComplexDouble {
                re: (*((*s).tmp).double.offset(s0 as isize)).re,
                im: (*((*s).tmp).double.offset(s0 as isize)).im,
            };
            init
        };
        *dst.offset(((2 as libc::c_int * i1) as libc::c_long * stride + stride) as isize) =
            src0.re * (*exp.offset(i0 as isize)).im - src0.im * (*exp.offset(i0 as isize)).re;
        *dst.offset(((2 as libc::c_int * i0) as libc::c_long * stride) as isize) =
            src0.re * (*exp.offset(i0 as isize)).re + src0.im * (*exp.offset(i0 as isize)).im;
        *dst.offset(((2 as libc::c_int * i0) as libc::c_long * stride + stride) as isize) =
            src1.re * (*exp.offset(i1 as isize)).im - src1.im * (*exp.offset(i1 as isize)).re;
        *dst.offset(((2 as libc::c_int * i1) as libc::c_long * stride) as isize) =
            src1.re * (*exp.offset(i1 as isize)).re + src1.im * (*exp.offset(i1 as isize)).im;
        i_1 += 1;
        i_1;
    }
}
static mut ff_tx_mdct_pfa_7xM_fwd_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"mdct_pfa_7xM_fwd_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_mdct_pfa_7xM_fwd_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_MDCT,
            flags: (AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | (1 as libc::c_ulonglong) << 59 as libc::c_int) as uint64_t,
            factors: [
                7 as libc::c_int,
                -(1 as libc::c_int),
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 2 as libc::c_int,
            min_len: 7 as libc::c_int * 2 as libc::c_int,
            max_len: -(1 as libc::c_int),
            init: Some(
                ff_tx_mdct_pfa_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn ff_tx_mdct_pfa_9xM_fwd_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut fft9in: [TXComplex; 9] = [TXComplex { re: 0., im: 0. }; 9];
    let mut src: *mut TXSample = _src as *mut TXSample;
    let mut dst: *mut TXSample = _dst as *mut TXSample;
    let mut exp: *mut TXComplex = (*s).exp.double;
    let mut tmp: TXComplex = TXComplex { re: 0., im: 0. };
    let m: libc::c_int = (*(*s).sub).len;
    let len4: libc::c_int = 9 as libc::c_int * m;
    let len3: libc::c_int = len4 * 3 as libc::c_int;
    let len8: libc::c_int = (*s).len >> 2 as libc::c_int;
    let mut in_map: *const libc::c_int = (*s).map;
    let mut out_map: *const libc::c_int = in_map.offset((9 as libc::c_int * m) as isize);
    let mut sub_map: *const libc::c_int = (*(*s).sub).map;
    stride = (stride as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<TXSample>() as libc::c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < m {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 9 as libc::c_int {
            let k: libc::c_int = *in_map.offset((i * 9 as libc::c_int + j) as isize);
            if k < len4 {
                tmp.re = -*src.offset((len4 + k) as isize)
                    + *src.offset((1 as libc::c_int * len4 - 1 as libc::c_int - k) as isize);
                tmp.im = -*src.offset((len3 + k) as isize)
                    + -*src.offset((1 as libc::c_int * len3 - 1 as libc::c_int - k) as isize);
            } else {
                tmp.re = -*src.offset((len4 + k) as isize)
                    + -*src.offset((5 as libc::c_int * len4 - 1 as libc::c_int - k) as isize);
                tmp.im = *src.offset((-len4 + k) as isize)
                    + -*src.offset((1 as libc::c_int * len3 - 1 as libc::c_int - k) as isize);
            }
            fft9in[j as usize].im = tmp.re * (*exp.offset((k >> 1 as libc::c_int) as isize)).re
                - tmp.im * (*exp.offset((k >> 1 as libc::c_int) as isize)).im;
            fft9in[j as usize].re = tmp.re * (*exp.offset((k >> 1 as libc::c_int) as isize)).im
                + tmp.im * (*exp.offset((k >> 1 as libc::c_int) as isize)).re;
            j += 1;
            j;
        }
        fft9(
            ((*s).tmp)
                .double
                .offset(*sub_map.offset(i as isize) as isize),
            fft9in.as_mut_ptr(),
            m as ptrdiff_t,
        );
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 9 as libc::c_int {
        ((*s).fn_0[0 as libc::c_int as usize]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0 as libc::c_int as isize),
            ((*s).tmp).double.offset((m * i_0) as isize) as *mut libc::c_void,
            ((*s).tmp).double.offset((m * i_0) as isize) as *mut libc::c_void,
            ::core::mem::size_of::<TXComplex>() as libc::c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < len8 {
        let i0: libc::c_int = len8 + i_1;
        let i1: libc::c_int = len8 - i_1 - 1 as libc::c_int;
        let s0: libc::c_int = *out_map.offset(i0 as isize);
        let s1: libc::c_int = *out_map.offset(i1 as isize);
        let mut src1: TXComplex = {
            let mut init = AVComplexDouble {
                re: (*((*s).tmp).double.offset(s1 as isize)).re,
                im: (*((*s).tmp).double.offset(s1 as isize)).im,
            };
            init
        };
        let mut src0: TXComplex = {
            let mut init = AVComplexDouble {
                re: (*((*s).tmp).double.offset(s0 as isize)).re,
                im: (*((*s).tmp).double.offset(s0 as isize)).im,
            };
            init
        };
        *dst.offset(((2 as libc::c_int * i1) as libc::c_long * stride + stride) as isize) =
            src0.re * (*exp.offset(i0 as isize)).im - src0.im * (*exp.offset(i0 as isize)).re;
        *dst.offset(((2 as libc::c_int * i0) as libc::c_long * stride) as isize) =
            src0.re * (*exp.offset(i0 as isize)).re + src0.im * (*exp.offset(i0 as isize)).im;
        *dst.offset(((2 as libc::c_int * i0) as libc::c_long * stride + stride) as isize) =
            src1.re * (*exp.offset(i1 as isize)).im - src1.im * (*exp.offset(i1 as isize)).re;
        *dst.offset(((2 as libc::c_int * i1) as libc::c_long * stride) as isize) =
            src1.re * (*exp.offset(i1 as isize)).re + src1.im * (*exp.offset(i1 as isize)).im;
        i_1 += 1;
        i_1;
    }
}
static mut ff_tx_mdct_pfa_9xM_fwd_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"mdct_pfa_9xM_fwd_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_mdct_pfa_9xM_fwd_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_MDCT,
            flags: (AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | (1 as libc::c_ulonglong) << 59 as libc::c_int) as uint64_t,
            factors: [
                9 as libc::c_int,
                -(1 as libc::c_int),
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 2 as libc::c_int,
            min_len: 9 as libc::c_int * 2 as libc::c_int,
            max_len: -(1 as libc::c_int),
            init: Some(
                ff_tx_mdct_pfa_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn ff_tx_mdct_pfa_15xM_fwd_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut fft15in: [TXComplex; 15] = [TXComplex { re: 0., im: 0. }; 15];
    let mut src: *mut TXSample = _src as *mut TXSample;
    let mut dst: *mut TXSample = _dst as *mut TXSample;
    let mut exp: *mut TXComplex = (*s).exp.double;
    let mut tmp: TXComplex = TXComplex { re: 0., im: 0. };
    let m: libc::c_int = (*(*s).sub).len;
    let len4: libc::c_int = 15 as libc::c_int * m;
    let len3: libc::c_int = len4 * 3 as libc::c_int;
    let len8: libc::c_int = (*s).len >> 2 as libc::c_int;
    let mut in_map: *const libc::c_int = (*s).map;
    let mut out_map: *const libc::c_int = in_map.offset((15 as libc::c_int * m) as isize);
    let mut sub_map: *const libc::c_int = (*(*s).sub).map;
    stride = (stride as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<TXSample>() as libc::c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < m {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 15 as libc::c_int {
            let k: libc::c_int = *in_map.offset((i * 15 as libc::c_int + j) as isize);
            if k < len4 {
                tmp.re = -*src.offset((len4 + k) as isize)
                    + *src.offset((1 as libc::c_int * len4 - 1 as libc::c_int - k) as isize);
                tmp.im = -*src.offset((len3 + k) as isize)
                    + -*src.offset((1 as libc::c_int * len3 - 1 as libc::c_int - k) as isize);
            } else {
                tmp.re = -*src.offset((len4 + k) as isize)
                    + -*src.offset((5 as libc::c_int * len4 - 1 as libc::c_int - k) as isize);
                tmp.im = *src.offset((-len4 + k) as isize)
                    + -*src.offset((1 as libc::c_int * len3 - 1 as libc::c_int - k) as isize);
            }
            fft15in[j as usize].im = tmp.re * (*exp.offset((k >> 1 as libc::c_int) as isize)).re
                - tmp.im * (*exp.offset((k >> 1 as libc::c_int) as isize)).im;
            fft15in[j as usize].re = tmp.re * (*exp.offset((k >> 1 as libc::c_int) as isize)).im
                + tmp.im * (*exp.offset((k >> 1 as libc::c_int) as isize)).re;
            j += 1;
            j;
        }
        fft15(
            ((*s).tmp)
                .double
                .offset(*sub_map.offset(i as isize) as isize),
            fft15in.as_mut_ptr(),
            m as ptrdiff_t,
        );
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 15 as libc::c_int {
        ((*s).fn_0[0 as libc::c_int as usize]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0 as libc::c_int as isize),
            ((*s).tmp).double.offset((m * i_0) as isize) as *mut libc::c_void,
            ((*s).tmp).double.offset((m * i_0) as isize) as *mut libc::c_void,
            ::core::mem::size_of::<TXComplex>() as libc::c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < len8 {
        let i0: libc::c_int = len8 + i_1;
        let i1: libc::c_int = len8 - i_1 - 1 as libc::c_int;
        let s0: libc::c_int = *out_map.offset(i0 as isize);
        let s1: libc::c_int = *out_map.offset(i1 as isize);
        let mut src1: TXComplex = {
            let mut init = AVComplexDouble {
                re: (*((*s).tmp).double.offset(s1 as isize)).re,
                im: (*((*s).tmp).double.offset(s1 as isize)).im,
            };
            init
        };
        let mut src0: TXComplex = {
            let mut init = AVComplexDouble {
                re: (*((*s).tmp).double.offset(s0 as isize)).re,
                im: (*((*s).tmp).double.offset(s0 as isize)).im,
            };
            init
        };
        *dst.offset(((2 as libc::c_int * i1) as libc::c_long * stride + stride) as isize) =
            src0.re * (*exp.offset(i0 as isize)).im - src0.im * (*exp.offset(i0 as isize)).re;
        *dst.offset(((2 as libc::c_int * i0) as libc::c_long * stride) as isize) =
            src0.re * (*exp.offset(i0 as isize)).re + src0.im * (*exp.offset(i0 as isize)).im;
        *dst.offset(((2 as libc::c_int * i0) as libc::c_long * stride + stride) as isize) =
            src1.re * (*exp.offset(i1 as isize)).im - src1.im * (*exp.offset(i1 as isize)).re;
        *dst.offset(((2 as libc::c_int * i1) as libc::c_long * stride) as isize) =
            src1.re * (*exp.offset(i1 as isize)).re + src1.im * (*exp.offset(i1 as isize)).im;
        i_1 += 1;
        i_1;
    }
}
static mut ff_tx_mdct_pfa_15xM_fwd_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"mdct_pfa_15xM_fwd_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_mdct_pfa_15xM_fwd_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_MDCT,
            flags: (AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | (1 as libc::c_ulonglong) << 59 as libc::c_int) as uint64_t,
            factors: [
                15 as libc::c_int,
                -(1 as libc::c_int),
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 2 as libc::c_int,
            min_len: 15 as libc::c_int * 2 as libc::c_int,
            max_len: -(1 as libc::c_int),
            init: Some(
                ff_tx_mdct_pfa_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
#[cold]
unsafe extern "C" fn ff_tx_rdft_init_double_c(
    mut s: *mut AVTXContext,
    mut cd: *const FFTXCodelet,
    mut flags: uint64_t,
    mut opts: *mut FFTXCodeletOptions,
    mut len: libc::c_int,
    mut inv: libc::c_int,
    mut scale: *const libc::c_void,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut f: libc::c_double = 0.;
    let mut m: libc::c_double = 0.;
    let mut tab: *mut TXSample = 0 as *mut TXSample;
    let mut r2r: uint64_t = flags & AV_TX_REAL_TO_REAL as libc::c_int as libc::c_ulong;
    let mut len4: libc::c_int = (len + 4 as libc::c_int - 1 as libc::c_int
        & !(4 as libc::c_int - 1 as libc::c_int))
        / 4 as libc::c_int;
    (*s).scale_d = *(scale as *mut libc::c_double);
    (*s).scale_f = (*s).scale_d as libc::c_float;
    flags &= !(AV_TX_REAL_TO_REAL as libc::c_int | AV_TX_REAL_TO_IMAGINARY as libc::c_int)
        as libc::c_ulong;
    ret = ff_tx_init_subtx(
        s,
        AV_TX_DOUBLE_FFT,
        flags,
        0 as *mut FFTXCodeletOptions,
        len >> 1 as libc::c_int,
        inv,
        scale,
    );
    if ret != 0 {
        return ret;
    }
    (*s).exp = AVTXNum {
        double: av_mallocz(
            ((8 as libc::c_int + 2 as libc::c_int * len4) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<TXComplex>() as libc::c_ulong),
        ) as *mut TXComplex,
    };
    if ((*s).exp).double.is_null() {
        return -(12 as libc::c_int);
    }
    tab = (*s).exp.double as *mut TXSample;
    f = 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64 / len as libc::c_double;
    m = if inv != 0 {
        2 as libc::c_int as libc::c_double * (*s).scale_d
    } else {
        (*s).scale_d
    };
    let fresh27 = tab;
    tab = tab.offset(1);
    *fresh27 = (if inv != 0 { 0.5f64 } else { 1.0f64 }) * m;
    let fresh28 = tab;
    tab = tab.offset(1);
    *fresh28 = if inv != 0 { 0.5f64 * m } else { 1.0f64 * m };
    let fresh29 = tab;
    tab = tab.offset(1);
    *fresh29 = m;
    let fresh30 = tab;
    tab = tab.offset(1);
    *fresh30 = -m;
    let fresh31 = tab;
    tab = tab.offset(1);
    *fresh31 = (0.5f64 - 0.0f64) * m;
    if r2r != 0 {
        let fresh32 = tab;
        tab = tab.offset(1);
        *fresh32 = (1 as libc::c_int as libc::c_float / (*s).scale_f) as TXSample;
    } else {
        let fresh33 = tab;
        tab = tab.offset(1);
        *fresh33 = (0.0f64 - 0.5f64) * m;
    }
    let fresh34 = tab;
    tab = tab.offset(1);
    *fresh34 = (0.5f64 - inv as libc::c_double) * m;
    let fresh35 = tab;
    tab = tab.offset(1);
    *fresh35 = -(0.5f64 - inv as libc::c_double) * m;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < len4 {
        let fresh36 = tab;
        tab = tab.offset(1);
        *fresh36 = cos(i as libc::c_double * f);
        i += 1;
        i;
    }
    tab = ((*s).exp.double as *mut TXSample)
        .offset(len4 as isize)
        .offset(8 as libc::c_int as isize);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < len4 {
        let fresh37 = tab;
        tab = tab.offset(1);
        *fresh37 = cos((len - i_0 * 4 as libc::c_int) as libc::c_double / 4.0f64 * f)
            * (if inv != 0 {
                1 as libc::c_int
            } else {
                -(1 as libc::c_int)
            }) as libc::c_double;
        i_0 += 1;
        i_0;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ff_tx_rdft_r2c_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let len2: libc::c_int = (*s).len >> 1 as libc::c_int;
    let len4: libc::c_int = (*s).len >> 2 as libc::c_int;
    let mut fact: *const TXSample = (*s).exp.double as *mut libc::c_void as *const TXSample;
    let mut tcos: *const TXSample = fact.offset(8 as libc::c_int as isize);
    let mut tsin: *const TXSample = tcos.offset(len4 as isize);
    let mut data: *mut TXComplex =
        (if 0 as libc::c_int != 0 { _src } else { _dst }) as *mut TXComplex;
    let mut t: [TXComplex; 3] = [TXComplex { re: 0., im: 0. }; 3];
    if 0 as libc::c_int == 0 {
        ((*s).fn_0[0 as libc::c_int as usize]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0 as libc::c_int as isize),
            data as *mut libc::c_void,
            _src,
            ::core::mem::size_of::<TXComplex>() as libc::c_ulong as ptrdiff_t,
        );
    } else {
        (*data.offset(0 as libc::c_int as isize)).im = (*data.offset(len2 as isize)).re;
    }
    t[0 as libc::c_int as usize].re = (*data.offset(0 as libc::c_int as isize)).re;
    (*data.offset(0 as libc::c_int as isize)).re =
        t[0 as libc::c_int as usize].re + (*data.offset(0 as libc::c_int as isize)).im;
    (*data.offset(0 as libc::c_int as isize)).im =
        t[0 as libc::c_int as usize].re - (*data.offset(0 as libc::c_int as isize)).im;
    (*data.offset(0 as libc::c_int as isize)).re =
        *fact.offset(0 as libc::c_int as isize) * (*data.offset(0 as libc::c_int as isize)).re;
    (*data.offset(0 as libc::c_int as isize)).im =
        *fact.offset(1 as libc::c_int as isize) * (*data.offset(0 as libc::c_int as isize)).im;
    (*data.offset(len4 as isize)).re =
        *fact.offset(2 as libc::c_int as isize) * (*data.offset(len4 as isize)).re;
    (*data.offset(len4 as isize)).im =
        *fact.offset(3 as libc::c_int as isize) * (*data.offset(len4 as isize)).im;
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < len4 {
        t[0 as libc::c_int as usize].re = *fact.offset(4 as libc::c_int as isize)
            * ((*data.offset(i as isize)).re + (*data.offset((len2 - i) as isize)).re);
        t[0 as libc::c_int as usize].im = *fact.offset(5 as libc::c_int as isize)
            * ((*data.offset(i as isize)).im - (*data.offset((len2 - i) as isize)).im);
        t[1 as libc::c_int as usize].re = *fact.offset(6 as libc::c_int as isize)
            * ((*data.offset(i as isize)).im + (*data.offset((len2 - i) as isize)).im);
        t[1 as libc::c_int as usize].im = *fact.offset(7 as libc::c_int as isize)
            * ((*data.offset(i as isize)).re - (*data.offset((len2 - i) as isize)).re);
        t[2 as libc::c_int as usize].re = t[1 as libc::c_int as usize].re
            * *tcos.offset(i as isize)
            - t[1 as libc::c_int as usize].im * *tsin.offset(i as isize);
        t[2 as libc::c_int as usize].im = t[1 as libc::c_int as usize].re
            * *tsin.offset(i as isize)
            + t[1 as libc::c_int as usize].im * *tcos.offset(i as isize);
        (*data.offset(i as isize)).re =
            t[0 as libc::c_int as usize].re + t[2 as libc::c_int as usize].re;
        (*data.offset(i as isize)).im =
            t[2 as libc::c_int as usize].im - t[0 as libc::c_int as usize].im;
        (*data.offset((len2 - i) as isize)).re =
            t[0 as libc::c_int as usize].re - t[2 as libc::c_int as usize].re;
        (*data.offset((len2 - i) as isize)).im =
            t[2 as libc::c_int as usize].im + t[0 as libc::c_int as usize].im;
        i += 1;
        i;
    }
    (*data.offset(len2 as isize)).re = (*data.offset(0 as libc::c_int as isize)).im;
    let ref mut fresh38 = (*data.offset(len2 as isize)).im;
    *fresh38 = 0 as libc::c_int as libc::c_double;
    (*data.offset(0 as libc::c_int as isize)).im = *fresh38;
}
static mut ff_tx_rdft_r2c_def_double_c: FFTXCodelet = FFTXCodelet {
    name: 0 as *const libc::c_char,
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
unsafe extern "C" fn ff_tx_rdft_c2r_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let len2: libc::c_int = (*s).len >> 1 as libc::c_int;
    let len4: libc::c_int = (*s).len >> 2 as libc::c_int;
    let mut fact: *const TXSample = (*s).exp.double as *mut libc::c_void as *const TXSample;
    let mut tcos: *const TXSample = fact.offset(8 as libc::c_int as isize);
    let mut tsin: *const TXSample = tcos.offset(len4 as isize);
    let mut data: *mut TXComplex =
        (if 1 as libc::c_int != 0 { _src } else { _dst }) as *mut TXComplex;
    let mut t: [TXComplex; 3] = [TXComplex { re: 0., im: 0. }; 3];
    if 1 as libc::c_int == 0 {
        ((*s).fn_0[0 as libc::c_int as usize]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0 as libc::c_int as isize),
            data as *mut libc::c_void,
            _src,
            ::core::mem::size_of::<TXComplex>() as libc::c_ulong as ptrdiff_t,
        );
    } else {
        (*data.offset(0 as libc::c_int as isize)).im = (*data.offset(len2 as isize)).re;
    }
    t[0 as libc::c_int as usize].re = (*data.offset(0 as libc::c_int as isize)).re;
    (*data.offset(0 as libc::c_int as isize)).re =
        t[0 as libc::c_int as usize].re + (*data.offset(0 as libc::c_int as isize)).im;
    (*data.offset(0 as libc::c_int as isize)).im =
        t[0 as libc::c_int as usize].re - (*data.offset(0 as libc::c_int as isize)).im;
    (*data.offset(0 as libc::c_int as isize)).re =
        *fact.offset(0 as libc::c_int as isize) * (*data.offset(0 as libc::c_int as isize)).re;
    (*data.offset(0 as libc::c_int as isize)).im =
        *fact.offset(1 as libc::c_int as isize) * (*data.offset(0 as libc::c_int as isize)).im;
    (*data.offset(len4 as isize)).re =
        *fact.offset(2 as libc::c_int as isize) * (*data.offset(len4 as isize)).re;
    (*data.offset(len4 as isize)).im =
        *fact.offset(3 as libc::c_int as isize) * (*data.offset(len4 as isize)).im;
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < len4 {
        t[0 as libc::c_int as usize].re = *fact.offset(4 as libc::c_int as isize)
            * ((*data.offset(i as isize)).re + (*data.offset((len2 - i) as isize)).re);
        t[0 as libc::c_int as usize].im = *fact.offset(5 as libc::c_int as isize)
            * ((*data.offset(i as isize)).im - (*data.offset((len2 - i) as isize)).im);
        t[1 as libc::c_int as usize].re = *fact.offset(6 as libc::c_int as isize)
            * ((*data.offset(i as isize)).im + (*data.offset((len2 - i) as isize)).im);
        t[1 as libc::c_int as usize].im = *fact.offset(7 as libc::c_int as isize)
            * ((*data.offset(i as isize)).re - (*data.offset((len2 - i) as isize)).re);
        t[2 as libc::c_int as usize].re = t[1 as libc::c_int as usize].re
            * *tcos.offset(i as isize)
            - t[1 as libc::c_int as usize].im * *tsin.offset(i as isize);
        t[2 as libc::c_int as usize].im = t[1 as libc::c_int as usize].re
            * *tsin.offset(i as isize)
            + t[1 as libc::c_int as usize].im * *tcos.offset(i as isize);
        (*data.offset(i as isize)).re =
            t[0 as libc::c_int as usize].re + t[2 as libc::c_int as usize].re;
        (*data.offset(i as isize)).im =
            t[2 as libc::c_int as usize].im - t[0 as libc::c_int as usize].im;
        (*data.offset((len2 - i) as isize)).re =
            t[0 as libc::c_int as usize].re - t[2 as libc::c_int as usize].re;
        (*data.offset((len2 - i) as isize)).im =
            t[2 as libc::c_int as usize].im + t[0 as libc::c_int as usize].im;
        i += 1;
        i;
    }
    ((*s).fn_0[0 as libc::c_int as usize]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0 as libc::c_int as isize),
        _dst,
        data as *mut libc::c_void,
        ::core::mem::size_of::<TXComplex>() as libc::c_ulong as ptrdiff_t,
    );
}
static mut ff_tx_rdft_c2r_def_double_c: FFTXCodelet = FFTXCodelet {
    name: 0 as *const libc::c_char,
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
unsafe extern "C" fn ff_tx_rdft_r2r_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let len: libc::c_int = (*s).len;
    let len2: libc::c_int = len >> 1 as libc::c_int;
    let len4: libc::c_int = len >> 2 as libc::c_int;
    let aligned_len4: libc::c_int = (len + 4 as libc::c_int - 1 as libc::c_int
        & !(4 as libc::c_int - 1 as libc::c_int))
        / 4 as libc::c_int;
    let mut fact: *const TXSample = (*s).exp.double as *mut libc::c_void as *const TXSample;
    let mut tcos: *const TXSample = fact.offset(8 as libc::c_int as isize);
    let mut tsin: *const TXSample = tcos.offset(aligned_len4 as isize);
    let mut data: *mut TXComplex = _dst as *mut TXComplex;
    let mut out: *mut TXSample = _dst as *mut TXSample;
    let mut tmp_dc: TXSample = 0.;
    let mut tmp_mid: TXSample = 0.;
    let mut tmp: [TXSample; 4] = [0.; 4];
    let mut sf: TXComplex = TXComplex { re: 0., im: 0. };
    let mut sl: TXComplex = TXComplex { re: 0., im: 0. };
    ((*s).fn_0[0 as libc::c_int as usize]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0 as libc::c_int as isize),
        _dst,
        _src,
        ::core::mem::size_of::<TXComplex>() as libc::c_ulong as ptrdiff_t,
    );
    tmp_dc = (*data.offset(0 as libc::c_int as isize)).re;
    (*data.offset(0 as libc::c_int as isize)).re =
        tmp_dc + (*data.offset(0 as libc::c_int as isize)).im;
    tmp_dc = tmp_dc - (*data.offset(0 as libc::c_int as isize)).im;
    (*data.offset(0 as libc::c_int as isize)).re =
        *fact.offset(0 as libc::c_int as isize) * (*data.offset(0 as libc::c_int as isize)).re;
    tmp_dc = *fact.offset(1 as libc::c_int as isize) * tmp_dc;
    (*data.offset(len4 as isize)).re =
        *fact.offset(2 as libc::c_int as isize) * (*data.offset(len4 as isize)).re;
    if 0 as libc::c_int == 0 {
        (*data.offset(len4 as isize)).im =
            *fact.offset(3 as libc::c_int as isize) * (*data.offset(len4 as isize)).im;
    } else {
        sf = *data.offset(len4 as isize);
        sl = *data.offset((len4 + 1 as libc::c_int) as isize);
        if AV_TX_REAL_TO_REAL as libc::c_int == AV_TX_REAL_TO_REAL as libc::c_int {
            tmp[0 as libc::c_int as usize] =
                *fact.offset(4 as libc::c_int as isize) * (sf.re + sl.re);
        } else {
            tmp[0 as libc::c_int as usize] =
                *fact.offset(5 as libc::c_int as isize) * (sf.im - sl.im);
        }
        tmp[1 as libc::c_int as usize] = *fact.offset(6 as libc::c_int as isize) * (sf.im + sl.im);
        tmp[2 as libc::c_int as usize] = *fact.offset(7 as libc::c_int as isize) * (sf.re - sl.re);
        if AV_TX_REAL_TO_REAL as libc::c_int == AV_TX_REAL_TO_REAL as libc::c_int {
            tmp[3 as libc::c_int as usize] = tmp[1 as libc::c_int as usize]
                * *tcos.offset(len4 as isize)
                - tmp[2 as libc::c_int as usize] * *tsin.offset(len4 as isize);
            tmp_mid = tmp[0 as libc::c_int as usize] - tmp[3 as libc::c_int as usize];
        } else {
            tmp[3 as libc::c_int as usize] = tmp[1 as libc::c_int as usize]
                * *tsin.offset(len4 as isize)
                + tmp[2 as libc::c_int as usize] * *tcos.offset(len4 as isize);
            tmp_mid = tmp[0 as libc::c_int as usize] + tmp[3 as libc::c_int as usize];
        }
    }
    let mut i: libc::c_int = 1 as libc::c_int;
    while i <= len4 {
        let mut tmp_0: [TXSample; 4] = [0.; 4];
        let mut sf_0: TXComplex = *data.offset(i as isize);
        let mut sl_0: TXComplex = *data.offset((len2 - i) as isize);
        if AV_TX_REAL_TO_REAL as libc::c_int == AV_TX_REAL_TO_REAL as libc::c_int {
            tmp_0[0 as libc::c_int as usize] =
                *fact.offset(4 as libc::c_int as isize) * (sf_0.re + sl_0.re);
        } else {
            tmp_0[0 as libc::c_int as usize] =
                *fact.offset(5 as libc::c_int as isize) * (sf_0.im - sl_0.im);
        }
        tmp_0[1 as libc::c_int as usize] =
            *fact.offset(6 as libc::c_int as isize) * (sf_0.im + sl_0.im);
        tmp_0[2 as libc::c_int as usize] =
            *fact.offset(7 as libc::c_int as isize) * (sf_0.re - sl_0.re);
        if AV_TX_REAL_TO_REAL as libc::c_int == AV_TX_REAL_TO_REAL as libc::c_int {
            tmp_0[3 as libc::c_int as usize] = tmp_0[1 as libc::c_int as usize]
                * *tcos.offset(i as isize)
                - tmp_0[2 as libc::c_int as usize] * *tsin.offset(i as isize);
            *out.offset(i as isize) =
                tmp_0[0 as libc::c_int as usize] + tmp_0[3 as libc::c_int as usize];
            *out.offset((len - i) as isize) =
                tmp_0[0 as libc::c_int as usize] - tmp_0[3 as libc::c_int as usize];
        } else {
            tmp_0[3 as libc::c_int as usize] = tmp_0[1 as libc::c_int as usize]
                * *tsin.offset(i as isize)
                + tmp_0[2 as libc::c_int as usize] * *tcos.offset(i as isize);
            *out.offset((i - 1 as libc::c_int) as isize) =
                tmp_0[3 as libc::c_int as usize] - tmp_0[0 as libc::c_int as usize];
            *out.offset((len - i - 1 as libc::c_int) as isize) =
                tmp_0[0 as libc::c_int as usize] + tmp_0[3 as libc::c_int as usize];
        }
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 1 as libc::c_int;
    while i_0
        < len4
            + (AV_TX_REAL_TO_REAL as libc::c_int == AV_TX_REAL_TO_IMAGINARY as libc::c_int)
                as libc::c_int
    {
        *out.offset((len2 - i_0) as isize) = *out.offset((len - i_0) as isize);
        i_0 += 1;
        i_0;
    }
    if AV_TX_REAL_TO_REAL as libc::c_int == AV_TX_REAL_TO_REAL as libc::c_int {
        *out.offset(len2 as isize) = tmp_dc;
    }
}
static mut ff_tx_rdft_r2r_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"rdft_r2r_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_rdft_r2r_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_RDFT,
            flags: ((AV_TX_UNALIGNED as libc::c_int
                | AV_TX_INPLACE as libc::c_int
                | AV_TX_REAL_TO_REAL as libc::c_int) as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | (1 as libc::c_ulonglong) << 59 as libc::c_int) as uint64_t,
            factors: [
                2 as libc::c_int + 2 as libc::c_int * (0 as libc::c_int == 0) as libc::c_int,
                -(1 as libc::c_int),
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 2 as libc::c_int,
            min_len: 2 as libc::c_int + 2 as libc::c_int * (0 as libc::c_int == 0) as libc::c_int,
            max_len: -(1 as libc::c_int),
            init: Some(
                ff_tx_rdft_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn ff_tx_rdft_r2r_mod2_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let len: libc::c_int = (*s).len;
    let len2: libc::c_int = len >> 1 as libc::c_int;
    let len4: libc::c_int = len >> 2 as libc::c_int;
    let aligned_len4: libc::c_int = (len + 4 as libc::c_int - 1 as libc::c_int
        & !(4 as libc::c_int - 1 as libc::c_int))
        / 4 as libc::c_int;
    let mut fact: *const TXSample = (*s).exp.double as *mut libc::c_void as *const TXSample;
    let mut tcos: *const TXSample = fact.offset(8 as libc::c_int as isize);
    let mut tsin: *const TXSample = tcos.offset(aligned_len4 as isize);
    let mut data: *mut TXComplex = _dst as *mut TXComplex;
    let mut out: *mut TXSample = _dst as *mut TXSample;
    let mut tmp_dc: TXSample = 0.;
    let mut tmp_mid: TXSample = 0.;
    let mut tmp: [TXSample; 4] = [0.; 4];
    let mut sf: TXComplex = TXComplex { re: 0., im: 0. };
    let mut sl: TXComplex = TXComplex { re: 0., im: 0. };
    ((*s).fn_0[0 as libc::c_int as usize]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0 as libc::c_int as isize),
        _dst,
        _src,
        ::core::mem::size_of::<TXComplex>() as libc::c_ulong as ptrdiff_t,
    );
    tmp_dc = (*data.offset(0 as libc::c_int as isize)).re;
    (*data.offset(0 as libc::c_int as isize)).re =
        tmp_dc + (*data.offset(0 as libc::c_int as isize)).im;
    tmp_dc = tmp_dc - (*data.offset(0 as libc::c_int as isize)).im;
    (*data.offset(0 as libc::c_int as isize)).re =
        *fact.offset(0 as libc::c_int as isize) * (*data.offset(0 as libc::c_int as isize)).re;
    tmp_dc = *fact.offset(1 as libc::c_int as isize) * tmp_dc;
    (*data.offset(len4 as isize)).re =
        *fact.offset(2 as libc::c_int as isize) * (*data.offset(len4 as isize)).re;
    if 1 as libc::c_int == 0 {
        (*data.offset(len4 as isize)).im =
            *fact.offset(3 as libc::c_int as isize) * (*data.offset(len4 as isize)).im;
    } else {
        sf = *data.offset(len4 as isize);
        sl = *data.offset((len4 + 1 as libc::c_int) as isize);
        if AV_TX_REAL_TO_REAL as libc::c_int == AV_TX_REAL_TO_REAL as libc::c_int {
            tmp[0 as libc::c_int as usize] =
                *fact.offset(4 as libc::c_int as isize) * (sf.re + sl.re);
        } else {
            tmp[0 as libc::c_int as usize] =
                *fact.offset(5 as libc::c_int as isize) * (sf.im - sl.im);
        }
        tmp[1 as libc::c_int as usize] = *fact.offset(6 as libc::c_int as isize) * (sf.im + sl.im);
        tmp[2 as libc::c_int as usize] = *fact.offset(7 as libc::c_int as isize) * (sf.re - sl.re);
        if AV_TX_REAL_TO_REAL as libc::c_int == AV_TX_REAL_TO_REAL as libc::c_int {
            tmp[3 as libc::c_int as usize] = tmp[1 as libc::c_int as usize]
                * *tcos.offset(len4 as isize)
                - tmp[2 as libc::c_int as usize] * *tsin.offset(len4 as isize);
            tmp_mid = tmp[0 as libc::c_int as usize] - tmp[3 as libc::c_int as usize];
        } else {
            tmp[3 as libc::c_int as usize] = tmp[1 as libc::c_int as usize]
                * *tsin.offset(len4 as isize)
                + tmp[2 as libc::c_int as usize] * *tcos.offset(len4 as isize);
            tmp_mid = tmp[0 as libc::c_int as usize] + tmp[3 as libc::c_int as usize];
        }
    }
    let mut i: libc::c_int = 1 as libc::c_int;
    while i <= len4 {
        let mut tmp_0: [TXSample; 4] = [0.; 4];
        let mut sf_0: TXComplex = *data.offset(i as isize);
        let mut sl_0: TXComplex = *data.offset((len2 - i) as isize);
        if AV_TX_REAL_TO_REAL as libc::c_int == AV_TX_REAL_TO_REAL as libc::c_int {
            tmp_0[0 as libc::c_int as usize] =
                *fact.offset(4 as libc::c_int as isize) * (sf_0.re + sl_0.re);
        } else {
            tmp_0[0 as libc::c_int as usize] =
                *fact.offset(5 as libc::c_int as isize) * (sf_0.im - sl_0.im);
        }
        tmp_0[1 as libc::c_int as usize] =
            *fact.offset(6 as libc::c_int as isize) * (sf_0.im + sl_0.im);
        tmp_0[2 as libc::c_int as usize] =
            *fact.offset(7 as libc::c_int as isize) * (sf_0.re - sl_0.re);
        if AV_TX_REAL_TO_REAL as libc::c_int == AV_TX_REAL_TO_REAL as libc::c_int {
            tmp_0[3 as libc::c_int as usize] = tmp_0[1 as libc::c_int as usize]
                * *tcos.offset(i as isize)
                - tmp_0[2 as libc::c_int as usize] * *tsin.offset(i as isize);
            *out.offset(i as isize) =
                tmp_0[0 as libc::c_int as usize] + tmp_0[3 as libc::c_int as usize];
            *out.offset((len - i) as isize) =
                tmp_0[0 as libc::c_int as usize] - tmp_0[3 as libc::c_int as usize];
        } else {
            tmp_0[3 as libc::c_int as usize] = tmp_0[1 as libc::c_int as usize]
                * *tsin.offset(i as isize)
                + tmp_0[2 as libc::c_int as usize] * *tcos.offset(i as isize);
            *out.offset((i - 1 as libc::c_int) as isize) =
                tmp_0[3 as libc::c_int as usize] - tmp_0[0 as libc::c_int as usize];
            *out.offset((len - i - 1 as libc::c_int) as isize) =
                tmp_0[0 as libc::c_int as usize] + tmp_0[3 as libc::c_int as usize];
        }
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 1 as libc::c_int;
    while i_0
        < len4
            + (AV_TX_REAL_TO_REAL as libc::c_int == AV_TX_REAL_TO_IMAGINARY as libc::c_int)
                as libc::c_int
    {
        *out.offset((len2 - i_0) as isize) = *out.offset((len - i_0) as isize);
        i_0 += 1;
        i_0;
    }
    if AV_TX_REAL_TO_REAL as libc::c_int == AV_TX_REAL_TO_REAL as libc::c_int {
        *out.offset(len2 as isize) = tmp_dc;
        *out.offset((len4 + 1 as libc::c_int) as isize) =
            tmp_mid * *fact.offset(5 as libc::c_int as isize);
    } else {
        *out.offset(len4 as isize) = tmp_mid;
    };
}
static mut ff_tx_rdft_r2r_mod2_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"rdft_r2r_mod2_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_rdft_r2r_mod2_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_RDFT,
            flags: ((AV_TX_UNALIGNED as libc::c_int
                | AV_TX_INPLACE as libc::c_int
                | AV_TX_REAL_TO_REAL as libc::c_int) as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | (1 as libc::c_ulonglong) << 59 as libc::c_int) as uint64_t,
            factors: [
                2 as libc::c_int + 2 as libc::c_int * (1 as libc::c_int == 0) as libc::c_int,
                -(1 as libc::c_int),
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 2 as libc::c_int,
            min_len: 2 as libc::c_int + 2 as libc::c_int * (1 as libc::c_int == 0) as libc::c_int,
            max_len: -(1 as libc::c_int),
            init: Some(
                ff_tx_rdft_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
static mut ff_tx_rdft_r2i_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"rdft_r2i_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_rdft_r2i_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_RDFT,
            flags: ((AV_TX_UNALIGNED as libc::c_int
                | AV_TX_INPLACE as libc::c_int
                | AV_TX_REAL_TO_IMAGINARY as libc::c_int) as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | (1 as libc::c_ulonglong) << 59 as libc::c_int) as uint64_t,
            factors: [
                2 as libc::c_int + 2 as libc::c_int * (0 as libc::c_int == 0) as libc::c_int,
                -(1 as libc::c_int),
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 2 as libc::c_int,
            min_len: 2 as libc::c_int + 2 as libc::c_int * (0 as libc::c_int == 0) as libc::c_int,
            max_len: -(1 as libc::c_int),
            init: Some(
                ff_tx_rdft_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn ff_tx_rdft_r2i_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let len: libc::c_int = (*s).len;
    let len2: libc::c_int = len >> 1 as libc::c_int;
    let len4: libc::c_int = len >> 2 as libc::c_int;
    let aligned_len4: libc::c_int = (len + 4 as libc::c_int - 1 as libc::c_int
        & !(4 as libc::c_int - 1 as libc::c_int))
        / 4 as libc::c_int;
    let mut fact: *const TXSample = (*s).exp.double as *mut libc::c_void as *const TXSample;
    let mut tcos: *const TXSample = fact.offset(8 as libc::c_int as isize);
    let mut tsin: *const TXSample = tcos.offset(aligned_len4 as isize);
    let mut data: *mut TXComplex = _dst as *mut TXComplex;
    let mut out: *mut TXSample = _dst as *mut TXSample;
    let mut tmp_dc: TXSample = 0.;
    let mut tmp_mid: TXSample = 0.;
    let mut tmp: [TXSample; 4] = [0.; 4];
    let mut sf: TXComplex = TXComplex { re: 0., im: 0. };
    let mut sl: TXComplex = TXComplex { re: 0., im: 0. };
    ((*s).fn_0[0 as libc::c_int as usize]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0 as libc::c_int as isize),
        _dst,
        _src,
        ::core::mem::size_of::<TXComplex>() as libc::c_ulong as ptrdiff_t,
    );
    tmp_dc = (*data.offset(0 as libc::c_int as isize)).re;
    (*data.offset(0 as libc::c_int as isize)).re =
        tmp_dc + (*data.offset(0 as libc::c_int as isize)).im;
    tmp_dc = tmp_dc - (*data.offset(0 as libc::c_int as isize)).im;
    (*data.offset(0 as libc::c_int as isize)).re =
        *fact.offset(0 as libc::c_int as isize) * (*data.offset(0 as libc::c_int as isize)).re;
    tmp_dc = *fact.offset(1 as libc::c_int as isize) * tmp_dc;
    (*data.offset(len4 as isize)).re =
        *fact.offset(2 as libc::c_int as isize) * (*data.offset(len4 as isize)).re;
    if 0 as libc::c_int == 0 {
        (*data.offset(len4 as isize)).im =
            *fact.offset(3 as libc::c_int as isize) * (*data.offset(len4 as isize)).im;
    } else {
        sf = *data.offset(len4 as isize);
        sl = *data.offset((len4 + 1 as libc::c_int) as isize);
        if AV_TX_REAL_TO_IMAGINARY as libc::c_int == AV_TX_REAL_TO_REAL as libc::c_int {
            tmp[0 as libc::c_int as usize] =
                *fact.offset(4 as libc::c_int as isize) * (sf.re + sl.re);
        } else {
            tmp[0 as libc::c_int as usize] =
                *fact.offset(5 as libc::c_int as isize) * (sf.im - sl.im);
        }
        tmp[1 as libc::c_int as usize] = *fact.offset(6 as libc::c_int as isize) * (sf.im + sl.im);
        tmp[2 as libc::c_int as usize] = *fact.offset(7 as libc::c_int as isize) * (sf.re - sl.re);
        if AV_TX_REAL_TO_IMAGINARY as libc::c_int == AV_TX_REAL_TO_REAL as libc::c_int {
            tmp[3 as libc::c_int as usize] = tmp[1 as libc::c_int as usize]
                * *tcos.offset(len4 as isize)
                - tmp[2 as libc::c_int as usize] * *tsin.offset(len4 as isize);
            tmp_mid = tmp[0 as libc::c_int as usize] - tmp[3 as libc::c_int as usize];
        } else {
            tmp[3 as libc::c_int as usize] = tmp[1 as libc::c_int as usize]
                * *tsin.offset(len4 as isize)
                + tmp[2 as libc::c_int as usize] * *tcos.offset(len4 as isize);
            tmp_mid = tmp[0 as libc::c_int as usize] + tmp[3 as libc::c_int as usize];
        }
    }
    let mut i: libc::c_int = 1 as libc::c_int;
    while i <= len4 {
        let mut tmp_0: [TXSample; 4] = [0.; 4];
        let mut sf_0: TXComplex = *data.offset(i as isize);
        let mut sl_0: TXComplex = *data.offset((len2 - i) as isize);
        if AV_TX_REAL_TO_IMAGINARY as libc::c_int == AV_TX_REAL_TO_REAL as libc::c_int {
            tmp_0[0 as libc::c_int as usize] =
                *fact.offset(4 as libc::c_int as isize) * (sf_0.re + sl_0.re);
        } else {
            tmp_0[0 as libc::c_int as usize] =
                *fact.offset(5 as libc::c_int as isize) * (sf_0.im - sl_0.im);
        }
        tmp_0[1 as libc::c_int as usize] =
            *fact.offset(6 as libc::c_int as isize) * (sf_0.im + sl_0.im);
        tmp_0[2 as libc::c_int as usize] =
            *fact.offset(7 as libc::c_int as isize) * (sf_0.re - sl_0.re);
        if AV_TX_REAL_TO_IMAGINARY as libc::c_int == AV_TX_REAL_TO_REAL as libc::c_int {
            tmp_0[3 as libc::c_int as usize] = tmp_0[1 as libc::c_int as usize]
                * *tcos.offset(i as isize)
                - tmp_0[2 as libc::c_int as usize] * *tsin.offset(i as isize);
            *out.offset(i as isize) =
                tmp_0[0 as libc::c_int as usize] + tmp_0[3 as libc::c_int as usize];
            *out.offset((len - i) as isize) =
                tmp_0[0 as libc::c_int as usize] - tmp_0[3 as libc::c_int as usize];
        } else {
            tmp_0[3 as libc::c_int as usize] = tmp_0[1 as libc::c_int as usize]
                * *tsin.offset(i as isize)
                + tmp_0[2 as libc::c_int as usize] * *tcos.offset(i as isize);
            *out.offset((i - 1 as libc::c_int) as isize) =
                tmp_0[3 as libc::c_int as usize] - tmp_0[0 as libc::c_int as usize];
            *out.offset((len - i - 1 as libc::c_int) as isize) =
                tmp_0[0 as libc::c_int as usize] + tmp_0[3 as libc::c_int as usize];
        }
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 1 as libc::c_int;
    while i_0
        < len4
            + (AV_TX_REAL_TO_IMAGINARY as libc::c_int == AV_TX_REAL_TO_IMAGINARY as libc::c_int)
                as libc::c_int
    {
        *out.offset((len2 - i_0) as isize) = *out.offset((len - i_0) as isize);
        i_0 += 1;
        i_0;
    }
    if AV_TX_REAL_TO_IMAGINARY as libc::c_int == AV_TX_REAL_TO_REAL as libc::c_int {
        *out.offset(len2 as isize) = tmp_dc;
    }
}
static mut ff_tx_rdft_r2i_mod2_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"rdft_r2i_mod2_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_rdft_r2i_mod2_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_RDFT,
            flags: ((AV_TX_UNALIGNED as libc::c_int
                | AV_TX_INPLACE as libc::c_int
                | AV_TX_REAL_TO_IMAGINARY as libc::c_int) as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | (1 as libc::c_ulonglong) << 59 as libc::c_int) as uint64_t,
            factors: [
                2 as libc::c_int + 2 as libc::c_int * (1 as libc::c_int == 0) as libc::c_int,
                -(1 as libc::c_int),
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 2 as libc::c_int,
            min_len: 2 as libc::c_int + 2 as libc::c_int * (1 as libc::c_int == 0) as libc::c_int,
            max_len: -(1 as libc::c_int),
            init: Some(
                ff_tx_rdft_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn ff_tx_rdft_r2i_mod2_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let len: libc::c_int = (*s).len;
    let len2: libc::c_int = len >> 1 as libc::c_int;
    let len4: libc::c_int = len >> 2 as libc::c_int;
    let aligned_len4: libc::c_int = (len + 4 as libc::c_int - 1 as libc::c_int
        & !(4 as libc::c_int - 1 as libc::c_int))
        / 4 as libc::c_int;
    let mut fact: *const TXSample = (*s).exp.double as *mut libc::c_void as *const TXSample;
    let mut tcos: *const TXSample = fact.offset(8 as libc::c_int as isize);
    let mut tsin: *const TXSample = tcos.offset(aligned_len4 as isize);
    let mut data: *mut TXComplex = _dst as *mut TXComplex;
    let mut out: *mut TXSample = _dst as *mut TXSample;
    let mut tmp_dc: TXSample = 0.;
    let mut tmp_mid: TXSample = 0.;
    let mut tmp: [TXSample; 4] = [0.; 4];
    let mut sf: TXComplex = TXComplex { re: 0., im: 0. };
    let mut sl: TXComplex = TXComplex { re: 0., im: 0. };
    ((*s).fn_0[0 as libc::c_int as usize]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0 as libc::c_int as isize),
        _dst,
        _src,
        ::core::mem::size_of::<TXComplex>() as libc::c_ulong as ptrdiff_t,
    );
    tmp_dc = (*data.offset(0 as libc::c_int as isize)).re;
    (*data.offset(0 as libc::c_int as isize)).re =
        tmp_dc + (*data.offset(0 as libc::c_int as isize)).im;
    tmp_dc = tmp_dc - (*data.offset(0 as libc::c_int as isize)).im;
    (*data.offset(0 as libc::c_int as isize)).re =
        *fact.offset(0 as libc::c_int as isize) * (*data.offset(0 as libc::c_int as isize)).re;
    tmp_dc = *fact.offset(1 as libc::c_int as isize) * tmp_dc;
    (*data.offset(len4 as isize)).re =
        *fact.offset(2 as libc::c_int as isize) * (*data.offset(len4 as isize)).re;
    if 1 as libc::c_int == 0 {
        (*data.offset(len4 as isize)).im =
            *fact.offset(3 as libc::c_int as isize) * (*data.offset(len4 as isize)).im;
    } else {
        sf = *data.offset(len4 as isize);
        sl = *data.offset((len4 + 1 as libc::c_int) as isize);
        if AV_TX_REAL_TO_IMAGINARY as libc::c_int == AV_TX_REAL_TO_REAL as libc::c_int {
            tmp[0 as libc::c_int as usize] =
                *fact.offset(4 as libc::c_int as isize) * (sf.re + sl.re);
        } else {
            tmp[0 as libc::c_int as usize] =
                *fact.offset(5 as libc::c_int as isize) * (sf.im - sl.im);
        }
        tmp[1 as libc::c_int as usize] = *fact.offset(6 as libc::c_int as isize) * (sf.im + sl.im);
        tmp[2 as libc::c_int as usize] = *fact.offset(7 as libc::c_int as isize) * (sf.re - sl.re);
        if AV_TX_REAL_TO_IMAGINARY as libc::c_int == AV_TX_REAL_TO_REAL as libc::c_int {
            tmp[3 as libc::c_int as usize] = tmp[1 as libc::c_int as usize]
                * *tcos.offset(len4 as isize)
                - tmp[2 as libc::c_int as usize] * *tsin.offset(len4 as isize);
            tmp_mid = tmp[0 as libc::c_int as usize] - tmp[3 as libc::c_int as usize];
        } else {
            tmp[3 as libc::c_int as usize] = tmp[1 as libc::c_int as usize]
                * *tsin.offset(len4 as isize)
                + tmp[2 as libc::c_int as usize] * *tcos.offset(len4 as isize);
            tmp_mid = tmp[0 as libc::c_int as usize] + tmp[3 as libc::c_int as usize];
        }
    }
    let mut i: libc::c_int = 1 as libc::c_int;
    while i <= len4 {
        let mut tmp_0: [TXSample; 4] = [0.; 4];
        let mut sf_0: TXComplex = *data.offset(i as isize);
        let mut sl_0: TXComplex = *data.offset((len2 - i) as isize);
        if AV_TX_REAL_TO_IMAGINARY as libc::c_int == AV_TX_REAL_TO_REAL as libc::c_int {
            tmp_0[0 as libc::c_int as usize] =
                *fact.offset(4 as libc::c_int as isize) * (sf_0.re + sl_0.re);
        } else {
            tmp_0[0 as libc::c_int as usize] =
                *fact.offset(5 as libc::c_int as isize) * (sf_0.im - sl_0.im);
        }
        tmp_0[1 as libc::c_int as usize] =
            *fact.offset(6 as libc::c_int as isize) * (sf_0.im + sl_0.im);
        tmp_0[2 as libc::c_int as usize] =
            *fact.offset(7 as libc::c_int as isize) * (sf_0.re - sl_0.re);
        if AV_TX_REAL_TO_IMAGINARY as libc::c_int == AV_TX_REAL_TO_REAL as libc::c_int {
            tmp_0[3 as libc::c_int as usize] = tmp_0[1 as libc::c_int as usize]
                * *tcos.offset(i as isize)
                - tmp_0[2 as libc::c_int as usize] * *tsin.offset(i as isize);
            *out.offset(i as isize) =
                tmp_0[0 as libc::c_int as usize] + tmp_0[3 as libc::c_int as usize];
            *out.offset((len - i) as isize) =
                tmp_0[0 as libc::c_int as usize] - tmp_0[3 as libc::c_int as usize];
        } else {
            tmp_0[3 as libc::c_int as usize] = tmp_0[1 as libc::c_int as usize]
                * *tsin.offset(i as isize)
                + tmp_0[2 as libc::c_int as usize] * *tcos.offset(i as isize);
            *out.offset((i - 1 as libc::c_int) as isize) =
                tmp_0[3 as libc::c_int as usize] - tmp_0[0 as libc::c_int as usize];
            *out.offset((len - i - 1 as libc::c_int) as isize) =
                tmp_0[0 as libc::c_int as usize] + tmp_0[3 as libc::c_int as usize];
        }
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 1 as libc::c_int;
    while i_0
        < len4
            + (AV_TX_REAL_TO_IMAGINARY as libc::c_int == AV_TX_REAL_TO_IMAGINARY as libc::c_int)
                as libc::c_int
    {
        *out.offset((len2 - i_0) as isize) = *out.offset((len - i_0) as isize);
        i_0 += 1;
        i_0;
    }
    if AV_TX_REAL_TO_IMAGINARY as libc::c_int == AV_TX_REAL_TO_REAL as libc::c_int {
        *out.offset(len2 as isize) = tmp_dc;
        *out.offset((len4 + 1 as libc::c_int) as isize) =
            tmp_mid * *fact.offset(5 as libc::c_int as isize);
    } else {
        *out.offset(len4 as isize) = tmp_mid;
    };
}
#[cold]
unsafe extern "C" fn ff_tx_dct_init_double_c(
    mut s: *mut AVTXContext,
    mut cd: *const FFTXCodelet,
    mut flags: uint64_t,
    mut opts: *mut FFTXCodeletOptions,
    mut len: libc::c_int,
    mut inv: libc::c_int,
    mut scale: *const libc::c_void,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut freq: libc::c_double = 0.;
    let mut tab: *mut TXSample = 0 as *mut TXSample;
    let mut rsc: libc::c_double = *(scale as *mut libc::c_double);
    if inv != 0 {
        len *= 2 as libc::c_int;
        (*s).len *= 2 as libc::c_int;
        rsc *= 0.5f64;
    }
    ret = ff_tx_init_subtx(
        s,
        AV_TX_DOUBLE_RDFT,
        flags,
        0 as *mut FFTXCodeletOptions,
        len,
        inv,
        &mut rsc as *mut libc::c_double as *const libc::c_void,
    );
    if ret != 0 {
        return ret;
    }
    (*s).exp = AVTXNum {
        double: av_malloc(
            ((len / 2 as libc::c_int * 3 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<TXSample>() as libc::c_ulong),
        ) as *mut TXComplex,
    };
    if ((*s).exp).double.is_null() {
        return -(12 as libc::c_int);
    }
    tab = (*s).exp.double as *mut TXSample;
    freq = 3.14159265358979323846f64 / (len * 2 as libc::c_int) as libc::c_double;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < len {
        *tab.offset(i as isize) = cos(i as libc::c_double * freq)
            * ((inv == 0) as libc::c_int + 1 as libc::c_int) as libc::c_double;
        i += 1;
        i;
    }
    if inv != 0 {
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < len / 2 as libc::c_int {
            *tab.offset((len + i_0) as isize) =
                0.5f64 / sin((2 as libc::c_int * i_0 + 1 as libc::c_int) as libc::c_double * freq);
            i_0 += 1;
            i_0;
        }
    } else {
        let mut i_1: libc::c_int = 0 as libc::c_int;
        while i_1 < len / 2 as libc::c_int {
            *tab.offset((len + i_1) as isize) =
                cos((len - 2 as libc::c_int * i_1 - 1 as libc::c_int) as libc::c_double * freq);
            i_1 += 1;
            i_1;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ff_tx_dctII_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut dst: *mut TXSample = _dst as *mut TXSample;
    let mut src: *mut TXSample = _src as *mut TXSample;
    let len: libc::c_int = (*s).len;
    let len2: libc::c_int = len >> 1 as libc::c_int;
    let mut exp: *const TXSample = (*s).exp.double as *mut libc::c_void as *const TXSample;
    let mut next: TXSample = 0.;
    let mut tmp1: TXSample = 0.;
    let mut tmp2: TXSample = 0.;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < len2 {
        let mut in1: TXSample = *src.offset(i as isize);
        let mut in2: TXSample = *src.offset((len - i - 1 as libc::c_int) as isize);
        let mut s_0: TXSample = *exp.offset((len + i) as isize);
        tmp1 = (in1 + in2) * 0.5f64;
        tmp2 = (in1 - in2) * s_0;
        *src.offset(i as isize) = tmp1 + tmp2;
        *src.offset((len - i - 1 as libc::c_int) as isize) = tmp1 - tmp2;
        i += 1;
        i;
    }
    ((*s).fn_0[0 as libc::c_int as usize]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0 as libc::c_int as isize),
        dst as *mut libc::c_void,
        src as *mut libc::c_void,
        ::core::mem::size_of::<TXComplex>() as libc::c_ulong as ptrdiff_t,
    );
    next = *dst.offset(len as isize);
    let mut i_0: libc::c_int = len - 2 as libc::c_int;
    while i_0 > 0 as libc::c_int {
        let mut tmp: TXSample = 0.;
        tmp = *exp.offset((len - i_0) as isize) * *dst.offset((i_0 + 0 as libc::c_int) as isize)
            - *exp.offset(i_0 as isize) * *dst.offset((i_0 + 1 as libc::c_int) as isize);
        *dst.offset(i_0 as isize) = *exp.offset((len - i_0) as isize)
            * *dst.offset((i_0 + 1 as libc::c_int) as isize)
            + *exp.offset(i_0 as isize) * *dst.offset((i_0 + 0 as libc::c_int) as isize);
        *dst.offset((i_0 + 1 as libc::c_int) as isize) = next;
        next += tmp;
        i_0 -= 2 as libc::c_int;
    }
    *dst.offset(0 as libc::c_int as isize) =
        *exp.offset(0 as libc::c_int as isize) * *dst.offset(0 as libc::c_int as isize);
    *dst.offset(1 as libc::c_int as isize) = next;
}
unsafe extern "C" fn ff_tx_dctIII_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut dst: *mut TXSample = _dst as *mut TXSample;
    let mut src: *mut TXSample = _src as *mut TXSample;
    let len: libc::c_int = (*s).len;
    let len2: libc::c_int = len >> 1 as libc::c_int;
    let mut exp: *const TXSample = (*s).exp.double as *mut libc::c_void as *const TXSample;
    let mut tmp1: TXSample = 0.;
    let mut tmp2: TXSample =
        2 as libc::c_int as libc::c_double * *src.offset((len - 1 as libc::c_int) as isize);
    *src.offset(len as isize) = tmp2;
    let mut i: libc::c_int = len - 2 as libc::c_int;
    while i >= 2 as libc::c_int {
        let mut val1: TXSample = *src.offset((i - 0 as libc::c_int) as isize);
        let mut val2: TXSample = *src.offset((i - 1 as libc::c_int) as isize)
            - *src.offset((i + 1 as libc::c_int) as isize);
        *src.offset((i + 1 as libc::c_int) as isize) =
            *exp.offset((len - i) as isize) * val1 - *exp.offset(i as isize) * val2;
        *src.offset(i as isize) =
            *exp.offset((len - i) as isize) * val2 + *exp.offset(i as isize) * val1;
        i -= 2 as libc::c_int;
    }
    ((*s).fn_0[0 as libc::c_int as usize]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0 as libc::c_int as isize),
        dst as *mut libc::c_void,
        src as *mut libc::c_void,
        ::core::mem::size_of::<libc::c_float>() as libc::c_ulong as ptrdiff_t,
    );
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < len2 {
        let mut in1: TXSample = *dst.offset(i_0 as isize);
        let mut in2: TXSample = *dst.offset((len - i_0 - 1 as libc::c_int) as isize);
        let mut c: TXSample = *exp.offset((len + i_0) as isize);
        tmp1 = in1 + in2;
        tmp2 = in1 - in2;
        tmp2 *= c;
        *dst.offset(i_0 as isize) = tmp1 + tmp2;
        *dst.offset((len - i_0 - 1 as libc::c_int) as isize) = tmp1 - tmp2;
        i_0 += 1;
        i_0;
    }
}
static mut ff_tx_dctII_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"dctII_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_dctII_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_DCT,
            flags: ((AV_TX_UNALIGNED as libc::c_int | AV_TX_INPLACE as libc::c_int)
                as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | (1 as libc::c_ulonglong) << 59 as libc::c_int) as uint64_t,
            factors: [
                2 as libc::c_int,
                -(1 as libc::c_int),
                0,
                0,
                0,
                0,
                0,
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
            min_len: 2 as libc::c_int,
            max_len: -(1 as libc::c_int),
            init: Some(
                ff_tx_dct_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
static mut ff_tx_dctIII_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"dctIII_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_dctIII_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_DCT,
            flags: ((AV_TX_UNALIGNED as libc::c_int | AV_TX_INPLACE as libc::c_int)
                as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | (1 as libc::c_ulonglong) << 60 as libc::c_int) as uint64_t,
            factors: [
                2 as libc::c_int,
                -(1 as libc::c_int),
                0,
                0,
                0,
                0,
                0,
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
            min_len: 2 as libc::c_int,
            max_len: -(1 as libc::c_int),
            init: Some(
                ff_tx_dct_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
#[cold]
unsafe extern "C" fn ff_tx_dcstI_init_double_c(
    mut s: *mut AVTXContext,
    mut cd: *const FFTXCodelet,
    mut flags: uint64_t,
    mut opts: *mut FFTXCodeletOptions,
    mut len: libc::c_int,
    mut inv: libc::c_int,
    mut scale: *const libc::c_void,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut rsc: libc::c_double = *(scale as *mut libc::c_double);
    if inv != 0 {
        len *= 2 as libc::c_int;
        (*s).len *= 2 as libc::c_int;
        rsc *= 0.5f64;
    }
    flags |= (if (*cd).type_0 as libc::c_uint == AV_TX_DOUBLE_DCT_I as libc::c_int as libc::c_uint {
        AV_TX_REAL_TO_REAL as libc::c_int
    } else {
        AV_TX_REAL_TO_IMAGINARY as libc::c_int
    }) as libc::c_ulong;
    ret = ff_tx_init_subtx(
        s,
        AV_TX_DOUBLE_RDFT,
        flags,
        0 as *mut FFTXCodeletOptions,
        (len - 1 as libc::c_int
            + 2 as libc::c_int
                * ((*cd).type_0 as libc::c_uint
                    == AV_TX_DOUBLE_DST_I as libc::c_int as libc::c_uint)
                    as libc::c_int)
            * 2 as libc::c_int,
        0 as libc::c_int,
        &mut rsc as *mut libc::c_double as *const libc::c_void,
    );
    if ret != 0 {
        return ret;
    }
    (*s).tmp = AVTXNum {
        double: av_mallocz(
            (((len + 1 as libc::c_int) * 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<TXSample>() as libc::c_ulong),
        ) as *mut TXComplex,
    };
    if ((*s).tmp).double.is_null() {
        return -(12 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ff_tx_dctI_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut dst: *mut TXSample = _dst as *mut TXSample;
    let mut src: *mut TXSample = _src as *mut TXSample;
    let len: libc::c_int = (*s).len - 1 as libc::c_int;
    let mut tmp: *mut TXSample = (*s).tmp.double as *mut TXSample;
    stride = (stride as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<TXSample>() as libc::c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < len {
        let ref mut fresh40 = *tmp.offset((2 as libc::c_int * len - i) as isize);
        *fresh40 = *src.offset((i as libc::c_long * stride) as isize);
        *tmp.offset(i as isize) = *fresh40;
        i += 1;
        i;
    }
    *tmp.offset(len as isize) = *src.offset((len as libc::c_long * stride) as isize);
    ((*s).fn_0[0 as libc::c_int as usize]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0 as libc::c_int as isize),
        dst as *mut libc::c_void,
        tmp as *mut libc::c_void,
        ::core::mem::size_of::<TXSample>() as libc::c_ulong as ptrdiff_t,
    );
}
unsafe extern "C" fn ff_tx_dstI_double_c(
    mut s: *mut AVTXContext,
    mut _dst: *mut libc::c_void,
    mut _src: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    let mut dst: *mut TXSample = _dst as *mut TXSample;
    let mut src: *mut TXSample = _src as *mut TXSample;
    let len: libc::c_int = (*s).len + 1 as libc::c_int;
    let mut tmp: *mut TXSample = (*s).tmp.double as *mut libc::c_void as *mut TXSample;
    stride = (stride as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<TXSample>() as libc::c_ulong) as ptrdiff_t
        as ptrdiff_t;
    *tmp.offset(0 as libc::c_int as isize) = 0 as libc::c_int as TXSample;
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < len {
        let mut a: TXSample =
            *src.offset(((i - 1 as libc::c_int) as libc::c_long * stride) as isize);
        *tmp.offset(i as isize) = -a;
        *tmp.offset((2 as libc::c_int * len - i) as isize) = a;
        i += 1;
        i;
    }
    *tmp.offset(len as isize) = 0 as libc::c_int as TXSample;
    ((*s).fn_0[0 as libc::c_int as usize]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0 as libc::c_int as isize),
        dst as *mut libc::c_void,
        tmp as *mut libc::c_void,
        ::core::mem::size_of::<libc::c_float>() as libc::c_ulong as ptrdiff_t,
    );
}
static mut ff_tx_dctI_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"dctI_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_dctI_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_DCT_I,
            flags: ((AV_TX_UNALIGNED as libc::c_int | AV_TX_INPLACE as libc::c_int)
                as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int) as uint64_t,
            factors: [
                2 as libc::c_int,
                -(1 as libc::c_int),
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 2 as libc::c_int,
            min_len: 2 as libc::c_int,
            max_len: -(1 as libc::c_int),
            init: Some(
                ff_tx_dcstI_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
static mut ff_tx_dstI_def_double_c: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"dstI_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_dstI_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_DST_I,
            flags: ((AV_TX_UNALIGNED as libc::c_int | AV_TX_INPLACE as libc::c_int)
                as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int) as uint64_t,
            factors: [
                2 as libc::c_int,
                -(1 as libc::c_int),
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 2 as libc::c_int,
            min_len: 2 as libc::c_int,
            max_len: -(1 as libc::c_int),
            init: Some(
                ff_tx_dcstI_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn ff_tx_mdct_gen_exp_double(
    mut s: *mut AVTXContext,
    mut pre_tab: *mut libc::c_int,
) -> libc::c_int {
    let mut off: libc::c_int = 0 as libc::c_int;
    let mut len4: libc::c_int = (*s).len >> 1 as libc::c_int;
    let mut scale: libc::c_double = (*s).scale_d;
    let theta: libc::c_double = (if scale < 0 as libc::c_int as libc::c_double {
        len4
    } else {
        0 as libc::c_int
    }) as libc::c_double
        + 1.0f64 / 8.0f64;
    let mut alloc: size_t = (if !pre_tab.is_null() {
        2 as libc::c_int * len4
    } else {
        len4
    }) as size_t;
    (*s).exp = AVTXNum {
        double: av_malloc_array(alloc, ::core::mem::size_of::<TXComplex>() as libc::c_ulong)
            as *mut TXComplex,
    };
    if ((*s).exp).double.is_null() {
        return -(12 as libc::c_int);
    }
    scale = sqrt(fabs(scale));
    if !pre_tab.is_null() {
        off = len4;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < len4 {
        let alpha: libc::c_double =
            1.57079632679489661923f64 * (i as libc::c_double + theta) / len4 as libc::c_double;
        *((*s).exp).double.offset((off + i) as isize) = {
            let mut init = AVComplexDouble {
                re: cos(alpha) * scale,
                im: sin(alpha) * scale,
            };
            init
        };
        i += 1;
        i;
    }
    if !pre_tab.is_null() {
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < len4 {
            *((*s).exp).double.offset(i_0 as isize) = *((*s).exp)
                .double
                .offset((len4 + *pre_tab.offset(i_0 as isize)) as isize);
            i_0 += 1;
            i_0;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub static mut ff_tx_codelet_list_double_c: [*const FFTXCodelet; 63] = unsafe {
    [
        &ff_tx_fft2_ns_def_double_c as *const FFTXCodelet,
        &ff_tx_fft4_ns_def_double_c as *const FFTXCodelet,
        &ff_tx_fft8_ns_def_double_c as *const FFTXCodelet,
        &ff_tx_fft16_ns_def_double_c as *const FFTXCodelet,
        &ff_tx_fft32_ns_def_double_c as *const FFTXCodelet,
        &ff_tx_fft64_ns_def_double_c as *const FFTXCodelet,
        &ff_tx_fft128_ns_def_double_c as *const FFTXCodelet,
        &ff_tx_fft256_ns_def_double_c as *const FFTXCodelet,
        &ff_tx_fft512_ns_def_double_c as *const FFTXCodelet,
        &ff_tx_fft1024_ns_def_double_c as *const FFTXCodelet,
        &ff_tx_fft2048_ns_def_double_c as *const FFTXCodelet,
        &ff_tx_fft4096_ns_def_double_c as *const FFTXCodelet,
        &ff_tx_fft8192_ns_def_double_c as *const FFTXCodelet,
        &ff_tx_fft16384_ns_def_double_c as *const FFTXCodelet,
        &ff_tx_fft32768_ns_def_double_c as *const FFTXCodelet,
        &ff_tx_fft65536_ns_def_double_c as *const FFTXCodelet,
        &ff_tx_fft131072_ns_def_double_c as *const FFTXCodelet,
        &ff_tx_fft262144_ns_def_double_c as *const FFTXCodelet,
        &ff_tx_fft524288_ns_def_double_c as *const FFTXCodelet,
        &ff_tx_fft1048576_ns_def_double_c as *const FFTXCodelet,
        &ff_tx_fft2097152_ns_def_double_c as *const FFTXCodelet,
        &ff_tx_fft3_ns_def_double_c as *const FFTXCodelet,
        &ff_tx_fft5_ns_def_double_c as *const FFTXCodelet,
        &ff_tx_fft7_ns_def_double_c as *const FFTXCodelet,
        &ff_tx_fft9_ns_def_double_c as *const FFTXCodelet,
        &ff_tx_fft15_ns_def_double_c as *const FFTXCodelet,
        &ff_tx_fft3_fwd_def_double_c as *const FFTXCodelet,
        &ff_tx_fft5_fwd_def_double_c as *const FFTXCodelet,
        &ff_tx_fft7_fwd_def_double_c as *const FFTXCodelet,
        &ff_tx_fft9_fwd_def_double_c as *const FFTXCodelet,
        &ff_tx_fft_def_double_c as *const FFTXCodelet,
        &ff_tx_fft_inplace_def_double_c as *const FFTXCodelet,
        &ff_tx_fft_inplace_small_def_double_c as *const FFTXCodelet,
        &ff_tx_fft_pfa_def_double_c as *const FFTXCodelet,
        &ff_tx_fft_pfa_ns_def_double_c as *const FFTXCodelet,
        &ff_tx_fft_naive_def_double_c as *const FFTXCodelet,
        &ff_tx_fft_naive_small_def_double_c as *const FFTXCodelet,
        &ff_tx_mdct_fwd_def_double_c as *const FFTXCodelet,
        &ff_tx_mdct_inv_def_double_c as *const FFTXCodelet,
        &ff_tx_mdct_pfa_3xM_fwd_def_double_c as *const FFTXCodelet,
        &ff_tx_mdct_pfa_5xM_fwd_def_double_c as *const FFTXCodelet,
        &ff_tx_mdct_pfa_7xM_fwd_def_double_c as *const FFTXCodelet,
        &ff_tx_mdct_pfa_9xM_fwd_def_double_c as *const FFTXCodelet,
        &ff_tx_mdct_pfa_15xM_fwd_def_double_c as *const FFTXCodelet,
        &ff_tx_mdct_pfa_3xM_inv_def_double_c as *const FFTXCodelet,
        &ff_tx_mdct_pfa_5xM_inv_def_double_c as *const FFTXCodelet,
        &ff_tx_mdct_pfa_7xM_inv_def_double_c as *const FFTXCodelet,
        &ff_tx_mdct_pfa_9xM_inv_def_double_c as *const FFTXCodelet,
        &ff_tx_mdct_pfa_15xM_inv_def_double_c as *const FFTXCodelet,
        &ff_tx_mdct_naive_fwd_def_double_c as *const FFTXCodelet,
        &ff_tx_mdct_naive_inv_def_double_c as *const FFTXCodelet,
        &ff_tx_mdct_inv_full_def_double_c as *const FFTXCodelet,
        &ff_tx_rdft_r2c_def_double_c as *const FFTXCodelet,
        &ff_tx_rdft_r2r_def_double_c as *const FFTXCodelet,
        &ff_tx_rdft_r2r_mod2_def_double_c as *const FFTXCodelet,
        &ff_tx_rdft_r2i_def_double_c as *const FFTXCodelet,
        &ff_tx_rdft_r2i_mod2_def_double_c as *const FFTXCodelet,
        &ff_tx_rdft_c2r_def_double_c as *const FFTXCodelet,
        &ff_tx_dctII_def_double_c as *const FFTXCodelet,
        &ff_tx_dctIII_def_double_c as *const FFTXCodelet,
        &ff_tx_dctI_def_double_c as *const FFTXCodelet,
        &ff_tx_dstI_def_double_c as *const FFTXCodelet,
        0 as *const FFTXCodelet,
    ]
};
unsafe extern "C" fn run_static_initializers() {
    ff_tx_rdft_r2c_def_double_c = {
        let mut init = FFTXCodelet {
            name: b"rdft_r2c_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_rdft_r2c_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_RDFT,
            flags: ((AV_TX_UNALIGNED as libc::c_int | AV_TX_INPLACE as libc::c_int)
                as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | (if 0 as libc::c_int != 0 {
                    (1 as libc::c_ulonglong) << 60 as libc::c_int
                } else {
                    (1 as libc::c_ulonglong) << 59 as libc::c_int
                })) as uint64_t,
            factors: [
                4 as libc::c_int,
                -(1 as libc::c_int),
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 2 as libc::c_int,
            min_len: 4 as libc::c_int,
            max_len: -(1 as libc::c_int),
            init: Some(
                ff_tx_rdft_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    };
    ff_tx_rdft_c2r_def_double_c = {
        let mut init = FFTXCodelet {
            name: b"rdft_c2r_double_c\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_rdft_c2r_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_DOUBLE_RDFT,
            flags: ((AV_TX_UNALIGNED as libc::c_int | AV_TX_INPLACE as libc::c_int)
                as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int
                | (if 1 as libc::c_int != 0 {
                    (1 as libc::c_ulonglong) << 60 as libc::c_int
                } else {
                    (1 as libc::c_ulonglong) << 59 as libc::c_int
                })) as uint64_t,
            factors: [
                4 as libc::c_int,
                -(1 as libc::c_int),
                0,
                0,
                0,
                0,
                0,
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
            nb_factors: 2 as libc::c_int,
            min_len: 4 as libc::c_int,
            max_len: -(1 as libc::c_int),
            init: Some(
                ff_tx_rdft_init_double_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *const FFTXCodelet,
                        uint64_t,
                        *mut FFTXCodeletOptions,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_BASE as libc::c_int,
        };
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
