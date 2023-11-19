use ::libc;
extern "C" {
    fn abort() -> !;
    fn av_log(avcl: *mut libc::c_void, level: libc::c_int, fmt: *const libc::c_char, _: ...);
    fn av_get_cpu_flags() -> libc::c_int;
    fn av_bprint_init(buf: *mut AVBPrint, size_init: libc::c_uint, size_max: libc::c_uint);
    fn av_bprintf(buf: *mut AVBPrint, fmt: *const libc::c_char, _: ...);
    static ff_tx_codelet_list_float_c: [*const FFTXCodelet; 0];
    static ff_tx_codelet_list_double_c: [*const FFTXCodelet; 0];
    static ff_tx_codelet_list_int32_c: [*const FFTXCodelet; 0];
    fn av_fast_realloc(
        ptr: *mut libc::c_void,
        size: *mut libc::c_uint,
        min_size: size_t,
    ) -> *mut libc::c_void;
    fn av_mallocz(size: size_t) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn av_malloc(size: size_t) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn av_freep(ptr: *mut libc::c_void);
    fn av_free(ptr: *mut libc::c_void);
    fn av_gcd(a: int64_t, b: int64_t) -> int64_t;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type ptrdiff_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVBPrint {
    pub str_0: *mut libc::c_char,
    pub len: libc::c_uint,
    pub size: libc::c_uint,
    pub size_max: libc::c_uint,
    pub reserved_internal_buffer: [libc::c_char; 1],
    pub reserved_padding: [libc::c_char; 1000],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVTXContext {
    pub len: libc::c_int,
    pub inv: libc::c_int,
    pub map: *mut libc::c_int,
    pub exp: *mut libc::c_void,
    pub tmp: *mut libc::c_void,
    pub sub: *mut AVTXContext,
    pub fn_0: [av_tx_fn; 4],
    pub nb_sub: libc::c_int,
    pub cd: [*const FFTXCodelet; 4],
    pub cd_self: *const FFTXCodelet,
    pub type_0: AVTXType,
    pub flags: uint64_t,
    pub map_dir: FFTXMapDirection,
    pub scale_f: libc::c_float,
    pub scale_d: libc::c_double,
    pub opaque: *mut libc::c_void,
}
pub type FFTXMapDirection = libc::c_uint;
pub const FF_TX_MAP_SCATTER: FFTXMapDirection = 2;
pub const FF_TX_MAP_GATHER: FFTXMapDirection = 1;
pub const FF_TX_MAP_NONE: FFTXMapDirection = 0;
pub type AVTXType = libc::c_uint;
pub const AV_TX_NB: AVTXType = 18;
pub const AV_TX_INT32_DST_I: AVTXType = 17;
pub const AV_TX_DOUBLE_DST_I: AVTXType = 16;
pub const AV_TX_FLOAT_DST_I: AVTXType = 15;
pub const AV_TX_INT32_DCT_I: AVTXType = 14;
pub const AV_TX_DOUBLE_DCT_I: AVTXType = 13;
pub const AV_TX_FLOAT_DCT_I: AVTXType = 12;
pub const AV_TX_INT32_DCT: AVTXType = 11;
pub const AV_TX_DOUBLE_DCT: AVTXType = 10;
pub const AV_TX_FLOAT_DCT: AVTXType = 9;
pub const AV_TX_INT32_RDFT: AVTXType = 8;
pub const AV_TX_DOUBLE_RDFT: AVTXType = 7;
pub const AV_TX_FLOAT_RDFT: AVTXType = 6;
pub const AV_TX_INT32_MDCT: AVTXType = 5;
pub const AV_TX_DOUBLE_MDCT: AVTXType = 3;
pub const AV_TX_FLOAT_MDCT: AVTXType = 1;
pub const AV_TX_INT32_FFT: AVTXType = 4;
pub const AV_TX_DOUBLE_FFT: AVTXType = 2;
pub const AV_TX_FLOAT_FFT: AVTXType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFTXCodelet {
    pub name: *const libc::c_char,
    pub function: av_tx_fn,
    pub type_0: AVTXType,
    pub flags: uint64_t,
    pub factors: [libc::c_int; 16],
    pub nb_factors: libc::c_int,
    pub min_len: libc::c_int,
    pub max_len: libc::c_int,
    pub init: Option<
        unsafe extern "C" fn(
            *mut AVTXContext,
            *const FFTXCodelet,
            uint64_t,
            *mut FFTXCodeletOptions,
            libc::c_int,
            libc::c_int,
            *const libc::c_void,
        ) -> libc::c_int,
    >,
    pub uninit: Option<unsafe extern "C" fn(*mut AVTXContext) -> libc::c_int>,
    pub cpu_flags: libc::c_int,
    pub prio: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFTXCodeletOptions {
    pub map_dir: FFTXMapDirection,
}
pub type av_tx_fn = Option<
    unsafe extern "C" fn(*mut AVTXContext, *mut libc::c_void, *mut libc::c_void, ptrdiff_t) -> (),
>;
pub type TXComplex = ();
pub type AVTXFlags = libc::c_uint;
pub const AV_TX_REAL_TO_IMAGINARY: AVTXFlags = 16;
pub const AV_TX_REAL_TO_REAL: AVTXFlags = 8;
pub const AV_TX_FULL_IMDCT: AVTXFlags = 4;
pub const AV_TX_UNALIGNED: AVTXFlags = 2;
pub const AV_TX_INPLACE: AVTXFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TXCodeletMatch {
    pub cd: *const FFTXCodelet,
    pub prio: libc::c_int,
}
pub const FF_TX_PRIO_MAX: FFTXCodeletPriority = 32768;
pub type FFTXCodeletPriority = libc::c_int;
pub const FF_TX_PRIO_MIN: FFTXCodeletPriority = -131072;
pub const FF_TX_PRIO_BASE: FFTXCodeletPriority = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFTXLenDecomp {
    pub len: libc::c_int,
    pub len2: libc::c_int,
    pub prio: libc::c_int,
    pub cd: *const FFTXCodelet,
}
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
#[inline(always)]
unsafe extern "C" fn mulinv(mut n: libc::c_int, mut m: libc::c_int) -> libc::c_int {
    n = n % m;
    let mut x: libc::c_int = 1 as libc::c_int;
    while x < m {
        if n * x % m == 1 as libc::c_int {
            return x;
        }
        x += 1;
        x;
    }
    if 0 as libc::c_int == 0 {
        av_log(
            0 as *mut libc::c_void,
            0 as libc::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8 as *const libc::c_char,
            b"0\0" as *const u8 as *const libc::c_char,
            b"libavutil/tx.c\0" as *const u8 as *const libc::c_char,
            39 as libc::c_int,
        );
        abort();
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_tx_gen_pfa_input_map(
    mut s: *mut AVTXContext,
    mut opts: *mut FFTXCodeletOptions,
    mut d1: libc::c_int,
    mut d2: libc::c_int,
) -> libc::c_int {
    let sl: libc::c_int = d1 * d2;
    (*s).map = av_malloc(
        ((*s).len as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if ((*s).map).is_null() {
        return -(12 as libc::c_int);
    }
    let mut k: libc::c_int = 0 as libc::c_int;
    while k < (*s).len {
        if (*s).inv != 0
            || !opts.is_null()
                && (*opts).map_dir as libc::c_uint
                    == FF_TX_MAP_SCATTER as libc::c_int as libc::c_uint
        {
            let mut m: libc::c_int = 0 as libc::c_int;
            while m < d2 {
                let mut n: libc::c_int = 0 as libc::c_int;
                while n < d1 {
                    *((*s).map).offset((k + (m * d1 + n * d2) % sl) as isize) = m * d1 + n;
                    n += 1;
                    n;
                }
                m += 1;
                m;
            }
        } else {
            let mut m_0: libc::c_int = 0 as libc::c_int;
            while m_0 < d2 {
                let mut n_0: libc::c_int = 0 as libc::c_int;
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
            let mut w: libc::c_int = 1 as libc::c_int;
            while w <= sl >> 1 as libc::c_int {
                let mut SWAP_tmp: libc::c_int = *((*s).map).offset((k + sl - w) as isize);
                *((*s).map).offset((k + sl - w) as isize) = *((*s).map).offset((k + w) as isize);
                *((*s).map).offset((k + w) as isize) = SWAP_tmp;
                w += 1;
                w;
            }
        }
        k += sl;
    }
    (*s).map_dir = (if !opts.is_null() {
        (*opts).map_dir as libc::c_uint
    } else {
        FF_TX_MAP_GATHER as libc::c_int as libc::c_uint
    }) as FFTXMapDirection;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_tx_gen_compound_mapping(
    mut s: *mut AVTXContext,
    mut opts: *mut FFTXCodeletOptions,
    mut inv: libc::c_int,
    mut n: libc::c_int,
    mut m: libc::c_int,
) -> libc::c_int {
    let mut in_map: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut out_map: *mut libc::c_int = 0 as *mut libc::c_int;
    let len: libc::c_int = n * m;
    let mut m_inv: libc::c_int = 0;
    let mut n_inv: libc::c_int = 0;
    if av_gcd(n as int64_t, m as int64_t) != 1 as libc::c_int as libc::c_long {
        return -(22 as libc::c_int);
    }
    m_inv = mulinv(m, n);
    n_inv = mulinv(n, m);
    (*s).map = av_malloc(
        ((2 as libc::c_int * len) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if ((*s).map).is_null() {
        return -(12 as libc::c_int);
    }
    in_map = (*s).map;
    out_map = ((*s).map).offset(len as isize);
    if !opts.is_null()
        && (*opts).map_dir as libc::c_uint == FF_TX_MAP_SCATTER as libc::c_int as libc::c_uint
    {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < m {
            let mut i: libc::c_int = 0 as libc::c_int;
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
        let mut j_0: libc::c_int = 0 as libc::c_int;
        while j_0 < m {
            let mut i_0: libc::c_int = 0 as libc::c_int;
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
        let mut i_1: libc::c_int = 0 as libc::c_int;
        while i_1 < m {
            let mut in_0: *mut libc::c_int =
                &mut *in_map.offset((i_1 * n + 1 as libc::c_int) as isize) as *mut libc::c_int;
            let mut j_1: libc::c_int = 0 as libc::c_int;
            while j_1 < n - 1 as libc::c_int >> 1 as libc::c_int {
                let mut SWAP_tmp: libc::c_int = *in_0.offset((n - j_1 - 2 as libc::c_int) as isize);
                *in_0.offset((n - j_1 - 2 as libc::c_int) as isize) = *in_0.offset(j_1 as isize);
                *in_0.offset(j_1 as isize) = SWAP_tmp;
                j_1 += 1;
                j_1;
            }
            i_1 += 1;
            i_1;
        }
    }
    (*s).map_dir = (if !opts.is_null() {
        (*opts).map_dir as libc::c_uint
    } else {
        FF_TX_MAP_GATHER as libc::c_int as libc::c_uint
    }) as FFTXMapDirection;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn split_radix_permutation(
    mut i: libc::c_int,
    mut len: libc::c_int,
    mut inv: libc::c_int,
) -> libc::c_int {
    len >>= 1 as libc::c_int;
    if len <= 1 as libc::c_int {
        return i & 1 as libc::c_int;
    }
    if i & len == 0 {
        return split_radix_permutation(i, len, inv) * 2 as libc::c_int;
    }
    len >>= 1 as libc::c_int;
    return split_radix_permutation(i, len, inv) * 4 as libc::c_int + 1 as libc::c_int
        - 2 as libc::c_int * ((i & len == 0) as libc::c_int ^ inv);
}
#[no_mangle]
pub unsafe extern "C" fn ff_tx_gen_ptwo_revtab(
    mut s: *mut AVTXContext,
    mut opts: *mut FFTXCodeletOptions,
) -> libc::c_int {
    let mut len: libc::c_int = (*s).len;
    (*s).map = av_malloc(
        (len as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if ((*s).map).is_null() {
        return -(12 as libc::c_int);
    }
    if !opts.is_null()
        && (*opts).map_dir as libc::c_uint == FF_TX_MAP_SCATTER as libc::c_int as libc::c_uint
    {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < (*s).len {
            *((*s).map).offset(
                (-split_radix_permutation(i, len, (*s).inv) & len - 1 as libc::c_int) as isize,
            ) = i;
            i += 1;
            i;
        }
    } else {
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < (*s).len {
            *((*s).map).offset(i_0 as isize) =
                -split_radix_permutation(i_0, len, (*s).inv) & len - 1 as libc::c_int;
            i_0 += 1;
            i_0;
        }
    }
    (*s).map_dir = (if !opts.is_null() {
        (*opts).map_dir as libc::c_uint
    } else {
        FF_TX_MAP_GATHER as libc::c_int as libc::c_uint
    }) as FFTXMapDirection;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_tx_gen_inplace_map(
    mut s: *mut AVTXContext,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut src_map: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut out_map_idx: libc::c_int = 0 as libc::c_int;
    if ((*s).sub).is_null() || ((*(*s).sub).map).is_null() {
        return -(22 as libc::c_int);
    }
    (*s).map = av_mallocz(
        (len as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if ((*s).map).is_null() {
        return -(12 as libc::c_int);
    }
    src_map = (*(*s).sub).map;
    let mut src: libc::c_int = 1 as libc::c_int;
    while src < (*s).len {
        let mut dst: libc::c_int = *src_map.offset(src as isize);
        let mut found: libc::c_int = 0 as libc::c_int;
        if !(dst <= src) {
            loop {
                let mut j: libc::c_int = 0 as libc::c_int;
                while j < out_map_idx {
                    if dst == *((*s).map).offset(j as isize) {
                        found = 1 as libc::c_int;
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
                out_map_idx = out_map_idx + 1;
                *((*s).map).offset(fresh0 as isize) = src;
            }
        }
        src += 1;
        src;
    }
    let fresh1 = out_map_idx;
    out_map_idx = out_map_idx + 1;
    *((*s).map).offset(fresh1 as isize) = 0 as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn parity_revtab_generator(
    mut revtab: *mut libc::c_int,
    mut n: libc::c_int,
    mut inv: libc::c_int,
    mut offset: libc::c_int,
    mut is_dual: libc::c_int,
    mut dual_high: libc::c_int,
    mut len: libc::c_int,
    mut basis: libc::c_int,
    mut dual_stride: libc::c_int,
    mut inv_lookup: libc::c_int,
) {
    len >>= 1 as libc::c_int;
    if len <= basis {
        let mut k1: libc::c_int = 0;
        let mut k2: libc::c_int = 0;
        let mut stride: libc::c_int = 0;
        let mut even_idx: libc::c_int = 0;
        let mut odd_idx: libc::c_int = 0;
        is_dual = (is_dual != 0 && dual_stride != 0) as libc::c_int;
        dual_high = is_dual & dual_high;
        stride = if is_dual != 0 {
            if dual_stride > len {
                len
            } else {
                dual_stride
            }
        } else {
            0 as libc::c_int
        };
        even_idx = offset + dual_high * (stride - 2 as libc::c_int * len);
        odd_idx = even_idx
            + len
            + (is_dual != 0 && dual_high == 0) as libc::c_int * len
            + dual_high * len;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < len {
            k1 = -split_radix_permutation(offset + i * 2 as libc::c_int + 0 as libc::c_int, n, inv)
                & n - 1 as libc::c_int;
            k2 = -split_radix_permutation(offset + i * 2 as libc::c_int + 1 as libc::c_int, n, inv)
                & n - 1 as libc::c_int;
            if inv_lookup != 0 {
                let fresh2 = even_idx;
                even_idx = even_idx + 1;
                *revtab.offset(fresh2 as isize) = k1;
                let fresh3 = odd_idx;
                odd_idx = odd_idx + 1;
                *revtab.offset(fresh3 as isize) = k2;
            } else {
                let fresh4 = even_idx;
                even_idx = even_idx + 1;
                *revtab.offset(k1 as isize) = fresh4;
                let fresh5 = odd_idx;
                odd_idx = odd_idx + 1;
                *revtab.offset(k2 as isize) = fresh5;
            }
            if stride != 0 && (i + 1 as libc::c_int) % stride == 0 {
                even_idx += stride;
                odd_idx += stride;
            }
            i += 1;
            i;
        }
        return;
    }
    parity_revtab_generator(
        revtab,
        n,
        inv,
        offset,
        0 as libc::c_int,
        0 as libc::c_int,
        len >> 0 as libc::c_int,
        basis,
        dual_stride,
        inv_lookup,
    );
    parity_revtab_generator(
        revtab,
        n,
        inv,
        offset + (len >> 0 as libc::c_int),
        1 as libc::c_int,
        0 as libc::c_int,
        len >> 1 as libc::c_int,
        basis,
        dual_stride,
        inv_lookup,
    );
    parity_revtab_generator(
        revtab,
        n,
        inv,
        offset + (len >> 0 as libc::c_int) + (len >> 1 as libc::c_int),
        1 as libc::c_int,
        1 as libc::c_int,
        len >> 1 as libc::c_int,
        basis,
        dual_stride,
        inv_lookup,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ff_tx_gen_split_radix_parity_revtab(
    mut s: *mut AVTXContext,
    mut len: libc::c_int,
    mut inv: libc::c_int,
    mut opts: *mut FFTXCodeletOptions,
    mut basis: libc::c_int,
    mut dual_stride: libc::c_int,
) -> libc::c_int {
    basis >>= 1 as libc::c_int;
    if len < basis {
        return -(22 as libc::c_int);
    }
    (*s).map = av_mallocz(
        (len as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if ((*s).map).is_null() {
        return -(12 as libc::c_int);
    }
    if !(dual_stride == 0 || dual_stride & dual_stride - 1 as libc::c_int == 0) {
        av_log(
            0 as *mut libc::c_void,
            0 as libc::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8 as *const libc::c_char,
            b"!dual_stride || !(dual_stride & (dual_stride - 1))\0" as *const u8
                as *const libc::c_char,
            b"libavutil/tx.c\0" as *const u8 as *const libc::c_char,
            251 as libc::c_int,
        );
        abort();
    }
    if !(dual_stride <= basis) {
        av_log(
            0 as *mut libc::c_void,
            0 as libc::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8 as *const libc::c_char,
            b"dual_stride <= basis\0" as *const u8 as *const libc::c_char,
            b"libavutil/tx.c\0" as *const u8 as *const libc::c_char,
            252 as libc::c_int,
        );
        abort();
    }
    parity_revtab_generator(
        (*s).map,
        len,
        inv,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        len,
        basis,
        dual_stride,
        if !opts.is_null() {
            ((*opts).map_dir as libc::c_uint == FF_TX_MAP_GATHER as libc::c_int as libc::c_uint)
                as libc::c_int
        } else {
            FF_TX_MAP_GATHER as libc::c_int
        },
    );
    (*s).map_dir = (if !opts.is_null() {
        (*opts).map_dir as libc::c_uint
    } else {
        FF_TX_MAP_GATHER as libc::c_int as libc::c_uint
    }) as FFTXMapDirection;
    return 0 as libc::c_int;
}
unsafe extern "C" fn reset_ctx(mut s: *mut AVTXContext, mut free_sub: libc::c_int) {
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
    if free_sub != 0 {
        av_freep(&mut (*s).sub as *mut *mut AVTXContext as *mut libc::c_void);
    }
    av_freep(&mut (*s).map as *mut *mut libc::c_int as *mut libc::c_void);
    av_freep(&mut (*s).exp as *mut *mut libc::c_void as *mut libc::c_void);
    av_freep(&mut (*s).tmp as *mut *mut libc::c_void as *mut libc::c_void);
    (*s).nb_sub = 0 as libc::c_int;
    (*s).opaque = 0 as *mut libc::c_void;
    memset(
        ((*s).fn_0).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<av_tx_fn>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ff_tx_clear_ctx(mut s: *mut AVTXContext) {
    reset_ctx(s, 0 as libc::c_int);
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn av_tx_uninit(mut ctx: *mut *mut AVTXContext) {
    if (*ctx).is_null() {
        return;
    }
    reset_ctx(*ctx, 1 as libc::c_int);
    av_freep(ctx as *mut libc::c_void);
}
#[cold]
unsafe extern "C" fn ff_tx_null_init(
    mut s: *mut AVTXContext,
    mut cd: *const FFTXCodelet,
    mut flags: uint64_t,
    mut opts: *mut FFTXCodeletOptions,
    mut len: libc::c_int,
    mut inv: libc::c_int,
    mut scale: *const libc::c_void,
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
    return 0 as libc::c_int;
}
unsafe extern "C" fn ff_tx_null(
    mut s: *mut AVTXContext,
    mut _out: *mut libc::c_void,
    mut _in: *mut libc::c_void,
    mut stride: ptrdiff_t,
) {
    memcpy(_out, _in, stride as libc::c_ulong);
}
static mut ff_tx_null_def: FFTXCodelet = unsafe {
    {
        let mut init = FFTXCodelet {
            name: b"null\0" as *const u8 as *const libc::c_char,
            function: Some(
                ff_tx_null
                    as unsafe extern "C" fn(
                        *mut AVTXContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        ptrdiff_t,
                    ) -> (),
            ),
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
            init: Some(
                ff_tx_null_init
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
            prio: FF_TX_PRIO_MAX as libc::c_int,
        };
        init
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
unsafe extern "C" fn get_codelet_prio(
    mut cd: *const FFTXCodelet,
    mut cpu_flags: libc::c_int,
    mut len: libc::c_int,
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
    return prio;
}
unsafe extern "C" fn cmp_decomp(
    mut a: *mut FFTXLenDecomp,
    mut b: *mut FFTXLenDecomp,
) -> libc::c_int {
    return ((*b).prio > (*a).prio) as libc::c_int - ((*b).prio < (*a).prio) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_tx_decompose_length(
    mut dst: *mut libc::c_int,
    mut type_0: AVTXType,
    mut len: libc::c_int,
    mut inv: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut nb_decomp: libc::c_int = 0 as libc::c_int;
    let mut ld: [FFTXLenDecomp; 512] = [FFTXLenDecomp {
        len: 0,
        len2: 0,
        prio: 0,
        cd: 0 as *const FFTXCodelet,
    }; 512];
    let mut codelet_list_idx: libc::c_int = codelet_list_num;
    let cpu_flags: libc::c_int = av_get_cpu_flags();
    's_9: loop {
        let fresh6 = codelet_list_idx;
        codelet_list_idx = codelet_list_idx - 1;
        if !(fresh6 != 0) {
            current_block = 4567019141635105728;
            break;
        }
        let mut list: *const *const FFTXCodelet = codelet_list[codelet_list_idx as usize];
        let mut cd: *const FFTXCodelet = 0 as *const FFTXCodelet;
        loop {
            let fresh7 = list;
            list = list.offset(1);
            cd = *fresh7;
            if cd.is_null() {
                break;
            }
            let mut fl: libc::c_int = len;
            let mut skip: libc::c_int = 0 as libc::c_int;
            let mut prio: libc::c_int = 0;
            let mut factors_product: libc::c_int = 1 as libc::c_int;
            let mut factors_mod: libc::c_int = 0 as libc::c_int;
            if nb_decomp >= 512 as libc::c_int {
                current_block = 12954599432099290578;
                break 's_9;
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
            if (*cd).cpu_flags != 0 as libc::c_int
                && cpu_flags & ((*cd).cpu_flags & !cpu_slow_mask) == 0
            {
                continue;
            }
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < 16 as libc::c_int {
                if (*cd).factors[i as usize] == 0 || fl == 1 as libc::c_int {
                    break;
                }
                if (*cd).factors[i as usize] == -(1 as libc::c_int) {
                    factors_mod += 1;
                    factors_mod;
                    factors_product *= fl;
                } else if fl % (*cd).factors[i as usize] == 0 {
                    factors_mod += 1;
                    factors_mod;
                    if (*cd).factors[i as usize] == 2 as libc::c_int {
                        let mut b: libc::c_int = ff_ctz_c(fl);
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
            if av_gcd(factors_product as int64_t, fl as int64_t) != 1 as libc::c_int as libc::c_long
            {
                continue;
            }
            if factors_product < (*cd).min_len
                || (*cd).max_len != -(1 as libc::c_int) && factors_product > (*cd).max_len
            {
                continue;
            }
            prio = get_codelet_prio(cd, cpu_flags, factors_product) * factors_product;
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while i_0 < nb_decomp {
                if factors_product == ld[i_0 as usize].len {
                    if prio > ld[i_0 as usize].prio {
                        ld[i_0 as usize].prio = prio;
                    }
                    skip = 1 as libc::c_int;
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
                return -(22 as libc::c_int);
            }
        }
        _ => {}
    }
    let mut stack: [[*mut libc::c_void; 2]; 64] = [[0 as *mut libc::c_void; 2]; 64];
    let mut sp: libc::c_int = 1 as libc::c_int;
    stack[0 as libc::c_int as usize][0 as libc::c_int as usize] =
        ld.as_mut_ptr() as *mut libc::c_void;
    stack[0 as libc::c_int as usize][1 as libc::c_int as usize] =
        ld.as_mut_ptr()
            .offset(nb_decomp as isize)
            .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void;
    while sp != 0 {
        sp -= 1;
        let mut start: *mut FFTXLenDecomp =
            stack[sp as usize][0 as libc::c_int as usize] as *mut FFTXLenDecomp;
        let mut end: *mut FFTXLenDecomp =
            stack[sp as usize][1 as libc::c_int as usize] as *mut FFTXLenDecomp;
        while start < end {
            if start < end.offset(-(1 as libc::c_int as isize)) {
                let mut checksort: libc::c_int = 0 as libc::c_int;
                let mut right: *mut FFTXLenDecomp = end.offset(-(2 as libc::c_int as isize));
                let mut left: *mut FFTXLenDecomp = start.offset(1 as libc::c_int as isize);
                let mut mid: *mut FFTXLenDecomp = start
                    .offset((end.offset_from(start) as libc::c_long >> 1 as libc::c_int) as isize);
                if cmp_decomp(start, end) > 0 as libc::c_int {
                    if cmp_decomp(end, mid) > 0 as libc::c_int {
                        let mut SWAP_tmp: FFTXLenDecomp = *mid;
                        *mid = *start;
                        *start = SWAP_tmp;
                    } else {
                        let mut SWAP_tmp_0: FFTXLenDecomp = *end;
                        *end = *start;
                        *start = SWAP_tmp_0;
                    }
                } else if cmp_decomp(start, mid) > 0 as libc::c_int {
                    let mut SWAP_tmp_1: FFTXLenDecomp = *mid;
                    *mid = *start;
                    *start = SWAP_tmp_1;
                } else {
                    checksort = 1 as libc::c_int;
                }
                if cmp_decomp(mid, end) > 0 as libc::c_int {
                    let mut SWAP_tmp_2: FFTXLenDecomp = *end;
                    *end = *mid;
                    *mid = SWAP_tmp_2;
                    checksort = 0 as libc::c_int;
                }
                if start == end.offset(-(2 as libc::c_int as isize)) {
                    break;
                }
                let mut SWAP_tmp_3: FFTXLenDecomp = *mid;
                *mid = *end.offset(-(1 as libc::c_int) as isize);
                *end.offset(-(1 as libc::c_int) as isize) = SWAP_tmp_3;
                while left <= right {
                    while left <= right
                        && cmp_decomp(left, end.offset(-(1 as libc::c_int as isize)))
                            < 0 as libc::c_int
                    {
                        left = left.offset(1);
                        left;
                    }
                    while left <= right
                        && cmp_decomp(right, end.offset(-(1 as libc::c_int as isize)))
                            > 0 as libc::c_int
                    {
                        right = right.offset(-1);
                        right;
                    }
                    if left <= right {
                        let mut SWAP_tmp_4: FFTXLenDecomp = *right;
                        *right = *left;
                        *left = SWAP_tmp_4;
                        left = left.offset(1);
                        left;
                        right = right.offset(-1);
                        right;
                    }
                }
                let mut SWAP_tmp_5: FFTXLenDecomp = *left;
                *left = *end.offset(-(1 as libc::c_int) as isize);
                *end.offset(-(1 as libc::c_int) as isize) = SWAP_tmp_5;
                if checksort != 0
                    && (mid == left.offset(-(1 as libc::c_int as isize)) || mid == left)
                {
                    mid = start;
                    while mid < end
                        && cmp_decomp(mid, mid.offset(1 as libc::c_int as isize))
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
                    let fresh8 = sp;
                    sp = sp + 1;
                    stack[fresh8 as usize][1 as libc::c_int as usize] = right as *mut libc::c_void;
                    start = left.offset(1 as libc::c_int as isize);
                } else {
                    stack[sp as usize][0 as libc::c_int as usize] =
                        left.offset(1 as libc::c_int as isize) as *mut libc::c_void;
                    let fresh9 = sp;
                    sp = sp + 1;
                    stack[fresh9 as usize][1 as libc::c_int as usize] = end as *mut libc::c_void;
                    end = right;
                }
            } else {
                if cmp_decomp(start, end) > 0 as libc::c_int {
                    let mut SWAP_tmp_6: FFTXLenDecomp = *end;
                    *end = *start;
                    *start = SWAP_tmp_6;
                }
                break;
            }
        }
    }
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < nb_decomp {
        if (*ld[i_1 as usize].cd).nb_factors > 1 as libc::c_int {
            *dst.offset(i_1 as isize) = ld[i_1 as usize].len2;
        } else {
            *dst.offset(i_1 as isize) = ld[i_1 as usize].len;
        }
        i_1 += 1;
        i_1;
    }
    return nb_decomp;
}
#[no_mangle]
pub unsafe extern "C" fn ff_tx_gen_default_map(
    mut s: *mut AVTXContext,
    mut opts: *mut FFTXCodeletOptions,
) -> libc::c_int {
    (*s).map = av_malloc(
        ((*s).len as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if ((*s).map).is_null() {
        return -(12 as libc::c_int);
    }
    *((*s).map).offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    if (*s).inv != 0 {
        let mut i: libc::c_int = 1 as libc::c_int;
        while i < (*s).len {
            *((*s).map).offset(i as isize) = (*s).len - i;
            i += 1;
            i;
        }
    } else {
        let mut i_0: libc::c_int = 1 as libc::c_int;
        while i_0 < (*s).len {
            *((*s).map).offset(i_0 as isize) = i_0;
            i_0 += 1;
            i_0;
        }
    }
    (*s).map_dir = FF_TX_MAP_GATHER;
    return 0 as libc::c_int;
}
unsafe extern "C" fn print_flags(mut bp: *mut AVBPrint, mut f: uint64_t) {
    let mut prev: libc::c_int = 0 as libc::c_int;
    let mut sep: *const libc::c_char = b", \0" as *const u8 as *const libc::c_char;
    av_bprintf(bp, b"flags: [\0" as *const u8 as *const libc::c_char);
    if f as libc::c_ulonglong & (1 as libc::c_ulonglong) << 62 as libc::c_int != 0 && {
        prev += 1;
        prev != 0
    } {
        av_bprintf(bp, b"aligned\0" as *const u8 as *const libc::c_char);
    }
    if f & AV_TX_UNALIGNED as libc::c_int as libc::c_ulong != 0 && {
        prev += 1;
        prev != 0
    } {
        av_bprintf(
            bp,
            b"%sunaligned\0" as *const u8 as *const libc::c_char,
            if prev > 1 as libc::c_int {
                sep
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if f & AV_TX_INPLACE as libc::c_int as libc::c_ulong != 0 && {
        prev += 1;
        prev != 0
    } {
        av_bprintf(
            bp,
            b"%sinplace\0" as *const u8 as *const libc::c_char,
            if prev > 1 as libc::c_int {
                sep
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if f as libc::c_ulonglong & (1 as libc::c_ulonglong) << 63 as libc::c_int != 0 && {
        prev += 1;
        prev != 0
    } {
        av_bprintf(
            bp,
            b"%sout_of_place\0" as *const u8 as *const libc::c_char,
            if prev > 1 as libc::c_int {
                sep
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if f as libc::c_ulonglong & (1 as libc::c_ulonglong) << 59 as libc::c_int != 0 && {
        prev += 1;
        prev != 0
    } {
        av_bprintf(
            bp,
            b"%sfwd_only\0" as *const u8 as *const libc::c_char,
            if prev > 1 as libc::c_int {
                sep
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if f as libc::c_ulonglong & (1 as libc::c_ulonglong) << 60 as libc::c_int != 0 && {
        prev += 1;
        prev != 0
    } {
        av_bprintf(
            bp,
            b"%sinv_only\0" as *const u8 as *const libc::c_char,
            if prev > 1 as libc::c_int {
                sep
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if f as libc::c_ulonglong & (1 as libc::c_ulonglong) << 61 as libc::c_int != 0 && {
        prev += 1;
        prev != 0
    } {
        av_bprintf(
            bp,
            b"%spreshuf\0" as *const u8 as *const libc::c_char,
            if prev > 1 as libc::c_int {
                sep
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if f & AV_TX_FULL_IMDCT as libc::c_int as libc::c_ulong != 0 && {
        prev += 1;
        prev != 0
    } {
        av_bprintf(
            bp,
            b"%simdct_full\0" as *const u8 as *const libc::c_char,
            if prev > 1 as libc::c_int {
                sep
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if f & AV_TX_REAL_TO_REAL as libc::c_int as libc::c_ulong != 0 && {
        prev += 1;
        prev != 0
    } {
        av_bprintf(
            bp,
            b"%sreal_to_real\0" as *const u8 as *const libc::c_char,
            if prev > 1 as libc::c_int {
                sep
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if f & AV_TX_REAL_TO_IMAGINARY as libc::c_int as libc::c_ulong != 0 && {
        prev += 1;
        prev != 0
    } {
        av_bprintf(
            bp,
            b"%sreal_to_imaginary\0" as *const u8 as *const libc::c_char,
            if prev > 1 as libc::c_int {
                sep
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if f as libc::c_ulonglong & (1 as libc::c_ulonglong) << 58 as libc::c_int != 0 && {
        prev += 1;
        prev != 0
    } {
        av_bprintf(
            bp,
            b"%sasm_call\0" as *const u8 as *const libc::c_char,
            if prev > 1 as libc::c_int {
                sep
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
    }
    av_bprintf(bp, b"]\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn print_type(mut bp: *mut AVBPrint, mut type_0: AVTXType) {
    av_bprintf(
        bp,
        b"%s\0" as *const u8 as *const libc::c_char,
        if type_0 as libc::c_uint == 2147483647 as libc::c_int as libc::c_uint {
            b"any\0" as *const u8 as *const libc::c_char
        } else if type_0 as libc::c_uint == AV_TX_FLOAT_FFT as libc::c_int as libc::c_uint {
            b"fft_float\0" as *const u8 as *const libc::c_char
        } else if type_0 as libc::c_uint == AV_TX_FLOAT_MDCT as libc::c_int as libc::c_uint {
            b"mdct_float\0" as *const u8 as *const libc::c_char
        } else if type_0 as libc::c_uint == AV_TX_FLOAT_RDFT as libc::c_int as libc::c_uint {
            b"rdft_float\0" as *const u8 as *const libc::c_char
        } else if type_0 as libc::c_uint == AV_TX_FLOAT_DCT_I as libc::c_int as libc::c_uint {
            b"dctI_float\0" as *const u8 as *const libc::c_char
        } else if type_0 as libc::c_uint == AV_TX_FLOAT_DST_I as libc::c_int as libc::c_uint {
            b"dstI_float\0" as *const u8 as *const libc::c_char
        } else if type_0 as libc::c_uint == AV_TX_DOUBLE_FFT as libc::c_int as libc::c_uint {
            b"fft_double\0" as *const u8 as *const libc::c_char
        } else if type_0 as libc::c_uint == AV_TX_DOUBLE_MDCT as libc::c_int as libc::c_uint {
            b"mdct_double\0" as *const u8 as *const libc::c_char
        } else if type_0 as libc::c_uint == AV_TX_DOUBLE_RDFT as libc::c_int as libc::c_uint {
            b"rdft_double\0" as *const u8 as *const libc::c_char
        } else if type_0 as libc::c_uint == AV_TX_DOUBLE_DCT_I as libc::c_int as libc::c_uint {
            b"dctI_double\0" as *const u8 as *const libc::c_char
        } else if type_0 as libc::c_uint == AV_TX_DOUBLE_DST_I as libc::c_int as libc::c_uint {
            b"dstI_double\0" as *const u8 as *const libc::c_char
        } else if type_0 as libc::c_uint == AV_TX_INT32_FFT as libc::c_int as libc::c_uint {
            b"fft_int32\0" as *const u8 as *const libc::c_char
        } else if type_0 as libc::c_uint == AV_TX_INT32_MDCT as libc::c_int as libc::c_uint {
            b"mdct_int32\0" as *const u8 as *const libc::c_char
        } else if type_0 as libc::c_uint == AV_TX_INT32_RDFT as libc::c_int as libc::c_uint {
            b"rdft_int32\0" as *const u8 as *const libc::c_char
        } else if type_0 as libc::c_uint == AV_TX_INT32_DCT_I as libc::c_int as libc::c_uint {
            b"dctI_int32\0" as *const u8 as *const libc::c_char
        } else if type_0 as libc::c_uint == AV_TX_INT32_DST_I as libc::c_int as libc::c_uint {
            b"dstI_int32\0" as *const u8 as *const libc::c_char
        } else {
            b"unknown\0" as *const u8 as *const libc::c_char
        },
    );
}
unsafe extern "C" fn print_cd_info(
    mut cd: *const FFTXCodelet,
    mut prio: libc::c_int,
    mut len: libc::c_int,
    mut print_prio: libc::c_int,
) {
    let mut bp: AVBPrint = AVBPrint {
        str_0: 0 as *mut libc::c_char,
        len: 0,
        size: 0,
        size_max: 0,
        reserved_internal_buffer: [0; 1],
        reserved_padding: [0; 1000],
    };
    av_bprint_init(
        &mut bp,
        0 as libc::c_int as libc::c_uint,
        1 as libc::c_int as libc::c_uint,
    );
    av_bprintf(
        &mut bp as *mut AVBPrint,
        b"%s - type: \0" as *const u8 as *const libc::c_char,
        (*cd).name,
    );
    print_type(&mut bp, (*cd).type_0);
    av_bprintf(
        &mut bp as *mut AVBPrint,
        b", len: \0" as *const u8 as *const libc::c_char,
    );
    if len == 0 {
        if (*cd).min_len != (*cd).max_len {
            av_bprintf(
                &mut bp as *mut AVBPrint,
                b"[%i, \0" as *const u8 as *const libc::c_char,
                (*cd).min_len,
            );
        }
        if (*cd).max_len == -(1 as libc::c_int) {
            av_bprintf(
                &mut bp as *mut AVBPrint,
                b"\xE2\x88\x9E\0" as *const u8 as *const libc::c_char,
            );
        } else {
            av_bprintf(
                &mut bp as *mut AVBPrint,
                b"%i\0" as *const u8 as *const libc::c_char,
                (*cd).max_len,
            );
        }
    } else {
        av_bprintf(
            &mut bp as *mut AVBPrint,
            b"%i\0" as *const u8 as *const libc::c_char,
            len,
        );
    }
    if (*cd).factors[1 as libc::c_int as usize] != 0 {
        av_bprintf(
            &mut bp as *mut AVBPrint,
            b"%s, factors\0" as *const u8 as *const libc::c_char,
            if len == 0 && (*cd).min_len != (*cd).max_len {
                b"]\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
        if (*cd).nb_factors == 0 {
            av_bprintf(
                &mut bp as *mut AVBPrint,
                b": [\0" as *const u8 as *const libc::c_char,
            );
        } else {
            av_bprintf(
                &mut bp as *mut AVBPrint,
                b"[%i]: [\0" as *const u8 as *const libc::c_char,
                (*cd).nb_factors,
            );
        }
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 16 as libc::c_int {
            if i != 0 && (*cd).factors[i as usize] != 0 {
                av_bprintf(
                    &mut bp as *mut AVBPrint,
                    b", \0" as *const u8 as *const libc::c_char,
                );
            }
            if (*cd).factors[i as usize] == -(1 as libc::c_int) {
                av_bprintf(
                    &mut bp as *mut AVBPrint,
                    b"any\0" as *const u8 as *const libc::c_char,
                );
            } else {
                if !((*cd).factors[i as usize] != 0) {
                    break;
                }
                av_bprintf(
                    &mut bp as *mut AVBPrint,
                    b"%i\0" as *const u8 as *const libc::c_char,
                    (*cd).factors[i as usize],
                );
            }
            i += 1;
            i;
        }
        av_bprintf(
            &mut bp as *mut AVBPrint,
            b"], \0" as *const u8 as *const libc::c_char,
        );
    } else {
        av_bprintf(
            &mut bp as *mut AVBPrint,
            b"%s, factor: %i, \0" as *const u8 as *const libc::c_char,
            if len == 0 && (*cd).min_len != (*cd).max_len {
                b"]\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            (*cd).factors[0 as libc::c_int as usize],
        );
    }
    print_flags(&mut bp, (*cd).flags);
    if print_prio != 0 {
        av_bprintf(
            &mut bp as *mut AVBPrint,
            b", prio: %i\0" as *const u8 as *const libc::c_char,
            prio,
        );
    }
    av_log(
        0 as *mut libc::c_void,
        48 as libc::c_int,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        bp.str_0,
    );
}
unsafe extern "C" fn print_tx_structure(mut s: *mut AVTXContext, mut depth: libc::c_int) {
    let mut cd: *const FFTXCodelet = (*s).cd_self;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i <= depth {
        av_log(
            0 as *mut libc::c_void,
            48 as libc::c_int,
            b"    \0" as *const u8 as *const libc::c_char,
        );
        i += 1;
        i;
    }
    print_cd_info(cd, (*cd).prio, (*s).len, 0 as libc::c_int);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < (*s).nb_sub {
        print_tx_structure(
            &mut *((*s).sub).offset(i_0 as isize),
            depth + 1 as libc::c_int,
        );
        i_0 += 1;
        i_0;
    }
}
unsafe extern "C" fn cmp_matches(
    mut a: *mut TXCodeletMatch,
    mut b: *mut TXCodeletMatch,
) -> libc::c_int {
    return ((*b).prio > (*a).prio) as libc::c_int - ((*b).prio < (*a).prio) as libc::c_int;
}
#[inline]
unsafe extern "C" fn check_cd_factors(
    mut cd: *const FFTXCodelet,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut matches: libc::c_int = 0 as libc::c_int;
    let mut any_flag: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        let mut factor: libc::c_int = (*cd).factors[i as usize];
        if factor == -(1 as libc::c_int) {
            any_flag = 1 as libc::c_int;
            matches += 1;
            matches;
        } else {
            if len <= 1 as libc::c_int || factor == 0 {
                break;
            }
            if factor == 2 as libc::c_int {
                let mut bits_2: libc::c_int = ff_ctz_c(len);
                if !(bits_2 == 0) {
                    len >>= bits_2;
                    matches += 1;
                    matches;
                }
            } else {
                let mut res: libc::c_int = len % factor;
                if !(res != 0) {
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
    return ((*cd).nb_factors <= matches && (any_flag != 0 || len == 1 as libc::c_int))
        as libc::c_int;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn ff_tx_init_subtx(
    mut s: *mut AVTXContext,
    mut type_0: AVTXType,
    mut flags: uint64_t,
    mut opts: *mut FFTXCodeletOptions,
    mut len: libc::c_int,
    mut inv: libc::c_int,
    mut scale: *const libc::c_void,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut sub: *mut AVTXContext = 0 as *mut AVTXContext;
    let mut cd_tmp: *mut TXCodeletMatch = 0 as *mut TXCodeletMatch;
    let mut cd_matches: *mut TXCodeletMatch = 0 as *mut TXCodeletMatch;
    let mut cd_matches_size: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut codelet_list_idx: libc::c_int = codelet_list_num;
    let mut nb_cd_matches: libc::c_int = 0 as libc::c_int;
    let mut bp: AVBPrint = AVBPrint {
        str_0: 0 as *mut libc::c_char,
        len: 0,
        size: 0,
        size_max: 0,
        reserved_internal_buffer: [0; 1],
        reserved_padding: [0; 1000],
    };
    let cpu_flags: libc::c_int = av_get_cpu_flags();
    let mut req_flags: uint64_t = flags;
    let mut inv_req_mask: uint64_t =
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
        codelet_list_idx = codelet_list_idx - 1;
        if !(fresh10 != 0) {
            break;
        }
        let mut list: *const *const FFTXCodelet = codelet_list[codelet_list_idx as usize];
        let mut cd: *const FFTXCodelet = 0 as *const FFTXCodelet;
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
            let ref mut fresh12 = (*cd_matches.offset(nb_cd_matches as isize)).cd;
            *fresh12 = cd;
            (*cd_matches.offset(nb_cd_matches as isize)).prio =
                get_codelet_prio(cd, cpu_flags, len);
            nb_cd_matches += 1;
            nb_cd_matches;
        }
    }
    av_bprint_init(
        &mut bp,
        0 as libc::c_int as libc::c_uint,
        1 as libc::c_int as libc::c_uint,
    );
    av_bprintf(
        &mut bp as *mut AVBPrint,
        b"For transform of length %i, %s, \0" as *const u8 as *const libc::c_char,
        len,
        if inv != 0 {
            b"inverse\0" as *const u8 as *const libc::c_char
        } else {
            b"forward\0" as *const u8 as *const libc::c_char
        },
    );
    print_type(&mut bp, type_0);
    av_bprintf(
        &mut bp as *mut AVBPrint,
        b", \0" as *const u8 as *const libc::c_char,
    );
    print_flags(&mut bp, flags);
    av_bprintf(
        &mut bp as *mut AVBPrint,
        b", found %i matches%s\0" as *const u8 as *const libc::c_char,
        nb_cd_matches,
        if nb_cd_matches != 0 {
            b":\0" as *const u8 as *const libc::c_char
        } else {
            b".\0" as *const u8 as *const libc::c_char
        },
    );
    if nb_cd_matches == 0 {
        return -(38 as libc::c_int);
    }
    let mut stack: [[*mut libc::c_void; 2]; 64] = [[0 as *mut libc::c_void; 2]; 64];
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
                        let mut SWAP_tmp: TXCodeletMatch = *mid;
                        *mid = *start;
                        *start = SWAP_tmp;
                    } else {
                        let mut SWAP_tmp_0: TXCodeletMatch = *end;
                        *end = *start;
                        *start = SWAP_tmp_0;
                    }
                } else if cmp_matches(start, mid) > 0 as libc::c_int {
                    let mut SWAP_tmp_1: TXCodeletMatch = *mid;
                    *mid = *start;
                    *start = SWAP_tmp_1;
                } else {
                    checksort = 1 as libc::c_int;
                }
                if cmp_matches(mid, end) > 0 as libc::c_int {
                    let mut SWAP_tmp_2: TXCodeletMatch = *end;
                    *end = *mid;
                    *mid = SWAP_tmp_2;
                    checksort = 0 as libc::c_int;
                }
                if start == end.offset(-(2 as libc::c_int as isize)) {
                    break;
                }
                let mut SWAP_tmp_3: TXCodeletMatch = *mid;
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
                        let mut SWAP_tmp_4: TXCodeletMatch = *right;
                        *right = *left;
                        *left = SWAP_tmp_4;
                        left = left.offset(1);
                        left;
                        right = right.offset(-1);
                        right;
                    }
                }
                let mut SWAP_tmp_5: TXCodeletMatch = *left;
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
                    sp = sp + 1;
                    stack[fresh13 as usize][1 as libc::c_int as usize] = right as *mut libc::c_void;
                    start = left.offset(1 as libc::c_int as isize);
                } else {
                    stack[sp as usize][0 as libc::c_int as usize] =
                        left.offset(1 as libc::c_int as isize) as *mut libc::c_void;
                    let fresh14 = sp;
                    sp = sp + 1;
                    stack[fresh14 as usize][1 as libc::c_int as usize] = end as *mut libc::c_void;
                    end = right;
                }
            } else {
                if cmp_matches(start, end) > 0 as libc::c_int {
                    let mut SWAP_tmp_6: TXCodeletMatch = *end;
                    *end = *start;
                    *start = SWAP_tmp_6;
                }
                break;
            }
        }
    }
    av_log(
        0 as *mut libc::c_void,
        48 as libc::c_int,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        bp.str_0,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < nb_cd_matches {
        av_log(
            0 as *mut libc::c_void,
            48 as libc::c_int,
            b"    %i: \0" as *const u8 as *const libc::c_char,
            i + 1 as libc::c_int,
        );
        print_cd_info(
            (*cd_matches.offset(i as isize)).cd,
            (*cd_matches.offset(i as isize)).prio,
            0 as libc::c_int,
            1 as libc::c_int,
        );
        i += 1;
        i;
    }
    if ((*s).sub).is_null() {
        sub = av_mallocz(
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<AVTXContext>() as libc::c_ulong),
        ) as *mut AVTXContext;
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
    match current_block {
        5706227035632243100 => {
            let mut i_0: libc::c_int = 0 as libc::c_int;
            loop {
                if !(i_0 < nb_cd_matches) {
                    current_block = 16937825661756021828;
                    break;
                }
                let mut cd_0: *const FFTXCodelet = (*cd_matches.offset(i_0 as isize)).cd;
                let mut sctx: *mut AVTXContext =
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
                        (*sctx).map =
                            av_malloc((len as libc::c_ulong).wrapping_mul(::core::mem::size_of::<
                                libc::c_int,
                            >(
                            )
                                as libc::c_ulong)) as *mut libc::c_int;
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
                        let mut tmp: *mut libc::c_int =
                            av_malloc((len as libc::c_ulong).wrapping_mul(::core::mem::size_of::<
                                libc::c_int,
                            >(
                            )
                                as libc::c_ulong)) as *mut libc::c_int;
                        if tmp.is_null() {
                            ret = -(12 as libc::c_int);
                            current_block = 7391434065428304855;
                            break;
                        } else {
                            memcpy(
                                tmp as *mut libc::c_void,
                                (*sctx).map as *const libc::c_void,
                                (len as libc::c_ulong).wrapping_mul(::core::mem::size_of::<
                                    libc::c_int,
                                >(
                                )
                                    as libc::c_ulong),
                            );
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
                    (*s).cd[(*s).nb_sub as usize] = 0 as *const FFTXCodelet;
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
        _ => {}
    }
    av_free(cd_matches as *mut libc::c_void);
    return ret;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn av_tx_init(
    mut ctx: *mut *mut AVTXContext,
    mut tx: *mut av_tx_fn,
    mut type_0: AVTXType,
    mut inv: libc::c_int,
    mut len: libc::c_int,
    mut scale: *const libc::c_void,
    mut flags: uint64_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut tmp: AVTXContext = {
        let mut init = AVTXContext {
            len: 0 as libc::c_int,
            inv: 0,
            map: 0 as *mut libc::c_int,
            exp: 0 as *mut libc::c_void,
            tmp: 0 as *mut libc::c_void,
            sub: 0 as *mut AVTXContext,
            fn_0: [None; 4],
            nb_sub: 0,
            cd: [0 as *const FFTXCodelet; 4],
            cd_self: 0 as *const FFTXCodelet,
            type_0: AV_TX_FLOAT_FFT,
            flags: 0,
            map_dir: FF_TX_MAP_NONE,
            scale_f: 0.,
            scale_d: 0.,
            opaque: 0 as *mut libc::c_void,
        };
        init
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
        0 as *mut FFTXCodeletOptions,
        len,
        inv,
        scale,
    );
    if ret < 0 as libc::c_int {
        return ret;
    }
    *ctx = &mut *(tmp.sub).offset(0 as libc::c_int as isize) as *mut AVTXContext;
    *tx = tmp.fn_0[0 as libc::c_int as usize];
    av_log(
        0 as *mut libc::c_void,
        48 as libc::c_int,
        b"Transform tree:\n\0" as *const u8 as *const libc::c_char,
    );
    print_tx_structure(*ctx, 0 as libc::c_int);
    return ret;
}
unsafe extern "C" fn run_static_initializers() {
    codelet_list_num = (::core::mem::size_of::<[*const *const FFTXCodelet; 4]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<*const *const FFTXCodelet>() as libc::c_ulong)
        as libc::c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
