use std::iter;

use ffmpeg_src_macro::ffmpeg_src;
use libc::{c_float, c_int, c_uchar, c_ushort};

pub mod coder;
pub mod encoder;
pub mod psy_model;
pub mod tables;

/// Tag for AAC syntax elements which comprise an element.
///
/// <https://wiki.multimedia.cx/index.php/Understanding_AAC#Frames_And_Syntax_Elements>
#[ffmpeg_src(file = "libavcodec/aac.h", lines = 54..=63, name = "RawDataBlockType")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum SyntaxElementType {
    End = 7,
    FillElement = 6,
    LowFrequencyEffects = 3,
    ChannelPairElement = 1,
    SingleChannelElement = 0,
}

impl SyntaxElementType {
    const fn channels(self) -> usize {
        match self {
            Self::ChannelPairElement => 2,
            _ => 1,
        }
    }
}

/// scalefactor difference that corresponds to scale difference in 512 times
#[ffmpeg_src(file = "libavcodec/aac.h", lines = 146)]
const SCALE_DIV_512: c_uchar = 36;

/// scalefactor index that corresponds to scale=1.0
#[ffmpeg_src(file = "libavcodec/aac.h", lines = 147)]
const SCALE_ONE_POS: c_uchar = 140;

/// scalefactor index maximum value
#[ffmpeg_src(file = "libavcodec/aac.h", lines = 148)]
const SCALE_MAX_POS: c_uchar = 255;

/// maximum scalefactor difference allowed by standard
#[ffmpeg_src(file = "libavcodec/aac.h", lines = 149)]
const SCALE_MAX_DIFF: c_uchar = 60;

/// codebook index corresponding to zero scalefactor indices difference
#[ffmpeg_src(file = "libavcodec/aac.h", lines = 150)]
const SCALE_DIFF_ZERO: c_uchar = 60;

/// [`pow2`](tables::PowSfTables::pow2) index corresponding to `pow(2, 0);`
#[ffmpeg_src(file = "libavcodec/aac.h", lines = 152)]
const POW_SF2_ZERO: c_uchar = 200;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum WindowSequence {
    LongStop = 3,
    EightShort = 2,
    LongStart = 1,
    #[default]
    OnlyLong = 0,
}

#[ffmpeg_src(file = "libavcodec/aac.h", lines = 169..=191)]
#[derive(Default, Copy, Clone)]
pub(crate) struct IndividualChannelStream {
    /// number of scalefactor bands per group
    max_sfb: c_uchar,
    window_sequence: [WindowSequence; 2],
    /// If set, use Kaiser-Bessel window, otherwise use a sine window.
    use_kb_window: [bool; 2],
    group_len: [c_uchar; 8],
    /// table of offsets to the lowest spectral coefficient of a scalefactor
    /// band, sfb, for a particular window
    swb_offset: &'static [c_ushort],
    /// table of scalefactor band sizes for a particular window
    swb_sizes: &'static [c_uchar],
    /// number of scalefactor window bands
    num_swb: c_int,
    num_windows: c_int,
    tns_max_bands: c_int,
    predictor_present: bool,
    /// set if a certain window is near clipping
    window_clipping: [bool; 8],
    /// set if any window is near clipping to the necessary atennuation factor
    /// to avoid it
    clip_avoidance_factor: c_float,
}

#[derive(Clone, Copy)]
struct WindowedIteration {
    w: c_int,
    group_len: c_uchar,
}

impl IndividualChannelStream {
    fn iter_windows(&self) -> impl Iterator<Item = WindowedIteration> {
        let Self {
            group_len,
            num_windows,
            ..
        } = *self;

        debug_assert!(num_windows <= 8, "num_windows: {num_windows}");

        let mut w = 0;
        iter::from_fn(move || {
            if w >= num_windows {
                return None;
            }

            let group_len = group_len[w as usize];
            let iter = WindowedIteration { w, group_len };
            w += c_int::from(group_len);
            Some(iter)
        })
    }

    fn iter_swb_sizes_sum(&self) -> impl Iterator<Item = (c_uchar, c_ushort)> {
        self.swb_sizes[..self.num_swb.try_into().unwrap()]
            .iter()
            .scan(0, |sum, &swb_size| {
                let next = *sum;
                *sum += c_ushort::from(swb_size);
                Some((swb_size, next))
            })
    }
}
