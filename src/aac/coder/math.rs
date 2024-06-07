use ffmpeg_src_macro::ffmpeg_src;
use ilog::IntLog;
use libc::{c_float, c_int, c_uchar, c_uint};

use crate::aac::{SCALE_DIV_512, SCALE_ONE_POS};

pub(super) trait Float {
    /// Return the minimum scalefactor where the quantized coef does not clip.
    #[ffmpeg_src(file = "libavcodec/aacenc_utils.h", lines = 156..=160)]
    fn coef2minsf(self) -> c_uchar;

    /// Compute `x^y`` for floating point `x`, `y`.
    ///
    /// Note: this function is faster than the libm variant due to mainly 2
    /// reasons:
    /// 1. It does not handle any edge cases. In particular, this is only
    ///    guaranteed to work correctly for `x > 0`.
    /// 2. It is not as accurate as a standard nearly "correctly rounded" libm
    ///    variant.
    #[ffmpeg_src(file = "libavutil/ffmath.h", lines = 52..=65, name = "ff_fast_powf")]
    fn fast_powf(self, y: Self) -> Self;

    /// approximates
    /// `exp10f(-3.0f*(0.5f + 0.5f * cosf(FFMIN(b,15.5f) / 15.5f)))`
    #[ffmpeg_src(file = "libavcodec/aacenc_utils.h", lines = 185..=191)]
    fn bval2bmax(self) -> Self;
}

impl Float for c_float {
    #[inline]
    fn coef2minsf(self) -> c_uchar {
        /// Clip a signed integer value into the 0-255 range.
        #[ffmpeg_src(file = "libavutil/common.h", lines = 204..=213, name = "av_clip_uint8_c")]
        #[inline(always)]
        fn clip_uint8_c(a: c_int) -> c_uchar {
            a.clamp(c_uchar::MIN.into(), c_uchar::MAX.into()) as c_uchar
        }

        clip_uint8_c(
            (self.log2() * 4. - 69. + Self::from(SCALE_ONE_POS) - Self::from(SCALE_DIV_512))
                as c_int,
        )
    }

    #[inline(always)]
    fn fast_powf(self, y: Self) -> Self {
        (self.ln() * y).exp()
    }

    #[inline(always)]
    fn bval2bmax(self) -> Self {
        0.001 + 0.0035 * self.powi(3) / 15.5_f32.powi(3)
    }
}

#[inline(always)]
pub(super) fn ff_log2_c(v: c_uint) -> c_int {
    // TODO: is this (the cast) correct??
    v.log2() as c_int
    // let mut n: c_int = 0;
    // if v & 0xffff0000 as c_uint != 0 {
    //     v >>= 16;
    //     n += 16;
    // }
    // if v & 0xff00 as c_int as c_uint != 0 {
    //     v >>= 8;
    //     n += 8;
    // }
    // n += ff_log2_tab[v as usize] as c_int;
    // return n;
}

/// Clip a signed integer to an unsigned power of two range.
#[ffmpeg_src(file = "libavutil/common.h", lines = 273..=283, name = "av_clip_uintp2_c")]
#[inline(always)]
pub(super) fn clip_uintp2_c(a: c_int, p: c_int) -> c_uint {
    (if a & !((1 << p) - 1) != 0 {
        !a >> 31 & ((1 << p) - 1)
    } else {
        a
    }) as c_uint
}

/// Linear congruential pseudorandom number generator
#[ffmpeg_src(file = "libavcodec/aacenc_utils.h", lines = 231..=242)]
#[inline(always)]
pub(super) fn lcg_random(previous_val: c_uint) -> c_int {
    previous_val
        .wrapping_mul(1_664_525)
        .wrapping_add(1_013_904_223) as c_int
}
