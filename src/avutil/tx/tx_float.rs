use std::{
    f64::consts::{FRAC_PI_2, PI},
    mem::size_of,
    ptr::{self, addr_of},
};

use ffi::num::AVComplexFloat;
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_uchar, c_uint, c_ulong, c_ulonglong, c_void,
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
    fn av_malloc_array(nmemb: c_ulong, size: c_ulong) -> *mut c_void;
    fn av_mallocz(size: c_ulong) -> *mut c_void;
    fn av_malloc(size: c_ulong) -> *mut c_void;
    fn pthread_once(
        __once_control: *mut pthread_once_t,
        __init_routine: Option<unsafe extern "C" fn() -> ()>,
    ) -> c_int;
    fn memcpy(_: *mut c_void, _: *const c_void, _: c_ulong) -> *mut c_void;
}
pub type TXComplex = AVComplexFloat;
pub type AVTXFlags = c_uint;
pub const AV_TX_REAL_TO_IMAGINARY: AVTXFlags = 16;
pub const AV_TX_REAL_TO_REAL: AVTXFlags = 8;
pub const AV_TX_FULL_IMDCT: AVTXFlags = 4;
pub const AV_TX_UNALIGNED: AVTXFlags = 2;
pub const AV_TX_INPLACE: AVTXFlags = 1;
pub type pthread_once_t = c_int;
pub type TXSample = c_float;
pub type TXUSample = c_float;
pub type FFTXCodeletPriority = c_int;
pub const FF_TX_PRIO_MIN: FFTXCodeletPriority = -131072;
pub const FF_TX_PRIO_BASE: FFTXCodeletPriority = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFTabInitData {
    pub func: Option<unsafe extern "C" fn() -> ()>,
    pub factors: [c_int; 4],
}
#[inline(always)]
unsafe extern "C" fn ff_ctz_c(v: c_int) -> c_int {
    static mut debruijn_ctz32: [c_uchar; 32] = [
        0, 1, 28, 2, 29, 14, 24, 3, 30, 22, 20, 15, 25, 17, 4, 8, 31, 27, 13, 23, 21, 19, 16, 7,
        26, 12, 18, 6, 11, 5, 10, 9,
    ];
    debruijn_ctz32[(((v & -v) as c_uint).wrapping_mul(0x77cb531 as c_uint) >> 27) as usize] as c_int
}

pub static mut ff_tx_tab_2097152_float: [TXSample; 524289] = [0.; 524289];

pub static mut ff_tx_tab_1048576_float: [TXSample; 262145] = [0.; 262145];

pub static mut ff_tx_tab_65536_float: [TXSample; 16385] = [0.; 16385];

pub static mut ff_tx_tab_32768_float: [TXSample; 8193] = [0.; 8193];

pub static mut ff_tx_tab_16384_float: [TXSample; 4097] = [0.; 4097];

pub static mut ff_tx_tab_32_float: [TXSample; 9] = [0.; 9];

pub static mut ff_tx_tab_8192_float: [TXSample; 2049] = [0.; 2049];

pub static mut ff_tx_tab_131072_float: [TXSample; 32769] = [0.; 32769];

pub static mut ff_tx_tab_4096_float: [TXSample; 1025] = [0.; 1025];

pub static mut ff_tx_tab_262144_float: [TXSample; 65537] = [0.; 65537];

pub static mut ff_tx_tab_2048_float: [TXSample; 513] = [0.; 513];

pub static mut ff_tx_tab_1024_float: [TXSample; 257] = [0.; 257];

pub static mut ff_tx_tab_524288_float: [TXSample; 131073] = [0.; 131073];

pub static mut ff_tx_tab_512_float: [TXSample; 129] = [0.; 129];

pub static mut ff_tx_tab_8_float: [TXSample; 3] = [0.; 3];

pub static mut ff_tx_tab_256_float: [TXSample; 65] = [0.; 65];

pub static mut ff_tx_tab_16_float: [TXSample; 5] = [0.; 5];

pub static mut ff_tx_tab_128_float: [TXSample; 33] = [0.; 33];

pub static mut ff_tx_tab_64_float: [TXSample; 17] = [0.; 17];

pub static mut ff_tx_tab_53_float: [TXSample; 12] = [0.; 12];

pub static mut ff_tx_tab_7_float: [TXSample; 6] = [0.; 6];

pub static mut ff_tx_tab_9_float: [TXSample; 8] = [0.; 8];
#[cold]
unsafe extern "C" fn ff_tx_init_tab_524288_float() {
    let freq: c_double = 2. * PI / 524288.;
    let mut tab: *mut TXSample = ff_tx_tab_524288_float.as_mut_ptr();
    let mut i: c_int = 0;
    while i < 524288 / 4 {
        let fresh0 = tab;
        tab = tab.offset(1);
        *fresh0 = cos(i as c_double * freq) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_2048_float() {
    let freq: c_double = 2. * PI / 2048.;
    let mut tab: *mut TXSample = ff_tx_tab_2048_float.as_mut_ptr();
    let mut i: c_int = 0;
    while i < 2048 / 4 {
        let fresh1 = tab;
        tab = tab.offset(1);
        *fresh1 = cos(i as c_double * freq) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_8_float() {
    let freq: c_double = 2. * PI / 8.;
    let mut tab: *mut TXSample = ff_tx_tab_8_float.as_mut_ptr();
    let mut i: c_int = 0;
    while i < 8 / 4 {
        let fresh2 = tab;
        tab = tab.offset(1);
        *fresh2 = cos(i as c_double * freq) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_128_float() {
    let freq: c_double = 2. * PI / 128.;
    let mut tab: *mut TXSample = ff_tx_tab_128_float.as_mut_ptr();
    let mut i: c_int = 0;
    while i < 128 / 4 {
        let fresh3 = tab;
        tab = tab.offset(1);
        *fresh3 = cos(i as c_double * freq) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_32_float() {
    let freq: c_double = 2. * PI / 32.;
    let mut tab: *mut TXSample = ff_tx_tab_32_float.as_mut_ptr();
    let mut i: c_int = 0;
    while i < 32 / 4 {
        let fresh4 = tab;
        tab = tab.offset(1);
        *fresh4 = cos(i as c_double * freq) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_256_float() {
    let freq: c_double = 2. * PI / 256.;
    let mut tab: *mut TXSample = ff_tx_tab_256_float.as_mut_ptr();
    let mut i: c_int = 0;
    while i < 256 / 4 {
        let fresh5 = tab;
        tab = tab.offset(1);
        *fresh5 = cos(i as c_double * freq) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_512_float() {
    let freq: c_double = 2. * PI / 512.;
    let mut tab: *mut TXSample = ff_tx_tab_512_float.as_mut_ptr();
    let mut i: c_int = 0;
    while i < 512 / 4 {
        let fresh6 = tab;
        tab = tab.offset(1);
        *fresh6 = cos(i as c_double * freq) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_32768_float() {
    let freq: c_double = 2. * PI / 32768.;
    let mut tab: *mut TXSample = ff_tx_tab_32768_float.as_mut_ptr();
    let mut i: c_int = 0;
    while i < 32768 / 4 {
        let fresh7 = tab;
        tab = tab.offset(1);
        *fresh7 = cos(i as c_double * freq) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_16384_float() {
    let freq: c_double = 2. * PI / 16384.;
    let mut tab: *mut TXSample = ff_tx_tab_16384_float.as_mut_ptr();
    let mut i: c_int = 0;
    while i < 16384 / 4 {
        let fresh8 = tab;
        tab = tab.offset(1);
        *fresh8 = cos(i as c_double * freq) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_4096_float() {
    let freq: c_double = 2. * PI / 4096.;
    let mut tab: *mut TXSample = ff_tx_tab_4096_float.as_mut_ptr();
    let mut i: c_int = 0;
    while i < 4096 / 4 {
        let fresh9 = tab;
        tab = tab.offset(1);
        *fresh9 = cos(i as c_double * freq) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_16_float() {
    let freq: c_double = 2. * PI / 16.;
    let mut tab: *mut TXSample = ff_tx_tab_16_float.as_mut_ptr();
    let mut i: c_int = 0;
    while i < 16 / 4 {
        let fresh10 = tab;
        tab = tab.offset(1);
        *fresh10 = cos(i as c_double * freq) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_8192_float() {
    let freq: c_double = 2. * PI / 8192.;
    let mut tab: *mut TXSample = ff_tx_tab_8192_float.as_mut_ptr();
    let mut i: c_int = 0;
    while i < 8192 / 4 {
        let fresh11 = tab;
        tab = tab.offset(1);
        *fresh11 = cos(i as c_double * freq) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_1024_float() {
    let freq: c_double = 2. * PI / 1024.;
    let mut tab: *mut TXSample = ff_tx_tab_1024_float.as_mut_ptr();
    let mut i: c_int = 0;
    while i < 1024 / 4 {
        let fresh12 = tab;
        tab = tab.offset(1);
        *fresh12 = cos(i as c_double * freq) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_65536_float() {
    let freq: c_double = 2. * PI / 65536.;
    let mut tab: *mut TXSample = ff_tx_tab_65536_float.as_mut_ptr();
    let mut i: c_int = 0;
    while i < 65536 / 4 {
        let fresh13 = tab;
        tab = tab.offset(1);
        *fresh13 = cos(i as c_double * freq) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_131072_float() {
    let freq: c_double = 2. * PI / 131072.;
    let mut tab: *mut TXSample = ff_tx_tab_131072_float.as_mut_ptr();
    let mut i: c_int = 0;
    while i < 131072 / 4 {
        let fresh14 = tab;
        tab = tab.offset(1);
        *fresh14 = cos(i as c_double * freq) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_262144_float() {
    let freq: c_double = 2. * PI / 262144.;
    let mut tab: *mut TXSample = ff_tx_tab_262144_float.as_mut_ptr();
    let mut i: c_int = 0;
    while i < 262144 / 4 {
        let fresh15 = tab;
        tab = tab.offset(1);
        *fresh15 = cos(i as c_double * freq) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_64_float() {
    let freq: c_double = 2. * PI / 64.;
    let mut tab: *mut TXSample = ff_tx_tab_64_float.as_mut_ptr();
    let mut i: c_int = 0;
    while i < 64 / 4 {
        let fresh16 = tab;
        tab = tab.offset(1);
        *fresh16 = cos(i as c_double * freq) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_1048576_float() {
    let freq: c_double = 2. * PI / 1048576.;
    let mut tab: *mut TXSample = ff_tx_tab_1048576_float.as_mut_ptr();
    let mut i: c_int = 0;
    while i < 1048576 / 4 {
        let fresh17 = tab;
        tab = tab.offset(1);
        *fresh17 = cos(i as c_double * freq) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_2097152_float() {
    let freq: c_double = 2. * PI / 2097152.;
    let mut tab: *mut TXSample = ff_tx_tab_2097152_float.as_mut_ptr();
    let mut i: c_int = 0;
    while i < 2097152 / 4 {
        let fresh18 = tab;
        tab = tab.offset(1);
        *fresh18 = cos(i as c_double * freq) as TXSample;
        i += 1;
        i;
    }
    *tab = 0 as TXSample;
}
static mut sr_tabs_init_funcs: [Option<unsafe extern "C" fn() -> ()>; 19] = [
    Some(ff_tx_init_tab_8_float as unsafe extern "C" fn() -> ()),
    Some(ff_tx_init_tab_16_float as unsafe extern "C" fn() -> ()),
    Some(ff_tx_init_tab_32_float as unsafe extern "C" fn() -> ()),
    Some(ff_tx_init_tab_64_float as unsafe extern "C" fn() -> ()),
    Some(ff_tx_init_tab_128_float as unsafe extern "C" fn() -> ()),
    Some(ff_tx_init_tab_256_float as unsafe extern "C" fn() -> ()),
    Some(ff_tx_init_tab_512_float as unsafe extern "C" fn() -> ()),
    Some(ff_tx_init_tab_1024_float as unsafe extern "C" fn() -> ()),
    Some(ff_tx_init_tab_2048_float as unsafe extern "C" fn() -> ()),
    Some(ff_tx_init_tab_4096_float as unsafe extern "C" fn() -> ()),
    Some(ff_tx_init_tab_8192_float as unsafe extern "C" fn() -> ()),
    Some(ff_tx_init_tab_16384_float as unsafe extern "C" fn() -> ()),
    Some(ff_tx_init_tab_32768_float as unsafe extern "C" fn() -> ()),
    Some(ff_tx_init_tab_65536_float as unsafe extern "C" fn() -> ()),
    Some(ff_tx_init_tab_131072_float as unsafe extern "C" fn() -> ()),
    Some(ff_tx_init_tab_262144_float as unsafe extern "C" fn() -> ()),
    Some(ff_tx_init_tab_524288_float as unsafe extern "C" fn() -> ()),
    Some(ff_tx_init_tab_1048576_float as unsafe extern "C" fn() -> ()),
    Some(ff_tx_init_tab_2097152_float as unsafe extern "C" fn() -> ()),
];

static mut sr_tabs_init_once: [pthread_once_t; 19] = [0; 19];

#[cold]
unsafe extern "C" fn ff_tx_init_tab_53_float() {
    ff_tx_tab_53_float[0] = cos(2. * PI / 5.) as TXSample;
    ff_tx_tab_53_float[1] = cos(2. * PI / 5.) as TXSample;
    ff_tx_tab_53_float[2] = cos(2. * PI / 10.) as TXSample;
    ff_tx_tab_53_float[3] = cos(2. * PI / 10.) as TXSample;
    ff_tx_tab_53_float[4] = sin(2. * PI / 5.) as TXSample;
    ff_tx_tab_53_float[5] = sin(2. * PI / 5.) as TXSample;
    ff_tx_tab_53_float[6] = sin(2. * PI / 10.) as TXSample;
    ff_tx_tab_53_float[7] = sin(2. * PI / 10.) as TXSample;
    ff_tx_tab_53_float[8] = cos(2. * PI / 12.) as TXSample;
    ff_tx_tab_53_float[9] = cos(2. * PI / 12.) as TXSample;
    ff_tx_tab_53_float[10] = cos(2. * PI / 6.) as TXSample;
    ff_tx_tab_53_float[11] = cos(8. * PI / 6.) as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_7_float() {
    ff_tx_tab_7_float[0] = cos(2. * PI / 7.) as TXSample;
    ff_tx_tab_7_float[1] = sin(2. * PI / 7.) as TXSample;
    ff_tx_tab_7_float[2] = sin(2. * PI / 28.) as TXSample;
    ff_tx_tab_7_float[3] = cos(2. * PI / 28.) as TXSample;
    ff_tx_tab_7_float[4] = cos(2. * PI / 14.) as TXSample;
    ff_tx_tab_7_float[5] = sin(2. * PI / 14.) as TXSample;
}
#[cold]
unsafe extern "C" fn ff_tx_init_tab_9_float() {
    ff_tx_tab_9_float[0] = cos(2. * PI / 3.) as TXSample;
    ff_tx_tab_9_float[1] = sin(2. * PI / 3.) as TXSample;
    ff_tx_tab_9_float[2] = cos(2. * PI / 9.) as TXSample;
    ff_tx_tab_9_float[3] = sin(2. * PI / 9.) as TXSample;
    ff_tx_tab_9_float[4] = cos(2. * PI / 36.) as TXSample;
    ff_tx_tab_9_float[5] = sin(2. * PI / 36.) as TXSample;
    ff_tx_tab_9_float[6] = ff_tx_tab_9_float[2] + ff_tx_tab_9_float[5];
    ff_tx_tab_9_float[7] = ff_tx_tab_9_float[3] - ff_tx_tab_9_float[4];
}
static mut nptwo_tabs_init_data: [FFTabInitData; 3] = [
    {
        FFTabInitData {
            func: Some(ff_tx_init_tab_53_float as unsafe extern "C" fn() -> ()),
            factors: [15, 5, 3, 0],
        }
    },
    {
        FFTabInitData {
            func: Some(ff_tx_init_tab_9_float as unsafe extern "C" fn() -> ()),
            factors: [9, 0, 0, 0],
        }
    },
    {
        FFTabInitData {
            func: Some(ff_tx_init_tab_7_float as unsafe extern "C" fn() -> ()),
            factors: [7, 0, 0, 0],
        }
    },
];

static mut nptwo_tabs_init_once: [pthread_once_t; 3] = [0; 3];

#[cold]
pub unsafe extern "C" fn ff_tx_init_tabs_float(mut len: c_int) {
    let factor_2: c_int = ff_ctz_c(len);
    if factor_2 != 0 {
        let idx: c_int = factor_2 - 3;
        let mut i: c_int = 0;
        while i <= idx {
            pthread_once(
                &mut *sr_tabs_init_once.as_mut_ptr().offset(i as isize),
                sr_tabs_init_funcs[i as usize],
            );
            i += 1;
            i;
        }
        len >>= factor_2;
    }
    let mut i_0: c_int = 0;
    while (i_0 as c_ulong)
        < (size_of::<[FFTabInitData; 3]>() as c_ulong)
            .wrapping_div(size_of::<FFTabInitData>() as c_ulong)
    {
        let mut f: c_int = 0;
        let mut f_idx: c_int = 0;
        if len <= 1 {
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
            pthread_once(
                &mut *nptwo_tabs_init_once.as_mut_ptr().offset(i_0 as isize),
                nptwo_tabs_init_data[i_0 as usize].func,
            );
            len /= f;
            break;
        }
        i_0 += 1;
        i_0;
    }
}
#[inline(always)]
unsafe extern "C" fn fft3(out: *mut TXComplex, in_0: *mut TXComplex, stride: ptrdiff_t) {
    let mut tmp: [TXComplex; 3] = [TXComplex { re: 0., im: 0. }; 3];
    let tab: *const TXSample = ff_tx_tab_53_float.as_mut_ptr();
    tmp[0] = *in_0.offset(0);
    tmp[1].re = (*in_0.offset(1)).im - (*in_0.offset(2)).im;
    tmp[2].im = (*in_0.offset(1)).im + (*in_0.offset(2)).im;
    tmp[1].im = (*in_0.offset(1)).re - (*in_0.offset(2)).re;
    tmp[2].re = (*in_0.offset(1)).re + (*in_0.offset(2)).re;
    (*out.offset((0 as c_long * stride) as isize)).re = tmp[0].re + tmp[2].re;
    (*out.offset((0 as c_long * stride) as isize)).im = tmp[0].im + tmp[2].im;
    tmp[1].re *= *tab.offset(8);
    tmp[1].im *= *tab.offset(9);
    tmp[2].re *= *tab.offset(10);
    tmp[2].im *= *tab.offset(10);
    (*out.offset((1 as c_long * stride) as isize)).re = tmp[0].re - tmp[2].re + tmp[1].re;
    (*out.offset((1 as c_long * stride) as isize)).im = tmp[0].im - tmp[2].im - tmp[1].im;
    (*out.offset((2 as c_long * stride) as isize)).re = tmp[0].re - tmp[2].re - tmp[1].re;
    (*out.offset((2 as c_long * stride) as isize)).im = tmp[0].im - tmp[2].im + tmp[1].im;
}
#[inline(always)]
unsafe extern "C" fn fft5(out: *mut TXComplex, in_0: *mut TXComplex, stride: ptrdiff_t) {
    let mut dc: TXComplex = TXComplex { re: 0., im: 0. };
    let mut z0: [TXComplex; 4] = [TXComplex { re: 0., im: 0. }; 4];
    let mut t: [TXComplex; 6] = [TXComplex { re: 0., im: 0. }; 6];
    let tab: *const TXSample = ff_tx_tab_53_float.as_mut_ptr();
    dc = *in_0.offset(0);
    t[1].im = (*in_0.offset(1)).re - (*in_0.offset(4)).re;
    t[0].re = (*in_0.offset(1)).re + (*in_0.offset(4)).re;
    t[1].re = (*in_0.offset(1)).im - (*in_0.offset(4)).im;
    t[0].im = (*in_0.offset(1)).im + (*in_0.offset(4)).im;
    t[3].im = (*in_0.offset(2)).re - (*in_0.offset(3)).re;
    t[2].re = (*in_0.offset(2)).re + (*in_0.offset(3)).re;
    t[3].re = (*in_0.offset(2)).im - (*in_0.offset(3)).im;
    t[2].im = (*in_0.offset(2)).im + (*in_0.offset(3)).im;
    (*out.offset((0 as c_long * stride) as isize)).re = dc.re + t[0].re + t[2].re;
    (*out.offset((0 as c_long * stride) as isize)).im = dc.im + t[0].im + t[2].im;
    t[4].re = *tab.offset(0) * t[2].re - *tab.offset(2) * t[0].re;
    t[0].re = *tab.offset(0) * t[0].re - *tab.offset(2) * t[2].re;
    t[4].im = *tab.offset(0) * t[2].im - *tab.offset(2) * t[0].im;
    t[0].im = *tab.offset(0) * t[0].im - *tab.offset(2) * t[2].im;
    t[5].re = *tab.offset(4) * t[3].re - *tab.offset(6) * t[1].re;
    t[1].re = *tab.offset(4) * t[1].re + *tab.offset(6) * t[3].re;
    t[5].im = *tab.offset(4) * t[3].im - *tab.offset(6) * t[1].im;
    t[1].im = *tab.offset(4) * t[1].im + *tab.offset(6) * t[3].im;
    z0[0].re = t[0].re - t[1].re;
    z0[3].re = t[0].re + t[1].re;
    z0[0].im = t[0].im - t[1].im;
    z0[3].im = t[0].im + t[1].im;
    z0[2].re = t[4].re - t[5].re;
    z0[1].re = t[4].re + t[5].re;
    z0[2].im = t[4].im - t[5].im;
    z0[1].im = t[4].im + t[5].im;
    (*out.offset((1 as c_long * stride) as isize)).re = dc.re + z0[3].re;
    (*out.offset((1 as c_long * stride) as isize)).im = dc.im + z0[0].im;
    (*out.offset((2 as c_long * stride) as isize)).re = dc.re + z0[2].re;
    (*out.offset((2 as c_long * stride) as isize)).im = dc.im + z0[1].im;
    (*out.offset((3 as c_long * stride) as isize)).re = dc.re + z0[1].re;
    (*out.offset((3 as c_long * stride) as isize)).im = dc.im + z0[2].im;
    (*out.offset((4 as c_long * stride) as isize)).re = dc.re + z0[0].re;
    (*out.offset((4 as c_long * stride) as isize)).im = dc.im + z0[3].im;
}
#[inline(always)]
unsafe extern "C" fn fft5_m1(out: *mut TXComplex, in_0: *mut TXComplex, stride: ptrdiff_t) {
    let mut dc: TXComplex = TXComplex { re: 0., im: 0. };
    let mut z0: [TXComplex; 4] = [TXComplex { re: 0., im: 0. }; 4];
    let mut t: [TXComplex; 6] = [TXComplex { re: 0., im: 0. }; 6];
    let tab: *const TXSample = ff_tx_tab_53_float.as_mut_ptr();
    dc = *in_0.offset(0);
    t[1].im = (*in_0.offset(1)).re - (*in_0.offset(4)).re;
    t[0].re = (*in_0.offset(1)).re + (*in_0.offset(4)).re;
    t[1].re = (*in_0.offset(1)).im - (*in_0.offset(4)).im;
    t[0].im = (*in_0.offset(1)).im + (*in_0.offset(4)).im;
    t[3].im = (*in_0.offset(2)).re - (*in_0.offset(3)).re;
    t[2].re = (*in_0.offset(2)).re + (*in_0.offset(3)).re;
    t[3].re = (*in_0.offset(2)).im - (*in_0.offset(3)).im;
    t[2].im = (*in_0.offset(2)).im + (*in_0.offset(3)).im;
    (*out.offset((0 as c_long * stride) as isize)).re = dc.re + t[0].re + t[2].re;
    (*out.offset((0 as c_long * stride) as isize)).im = dc.im + t[0].im + t[2].im;
    t[4].re = *tab.offset(0) * t[2].re - *tab.offset(2) * t[0].re;
    t[0].re = *tab.offset(0) * t[0].re - *tab.offset(2) * t[2].re;
    t[4].im = *tab.offset(0) * t[2].im - *tab.offset(2) * t[0].im;
    t[0].im = *tab.offset(0) * t[0].im - *tab.offset(2) * t[2].im;
    t[5].re = *tab.offset(4) * t[3].re - *tab.offset(6) * t[1].re;
    t[1].re = *tab.offset(4) * t[1].re + *tab.offset(6) * t[3].re;
    t[5].im = *tab.offset(4) * t[3].im - *tab.offset(6) * t[1].im;
    t[1].im = *tab.offset(4) * t[1].im + *tab.offset(6) * t[3].im;
    z0[0].re = t[0].re - t[1].re;
    z0[3].re = t[0].re + t[1].re;
    z0[0].im = t[0].im - t[1].im;
    z0[3].im = t[0].im + t[1].im;
    z0[2].re = t[4].re - t[5].re;
    z0[1].re = t[4].re + t[5].re;
    z0[2].im = t[4].im - t[5].im;
    z0[1].im = t[4].im + t[5].im;
    (*out.offset((6 as c_long * stride) as isize)).re = dc.re + z0[3].re;
    (*out.offset((6 as c_long * stride) as isize)).im = dc.im + z0[0].im;
    (*out.offset((12 as c_long * stride) as isize)).re = dc.re + z0[2].re;
    (*out.offset((12 as c_long * stride) as isize)).im = dc.im + z0[1].im;
    (*out.offset((3 as c_long * stride) as isize)).re = dc.re + z0[1].re;
    (*out.offset((3 as c_long * stride) as isize)).im = dc.im + z0[2].im;
    (*out.offset((9 as c_long * stride) as isize)).re = dc.re + z0[0].re;
    (*out.offset((9 as c_long * stride) as isize)).im = dc.im + z0[3].im;
}
#[inline(always)]
unsafe extern "C" fn fft5_m2(out: *mut TXComplex, in_0: *mut TXComplex, stride: ptrdiff_t) {
    let mut dc: TXComplex = TXComplex { re: 0., im: 0. };
    let mut z0: [TXComplex; 4] = [TXComplex { re: 0., im: 0. }; 4];
    let mut t: [TXComplex; 6] = [TXComplex { re: 0., im: 0. }; 6];
    let tab: *const TXSample = ff_tx_tab_53_float.as_mut_ptr();
    dc = *in_0.offset(0);
    t[1].im = (*in_0.offset(1)).re - (*in_0.offset(4)).re;
    t[0].re = (*in_0.offset(1)).re + (*in_0.offset(4)).re;
    t[1].re = (*in_0.offset(1)).im - (*in_0.offset(4)).im;
    t[0].im = (*in_0.offset(1)).im + (*in_0.offset(4)).im;
    t[3].im = (*in_0.offset(2)).re - (*in_0.offset(3)).re;
    t[2].re = (*in_0.offset(2)).re + (*in_0.offset(3)).re;
    t[3].re = (*in_0.offset(2)).im - (*in_0.offset(3)).im;
    t[2].im = (*in_0.offset(2)).im + (*in_0.offset(3)).im;
    (*out.offset((10 as c_long * stride) as isize)).re = dc.re + t[0].re + t[2].re;
    (*out.offset((10 as c_long * stride) as isize)).im = dc.im + t[0].im + t[2].im;
    t[4].re = *tab.offset(0) * t[2].re - *tab.offset(2) * t[0].re;
    t[0].re = *tab.offset(0) * t[0].re - *tab.offset(2) * t[2].re;
    t[4].im = *tab.offset(0) * t[2].im - *tab.offset(2) * t[0].im;
    t[0].im = *tab.offset(0) * t[0].im - *tab.offset(2) * t[2].im;
    t[5].re = *tab.offset(4) * t[3].re - *tab.offset(6) * t[1].re;
    t[1].re = *tab.offset(4) * t[1].re + *tab.offset(6) * t[3].re;
    t[5].im = *tab.offset(4) * t[3].im - *tab.offset(6) * t[1].im;
    t[1].im = *tab.offset(4) * t[1].im + *tab.offset(6) * t[3].im;
    z0[0].re = t[0].re - t[1].re;
    z0[3].re = t[0].re + t[1].re;
    z0[0].im = t[0].im - t[1].im;
    z0[3].im = t[0].im + t[1].im;
    z0[2].re = t[4].re - t[5].re;
    z0[1].re = t[4].re + t[5].re;
    z0[2].im = t[4].im - t[5].im;
    z0[1].im = t[4].im + t[5].im;
    (*out.offset((1 as c_long * stride) as isize)).re = dc.re + z0[3].re;
    (*out.offset((1 as c_long * stride) as isize)).im = dc.im + z0[0].im;
    (*out.offset((7 as c_long * stride) as isize)).re = dc.re + z0[2].re;
    (*out.offset((7 as c_long * stride) as isize)).im = dc.im + z0[1].im;
    (*out.offset((13 as c_long * stride) as isize)).re = dc.re + z0[1].re;
    (*out.offset((13 as c_long * stride) as isize)).im = dc.im + z0[2].im;
    (*out.offset((4 as c_long * stride) as isize)).re = dc.re + z0[0].re;
    (*out.offset((4 as c_long * stride) as isize)).im = dc.im + z0[3].im;
}
#[inline(always)]
unsafe extern "C" fn fft5_m3(out: *mut TXComplex, in_0: *mut TXComplex, stride: ptrdiff_t) {
    let mut dc: TXComplex = TXComplex { re: 0., im: 0. };
    let mut z0: [TXComplex; 4] = [TXComplex { re: 0., im: 0. }; 4];
    let mut t: [TXComplex; 6] = [TXComplex { re: 0., im: 0. }; 6];
    let tab: *const TXSample = ff_tx_tab_53_float.as_mut_ptr();
    dc = *in_0.offset(0);
    t[1].im = (*in_0.offset(1)).re - (*in_0.offset(4)).re;
    t[0].re = (*in_0.offset(1)).re + (*in_0.offset(4)).re;
    t[1].re = (*in_0.offset(1)).im - (*in_0.offset(4)).im;
    t[0].im = (*in_0.offset(1)).im + (*in_0.offset(4)).im;
    t[3].im = (*in_0.offset(2)).re - (*in_0.offset(3)).re;
    t[2].re = (*in_0.offset(2)).re + (*in_0.offset(3)).re;
    t[3].re = (*in_0.offset(2)).im - (*in_0.offset(3)).im;
    t[2].im = (*in_0.offset(2)).im + (*in_0.offset(3)).im;
    (*out.offset((5 as c_long * stride) as isize)).re = dc.re + t[0].re + t[2].re;
    (*out.offset((5 as c_long * stride) as isize)).im = dc.im + t[0].im + t[2].im;
    t[4].re = *tab.offset(0) * t[2].re - *tab.offset(2) * t[0].re;
    t[0].re = *tab.offset(0) * t[0].re - *tab.offset(2) * t[2].re;
    t[4].im = *tab.offset(0) * t[2].im - *tab.offset(2) * t[0].im;
    t[0].im = *tab.offset(0) * t[0].im - *tab.offset(2) * t[2].im;
    t[5].re = *tab.offset(4) * t[3].re - *tab.offset(6) * t[1].re;
    t[1].re = *tab.offset(4) * t[1].re + *tab.offset(6) * t[3].re;
    t[5].im = *tab.offset(4) * t[3].im - *tab.offset(6) * t[1].im;
    t[1].im = *tab.offset(4) * t[1].im + *tab.offset(6) * t[3].im;
    z0[0].re = t[0].re - t[1].re;
    z0[3].re = t[0].re + t[1].re;
    z0[0].im = t[0].im - t[1].im;
    z0[3].im = t[0].im + t[1].im;
    z0[2].re = t[4].re - t[5].re;
    z0[1].re = t[4].re + t[5].re;
    z0[2].im = t[4].im - t[5].im;
    z0[1].im = t[4].im + t[5].im;
    (*out.offset((11 as c_long * stride) as isize)).re = dc.re + z0[3].re;
    (*out.offset((11 as c_long * stride) as isize)).im = dc.im + z0[0].im;
    (*out.offset((2 as c_long * stride) as isize)).re = dc.re + z0[2].re;
    (*out.offset((2 as c_long * stride) as isize)).im = dc.im + z0[1].im;
    (*out.offset((8 as c_long * stride) as isize)).re = dc.re + z0[1].re;
    (*out.offset((8 as c_long * stride) as isize)).im = dc.im + z0[2].im;
    (*out.offset((14 as c_long * stride) as isize)).re = dc.re + z0[0].re;
    (*out.offset((14 as c_long * stride) as isize)).im = dc.im + z0[3].im;
}
#[inline(always)]
unsafe extern "C" fn fft7(out: *mut TXComplex, in_0: *mut TXComplex, stride: ptrdiff_t) {
    let mut dc: TXComplex = TXComplex { re: 0., im: 0. };
    let mut t: [TXComplex; 6] = [TXComplex { re: 0., im: 0. }; 6];
    let mut z: [TXComplex; 3] = [TXComplex { re: 0., im: 0. }; 3];
    let tab: *const TXComplex = ff_tx_tab_7_float.as_mut_ptr() as *const TXComplex;
    dc = *in_0.offset(0);
    t[1].re = (*in_0.offset(1)).re - (*in_0.offset(6)).re;
    t[0].re = (*in_0.offset(1)).re + (*in_0.offset(6)).re;
    t[1].im = (*in_0.offset(1)).im - (*in_0.offset(6)).im;
    t[0].im = (*in_0.offset(1)).im + (*in_0.offset(6)).im;
    t[3].re = (*in_0.offset(2)).re - (*in_0.offset(5)).re;
    t[2].re = (*in_0.offset(2)).re + (*in_0.offset(5)).re;
    t[3].im = (*in_0.offset(2)).im - (*in_0.offset(5)).im;
    t[2].im = (*in_0.offset(2)).im + (*in_0.offset(5)).im;
    t[5].re = (*in_0.offset(3)).re - (*in_0.offset(4)).re;
    t[4].re = (*in_0.offset(3)).re + (*in_0.offset(4)).re;
    t[5].im = (*in_0.offset(3)).im - (*in_0.offset(4)).im;
    t[4].im = (*in_0.offset(3)).im + (*in_0.offset(4)).im;
    (*out.offset((0 as c_long * stride) as isize)).re = dc.re + t[0].re + t[2].re + t[4].re;
    (*out.offset((0 as c_long * stride) as isize)).im = dc.im + t[0].im + t[2].im + t[4].im;
    z[0].re = (*tab.offset(0)).re * t[0].re
        - (*tab.offset(2)).re * t[4].re
        - (*tab.offset(1)).re * t[2].re;
    z[1].re = (*tab.offset(0)).re * t[4].re
        - (*tab.offset(1)).re * t[0].re
        - (*tab.offset(2)).re * t[2].re;
    z[2].re = (*tab.offset(0)).re * t[2].re
        - (*tab.offset(2)).re * t[0].re
        - (*tab.offset(1)).re * t[4].re;
    z[0].im = (*tab.offset(0)).re * t[0].im
        - (*tab.offset(1)).re * t[2].im
        - (*tab.offset(2)).re * t[4].im;
    z[1].im = (*tab.offset(0)).re * t[4].im
        - (*tab.offset(1)).re * t[0].im
        - (*tab.offset(2)).re * t[2].im;
    z[2].im = (*tab.offset(0)).re * t[2].im
        - (*tab.offset(2)).re * t[0].im
        - (*tab.offset(1)).re * t[4].im;
    t[0].re = (*tab.offset(2)).im * t[1].im + (*tab.offset(1)).im * t[5].im
        - (*tab.offset(0)).im * t[3].im;
    t[2].re = (*tab.offset(0)).im * t[5].im + (*tab.offset(2)).im * t[3].im
        - (*tab.offset(1)).im * t[1].im;
    t[4].re = (*tab.offset(2)).im * t[5].im
        + (*tab.offset(1)).im * t[3].im
        + (*tab.offset(0)).im * t[1].im;
    t[0].im = (*tab.offset(0)).im * t[1].re
        + (*tab.offset(1)).im * t[3].re
        + (*tab.offset(2)).im * t[5].re;
    t[2].im = (*tab.offset(2)).im * t[3].re + (*tab.offset(0)).im * t[5].re
        - (*tab.offset(1)).im * t[1].re;
    t[4].im = (*tab.offset(2)).im * t[1].re + (*tab.offset(1)).im * t[5].re
        - (*tab.offset(0)).im * t[3].re;
    t[1].re = z[0].re - t[4].re;
    z[0].re += t[4].re;
    t[3].re = z[1].re - t[2].re;
    z[1].re += t[2].re;
    t[5].re = z[2].re - t[0].re;
    z[2].re += t[0].re;
    t[1].im = z[0].im - t[0].im;
    z[0].im += t[0].im;
    t[3].im = z[1].im - t[2].im;
    z[1].im += t[2].im;
    t[5].im = z[2].im - t[4].im;
    z[2].im += t[4].im;
    (*out.offset((1 as c_long * stride) as isize)).re = dc.re + z[0].re;
    (*out.offset((1 as c_long * stride) as isize)).im = dc.im + t[1].im;
    (*out.offset((2 as c_long * stride) as isize)).re = dc.re + t[3].re;
    (*out.offset((2 as c_long * stride) as isize)).im = dc.im + z[1].im;
    (*out.offset((3 as c_long * stride) as isize)).re = dc.re + z[2].re;
    (*out.offset((3 as c_long * stride) as isize)).im = dc.im + t[5].im;
    (*out.offset((4 as c_long * stride) as isize)).re = dc.re + t[5].re;
    (*out.offset((4 as c_long * stride) as isize)).im = dc.im + z[2].im;
    (*out.offset((5 as c_long * stride) as isize)).re = dc.re + z[1].re;
    (*out.offset((5 as c_long * stride) as isize)).im = dc.im + t[3].im;
    (*out.offset((6 as c_long * stride) as isize)).re = dc.re + t[1].re;
    (*out.offset((6 as c_long * stride) as isize)).im = dc.im + z[0].im;
}
#[inline(always)]
unsafe extern "C" fn fft9(out: *mut TXComplex, in_0: *mut TXComplex, stride: ptrdiff_t) {
    let tab: *const TXComplex = ff_tx_tab_9_float.as_mut_ptr() as *const TXComplex;
    let mut dc: TXComplex = TXComplex { re: 0., im: 0. };
    let mut t: [TXComplex; 16] = [TXComplex { re: 0., im: 0. }; 16];
    let mut w: [TXComplex; 4] = [TXComplex { re: 0., im: 0. }; 4];
    let mut x: [TXComplex; 5] = [TXComplex { re: 0., im: 0. }; 5];
    let mut y: [TXComplex; 5] = [TXComplex { re: 0., im: 0. }; 5];
    let mut z: [TXComplex; 2] = [TXComplex { re: 0., im: 0. }; 2];
    dc = *in_0.offset(0);
    t[1].re = (*in_0.offset(1)).re - (*in_0.offset(8)).re;
    t[0].re = (*in_0.offset(1)).re + (*in_0.offset(8)).re;
    t[1].im = (*in_0.offset(1)).im - (*in_0.offset(8)).im;
    t[0].im = (*in_0.offset(1)).im + (*in_0.offset(8)).im;
    t[3].re = (*in_0.offset(2)).re - (*in_0.offset(7)).re;
    t[2].re = (*in_0.offset(2)).re + (*in_0.offset(7)).re;
    t[3].im = (*in_0.offset(2)).im - (*in_0.offset(7)).im;
    t[2].im = (*in_0.offset(2)).im + (*in_0.offset(7)).im;
    t[5].re = (*in_0.offset(3)).re - (*in_0.offset(6)).re;
    t[4].re = (*in_0.offset(3)).re + (*in_0.offset(6)).re;
    t[5].im = (*in_0.offset(3)).im - (*in_0.offset(6)).im;
    t[4].im = (*in_0.offset(3)).im + (*in_0.offset(6)).im;
    t[7].re = (*in_0.offset(4)).re - (*in_0.offset(5)).re;
    t[6].re = (*in_0.offset(4)).re + (*in_0.offset(5)).re;
    t[7].im = (*in_0.offset(4)).im - (*in_0.offset(5)).im;
    t[6].im = (*in_0.offset(4)).im + (*in_0.offset(5)).im;
    w[0].re = t[0].re - t[6].re;
    w[0].im = t[0].im - t[6].im;
    w[1].re = t[2].re - t[6].re;
    w[1].im = t[2].im - t[6].im;
    w[2].re = t[1].re - t[7].re;
    w[2].im = t[1].im - t[7].im;
    w[3].re = t[3].re + t[7].re;
    w[3].im = t[3].im + t[7].im;
    z[0].re = dc.re + t[4].re;
    z[0].im = dc.im + t[4].im;
    z[1].re = t[0].re + t[2].re + t[6].re;
    z[1].im = t[0].im + t[2].im + t[6].im;
    (*out.offset((0 as c_long * stride) as isize)).re = z[0].re + z[1].re;
    (*out.offset((0 as c_long * stride) as isize)).im = z[0].im + z[1].im;
    y[3].re = (*tab.offset(0)).im * (t[1].re - t[3].re + t[7].re);
    y[3].im = (*tab.offset(0)).im * (t[1].im - t[3].im + t[7].im);
    x[3].re = z[0].re + (*tab.offset(0)).re * z[1].re;
    x[3].im = z[0].im + (*tab.offset(0)).re * z[1].im;
    z[0].re = dc.re + (*tab.offset(0)).re * t[4].re;
    z[0].im = dc.im + (*tab.offset(0)).re * t[4].im;
    x[1].re = (*tab.offset(1)).re * w[0].re + (*tab.offset(2)).im * w[1].re;
    x[1].im = (*tab.offset(1)).re * w[0].im + (*tab.offset(2)).im * w[1].im;
    x[2].re = (*tab.offset(2)).im * w[0].re - (*tab.offset(3)).re * w[1].re;
    x[2].im = (*tab.offset(2)).im * w[0].im - (*tab.offset(3)).re * w[1].im;
    y[1].re = (*tab.offset(1)).im * w[2].re + (*tab.offset(2)).re * w[3].re;
    y[1].im = (*tab.offset(1)).im * w[2].im + (*tab.offset(2)).re * w[3].im;
    y[2].re = (*tab.offset(2)).re * w[2].re - (*tab.offset(3)).im * w[3].re;
    y[2].im = (*tab.offset(2)).re * w[2].im - (*tab.offset(3)).im * w[3].im;
    y[0].re = (*tab.offset(0)).im * t[5].re;
    y[0].im = (*tab.offset(0)).im * t[5].im;
    x[4].re = x[1].re + x[2].re;
    x[4].im = x[1].im + x[2].im;
    y[4].re = y[1].re - y[2].re;
    y[4].im = y[1].im - y[2].im;
    x[1].re += z[0].re;
    x[1].im += z[0].im;
    y[1].re += y[0].re;
    y[1].im += y[0].im;
    x[2].re += z[0].re;
    x[2].im += z[0].im;
    y[2].re -= y[0].re;
    y[2].im -= y[0].im;
    x[4].re = z[0].re - x[4].re;
    x[4].im = z[0].im - x[4].im;
    y[4].re = y[0].re - y[4].re;
    y[4].im = y[0].im - y[4].im;
    *out.offset((1 as c_long * stride) as isize) = {
        AVComplexFloat {
            re: x[1].re + y[1].im,
            im: x[1].im - y[1].re,
        }
    };
    *out.offset((2 as c_long * stride) as isize) = {
        AVComplexFloat {
            re: x[2].re + y[2].im,
            im: x[2].im - y[2].re,
        }
    };
    *out.offset((3 as c_long * stride) as isize) = {
        AVComplexFloat {
            re: x[3].re + y[3].im,
            im: x[3].im - y[3].re,
        }
    };
    *out.offset((4 as c_long * stride) as isize) = {
        AVComplexFloat {
            re: x[4].re + y[4].im,
            im: x[4].im - y[4].re,
        }
    };
    *out.offset((5 as c_long * stride) as isize) = {
        AVComplexFloat {
            re: x[4].re - y[4].im,
            im: x[4].im + y[4].re,
        }
    };
    *out.offset((6 as c_long * stride) as isize) = {
        AVComplexFloat {
            re: x[3].re - y[3].im,
            im: x[3].im + y[3].re,
        }
    };
    *out.offset((7 as c_long * stride) as isize) = {
        AVComplexFloat {
            re: x[2].re - y[2].im,
            im: x[2].im + y[2].re,
        }
    };
    *out.offset((8 as c_long * stride) as isize) = {
        AVComplexFloat {
            re: x[1].re - y[1].im,
            im: x[1].im + y[1].re,
        }
    };
}
#[inline(always)]
unsafe extern "C" fn fft15(out: *mut TXComplex, in_0: *mut TXComplex, stride: ptrdiff_t) {
    let mut tmp: [TXComplex; 15] = [TXComplex { re: 0., im: 0. }; 15];
    let mut i: c_int = 0;
    while i < 5 {
        fft3(
            tmp.as_mut_ptr().offset(i as isize),
            in_0.offset((i * 3) as isize),
            5 as ptrdiff_t,
        );
        i += 1;
        i;
    }
    fft5_m1(out, tmp.as_mut_ptr().offset(0), stride);
    fft5_m2(out, tmp.as_mut_ptr().offset(5), stride);
    fft5_m3(out, tmp.as_mut_ptr().offset(10), stride);
}
#[cold]
unsafe extern "C" fn ff_tx_fft_factor_init_float_c(
    s: *mut AVTXContext,
    _cd: *const FFTXCodelet,
    flags: c_ulong,
    opts: *mut FFTXCodeletOptions,
    len: c_int,
    _inv: c_int,
    _scale: *const c_void,
) -> c_int {
    let mut ret: c_int = 0;
    ff_tx_init_tabs_float(len);
    if len == 15 {
        ret = ff_tx_gen_pfa_input_map(s, opts, 3, 5);
    } else if flags as c_ulonglong & (1 as c_ulonglong) << 61 != 0 {
        ret = ff_tx_gen_default_map(s, opts);
    }
    ret
}
static mut ff_tx_fft3_ns_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft3_ns_float_c".as_ptr(),
    function: Some(
        ff_tx_fft3_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: (AV_TX_INPLACE as c_int as c_ulonglong
        | (1 as c_ulonglong) << 63
        | AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 61) as c_ulong,
    factors: [3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 3,
    max_len: 3,
    init: Some(
        ff_tx_fft_factor_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

static mut ff_tx_fft3_fwd_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft3_fwd_float_c".as_ptr(),
    function: Some(
        ff_tx_fft3_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: (AV_TX_INPLACE as c_int as c_ulonglong
        | (1 as c_ulonglong) << 63
        | AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 59) as c_ulong,
    factors: [3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 3,
    max_len: 3,
    init: Some(
        ff_tx_fft_factor_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

unsafe extern "C" fn ff_tx_fft3_float_c(
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
unsafe extern "C" fn ff_tx_fft5_float_c(
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
static mut ff_tx_fft5_fwd_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft5_fwd_float_c".as_ptr(),
    function: Some(
        ff_tx_fft5_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: (AV_TX_INPLACE as c_int as c_ulonglong
        | (1 as c_ulonglong) << 63
        | AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 59) as c_ulong,
    factors: [5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 5,
    max_len: 5,
    init: Some(
        ff_tx_fft_factor_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

static mut ff_tx_fft5_ns_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft5_ns_float_c".as_ptr(),
    function: Some(
        ff_tx_fft5_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: (AV_TX_INPLACE as c_int as c_ulonglong
        | (1 as c_ulonglong) << 63
        | AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 61) as c_ulong,
    factors: [5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 5,
    max_len: 5,
    init: Some(
        ff_tx_fft_factor_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

unsafe extern "C" fn ff_tx_fft7_float_c(
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
static mut ff_tx_fft7_ns_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft7_ns_float_c".as_ptr(),
    function: Some(
        ff_tx_fft7_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: (AV_TX_INPLACE as c_int as c_ulonglong
        | (1 as c_ulonglong) << 63
        | AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 61) as c_ulong,
    factors: [7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 7,
    max_len: 7,
    init: Some(
        ff_tx_fft_factor_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

static mut ff_tx_fft7_fwd_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft7_fwd_float_c".as_ptr(),
    function: Some(
        ff_tx_fft7_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: (AV_TX_INPLACE as c_int as c_ulonglong
        | (1 as c_ulonglong) << 63
        | AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 59) as c_ulong,
    factors: [7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 7,
    max_len: 7,
    init: Some(
        ff_tx_fft_factor_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

unsafe extern "C" fn ff_tx_fft9_float_c(
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
static mut ff_tx_fft9_ns_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft9_ns_float_c".as_ptr(),
    function: Some(
        ff_tx_fft9_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: (AV_TX_INPLACE as c_int as c_ulonglong
        | (1 as c_ulonglong) << 63
        | AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 61) as c_ulong,
    factors: [9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 9,
    max_len: 9,
    init: Some(
        ff_tx_fft_factor_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

static mut ff_tx_fft9_fwd_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft9_fwd_float_c".as_ptr(),
    function: Some(
        ff_tx_fft9_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: (AV_TX_INPLACE as c_int as c_ulonglong
        | (1 as c_ulonglong) << 63
        | AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 59) as c_ulong,
    factors: [9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 9,
    max_len: 9,
    init: Some(
        ff_tx_fft_factor_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

static mut ff_tx_fft15_ns_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft15_ns_float_c".as_ptr(),
    function: Some(
        ff_tx_fft15_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: (AV_TX_INPLACE as c_int as c_ulonglong
        | (1 as c_ulonglong) << 63
        | AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 61) as c_ulong,
    factors: [15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 15,
    max_len: 15,
    init: Some(
        ff_tx_fft_factor_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

unsafe extern "C" fn ff_tx_fft15_float_c(
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
unsafe extern "C" fn ff_tx_fft_sr_combine_float_c(
    mut z: *mut TXComplex,
    mut cos_0: *const TXSample,
    len: c_int,
) {
    let o1: c_int = 2 * len;
    let o2: c_int = 4 * len;
    let o3: c_int = 6 * len;
    let mut wim: *const TXSample = cos_0.offset(o1 as isize).offset(-7);
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
    let mut i: c_int = 0;
    while i < len {
        t1 = (*z.offset((o2 + 0) as isize)).re * *cos_0.offset(0)
            - (*z.offset((o2 + 0) as isize)).im * -*wim.offset(7);
        t2 = (*z.offset((o2 + 0) as isize)).re * -*wim.offset(7)
            + (*z.offset((o2 + 0) as isize)).im * *cos_0.offset(0);
        t5 = (*z.offset((o3 + 0) as isize)).re * *cos_0.offset(0)
            - (*z.offset((o3 + 0) as isize)).im * *wim.offset(7);
        t6 = (*z.offset((o3 + 0) as isize)).re * *wim.offset(7)
            + (*z.offset((o3 + 0) as isize)).im * *cos_0.offset(0);
        r0 = (*z.offset(0)).re;
        i0 = (*z.offset(0)).im;
        r1 = (*z.offset((o1 + 0) as isize)).re;
        i1 = (*z.offset((o1 + 0) as isize)).im;
        t3 = t5 - t1;
        t5 += t1;
        (*z.offset((o2 + 0) as isize)).re = r0 - t5;
        (*z.offset(0)).re = r0 + t5;
        (*z.offset((o3 + 0) as isize)).im = i1 - t3;
        (*z.offset((o1 + 0) as isize)).im = i1 + t3;
        t4 = t2 - t6;
        t6 += t2;
        (*z.offset((o3 + 0) as isize)).re = r1 - t4;
        (*z.offset((o1 + 0) as isize)).re = r1 + t4;
        (*z.offset((o2 + 0) as isize)).im = i0 - t6;
        (*z.offset(0)).im = i0 + t6;
        t1 = (*z.offset((o2 + 2) as isize)).re * *cos_0.offset(2)
            - (*z.offset((o2 + 2) as isize)).im * -*wim.offset(5);
        t2 = (*z.offset((o2 + 2) as isize)).re * -*wim.offset(5)
            + (*z.offset((o2 + 2) as isize)).im * *cos_0.offset(2);
        t5 = (*z.offset((o3 + 2) as isize)).re * *cos_0.offset(2)
            - (*z.offset((o3 + 2) as isize)).im * *wim.offset(5);
        t6 = (*z.offset((o3 + 2) as isize)).re * *wim.offset(5)
            + (*z.offset((o3 + 2) as isize)).im * *cos_0.offset(2);
        r0 = (*z.offset(2)).re;
        i0 = (*z.offset(2)).im;
        r1 = (*z.offset((o1 + 2) as isize)).re;
        i1 = (*z.offset((o1 + 2) as isize)).im;
        t3 = t5 - t1;
        t5 += t1;
        (*z.offset((o2 + 2) as isize)).re = r0 - t5;
        (*z.offset(2)).re = r0 + t5;
        (*z.offset((o3 + 2) as isize)).im = i1 - t3;
        (*z.offset((o1 + 2) as isize)).im = i1 + t3;
        t4 = t2 - t6;
        t6 += t2;
        (*z.offset((o3 + 2) as isize)).re = r1 - t4;
        (*z.offset((o1 + 2) as isize)).re = r1 + t4;
        (*z.offset((o2 + 2) as isize)).im = i0 - t6;
        (*z.offset(2)).im = i0 + t6;
        t1 = (*z.offset((o2 + 4) as isize)).re * *cos_0.offset(4)
            - (*z.offset((o2 + 4) as isize)).im * -*wim.offset(3);
        t2 = (*z.offset((o2 + 4) as isize)).re * -*wim.offset(3)
            + (*z.offset((o2 + 4) as isize)).im * *cos_0.offset(4);
        t5 = (*z.offset((o3 + 4) as isize)).re * *cos_0.offset(4)
            - (*z.offset((o3 + 4) as isize)).im * *wim.offset(3);
        t6 = (*z.offset((o3 + 4) as isize)).re * *wim.offset(3)
            + (*z.offset((o3 + 4) as isize)).im * *cos_0.offset(4);
        r0 = (*z.offset(4)).re;
        i0 = (*z.offset(4)).im;
        r1 = (*z.offset((o1 + 4) as isize)).re;
        i1 = (*z.offset((o1 + 4) as isize)).im;
        t3 = t5 - t1;
        t5 += t1;
        (*z.offset((o2 + 4) as isize)).re = r0 - t5;
        (*z.offset(4)).re = r0 + t5;
        (*z.offset((o3 + 4) as isize)).im = i1 - t3;
        (*z.offset((o1 + 4) as isize)).im = i1 + t3;
        t4 = t2 - t6;
        t6 += t2;
        (*z.offset((o3 + 4) as isize)).re = r1 - t4;
        (*z.offset((o1 + 4) as isize)).re = r1 + t4;
        (*z.offset((o2 + 4) as isize)).im = i0 - t6;
        (*z.offset(4)).im = i0 + t6;
        t1 = (*z.offset((o2 + 6) as isize)).re * *cos_0.offset(6)
            - (*z.offset((o2 + 6) as isize)).im * -*wim.offset(1);
        t2 = (*z.offset((o2 + 6) as isize)).re * -*wim.offset(1)
            + (*z.offset((o2 + 6) as isize)).im * *cos_0.offset(6);
        t5 = (*z.offset((o3 + 6) as isize)).re * *cos_0.offset(6)
            - (*z.offset((o3 + 6) as isize)).im * *wim.offset(1);
        t6 = (*z.offset((o3 + 6) as isize)).re * *wim.offset(1)
            + (*z.offset((o3 + 6) as isize)).im * *cos_0.offset(6);
        r0 = (*z.offset(6)).re;
        i0 = (*z.offset(6)).im;
        r1 = (*z.offset((o1 + 6) as isize)).re;
        i1 = (*z.offset((o1 + 6) as isize)).im;
        t3 = t5 - t1;
        t5 += t1;
        (*z.offset((o2 + 6) as isize)).re = r0 - t5;
        (*z.offset(6)).re = r0 + t5;
        (*z.offset((o3 + 6) as isize)).im = i1 - t3;
        (*z.offset((o1 + 6) as isize)).im = i1 + t3;
        t4 = t2 - t6;
        t6 += t2;
        (*z.offset((o3 + 6) as isize)).re = r1 - t4;
        (*z.offset((o1 + 6) as isize)).re = r1 + t4;
        (*z.offset((o2 + 6) as isize)).im = i0 - t6;
        (*z.offset(6)).im = i0 + t6;
        t1 = (*z.offset((o2 + 1) as isize)).re * *cos_0.offset(1)
            - (*z.offset((o2 + 1) as isize)).im * -*wim.offset(6);
        t2 = (*z.offset((o2 + 1) as isize)).re * -*wim.offset(6)
            + (*z.offset((o2 + 1) as isize)).im * *cos_0.offset(1);
        t5 = (*z.offset((o3 + 1) as isize)).re * *cos_0.offset(1)
            - (*z.offset((o3 + 1) as isize)).im * *wim.offset(6);
        t6 = (*z.offset((o3 + 1) as isize)).re * *wim.offset(6)
            + (*z.offset((o3 + 1) as isize)).im * *cos_0.offset(1);
        r0 = (*z.offset(1)).re;
        i0 = (*z.offset(1)).im;
        r1 = (*z.offset((o1 + 1) as isize)).re;
        i1 = (*z.offset((o1 + 1) as isize)).im;
        t3 = t5 - t1;
        t5 += t1;
        (*z.offset((o2 + 1) as isize)).re = r0 - t5;
        (*z.offset(1)).re = r0 + t5;
        (*z.offset((o3 + 1) as isize)).im = i1 - t3;
        (*z.offset((o1 + 1) as isize)).im = i1 + t3;
        t4 = t2 - t6;
        t6 += t2;
        (*z.offset((o3 + 1) as isize)).re = r1 - t4;
        (*z.offset((o1 + 1) as isize)).re = r1 + t4;
        (*z.offset((o2 + 1) as isize)).im = i0 - t6;
        (*z.offset(1)).im = i0 + t6;
        t1 = (*z.offset((o2 + 3) as isize)).re * *cos_0.offset(3)
            - (*z.offset((o2 + 3) as isize)).im * -*wim.offset(4);
        t2 = (*z.offset((o2 + 3) as isize)).re * -*wim.offset(4)
            + (*z.offset((o2 + 3) as isize)).im * *cos_0.offset(3);
        t5 = (*z.offset((o3 + 3) as isize)).re * *cos_0.offset(3)
            - (*z.offset((o3 + 3) as isize)).im * *wim.offset(4);
        t6 = (*z.offset((o3 + 3) as isize)).re * *wim.offset(4)
            + (*z.offset((o3 + 3) as isize)).im * *cos_0.offset(3);
        r0 = (*z.offset(3)).re;
        i0 = (*z.offset(3)).im;
        r1 = (*z.offset((o1 + 3) as isize)).re;
        i1 = (*z.offset((o1 + 3) as isize)).im;
        t3 = t5 - t1;
        t5 += t1;
        (*z.offset((o2 + 3) as isize)).re = r0 - t5;
        (*z.offset(3)).re = r0 + t5;
        (*z.offset((o3 + 3) as isize)).im = i1 - t3;
        (*z.offset((o1 + 3) as isize)).im = i1 + t3;
        t4 = t2 - t6;
        t6 += t2;
        (*z.offset((o3 + 3) as isize)).re = r1 - t4;
        (*z.offset((o1 + 3) as isize)).re = r1 + t4;
        (*z.offset((o2 + 3) as isize)).im = i0 - t6;
        (*z.offset(3)).im = i0 + t6;
        t1 = (*z.offset((o2 + 5) as isize)).re * *cos_0.offset(5)
            - (*z.offset((o2 + 5) as isize)).im * -*wim.offset(2);
        t2 = (*z.offset((o2 + 5) as isize)).re * -*wim.offset(2)
            + (*z.offset((o2 + 5) as isize)).im * *cos_0.offset(5);
        t5 = (*z.offset((o3 + 5) as isize)).re * *cos_0.offset(5)
            - (*z.offset((o3 + 5) as isize)).im * *wim.offset(2);
        t6 = (*z.offset((o3 + 5) as isize)).re * *wim.offset(2)
            + (*z.offset((o3 + 5) as isize)).im * *cos_0.offset(5);
        r0 = (*z.offset(5)).re;
        i0 = (*z.offset(5)).im;
        r1 = (*z.offset((o1 + 5) as isize)).re;
        i1 = (*z.offset((o1 + 5) as isize)).im;
        t3 = t5 - t1;
        t5 += t1;
        (*z.offset((o2 + 5) as isize)).re = r0 - t5;
        (*z.offset(5)).re = r0 + t5;
        (*z.offset((o3 + 5) as isize)).im = i1 - t3;
        (*z.offset((o1 + 5) as isize)).im = i1 + t3;
        t4 = t2 - t6;
        t6 += t2;
        (*z.offset((o3 + 5) as isize)).re = r1 - t4;
        (*z.offset((o1 + 5) as isize)).re = r1 + t4;
        (*z.offset((o2 + 5) as isize)).im = i0 - t6;
        (*z.offset(5)).im = i0 + t6;
        t1 = (*z.offset((o2 + 7) as isize)).re * *cos_0.offset(7)
            - (*z.offset((o2 + 7) as isize)).im * -*wim.offset(0);
        t2 = (*z.offset((o2 + 7) as isize)).re * -*wim.offset(0)
            + (*z.offset((o2 + 7) as isize)).im * *cos_0.offset(7);
        t5 = (*z.offset((o3 + 7) as isize)).re * *cos_0.offset(7)
            - (*z.offset((o3 + 7) as isize)).im * *wim.offset(0);
        t6 = (*z.offset((o3 + 7) as isize)).re * *wim.offset(0)
            + (*z.offset((o3 + 7) as isize)).im * *cos_0.offset(7);
        r0 = (*z.offset(7)).re;
        i0 = (*z.offset(7)).im;
        r1 = (*z.offset((o1 + 7) as isize)).re;
        i1 = (*z.offset((o1 + 7) as isize)).im;
        t3 = t5 - t1;
        t5 += t1;
        (*z.offset((o2 + 7) as isize)).re = r0 - t5;
        (*z.offset(7)).re = r0 + t5;
        (*z.offset((o3 + 7) as isize)).im = i1 - t3;
        (*z.offset((o1 + 7) as isize)).im = i1 + t3;
        t4 = t2 - t6;
        t6 += t2;
        (*z.offset((o3 + 7) as isize)).re = r1 - t4;
        (*z.offset((o1 + 7) as isize)).re = r1 + t4;
        (*z.offset((o2 + 7) as isize)).im = i0 - t6;
        (*z.offset(7)).im = i0 + t6;
        z = z.offset((2 * 4) as isize);
        cos_0 = cos_0.offset((2 * 4) as isize);
        wim = wim.offset(-((2 * 4) as isize));
        i += 4;
    }
}
#[cold]
unsafe extern "C" fn ff_tx_fft_sr_codelet_init_float_c(
    s: *mut AVTXContext,
    _cd: *const FFTXCodelet,
    _flags: c_ulong,
    opts: *mut FFTXCodeletOptions,
    len: c_int,
    _inv: c_int,
    _scale: *const c_void,
) -> c_int {
    ff_tx_init_tabs_float(len);
    ff_tx_gen_ptwo_revtab(s, opts)
}
unsafe extern "C" fn ff_tx_fft2_ns_float_c(
    _s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    _stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let mut tmp: TXComplex = TXComplex { re: 0., im: 0. };
    tmp.re = (*src.offset(0)).re - (*src.offset(1)).re;
    (*dst.offset(0)).re = (*src.offset(0)).re + (*src.offset(1)).re;
    tmp.im = (*src.offset(0)).im - (*src.offset(1)).im;
    (*dst.offset(0)).im = (*src.offset(0)).im + (*src.offset(1)).im;
    *dst.offset(1) = tmp;
}
unsafe extern "C" fn ff_tx_fft4_ns_float_c(
    _s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    _stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let mut t1: TXSample = 0.;
    let mut t2: TXSample = 0.;
    let mut t3: TXSample = 0.;
    let mut t4: TXSample = 0.;
    let mut t5: TXSample = 0.;
    let mut t6: TXSample = 0.;
    let mut t7: TXSample = 0.;
    let mut t8: TXSample = 0.;
    t3 = (*src.offset(0)).re - (*src.offset(1)).re;
    t1 = (*src.offset(0)).re + (*src.offset(1)).re;
    t8 = (*src.offset(3)).re - (*src.offset(2)).re;
    t6 = (*src.offset(3)).re + (*src.offset(2)).re;
    (*dst.offset(2)).re = t1 - t6;
    (*dst.offset(0)).re = t1 + t6;
    t4 = (*src.offset(0)).im - (*src.offset(1)).im;
    t2 = (*src.offset(0)).im + (*src.offset(1)).im;
    t7 = (*src.offset(2)).im - (*src.offset(3)).im;
    t5 = (*src.offset(2)).im + (*src.offset(3)).im;
    (*dst.offset(3)).im = t4 - t8;
    (*dst.offset(1)).im = t4 + t8;
    (*dst.offset(3)).re = t3 - t7;
    (*dst.offset(1)).re = t3 + t7;
    (*dst.offset(2)).im = t2 - t5;
    (*dst.offset(0)).im = t2 + t5;
}
unsafe extern "C" fn ff_tx_fft8_ns_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
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
    let cos_0: TXSample = ff_tx_tab_8_float[1];
    ff_tx_fft4_ns_float_c(s, dst as *mut c_void, src as *mut c_void, stride);
    t1 = (*src.offset(4)).re - -(*src.offset(5)).re;
    (*dst.offset(5)).re = (*src.offset(4)).re + -(*src.offset(5)).re;
    t2 = (*src.offset(4)).im - -(*src.offset(5)).im;
    (*dst.offset(5)).im = (*src.offset(4)).im + -(*src.offset(5)).im;
    t5 = (*src.offset(6)).re - -(*src.offset(7)).re;
    (*dst.offset(7)).re = (*src.offset(6)).re + -(*src.offset(7)).re;
    t6 = (*src.offset(6)).im - -(*src.offset(7)).im;
    (*dst.offset(7)).im = (*src.offset(6)).im + -(*src.offset(7)).im;
    r0 = (*dst.offset(0)).re;
    i0 = (*dst.offset(0)).im;
    r1 = (*dst.offset(2)).re;
    i1 = (*dst.offset(2)).im;
    t3 = t5 - t1;
    t5 += t1;
    (*dst.offset(4)).re = r0 - t5;
    (*dst.offset(0)).re = r0 + t5;
    (*dst.offset(6)).im = i1 - t3;
    (*dst.offset(2)).im = i1 + t3;
    t4 = t2 - t6;
    t6 += t2;
    (*dst.offset(6)).re = r1 - t4;
    (*dst.offset(2)).re = r1 + t4;
    (*dst.offset(4)).im = i0 - t6;
    (*dst.offset(0)).im = i0 + t6;
    t1 = (*dst.offset(5)).re * cos_0 - (*dst.offset(5)).im * -cos_0;
    t2 = (*dst.offset(5)).re * -cos_0 + (*dst.offset(5)).im * cos_0;
    t5 = (*dst.offset(7)).re * cos_0 - (*dst.offset(7)).im * cos_0;
    t6 = (*dst.offset(7)).re * cos_0 + (*dst.offset(7)).im * cos_0;
    r0 = (*dst.offset(1)).re;
    i0 = (*dst.offset(1)).im;
    r1 = (*dst.offset(3)).re;
    i1 = (*dst.offset(3)).im;
    t3 = t5 - t1;
    t5 += t1;
    (*dst.offset(5)).re = r0 - t5;
    (*dst.offset(1)).re = r0 + t5;
    (*dst.offset(7)).im = i1 - t3;
    (*dst.offset(3)).im = i1 + t3;
    t4 = t2 - t6;
    t6 += t2;
    (*dst.offset(7)).re = r1 - t4;
    (*dst.offset(3)).re = r1 + t4;
    (*dst.offset(5)).im = i0 - t6;
    (*dst.offset(1)).im = i0 + t6;
}
unsafe extern "C" fn ff_tx_fft16_ns_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_16_float.as_mut_ptr();
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
    let cos_16_1: TXSample = *cos_0.offset(1);
    let cos_16_2: TXSample = *cos_0.offset(2);
    let cos_16_3: TXSample = *cos_0.offset(3);
    ff_tx_fft8_ns_float_c(
        s,
        dst.offset(0) as *mut c_void,
        src.offset(0) as *mut c_void,
        stride,
    );
    ff_tx_fft4_ns_float_c(
        s,
        dst.offset(8) as *mut c_void,
        src.offset(8) as *mut c_void,
        stride,
    );
    ff_tx_fft4_ns_float_c(
        s,
        dst.offset(12) as *mut c_void,
        src.offset(12) as *mut c_void,
        stride,
    );
    t1 = (*dst.offset(8)).re;
    t2 = (*dst.offset(8)).im;
    t5 = (*dst.offset(12)).re;
    t6 = (*dst.offset(12)).im;
    r0 = (*dst.offset(0)).re;
    i0 = (*dst.offset(0)).im;
    r1 = (*dst.offset(4)).re;
    i1 = (*dst.offset(4)).im;
    t3 = t5 - t1;
    t5 += t1;
    (*dst.offset(8)).re = r0 - t5;
    (*dst.offset(0)).re = r0 + t5;
    (*dst.offset(12)).im = i1 - t3;
    (*dst.offset(4)).im = i1 + t3;
    t4 = t2 - t6;
    t6 += t2;
    (*dst.offset(12)).re = r1 - t4;
    (*dst.offset(4)).re = r1 + t4;
    (*dst.offset(8)).im = i0 - t6;
    (*dst.offset(0)).im = i0 + t6;
    t1 = (*dst.offset(10)).re * cos_16_2 - (*dst.offset(10)).im * -cos_16_2;
    t2 = (*dst.offset(10)).re * -cos_16_2 + (*dst.offset(10)).im * cos_16_2;
    t5 = (*dst.offset(14)).re * cos_16_2 - (*dst.offset(14)).im * cos_16_2;
    t6 = (*dst.offset(14)).re * cos_16_2 + (*dst.offset(14)).im * cos_16_2;
    r0 = (*dst.offset(2)).re;
    i0 = (*dst.offset(2)).im;
    r1 = (*dst.offset(6)).re;
    i1 = (*dst.offset(6)).im;
    t3 = t5 - t1;
    t5 += t1;
    (*dst.offset(10)).re = r0 - t5;
    (*dst.offset(2)).re = r0 + t5;
    (*dst.offset(14)).im = i1 - t3;
    (*dst.offset(6)).im = i1 + t3;
    t4 = t2 - t6;
    t6 += t2;
    (*dst.offset(14)).re = r1 - t4;
    (*dst.offset(6)).re = r1 + t4;
    (*dst.offset(10)).im = i0 - t6;
    (*dst.offset(2)).im = i0 + t6;
    t1 = (*dst.offset(9)).re * cos_16_1 - (*dst.offset(9)).im * -cos_16_3;
    t2 = (*dst.offset(9)).re * -cos_16_3 + (*dst.offset(9)).im * cos_16_1;
    t5 = (*dst.offset(13)).re * cos_16_1 - (*dst.offset(13)).im * cos_16_3;
    t6 = (*dst.offset(13)).re * cos_16_3 + (*dst.offset(13)).im * cos_16_1;
    r0 = (*dst.offset(1)).re;
    i0 = (*dst.offset(1)).im;
    r1 = (*dst.offset(5)).re;
    i1 = (*dst.offset(5)).im;
    t3 = t5 - t1;
    t5 += t1;
    (*dst.offset(9)).re = r0 - t5;
    (*dst.offset(1)).re = r0 + t5;
    (*dst.offset(13)).im = i1 - t3;
    (*dst.offset(5)).im = i1 + t3;
    t4 = t2 - t6;
    t6 += t2;
    (*dst.offset(13)).re = r1 - t4;
    (*dst.offset(5)).re = r1 + t4;
    (*dst.offset(9)).im = i0 - t6;
    (*dst.offset(1)).im = i0 + t6;
    t1 = (*dst.offset(11)).re * cos_16_3 - (*dst.offset(11)).im * -cos_16_1;
    t2 = (*dst.offset(11)).re * -cos_16_1 + (*dst.offset(11)).im * cos_16_3;
    t5 = (*dst.offset(15)).re * cos_16_3 - (*dst.offset(15)).im * cos_16_1;
    t6 = (*dst.offset(15)).re * cos_16_1 + (*dst.offset(15)).im * cos_16_3;
    r0 = (*dst.offset(3)).re;
    i0 = (*dst.offset(3)).im;
    r1 = (*dst.offset(7)).re;
    i1 = (*dst.offset(7)).im;
    t3 = t5 - t1;
    t5 += t1;
    (*dst.offset(11)).re = r0 - t5;
    (*dst.offset(3)).re = r0 + t5;
    (*dst.offset(15)).im = i1 - t3;
    (*dst.offset(7)).im = i1 + t3;
    t4 = t2 - t6;
    t6 += t2;
    (*dst.offset(15)).re = r1 - t4;
    (*dst.offset(7)).re = r1 + t4;
    (*dst.offset(11)).im = i0 - t6;
    (*dst.offset(3)).im = i0 + t6;
}
static mut ff_tx_fft2_ns_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft2_ns_float_c".as_ptr(),
    function: Some(
        ff_tx_fft2_ns_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: ((1 as c_ulonglong) << 63
        | AV_TX_INPLACE as c_int as c_ulonglong
        | AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 61) as c_ulong,
    factors: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 2,
    max_len: 2,
    init: Some(
        ff_tx_fft_sr_codelet_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

static mut ff_tx_fft4_ns_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft4_ns_float_c".as_ptr(),
    function: Some(
        ff_tx_fft4_ns_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: ((1 as c_ulonglong) << 63
        | AV_TX_INPLACE as c_int as c_ulonglong
        | AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 61) as c_ulong,
    factors: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 4,
    max_len: 4,
    init: Some(
        ff_tx_fft_sr_codelet_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

static mut ff_tx_fft8_ns_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft8_ns_float_c".as_ptr(),
    function: Some(
        ff_tx_fft8_ns_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: ((1 as c_ulonglong) << 63
        | AV_TX_INPLACE as c_int as c_ulonglong
        | AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 61) as c_ulong,
    factors: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 8,
    max_len: 8,
    init: Some(
        ff_tx_fft_sr_codelet_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

static mut ff_tx_fft16_ns_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft16_ns_float_c".as_ptr(),
    function: Some(
        ff_tx_fft16_ns_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: ((1 as c_ulonglong) << 63
        | AV_TX_INPLACE as c_int as c_ulonglong
        | AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 61) as c_ulong,
    factors: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 16,
    max_len: 16,
    init: Some(
        ff_tx_fft_sr_codelet_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

static mut ff_tx_fft32_ns_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft32_ns_float_c".as_ptr(),
    function: Some(
        ff_tx_fft32_ns_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: ((1 as c_ulonglong) << 63
        | AV_TX_INPLACE as c_int as c_ulonglong
        | AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 61) as c_ulong,
    factors: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 32,
    max_len: 32,
    init: Some(
        ff_tx_fft_sr_codelet_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

unsafe extern "C" fn ff_tx_fft32_ns_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_32_float.as_mut_ptr();
    ff_tx_fft16_ns_float_c(s, dst as *mut c_void, src as *mut c_void, stride);
    ff_tx_fft8_ns_float_c(
        s,
        dst.offset((8 * 2) as isize) as *mut c_void,
        src.offset((8 * 2) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft8_ns_float_c(
        s,
        dst.offset((8 * 3) as isize) as *mut c_void,
        src.offset((8 * 3) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft_sr_combine_float_c(dst, cos_0, 8 >> 1);
}
unsafe extern "C" fn ff_tx_fft64_ns_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_64_float.as_mut_ptr();
    ff_tx_fft32_ns_float_c(s, dst as *mut c_void, src as *mut c_void, stride);
    ff_tx_fft16_ns_float_c(
        s,
        dst.offset((16 * 2) as isize) as *mut c_void,
        src.offset((16 * 2) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft16_ns_float_c(
        s,
        dst.offset((16 * 3) as isize) as *mut c_void,
        src.offset((16 * 3) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft_sr_combine_float_c(dst, cos_0, 16 >> 1);
}
static mut ff_tx_fft64_ns_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft64_ns_float_c".as_ptr(),
    function: Some(
        ff_tx_fft64_ns_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: ((1 as c_ulonglong) << 63
        | AV_TX_INPLACE as c_int as c_ulonglong
        | AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 61) as c_ulong,
    factors: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 64,
    max_len: 64,
    init: Some(
        ff_tx_fft_sr_codelet_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

unsafe extern "C" fn ff_tx_fft128_ns_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_128_float.as_mut_ptr();
    ff_tx_fft64_ns_float_c(s, dst as *mut c_void, src as *mut c_void, stride);
    ff_tx_fft32_ns_float_c(
        s,
        dst.offset((32 * 2) as isize) as *mut c_void,
        src.offset((32 * 2) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft32_ns_float_c(
        s,
        dst.offset((32 * 3) as isize) as *mut c_void,
        src.offset((32 * 3) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft_sr_combine_float_c(dst, cos_0, 32 >> 1);
}
static mut ff_tx_fft128_ns_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft128_ns_float_c".as_ptr(),
    function: Some(
        ff_tx_fft128_ns_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: ((1 as c_ulonglong) << 63
        | AV_TX_INPLACE as c_int as c_ulonglong
        | AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 61) as c_ulong,
    factors: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 128,
    max_len: 128,
    init: Some(
        ff_tx_fft_sr_codelet_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

unsafe extern "C" fn ff_tx_fft256_ns_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_256_float.as_mut_ptr();
    ff_tx_fft128_ns_float_c(s, dst as *mut c_void, src as *mut c_void, stride);
    ff_tx_fft64_ns_float_c(
        s,
        dst.offset((64 * 2) as isize) as *mut c_void,
        src.offset((64 * 2) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft64_ns_float_c(
        s,
        dst.offset((64 * 3) as isize) as *mut c_void,
        src.offset((64 * 3) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft_sr_combine_float_c(dst, cos_0, 64 >> 1);
}
static mut ff_tx_fft256_ns_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft256_ns_float_c".as_ptr(),
    function: Some(
        ff_tx_fft256_ns_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: ((1 as c_ulonglong) << 63
        | AV_TX_INPLACE as c_int as c_ulonglong
        | AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 61) as c_ulong,
    factors: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 256,
    max_len: 256,
    init: Some(
        ff_tx_fft_sr_codelet_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

unsafe extern "C" fn ff_tx_fft512_ns_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_512_float.as_mut_ptr();
    ff_tx_fft256_ns_float_c(s, dst as *mut c_void, src as *mut c_void, stride);
    ff_tx_fft128_ns_float_c(
        s,
        dst.offset((128 * 2) as isize) as *mut c_void,
        src.offset((128 * 2) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft128_ns_float_c(
        s,
        dst.offset((128 * 3) as isize) as *mut c_void,
        src.offset((128 * 3) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft_sr_combine_float_c(dst, cos_0, 128 >> 1);
}
static mut ff_tx_fft512_ns_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft512_ns_float_c".as_ptr(),
    function: Some(
        ff_tx_fft512_ns_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: ((1 as c_ulonglong) << 63
        | AV_TX_INPLACE as c_int as c_ulonglong
        | AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 61) as c_ulong,
    factors: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 512,
    max_len: 512,
    init: Some(
        ff_tx_fft_sr_codelet_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

unsafe extern "C" fn ff_tx_fft1024_ns_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_1024_float.as_mut_ptr();
    ff_tx_fft512_ns_float_c(s, dst as *mut c_void, src as *mut c_void, stride);
    ff_tx_fft256_ns_float_c(
        s,
        dst.offset((256 * 2) as isize) as *mut c_void,
        src.offset((256 * 2) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft256_ns_float_c(
        s,
        dst.offset((256 * 3) as isize) as *mut c_void,
        src.offset((256 * 3) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft_sr_combine_float_c(dst, cos_0, 256 >> 1);
}
static mut ff_tx_fft1024_ns_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft1024_ns_float_c".as_ptr(),
    function: Some(
        ff_tx_fft1024_ns_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: ((1 as c_ulonglong) << 63
        | AV_TX_INPLACE as c_int as c_ulonglong
        | AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 61) as c_ulong,
    factors: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 1024,
    max_len: 1024,
    init: Some(
        ff_tx_fft_sr_codelet_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

unsafe extern "C" fn ff_tx_fft2048_ns_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_2048_float.as_mut_ptr();
    ff_tx_fft1024_ns_float_c(s, dst as *mut c_void, src as *mut c_void, stride);
    ff_tx_fft512_ns_float_c(
        s,
        dst.offset((512 * 2) as isize) as *mut c_void,
        src.offset((512 * 2) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft512_ns_float_c(
        s,
        dst.offset((512 * 3) as isize) as *mut c_void,
        src.offset((512 * 3) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft_sr_combine_float_c(dst, cos_0, 512 >> 1);
}
static mut ff_tx_fft2048_ns_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft2048_ns_float_c".as_ptr(),
    function: Some(
        ff_tx_fft2048_ns_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: ((1 as c_ulonglong) << 63
        | AV_TX_INPLACE as c_int as c_ulonglong
        | AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 61) as c_ulong,
    factors: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 2048,
    max_len: 2048,
    init: Some(
        ff_tx_fft_sr_codelet_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

unsafe extern "C" fn ff_tx_fft4096_ns_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_4096_float.as_mut_ptr();
    ff_tx_fft2048_ns_float_c(s, dst as *mut c_void, src as *mut c_void, stride);
    ff_tx_fft1024_ns_float_c(
        s,
        dst.offset((1024 * 2) as isize) as *mut c_void,
        src.offset((1024 * 2) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft1024_ns_float_c(
        s,
        dst.offset((1024 * 3) as isize) as *mut c_void,
        src.offset((1024 * 3) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft_sr_combine_float_c(dst, cos_0, 1024 >> 1);
}
static mut ff_tx_fft4096_ns_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft4096_ns_float_c".as_ptr(),
    function: Some(
        ff_tx_fft4096_ns_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: ((1 as c_ulonglong) << 63
        | AV_TX_INPLACE as c_int as c_ulonglong
        | AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 61) as c_ulong,
    factors: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 4096,
    max_len: 4096,
    init: Some(
        ff_tx_fft_sr_codelet_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

static mut ff_tx_fft8192_ns_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft8192_ns_float_c".as_ptr(),
    function: Some(
        ff_tx_fft8192_ns_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: ((1 as c_ulonglong) << 63
        | AV_TX_INPLACE as c_int as c_ulonglong
        | AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 61) as c_ulong,
    factors: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 8192,
    max_len: 8192,
    init: Some(
        ff_tx_fft_sr_codelet_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

unsafe extern "C" fn ff_tx_fft8192_ns_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_8192_float.as_mut_ptr();
    ff_tx_fft4096_ns_float_c(s, dst as *mut c_void, src as *mut c_void, stride);
    ff_tx_fft2048_ns_float_c(
        s,
        dst.offset((2048 * 2) as isize) as *mut c_void,
        src.offset((2048 * 2) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft2048_ns_float_c(
        s,
        dst.offset((2048 * 3) as isize) as *mut c_void,
        src.offset((2048 * 3) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft_sr_combine_float_c(dst, cos_0, 2048 >> 1);
}
static mut ff_tx_fft16384_ns_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft16384_ns_float_c".as_ptr(),
    function: Some(
        ff_tx_fft16384_ns_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: ((1 as c_ulonglong) << 63
        | AV_TX_INPLACE as c_int as c_ulonglong
        | AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 61) as c_ulong,
    factors: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 16384,
    max_len: 16384,
    init: Some(
        ff_tx_fft_sr_codelet_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

unsafe extern "C" fn ff_tx_fft16384_ns_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_16384_float.as_mut_ptr();
    ff_tx_fft8192_ns_float_c(s, dst as *mut c_void, src as *mut c_void, stride);
    ff_tx_fft4096_ns_float_c(
        s,
        dst.offset((4096 * 2) as isize) as *mut c_void,
        src.offset((4096 * 2) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft4096_ns_float_c(
        s,
        dst.offset((4096 * 3) as isize) as *mut c_void,
        src.offset((4096 * 3) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft_sr_combine_float_c(dst, cos_0, 4096 >> 1);
}
static mut ff_tx_fft32768_ns_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft32768_ns_float_c".as_ptr(),
    function: Some(
        ff_tx_fft32768_ns_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: ((1 as c_ulonglong) << 63
        | AV_TX_INPLACE as c_int as c_ulonglong
        | AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 61) as c_ulong,
    factors: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 32768,
    max_len: 32768,
    init: Some(
        ff_tx_fft_sr_codelet_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

unsafe extern "C" fn ff_tx_fft32768_ns_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_32768_float.as_mut_ptr();
    ff_tx_fft16384_ns_float_c(s, dst as *mut c_void, src as *mut c_void, stride);
    ff_tx_fft8192_ns_float_c(
        s,
        dst.offset((8192 * 2) as isize) as *mut c_void,
        src.offset((8192 * 2) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft8192_ns_float_c(
        s,
        dst.offset((8192 * 3) as isize) as *mut c_void,
        src.offset((8192 * 3) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft_sr_combine_float_c(dst, cos_0, 8192 >> 1);
}
unsafe extern "C" fn ff_tx_fft65536_ns_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_65536_float.as_mut_ptr();
    ff_tx_fft32768_ns_float_c(s, dst as *mut c_void, src as *mut c_void, stride);
    ff_tx_fft16384_ns_float_c(
        s,
        dst.offset((16384 * 2) as isize) as *mut c_void,
        src.offset((16384 * 2) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft16384_ns_float_c(
        s,
        dst.offset((16384 * 3) as isize) as *mut c_void,
        src.offset((16384 * 3) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft_sr_combine_float_c(dst, cos_0, 16384 >> 1);
}
static mut ff_tx_fft65536_ns_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft65536_ns_float_c".as_ptr(),
    function: Some(
        ff_tx_fft65536_ns_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: ((1 as c_ulonglong) << 63
        | AV_TX_INPLACE as c_int as c_ulonglong
        | AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 61) as c_ulong,
    factors: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 65536,
    max_len: 65536,
    init: Some(
        ff_tx_fft_sr_codelet_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

static mut ff_tx_fft131072_ns_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft131072_ns_float_c".as_ptr(),
    function: Some(
        ff_tx_fft131072_ns_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: ((1 as c_ulonglong) << 63
        | AV_TX_INPLACE as c_int as c_ulonglong
        | AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 61) as c_ulong,
    factors: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 131072,
    max_len: 131072,
    init: Some(
        ff_tx_fft_sr_codelet_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

unsafe extern "C" fn ff_tx_fft131072_ns_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_131072_float.as_mut_ptr();
    ff_tx_fft65536_ns_float_c(s, dst as *mut c_void, src as *mut c_void, stride);
    ff_tx_fft32768_ns_float_c(
        s,
        dst.offset((32768 * 2) as isize) as *mut c_void,
        src.offset((32768 * 2) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft32768_ns_float_c(
        s,
        dst.offset((32768 * 3) as isize) as *mut c_void,
        src.offset((32768 * 3) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft_sr_combine_float_c(dst, cos_0, 32768 >> 1);
}
unsafe extern "C" fn ff_tx_fft262144_ns_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_262144_float.as_mut_ptr();
    ff_tx_fft131072_ns_float_c(s, dst as *mut c_void, src as *mut c_void, stride);
    ff_tx_fft65536_ns_float_c(
        s,
        dst.offset((65536 * 2) as isize) as *mut c_void,
        src.offset((65536 * 2) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft65536_ns_float_c(
        s,
        dst.offset((65536 * 3) as isize) as *mut c_void,
        src.offset((65536 * 3) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft_sr_combine_float_c(dst, cos_0, 65536 >> 1);
}
static mut ff_tx_fft262144_ns_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft262144_ns_float_c".as_ptr(),
    function: Some(
        ff_tx_fft262144_ns_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: ((1 as c_ulonglong) << 63
        | AV_TX_INPLACE as c_int as c_ulonglong
        | AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 61) as c_ulong,
    factors: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 262144,
    max_len: 262144,
    init: Some(
        ff_tx_fft_sr_codelet_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

static mut ff_tx_fft524288_ns_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft524288_ns_float_c".as_ptr(),
    function: Some(
        ff_tx_fft524288_ns_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: ((1 as c_ulonglong) << 63
        | AV_TX_INPLACE as c_int as c_ulonglong
        | AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 61) as c_ulong,
    factors: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 524288,
    max_len: 524288,
    init: Some(
        ff_tx_fft_sr_codelet_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

unsafe extern "C" fn ff_tx_fft524288_ns_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_524288_float.as_mut_ptr();
    ff_tx_fft262144_ns_float_c(s, dst as *mut c_void, src as *mut c_void, stride);
    ff_tx_fft131072_ns_float_c(
        s,
        dst.offset((131072 * 2) as isize) as *mut c_void,
        src.offset((131072 * 2) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft131072_ns_float_c(
        s,
        dst.offset((131072 * 3) as isize) as *mut c_void,
        src.offset((131072 * 3) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft_sr_combine_float_c(dst, cos_0, 131072 >> 1);
}
unsafe extern "C" fn ff_tx_fft1048576_ns_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_1048576_float.as_mut_ptr();
    ff_tx_fft524288_ns_float_c(s, dst as *mut c_void, src as *mut c_void, stride);
    ff_tx_fft262144_ns_float_c(
        s,
        dst.offset((262144 * 2) as isize) as *mut c_void,
        src.offset((262144 * 2) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft262144_ns_float_c(
        s,
        dst.offset((262144 * 3) as isize) as *mut c_void,
        src.offset((262144 * 3) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft_sr_combine_float_c(dst, cos_0, 262144 >> 1);
}
static mut ff_tx_fft1048576_ns_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft1048576_ns_float_c".as_ptr(),
    function: Some(
        ff_tx_fft1048576_ns_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: ((1 as c_ulonglong) << 63
        | AV_TX_INPLACE as c_int as c_ulonglong
        | AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 61) as c_ulong,
    factors: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 1048576,
    max_len: 1048576,
    init: Some(
        ff_tx_fft_sr_codelet_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

unsafe extern "C" fn ff_tx_fft2097152_ns_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let cos_0: *const TXSample = ff_tx_tab_2097152_float.as_mut_ptr();
    ff_tx_fft1048576_ns_float_c(s, dst as *mut c_void, src as *mut c_void, stride);
    ff_tx_fft524288_ns_float_c(
        s,
        dst.offset((524288 * 2) as isize) as *mut c_void,
        src.offset((524288 * 2) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft524288_ns_float_c(
        s,
        dst.offset((524288 * 3) as isize) as *mut c_void,
        src.offset((524288 * 3) as isize) as *mut c_void,
        stride,
    );
    ff_tx_fft_sr_combine_float_c(dst, cos_0, 524288 >> 1);
}
static mut ff_tx_fft2097152_ns_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft2097152_ns_float_c".as_ptr(),
    function: Some(
        ff_tx_fft2097152_ns_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: ((1 as c_ulonglong) << 63
        | AV_TX_INPLACE as c_int as c_ulonglong
        | AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 61) as c_ulong,
    factors: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 2097152,
    max_len: 2097152,
    init: Some(
        ff_tx_fft_sr_codelet_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

#[cold]
unsafe extern "C" fn ff_tx_fft_init_float_c(
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
    flags = (flags as c_ulonglong & !((1 as c_ulonglong) << 63)) as c_ulong;
    flags |= AV_TX_INPLACE as c_int as c_ulong;
    flags = (flags as c_ulonglong | (1 as c_ulonglong) << 61) as c_ulong;
    ret = ff_tx_init_subtx(s, AV_TX_FLOAT_FFT, flags, &mut sub_opts, len, inv, scale);
    if ret != 0 {
        return ret;
    }
    if is_inplace != 0 && {
        ret = ff_tx_gen_inplace_map(s, len);
        ret != 0
    } {
        return ret;
    }
    0
}
#[cold]
unsafe extern "C" fn ff_tx_fft_inplace_small_init_float_c(
    s: *mut AVTXContext,
    cd: *const FFTXCodelet,
    mut flags: c_ulong,
    opts: *mut FFTXCodeletOptions,
    len: c_int,
    inv: c_int,
    scale: *const c_void,
) -> c_int {
    (*s).tmp = AVTXNum {
        float: av_malloc((len as c_ulong).wrapping_mul(size_of::<TXComplex>() as c_ulong))
            as *mut TXComplex,
    };
    if ((*s).tmp).float.is_null() {
        return -12;
    }
    flags &= !(AV_TX_INPLACE as c_int) as c_ulong;
    ff_tx_fft_init_float_c(s, cd, flags, opts, len, inv, scale)
}
unsafe extern "C" fn ff_tx_fft_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst1: *mut TXComplex = (if (*s).flags & AV_TX_INPLACE as c_int as c_ulong != 0 {
        (*s).tmp.float as *mut c_void
    } else {
        _dst
    }) as *mut TXComplex;
    let dst2: *mut TXComplex = _dst as *mut TXComplex;
    let map: *mut c_int = (*((*s).sub).offset(0)).map;
    let len: c_int = (*s).len;
    let mut i: c_int = 0;
    while i < len {
        *dst1.offset(i as isize) = *src.offset(*map.offset(i as isize) as isize);
        i += 1;
        i;
    }
    ((*s).fn_0[0]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0),
        dst2 as *mut c_void,
        dst1 as *mut c_void,
        stride,
    );
}
unsafe extern "C" fn ff_tx_fft_inplace_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let mut tmp: TXComplex = TXComplex { re: 0., im: 0. };
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
    ((*s).fn_0[0]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0),
        dst as *mut c_void,
        src as *mut c_void,
        stride,
    );
}
static mut ff_tx_fft_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft_float_c".as_ptr(),
    function: Some(
        ff_tx_fft_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: (AV_TX_UNALIGNED as c_int as c_ulonglong | (1 as c_ulonglong) << 63) as c_ulong,
    factors: [-1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 2,
    max_len: -1,
    init: Some(
        ff_tx_fft_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

static mut ff_tx_fft_inplace_small_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft_inplace_small_float_c".as_ptr(),
    function: Some(
        ff_tx_fft_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: (AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 63
        | AV_TX_INPLACE as c_int as c_ulonglong) as c_ulong,
    factors: [-1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 2,
    max_len: 65536,
    init: Some(
        ff_tx_fft_inplace_small_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int - 256,
};

static mut ff_tx_fft_inplace_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft_inplace_float_c".as_ptr(),
    function: Some(
        ff_tx_fft_inplace_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: (AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 63
        | AV_TX_INPLACE as c_int as c_ulonglong) as c_ulong,
    factors: [-1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 2,
    max_len: -1,
    init: Some(
        ff_tx_fft_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int - 512,
};

#[cold]
unsafe extern "C" fn ff_tx_fft_init_naive_small_float_c(
    s: *mut AVTXContext,
    _cd: *const FFTXCodelet,
    _flags: c_ulong,
    _opts: *mut FFTXCodeletOptions,
    len: c_int,
    _inv: c_int,
    _scale: *const c_void,
) -> c_int {
    let phase: c_double = if (*s).inv != 0 {
        2. * PI / len as c_double
    } else {
        -2. * PI / len as c_double
    };
    (*s).exp = AVTXNum {
        float: av_malloc(((len * len) as c_ulong).wrapping_mul(size_of::<TXComplex>() as c_ulong))
            as *mut TXComplex,
    };
    if ((*s).exp).float.is_null() {
        return -12;
    }
    let mut i: c_int = 0;
    while i < len {
        let mut j: c_int = 0;
        while j < len {
            let factor: c_double = phase * i as c_double * j as c_double;
            *((*s).exp).float.offset((i * j) as isize) = {
                AVComplexFloat {
                    re: cos(factor) as c_float,
                    im: sin(factor) as c_float,
                }
            };
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    0
}
unsafe extern "C" fn ff_tx_fft_naive_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let src: *mut TXComplex = _src as *mut TXComplex;
    let dst: *mut TXComplex = _dst as *mut TXComplex;
    let n: c_int = (*s).len;
    let phase: c_double = if (*s).inv != 0 {
        2. * PI / n as c_double
    } else {
        -2. * PI / n as c_double
    };
    stride = (stride as c_ulong).wrapping_div(size_of::<TXComplex>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: c_int = 0;
    while i < n {
        let mut tmp: TXComplex = { AVComplexFloat { re: 0., im: 0. } };
        let mut j: c_int = 0;
        while j < n {
            let factor: c_double = phase * i as c_double * j as c_double;
            let mult: TXComplex = {
                AVComplexFloat {
                    re: cos(factor) as c_float,
                    im: sin(factor) as c_float,
                }
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
        *dst.offset((i as c_long * stride) as isize) = tmp;
        i += 1;
        i;
    }
}
unsafe extern "C" fn ff_tx_fft_naive_small_float_c(
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
    let mut i: c_int = 0;
    while i < n {
        let mut tmp: TXComplex = { AVComplexFloat { re: 0., im: 0. } };
        let mut j: c_int = 0;
        while j < n {
            let mut res: TXComplex = TXComplex { re: 0., im: 0. };
            let mult: TXComplex = *((*s).exp).float.offset((i * j) as isize);
            res.re =
                (*src.offset(j as isize)).re * mult.re - (*src.offset(j as isize)).im * mult.im;
            res.im =
                (*src.offset(j as isize)).re * mult.im + (*src.offset(j as isize)).im * mult.re;
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
static mut ff_tx_fft_naive_small_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft_naive_small_float_c".as_ptr(),
    function: Some(
        ff_tx_fft_naive_small_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: (AV_TX_UNALIGNED as c_int as c_ulonglong | (1 as c_ulonglong) << 63) as c_ulong,
    factors: [-1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 2,
    max_len: 1024,
    init: Some(
        ff_tx_fft_init_naive_small_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_MIN as c_int / 2,
};

static mut ff_tx_fft_naive_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft_naive_float_c".as_ptr(),
    function: Some(
        ff_tx_fft_naive_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: (AV_TX_UNALIGNED as c_int as c_ulonglong | (1 as c_ulonglong) << 63) as c_ulong,
    factors: [-1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 1,
    min_len: 2,
    max_len: -1,
    init: None,
    uninit: None,
    cpu_flags: 0,
    prio: FF_TX_PRIO_MIN as c_int,
};

#[cold]
unsafe extern "C" fn ff_tx_fft_pfa_init_float_c(
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
    let ps: c_int = (flags as c_ulonglong & (1 as c_ulonglong) << 61) as c_int;
    let mut sub_opts: FFTXCodeletOptions = {
        FFTXCodeletOptions {
            map_dir: FF_TX_MAP_GATHER,
        }
    };
    let mut extra_tmp_len: c_ulong = 0 as c_ulong;
    let mut len_list: [c_int; 512] = [0; 512];
    ret = ff_tx_decompose_length(len_list.as_mut_ptr(), AV_TX_FLOAT_FFT, len, inv);
    if ret < 0 {
        return ret;
    }
    let mut current_block_30: u64;
    let mut i: c_int = 0;
    's_17: while i < ret {
        let mut len1: c_int = len_list[i as usize];
        let mut len2: c_int = len / len1;
        if len2 & len2 - 1 != 0 {
            std::mem::swap(&mut len2, &mut len1);
        }
        ff_tx_clear_ctx(s);
        sub_opts.map_dir = FF_TX_MAP_GATHER;
        flags &= !(AV_TX_INPLACE as c_int) as c_ulong;
        flags = (flags as c_ulonglong | (1 as c_ulonglong) << 63) as c_ulong;
        flags = (flags as c_ulonglong | (1 as c_ulonglong) << 61) as c_ulong;
        ret = ff_tx_init_subtx(s, AV_TX_FLOAT_FFT, flags, &mut sub_opts, len1, inv, scale);
        if ret == -12 {
            return ret;
        } else {
            if ret < 0 {
                flags = (flags as c_ulonglong & !((1 as c_ulonglong) << 61)) as c_ulong;
                ret = ff_tx_init_subtx(s, AV_TX_FLOAT_FFT, flags, &mut sub_opts, len1, inv, scale);
                if ret == -12 {
                    return ret;
                } else if ret < 0 {
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
                    flags = (flags as c_ulonglong | (1 as c_ulonglong) << 61) as c_ulong;
                    loop {
                        flags = (flags as c_ulonglong & !((1 as c_ulonglong) << 63)) as c_ulong;
                        flags |= AV_TX_INPLACE as c_int as c_ulong;
                        ret = ff_tx_init_subtx(
                            s,
                            AV_TX_FLOAT_FFT,
                            flags,
                            &mut sub_opts,
                            len2,
                            inv,
                            scale,
                        );
                        if ret == -12 {
                            return ret;
                        } else {
                            if !(ret < 0) {
                                break 's_17;
                            }
                            flags = (flags as c_ulonglong | (1 as c_ulonglong) << 63) as c_ulong;
                            flags &= !(AV_TX_INPLACE as c_int) as c_ulong;
                            ret = ff_tx_init_subtx(
                                s,
                                AV_TX_FLOAT_FFT,
                                flags,
                                &mut sub_opts,
                                len2,
                                inv,
                                scale,
                            );
                            if ret == -12 {
                                return ret;
                            } else {
                                if !(ret < 0) {
                                    break 's_17;
                                }
                                if !(flags as c_ulonglong & (1 as c_ulonglong) << 61 != 0) {
                                    break;
                                }
                                flags =
                                    (flags as c_ulonglong & !((1 as c_ulonglong) << 61)) as c_ulong;
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
    if ret < 0 {
        return ret;
    }
    ret = ff_tx_gen_compound_mapping(
        s,
        opts,
        0,
        (*((*s).sub).offset(0)).len,
        (*((*s).sub).offset(1)).len,
    );
    if ret != 0 {
        return ret;
    }
    (*s).tmp = AVTXNum {
        float: av_malloc((len as c_ulong).wrapping_mul(size_of::<TXComplex>() as c_ulong))
            as *mut TXComplex,
    };
    if ((*s).tmp).float.is_null() {
        return -12;
    }
    tmp = (*s).tmp.float as *mut c_int;
    let mut k: c_int = 0;
    while k < len {
        memcpy(
            tmp as *mut c_void,
            &mut *((*s).map).offset(k as isize) as *mut c_int as *const c_void,
            ((*((*s).sub).offset(0)).len as c_ulong).wrapping_mul(size_of::<c_int>() as c_ulong),
        );
        let mut i_0: c_int = 0;
        while i_0 < (*((*s).sub).offset(0)).len {
            *((*s).map).offset((k + i_0) as isize) =
                *tmp.offset(*((*((*s).sub).offset(0)).map).offset(i_0 as isize) as isize);
            i_0 += 1;
            i_0;
        }
        k += (*((*s).sub).offset(0)).len;
    }
    if (*((*s).sub).offset(1)).flags & AV_TX_INPLACE as c_int as c_ulong == 0 {
        extra_tmp_len = len as c_ulong;
    } else if ps == 0 {
        extra_tmp_len = (*((*s).sub).offset(0)).len as c_ulong;
    }
    if extra_tmp_len != 0 && {
        (*s).exp = AVTXNum {
            float: av_malloc(extra_tmp_len.wrapping_mul(size_of::<TXComplex>() as c_ulong))
                as *mut TXComplex,
        };
        ((*s).exp).float.is_null()
    } {
        return -12;
    }
    0
}
unsafe extern "C" fn ff_tx_fft_pfa_float_c(
    s: *mut AVTXContext,
    mut _out: *mut c_void,
    mut _in: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let n: c_int = (*((*s).sub).offset(0)).len;
    let m: c_int = (*((*s).sub).offset(1)).len;
    let l: c_int = (*s).len;
    let in_map: *const c_int = (*s).map;
    let out_map: *const c_int = in_map.offset(l as isize);
    let sub_map: *const c_int = (*((*s).sub).offset(1)).map;
    let tmp1: *mut TXComplex =
        if (*((*s).sub).offset(1)).flags & AV_TX_INPLACE as c_int as c_ulong != 0 {
            (*s).tmp.float
        } else {
            (*s).exp.float
        };
    let in_0: *mut TXComplex = _in as *mut TXComplex;
    let out: *mut TXComplex = _out as *mut TXComplex;
    stride = (stride as c_ulong).wrapping_div(size_of::<TXComplex>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: c_int = 0;
    while i < m {
        let mut j: c_int = 0;
        while j < n {
            *((*s).exp).float.offset(j as isize) =
                *in_0.offset(*in_map.offset((i * n + j) as isize) as isize);
            j += 1;
            j;
        }
        ((*s).fn_0[0]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0),
            &mut *((*s).tmp)
                .float
                .offset(*sub_map.offset(i as isize) as isize) as *mut TXComplex
                as *mut c_void,
            (*s).exp.float as *mut c_void,
            (m as c_ulong).wrapping_mul(size_of::<TXComplex>() as c_ulong) as ptrdiff_t,
        );
        i += 1;
        i;
    }
    let mut i_0: c_int = 0;
    while i_0 < n {
        ((*s).fn_0[1]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(1),
            &mut *tmp1.offset((m * i_0) as isize) as *mut TXComplex as *mut c_void,
            &mut *((*s).tmp).float.offset((m * i_0) as isize) as *mut TXComplex as *mut c_void,
            size_of::<TXComplex>() as c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: c_int = 0;
    while i_1 < l {
        *out.offset((i_1 as c_long * stride) as isize) =
            *tmp1.offset(*out_map.offset(i_1 as isize) as isize);
        i_1 += 1;
        i_1;
    }
}
unsafe extern "C" fn ff_tx_fft_pfa_ns_float_c(
    s: *mut AVTXContext,
    mut _out: *mut c_void,
    mut _in: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let n: c_int = (*((*s).sub).offset(0)).len;
    let m: c_int = (*((*s).sub).offset(1)).len;
    let l: c_int = (*s).len;
    let in_map: *const c_int = (*s).map;
    let out_map: *const c_int = in_map.offset(l as isize);
    let sub_map: *const c_int = (*((*s).sub).offset(1)).map;
    let tmp1: *mut TXComplex =
        if (*((*s).sub).offset(1)).flags & AV_TX_INPLACE as c_int as c_ulong != 0 {
            (*s).tmp.float
        } else {
            (*s).exp.float
        };
    let in_0: *mut TXComplex = _in as *mut TXComplex;
    let out: *mut TXComplex = _out as *mut TXComplex;
    stride = (stride as c_ulong).wrapping_div(size_of::<TXComplex>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: c_int = 0;
    while i < m {
        ((*s).fn_0[0]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0),
            &mut *((*s).tmp)
                .float
                .offset(*sub_map.offset(i as isize) as isize) as *mut TXComplex
                as *mut c_void,
            &mut *in_0.offset((i * n) as isize) as *mut TXComplex as *mut c_void,
            (m as c_ulong).wrapping_mul(size_of::<TXComplex>() as c_ulong) as ptrdiff_t,
        );
        i += 1;
        i;
    }
    let mut i_0: c_int = 0;
    while i_0 < n {
        ((*s).fn_0[1]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(1),
            &mut *tmp1.offset((m * i_0) as isize) as *mut TXComplex as *mut c_void,
            &mut *((*s).tmp).float.offset((m * i_0) as isize) as *mut TXComplex as *mut c_void,
            size_of::<TXComplex>() as c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: c_int = 0;
    while i_1 < l {
        *out.offset((i_1 as c_long * stride) as isize) =
            *tmp1.offset(*out_map.offset(i_1 as isize) as isize);
        i_1 += 1;
        i_1;
    }
}
static mut ff_tx_fft_pfa_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft_pfa_float_c".as_ptr(),
    function: Some(
        ff_tx_fft_pfa_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: ((AV_TX_UNALIGNED as c_int | AV_TX_INPLACE as c_int) as c_ulonglong
        | (1 as c_ulonglong) << 63) as c_ulong,
    factors: [7, 5, 3, 2, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 2,
    min_len: 2 * 3,
    max_len: -1,
    init: Some(
        ff_tx_fft_pfa_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

static mut ff_tx_fft_pfa_ns_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"fft_pfa_ns_float_c".as_ptr(),
    function: Some(
        ff_tx_fft_pfa_ns_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_FFT,
    flags: ((AV_TX_UNALIGNED as c_int | AV_TX_INPLACE as c_int) as c_ulonglong
        | (1 as c_ulonglong) << 63
        | (1 as c_ulonglong) << 61) as c_ulong,
    factors: [7, 5, 3, 2, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 2,
    min_len: 2 * 3,
    max_len: -1,
    init: Some(
        ff_tx_fft_pfa_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

#[cold]
unsafe extern "C" fn ff_tx_mdct_naive_init_float_c(
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
    0
}
unsafe extern "C" fn ff_tx_mdct_naive_fwd_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let src: *mut TXSample = _src as *mut TXSample;
    let dst: *mut TXSample = _dst as *mut TXSample;
    let scale: c_double = (*s).scale_d;
    let len: c_int = (*s).len;
    let phase: c_double = PI / (4. * len as c_double);
    stride = (stride as c_ulong).wrapping_div(size_of::<TXSample>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: c_int = 0;
    while i < len {
        let mut sum: c_double = 0.;
        let mut j: c_int = 0;
        while j < len * 2 {
            let a: c_int = (2 * j + 1 + len) * (2 * i + 1);
            sum += *src.offset(j as isize) as c_double * cos(a as c_double * phase);
            j += 1;
            j;
        }
        *dst.offset((i as c_long * stride) as isize) = (sum * scale) as TXSample;
        i += 1;
        i;
    }
}
unsafe extern "C" fn ff_tx_mdct_naive_inv_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let src: *mut TXSample = _src as *mut TXSample;
    let dst: *mut TXSample = _dst as *mut TXSample;
    let scale: c_double = (*s).scale_d;
    let len: c_int = (*s).len >> 1;
    let len2: c_int = len * 2;
    let phase: c_double = PI / (4. * len2 as c_double);
    stride = (stride as c_ulong).wrapping_div(size_of::<TXSample>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: c_int = 0;
    while i < len {
        let mut sum_d: c_double = 0.;
        let mut sum_u: c_double = 0.;
        let i_d: c_double = phase * (4 * len - 2 * i - 1) as c_double;
        let i_u: c_double = phase * (3 * len2 + 2 * i + 1) as c_double;
        let mut j: c_int = 0;
        while j < len2 {
            let a: c_double = (2 * j + 1) as c_double;
            let a_d: c_double = cos(a * i_d);
            let a_u: c_double = cos(a * i_u);
            let val: c_double = *src.offset((j as c_long * stride) as isize) as c_double;
            sum_d += a_d * val;
            sum_u += a_u * val;
            j += 1;
            j;
        }
        *dst.offset((i + 0) as isize) = (sum_d * scale) as TXSample;
        *dst.offset((i + len) as isize) = (-sum_u * scale) as TXSample;
        i += 1;
        i;
    }
}
static mut ff_tx_mdct_naive_fwd_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"mdct_naive_fwd_float_c".as_ptr(),
    function: Some(
        ff_tx_mdct_naive_fwd_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_MDCT,
    flags: (AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 63
        | (1 as c_ulonglong) << 59) as c_ulong,
    factors: [2, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 2,
    min_len: 2,
    max_len: -1,
    init: Some(
        ff_tx_mdct_naive_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_MIN as c_int,
};

static mut ff_tx_mdct_naive_inv_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"mdct_naive_inv_float_c".as_ptr(),
    function: Some(
        ff_tx_mdct_naive_inv_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_MDCT,
    flags: (AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 63
        | (1 as c_ulonglong) << 60) as c_ulong,
    factors: [2, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 2,
    min_len: 2,
    max_len: -1,
    init: Some(
        ff_tx_mdct_naive_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_MIN as c_int,
};

#[cold]
unsafe extern "C" fn ff_tx_mdct_init_float_c(
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
    flags = (flags as c_ulonglong & !((1 as c_ulonglong) << 63)) as c_ulong;
    flags |= AV_TX_INPLACE as c_int as c_ulong;
    flags = (flags as c_ulonglong | (1 as c_ulonglong) << 61) as c_ulong;
    ret = ff_tx_init_subtx(
        s,
        AV_TX_FLOAT_FFT,
        flags,
        &mut sub_opts,
        len >> 1,
        inv,
        scale,
    );
    if ret != 0 {
        flags = (flags as c_ulonglong & !((1 as c_ulonglong) << 61)) as c_ulong;
        ret = ff_tx_init_subtx(
            s,
            AV_TX_FLOAT_FFT,
            flags,
            &mut sub_opts,
            len >> 1,
            inv,
            scale,
        );
        if ret != 0 {
            return ret;
        }
    }
    (*s).map = av_malloc(((len >> 1) as c_ulong).wrapping_mul(size_of::<c_int>() as c_ulong))
        as *mut c_int;
    if ((*s).map).is_null() {
        return -12;
    }
    if (*((*s).sub).offset(0)).flags as c_ulonglong & (1 as c_ulonglong) << 61 != 0 {
        memcpy(
            (*s).map as *mut c_void,
            (*(*s).sub).map as *const c_void,
            ((len >> 1) as c_ulong).wrapping_mul(size_of::<c_int>() as c_ulong),
        );
    } else {
        let mut i: c_int = 0;
        while i < len >> 1 {
            *((*s).map).offset(i as isize) = i;
            i += 1;
            i;
        }
    }
    ret = ff_tx_mdct_gen_exp_float(
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
        let mut i_0: c_int = 0;
        while i_0 < (*s).len >> 1 {
            *((*s).map).offset(i_0 as isize) <<= 1;
            i_0 += 1;
            i_0;
        }
    }
    0
}
unsafe extern "C" fn ff_tx_mdct_fwd_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let src: *mut TXSample = _src as *mut TXSample;
    let dst: *mut TXSample = _dst as *mut TXSample;
    let exp: *mut TXComplex = (*s).exp.float;
    let mut tmp: TXComplex = TXComplex { re: 0., im: 0. };
    let z: *mut TXComplex = _dst as *mut TXComplex;
    let len2: c_int = (*s).len >> 1;
    let len4: c_int = (*s).len >> 2;
    let len3: c_int = len2 * 3;
    let sub_map: *const c_int = (*s).map;
    stride = (stride as c_ulong).wrapping_div(size_of::<TXSample>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: c_int = 0;
    while i < len2 {
        let k: c_int = 2 * i;
        let idx: c_int = *sub_map.offset(i as isize);
        if k < len2 {
            tmp.re = -*src.offset((len2 + k) as isize) + *src.offset((1 * len2 - 1 - k) as isize);
            tmp.im = -*src.offset((len3 + k) as isize) + -*src.offset((1 * len3 - 1 - k) as isize);
        } else {
            tmp.re = -*src.offset((len2 + k) as isize) + -*src.offset((5 * len2 - 1 - k) as isize);
            tmp.im = *src.offset((-len2 + k) as isize) + -*src.offset((1 * len3 - 1 - k) as isize);
        }
        (*z.offset(idx as isize)).im =
            tmp.re * (*exp.offset(i as isize)).re - tmp.im * (*exp.offset(i as isize)).im;
        (*z.offset(idx as isize)).re =
            tmp.re * (*exp.offset(i as isize)).im + tmp.im * (*exp.offset(i as isize)).re;
        i += 1;
        i;
    }
    ((*s).fn_0[0]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0),
        z as *mut c_void,
        z as *mut c_void,
        size_of::<TXComplex>() as c_ulong as ptrdiff_t,
    );
    let mut i_0: c_int = 0;
    while i_0 < len4 {
        let i0: c_int = len4 + i_0;
        let i1: c_int = len4 - i_0 - 1;
        let src1: TXComplex = {
            AVComplexFloat {
                re: (*z.offset(i1 as isize)).re,
                im: (*z.offset(i1 as isize)).im,
            }
        };
        let src0: TXComplex = {
            AVComplexFloat {
                re: (*z.offset(i0 as isize)).re,
                im: (*z.offset(i0 as isize)).im,
            }
        };
        *dst.offset(((2 * i1) as c_long * stride + stride) as isize) =
            src0.re * (*exp.offset(i0 as isize)).im - src0.im * (*exp.offset(i0 as isize)).re;
        *dst.offset(((2 * i0) as c_long * stride) as isize) =
            src0.re * (*exp.offset(i0 as isize)).re + src0.im * (*exp.offset(i0 as isize)).im;
        *dst.offset(((2 * i0) as c_long * stride + stride) as isize) =
            src1.re * (*exp.offset(i1 as isize)).im - src1.im * (*exp.offset(i1 as isize)).re;
        *dst.offset(((2 * i1) as c_long * stride) as isize) =
            src1.re * (*exp.offset(i1 as isize)).re + src1.im * (*exp.offset(i1 as isize)).im;
        i_0 += 1;
        i_0;
    }
}
unsafe extern "C" fn ff_tx_mdct_inv_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let z: *mut TXComplex = _dst as *mut TXComplex;
    let mut exp: *mut TXComplex = (*s).exp.float;
    let src: *const TXSample = _src as *const TXSample;
    let mut in1: *const TXSample = std::ptr::null::<TXSample>();
    let mut in2: *const TXSample = std::ptr::null::<TXSample>();
    let len2: c_int = (*s).len >> 1;
    let len4: c_int = (*s).len >> 2;
    let sub_map: *const c_int = (*s).map;
    stride = (stride as c_ulong).wrapping_div(size_of::<TXSample>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    in1 = src;
    in2 = src.offset(((len2 * 2 - 1) as c_long * stride) as isize);
    let mut i: c_int = 0;
    while i < len2 {
        let k: c_int = *sub_map.offset(i as isize);
        let tmp: TXComplex = {
            AVComplexFloat {
                re: *in2.offset((-k as c_long * stride) as isize),
                im: *in1.offset((k as c_long * stride) as isize),
            }
        };
        (*z.offset(i as isize)).re =
            tmp.re * (*exp.offset(i as isize)).re - tmp.im * (*exp.offset(i as isize)).im;
        (*z.offset(i as isize)).im =
            tmp.re * (*exp.offset(i as isize)).im + tmp.im * (*exp.offset(i as isize)).re;
        i += 1;
        i;
    }
    ((*s).fn_0[0]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0),
        z as *mut c_void,
        z as *mut c_void,
        size_of::<TXComplex>() as c_ulong as ptrdiff_t,
    );
    exp = exp.offset(len2 as isize);
    let mut i_0: c_int = 0;
    while i_0 < len4 {
        let i0: c_int = len4 + i_0;
        let i1: c_int = len4 - i_0 - 1;
        let src1: TXComplex = {
            AVComplexFloat {
                re: (*z.offset(i1 as isize)).im,
                im: (*z.offset(i1 as isize)).re,
            }
        };
        let src0: TXComplex = {
            AVComplexFloat {
                re: (*z.offset(i0 as isize)).im,
                im: (*z.offset(i0 as isize)).re,
            }
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
static mut ff_tx_mdct_fwd_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"mdct_fwd_float_c".as_ptr(),
    function: Some(
        ff_tx_mdct_fwd_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_MDCT,
    flags: (AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 63
        | (1 as c_ulonglong) << 59) as c_ulong,
    factors: [2, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 2,
    min_len: 2,
    max_len: -1,
    init: Some(
        ff_tx_mdct_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

static mut ff_tx_mdct_inv_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"mdct_inv_float_c".as_ptr(),
    function: Some(
        ff_tx_mdct_inv_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_MDCT,
    flags: (AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 63
        | (1 as c_ulonglong) << 60) as c_ulong,
    factors: [2, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 2,
    min_len: 2,
    max_len: -1,
    init: Some(
        ff_tx_mdct_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

#[cold]
unsafe extern "C" fn ff_tx_mdct_inv_full_init_float_c(
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
        AV_TX_FLOAT_MDCT,
        flags,
        std::ptr::null_mut::<FFTXCodeletOptions>(),
        len,
        1,
        scale,
    );
    if ret != 0 {
        return ret;
    }
    0
}
unsafe extern "C" fn ff_tx_mdct_inv_full_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let len: c_int = (*s).len << 1;
    let len2: c_int = len >> 1;
    let len4: c_int = len >> 2;
    let dst: *mut TXSample = _dst as *mut TXSample;
    ((*s).fn_0[0]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0),
        dst.offset(len4 as isize) as *mut c_void,
        _src,
        stride,
    );
    stride = (stride as c_ulong).wrapping_div(size_of::<TXSample>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: c_int = 0;
    while i < len4 {
        *dst.offset((i as c_long * stride) as isize) =
            -*dst.offset(((len2 - i - 1) as c_long * stride) as isize);
        *dst.offset(((len - i - 1) as c_long * stride) as isize) =
            *dst.offset(((len2 + i + 0) as c_long * stride) as isize);
        i += 1;
        i;
    }
}
static mut ff_tx_mdct_inv_full_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"mdct_inv_full_float_c".as_ptr(),
    function: Some(
        ff_tx_mdct_inv_full_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_MDCT,
    flags: ((AV_TX_UNALIGNED as c_int | AV_TX_INPLACE as c_int) as c_ulonglong
        | (1 as c_ulonglong) << 63
        | AV_TX_FULL_IMDCT as c_int as c_ulonglong) as c_ulong,
    factors: [2, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 2,
    min_len: 2,
    max_len: -1,
    init: Some(
        ff_tx_mdct_inv_full_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

#[cold]
unsafe extern "C" fn ff_tx_mdct_pfa_init_float_c(
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
    len >>= 1;
    sub_len = len / (*cd).factors[0];
    (*s).scale_d = *(scale as *mut c_float) as c_double;
    (*s).scale_f = (*s).scale_d as c_float;
    flags = (flags as c_ulonglong & !((1 as c_ulonglong) << 63)) as c_ulong;
    flags |= AV_TX_INPLACE as c_int as c_ulong;
    flags = (flags as c_ulonglong | (1 as c_ulonglong) << 61) as c_ulong;
    ret = ff_tx_init_subtx(
        s,
        AV_TX_FLOAT_FFT,
        flags,
        &mut sub_opts,
        sub_len,
        inv,
        scale,
    );
    if ret != 0 {
        return ret;
    }
    ret = ff_tx_gen_compound_mapping(s, opts, (*s).inv, (*cd).factors[0], sub_len);
    if ret != 0 {
        return ret;
    }
    if (*cd).factors[0] == 15 {
        let mut mtmp: [c_int; 15] = [0; 15];
        let mut k: c_int = 0;
        while k < len {
            memcpy(
                mtmp.as_mut_ptr() as *mut c_void,
                &mut *((*s).map).offset(k as isize) as *mut c_int as *const c_void,
                ((3 * 5) as c_ulong).wrapping_mul(size_of::<c_int>() as c_ulong),
            );
            let mut m: c_int = 0;
            while m < 5 {
                let mut n: c_int = 0;
                while n < 3 {
                    *((*s).map).offset((k + m * 3 + n) as isize) =
                        mtmp[((m * 3 + n * 5) % (3 * 5)) as usize];
                    n += 1;
                    n;
                }
                m += 1;
                m;
            }
            k += 3 * 5;
        }
    }
    ret = ff_tx_mdct_gen_exp_float(
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
    let mut i: c_int = 0;
    while i < len {
        *((*s).map).offset(i as isize) <<= 1;
        i += 1;
        i;
    }
    (*s).tmp = AVTXNum {
        float: av_malloc((len as c_ulong).wrapping_mul(size_of::<TXComplex>() as c_ulong))
            as *mut TXComplex,
    };
    if ((*s).tmp).float.is_null() {
        return -12;
    }
    ff_tx_init_tabs_float(len / sub_len);
    0
}
static mut ff_tx_mdct_pfa_3xM_inv_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"mdct_pfa_3xM_inv_float_c".as_ptr(),
    function: Some(
        ff_tx_mdct_pfa_3xM_inv_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_MDCT,
    flags: (AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 63
        | (1 as c_ulonglong) << 60) as c_ulong,
    factors: [3, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 2,
    min_len: 3 * 2,
    max_len: -1,
    init: Some(
        ff_tx_mdct_pfa_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

unsafe extern "C" fn ff_tx_mdct_pfa_3xM_inv_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let mut fft3in: [TXComplex; 3] = [TXComplex { re: 0., im: 0. }; 3];
    let z: *mut TXComplex = _dst as *mut TXComplex;
    let mut exp: *mut TXComplex = (*s).exp.float;
    let src: *const TXSample = _src as *const TXSample;
    let mut in1: *const TXSample = std::ptr::null::<TXSample>();
    let mut in2: *const TXSample = std::ptr::null::<TXSample>();
    let len4: c_int = (*s).len >> 2;
    let len2: c_int = (*s).len >> 1;
    let m: c_int = (*(*s).sub).len;
    let mut in_map: *const c_int = (*s).map;
    let out_map: *const c_int = in_map.offset((3 * m) as isize);
    let mut sub_map: *const c_int = (*(*s).sub).map;
    stride = (stride as c_ulong).wrapping_div(size_of::<TXSample>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    in1 = src;
    in2 = src.offset(((3 * m * 2 - 1) as c_long * stride) as isize);
    let mut i: c_int = 0;
    while i < len2 {
        let mut j: c_int = 0;
        while j < 3 {
            let k: c_int = *in_map.offset(j as isize);
            let tmp: TXComplex = {
                AVComplexFloat {
                    re: *in2.offset((-k as c_long * stride) as isize),
                    im: *in1.offset((k as c_long * stride) as isize),
                }
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
            ((*s).tmp).float.offset(*fresh22 as isize),
            fft3in.as_mut_ptr(),
            m as ptrdiff_t,
        );
        exp = exp.offset(3);
        in_map = in_map.offset(3);
        i += 3;
    }
    let mut i_0: c_int = 0;
    while i_0 < 3 {
        ((*s).fn_0[0]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0),
            ((*s).tmp).float.offset((m * i_0) as isize) as *mut c_void,
            ((*s).tmp).float.offset((m * i_0) as isize) as *mut c_void,
            size_of::<TXComplex>() as c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: c_int = 0;
    while i_1 < len4 {
        let i0: c_int = len4 + i_1;
        let i1: c_int = len4 - i_1 - 1;
        let s0: c_int = *out_map.offset(i0 as isize);
        let s1: c_int = *out_map.offset(i1 as isize);
        let src1: TXComplex = {
            AVComplexFloat {
                re: (*((*s).tmp).float.offset(s1 as isize)).im,
                im: (*((*s).tmp).float.offset(s1 as isize)).re,
            }
        };
        let src0: TXComplex = {
            AVComplexFloat {
                re: (*((*s).tmp).float.offset(s0 as isize)).im,
                im: (*((*s).tmp).float.offset(s0 as isize)).re,
            }
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
static mut ff_tx_mdct_pfa_5xM_inv_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"mdct_pfa_5xM_inv_float_c".as_ptr(),
    function: Some(
        ff_tx_mdct_pfa_5xM_inv_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_MDCT,
    flags: (AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 63
        | (1 as c_ulonglong) << 60) as c_ulong,
    factors: [5, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 2,
    min_len: 5 * 2,
    max_len: -1,
    init: Some(
        ff_tx_mdct_pfa_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

unsafe extern "C" fn ff_tx_mdct_pfa_5xM_inv_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let mut fft5in: [TXComplex; 5] = [TXComplex { re: 0., im: 0. }; 5];
    let z: *mut TXComplex = _dst as *mut TXComplex;
    let mut exp: *mut TXComplex = (*s).exp.float;
    let src: *const TXSample = _src as *const TXSample;
    let mut in1: *const TXSample = std::ptr::null::<TXSample>();
    let mut in2: *const TXSample = std::ptr::null::<TXSample>();
    let len4: c_int = (*s).len >> 2;
    let len2: c_int = (*s).len >> 1;
    let m: c_int = (*(*s).sub).len;
    let mut in_map: *const c_int = (*s).map;
    let out_map: *const c_int = in_map.offset((5 * m) as isize);
    let mut sub_map: *const c_int = (*(*s).sub).map;
    stride = (stride as c_ulong).wrapping_div(size_of::<TXSample>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    in1 = src;
    in2 = src.offset(((5 * m * 2 - 1) as c_long * stride) as isize);
    let mut i: c_int = 0;
    while i < len2 {
        let mut j: c_int = 0;
        while j < 5 {
            let k: c_int = *in_map.offset(j as isize);
            let tmp: TXComplex = {
                AVComplexFloat {
                    re: *in2.offset((-k as c_long * stride) as isize),
                    im: *in1.offset((k as c_long * stride) as isize),
                }
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
            ((*s).tmp).float.offset(*fresh23 as isize),
            fft5in.as_mut_ptr(),
            m as ptrdiff_t,
        );
        exp = exp.offset(5);
        in_map = in_map.offset(5);
        i += 5;
    }
    let mut i_0: c_int = 0;
    while i_0 < 5 {
        ((*s).fn_0[0]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0),
            ((*s).tmp).float.offset((m * i_0) as isize) as *mut c_void,
            ((*s).tmp).float.offset((m * i_0) as isize) as *mut c_void,
            size_of::<TXComplex>() as c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: c_int = 0;
    while i_1 < len4 {
        let i0: c_int = len4 + i_1;
        let i1: c_int = len4 - i_1 - 1;
        let s0: c_int = *out_map.offset(i0 as isize);
        let s1: c_int = *out_map.offset(i1 as isize);
        let src1: TXComplex = {
            AVComplexFloat {
                re: (*((*s).tmp).float.offset(s1 as isize)).im,
                im: (*((*s).tmp).float.offset(s1 as isize)).re,
            }
        };
        let src0: TXComplex = {
            AVComplexFloat {
                re: (*((*s).tmp).float.offset(s0 as isize)).im,
                im: (*((*s).tmp).float.offset(s0 as isize)).re,
            }
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
unsafe extern "C" fn ff_tx_mdct_pfa_7xM_inv_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let mut fft7in: [TXComplex; 7] = [TXComplex { re: 0., im: 0. }; 7];
    let z: *mut TXComplex = _dst as *mut TXComplex;
    let mut exp: *mut TXComplex = (*s).exp.float;
    let src: *const TXSample = _src as *const TXSample;
    let mut in1: *const TXSample = std::ptr::null::<TXSample>();
    let mut in2: *const TXSample = std::ptr::null::<TXSample>();
    let len4: c_int = (*s).len >> 2;
    let len2: c_int = (*s).len >> 1;
    let m: c_int = (*(*s).sub).len;
    let mut in_map: *const c_int = (*s).map;
    let out_map: *const c_int = in_map.offset((7 * m) as isize);
    let mut sub_map: *const c_int = (*(*s).sub).map;
    stride = (stride as c_ulong).wrapping_div(size_of::<TXSample>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    in1 = src;
    in2 = src.offset(((7 * m * 2 - 1) as c_long * stride) as isize);
    let mut i: c_int = 0;
    while i < len2 {
        let mut j: c_int = 0;
        while j < 7 {
            let k: c_int = *in_map.offset(j as isize);
            let tmp: TXComplex = {
                AVComplexFloat {
                    re: *in2.offset((-k as c_long * stride) as isize),
                    im: *in1.offset((k as c_long * stride) as isize),
                }
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
            ((*s).tmp).float.offset(*fresh24 as isize),
            fft7in.as_mut_ptr(),
            m as ptrdiff_t,
        );
        exp = exp.offset(7);
        in_map = in_map.offset(7);
        i += 7;
    }
    let mut i_0: c_int = 0;
    while i_0 < 7 {
        ((*s).fn_0[0]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0),
            ((*s).tmp).float.offset((m * i_0) as isize) as *mut c_void,
            ((*s).tmp).float.offset((m * i_0) as isize) as *mut c_void,
            size_of::<TXComplex>() as c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: c_int = 0;
    while i_1 < len4 {
        let i0: c_int = len4 + i_1;
        let i1: c_int = len4 - i_1 - 1;
        let s0: c_int = *out_map.offset(i0 as isize);
        let s1: c_int = *out_map.offset(i1 as isize);
        let src1: TXComplex = {
            AVComplexFloat {
                re: (*((*s).tmp).float.offset(s1 as isize)).im,
                im: (*((*s).tmp).float.offset(s1 as isize)).re,
            }
        };
        let src0: TXComplex = {
            AVComplexFloat {
                re: (*((*s).tmp).float.offset(s0 as isize)).im,
                im: (*((*s).tmp).float.offset(s0 as isize)).re,
            }
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
static mut ff_tx_mdct_pfa_7xM_inv_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"mdct_pfa_7xM_inv_float_c".as_ptr(),
    function: Some(
        ff_tx_mdct_pfa_7xM_inv_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_MDCT,
    flags: (AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 63
        | (1 as c_ulonglong) << 60) as c_ulong,
    factors: [7, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 2,
    min_len: 7 * 2,
    max_len: -1,
    init: Some(
        ff_tx_mdct_pfa_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

static mut ff_tx_mdct_pfa_9xM_inv_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"mdct_pfa_9xM_inv_float_c".as_ptr(),
    function: Some(
        ff_tx_mdct_pfa_9xM_inv_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_MDCT,
    flags: (AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 63
        | (1 as c_ulonglong) << 60) as c_ulong,
    factors: [9, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 2,
    min_len: 9 * 2,
    max_len: -1,
    init: Some(
        ff_tx_mdct_pfa_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

unsafe extern "C" fn ff_tx_mdct_pfa_9xM_inv_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let mut fft9in: [TXComplex; 9] = [TXComplex { re: 0., im: 0. }; 9];
    let z: *mut TXComplex = _dst as *mut TXComplex;
    let mut exp: *mut TXComplex = (*s).exp.float;
    let src: *const TXSample = _src as *const TXSample;
    let mut in1: *const TXSample = std::ptr::null::<TXSample>();
    let mut in2: *const TXSample = std::ptr::null::<TXSample>();
    let len4: c_int = (*s).len >> 2;
    let len2: c_int = (*s).len >> 1;
    let m: c_int = (*(*s).sub).len;
    let mut in_map: *const c_int = (*s).map;
    let out_map: *const c_int = in_map.offset((9 * m) as isize);
    let mut sub_map: *const c_int = (*(*s).sub).map;
    stride = (stride as c_ulong).wrapping_div(size_of::<TXSample>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    in1 = src;
    in2 = src.offset(((9 * m * 2 - 1) as c_long * stride) as isize);
    let mut i: c_int = 0;
    while i < len2 {
        let mut j: c_int = 0;
        while j < 9 {
            let k: c_int = *in_map.offset(j as isize);
            let tmp: TXComplex = {
                AVComplexFloat {
                    re: *in2.offset((-k as c_long * stride) as isize),
                    im: *in1.offset((k as c_long * stride) as isize),
                }
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
            ((*s).tmp).float.offset(*fresh25 as isize),
            fft9in.as_mut_ptr(),
            m as ptrdiff_t,
        );
        exp = exp.offset(9);
        in_map = in_map.offset(9);
        i += 9;
    }
    let mut i_0: c_int = 0;
    while i_0 < 9 {
        ((*s).fn_0[0]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0),
            ((*s).tmp).float.offset((m * i_0) as isize) as *mut c_void,
            ((*s).tmp).float.offset((m * i_0) as isize) as *mut c_void,
            size_of::<TXComplex>() as c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: c_int = 0;
    while i_1 < len4 {
        let i0: c_int = len4 + i_1;
        let i1: c_int = len4 - i_1 - 1;
        let s0: c_int = *out_map.offset(i0 as isize);
        let s1: c_int = *out_map.offset(i1 as isize);
        let src1: TXComplex = {
            AVComplexFloat {
                re: (*((*s).tmp).float.offset(s1 as isize)).im,
                im: (*((*s).tmp).float.offset(s1 as isize)).re,
            }
        };
        let src0: TXComplex = {
            AVComplexFloat {
                re: (*((*s).tmp).float.offset(s0 as isize)).im,
                im: (*((*s).tmp).float.offset(s0 as isize)).re,
            }
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
unsafe extern "C" fn ff_tx_mdct_pfa_15xM_inv_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let mut fft15in: [TXComplex; 15] = [TXComplex { re: 0., im: 0. }; 15];
    let z: *mut TXComplex = _dst as *mut TXComplex;
    let mut exp: *mut TXComplex = (*s).exp.float;
    let src: *const TXSample = _src as *const TXSample;
    let mut in1: *const TXSample = std::ptr::null::<TXSample>();
    let mut in2: *const TXSample = std::ptr::null::<TXSample>();
    let len4: c_int = (*s).len >> 2;
    let len2: c_int = (*s).len >> 1;
    let m: c_int = (*(*s).sub).len;
    let mut in_map: *const c_int = (*s).map;
    let out_map: *const c_int = in_map.offset((15 * m) as isize);
    let mut sub_map: *const c_int = (*(*s).sub).map;
    stride = (stride as c_ulong).wrapping_div(size_of::<TXSample>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    in1 = src;
    in2 = src.offset(((15 * m * 2 - 1) as c_long * stride) as isize);
    let mut i: c_int = 0;
    while i < len2 {
        let mut j: c_int = 0;
        while j < 15 {
            let k: c_int = *in_map.offset(j as isize);
            let tmp: TXComplex = {
                AVComplexFloat {
                    re: *in2.offset((-k as c_long * stride) as isize),
                    im: *in1.offset((k as c_long * stride) as isize),
                }
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
            ((*s).tmp).float.offset(*fresh26 as isize),
            fft15in.as_mut_ptr(),
            m as ptrdiff_t,
        );
        exp = exp.offset(15);
        in_map = in_map.offset(15);
        i += 15;
    }
    let mut i_0: c_int = 0;
    while i_0 < 15 {
        ((*s).fn_0[0]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0),
            ((*s).tmp).float.offset((m * i_0) as isize) as *mut c_void,
            ((*s).tmp).float.offset((m * i_0) as isize) as *mut c_void,
            size_of::<TXComplex>() as c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: c_int = 0;
    while i_1 < len4 {
        let i0: c_int = len4 + i_1;
        let i1: c_int = len4 - i_1 - 1;
        let s0: c_int = *out_map.offset(i0 as isize);
        let s1: c_int = *out_map.offset(i1 as isize);
        let src1: TXComplex = {
            AVComplexFloat {
                re: (*((*s).tmp).float.offset(s1 as isize)).im,
                im: (*((*s).tmp).float.offset(s1 as isize)).re,
            }
        };
        let src0: TXComplex = {
            AVComplexFloat {
                re: (*((*s).tmp).float.offset(s0 as isize)).im,
                im: (*((*s).tmp).float.offset(s0 as isize)).re,
            }
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
static mut ff_tx_mdct_pfa_15xM_inv_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"mdct_pfa_15xM_inv_float_c".as_ptr(),
    function: Some(
        ff_tx_mdct_pfa_15xM_inv_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_MDCT,
    flags: (AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 63
        | (1 as c_ulonglong) << 60) as c_ulong,
    factors: [15, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 2,
    min_len: 15 * 2,
    max_len: -1,
    init: Some(
        ff_tx_mdct_pfa_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

static mut ff_tx_mdct_pfa_3xM_fwd_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"mdct_pfa_3xM_fwd_float_c".as_ptr(),
    function: Some(
        ff_tx_mdct_pfa_3xM_fwd_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_MDCT,
    flags: (AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 63
        | (1 as c_ulonglong) << 59) as c_ulong,
    factors: [3, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 2,
    min_len: 3 * 2,
    max_len: -1,
    init: Some(
        ff_tx_mdct_pfa_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

unsafe extern "C" fn ff_tx_mdct_pfa_3xM_fwd_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let mut fft3in: [TXComplex; 3] = [TXComplex { re: 0., im: 0. }; 3];
    let src: *mut TXSample = _src as *mut TXSample;
    let dst: *mut TXSample = _dst as *mut TXSample;
    let exp: *mut TXComplex = (*s).exp.float;
    let mut tmp: TXComplex = TXComplex { re: 0., im: 0. };
    let m: c_int = (*(*s).sub).len;
    let len4: c_int = 3 * m;
    let len3: c_int = len4 * 3;
    let len8: c_int = (*s).len >> 2;
    let in_map: *const c_int = (*s).map;
    let out_map: *const c_int = in_map.offset((3 * m) as isize);
    let sub_map: *const c_int = (*(*s).sub).map;
    stride = (stride as c_ulong).wrapping_div(size_of::<TXSample>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: c_int = 0;
    while i < m {
        let mut j: c_int = 0;
        while j < 3 {
            let k: c_int = *in_map.offset((i * 3 + j) as isize);
            if k < len4 {
                tmp.re =
                    -*src.offset((len4 + k) as isize) + *src.offset((1 * len4 - 1 - k) as isize);
                tmp.im =
                    -*src.offset((len3 + k) as isize) + -*src.offset((1 * len3 - 1 - k) as isize);
            } else {
                tmp.re =
                    -*src.offset((len4 + k) as isize) + -*src.offset((5 * len4 - 1 - k) as isize);
                tmp.im =
                    *src.offset((-len4 + k) as isize) + -*src.offset((1 * len3 - 1 - k) as isize);
            }
            fft3in[j as usize].im = tmp.re * (*exp.offset((k >> 1) as isize)).re
                - tmp.im * (*exp.offset((k >> 1) as isize)).im;
            fft3in[j as usize].re = tmp.re * (*exp.offset((k >> 1) as isize)).im
                + tmp.im * (*exp.offset((k >> 1) as isize)).re;
            j += 1;
            j;
        }
        fft3(
            ((*s).tmp)
                .float
                .offset(*sub_map.offset(i as isize) as isize),
            fft3in.as_mut_ptr(),
            m as ptrdiff_t,
        );
        i += 1;
        i;
    }
    let mut i_0: c_int = 0;
    while i_0 < 3 {
        ((*s).fn_0[0]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0),
            ((*s).tmp).float.offset((m * i_0) as isize) as *mut c_void,
            ((*s).tmp).float.offset((m * i_0) as isize) as *mut c_void,
            size_of::<TXComplex>() as c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: c_int = 0;
    while i_1 < len8 {
        let i0: c_int = len8 + i_1;
        let i1: c_int = len8 - i_1 - 1;
        let s0: c_int = *out_map.offset(i0 as isize);
        let s1: c_int = *out_map.offset(i1 as isize);
        let src1: TXComplex = {
            AVComplexFloat {
                re: (*((*s).tmp).float.offset(s1 as isize)).re,
                im: (*((*s).tmp).float.offset(s1 as isize)).im,
            }
        };
        let src0: TXComplex = {
            AVComplexFloat {
                re: (*((*s).tmp).float.offset(s0 as isize)).re,
                im: (*((*s).tmp).float.offset(s0 as isize)).im,
            }
        };
        *dst.offset(((2 * i1) as c_long * stride + stride) as isize) =
            src0.re * (*exp.offset(i0 as isize)).im - src0.im * (*exp.offset(i0 as isize)).re;
        *dst.offset(((2 * i0) as c_long * stride) as isize) =
            src0.re * (*exp.offset(i0 as isize)).re + src0.im * (*exp.offset(i0 as isize)).im;
        *dst.offset(((2 * i0) as c_long * stride + stride) as isize) =
            src1.re * (*exp.offset(i1 as isize)).im - src1.im * (*exp.offset(i1 as isize)).re;
        *dst.offset(((2 * i1) as c_long * stride) as isize) =
            src1.re * (*exp.offset(i1 as isize)).re + src1.im * (*exp.offset(i1 as isize)).im;
        i_1 += 1;
        i_1;
    }
}
unsafe extern "C" fn ff_tx_mdct_pfa_5xM_fwd_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let mut fft5in: [TXComplex; 5] = [TXComplex { re: 0., im: 0. }; 5];
    let src: *mut TXSample = _src as *mut TXSample;
    let dst: *mut TXSample = _dst as *mut TXSample;
    let exp: *mut TXComplex = (*s).exp.float;
    let mut tmp: TXComplex = TXComplex { re: 0., im: 0. };
    let m: c_int = (*(*s).sub).len;
    let len4: c_int = 5 * m;
    let len3: c_int = len4 * 3;
    let len8: c_int = (*s).len >> 2;
    let in_map: *const c_int = (*s).map;
    let out_map: *const c_int = in_map.offset((5 * m) as isize);
    let sub_map: *const c_int = (*(*s).sub).map;
    stride = (stride as c_ulong).wrapping_div(size_of::<TXSample>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: c_int = 0;
    while i < m {
        let mut j: c_int = 0;
        while j < 5 {
            let k: c_int = *in_map.offset((i * 5 + j) as isize);
            if k < len4 {
                tmp.re =
                    -*src.offset((len4 + k) as isize) + *src.offset((1 * len4 - 1 - k) as isize);
                tmp.im =
                    -*src.offset((len3 + k) as isize) + -*src.offset((1 * len3 - 1 - k) as isize);
            } else {
                tmp.re =
                    -*src.offset((len4 + k) as isize) + -*src.offset((5 * len4 - 1 - k) as isize);
                tmp.im =
                    *src.offset((-len4 + k) as isize) + -*src.offset((1 * len3 - 1 - k) as isize);
            }
            fft5in[j as usize].im = tmp.re * (*exp.offset((k >> 1) as isize)).re
                - tmp.im * (*exp.offset((k >> 1) as isize)).im;
            fft5in[j as usize].re = tmp.re * (*exp.offset((k >> 1) as isize)).im
                + tmp.im * (*exp.offset((k >> 1) as isize)).re;
            j += 1;
            j;
        }
        fft5(
            ((*s).tmp)
                .float
                .offset(*sub_map.offset(i as isize) as isize),
            fft5in.as_mut_ptr(),
            m as ptrdiff_t,
        );
        i += 1;
        i;
    }
    let mut i_0: c_int = 0;
    while i_0 < 5 {
        ((*s).fn_0[0]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0),
            ((*s).tmp).float.offset((m * i_0) as isize) as *mut c_void,
            ((*s).tmp).float.offset((m * i_0) as isize) as *mut c_void,
            size_of::<TXComplex>() as c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: c_int = 0;
    while i_1 < len8 {
        let i0: c_int = len8 + i_1;
        let i1: c_int = len8 - i_1 - 1;
        let s0: c_int = *out_map.offset(i0 as isize);
        let s1: c_int = *out_map.offset(i1 as isize);
        let src1: TXComplex = {
            AVComplexFloat {
                re: (*((*s).tmp).float.offset(s1 as isize)).re,
                im: (*((*s).tmp).float.offset(s1 as isize)).im,
            }
        };
        let src0: TXComplex = {
            AVComplexFloat {
                re: (*((*s).tmp).float.offset(s0 as isize)).re,
                im: (*((*s).tmp).float.offset(s0 as isize)).im,
            }
        };
        *dst.offset(((2 * i1) as c_long * stride + stride) as isize) =
            src0.re * (*exp.offset(i0 as isize)).im - src0.im * (*exp.offset(i0 as isize)).re;
        *dst.offset(((2 * i0) as c_long * stride) as isize) =
            src0.re * (*exp.offset(i0 as isize)).re + src0.im * (*exp.offset(i0 as isize)).im;
        *dst.offset(((2 * i0) as c_long * stride + stride) as isize) =
            src1.re * (*exp.offset(i1 as isize)).im - src1.im * (*exp.offset(i1 as isize)).re;
        *dst.offset(((2 * i1) as c_long * stride) as isize) =
            src1.re * (*exp.offset(i1 as isize)).re + src1.im * (*exp.offset(i1 as isize)).im;
        i_1 += 1;
        i_1;
    }
}
static mut ff_tx_mdct_pfa_5xM_fwd_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"mdct_pfa_5xM_fwd_float_c".as_ptr(),
    function: Some(
        ff_tx_mdct_pfa_5xM_fwd_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_MDCT,
    flags: (AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 63
        | (1 as c_ulonglong) << 59) as c_ulong,
    factors: [5, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 2,
    min_len: 5 * 2,
    max_len: -1,
    init: Some(
        ff_tx_mdct_pfa_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

static mut ff_tx_mdct_pfa_7xM_fwd_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"mdct_pfa_7xM_fwd_float_c".as_ptr(),
    function: Some(
        ff_tx_mdct_pfa_7xM_fwd_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_MDCT,
    flags: (AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 63
        | (1 as c_ulonglong) << 59) as c_ulong,
    factors: [7, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 2,
    min_len: 7 * 2,
    max_len: -1,
    init: Some(
        ff_tx_mdct_pfa_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

unsafe extern "C" fn ff_tx_mdct_pfa_7xM_fwd_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let mut fft7in: [TXComplex; 7] = [TXComplex { re: 0., im: 0. }; 7];
    let src: *mut TXSample = _src as *mut TXSample;
    let dst: *mut TXSample = _dst as *mut TXSample;
    let exp: *mut TXComplex = (*s).exp.float;
    let mut tmp: TXComplex = TXComplex { re: 0., im: 0. };
    let m: c_int = (*(*s).sub).len;
    let len4: c_int = 7 * m;
    let len3: c_int = len4 * 3;
    let len8: c_int = (*s).len >> 2;
    let in_map: *const c_int = (*s).map;
    let out_map: *const c_int = in_map.offset((7 * m) as isize);
    let sub_map: *const c_int = (*(*s).sub).map;
    stride = (stride as c_ulong).wrapping_div(size_of::<TXSample>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: c_int = 0;
    while i < m {
        let mut j: c_int = 0;
        while j < 7 {
            let k: c_int = *in_map.offset((i * 7 + j) as isize);
            if k < len4 {
                tmp.re =
                    -*src.offset((len4 + k) as isize) + *src.offset((1 * len4 - 1 - k) as isize);
                tmp.im =
                    -*src.offset((len3 + k) as isize) + -*src.offset((1 * len3 - 1 - k) as isize);
            } else {
                tmp.re =
                    -*src.offset((len4 + k) as isize) + -*src.offset((5 * len4 - 1 - k) as isize);
                tmp.im =
                    *src.offset((-len4 + k) as isize) + -*src.offset((1 * len3 - 1 - k) as isize);
            }
            fft7in[j as usize].im = tmp.re * (*exp.offset((k >> 1) as isize)).re
                - tmp.im * (*exp.offset((k >> 1) as isize)).im;
            fft7in[j as usize].re = tmp.re * (*exp.offset((k >> 1) as isize)).im
                + tmp.im * (*exp.offset((k >> 1) as isize)).re;
            j += 1;
            j;
        }
        fft7(
            ((*s).tmp)
                .float
                .offset(*sub_map.offset(i as isize) as isize),
            fft7in.as_mut_ptr(),
            m as ptrdiff_t,
        );
        i += 1;
        i;
    }
    let mut i_0: c_int = 0;
    while i_0 < 7 {
        ((*s).fn_0[0]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0),
            ((*s).tmp).float.offset((m * i_0) as isize) as *mut c_void,
            ((*s).tmp).float.offset((m * i_0) as isize) as *mut c_void,
            size_of::<TXComplex>() as c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: c_int = 0;
    while i_1 < len8 {
        let i0: c_int = len8 + i_1;
        let i1: c_int = len8 - i_1 - 1;
        let s0: c_int = *out_map.offset(i0 as isize);
        let s1: c_int = *out_map.offset(i1 as isize);
        let src1: TXComplex = {
            AVComplexFloat {
                re: (*((*s).tmp).float.offset(s1 as isize)).re,
                im: (*((*s).tmp).float.offset(s1 as isize)).im,
            }
        };
        let src0: TXComplex = {
            AVComplexFloat {
                re: (*((*s).tmp).float.offset(s0 as isize)).re,
                im: (*((*s).tmp).float.offset(s0 as isize)).im,
            }
        };
        *dst.offset(((2 * i1) as c_long * stride + stride) as isize) =
            src0.re * (*exp.offset(i0 as isize)).im - src0.im * (*exp.offset(i0 as isize)).re;
        *dst.offset(((2 * i0) as c_long * stride) as isize) =
            src0.re * (*exp.offset(i0 as isize)).re + src0.im * (*exp.offset(i0 as isize)).im;
        *dst.offset(((2 * i0) as c_long * stride + stride) as isize) =
            src1.re * (*exp.offset(i1 as isize)).im - src1.im * (*exp.offset(i1 as isize)).re;
        *dst.offset(((2 * i1) as c_long * stride) as isize) =
            src1.re * (*exp.offset(i1 as isize)).re + src1.im * (*exp.offset(i1 as isize)).im;
        i_1 += 1;
        i_1;
    }
}
static mut ff_tx_mdct_pfa_9xM_fwd_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"mdct_pfa_9xM_fwd_float_c".as_ptr(),
    function: Some(
        ff_tx_mdct_pfa_9xM_fwd_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_MDCT,
    flags: (AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 63
        | (1 as c_ulonglong) << 59) as c_ulong,
    factors: [9, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 2,
    min_len: 9 * 2,
    max_len: -1,
    init: Some(
        ff_tx_mdct_pfa_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

unsafe extern "C" fn ff_tx_mdct_pfa_9xM_fwd_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let mut fft9in: [TXComplex; 9] = [TXComplex { re: 0., im: 0. }; 9];
    let src: *mut TXSample = _src as *mut TXSample;
    let dst: *mut TXSample = _dst as *mut TXSample;
    let exp: *mut TXComplex = (*s).exp.float;
    let mut tmp: TXComplex = TXComplex { re: 0., im: 0. };
    let m: c_int = (*(*s).sub).len;
    let len4: c_int = 9 * m;
    let len3: c_int = len4 * 3;
    let len8: c_int = (*s).len >> 2;
    let in_map: *const c_int = (*s).map;
    let out_map: *const c_int = in_map.offset((9 * m) as isize);
    let sub_map: *const c_int = (*(*s).sub).map;
    stride = (stride as c_ulong).wrapping_div(size_of::<TXSample>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: c_int = 0;
    while i < m {
        let mut j: c_int = 0;
        while j < 9 {
            let k: c_int = *in_map.offset((i * 9 + j) as isize);
            if k < len4 {
                tmp.re =
                    -*src.offset((len4 + k) as isize) + *src.offset((1 * len4 - 1 - k) as isize);
                tmp.im =
                    -*src.offset((len3 + k) as isize) + -*src.offset((1 * len3 - 1 - k) as isize);
            } else {
                tmp.re =
                    -*src.offset((len4 + k) as isize) + -*src.offset((5 * len4 - 1 - k) as isize);
                tmp.im =
                    *src.offset((-len4 + k) as isize) + -*src.offset((1 * len3 - 1 - k) as isize);
            }
            fft9in[j as usize].im = tmp.re * (*exp.offset((k >> 1) as isize)).re
                - tmp.im * (*exp.offset((k >> 1) as isize)).im;
            fft9in[j as usize].re = tmp.re * (*exp.offset((k >> 1) as isize)).im
                + tmp.im * (*exp.offset((k >> 1) as isize)).re;
            j += 1;
            j;
        }
        fft9(
            ((*s).tmp)
                .float
                .offset(*sub_map.offset(i as isize) as isize),
            fft9in.as_mut_ptr(),
            m as ptrdiff_t,
        );
        i += 1;
        i;
    }
    let mut i_0: c_int = 0;
    while i_0 < 9 {
        ((*s).fn_0[0]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0),
            ((*s).tmp).float.offset((m * i_0) as isize) as *mut c_void,
            ((*s).tmp).float.offset((m * i_0) as isize) as *mut c_void,
            size_of::<TXComplex>() as c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: c_int = 0;
    while i_1 < len8 {
        let i0: c_int = len8 + i_1;
        let i1: c_int = len8 - i_1 - 1;
        let s0: c_int = *out_map.offset(i0 as isize);
        let s1: c_int = *out_map.offset(i1 as isize);
        let src1: TXComplex = {
            AVComplexFloat {
                re: (*((*s).tmp).float.offset(s1 as isize)).re,
                im: (*((*s).tmp).float.offset(s1 as isize)).im,
            }
        };
        let src0: TXComplex = {
            AVComplexFloat {
                re: (*((*s).tmp).float.offset(s0 as isize)).re,
                im: (*((*s).tmp).float.offset(s0 as isize)).im,
            }
        };
        *dst.offset(((2 * i1) as c_long * stride + stride) as isize) =
            src0.re * (*exp.offset(i0 as isize)).im - src0.im * (*exp.offset(i0 as isize)).re;
        *dst.offset(((2 * i0) as c_long * stride) as isize) =
            src0.re * (*exp.offset(i0 as isize)).re + src0.im * (*exp.offset(i0 as isize)).im;
        *dst.offset(((2 * i0) as c_long * stride + stride) as isize) =
            src1.re * (*exp.offset(i1 as isize)).im - src1.im * (*exp.offset(i1 as isize)).re;
        *dst.offset(((2 * i1) as c_long * stride) as isize) =
            src1.re * (*exp.offset(i1 as isize)).re + src1.im * (*exp.offset(i1 as isize)).im;
        i_1 += 1;
        i_1;
    }
}
static mut ff_tx_mdct_pfa_15xM_fwd_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"mdct_pfa_15xM_fwd_float_c".as_ptr(),
    function: Some(
        ff_tx_mdct_pfa_15xM_fwd_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_MDCT,
    flags: (AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 63
        | (1 as c_ulonglong) << 59) as c_ulong,
    factors: [15, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 2,
    min_len: 15 * 2,
    max_len: -1,
    init: Some(
        ff_tx_mdct_pfa_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

unsafe extern "C" fn ff_tx_mdct_pfa_15xM_fwd_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let mut fft15in: [TXComplex; 15] = [TXComplex { re: 0., im: 0. }; 15];
    let src: *mut TXSample = _src as *mut TXSample;
    let dst: *mut TXSample = _dst as *mut TXSample;
    let exp: *mut TXComplex = (*s).exp.float;
    let mut tmp: TXComplex = TXComplex { re: 0., im: 0. };
    let m: c_int = (*(*s).sub).len;
    let len4: c_int = 15 * m;
    let len3: c_int = len4 * 3;
    let len8: c_int = (*s).len >> 2;
    let in_map: *const c_int = (*s).map;
    let out_map: *const c_int = in_map.offset((15 * m) as isize);
    let sub_map: *const c_int = (*(*s).sub).map;
    stride = (stride as c_ulong).wrapping_div(size_of::<TXSample>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: c_int = 0;
    while i < m {
        let mut j: c_int = 0;
        while j < 15 {
            let k: c_int = *in_map.offset((i * 15 + j) as isize);
            if k < len4 {
                tmp.re =
                    -*src.offset((len4 + k) as isize) + *src.offset((1 * len4 - 1 - k) as isize);
                tmp.im =
                    -*src.offset((len3 + k) as isize) + -*src.offset((1 * len3 - 1 - k) as isize);
            } else {
                tmp.re =
                    -*src.offset((len4 + k) as isize) + -*src.offset((5 * len4 - 1 - k) as isize);
                tmp.im =
                    *src.offset((-len4 + k) as isize) + -*src.offset((1 * len3 - 1 - k) as isize);
            }
            fft15in[j as usize].im = tmp.re * (*exp.offset((k >> 1) as isize)).re
                - tmp.im * (*exp.offset((k >> 1) as isize)).im;
            fft15in[j as usize].re = tmp.re * (*exp.offset((k >> 1) as isize)).im
                + tmp.im * (*exp.offset((k >> 1) as isize)).re;
            j += 1;
            j;
        }
        fft15(
            ((*s).tmp)
                .float
                .offset(*sub_map.offset(i as isize) as isize),
            fft15in.as_mut_ptr(),
            m as ptrdiff_t,
        );
        i += 1;
        i;
    }
    let mut i_0: c_int = 0;
    while i_0 < 15 {
        ((*s).fn_0[0]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0),
            ((*s).tmp).float.offset((m * i_0) as isize) as *mut c_void,
            ((*s).tmp).float.offset((m * i_0) as isize) as *mut c_void,
            size_of::<TXComplex>() as c_ulong as ptrdiff_t,
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: c_int = 0;
    while i_1 < len8 {
        let i0: c_int = len8 + i_1;
        let i1: c_int = len8 - i_1 - 1;
        let s0: c_int = *out_map.offset(i0 as isize);
        let s1: c_int = *out_map.offset(i1 as isize);
        let src1: TXComplex = {
            AVComplexFloat {
                re: (*((*s).tmp).float.offset(s1 as isize)).re,
                im: (*((*s).tmp).float.offset(s1 as isize)).im,
            }
        };
        let src0: TXComplex = {
            AVComplexFloat {
                re: (*((*s).tmp).float.offset(s0 as isize)).re,
                im: (*((*s).tmp).float.offset(s0 as isize)).im,
            }
        };
        *dst.offset(((2 * i1) as c_long * stride + stride) as isize) =
            src0.re * (*exp.offset(i0 as isize)).im - src0.im * (*exp.offset(i0 as isize)).re;
        *dst.offset(((2 * i0) as c_long * stride) as isize) =
            src0.re * (*exp.offset(i0 as isize)).re + src0.im * (*exp.offset(i0 as isize)).im;
        *dst.offset(((2 * i0) as c_long * stride + stride) as isize) =
            src1.re * (*exp.offset(i1 as isize)).im - src1.im * (*exp.offset(i1 as isize)).re;
        *dst.offset(((2 * i1) as c_long * stride) as isize) =
            src1.re * (*exp.offset(i1 as isize)).re + src1.im * (*exp.offset(i1 as isize)).im;
        i_1 += 1;
        i_1;
    }
}
#[cold]
unsafe extern "C" fn ff_tx_rdft_init_float_c(
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
    let len4: c_int = (len + 4 - 1 & !(4 - 1)) / 4;
    (*s).scale_d = *(scale as *mut c_float) as c_double;
    (*s).scale_f = (*s).scale_d as c_float;
    flags &= !(AV_TX_REAL_TO_REAL as c_int | AV_TX_REAL_TO_IMAGINARY as c_int) as c_ulong;
    ret = ff_tx_init_subtx(
        s,
        AV_TX_FLOAT_FFT,
        flags,
        std::ptr::null_mut::<FFTXCodeletOptions>(),
        len >> 1,
        inv,
        scale,
    );
    if ret != 0 {
        return ret;
    }
    (*s).exp = AVTXNum {
        float: av_mallocz(
            ((8 + 2 * len4) as c_ulong).wrapping_mul(size_of::<TXComplex>() as c_ulong),
        ) as *mut TXComplex,
    };
    if ((*s).exp).float.is_null() {
        return -12;
    }
    tab = (*s).exp.float as *mut TXSample;
    f = 2. * PI / len as c_double;
    m = if inv != 0 {
        2. * (*s).scale_d
    } else {
        (*s).scale_d
    };
    let fresh27 = tab;
    tab = tab.offset(1);
    *fresh27 = ((if inv != 0 { 0.5 } else { 1. }) * m) as TXSample;
    let fresh28 = tab;
    tab = tab.offset(1);
    *fresh28 = (if inv != 0 { 0.5 * m } else { 1. * m }) as TXSample;
    let fresh29 = tab;
    tab = tab.offset(1);
    *fresh29 = m as TXSample;
    let fresh30 = tab;
    tab = tab.offset(1);
    *fresh30 = -m as TXSample;
    let fresh31 = tab;
    tab = tab.offset(1);
    *fresh31 = ((0.5 - 0.) * m) as TXSample;
    if r2r != 0 {
        let fresh32 = tab;
        tab = tab.offset(1);
        *fresh32 = 1. / (*s).scale_f;
    } else {
        let fresh33 = tab;
        tab = tab.offset(1);
        *fresh33 = ((0. - 0.5) * m) as TXSample;
    }
    let fresh34 = tab;
    tab = tab.offset(1);
    *fresh34 = ((0.5 - inv as c_double) * m) as TXSample;
    let fresh35 = tab;
    tab = tab.offset(1);
    *fresh35 = (-(0.5 - inv as c_double) * m) as TXSample;
    let mut i: c_int = 0;
    while i < len4 {
        let fresh36 = tab;
        tab = tab.offset(1);
        *fresh36 = cos(i as c_double * f) as TXSample;
        i += 1;
        i;
    }
    tab = ((*s).exp.float as *mut TXSample)
        .offset(len4 as isize)
        .offset(8);
    let mut i_0: c_int = 0;
    while i_0 < len4 {
        let fresh37 = tab;
        tab = tab.offset(1);
        *fresh37 = (cos((len - i_0 * 4) as c_double / 4. * f)
            * (if inv != 0 { 1 } else { -1 }) as c_double) as TXSample;
        i_0 += 1;
        i_0;
    }
    0
}
static mut ff_tx_rdft_r2c_def_float_c: FFTXCodelet = FFTXCodelet {
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
unsafe extern "C" fn ff_tx_rdft_r2c_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    _stride: ptrdiff_t,
) {
    let len2: c_int = (*s).len >> 1;
    let len4: c_int = (*s).len >> 2;
    let fact: *const TXSample = (*s).exp.float as *mut c_void as *const TXSample;
    let tcos: *const TXSample = fact.offset(8);
    let tsin: *const TXSample = tcos.offset(len4 as isize);
    let data: *mut TXComplex = (if 0 != 0 { _src } else { _dst }) as *mut TXComplex;
    let mut t: [TXComplex; 3] = [TXComplex { re: 0., im: 0. }; 3];
    if 0 == 0 {
        ((*s).fn_0[0]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0),
            data as *mut c_void,
            _src,
            size_of::<TXComplex>() as c_ulong as ptrdiff_t,
        );
    } else {
        (*data.offset(0)).im = (*data.offset(len2 as isize)).re;
    }
    t[0].re = (*data.offset(0)).re;
    (*data.offset(0)).re = t[0].re + (*data.offset(0)).im;
    (*data.offset(0)).im = t[0].re - (*data.offset(0)).im;
    (*data.offset(0)).re = *fact.offset(0) * (*data.offset(0)).re;
    (*data.offset(0)).im = *fact.offset(1) * (*data.offset(0)).im;
    (*data.offset(len4 as isize)).re = *fact.offset(2) * (*data.offset(len4 as isize)).re;
    (*data.offset(len4 as isize)).im = *fact.offset(3) * (*data.offset(len4 as isize)).im;
    let mut i: c_int = 1;
    while i < len4 {
        t[0].re = *fact.offset(4)
            * ((*data.offset(i as isize)).re + (*data.offset((len2 - i) as isize)).re);
        t[0].im = *fact.offset(5)
            * ((*data.offset(i as isize)).im - (*data.offset((len2 - i) as isize)).im);
        t[1].re = *fact.offset(6)
            * ((*data.offset(i as isize)).im + (*data.offset((len2 - i) as isize)).im);
        t[1].im = *fact.offset(7)
            * ((*data.offset(i as isize)).re - (*data.offset((len2 - i) as isize)).re);
        t[2].re = t[1].re * *tcos.offset(i as isize) - t[1].im * *tsin.offset(i as isize);
        t[2].im = t[1].re * *tsin.offset(i as isize) + t[1].im * *tcos.offset(i as isize);
        (*data.offset(i as isize)).re = t[0].re + t[2].re;
        (*data.offset(i as isize)).im = t[2].im - t[0].im;
        (*data.offset((len2 - i) as isize)).re = t[0].re - t[2].re;
        (*data.offset((len2 - i) as isize)).im = t[2].im + t[0].im;
        i += 1;
        i;
    }
    (*data.offset(len2 as isize)).re = (*data.offset(0)).im;
    let fresh38 = &mut (*data.offset(len2 as isize)).im;
    *fresh38 = 0.;
    (*data.offset(0)).im = *fresh38;
}
unsafe extern "C" fn ff_tx_rdft_c2r_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    _stride: ptrdiff_t,
) {
    let len2: c_int = (*s).len >> 1;
    let len4: c_int = (*s).len >> 2;
    let fact: *const TXSample = (*s).exp.float as *mut c_void as *const TXSample;
    let tcos: *const TXSample = fact.offset(8);
    let tsin: *const TXSample = tcos.offset(len4 as isize);
    let data: *mut TXComplex = (if 1 != 0 { _src } else { _dst }) as *mut TXComplex;
    let mut t: [TXComplex; 3] = [TXComplex { re: 0., im: 0. }; 3];
    if 1 == 0 {
        ((*s).fn_0[0]).expect("non-null function pointer")(
            &mut *((*s).sub).offset(0),
            data as *mut c_void,
            _src,
            size_of::<TXComplex>() as c_ulong as ptrdiff_t,
        );
    } else {
        (*data.offset(0)).im = (*data.offset(len2 as isize)).re;
    }
    t[0].re = (*data.offset(0)).re;
    (*data.offset(0)).re = t[0].re + (*data.offset(0)).im;
    (*data.offset(0)).im = t[0].re - (*data.offset(0)).im;
    (*data.offset(0)).re = *fact.offset(0) * (*data.offset(0)).re;
    (*data.offset(0)).im = *fact.offset(1) * (*data.offset(0)).im;
    (*data.offset(len4 as isize)).re = *fact.offset(2) * (*data.offset(len4 as isize)).re;
    (*data.offset(len4 as isize)).im = *fact.offset(3) * (*data.offset(len4 as isize)).im;
    let mut i: c_int = 1;
    while i < len4 {
        t[0].re = *fact.offset(4)
            * ((*data.offset(i as isize)).re + (*data.offset((len2 - i) as isize)).re);
        t[0].im = *fact.offset(5)
            * ((*data.offset(i as isize)).im - (*data.offset((len2 - i) as isize)).im);
        t[1].re = *fact.offset(6)
            * ((*data.offset(i as isize)).im + (*data.offset((len2 - i) as isize)).im);
        t[1].im = *fact.offset(7)
            * ((*data.offset(i as isize)).re - (*data.offset((len2 - i) as isize)).re);
        t[2].re = t[1].re * *tcos.offset(i as isize) - t[1].im * *tsin.offset(i as isize);
        t[2].im = t[1].re * *tsin.offset(i as isize) + t[1].im * *tcos.offset(i as isize);
        (*data.offset(i as isize)).re = t[0].re + t[2].re;
        (*data.offset(i as isize)).im = t[2].im - t[0].im;
        (*data.offset((len2 - i) as isize)).re = t[0].re - t[2].re;
        (*data.offset((len2 - i) as isize)).im = t[2].im + t[0].im;
        i += 1;
        i;
    }
    ((*s).fn_0[0]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0),
        _dst,
        data as *mut c_void,
        size_of::<TXComplex>() as c_ulong as ptrdiff_t,
    );
}
static mut ff_tx_rdft_c2r_def_float_c: FFTXCodelet = FFTXCodelet {
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
unsafe extern "C" fn ff_tx_rdft_r2r_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    _stride: ptrdiff_t,
) {
    let len: c_int = (*s).len;
    let len2: c_int = len >> 1;
    let len4: c_int = len >> 2;
    let aligned_len4: c_int = (len + 4 - 1 & !(4 - 1)) / 4;
    let fact: *const TXSample = (*s).exp.float as *mut c_void as *const TXSample;
    let tcos: *const TXSample = fact.offset(8);
    let tsin: *const TXSample = tcos.offset(aligned_len4 as isize);
    let data: *mut TXComplex = _dst as *mut TXComplex;
    let out: *mut TXSample = _dst as *mut TXSample;
    let mut tmp_dc: TXSample = 0.;
    let mut tmp_mid: TXSample = 0.;
    let mut tmp: [TXSample; 4] = [0.; 4];
    let mut sf: TXComplex = TXComplex { re: 0., im: 0. };
    let mut sl: TXComplex = TXComplex { re: 0., im: 0. };
    ((*s).fn_0[0]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0),
        _dst,
        _src,
        size_of::<TXComplex>() as c_ulong as ptrdiff_t,
    );
    tmp_dc = (*data.offset(0)).re;
    (*data.offset(0)).re = tmp_dc + (*data.offset(0)).im;
    tmp_dc -= (*data.offset(0)).im;
    (*data.offset(0)).re = *fact.offset(0) * (*data.offset(0)).re;
    tmp_dc *= *fact.offset(1);
    (*data.offset(len4 as isize)).re = *fact.offset(2) * (*data.offset(len4 as isize)).re;
    if 0 == 0 {
        (*data.offset(len4 as isize)).im = *fact.offset(3) * (*data.offset(len4 as isize)).im;
    } else {
        sf = *data.offset(len4 as isize);
        sl = *data.offset((len4 + 1) as isize);
        if AV_TX_REAL_TO_REAL as c_int == AV_TX_REAL_TO_REAL as c_int {
            tmp[0] = *fact.offset(4) * (sf.re + sl.re);
        } else {
            tmp[0] = *fact.offset(5) * (sf.im - sl.im);
        }
        tmp[1] = *fact.offset(6) * (sf.im + sl.im);
        tmp[2] = *fact.offset(7) * (sf.re - sl.re);
        if AV_TX_REAL_TO_REAL as c_int == AV_TX_REAL_TO_REAL as c_int {
            tmp[3] = tmp[1] * *tcos.offset(len4 as isize) - tmp[2] * *tsin.offset(len4 as isize);
            tmp_mid = tmp[0] - tmp[3];
        } else {
            tmp[3] = tmp[1] * *tsin.offset(len4 as isize) + tmp[2] * *tcos.offset(len4 as isize);
            tmp_mid = tmp[0] + tmp[3];
        }
    }
    let mut i: c_int = 1;
    while i <= len4 {
        let mut tmp_0: [TXSample; 4] = [0.; 4];
        let sf_0: TXComplex = *data.offset(i as isize);
        let sl_0: TXComplex = *data.offset((len2 - i) as isize);
        if AV_TX_REAL_TO_REAL as c_int == AV_TX_REAL_TO_REAL as c_int {
            tmp_0[0] = *fact.offset(4) * (sf_0.re + sl_0.re);
        } else {
            tmp_0[0] = *fact.offset(5) * (sf_0.im - sl_0.im);
        }
        tmp_0[1] = *fact.offset(6) * (sf_0.im + sl_0.im);
        tmp_0[2] = *fact.offset(7) * (sf_0.re - sl_0.re);
        if AV_TX_REAL_TO_REAL as c_int == AV_TX_REAL_TO_REAL as c_int {
            tmp_0[3] = tmp_0[1] * *tcos.offset(i as isize) - tmp_0[2] * *tsin.offset(i as isize);
            *out.offset(i as isize) = tmp_0[0] + tmp_0[3];
            *out.offset((len - i) as isize) = tmp_0[0] - tmp_0[3];
        } else {
            tmp_0[3] = tmp_0[1] * *tsin.offset(i as isize) + tmp_0[2] * *tcos.offset(i as isize);
            *out.offset((i - 1) as isize) = tmp_0[3] - tmp_0[0];
            *out.offset((len - i - 1) as isize) = tmp_0[0] + tmp_0[3];
        }
        i += 1;
        i;
    }
    let mut i_0: c_int = 1;
    while i_0 < len4 + (AV_TX_REAL_TO_REAL as c_int == AV_TX_REAL_TO_IMAGINARY as c_int) as c_int {
        *out.offset((len2 - i_0) as isize) = *out.offset((len - i_0) as isize);
        i_0 += 1;
        i_0;
    }
    if AV_TX_REAL_TO_REAL as c_int == AV_TX_REAL_TO_REAL as c_int {
        *out.offset(len2 as isize) = tmp_dc;
    }
}
static mut ff_tx_rdft_r2r_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"rdft_r2r_float_c".as_ptr(),
    function: Some(
        ff_tx_rdft_r2r_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_RDFT,
    flags: ((AV_TX_UNALIGNED as c_int | AV_TX_INPLACE as c_int | AV_TX_REAL_TO_REAL as c_int)
        as c_ulonglong
        | (1 as c_ulonglong) << 63
        | (1 as c_ulonglong) << 59) as c_ulong,
    factors: [
        2 + 2 * (0 == 0) as c_int,
        -1,
        0,
        0,
        0,
        0,
        0,
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
    nb_factors: 2,
    min_len: 2 + 2 * (0 == 0) as c_int,
    max_len: -1,
    init: Some(
        ff_tx_rdft_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

static mut ff_tx_rdft_r2r_mod2_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"rdft_r2r_mod2_float_c".as_ptr(),
    function: Some(
        ff_tx_rdft_r2r_mod2_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_RDFT,
    flags: ((AV_TX_UNALIGNED as c_int | AV_TX_INPLACE as c_int | AV_TX_REAL_TO_REAL as c_int)
        as c_ulonglong
        | (1 as c_ulonglong) << 63
        | (1 as c_ulonglong) << 59) as c_ulong,
    factors: [
        2 + 2 * (1 == 0) as c_int,
        -1,
        0,
        0,
        0,
        0,
        0,
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
    nb_factors: 2,
    min_len: 2 + 2 * (1 == 0) as c_int,
    max_len: -1,
    init: Some(
        ff_tx_rdft_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

unsafe extern "C" fn ff_tx_rdft_r2r_mod2_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    _stride: ptrdiff_t,
) {
    let len: c_int = (*s).len;
    let len2: c_int = len >> 1;
    let len4: c_int = len >> 2;
    let aligned_len4: c_int = (len + 4 - 1 & !(4 - 1)) / 4;
    let fact: *const TXSample = (*s).exp.float as *mut c_void as *const TXSample;
    let tcos: *const TXSample = fact.offset(8);
    let tsin: *const TXSample = tcos.offset(aligned_len4 as isize);
    let data: *mut TXComplex = _dst as *mut TXComplex;
    let out: *mut TXSample = _dst as *mut TXSample;
    let mut tmp_dc: TXSample = 0.;
    let mut tmp_mid: TXSample = 0.;
    let mut tmp: [TXSample; 4] = [0.; 4];
    let mut sf: TXComplex = TXComplex { re: 0., im: 0. };
    let mut sl: TXComplex = TXComplex { re: 0., im: 0. };
    ((*s).fn_0[0]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0),
        _dst,
        _src,
        size_of::<TXComplex>() as c_ulong as ptrdiff_t,
    );
    tmp_dc = (*data.offset(0)).re;
    (*data.offset(0)).re = tmp_dc + (*data.offset(0)).im;
    tmp_dc -= (*data.offset(0)).im;
    (*data.offset(0)).re = *fact.offset(0) * (*data.offset(0)).re;
    tmp_dc *= *fact.offset(1);
    (*data.offset(len4 as isize)).re = *fact.offset(2) * (*data.offset(len4 as isize)).re;
    if 1 == 0 {
        (*data.offset(len4 as isize)).im = *fact.offset(3) * (*data.offset(len4 as isize)).im;
    } else {
        sf = *data.offset(len4 as isize);
        sl = *data.offset((len4 + 1) as isize);
        if AV_TX_REAL_TO_REAL as c_int == AV_TX_REAL_TO_REAL as c_int {
            tmp[0] = *fact.offset(4) * (sf.re + sl.re);
        } else {
            tmp[0] = *fact.offset(5) * (sf.im - sl.im);
        }
        tmp[1] = *fact.offset(6) * (sf.im + sl.im);
        tmp[2] = *fact.offset(7) * (sf.re - sl.re);
        if AV_TX_REAL_TO_REAL as c_int == AV_TX_REAL_TO_REAL as c_int {
            tmp[3] = tmp[1] * *tcos.offset(len4 as isize) - tmp[2] * *tsin.offset(len4 as isize);
            tmp_mid = tmp[0] - tmp[3];
        } else {
            tmp[3] = tmp[1] * *tsin.offset(len4 as isize) + tmp[2] * *tcos.offset(len4 as isize);
            tmp_mid = tmp[0] + tmp[3];
        }
    }
    let mut i: c_int = 1;
    while i <= len4 {
        let mut tmp_0: [TXSample; 4] = [0.; 4];
        let sf_0: TXComplex = *data.offset(i as isize);
        let sl_0: TXComplex = *data.offset((len2 - i) as isize);
        if AV_TX_REAL_TO_REAL as c_int == AV_TX_REAL_TO_REAL as c_int {
            tmp_0[0] = *fact.offset(4) * (sf_0.re + sl_0.re);
        } else {
            tmp_0[0] = *fact.offset(5) * (sf_0.im - sl_0.im);
        }
        tmp_0[1] = *fact.offset(6) * (sf_0.im + sl_0.im);
        tmp_0[2] = *fact.offset(7) * (sf_0.re - sl_0.re);
        if AV_TX_REAL_TO_REAL as c_int == AV_TX_REAL_TO_REAL as c_int {
            tmp_0[3] = tmp_0[1] * *tcos.offset(i as isize) - tmp_0[2] * *tsin.offset(i as isize);
            *out.offset(i as isize) = tmp_0[0] + tmp_0[3];
            *out.offset((len - i) as isize) = tmp_0[0] - tmp_0[3];
        } else {
            tmp_0[3] = tmp_0[1] * *tsin.offset(i as isize) + tmp_0[2] * *tcos.offset(i as isize);
            *out.offset((i - 1) as isize) = tmp_0[3] - tmp_0[0];
            *out.offset((len - i - 1) as isize) = tmp_0[0] + tmp_0[3];
        }
        i += 1;
        i;
    }
    let mut i_0: c_int = 1;
    while i_0 < len4 + (AV_TX_REAL_TO_REAL as c_int == AV_TX_REAL_TO_IMAGINARY as c_int) as c_int {
        *out.offset((len2 - i_0) as isize) = *out.offset((len - i_0) as isize);
        i_0 += 1;
        i_0;
    }
    if AV_TX_REAL_TO_REAL as c_int == AV_TX_REAL_TO_REAL as c_int {
        *out.offset(len2 as isize) = tmp_dc;
        *out.offset((len4 + 1) as isize) = tmp_mid * *fact.offset(5);
    } else {
        *out.offset(len4 as isize) = tmp_mid;
    };
}
static mut ff_tx_rdft_r2i_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"rdft_r2i_float_c".as_ptr(),
    function: Some(
        ff_tx_rdft_r2i_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_RDFT,
    flags: ((AV_TX_UNALIGNED as c_int | AV_TX_INPLACE as c_int | AV_TX_REAL_TO_IMAGINARY as c_int)
        as c_ulonglong
        | (1 as c_ulonglong) << 63
        | (1 as c_ulonglong) << 59) as c_ulong,
    factors: [
        2 + 2 * (0 == 0) as c_int,
        -1,
        0,
        0,
        0,
        0,
        0,
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
    nb_factors: 2,
    min_len: 2 + 2 * (0 == 0) as c_int,
    max_len: -1,
    init: Some(
        ff_tx_rdft_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

unsafe extern "C" fn ff_tx_rdft_r2i_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    _stride: ptrdiff_t,
) {
    let len: c_int = (*s).len;
    let len2: c_int = len >> 1;
    let len4: c_int = len >> 2;
    let aligned_len4: c_int = (len + 4 - 1 & !(4 - 1)) / 4;
    let fact: *const TXSample = (*s).exp.float as *mut c_void as *const TXSample;
    let tcos: *const TXSample = fact.offset(8);
    let tsin: *const TXSample = tcos.offset(aligned_len4 as isize);
    let data: *mut TXComplex = _dst as *mut TXComplex;
    let out: *mut TXSample = _dst as *mut TXSample;
    let mut tmp_dc: TXSample = 0.;
    let mut tmp_mid: TXSample = 0.;
    let mut tmp: [TXSample; 4] = [0.; 4];
    let mut sf: TXComplex = TXComplex { re: 0., im: 0. };
    let mut sl: TXComplex = TXComplex { re: 0., im: 0. };
    ((*s).fn_0[0]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0),
        _dst,
        _src,
        size_of::<TXComplex>() as c_ulong as ptrdiff_t,
    );
    tmp_dc = (*data.offset(0)).re;
    (*data.offset(0)).re = tmp_dc + (*data.offset(0)).im;
    tmp_dc -= (*data.offset(0)).im;
    (*data.offset(0)).re = *fact.offset(0) * (*data.offset(0)).re;
    tmp_dc *= *fact.offset(1);
    (*data.offset(len4 as isize)).re = *fact.offset(2) * (*data.offset(len4 as isize)).re;
    if 0 == 0 {
        (*data.offset(len4 as isize)).im = *fact.offset(3) * (*data.offset(len4 as isize)).im;
    } else {
        sf = *data.offset(len4 as isize);
        sl = *data.offset((len4 + 1) as isize);
        if AV_TX_REAL_TO_IMAGINARY as c_int == AV_TX_REAL_TO_REAL as c_int {
            tmp[0] = *fact.offset(4) * (sf.re + sl.re);
        } else {
            tmp[0] = *fact.offset(5) * (sf.im - sl.im);
        }
        tmp[1] = *fact.offset(6) * (sf.im + sl.im);
        tmp[2] = *fact.offset(7) * (sf.re - sl.re);
        if AV_TX_REAL_TO_IMAGINARY as c_int == AV_TX_REAL_TO_REAL as c_int {
            tmp[3] = tmp[1] * *tcos.offset(len4 as isize) - tmp[2] * *tsin.offset(len4 as isize);
            tmp_mid = tmp[0] - tmp[3];
        } else {
            tmp[3] = tmp[1] * *tsin.offset(len4 as isize) + tmp[2] * *tcos.offset(len4 as isize);
            tmp_mid = tmp[0] + tmp[3];
        }
    }
    let mut i: c_int = 1;
    while i <= len4 {
        let mut tmp_0: [TXSample; 4] = [0.; 4];
        let sf_0: TXComplex = *data.offset(i as isize);
        let sl_0: TXComplex = *data.offset((len2 - i) as isize);
        if AV_TX_REAL_TO_IMAGINARY as c_int == AV_TX_REAL_TO_REAL as c_int {
            tmp_0[0] = *fact.offset(4) * (sf_0.re + sl_0.re);
        } else {
            tmp_0[0] = *fact.offset(5) * (sf_0.im - sl_0.im);
        }
        tmp_0[1] = *fact.offset(6) * (sf_0.im + sl_0.im);
        tmp_0[2] = *fact.offset(7) * (sf_0.re - sl_0.re);
        if AV_TX_REAL_TO_IMAGINARY as c_int == AV_TX_REAL_TO_REAL as c_int {
            tmp_0[3] = tmp_0[1] * *tcos.offset(i as isize) - tmp_0[2] * *tsin.offset(i as isize);
            *out.offset(i as isize) = tmp_0[0] + tmp_0[3];
            *out.offset((len - i) as isize) = tmp_0[0] - tmp_0[3];
        } else {
            tmp_0[3] = tmp_0[1] * *tsin.offset(i as isize) + tmp_0[2] * *tcos.offset(i as isize);
            *out.offset((i - 1) as isize) = tmp_0[3] - tmp_0[0];
            *out.offset((len - i - 1) as isize) = tmp_0[0] + tmp_0[3];
        }
        i += 1;
        i;
    }
    let mut i_0: c_int = 1;
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
static mut ff_tx_rdft_r2i_mod2_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"rdft_r2i_mod2_float_c".as_ptr(),
    function: Some(
        ff_tx_rdft_r2i_mod2_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_RDFT,
    flags: ((AV_TX_UNALIGNED as c_int | AV_TX_INPLACE as c_int | AV_TX_REAL_TO_IMAGINARY as c_int)
        as c_ulonglong
        | (1 as c_ulonglong) << 63
        | (1 as c_ulonglong) << 59) as c_ulong,
    factors: [
        2 + 2 * (1 == 0) as c_int,
        -1,
        0,
        0,
        0,
        0,
        0,
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
    nb_factors: 2,
    min_len: 2 + 2 * (1 == 0) as c_int,
    max_len: -1,
    init: Some(
        ff_tx_rdft_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

unsafe extern "C" fn ff_tx_rdft_r2i_mod2_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    _stride: ptrdiff_t,
) {
    let len: c_int = (*s).len;
    let len2: c_int = len >> 1;
    let len4: c_int = len >> 2;
    let aligned_len4: c_int = (len + 4 - 1 & !(4 - 1)) / 4;
    let fact: *const TXSample = (*s).exp.float as *mut c_void as *const TXSample;
    let tcos: *const TXSample = fact.offset(8);
    let tsin: *const TXSample = tcos.offset(aligned_len4 as isize);
    let data: *mut TXComplex = _dst as *mut TXComplex;
    let out: *mut TXSample = _dst as *mut TXSample;
    let mut tmp_dc: TXSample = 0.;
    let mut tmp_mid: TXSample = 0.;
    let mut tmp: [TXSample; 4] = [0.; 4];
    let mut sf: TXComplex = TXComplex { re: 0., im: 0. };
    let mut sl: TXComplex = TXComplex { re: 0., im: 0. };
    ((*s).fn_0[0]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0),
        _dst,
        _src,
        size_of::<TXComplex>() as c_ulong as ptrdiff_t,
    );
    tmp_dc = (*data.offset(0)).re;
    (*data.offset(0)).re = tmp_dc + (*data.offset(0)).im;
    tmp_dc -= (*data.offset(0)).im;
    (*data.offset(0)).re = *fact.offset(0) * (*data.offset(0)).re;
    tmp_dc *= *fact.offset(1);
    (*data.offset(len4 as isize)).re = *fact.offset(2) * (*data.offset(len4 as isize)).re;
    if 1 == 0 {
        (*data.offset(len4 as isize)).im = *fact.offset(3) * (*data.offset(len4 as isize)).im;
    } else {
        sf = *data.offset(len4 as isize);
        sl = *data.offset((len4 + 1) as isize);
        if AV_TX_REAL_TO_IMAGINARY as c_int == AV_TX_REAL_TO_REAL as c_int {
            tmp[0] = *fact.offset(4) * (sf.re + sl.re);
        } else {
            tmp[0] = *fact.offset(5) * (sf.im - sl.im);
        }
        tmp[1] = *fact.offset(6) * (sf.im + sl.im);
        tmp[2] = *fact.offset(7) * (sf.re - sl.re);
        if AV_TX_REAL_TO_IMAGINARY as c_int == AV_TX_REAL_TO_REAL as c_int {
            tmp[3] = tmp[1] * *tcos.offset(len4 as isize) - tmp[2] * *tsin.offset(len4 as isize);
            tmp_mid = tmp[0] - tmp[3];
        } else {
            tmp[3] = tmp[1] * *tsin.offset(len4 as isize) + tmp[2] * *tcos.offset(len4 as isize);
            tmp_mid = tmp[0] + tmp[3];
        }
    }
    let mut i: c_int = 1;
    while i <= len4 {
        let mut tmp_0: [TXSample; 4] = [0.; 4];
        let sf_0: TXComplex = *data.offset(i as isize);
        let sl_0: TXComplex = *data.offset((len2 - i) as isize);
        if AV_TX_REAL_TO_IMAGINARY as c_int == AV_TX_REAL_TO_REAL as c_int {
            tmp_0[0] = *fact.offset(4) * (sf_0.re + sl_0.re);
        } else {
            tmp_0[0] = *fact.offset(5) * (sf_0.im - sl_0.im);
        }
        tmp_0[1] = *fact.offset(6) * (sf_0.im + sl_0.im);
        tmp_0[2] = *fact.offset(7) * (sf_0.re - sl_0.re);
        if AV_TX_REAL_TO_IMAGINARY as c_int == AV_TX_REAL_TO_REAL as c_int {
            tmp_0[3] = tmp_0[1] * *tcos.offset(i as isize) - tmp_0[2] * *tsin.offset(i as isize);
            *out.offset(i as isize) = tmp_0[0] + tmp_0[3];
            *out.offset((len - i) as isize) = tmp_0[0] - tmp_0[3];
        } else {
            tmp_0[3] = tmp_0[1] * *tsin.offset(i as isize) + tmp_0[2] * *tcos.offset(i as isize);
            *out.offset((i - 1) as isize) = tmp_0[3] - tmp_0[0];
            *out.offset((len - i - 1) as isize) = tmp_0[0] + tmp_0[3];
        }
        i += 1;
        i;
    }
    let mut i_0: c_int = 1;
    while i_0
        < len4 + (AV_TX_REAL_TO_IMAGINARY as c_int == AV_TX_REAL_TO_IMAGINARY as c_int) as c_int
    {
        *out.offset((len2 - i_0) as isize) = *out.offset((len - i_0) as isize);
        i_0 += 1;
        i_0;
    }
    if AV_TX_REAL_TO_IMAGINARY as c_int == AV_TX_REAL_TO_REAL as c_int {
        *out.offset(len2 as isize) = tmp_dc;
        *out.offset((len4 + 1) as isize) = tmp_mid * *fact.offset(5);
    } else {
        *out.offset(len4 as isize) = tmp_mid;
    };
}
#[cold]
unsafe extern "C" fn ff_tx_dct_init_float_c(
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
        len *= 2;
        (*s).len *= 2;
        rsc = (rsc as c_double * 0.5) as c_float;
    }
    ret = ff_tx_init_subtx(
        s,
        AV_TX_FLOAT_RDFT,
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
        float: av_malloc(((len / 2 * 3) as c_ulong).wrapping_mul(size_of::<TXSample>() as c_ulong))
            as *mut TXComplex,
    };
    if ((*s).exp).float.is_null() {
        return -12;
    }
    tab = (*s).exp.float as *mut TXSample;
    freq = PI / (len * 2) as c_double;
    let mut i: c_int = 0;
    while i < len {
        *tab.offset(i as isize) =
            (cos(i as c_double * freq) * ((inv == 0) as c_int + 1) as c_double) as TXSample;
        i += 1;
        i;
    }
    if inv != 0 {
        let mut i_0: c_int = 0;
        while i_0 < len / 2 {
            *tab.offset((len + i_0) as isize) =
                (0.5 / sin((2 * i_0 + 1) as c_double * freq)) as TXSample;
            i_0 += 1;
            i_0;
        }
    } else {
        let mut i_1: c_int = 0;
        while i_1 < len / 2 {
            *tab.offset((len + i_1) as isize) =
                cos((len - 2 * i_1 - 1) as c_double * freq) as TXSample;
            i_1 += 1;
            i_1;
        }
    }
    0
}
unsafe extern "C" fn ff_tx_dctII_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    _stride: ptrdiff_t,
) {
    let dst: *mut TXSample = _dst as *mut TXSample;
    let src: *mut TXSample = _src as *mut TXSample;
    let len: c_int = (*s).len;
    let len2: c_int = len >> 1;
    let exp: *const TXSample = (*s).exp.float as *mut c_void as *const TXSample;
    let mut next: TXSample = 0.;
    let mut tmp1: TXSample = 0.;
    let mut tmp2: TXSample = 0.;
    let mut i: c_int = 0;
    while i < len2 {
        let in1: TXSample = *src.offset(i as isize);
        let in2: TXSample = *src.offset((len - i - 1) as isize);
        let s_0: TXSample = *exp.offset((len + i) as isize);
        tmp1 = ((in1 + in2) as c_double * 0.5) as TXSample;
        tmp2 = (in1 - in2) * s_0;
        *src.offset(i as isize) = tmp1 + tmp2;
        *src.offset((len - i - 1) as isize) = tmp1 - tmp2;
        i += 1;
        i;
    }
    ((*s).fn_0[0]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0),
        dst as *mut c_void,
        src as *mut c_void,
        size_of::<TXComplex>() as c_ulong as ptrdiff_t,
    );
    next = *dst.offset(len as isize);
    let mut i_0: c_int = len - 2;
    while i_0 > 0 {
        let mut tmp: TXSample = 0.;
        tmp = *exp.offset((len - i_0) as isize) * *dst.offset((i_0 + 0) as isize)
            - *exp.offset(i_0 as isize) * *dst.offset((i_0 + 1) as isize);
        *dst.offset(i_0 as isize) = *exp.offset((len - i_0) as isize)
            * *dst.offset((i_0 + 1) as isize)
            + *exp.offset(i_0 as isize) * *dst.offset((i_0 + 0) as isize);
        *dst.offset((i_0 + 1) as isize) = next;
        next += tmp;
        i_0 -= 2;
    }
    *dst.offset(0) = *exp.offset(0) * *dst.offset(0);
    *dst.offset(1) = next;
}
unsafe extern "C" fn ff_tx_dctIII_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    _stride: ptrdiff_t,
) {
    let dst: *mut TXSample = _dst as *mut TXSample;
    let src: *mut TXSample = _src as *mut TXSample;
    let len: c_int = (*s).len;
    let len2: c_int = len >> 1;
    let exp: *const TXSample = (*s).exp.float as *mut c_void as *const TXSample;
    let mut tmp1: TXSample = 0.;
    let mut tmp2: TXSample = 2. * *src.offset((len - 1) as isize);
    *src.offset(len as isize) = tmp2;
    let mut i: c_int = len - 2;
    while i >= 2 {
        let val1: TXSample = *src.offset((i - 0) as isize);
        let val2: TXSample = *src.offset((i - 1) as isize) - *src.offset((i + 1) as isize);
        *src.offset((i + 1) as isize) =
            *exp.offset((len - i) as isize) * val1 - *exp.offset(i as isize) * val2;
        *src.offset(i as isize) =
            *exp.offset((len - i) as isize) * val2 + *exp.offset(i as isize) * val1;
        i -= 2;
    }
    ((*s).fn_0[0]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0),
        dst as *mut c_void,
        src as *mut c_void,
        size_of::<c_float>() as c_ulong as ptrdiff_t,
    );
    let mut i_0: c_int = 0;
    while i_0 < len2 {
        let in1: TXSample = *dst.offset(i_0 as isize);
        let in2: TXSample = *dst.offset((len - i_0 - 1) as isize);
        let c: TXSample = *exp.offset((len + i_0) as isize);
        tmp1 = in1 + in2;
        tmp2 = in1 - in2;
        tmp2 *= c;
        *dst.offset(i_0 as isize) = tmp1 + tmp2;
        *dst.offset((len - i_0 - 1) as isize) = tmp1 - tmp2;
        i_0 += 1;
        i_0;
    }
}
static mut ff_tx_dctII_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"dctII_float_c".as_ptr(),
    function: Some(
        ff_tx_dctII_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_DCT,
    flags: ((AV_TX_UNALIGNED as c_int | AV_TX_INPLACE as c_int) as c_ulonglong
        | (1 as c_ulonglong) << 63
        | (1 as c_ulonglong) << 59) as c_ulong,
    factors: [2, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 0,
    min_len: 2,
    max_len: -1,
    init: Some(
        ff_tx_dct_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

static mut ff_tx_dctIII_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"dctIII_float_c".as_ptr(),
    function: Some(
        ff_tx_dctIII_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_DCT,
    flags: ((AV_TX_UNALIGNED as c_int | AV_TX_INPLACE as c_int) as c_ulonglong
        | (1 as c_ulonglong) << 63
        | (1 as c_ulonglong) << 60) as c_ulong,
    factors: [2, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 0,
    min_len: 2,
    max_len: -1,
    init: Some(
        ff_tx_dct_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

#[cold]
unsafe extern "C" fn ff_tx_dcstI_init_float_c(
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
        len *= 2;
        (*s).len *= 2;
        rsc = (rsc as c_double * 0.5) as c_float;
    }
    flags |= (if (*cd).type_0 as c_uint == AV_TX_FLOAT_DCT_I as c_int as c_uint {
        AV_TX_REAL_TO_REAL as c_int
    } else {
        AV_TX_REAL_TO_IMAGINARY as c_int
    }) as c_ulong;
    ret = ff_tx_init_subtx(
        s,
        AV_TX_FLOAT_RDFT,
        flags,
        std::ptr::null_mut::<FFTXCodeletOptions>(),
        (len - 1 + 2 * ((*cd).type_0 as c_uint == AV_TX_FLOAT_DST_I as c_int as c_uint) as c_int)
            * 2,
        0,
        &mut rsc as *mut c_float as *const c_void,
    );
    if ret != 0 {
        return ret;
    }
    (*s).tmp = AVTXNum {
        float: av_mallocz(
            (((len + 1) * 2) as c_ulong).wrapping_mul(size_of::<TXSample>() as c_ulong),
        ) as *mut TXComplex,
    };
    if ((*s).tmp).float.is_null() {
        return -12;
    }
    0
}
unsafe extern "C" fn ff_tx_dctI_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let dst: *mut TXSample = _dst as *mut TXSample;
    let src: *mut TXSample = _src as *mut TXSample;
    let len: c_int = (*s).len - 1;
    let tmp: *mut TXSample = (*s).tmp.float as *mut TXSample;
    stride = (stride as c_ulong).wrapping_div(size_of::<TXSample>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    let mut i: c_int = 0;
    while i < len {
        let fresh40 = &mut (*tmp.offset((2 * len - i) as isize));
        *fresh40 = *src.offset((i as c_long * stride) as isize);
        *tmp.offset(i as isize) = *fresh40;
        i += 1;
        i;
    }
    *tmp.offset(len as isize) = *src.offset((len as c_long * stride) as isize);
    ((*s).fn_0[0]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0),
        dst as *mut c_void,
        tmp as *mut c_void,
        size_of::<TXSample>() as c_ulong as ptrdiff_t,
    );
}
unsafe extern "C" fn ff_tx_dstI_float_c(
    s: *mut AVTXContext,
    mut _dst: *mut c_void,
    mut _src: *mut c_void,
    mut stride: ptrdiff_t,
) {
    let dst: *mut TXSample = _dst as *mut TXSample;
    let src: *mut TXSample = _src as *mut TXSample;
    let len: c_int = (*s).len + 1;
    let tmp: *mut TXSample = (*s).tmp.float as *mut c_void as *mut TXSample;
    stride = (stride as c_ulong).wrapping_div(size_of::<TXSample>() as c_ulong) as ptrdiff_t
        as ptrdiff_t;
    *tmp.offset(0) = 0 as TXSample;
    let mut i: c_int = 1;
    while i < len {
        let a: TXSample = *src.offset(((i - 1) as c_long * stride) as isize);
        *tmp.offset(i as isize) = -a;
        *tmp.offset((2 * len - i) as isize) = a;
        i += 1;
        i;
    }
    *tmp.offset(len as isize) = 0 as TXSample;
    ((*s).fn_0[0]).expect("non-null function pointer")(
        &mut *((*s).sub).offset(0),
        dst as *mut c_void,
        tmp as *mut c_void,
        size_of::<c_float>() as c_ulong as ptrdiff_t,
    );
}
static mut ff_tx_dctI_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"dctI_float_c".as_ptr(),
    function: Some(
        ff_tx_dctI_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_DCT_I,
    flags: ((AV_TX_UNALIGNED as c_int | AV_TX_INPLACE as c_int) as c_ulonglong
        | (1 as c_ulonglong) << 63) as c_ulong,
    factors: [2, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 2,
    min_len: 2,
    max_len: -1,
    init: Some(
        ff_tx_dcstI_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

static mut ff_tx_dstI_def_float_c: FFTXCodelet = FFTXCodelet {
    name: c"dstI_float_c".as_ptr(),
    function: Some(
        ff_tx_dstI_float_c
            as unsafe extern "C" fn(*mut AVTXContext, *mut c_void, *mut c_void, ptrdiff_t) -> (),
    ),
    type_0: AV_TX_FLOAT_DST_I,
    flags: ((AV_TX_UNALIGNED as c_int | AV_TX_INPLACE as c_int) as c_ulonglong
        | (1 as c_ulonglong) << 63) as c_ulong,
    factors: [2, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 2,
    min_len: 2,
    max_len: -1,
    init: Some(
        ff_tx_dcstI_init_float_c
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
    cpu_flags: 0,
    prio: FF_TX_PRIO_BASE as c_int,
};

pub unsafe extern "C" fn ff_tx_mdct_gen_exp_float(
    s: *mut AVTXContext,
    pre_tab: *mut c_int,
) -> c_int {
    let mut off: c_int = 0;
    let len4: c_int = (*s).len >> 1;
    let mut scale: c_double = (*s).scale_d;
    let theta: c_double = (if scale < 0. { len4 } else { 0 }) as c_double + 1. / 8.;
    let alloc: c_ulong = (if !pre_tab.is_null() { 2 * len4 } else { len4 }) as c_ulong;
    (*s).exp = AVTXNum {
        float: av_malloc_array(alloc, size_of::<TXComplex>() as c_ulong) as *mut TXComplex,
    };
    if ((*s).exp).float.is_null() {
        return -12;
    }
    scale = sqrt(fabs(scale));
    if !pre_tab.is_null() {
        off = len4;
    }
    let mut i: c_int = 0;
    while i < len4 {
        let alpha: c_double = FRAC_PI_2 * (i as c_double + theta) / len4 as c_double;
        *((*s).exp).float.offset((off + i) as isize) = {
            AVComplexFloat {
                re: (cos(alpha) * scale) as c_float,
                im: (sin(alpha) * scale) as c_float,
            }
        };
        i += 1;
        i;
    }
    if !pre_tab.is_null() {
        let mut i_0: c_int = 0;
        while i_0 < len4 {
            *((*s).exp).float.offset(i_0 as isize) = *((*s).exp)
                .float
                .offset((len4 + *pre_tab.offset(i_0 as isize)) as isize);
            i_0 += 1;
            i_0;
        }
    }
    0
}

pub static mut ff_tx_codelet_list_float_c: [*const FFTXCodelet; 63] = unsafe {
    [
        addr_of!(ff_tx_fft2_ns_def_float_c),
        addr_of!(ff_tx_fft4_ns_def_float_c),
        addr_of!(ff_tx_fft8_ns_def_float_c),
        addr_of!(ff_tx_fft16_ns_def_float_c),
        addr_of!(ff_tx_fft32_ns_def_float_c),
        addr_of!(ff_tx_fft64_ns_def_float_c),
        addr_of!(ff_tx_fft128_ns_def_float_c),
        addr_of!(ff_tx_fft256_ns_def_float_c),
        addr_of!(ff_tx_fft512_ns_def_float_c),
        addr_of!(ff_tx_fft1024_ns_def_float_c),
        addr_of!(ff_tx_fft2048_ns_def_float_c),
        addr_of!(ff_tx_fft4096_ns_def_float_c),
        addr_of!(ff_tx_fft8192_ns_def_float_c),
        addr_of!(ff_tx_fft16384_ns_def_float_c),
        addr_of!(ff_tx_fft32768_ns_def_float_c),
        addr_of!(ff_tx_fft65536_ns_def_float_c),
        addr_of!(ff_tx_fft131072_ns_def_float_c),
        addr_of!(ff_tx_fft262144_ns_def_float_c),
        addr_of!(ff_tx_fft524288_ns_def_float_c),
        addr_of!(ff_tx_fft1048576_ns_def_float_c),
        addr_of!(ff_tx_fft2097152_ns_def_float_c),
        addr_of!(ff_tx_fft3_ns_def_float_c),
        addr_of!(ff_tx_fft5_ns_def_float_c),
        addr_of!(ff_tx_fft7_ns_def_float_c),
        addr_of!(ff_tx_fft9_ns_def_float_c),
        addr_of!(ff_tx_fft15_ns_def_float_c),
        addr_of!(ff_tx_fft3_fwd_def_float_c),
        addr_of!(ff_tx_fft5_fwd_def_float_c),
        addr_of!(ff_tx_fft7_fwd_def_float_c),
        addr_of!(ff_tx_fft9_fwd_def_float_c),
        addr_of!(ff_tx_fft_def_float_c),
        addr_of!(ff_tx_fft_inplace_def_float_c),
        addr_of!(ff_tx_fft_inplace_small_def_float_c),
        addr_of!(ff_tx_fft_pfa_def_float_c),
        addr_of!(ff_tx_fft_pfa_ns_def_float_c),
        addr_of!(ff_tx_fft_naive_def_float_c),
        addr_of!(ff_tx_fft_naive_small_def_float_c),
        addr_of!(ff_tx_mdct_fwd_def_float_c),
        addr_of!(ff_tx_mdct_inv_def_float_c),
        addr_of!(ff_tx_mdct_pfa_3xM_fwd_def_float_c),
        addr_of!(ff_tx_mdct_pfa_5xM_fwd_def_float_c),
        addr_of!(ff_tx_mdct_pfa_7xM_fwd_def_float_c),
        addr_of!(ff_tx_mdct_pfa_9xM_fwd_def_float_c),
        addr_of!(ff_tx_mdct_pfa_15xM_fwd_def_float_c),
        addr_of!(ff_tx_mdct_pfa_3xM_inv_def_float_c),
        addr_of!(ff_tx_mdct_pfa_5xM_inv_def_float_c),
        addr_of!(ff_tx_mdct_pfa_7xM_inv_def_float_c),
        addr_of!(ff_tx_mdct_pfa_9xM_inv_def_float_c),
        addr_of!(ff_tx_mdct_pfa_15xM_inv_def_float_c),
        addr_of!(ff_tx_mdct_naive_fwd_def_float_c),
        addr_of!(ff_tx_mdct_naive_inv_def_float_c),
        addr_of!(ff_tx_mdct_inv_full_def_float_c),
        addr_of!(ff_tx_rdft_r2c_def_float_c),
        addr_of!(ff_tx_rdft_r2r_def_float_c),
        addr_of!(ff_tx_rdft_r2r_mod2_def_float_c),
        addr_of!(ff_tx_rdft_r2i_def_float_c),
        addr_of!(ff_tx_rdft_r2i_mod2_def_float_c),
        addr_of!(ff_tx_rdft_c2r_def_float_c),
        addr_of!(ff_tx_dctII_def_float_c),
        addr_of!(ff_tx_dctIII_def_float_c),
        addr_of!(ff_tx_dctI_def_float_c),
        addr_of!(ff_tx_dstI_def_float_c),
        ptr::null(),
    ]
};
unsafe extern "C" fn run_static_initializers() {
    ff_tx_rdft_r2c_def_float_c = {
        FFTXCodelet {
            name: c"rdft_r2c_float_c".as_ptr(),
            function: Some(
                ff_tx_rdft_r2c_float_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_FLOAT_RDFT,
            flags: ((AV_TX_UNALIGNED as c_int | AV_TX_INPLACE as c_int) as c_ulonglong
                | (1 as c_ulonglong) << 63
                | (if 0 != 0 {
                    (1 as c_ulonglong) << 60
                } else {
                    (1 as c_ulonglong) << 59
                })) as c_ulong,
            factors: [4, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 2,
            min_len: 4,
            max_len: -1,
            init: Some(
                ff_tx_rdft_init_float_c
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
            cpu_flags: 0,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    };
    ff_tx_rdft_c2r_def_float_c = {
        FFTXCodelet {
            name: c"rdft_c2r_float_c".as_ptr(),
            function: Some(
                ff_tx_rdft_c2r_float_c
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut c_void,
                        *mut c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
            type_0: AV_TX_FLOAT_RDFT,
            flags: ((AV_TX_UNALIGNED as c_int | AV_TX_INPLACE as c_int) as c_ulonglong
                | (1 as c_ulonglong) << 63
                | (if 1 != 0 {
                    (1 as c_ulonglong) << 60
                } else {
                    (1 as c_ulonglong) << 59
                })) as c_ulong,
            factors: [4, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            nb_factors: 2,
            min_len: 4,
            max_len: -1,
            init: Some(
                ff_tx_rdft_init_float_c
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
            cpu_flags: 0,
            prio: FF_TX_PRIO_BASE as c_int,
        }
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
