#![deny(dead_code)]

mod tx_double;
mod tx_float;
mod tx_int32;

use crate::types::*;
use std::{
    alloc::{alloc, alloc_zeroed, Layout},
    ptr,
};

use ::libc;

use self::{
    tx_double::ff_tx_codelet_list_double_c, tx_float::ff_tx_codelet_list_float_c,
    tx_int32::ff_tx_codelet_list_int32_c,
};
extern "C" {
    fn av_get_cpu_flags() -> libc::c_int;
    fn av_fast_realloc(
        ptr: *mut libc::c_void,
        size: *mut libc::c_uint,
        min_size: size_t,
    ) -> *mut libc::c_void;
    fn av_freep(ptr: *mut libc::c_void);
    fn av_free(ptr: *mut libc::c_void);
}
#[inline(always)]
unsafe fn ff_ctz_c(v: libc::c_int) -> libc::c_int {
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
    debruijn_ctz32[(((v & -v) as libc::c_uint).wrapping_mul(0x77cb531 as libc::c_uint)
        >> 27 as libc::c_int) as usize] as libc::c_int
}

unsafe fn reset_ctx(s: *mut AVTXContext, free_sub: libc::c_int) {
    if s.is_null() {
        return;
    }
    if !((*s).sub).is_null() {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            reset_ctx(
                &mut *((*s).sub).offset(i as isize),
                free_sub + 1 as libc::c_int,
            );
            i += 1;
            i;
        }
    }
    if !((*s).cd_self).is_null() && ((*(*s).cd_self).uninit).is_some() {
        ((*(*s).cd_self).uninit).expect("non-null function pointer")(s);
    }
    // TODO: this leaks 🚿
    if free_sub != 0 {
        // av_freep(&mut (*s).sub as *mut *mut AVTXContext as *mut libc::c_void);
    }
    // av_freep(&mut (*s).map as *mut *mut libc::c_int as *mut libc::c_void);
    // av_freep(&mut (*s).exp as *mut *mut libc::c_void as *mut libc::c_void);
    // av_freep(&mut (*s).tmp as *mut *mut libc::c_void as *mut libc::c_void);
    (*s).nb_sub = 0 as libc::c_int;
    (*s).opaque = std::ptr::null_mut::<libc::c_void>();
    (*s).fn_0[0] = None;
}

#[cold]
pub(crate) unsafe fn av_tx_uninit(ctx: *mut *mut AVTXContext) {
    if (*ctx).is_null() {
        return;
    }
    reset_ctx(*ctx, 1 as libc::c_int);
    av_freep(ctx as *mut libc::c_void);
}
#[cold]
unsafe extern "C" fn ff_tx_null_init(
    s: *mut AVTXContext,
    _cd: *const FFTXCodelet,
    _flags: uint64_t,
    _opts: *mut FFTXCodeletOptions,
    _len: libc::c_int,
    _inv: libc::c_int,
    _scale: *const libc::c_void,
) -> libc::c_int {
    if (*s).type_0 as libc::c_uint == AV_TX_FLOAT_MDCT as libc::c_int as libc::c_uint
        || (*s).type_0 as libc::c_uint == AV_TX_DOUBLE_MDCT as libc::c_int as libc::c_uint
        || (*s).type_0 as libc::c_uint == AV_TX_INT32_MDCT as libc::c_int as libc::c_uint
        || ((*s).type_0 as libc::c_uint == AV_TX_FLOAT_RDFT as libc::c_int as libc::c_uint
            || (*s).type_0 as libc::c_uint == AV_TX_DOUBLE_RDFT as libc::c_int as libc::c_uint
            || (*s).type_0 as libc::c_uint == AV_TX_INT32_RDFT as libc::c_int as libc::c_uint)
    {
        return -(22 as libc::c_int);
    }
    0 as libc::c_int
}
unsafe extern "C" fn ff_tx_null(
    _s: *mut AVTXContext,
    mut _out: *mut libc::c_void,
    mut _in: *mut libc::c_void,
    stride: ptrdiff_t,
) {
    ptr::copy_nonoverlapping(_in, _out, stride as usize);
}
static mut ff_tx_null_def: FFTXCodelet = unsafe {
    {
        FFTXCodelet {
            name: b"null\0" as *const u8 as *const libc::c_char,
            function: Some(ff_tx_null),
            type_0: 2147483647 as AVTXType,
            flags: (AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 62 as libc::c_int
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
            nb_factors: 0,
            min_len: 1 as libc::c_int,
            max_len: 1 as libc::c_int,
            init: Some(ff_tx_null_init),
            uninit: None,
            cpu_flags: 0 as libc::c_int,
            prio: FF_TX_PRIO_MAX as libc::c_int,
        }
    }
};
static mut ff_tx_null_list: [*const FFTXCodelet; 2] = unsafe {
    [
        &ff_tx_null_def as *const FFTXCodelet,
        0 as *const FFTXCodelet,
    ]
};
static mut codelet_list: [*const *const FFTXCodelet; 4] = unsafe {
    [
        ff_tx_codelet_list_float_c.as_ptr(),
        ff_tx_codelet_list_double_c.as_ptr(),
        ff_tx_codelet_list_int32_c.as_ptr(),
        ff_tx_null_list.as_ptr(),
    ]
};
static mut codelet_list_num: libc::c_int = 0;
static mut cpu_slow_mask: libc::c_int = 0x40000000 as libc::c_int
    | 0x20000000 as libc::c_int
    | 0x10000000 as libc::c_int
    | 0x4000000 as libc::c_int
    | 0x8000000 as libc::c_int
    | 0x2000000 as libc::c_int;
static mut cpu_slow_penalties: [[libc::c_int; 2]; 6] = [
    [
        0x40000000 as libc::c_int,
        1 as libc::c_int + 64 as libc::c_int,
    ],
    [
        0x20000000 as libc::c_int,
        1 as libc::c_int + 64 as libc::c_int,
    ],
    [
        0x4000000 as libc::c_int,
        1 as libc::c_int + 64 as libc::c_int,
    ],
    [
        0x10000000 as libc::c_int,
        1 as libc::c_int + 128 as libc::c_int,
    ],
    [
        0x8000000 as libc::c_int,
        1 as libc::c_int + 128 as libc::c_int,
    ],
    [
        0x2000000 as libc::c_int,
        1 as libc::c_int + 32 as libc::c_int,
    ],
];
unsafe fn get_codelet_prio(
    cd: *const FFTXCodelet,
    cpu_flags: libc::c_int,
    len: libc::c_int,
) -> libc::c_int {
    let mut prio: libc::c_int = (*cd).prio;
    let mut max_factor: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[[libc::c_int; 2]; 6]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<[libc::c_int; 2]>() as libc::c_ulong)
    {
        if cpu_flags & (*cd).cpu_flags & cpu_slow_penalties[i as usize][0 as libc::c_int as usize]
            != 0
        {
            prio -= cpu_slow_penalties[i as usize][1 as libc::c_int as usize];
        }
        i += 1;
        i;
    }
    if (*cd).flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 62 as libc::c_int != 0
        && (*cd).flags & AV_TX_UNALIGNED as libc::c_int as libc::c_ulong == 0
    {
        prio += 64 as libc::c_int;
    }
    if len == (*cd).min_len && len == (*cd).max_len {
        prio += 64 as libc::c_int;
    }
    if (*cd).flags as libc::c_ulonglong
        & ((1 as libc::c_ulonglong) << 59 as libc::c_int
            | (1 as libc::c_ulonglong) << 60 as libc::c_int)
        != 0
    {
        prio += 64 as libc::c_int;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 4 as libc::c_int {
        max_factor = if (*cd).factors[i_0 as usize] > max_factor {
            (*cd).factors[i_0 as usize]
        } else {
            max_factor
        };
        i_0 += 1;
        i_0;
    }
    if max_factor != 0 {
        prio += 16 as libc::c_int * max_factor;
    }
    prio
}

unsafe fn cmp_matches(a: *mut TXCodeletMatch, b: *mut TXCodeletMatch) -> libc::c_int {
    ((*b).prio > (*a).prio) as libc::c_int - ((*b).prio < (*a).prio) as libc::c_int
}
#[inline]
unsafe fn check_cd_factors(cd: *const FFTXCodelet, mut len: libc::c_int) -> libc::c_int {
    let mut matches: libc::c_int = 0 as libc::c_int;
    let mut any_flag: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        let factor: libc::c_int = (*cd).factors[i as usize];
        if factor == -(1 as libc::c_int) {
            any_flag = 1 as libc::c_int;
            matches += 1;
            matches;
        } else {
            if len <= 1 as libc::c_int || factor == 0 {
                break;
            }
            if factor == 2 as libc::c_int {
                let bits_2: libc::c_int = ff_ctz_c(len);
                if bits_2 != 0 {
                    len >>= bits_2;
                    matches += 1;
                    matches;
                }
            } else {
                let mut res: libc::c_int = len % factor;
                if res == 0 {
                    while res == 0 {
                        len /= factor;
                        res = len % factor;
                    }
                    matches += 1;
                    matches;
                }
            }
        }
        i += 1;
        i;
    }
    ((*cd).nb_factors <= matches && (any_flag != 0 || len == 1 as libc::c_int)) as libc::c_int
}

#[cold]
unsafe fn ff_tx_init_subtx(
    s: *mut AVTXContext,
    type_0: AVTXType,
    flags: uint64_t,
    opts: *mut FFTXCodeletOptions,
    len: libc::c_int,
    inv: libc::c_int,
    scale: *const libc::c_void,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut sub: *mut AVTXContext = std::ptr::null_mut::<AVTXContext>();
    let mut cd_tmp: *mut TXCodeletMatch = std::ptr::null_mut::<TXCodeletMatch>();
    let mut cd_matches: *mut TXCodeletMatch = std::ptr::null_mut::<TXCodeletMatch>();
    let mut cd_matches_size: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut codelet_list_idx: libc::c_int = codelet_list_num;
    let mut nb_cd_matches: libc::c_int = 0 as libc::c_int;
    // let mut bp: AVBPrint = AVBPrint {
    //     str_0: 0 as *mut libc::c_char,
    //     len: 0,
    //     size: 0,
    //     size_max: 0,
    //     reserved_internal_buffer: [0; 1],
    //     reserved_padding: [0; 1000],
    // };
    let cpu_flags: libc::c_int = av_get_cpu_flags();
    let mut req_flags: uint64_t = flags;
    let inv_req_mask: uint64_t =
        ((AV_TX_FULL_IMDCT as libc::c_int
            | AV_TX_REAL_TO_REAL as libc::c_int
            | AV_TX_REAL_TO_IMAGINARY as libc::c_int) as libc::c_ulonglong
            | (1 as libc::c_ulonglong) << 61 as libc::c_int
            | (1 as libc::c_ulonglong) << 58 as libc::c_int) as uint64_t;
    if req_flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 62 as libc::c_int != 0 {
        req_flags |= AV_TX_UNALIGNED as libc::c_int as libc::c_ulong;
    }
    if req_flags & AV_TX_INPLACE as libc::c_int as libc::c_ulong != 0
        && req_flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 63 as libc::c_int != 0
    {
        req_flags = (req_flags as libc::c_ulonglong
            & !(AV_TX_INPLACE as libc::c_int as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 63 as libc::c_int)) as uint64_t;
    }
    if req_flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 62 as libc::c_int != 0
        && req_flags & AV_TX_UNALIGNED as libc::c_int as libc::c_ulong != 0
    {
        req_flags = (req_flags as libc::c_ulonglong
            & !((1 as libc::c_ulonglong) << 62 as libc::c_int
                | AV_TX_UNALIGNED as libc::c_int as libc::c_ulonglong))
            as uint64_t;
    }
    loop {
        let fresh10 = codelet_list_idx;
        codelet_list_idx -= 1;
        if fresh10 == 0 {
            break;
        }
        let mut list: *const *const FFTXCodelet = codelet_list[codelet_list_idx as usize];
        let mut cd: *const FFTXCodelet = std::ptr::null::<FFTXCodelet>();
        loop {
            let fresh11 = list;
            list = list.offset(1);
            cd = *fresh11;
            if cd.is_null() {
                break;
            }
            if (*cd).type_0 as libc::c_uint != 2147483647 as libc::c_int as libc::c_uint
                && type_0 as libc::c_uint != (*cd).type_0 as libc::c_uint
            {
                continue;
            }
            if (*cd).flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 59 as libc::c_int != 0
                && inv != 0
                || (*cd).flags as libc::c_ulonglong
                    & ((1 as libc::c_ulonglong) << 60 as libc::c_int
                        | AV_TX_FULL_IMDCT as libc::c_int as libc::c_ulonglong)
                    != 0
                    && inv == 0
                || (*cd).flags as libc::c_ulonglong
                    & ((1 as libc::c_ulonglong) << 59 as libc::c_int
                        | AV_TX_REAL_TO_REAL as libc::c_int as libc::c_ulonglong)
                    != 0
                    && inv != 0
                || (*cd).flags as libc::c_ulonglong
                    & ((1 as libc::c_ulonglong) << 59 as libc::c_int
                        | AV_TX_REAL_TO_IMAGINARY as libc::c_int as libc::c_ulonglong)
                    != 0
                    && inv != 0
            {
                continue;
            }
            if req_flags & (*cd).flags != req_flags
                || inv_req_mask & (*cd).flags != req_flags & inv_req_mask
            {
                continue;
            }
            if len < (*cd).min_len || (*cd).max_len != -(1 as libc::c_int) && len > (*cd).max_len {
                continue;
            }
            if (*cd).cpu_flags != 0 as libc::c_int
                && cpu_flags & ((*cd).cpu_flags & !cpu_slow_mask) == 0
            {
                continue;
            }
            if check_cd_factors(cd, len) == 0 {
                continue;
            }
            cd_tmp = av_fast_realloc(
                cd_matches as *mut libc::c_void,
                &mut cd_matches_size,
                (::core::mem::size_of::<TXCodeletMatch>() as libc::c_ulong)
                    .wrapping_mul((nb_cd_matches + 1 as libc::c_int) as libc::c_ulong),
            ) as *mut TXCodeletMatch;
            if cd_tmp.is_null() {
                av_free(cd_matches as *mut libc::c_void);
                return -(12 as libc::c_int);
            }
            cd_matches = cd_tmp;
            let fresh12 = &mut (*cd_matches.offset(nb_cd_matches as isize)).cd;
            *fresh12 = cd;
            (*cd_matches.offset(nb_cd_matches as isize)).prio =
                get_codelet_prio(cd, cpu_flags, len);
            nb_cd_matches += 1;
            nb_cd_matches;
        }
    }
    // av_bprint_init(
    //     &mut bp,
    //     0 as libc::c_int as libc::c_uint,
    //     1 as libc::c_int as libc::c_uint,
    // );
    // av_bprintf(
    //     &mut bp as *mut AVBPrint,
    //     b"For transform of length %i, %s, \0" as *const u8 as *const libc::c_char,
    //     len,
    //     if inv != 0 {
    //         b"inverse\0" as *const u8 as *const libc::c_char
    //     } else {
    //         b"forward\0" as *const u8 as *const libc::c_char
    //     },
    // );
    // print_type(&mut bp, type_0);
    // av_bprintf(
    //     &mut bp as *mut AVBPrint,
    //     b", \0" as *const u8 as *const libc::c_char,
    // );
    // print_flags(&mut bp, flags);
    // av_bprintf(
    //     &mut bp as *mut AVBPrint,
    //     b", found %i matches%s\0" as *const u8 as *const libc::c_char,
    //     nb_cd_matches,
    //     if nb_cd_matches != 0 {
    //         b":\0" as *const u8 as *const libc::c_char
    //     } else {
    //         b".\0" as *const u8 as *const libc::c_char
    //     },
    // );
    if nb_cd_matches == 0 {
        return -(38 as libc::c_int);
    }
    let mut stack: [[*mut libc::c_void; 2]; 64] = [[std::ptr::null_mut::<libc::c_void>(); 2]; 64];
    let mut sp: libc::c_int = 1 as libc::c_int;
    stack[0 as libc::c_int as usize][0 as libc::c_int as usize] = cd_matches as *mut libc::c_void;
    stack[0 as libc::c_int as usize][1 as libc::c_int as usize] = cd_matches
        .offset(nb_cd_matches as isize)
        .offset(-(1 as libc::c_int as isize))
        as *mut libc::c_void;
    while sp != 0 {
        sp -= 1;
        let mut start: *mut TXCodeletMatch =
            stack[sp as usize][0 as libc::c_int as usize] as *mut TXCodeletMatch;
        let mut end: *mut TXCodeletMatch =
            stack[sp as usize][1 as libc::c_int as usize] as *mut TXCodeletMatch;
        while start < end {
            if start < end.offset(-(1 as libc::c_int as isize)) {
                let mut checksort: libc::c_int = 0 as libc::c_int;
                let mut right: *mut TXCodeletMatch = end.offset(-(2 as libc::c_int as isize));
                let mut left: *mut TXCodeletMatch = start.offset(1 as libc::c_int as isize);
                let mut mid: *mut TXCodeletMatch = start
                    .offset((end.offset_from(start) as libc::c_long >> 1 as libc::c_int) as isize);
                if cmp_matches(start, end) > 0 as libc::c_int {
                    if cmp_matches(end, mid) > 0 as libc::c_int {
                        core::ptr::swap(mid, start);
                    } else {
                        core::ptr::swap(end, start);
                    }
                } else if cmp_matches(start, mid) > 0 as libc::c_int {
                    core::ptr::swap(mid, start);
                } else {
                    checksort = 1 as libc::c_int;
                }
                if cmp_matches(mid, end) > 0 as libc::c_int {
                    core::ptr::swap(end, mid);
                    checksort = 0 as libc::c_int;
                }
                if start == end.offset(-(2 as libc::c_int as isize)) {
                    break;
                }
                let SWAP_tmp_3: TXCodeletMatch = *mid;
                *mid = *end.offset(-(1 as libc::c_int) as isize);
                *end.offset(-(1 as libc::c_int) as isize) = SWAP_tmp_3;
                while left <= right {
                    while left <= right
                        && cmp_matches(left, end.offset(-(1 as libc::c_int as isize)))
                            < 0 as libc::c_int
                    {
                        left = left.offset(1);
                        left;
                    }
                    while left <= right
                        && cmp_matches(right, end.offset(-(1 as libc::c_int as isize)))
                            > 0 as libc::c_int
                    {
                        right = right.offset(-1);
                        right;
                    }
                    if left <= right {
                        core::ptr::swap(right, left);
                        left = left.offset(1);
                        left;
                        right = right.offset(-1);
                        right;
                    }
                }
                let SWAP_tmp_5: TXCodeletMatch = *left;
                *left = *end.offset(-(1 as libc::c_int) as isize);
                *end.offset(-(1 as libc::c_int) as isize) = SWAP_tmp_5;
                if checksort != 0
                    && (mid == left.offset(-(1 as libc::c_int as isize)) || mid == left)
                {
                    mid = start;
                    while mid < end
                        && cmp_matches(mid, mid.offset(1 as libc::c_int as isize))
                            <= 0 as libc::c_int
                    {
                        mid = mid.offset(1);
                        mid;
                    }
                    if mid == end {
                        break;
                    }
                }
                if (end.offset_from(left) as libc::c_long) < left.offset_from(start) as libc::c_long
                {
                    stack[sp as usize][0 as libc::c_int as usize] = start as *mut libc::c_void;
                    let fresh13 = sp;
                    sp += 1;
                    stack[fresh13 as usize][1 as libc::c_int as usize] = right as *mut libc::c_void;
                    start = left.offset(1 as libc::c_int as isize);
                } else {
                    stack[sp as usize][0 as libc::c_int as usize] =
                        left.offset(1 as libc::c_int as isize) as *mut libc::c_void;
                    let fresh14 = sp;
                    sp += 1;
                    stack[fresh14 as usize][1 as libc::c_int as usize] = end as *mut libc::c_void;
                    end = right;
                }
            } else {
                if cmp_matches(start, end) > 0 as libc::c_int {
                    core::ptr::swap(end, start);
                }
                break;
            }
        }
    }
    // av_log(
    //     0 as *mut libc::c_void,
    //     48 as libc::c_int,
    //     b"%s\n\0" as *const u8 as *const libc::c_char,
    //     bp.str_0,
    // );
    let _i: libc::c_int = 0 as libc::c_int;
    // while i < nb_cd_matches {
    //     av_log(
    //         0 as *mut libc::c_void,
    //         48 as libc::c_int,
    //         b"    %i: \0" as *const u8 as *const libc::c_char,
    //         i + 1 as libc::c_int,
    //     );
    //     print_cd_info(
    //         (*cd_matches.offset(i as isize)).cd,
    //         (*cd_matches.offset(i as isize)).prio,
    //         0 as libc::c_int,
    //         1 as libc::c_int,
    //     );
    //     i += 1;
    //     i;
    // }
    if ((*s).sub).is_null() {
        sub = alloc_zeroed(Layout::array::<AVTXContext>(4).unwrap()).cast();
        (*s).sub = sub;
        if sub.is_null() {
            ret = -(12 as libc::c_int);
            current_block = 7391434065428304855;
        } else {
            current_block = 5706227035632243100;
        }
    } else {
        current_block = 5706227035632243100;
    }
    if let 5706227035632243100 = current_block {
        let mut i_0: libc::c_int = 0 as libc::c_int;
        loop {
            if i_0 >= nb_cd_matches {
                current_block = 16937825661756021828;
                break;
            }
            let cd_0: *const FFTXCodelet = (*cd_matches.offset(i_0 as isize)).cd;
            let sctx: *mut AVTXContext =
                &mut *((*s).sub).offset((*s).nb_sub as isize) as *mut AVTXContext;
            (*sctx).len = len;
            (*sctx).inv = inv;
            (*sctx).type_0 = type_0;
            (*sctx).flags = (*cd_0).flags | flags;
            (*sctx).cd_self = cd_0;
            (*s).fn_0[(*s).nb_sub as usize] = (*cd_0).function;
            (*s).cd[(*s).nb_sub as usize] = cd_0;
            ret = 0 as libc::c_int;
            if ((*cd_0).init).is_some() {
                ret = ((*cd_0).init).expect("non-null function pointer")(
                    sctx, cd_0, flags, opts, len, inv, scale,
                );
            }
            if ret >= 0 as libc::c_int {
                if !opts.is_null()
                    && (*opts).map_dir as libc::c_uint
                        != FF_TX_MAP_NONE as libc::c_int as libc::c_uint
                    && (*sctx).map_dir as libc::c_uint
                        == FF_TX_MAP_NONE as libc::c_int as libc::c_uint
                {
                    (*sctx).map = alloc(Layout::array::<libc::c_int>(len as usize).unwrap()).cast();
                    if ((*sctx).map).is_null() {
                        ret = -(12 as libc::c_int);
                        current_block = 7391434065428304855;
                        break;
                    } else {
                        let mut i_1: libc::c_int = 0 as libc::c_int;
                        while i_1 < len {
                            *((*sctx).map).offset(i_1 as isize) = i_1;
                            i_1 += 1;
                            i_1;
                        }
                    }
                } else if !opts.is_null()
                    && (*opts).map_dir as libc::c_uint != (*sctx).map_dir as libc::c_uint
                {
                    let tmp: *mut libc::c_int =
                        alloc(Layout::array::<libc::c_int>(len as usize).unwrap()).cast();
                    if tmp.is_null() {
                        ret = -(12 as libc::c_int);
                        current_block = 7391434065428304855;
                        break;
                    } else {
                        ptr::copy_nonoverlapping((*sctx).map, tmp, len as usize);
                        let mut i_2: libc::c_int = 0 as libc::c_int;
                        while i_2 < len {
                            *((*sctx).map).offset(*tmp.offset(i_2 as isize) as isize) = i_2;
                            i_2 += 1;
                            i_2;
                        }
                        av_free(tmp as *mut libc::c_void);
                    }
                }
                (*s).nb_sub += 1;
                (*s).nb_sub;
                current_block = 7391434065428304855;
                break;
            } else {
                (*s).fn_0[(*s).nb_sub as usize] = None;
                (*s).cd[(*s).nb_sub as usize] = std::ptr::null::<FFTXCodelet>();
                reset_ctx(sctx, 0 as libc::c_int);
                if ret == -(12 as libc::c_int) {
                    current_block = 16937825661756021828;
                    break;
                }
                i_0 += 1;
                i_0;
            }
        }
        match current_block {
            7391434065428304855 => {}
            _ => {
                if (*s).nb_sub == 0 {
                    av_freep(&mut (*s).sub as *mut *mut AVTXContext as *mut libc::c_void);
                }
            }
        }
    }
    av_free(cd_matches as *mut libc::c_void);
    ret
}

#[cold]
pub(crate) unsafe fn av_tx_init(
    ctx: *mut *mut AVTXContext,
    tx: *mut av_tx_fn,
    type_0: AVTXType,
    inv: libc::c_int,
    len: libc::c_int,
    mut scale: *const libc::c_void,
    mut flags: uint64_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut tmp: AVTXContext = {
        AVTXContext {
            len: 0 as libc::c_int,
            inv: 0,
            map: std::ptr::null_mut::<libc::c_int>(),
            exp: AVTXNum {
                void: std::ptr::null_mut(),
            },
            tmp: AVTXNum {
                void: std::ptr::null_mut(),
            },
            sub: std::ptr::null_mut::<AVTXContext>(),
            fn_0: [None; 4],
            nb_sub: 0,
            cd: [std::ptr::null::<FFTXCodelet>(); 4],
            cd_self: std::ptr::null::<FFTXCodelet>(),
            type_0: AV_TX_FLOAT_FFT,
            flags: 0,
            map_dir: FF_TX_MAP_NONE,
            scale_f: 0.,
            scale_d: 0.,
            opaque: std::ptr::null_mut::<libc::c_void>(),
        }
    };
    let default_scale_d: libc::c_double = 1.0f64;
    let default_scale_f: libc::c_float = 1.0f32;
    if len == 0
        || type_0 as libc::c_uint >= AV_TX_NB as libc::c_int as libc::c_uint
        || ctx.is_null()
        || tx.is_null()
    {
        return -(22 as libc::c_int);
    }
    if flags & AV_TX_UNALIGNED as libc::c_int as libc::c_ulong == 0 {
        flags = (flags as libc::c_ulonglong | (1 as libc::c_ulonglong) << 62 as libc::c_int)
            as uint64_t;
    }
    if flags & AV_TX_INPLACE as libc::c_int as libc::c_ulong == 0 {
        flags = (flags as libc::c_ulonglong | (1 as libc::c_ulonglong) << 63 as libc::c_int)
            as uint64_t;
    }
    if scale.is_null()
        && (type_0 as libc::c_uint == AV_TX_FLOAT_MDCT as libc::c_int as libc::c_uint
            || type_0 as libc::c_uint == AV_TX_INT32_MDCT as libc::c_int as libc::c_uint)
    {
        scale = &default_scale_f as *const libc::c_float as *const libc::c_void;
    } else if scale.is_null()
        && type_0 as libc::c_uint == AV_TX_DOUBLE_MDCT as libc::c_int as libc::c_uint
    {
        scale = &default_scale_d as *const libc::c_double as *const libc::c_void;
    }
    ret = ff_tx_init_subtx(
        &mut tmp,
        type_0,
        flags,
        std::ptr::null_mut::<FFTXCodeletOptions>(),
        len,
        inv,
        scale,
    );
    if ret < 0 as libc::c_int {
        return ret;
    }
    *ctx = &mut *(tmp.sub).offset(0 as libc::c_int as isize) as *mut AVTXContext;
    *tx = tmp.fn_0[0 as libc::c_int as usize];
    // av_log(
    //     0 as *mut libc::c_void,
    //     48 as libc::c_int,
    //     b"Transform tree:\n\0" as *const u8 as *const libc::c_char,
    // );
    // print_tx_structure(*ctx, 0 as libc::c_int);
    ret
}
unsafe fn run_static_initializers() {
    codelet_list_num = (::core::mem::size_of::<[*const *const FFTXCodelet; 4]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<*const *const FFTXCodelet>() as libc::c_ulong)
        as libc::c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe fn(); 1] = [run_static_initializers];
