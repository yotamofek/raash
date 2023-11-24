//! [Linear predictive coding](https://en.wikipedia.org/wiki/Linear_predictive_coding)

use std::{f64::consts::PI, iter::zip};

use itertools::izip;
use libc::{c_double, c_float, c_int, c_long};

/// LPC analysis type
///
/// Source: [libavcodec/lpc.h](https://github.com/ffmpeg/ffmpeg/blob/c8c4a162fc18c0fd99bada66d9ea3b48c64b2450/libavcodec/lpc.h#L41-L51)
#[repr(i32)]
#[derive(Clone, Copy)]
pub enum Type {
    /// Levinson-Durbin recursion
    Levinson = 2,
}

// const MIN_ORDER: c_int = 1;
const MAX_ORDER: c_int = 32;

#[derive(Clone)]
#[repr(C)]
pub struct LPCContext {
    blocksize: c_int,
    max_order: c_int,
    lpc_type: Type,
    // original ffmpeg code uses two pointers to the same allocation, `windowed_samples`
    // points to `windowed_buffer + padding_size` and so can be indexed with negative
    // numbers. we just add `padding_size` to the used indices instead.
    windowed_samples: Box<[c_double]>,
}

/// Schur recursion.
/// Produces reflection coefficients from autocorrelation data.
///
/// Source: [avcodec/lpc.h](https://github.com/ffmpeg/ffmpeg/blob/c8c4a162fc18c0fd99bada66d9ea3b48c64b2450/libavcodec/lpc.h#L136-L161)
#[inline]
fn compute_ref_coefs(
    autoc: &[c_double],
    max_order: c_int,
    ref_0: &mut [c_double],
    mut error: Option<&mut [c_double]>,
) {
    let mut gen0 = [0.; MAX_ORDER as usize];
    let mut gen1 = [0.; MAX_ORDER as usize];
    let mut i = 0 as c_int;
    while i < max_order {
        gen1[i as usize] = autoc[(i + 1) as usize];
        gen0[i as usize] = gen1[i as usize];
        i += 1;
    }
    let mut err = autoc[0];
    ref_0[0] = -gen1[0] / if err != 0. { err } else { 1. };
    err += gen1[0] * ref_0[0];
    if let Some(error) = &mut error {
        error[0] = err;
    }
    let mut i = 1 as c_int;
    while i < max_order {
        let mut j = 0 as c_int;
        while j < max_order - i {
            gen1[j as usize] = gen1[(j + 1) as usize] + ref_0[(i - 1) as usize] * gen0[j as usize];
            gen0[j as usize] += gen1[(j + 1) as usize] * ref_0[(i - 1) as usize];
            j += 1;
        }
        ref_0[i as usize] = -gen1[0 as c_int as usize] / if err != 0. { err } else { 1. };
        err += gen1[0 as c_int as usize] * ref_0[i as usize];
        if let Some(error) = &mut error {
            error[i as usize] = err;
        }
        i += 1;
    }
}

impl LPCContext {
    const fn padding_size(max_order: c_int) -> usize {
        /// Source: [libavutil/macros.h](https://github.com/ffmpeg/ffmpeg/blob/2dd8acbe800f6ea3b72ebe730f8ed95a5c3dd407/libavutil/macros.h#L78)
        const fn align(x: c_int, a: c_int) -> c_int {
            (x + a - 1) & !(a - 1)
        }

        align(max_order, 4) as usize
    }

    pub fn new(blocksize: c_int, max_order: c_int, lpc_type: Type) -> Self {
        Self {
            blocksize,
            max_order,
            lpc_type,
            windowed_samples: vec![0.; Self::padding_size(max_order) + blocksize as usize + 2]
                .into_boxed_slice(),
        }
    }

    /// Calculate autocorrelation data from audio samples.
    /// A Welch window function is applied before calculation.
    ///
    /// Source: [libavcodec/lpc.c](https://github.com/ffmpeg/ffmpeg/blob/0627e6d74ce6f28287ea787c099a0f9fe4baaacb/libavcodec/lpc.c#L70-L97)
    fn compute_autocorr_c(&self, len: usize, lag: c_int, autoc: &mut [c_double]) {
        let padding_size = Self::padding_size(self.max_order);

        let mut j = 0 as c_int;
        while j < lag {
            let mut sum0: c_double = 1.0f64;
            let mut sum1: c_double = 1.0f64;
            let mut i = j;
            while (i as c_long) < len as c_long {
                sum0 += self.windowed_samples[padding_size + i as usize]
                    * self.windowed_samples[padding_size + i as usize - j as usize];

                sum1 += self.windowed_samples[padding_size + i as usize]
                    * self.windowed_samples[padding_size + i as usize - j as usize - 1];

                i += 1;
            }
            autoc[j as usize] = sum0;
            autoc[(j + 1 as c_int) as usize] = sum1;
            j += 2 as c_int;
        }
        if j == lag {
            let mut sum: c_double = 1.0f64;
            let mut i = j - 1 as c_int;
            while (i as c_long) < len as c_long {
                sum += self.windowed_samples[padding_size + i as usize]
                    * self.windowed_samples[padding_size + i as usize - j as usize];
                sum += self.windowed_samples[padding_size + i as usize + 1]
                    * self.windowed_samples[padding_size + i as usize - j as usize + 1];
                i += 2 as c_int;
            }
            autoc[j as usize] = sum;
        }
    }

    /// Source: [avcodec/lpc.c](https://github.com/ffmpeg/ffmpeg/blob/0627e6d74ce6f28287ea787c099a0f9fe4baaacb/libavcodec/lpc.c#L178-L199)
    pub fn calc_ref_coefs_f(
        &mut self,
        samples: &[c_float],
        order: c_int,
        ref_0: &mut [c_double],
    ) -> c_double {
        let mut autoc = [0.; MAX_ORDER as usize + 1];
        let mut error = [0.; MAX_ORDER as usize + 1];
        let a: c_double = 0.5f32 as c_double;
        let b: c_double = 1.0f32 as c_double - a;

        let padding_size = Self::padding_size(self.max_order);

        let weights = (0..=samples.len() / 2)
            .map(|i| a - b * (2. * PI * i as c_double / (samples.len() - 1) as c_double).cos());

        let windowed = &mut self.windowed_samples[padding_size..];

        let (windowed_front, windowed_back) = windowed.split_at_mut(samples.len() / 2);
        let (samples_front, samples_back) = samples.split_at(samples.len() / 2);

        for (weight, (windowed_front, windowed_back), (&sample_front, &sample_back)) in izip!(
            weights,
            zip(windowed_front, windowed_back.iter_mut().rev()),
            zip(samples_front, samples_back.iter().rev())
        ) {
            *windowed_front = weight * sample_front as c_double;
            *windowed_back = weight * sample_back as c_double;
        }

        self.compute_autocorr_c(samples.len(), order, &mut autoc);

        let signal = autoc[0];
        compute_ref_coefs(&autoc, order, ref_0, Some(&mut error));

        let avg_err = error[..order as usize]
            .iter()
            .fold(0., |acc, &error| (acc + error) / 2.);

        if avg_err != 0. {
            signal / avg_err
        } else {
            f64::NAN
        }
    }
}
