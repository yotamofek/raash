use std::mem::size_of;

use libc::{c_double, c_float, c_int};
use thiserror::Error;

/// Audio sample formats
///
/// - The data described by the sample format is always in native-endian order.
///   Sample values can be expressed by native C types, hence the lack of a
///   signed 24-bit sample format even though it is a common raw audio data
///   format.
///
/// - The floating-point formats are based on full volume being in the range
///   `[-1.0, 1.0]`. Any values outside this range are beyond full volume level.
///
/// - The data layout as used in av_samples_fill_arrays() and elsewhere in
///   FFmpeg (such as AVFrame in libavcodec) is as follows:
///
/// For planar sample formats, each audio channel is in a separate data
/// plane, and linesize is the buffer size, in bytes, for a single plane. All
/// data planes must be the same size. For packed sample formats, only the
/// first data plane is used, and samples for each channel are interleaved.
/// In this case, linesize is the buffer size, in bytes, for the 1 plane.
#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub(super) enum SampleFormat {
    /// unsigned 8 bits
    U8 = 0,
    /// signed 16 bits
    S16,
    /// signed 32 bits
    S32,
    /// float
    Float,
    /// double
    Double,
    /// unsigned 8 bits, planar
    U8P,
    /// signed 16 bits, planar
    S16P,
    /// signed 32 bits, planar
    S32P,
    /// float, planar
    FloatP,
    /// double, planar
    DoubleP,
    /// signed 64 bits
    S64,
    /// signed 64 bits, planar
    S64P,
}

impl SampleFormat {
    /// Size (in bytes) of samples in this format
    pub(super) fn size(self) -> usize {
        match self {
            Self::U8 | Self::U8P => size_of::<u8>(),
            Self::S16 | Self::S16P => size_of::<i16>(),
            Self::S32 | Self::S32P => size_of::<i32>(),
            Self::Float | Self::FloatP => size_of::<c_float>(),
            Self::Double | Self::DoubleP => size_of::<c_double>(),
            Self::S64 | Self::S64P => size_of::<i64>(),
        }
    }
}

#[derive(Debug, Clone, Copy, Error)]
#[error("Unknown AVSampleFormat: {0}")]
pub(super) struct Unknown(pub c_int);

impl TryFrom<c_int> for SampleFormat {
    type Error = Unknown;

    fn try_from(value: c_int) -> Result<Self, Self::Error> {
        macro_rules! var {
            ($($var:ident),+) => {
                Ok(match value {
                    $(value if value == Self::$var as c_int => Self::$var,)+
                    _ => return Err(Unknown(value)),
                })
            };
        }

        var!(U8, S16, S32, Float, Double, U8P, S16P, S32P, FloatP, DoubleP, S64, S64P)
    }
}
