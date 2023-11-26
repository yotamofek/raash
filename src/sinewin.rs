use std::{array, f64::consts::PI, ops::Deref};

use libc::{c_double, c_float};
use once_cell::sync::Lazy;

pub struct SineWindow<const N: usize>([c_float; N]);

impl<const N: usize> Deref for SineWindow<N> {
    type Target = [c_float; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> SineWindow<N> {
    /// Source: [libavcodec/sinewin_tablegen.h](https://github.com/FFmpeg/FFmpeg/blob/2020ef9770d6bdf4ed2d8a32595d0e70afd2db8f/libavcodec/sinewin_tablegen.h#L59C1-L64)
    fn init() -> Self {
        Self(array::from_fn(|i| {
            (((i as c_double + 0.5) * (PI / (2.0 * N as c_double))) as c_float).sin()
        }))
    }
}

const fn lazy_sine_window<const N: usize>() -> Lazy<SineWindow<N>> {
    Lazy::new(SineWindow::init)
}

pub(crate) static mut SINE_WIN_128: Lazy<SineWindow<128>> = lazy_sine_window();
pub(crate) static mut SINE_WIN_1024: Lazy<SineWindow<1024>> = lazy_sine_window();
