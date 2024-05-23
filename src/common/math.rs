use std::f32;

use ffmpeg_src_macro::ffmpeg_src;
use libc::c_float;

pub(crate) trait Exp10 {
    /// Compute `10^x` for floating point values.
    ///
    /// Note: this function is by no means "correctly rounded", and is meant as
    /// a fast, reasonably accurate approximation. For instance, maximum
    /// relative error for the double precision variant is ~ 1e-13 for very
    /// small and very large values. This is ~2x faster than GNU libm's
    /// approach, which is still off by 2ulp on some inputs.
    #[ffmpeg_src(file = "libavutil/ffmath.h", lines = 32..=50, name = "ff_exp10")]
    fn exp10(x: Self) -> Self;
}

impl Exp10 for c_float {
    #[inline(always)]
    fn exp10(x: Self) -> Self {
        (f32::consts::LOG2_10 * x).exp2()
    }
}
