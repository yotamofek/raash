#![warn(dead_code)]

mod tx_double;
mod tx_float;
mod tx_int32;

use std::{
    alloc::{alloc, alloc_zeroed, Layout},
    mem::size_of,
    ptr::{self, addr_of},
};

use libc::{
    c_char, c_double, c_float, c_int, c_long, c_uchar, c_uint, c_ulong, c_ulonglong, c_void,
};

use self::{
    tx_double::ff_tx_codelet_list_double_c, tx_float::ff_tx_codelet_list_float_c,
    tx_int32::ff_tx_codelet_list_int32_c,
};
use crate::types::*;
extern "C" {
    fn av_get_cpu_flags() -> c_int;
    fn av_fast_realloc(ptr: *mut c_void, size: *mut c_uint, min_size: c_ulong) -> *mut c_void;
    fn av_freep(ptr: *mut c_void);
    fn av_free(ptr: *mut c_void);

    fn abort() -> !;
    fn av_log(avcl: *mut c_void, level: c_int, fmt: *const c_char, _: ...);
    fn av_mallocz(size: c_ulong) -> *mut c_void;
    fn av_malloc(size: c_ulong) -> *mut c_void;
    fn av_gcd(a: c_long, b: c_long) -> c_long;

}

#[inline(always)]
unsafe fn ff_ctz_c(v: c_int) -> c_int {
    static DEBRUIJN_CTZ32: [c_uchar; 32] = [
        0, 1, 28, 2, 29, 14, 24, 3, 30, 22, 20, 15, 25, 17, 4, 8, 31, 27, 13, 23, 21, 19, 16, 7,
        26, 12, 18, 6, 11, 5, 10, 9,
    ];
    DEBRUIJN_CTZ32[(((v & -v) as c_uint).wrapping_mul(0x77cb531 as c_uint) >> 27) as usize] as c_int
}

unsafe fn reset_ctx(s: *mut AVTXContext, free_sub: c_int) {
    if s.is_null() {
        return;
    }
    if !((*s).sub).is_null() {
        let mut i: c_int = 0;
        while i < 4 {
            reset_ctx(&mut *((*s).sub).offset(i as isize), free_sub + 1);
            i += 1;
            i;
        }
    }
    if !((*s).cd_self).is_null() && ((*(*s).cd_self).uninit).is_some() {
        ((*(*s).cd_self).uninit).expect("non-null function pointer")(s);
    }
    // TODO: this leaks 🚿
    if free_sub != 0 {
        // av_freep(&mut (*s).sub as *mut *mut AVTXContext as *mut c_void);
    }
    // av_freep(&mut (*s).map as *mut *mut c_int as *mut c_void);
    // av_freep(&mut (*s).exp as *mut *mut c_void as *mut c_void);
    // av_freep(&mut (*s).tmp as *mut *mut c_void as *mut c_void);
    (*s).nb_sub = 0;
    (*s).opaque = std::ptr::null_mut::<c_void>();
    (*s).fn_0[0] = None;
}

#[cold]
pub(crate) unsafe fn av_tx_uninit(ctx: *mut *mut AVTXContext) {
    if (*ctx).is_null() {
        return;
    }
    reset_ctx(*ctx, 1);
    av_freep(ctx as *mut c_void);
}
#[cold]
unsafe extern "C" fn ff_tx_null_init(
    s: *mut AVTXContext,
    _cd: *const FFTXCodelet,
    _flags: c_ulong,
    _opts: *mut FFTXCodeletOptions,
    _len: c_int,
    _inv: c_int,
    _scale: *const c_void,
) -> c_int {
    if (*s).type_0 as c_uint == AV_TX_FLOAT_MDCT as c_int as c_uint
        || (*s).type_0 as c_uint == AV_TX_DOUBLE_MDCT as c_int as c_uint
        || (*s).type_0 as c_uint == AV_TX_INT32_MDCT as c_int as c_uint
        || ((*s).type_0 as c_uint == AV_TX_FLOAT_RDFT as c_int as c_uint
            || (*s).type_0 as c_uint == AV_TX_DOUBLE_RDFT as c_int as c_uint
            || (*s).type_0 as c_uint == AV_TX_INT32_RDFT as c_int as c_uint)
    {
        return -22;
    }
    0
}
unsafe extern "C" fn ff_tx_null(
    _s: *mut AVTXContext,
    mut _out: *mut c_void,
    mut _in: *mut c_void,
    stride: ptrdiff_t,
) {
    ptr::copy_nonoverlapping(_in, _out, stride as usize);
}
static mut ff_tx_null_def: FFTXCodelet = FFTXCodelet {
    name: c"null".as_ptr(),
    function: Some(ff_tx_null),
    type_0: 2147483647 as AVTXType,
    flags: (AV_TX_UNALIGNED as c_int as c_ulonglong
        | (1 as c_ulonglong) << 62
        | (1 as c_ulonglong) << 63
        | AV_TX_INPLACE as c_int as c_ulonglong) as c_ulong,
    factors: [-1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nb_factors: 0,
    min_len: 1,
    max_len: 1,
    init: Some(ff_tx_null_init),
    uninit: None,
    cpu_flags: 0,
    prio: FF_TX_PRIO_MAX as c_int,
};

static mut ff_tx_null_list: [*const FFTXCodelet; 2] =
    unsafe { [addr_of!(ff_tx_null_def), ptr::null()] };

static mut codelet_list: [*const *const FFTXCodelet; 4] = unsafe {
    [
        ff_tx_codelet_list_float_c.as_ptr(),
        ff_tx_codelet_list_double_c.as_ptr(),
        ff_tx_codelet_list_int32_c.as_ptr(),
        ff_tx_null_list.as_ptr(),
    ]
};
static mut codelet_list_num: c_int = 0;
static mut cpu_slow_mask: c_int = 0x40000000 as c_int
    | 0x20000000 as c_int
    | 0x10000000 as c_int
    | 0x4000000 as c_int
    | 0x8000000 as c_int
    | 0x2000000 as c_int;
static mut cpu_slow_penalties: [[c_int; 2]; 6] = [
    [0x40000000 as c_int, 1 + 64],
    [0x20000000 as c_int, 1 + 64],
    [0x4000000 as c_int, 1 + 64],
    [0x10000000 as c_int, 1 + 128],
    [0x8000000 as c_int, 1 + 128],
    [0x2000000 as c_int, 1 + 32],
];
unsafe fn get_codelet_prio(cd: *const FFTXCodelet, cpu_flags: c_int, len: c_int) -> c_int {
    let mut prio: c_int = (*cd).prio;
    let mut max_factor: c_int = 0;
    let mut i: c_int = 0;
    while (i as c_ulong)
        < (size_of::<[[c_int; 2]; 6]>() as c_ulong).wrapping_div(size_of::<[c_int; 2]>() as c_ulong)
    {
        if cpu_flags & (*cd).cpu_flags & cpu_slow_penalties[i as usize][0] != 0 {
            prio -= cpu_slow_penalties[i as usize][1];
        }
        i += 1;
        i;
    }
    if (*cd).flags as c_ulonglong & (1 as c_ulonglong) << 62 != 0
        && (*cd).flags & AV_TX_UNALIGNED as c_int as c_ulong == 0
    {
        prio += 64;
    }
    if len == (*cd).min_len && len == (*cd).max_len {
        prio += 64;
    }
    if (*cd).flags as c_ulonglong & ((1 as c_ulonglong) << 59 | (1 as c_ulonglong) << 60) != 0 {
        prio += 64;
    }
    let mut i_0: c_int = 0;
    while i_0 < 4 {
        max_factor = if (*cd).factors[i_0 as usize] > max_factor {
            (*cd).factors[i_0 as usize]
        } else {
            max_factor
        };
        i_0 += 1;
        i_0;
    }
    if max_factor != 0 {
        prio += 16 * max_factor;
    }
    prio
}

unsafe fn cmp_matches(a: *mut TXCodeletMatch, b: *mut TXCodeletMatch) -> c_int {
    ((*b).prio > (*a).prio) as c_int - ((*b).prio < (*a).prio) as c_int
}
#[inline]
unsafe fn check_cd_factors(cd: *const FFTXCodelet, mut len: c_int) -> c_int {
    let mut matches: c_int = 0;
    let mut any_flag: c_int = 0;
    let mut i: c_int = 0;
    while i < 16 {
        let factor: c_int = (*cd).factors[i as usize];
        if factor == -1 {
            any_flag = 1;
            matches += 1;
            matches;
        } else {
            if len <= 1 || factor == 0 {
                break;
            }
            if factor == 2 {
                let bits_2: c_int = ff_ctz_c(len);
                if bits_2 != 0 {
                    len >>= bits_2;
                    matches += 1;
                    matches;
                }
            } else {
                let mut res: c_int = len % factor;
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
    ((*cd).nb_factors <= matches && (any_flag != 0 || len == 1)) as c_int
}

#[cold]
unsafe fn ff_tx_init_subtx(
    s: *mut AVTXContext,
    type_0: AVTXType,
    flags: c_ulong,
    opts: *mut FFTXCodeletOptions,
    len: c_int,
    inv: c_int,
    scale: *const c_void,
) -> c_int {
    let mut current_block: u64;
    let mut ret: c_int = 0;
    let mut sub: *mut AVTXContext = std::ptr::null_mut::<AVTXContext>();
    let mut cd_tmp: *mut TXCodeletMatch = std::ptr::null_mut::<TXCodeletMatch>();
    let mut cd_matches: *mut TXCodeletMatch = std::ptr::null_mut::<TXCodeletMatch>();
    let mut cd_matches_size: c_uint = 0 as c_uint;
    let mut codelet_list_idx: c_int = codelet_list_num;
    let mut nb_cd_matches: c_int = 0;
    // let mut bp: AVBPrint = AVBPrint {
    //     str_0: 0 as *mut c_char,
    //     len: 0,
    //     size: 0,
    //     size_max: 0,
    //     reserved_internal_buffer: [0; 1],
    //     reserved_padding: [0; 1000],
    // };
    let cpu_flags: c_int = av_get_cpu_flags();
    let mut req_flags: c_ulong = flags;
    let inv_req_mask: c_ulong = ((AV_TX_FULL_IMDCT as c_int
        | AV_TX_REAL_TO_REAL as c_int
        | AV_TX_REAL_TO_IMAGINARY as c_int) as c_ulonglong
        | (1 as c_ulonglong) << 61
        | (1 as c_ulonglong) << 58) as c_ulong;
    if req_flags as c_ulonglong & (1 as c_ulonglong) << 62 != 0 {
        req_flags |= AV_TX_UNALIGNED as c_int as c_ulong;
    }
    if req_flags & AV_TX_INPLACE as c_int as c_ulong != 0
        && req_flags as c_ulonglong & (1 as c_ulonglong) << 63 != 0
    {
        req_flags = (req_flags as c_ulonglong
            & !(AV_TX_INPLACE as c_int as c_ulonglong | (1 as c_ulonglong) << 63))
            as c_ulong;
    }
    if req_flags as c_ulonglong & (1 as c_ulonglong) << 62 != 0
        && req_flags & AV_TX_UNALIGNED as c_int as c_ulong != 0
    {
        req_flags = (req_flags as c_ulonglong
            & !((1 as c_ulonglong) << 62 | AV_TX_UNALIGNED as c_int as c_ulonglong))
            as c_ulong;
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
            if (*cd).type_0 as c_uint != 2147483647 as c_uint
                && type_0 as c_uint != (*cd).type_0 as c_uint
            {
                continue;
            }
            if (*cd).flags as c_ulonglong & (1 as c_ulonglong) << 59 != 0 && inv != 0
                || (*cd).flags as c_ulonglong
                    & ((1 as c_ulonglong) << 60 | AV_TX_FULL_IMDCT as c_int as c_ulonglong)
                    != 0
                    && inv == 0
                || (*cd).flags as c_ulonglong
                    & ((1 as c_ulonglong) << 59 | AV_TX_REAL_TO_REAL as c_int as c_ulonglong)
                    != 0
                    && inv != 0
                || (*cd).flags as c_ulonglong
                    & ((1 as c_ulonglong) << 59 | AV_TX_REAL_TO_IMAGINARY as c_int as c_ulonglong)
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
            if len < (*cd).min_len || (*cd).max_len != -1 && len > (*cd).max_len {
                continue;
            }
            if (*cd).cpu_flags != 0 && cpu_flags & ((*cd).cpu_flags & !cpu_slow_mask) == 0 {
                continue;
            }
            if check_cd_factors(cd, len) == 0 {
                continue;
            }
            cd_tmp = av_fast_realloc(
                cd_matches as *mut c_void,
                &mut cd_matches_size,
                (size_of::<TXCodeletMatch>() as c_ulong)
                    .wrapping_mul((nb_cd_matches + 1) as c_ulong),
            ) as *mut TXCodeletMatch;
            if cd_tmp.is_null() {
                av_free(cd_matches as *mut c_void);
                return -12;
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
    //     0 as c_uint,
    //     1 as c_uint,
    // );
    // av_bprintf(
    //     &mut bp as *mut AVBPrint,
    //     b"For transform of length %i, %s, \0" as *const u8 as *const c_char,
    //     len,
    //     if inv != 0 {
    //         b"inverse\0" as *const u8 as *const c_char
    //     } else {
    //         b"forward\0" as *const u8 as *const c_char
    //     },
    // );
    // print_type(&mut bp, type_0);
    // av_bprintf(
    //     &mut bp as *mut AVBPrint,
    //     b", \0" as *const u8 as *const c_char,
    // );
    // print_flags(&mut bp, flags);
    // av_bprintf(
    //     &mut bp as *mut AVBPrint,
    //     b", found %i matches%s\0" as *const u8 as *const c_char,
    //     nb_cd_matches,
    //     if nb_cd_matches != 0 {
    //         b":\0" as *const u8 as *const c_char
    //     } else {
    //         b".\0" as *const u8 as *const c_char
    //     },
    // );
    if nb_cd_matches == 0 {
        return -38;
    }
    let mut stack: [[*mut c_void; 2]; 64] = [[std::ptr::null_mut::<c_void>(); 2]; 64];
    let mut sp: c_int = 1;
    stack[0][0] = cd_matches as *mut c_void;
    stack[0][1] = cd_matches.offset(nb_cd_matches as isize).offset(-1) as *mut c_void;
    while sp != 0 {
        sp -= 1;
        let mut start: *mut TXCodeletMatch = stack[sp as usize][0] as *mut TXCodeletMatch;
        let mut end: *mut TXCodeletMatch = stack[sp as usize][1] as *mut TXCodeletMatch;
        while start < end {
            if start < end.offset(-1) {
                let mut checksort: c_int = 0;
                let mut right: *mut TXCodeletMatch = end.offset(-2);
                let mut left: *mut TXCodeletMatch = start.offset(1);
                let mut mid: *mut TXCodeletMatch =
                    start.offset((end.offset_from(start) as c_long >> 1) as isize);
                if cmp_matches(start, end) > 0 {
                    if cmp_matches(end, mid) > 0 {
                        core::ptr::swap(mid, start);
                    } else {
                        core::ptr::swap(end, start);
                    }
                } else if cmp_matches(start, mid) > 0 {
                    core::ptr::swap(mid, start);
                } else {
                    checksort = 1;
                }
                if cmp_matches(mid, end) > 0 {
                    core::ptr::swap(end, mid);
                    checksort = 0;
                }
                if start == end.offset(-2) {
                    break;
                }
                let SWAP_tmp_3: TXCodeletMatch = *mid;
                *mid = *end.offset(-1);
                *end.offset(-1) = SWAP_tmp_3;
                while left <= right {
                    while left <= right && cmp_matches(left, end.offset(-1)) < 0 {
                        left = left.offset(1);
                        left;
                    }
                    while left <= right && cmp_matches(right, end.offset(-1)) > 0 {
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
                *left = *end.offset(-1);
                *end.offset(-1) = SWAP_tmp_5;
                if checksort != 0 && (mid == left.offset(-1) || mid == left) {
                    mid = start;
                    while mid < end && cmp_matches(mid, mid.offset(1)) <= 0 {
                        mid = mid.offset(1);
                        mid;
                    }
                    if mid == end {
                        break;
                    }
                }
                if (end.offset_from(left) as c_long) < left.offset_from(start) as c_long {
                    stack[sp as usize][0] = start as *mut c_void;
                    let fresh13 = sp;
                    sp += 1;
                    stack[fresh13 as usize][1] = right as *mut c_void;
                    start = left.offset(1);
                } else {
                    stack[sp as usize][0] = left.offset(1) as *mut c_void;
                    let fresh14 = sp;
                    sp += 1;
                    stack[fresh14 as usize][1] = end as *mut c_void;
                    end = right;
                }
            } else {
                if cmp_matches(start, end) > 0 {
                    core::ptr::swap(end, start);
                }
                break;
            }
        }
    }
    // av_log(
    //     0 as *mut c_void,
    //     48,
    //     b"%s\n\0" as *const u8 as *const c_char,
    //     bp.str_0,
    // );
    let _i: c_int = 0;
    // while i < nb_cd_matches {
    //     av_log(
    //         0 as *mut c_void,
    //         48,
    //         b"    %i: \0" as *const u8 as *const c_char,
    //         i + 1,
    //     );
    //     print_cd_info(
    //         (*cd_matches.offset(i as isize)).cd,
    //         (*cd_matches.offset(i as isize)).prio,
    //         0,
    //         1,
    //     );
    //     i += 1;
    //     i;
    // }
    if ((*s).sub).is_null() {
        sub = alloc_zeroed(Layout::array::<AVTXContext>(4).unwrap()).cast();
        (*s).sub = sub;
        if sub.is_null() {
            ret = -12;
            current_block = 7391434065428304855;
        } else {
            current_block = 5706227035632243100;
        }
    } else {
        current_block = 5706227035632243100;
    }
    if let 5706227035632243100 = current_block {
        let mut i_0: c_int = 0;
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
            ret = 0;
            if ((*cd_0).init).is_some() {
                ret = ((*cd_0).init).expect("non-null function pointer")(
                    sctx, cd_0, flags, opts, len, inv, scale,
                );
            }
            if ret >= 0 {
                if !opts.is_null()
                    && (*opts).map_dir as c_uint != FF_TX_MAP_NONE as c_int as c_uint
                    && (*sctx).map_dir as c_uint == FF_TX_MAP_NONE as c_int as c_uint
                {
                    (*sctx).map = alloc(Layout::array::<c_int>(len as usize).unwrap()).cast();
                    if ((*sctx).map).is_null() {
                        ret = -12;
                        current_block = 7391434065428304855;
                        break;
                    } else {
                        let mut i_1: c_int = 0;
                        while i_1 < len {
                            *((*sctx).map).offset(i_1 as isize) = i_1;
                            i_1 += 1;
                            i_1;
                        }
                    }
                } else if !opts.is_null() && (*opts).map_dir as c_uint != (*sctx).map_dir as c_uint
                {
                    let tmp: *mut c_int =
                        alloc(Layout::array::<c_int>(len as usize).unwrap()).cast();
                    if tmp.is_null() {
                        ret = -12;
                        current_block = 7391434065428304855;
                        break;
                    } else {
                        ptr::copy_nonoverlapping((*sctx).map, tmp, len as usize);
                        let mut i_2: c_int = 0;
                        while i_2 < len {
                            *((*sctx).map).offset(*tmp.offset(i_2 as isize) as isize) = i_2;
                            i_2 += 1;
                            i_2;
                        }
                        av_free(tmp as *mut c_void);
                    }
                }
                (*s).nb_sub += 1;
                (*s).nb_sub;
                current_block = 7391434065428304855;
                break;
            } else {
                (*s).fn_0[(*s).nb_sub as usize] = None;
                (*s).cd[(*s).nb_sub as usize] = std::ptr::null::<FFTXCodelet>();
                reset_ctx(sctx, 0);
                if ret == -12 {
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
                    av_freep(&mut (*s).sub as *mut *mut AVTXContext as *mut c_void);
                }
            }
        }
    }
    av_free(cd_matches as *mut c_void);
    ret
}

#[cold]
pub(crate) unsafe fn av_tx_init(
    ctx: *mut *mut AVTXContext,
    tx: *mut av_tx_fn,
    type_0: AVTXType,
    inv: c_int,
    len: c_int,
    mut scale: *const c_void,
    mut flags: c_ulong,
) -> c_int {
    let mut ret: c_int = 0;
    let mut tmp: AVTXContext = {
        AVTXContext {
            len: 0,
            inv: 0,
            map: std::ptr::null_mut::<c_int>(),
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
            opaque: std::ptr::null_mut::<c_void>(),
        }
    };
    let default_scale_d: c_double = 1.0f64;
    let default_scale_f: c_float = 1.0f32;
    if len == 0 || type_0 as c_uint >= AV_TX_NB as c_int as c_uint || ctx.is_null() || tx.is_null()
    {
        return -22;
    }
    if flags & AV_TX_UNALIGNED as c_int as c_ulong == 0 {
        flags = (flags as c_ulonglong | (1 as c_ulonglong) << 62) as c_ulong;
    }
    if flags & AV_TX_INPLACE as c_int as c_ulong == 0 {
        flags = (flags as c_ulonglong | (1 as c_ulonglong) << 63) as c_ulong;
    }
    if scale.is_null()
        && (type_0 as c_uint == AV_TX_FLOAT_MDCT as c_int as c_uint
            || type_0 as c_uint == AV_TX_INT32_MDCT as c_int as c_uint)
    {
        scale = &default_scale_f as *const c_float as *const c_void;
    } else if scale.is_null() && type_0 as c_uint == AV_TX_DOUBLE_MDCT as c_int as c_uint {
        scale = &default_scale_d as *const c_double as *const c_void;
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
    if ret < 0 {
        return ret;
    }
    *ctx = &mut *(tmp.sub).offset(0) as *mut AVTXContext;
    *tx = tmp.fn_0[0];
    // av_log(
    //     0 as *mut c_void,
    //     48,
    //     b"Transform tree:\n\0" as *const u8 as *const c_char,
    // );
    // print_tx_structure(*ctx, 0);
    ret
}
unsafe fn run_static_initializers() {
    codelet_list_num = (size_of::<[*const *const FFTXCodelet; 4]>() as c_ulong)
        .wrapping_div(size_of::<*const *const FFTXCodelet>() as c_ulong)
        as c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe fn(); 1] = [run_static_initializers];

#[inline(always)]
unsafe extern "C" fn mulinv(mut n: c_int, m: c_int) -> c_int {
    n %= m;
    let mut x: c_int = 1;
    while x < m {
        if n * x % m == 1 {
            return x;
        }
        x += 1;
        x;
    }
    if 0 == 0 {
        av_log(
            std::ptr::null_mut::<c_void>(),
            0,
            b"Assertion %s failed at %s:%d\n\0" as *const u8 as *const c_char,
            b"0\0" as *const u8 as *const c_char,
            b"libavutil/tx.c\0" as *const u8 as *const c_char,
            39,
        );
        abort();
    }
    0
}

pub unsafe extern "C" fn ff_tx_gen_pfa_input_map(
    s: *mut AVTXContext,
    opts: *mut FFTXCodeletOptions,
    d1: c_int,
    d2: c_int,
) -> c_int {
    let sl: c_int = d1 * d2;
    (*s).map =
        av_malloc(((*s).len as c_ulong).wrapping_mul(size_of::<c_int>() as c_ulong)) as *mut c_int;
    if ((*s).map).is_null() {
        return -12;
    }
    let mut k: c_int = 0;
    while k < (*s).len {
        if (*s).inv != 0
            || !opts.is_null() && (*opts).map_dir as c_uint == FF_TX_MAP_SCATTER as c_int as c_uint
        {
            let mut m: c_int = 0;
            while m < d2 {
                let mut n: c_int = 0;
                while n < d1 {
                    *((*s).map).offset((k + (m * d1 + n * d2) % sl) as isize) = m * d1 + n;
                    n += 1;
                    n;
                }
                m += 1;
                m;
            }
        } else {
            let mut m_0: c_int = 0;
            while m_0 < d2 {
                let mut n_0: c_int = 0;
                while n_0 < d1 {
                    *((*s).map).offset((k + m_0 * d1 + n_0) as isize) = (m_0 * d1 + n_0 * d2) % sl;
                    n_0 += 1;
                    n_0;
                }
                m_0 += 1;
                m_0;
            }
        }
        if (*s).inv != 0 {
            let mut w: c_int = 1;
            while w <= sl >> 1 {
                let SWAP_tmp: c_int = *((*s).map).offset((k + sl - w) as isize);
                *((*s).map).offset((k + sl - w) as isize) = *((*s).map).offset((k + w) as isize);
                *((*s).map).offset((k + w) as isize) = SWAP_tmp;
                w += 1;
                w;
            }
        }
        k += sl;
    }
    (*s).map_dir = (if !opts.is_null() {
        (*opts).map_dir as c_uint
    } else {
        FF_TX_MAP_GATHER as c_int as c_uint
    }) as FFTXMapDirection;
    0
}

pub unsafe extern "C" fn ff_tx_gen_compound_mapping(
    s: *mut AVTXContext,
    opts: *mut FFTXCodeletOptions,
    inv: c_int,
    n: c_int,
    m: c_int,
) -> c_int {
    let mut in_map: *mut c_int = std::ptr::null_mut::<c_int>();
    let mut out_map: *mut c_int = std::ptr::null_mut::<c_int>();
    let len: c_int = n * m;
    let mut m_inv: c_int = 0;
    let mut n_inv: c_int = 0;
    if av_gcd(n as c_long, m as c_long) != 1 as c_long {
        return -22;
    }
    m_inv = mulinv(m, n);
    n_inv = mulinv(n, m);
    (*s).map =
        av_malloc(((2 * len) as c_ulong).wrapping_mul(size_of::<c_int>() as c_ulong)) as *mut c_int;
    if ((*s).map).is_null() {
        return -12;
    }
    in_map = (*s).map;
    out_map = ((*s).map).offset(len as isize);
    if !opts.is_null() && (*opts).map_dir as c_uint == FF_TX_MAP_SCATTER as c_int as c_uint {
        let mut j: c_int = 0;
        while j < m {
            let mut i: c_int = 0;
            while i < n {
                *in_map.offset(((i * m + j * n) % len) as isize) = j * n + i;
                *out_map.offset(((i * m * m_inv + j * n * n_inv) % len) as isize) = i * m + j;
                i += 1;
                i;
            }
            j += 1;
            j;
        }
    } else {
        let mut j_0: c_int = 0;
        while j_0 < m {
            let mut i_0: c_int = 0;
            while i_0 < n {
                *in_map.offset((j_0 * n + i_0) as isize) = (i_0 * m + j_0 * n) % len;
                *out_map.offset(((i_0 * m * m_inv + j_0 * n * n_inv) % len) as isize) =
                    i_0 * m + j_0;
                i_0 += 1;
                i_0;
            }
            j_0 += 1;
            j_0;
        }
    }
    if inv != 0 {
        let mut i_1: c_int = 0;
        while i_1 < m {
            let in_0: *mut c_int = &mut *in_map.offset((i_1 * n + 1) as isize) as *mut c_int;
            let mut j_1: c_int = 0;
            while j_1 < n - 1 >> 1 {
                let SWAP_tmp: c_int = *in_0.offset((n - j_1 - 2) as isize);
                *in_0.offset((n - j_1 - 2) as isize) = *in_0.offset(j_1 as isize);
                *in_0.offset(j_1 as isize) = SWAP_tmp;
                j_1 += 1;
                j_1;
            }
            i_1 += 1;
            i_1;
        }
    }
    (*s).map_dir = (if !opts.is_null() {
        (*opts).map_dir as c_uint
    } else {
        FF_TX_MAP_GATHER as c_int as c_uint
    }) as FFTXMapDirection;
    0
}
#[inline]
unsafe extern "C" fn split_radix_permutation(i: c_int, mut len: c_int, inv: c_int) -> c_int {
    len >>= 1;
    if len <= 1 {
        return i & 1;
    }
    if i & len == 0 {
        return split_radix_permutation(i, len, inv) * 2;
    }
    len >>= 1;
    split_radix_permutation(i, len, inv) * 4 + 1 - 2 * ((i & len == 0) as c_int ^ inv)
}

pub unsafe extern "C" fn ff_tx_gen_ptwo_revtab(
    s: *mut AVTXContext,
    opts: *mut FFTXCodeletOptions,
) -> c_int {
    let len: c_int = (*s).len;
    (*s).map =
        av_malloc((len as c_ulong).wrapping_mul(size_of::<c_int>() as c_ulong)) as *mut c_int;
    if ((*s).map).is_null() {
        return -12;
    }
    if !opts.is_null() && (*opts).map_dir as c_uint == FF_TX_MAP_SCATTER as c_int as c_uint {
        let mut i: c_int = 0;
        while i < (*s).len {
            *((*s).map).offset((-split_radix_permutation(i, len, (*s).inv) & len - 1) as isize) = i;
            i += 1;
            i;
        }
    } else {
        let mut i_0: c_int = 0;
        while i_0 < (*s).len {
            *((*s).map).offset(i_0 as isize) =
                -split_radix_permutation(i_0, len, (*s).inv) & len - 1;
            i_0 += 1;
            i_0;
        }
    }
    (*s).map_dir = (if !opts.is_null() {
        (*opts).map_dir as c_uint
    } else {
        FF_TX_MAP_GATHER as c_int as c_uint
    }) as FFTXMapDirection;
    0
}

pub unsafe extern "C" fn ff_tx_gen_inplace_map(s: *mut AVTXContext, len: c_int) -> c_int {
    let mut src_map: *mut c_int = std::ptr::null_mut::<c_int>();
    let mut out_map_idx: c_int = 0;
    if ((*s).sub).is_null() || ((*(*s).sub).map).is_null() {
        return -22;
    }
    (*s).map =
        av_mallocz((len as c_ulong).wrapping_mul(size_of::<c_int>() as c_ulong)) as *mut c_int;
    if ((*s).map).is_null() {
        return -12;
    }
    src_map = (*(*s).sub).map;
    let mut src: c_int = 1;
    while src < (*s).len {
        let mut dst: c_int = *src_map.offset(src as isize);
        let mut found: c_int = 0;
        if !(dst <= src) {
            loop {
                let mut j: c_int = 0;
                while j < out_map_idx {
                    if dst == *((*s).map).offset(j as isize) {
                        found = 1;
                        break;
                    } else {
                        j += 1;
                        j;
                    }
                }
                dst = *src_map.offset(dst as isize);
                if !(dst != src && found == 0) {
                    break;
                }
            }
            if found == 0 {
                let fresh0 = out_map_idx;
                out_map_idx += 1;
                *((*s).map).offset(fresh0 as isize) = src;
            }
        }
        src += 1;
        src;
    }
    let fresh1 = out_map_idx;
    out_map_idx += 1;
    *((*s).map).offset(fresh1 as isize) = 0;
    0
}

pub unsafe extern "C" fn ff_tx_gen_default_map(
    s: *mut AVTXContext,
    _opts: *mut FFTXCodeletOptions,
) -> c_int {
    (*s).map =
        av_malloc(((*s).len as c_ulong).wrapping_mul(size_of::<c_int>() as c_ulong)) as *mut c_int;
    if ((*s).map).is_null() {
        return -12;
    }
    *((*s).map).offset(0) = 0;
    if (*s).inv != 0 {
        let mut i: c_int = 1;
        while i < (*s).len {
            *((*s).map).offset(i as isize) = (*s).len - i;
            i += 1;
            i;
        }
    } else {
        let mut i_0: c_int = 1;
        while i_0 < (*s).len {
            *((*s).map).offset(i_0 as isize) = i_0;
            i_0 += 1;
            i_0;
        }
    }
    (*s).map_dir = FF_TX_MAP_GATHER;
    0
}

pub unsafe extern "C" fn ff_tx_decompose_length(
    dst: *mut c_int,
    type_0: AVTXType,
    len: c_int,
    inv: c_int,
) -> c_int {
    let current_block: u64;
    let mut nb_decomp: c_int = 0;
    let mut ld: [FFTXLenDecomp; 512] = [FFTXLenDecomp {
        len: 0,
        len2: 0,
        prio: 0,
        cd: std::ptr::null::<FFTXCodelet>(),
    }; 512];
    let mut codelet_list_idx: c_int = codelet_list_num;
    let cpu_flags: c_int = av_get_cpu_flags();
    's_9: loop {
        let fresh6 = codelet_list_idx;
        codelet_list_idx -= 1;
        if !(fresh6 != 0) {
            current_block = 4567019141635105728;
            break;
        }
        let mut list: *const *const FFTXCodelet = codelet_list[codelet_list_idx as usize];
        let mut cd: *const FFTXCodelet = std::ptr::null::<FFTXCodelet>();
        loop {
            let fresh7 = list;
            list = list.offset(1);
            cd = *fresh7;
            if cd.is_null() {
                break;
            }
            let mut fl: c_int = len;
            let mut skip: c_int = 0;
            let mut prio: c_int = 0;
            let mut factors_product: c_int = 1;
            let mut factors_mod: c_int = 0;
            if nb_decomp >= 512 {
                current_block = 12954599432099290578;
                break 's_9;
            }
            if (*cd).type_0 as c_uint != 2147483647 as c_uint
                && type_0 as c_uint != (*cd).type_0 as c_uint
            {
                continue;
            }
            if (*cd).flags as c_ulonglong & (1 as c_ulonglong) << 59 != 0 && inv != 0
                || (*cd).flags as c_ulonglong
                    & ((1 as c_ulonglong) << 60 | AV_TX_FULL_IMDCT as c_int as c_ulonglong)
                    != 0
                    && inv == 0
                || (*cd).flags as c_ulonglong
                    & ((1 as c_ulonglong) << 59 | AV_TX_REAL_TO_REAL as c_int as c_ulonglong)
                    != 0
                    && inv != 0
                || (*cd).flags as c_ulonglong
                    & ((1 as c_ulonglong) << 59 | AV_TX_REAL_TO_IMAGINARY as c_int as c_ulonglong)
                    != 0
                    && inv != 0
            {
                continue;
            }
            if (*cd).cpu_flags != 0 && cpu_flags & ((*cd).cpu_flags & !cpu_slow_mask) == 0 {
                continue;
            }
            let mut i: c_int = 0;
            while i < 16 {
                if (*cd).factors[i as usize] == 0 || fl == 1 {
                    break;
                }
                if (*cd).factors[i as usize] == -1 {
                    factors_mod += 1;
                    factors_mod;
                    factors_product *= fl;
                } else if fl % (*cd).factors[i as usize] == 0 {
                    factors_mod += 1;
                    factors_mod;
                    if (*cd).factors[i as usize] == 2 {
                        let b: c_int = ff_ctz_c(fl);
                        fl >>= b;
                        factors_product <<= b;
                    } else {
                        loop {
                            fl /= (*cd).factors[i as usize];
                            factors_product *= (*cd).factors[i as usize];
                            if !(fl % (*cd).factors[i as usize] == 0) {
                                break;
                            }
                        }
                    }
                }
                i += 1;
                i;
            }
            if factors_mod < (*cd).nb_factors || len == factors_product {
                continue;
            }
            if av_gcd(factors_product as c_long, fl as c_long) != 1 as c_long {
                continue;
            }
            if factors_product < (*cd).min_len
                || (*cd).max_len != -1 && factors_product > (*cd).max_len
            {
                continue;
            }
            prio = get_codelet_prio(cd, cpu_flags, factors_product) * factors_product;
            let mut i_0: c_int = 0;
            while i_0 < nb_decomp {
                if factors_product == ld[i_0 as usize].len {
                    if prio > ld[i_0 as usize].prio {
                        ld[i_0 as usize].prio = prio;
                    }
                    skip = 1;
                    break;
                } else {
                    i_0 += 1;
                    i_0;
                }
            }
            if skip == 0 {
                ld[nb_decomp as usize].cd = cd;
                ld[nb_decomp as usize].len = factors_product;
                ld[nb_decomp as usize].len2 = fl;
                ld[nb_decomp as usize].prio = prio;
                nb_decomp += 1;
                nb_decomp;
            }
        }
    }
    match current_block {
        4567019141635105728 => {
            if nb_decomp == 0 {
                return -22;
            }
        }
        _ => {}
    }
    let mut stack: [[*mut c_void; 2]; 64] = [[std::ptr::null_mut::<c_void>(); 2]; 64];
    let mut sp: c_int = 1;
    stack[0][0] = ld.as_mut_ptr() as *mut c_void;
    stack[0][1] = ld.as_mut_ptr().offset(nb_decomp as isize).offset(-1) as *mut c_void;
    while sp != 0 {
        sp -= 1;
        let mut start: *mut FFTXLenDecomp = stack[sp as usize][0] as *mut FFTXLenDecomp;
        let mut end: *mut FFTXLenDecomp = stack[sp as usize][1] as *mut FFTXLenDecomp;
        while start < end {
            if start < end.offset(-1) {
                let mut checksort: c_int = 0;
                let mut right: *mut FFTXLenDecomp = end.offset(-2);
                let mut left: *mut FFTXLenDecomp = start.offset(1);
                let mut mid: *mut FFTXLenDecomp =
                    start.offset((end.offset_from(start) as c_long >> 1) as isize);
                if cmp_decomp(start, end) > 0 {
                    if cmp_decomp(end, mid) > 0 {
                        core::ptr::swap(mid, start);
                    } else {
                        core::ptr::swap(end, start);
                    }
                } else if cmp_decomp(start, mid) > 0 {
                    core::ptr::swap(mid, start);
                } else {
                    checksort = 1;
                }
                if cmp_decomp(mid, end) > 0 {
                    core::ptr::swap(end, mid);
                    checksort = 0;
                }
                if start == end.offset(-2) {
                    break;
                }
                let SWAP_tmp_3: FFTXLenDecomp = *mid;
                *mid = *end.offset(-1);
                *end.offset(-1) = SWAP_tmp_3;
                while left <= right {
                    while left <= right && cmp_decomp(left, end.offset(-1)) < 0 {
                        left = left.offset(1);
                        left;
                    }
                    while left <= right && cmp_decomp(right, end.offset(-1)) > 0 {
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
                let SWAP_tmp_5: FFTXLenDecomp = *left;
                *left = *end.offset(-1);
                *end.offset(-1) = SWAP_tmp_5;
                if checksort != 0 && (mid == left.offset(-1) || mid == left) {
                    mid = start;
                    while mid < end && cmp_decomp(mid, mid.offset(1)) <= 0 {
                        mid = mid.offset(1);
                        mid;
                    }
                    if mid == end {
                        break;
                    }
                }
                if (end.offset_from(left) as c_long) < left.offset_from(start) as c_long {
                    stack[sp as usize][0] = start as *mut c_void;
                    let fresh8 = sp;
                    sp += 1;
                    stack[fresh8 as usize][1] = right as *mut c_void;
                    start = left.offset(1);
                } else {
                    stack[sp as usize][0] = left.offset(1) as *mut c_void;
                    let fresh9 = sp;
                    sp += 1;
                    stack[fresh9 as usize][1] = end as *mut c_void;
                    end = right;
                }
            } else {
                if cmp_decomp(start, end) > 0 {
                    core::ptr::swap(end, start);
                }
                break;
            }
        }
    }
    let mut i_1: c_int = 0;
    while i_1 < nb_decomp {
        if (*ld[i_1 as usize].cd).nb_factors > 1 {
            *dst.offset(i_1 as isize) = ld[i_1 as usize].len2;
        } else {
            *dst.offset(i_1 as isize) = ld[i_1 as usize].len;
        }
        i_1 += 1;
        i_1;
    }
    nb_decomp
}

pub unsafe extern "C" fn ff_tx_clear_ctx(s: *mut AVTXContext) {
    reset_ctx(s, 0);
}

unsafe extern "C" fn cmp_decomp(a: *mut FFTXLenDecomp, b: *mut FFTXLenDecomp) -> c_int {
    ((*b).prio > (*a).prio) as c_int - ((*b).prio < (*a).prio) as c_int
}
