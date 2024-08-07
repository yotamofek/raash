//! [Linear predictive coding](https://en.wikipedia.org/wiki/Linear_predictive_coding)

use std::f64::consts::PI;

use ffmpeg_src_macro::ffmpeg_src;
use libc::{c_double, c_float, c_int, c_long};

/// LPC analysis type
#[ffmpeg_src(file = "libavcodec/lpc.h", lines = 41..=51, name = "FFLPCType")]
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

pub struct RefCoeffs {
    pub gain: c_double,
    pub coeffs: [c_double; 32],
}

/// Schur recursion.
/// Produces reflection coefficients from autocorrelation data.
#[ffmpeg_src(file = "libavcodec/lpc.h", lines = 136..=161)]
#[inline]
fn compute_ref_coefs(
    autoc: &[c_double],
    max_order: c_int,
    mut error: Option<&mut [c_double]>,
) -> [c_double; 32] {
    let mut ref_0 = [0.; 32];
    let mut gen0 = [0.; MAX_ORDER as usize];
    let mut gen1 = [0.; MAX_ORDER as usize];
    let mut i = 0;
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
    let mut i = 1;
    while i < max_order {
        let mut j = 0;
        while j < max_order - i {
            gen1[j as usize] = gen1[(j + 1) as usize] + ref_0[(i - 1) as usize] * gen0[j as usize];
            gen0[j as usize] += gen1[(j + 1) as usize] * ref_0[(i - 1) as usize];
            j += 1;
        }
        ref_0[i as usize] = -gen1[0] / if err != 0. { err } else { 1. };
        err += gen1[0] * ref_0[i as usize];
        if let Some(error) = &mut error {
            error[i as usize] = err;
        }
        i += 1;
    }

    ref_0
}

impl LPCContext {
    const fn padding_size(max_order: c_int) -> usize {
        #[ffmpeg_src(file = "libavutil/macros.h", lines = 78, name = "FFALIGN")]
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
    #[ffmpeg_src(file = "libavcodec/lpc.c", lines = 70..=97)]
    fn compute_autocorr_c(&self, len: usize, lag: c_int, autoc: &mut [c_double]) {
        let padding_size = Self::padding_size(self.max_order);

        let mut j = 0;
        while j < lag {
            let mut sum0: c_double = 1.;
            let mut sum1: c_double = 1.;
            let mut i = j;
            while (i as c_long) < len as c_long {
                sum0 += self.windowed_samples[padding_size + i as usize]
                    * self.windowed_samples[padding_size + i as usize - j as usize];

                sum1 += self.windowed_samples[padding_size + i as usize]
                    * self.windowed_samples[padding_size + i as usize - j as usize - 1];

                i += 1;
            }
            autoc[j as usize] = sum0;
            autoc[(j + 1) as usize] = sum1;
            j += 2;
        }
        if j == lag {
            let mut sum: c_double = 1.;
            let mut i = j - 1;
            while (i as c_long) < len as c_long {
                sum += self.windowed_samples[padding_size + i as usize]
                    * self.windowed_samples[padding_size + i as usize - j as usize];
                sum += self.windowed_samples[padding_size + i as usize + 1]
                    * self.windowed_samples[padding_size + i as usize - j as usize + 1];
                i += 2;
            }
            autoc[j as usize] = sum;
        }
    }

    #[ffmpeg_src(file = "libavcodec/lpc.c", lines = 178..=199, name = "ff_lpc_calc_ref_coefs_f")]
    pub fn calc_ref_coefs_f(&mut self, samples: &[c_float], order: c_int) -> RefCoeffs {
        let mut autoc = [0.; MAX_ORDER as usize + 1];
        let mut error = [0.; MAX_ORDER as usize + 1];
        let a: c_double = 0.5;
        let b: c_double = 1. - a;

        let padding_size = Self::padding_size(self.max_order);

        for i in 0..=samples.len() / 2 {
            let weight: c_double =
                a - b * (2. * PI * i as c_double / (samples.len() as c_int - 1) as c_double).cos();
            self.windowed_samples[padding_size + i] = weight * samples[i] as c_double;
            self.windowed_samples[padding_size + samples.len() - 1 - i] =
                weight * samples[samples.len() - 1 - i] as c_double;
        }

        self.compute_autocorr_c(samples.len(), order, &mut autoc);

        let [signal, ..] = autoc;
        let coeffs = compute_ref_coefs(&autoc, order, Some(&mut error));

        let avg_err = error[..order as usize]
            .iter()
            .fold(0., |acc, &error| (acc + error) / 2.);

        let gain = if avg_err != 0. {
            signal / avg_err
        } else {
            f64::NAN
        };

        RefCoeffs { gain, coeffs }
    }
}
