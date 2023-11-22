//! [Linear predictive coding](https://en.wikipedia.org/wiki/Linear_predictive_coding)

#![allow(clippy::self_assignment)]

use std::f64::consts::PI;

use itertools::izip;
use libc::{c_double, c_float, c_int, c_long};

use crate::common::*;

pub(crate) type FFLPCType = c_int;
pub(crate) const FF_LPC_TYPE_LEVINSON: FFLPCType = 2;

#[derive(Clone)]
#[repr(C)]
pub(crate) struct LPCContext {
    blocksize: c_int,
    max_order: c_int,
    lpc_type: FFLPCType,
    // windowed_buffer: Box<[c_double]>,
    windowed_samples: Box<[c_double]>,
}

#[inline]
unsafe fn compute_ref_coefs(
    autoc: *const c_double,
    max_order: c_int,
    ref_0: *mut c_double,
    error: *mut c_double,
) {
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut err: c_double = 0.;
    let mut gen0: [c_double; 32] = [0.; 32];
    let mut gen1: [c_double; 32] = [0.; 32];
    i = 0 as c_int;
    while i < max_order {
        gen1[i as usize] = *autoc.offset((i + 1 as c_int) as isize);
        gen0[i as usize] = gen1[i as usize];
        i += 1;
        i;
    }
    err = *autoc.offset(0 as c_int as isize);
    *ref_0.offset(0 as c_int as isize) = -gen1[0 as c_int as usize]
        / (if 0 as c_int != 0 || err != 0. {
            err
        } else {
            1.
        });
    err += gen1[0 as c_int as usize] * *ref_0.offset(0 as c_int as isize);
    if !error.is_null() {
        *error.offset(0 as c_int as isize) = err;
    }
    i = 1 as c_int;
    while i < max_order {
        j = 0 as c_int;
        while j < max_order - i {
            gen1[j as usize] = gen1[(j + 1 as c_int) as usize]
                + *ref_0.offset((i - 1 as c_int) as isize) * gen0[j as usize];
            gen0[j as usize] +=
                gen1[(j + 1 as c_int) as usize] * *ref_0.offset((i - 1 as c_int) as isize);
            j += 1;
            j;
        }
        *ref_0.offset(i as isize) = -gen1[0 as c_int as usize]
            / (if 0 as c_int != 0 || err != 0. {
                err
            } else {
                1.
            });
        err += gen1[0 as c_int as usize] * *ref_0.offset(i as isize);
        if !error.is_null() {
            *error.offset(i as isize) = err;
        }
        i += 1;
        i;
    }
}

impl LPCContext {
    const fn padding_size(max_order: c_int) -> usize {
        // see https://ffmpeg.org/doxygen/trunk/macros_8h.html#acae4ba605b3a7d535b7ac058ffe96892
        const fn align(x: c_int, a: c_int) -> c_int {
            (x + a - 1) & !(a - 1)
        }

        align(max_order, 4) as usize
    }

    pub(crate) fn new(blocksize: c_int, max_order: c_int, lpc_type: FFLPCType) -> Self {
        Self {
            blocksize,
            max_order,
            lpc_type,
            // original ffmpeg code uses two pointers to the same allocation, `windowed_samples`
            // points to `windowed_buffer + padding_size` and so can be indexed with negative
            // numbers. we just add `padding_size` to the used indices instead.
            windowed_samples: vec![0.; Self::padding_size(max_order) + blocksize as usize + 2]
                .into_boxed_slice(),
        }
    }

    unsafe fn compute_autocorr_c(&self, len: usize, lag: c_int, autoc: &mut [c_double]) {
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
                i;
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

    pub(crate) unsafe fn calc_ref_coefs_f(
        &mut self,
        samples: &[c_float],
        order: c_int,
        ref_0: *mut c_double,
    ) -> c_double {
        let mut autoc: [c_double; 33] = [0.; _];
        let mut error: [c_double; 33] = [0.; _];
        let a: c_double = 0.5f32 as c_double;
        let b: c_double = 1.0f32 as c_double - a;

        let padding_size = Self::padding_size(self.max_order);

        // let weights = (0..=samples.len() / 2)
        //     .map(|i| a - b * (2. * PI * i as c_double / (samples.len() - 1) as
        // c_double).cos()); let (windowed_front, windowed_back) = self
        //     .windowed_samples
        //     .split_at_mut(padding_size + samples.len() / 2);
        // let (samples_front, samples_back) = samples.split_at(samples.len() / 2);
        // for (weight, front, back) in izip!(weights, windowed_front,
        // windowed_back.iter_mut().rev()) {
        // }

        for i in 0..=samples.len() / 2 {
            let weight: c_double =
                a - b * cos(2. * PI * i as c_double / (samples.len() as c_int - 1) as c_double);
            (self.windowed_samples)[padding_size + i] = weight * samples[i] as c_double;
            (self.windowed_samples)[padding_size + samples.len() - 1 - i] =
                weight * samples[samples.len() - 1 - i] as c_double;
        }

        self.compute_autocorr_c(samples.len(), order, &mut autoc);

        let signal = autoc[0];
        compute_ref_coefs(autoc.as_mut_ptr(), order, ref_0, error.as_mut_ptr());

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
