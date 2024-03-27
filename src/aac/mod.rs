use std::iter;

use ffmpeg_src_macro::ffmpeg_src;
use libc::{c_float, c_int, c_uchar, c_uint, c_ushort};

use self::encoder::LongTermPrediction;
use crate::array::Array;

pub mod coder;
pub mod encoder;
pub mod psy_model;
pub mod tables;

/// Tag for AAC syntax elements which comprise an element.
///
/// <https://wiki.multimedia.cx/index.php/Understanding_AAC#Frames_And_Syntax_Elements>
#[ffmpeg_src(file = "libavcodec/aac.h", lines = 54..=63, name = "RawDataBlockType")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum SyntaxElementType {
    End = 7,
    FillElement = 6,
    LowFrequencyEffects = 3,
    ChannelPairElement = 1,
    SingleChannelElement = 0,
}

#[derive(Debug)]
pub struct UnknownSyntaxElementType();

impl TryFrom<u32> for SyntaxElementType {
    type Error = UnknownSyntaxElementType;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            7 => Self::End,
            6 => Self::FillElement,
            3 => Self::LowFrequencyEffects,
            1 => Self::ChannelPairElement,
            0 => Self::SingleChannelElement,
            _ => return Err(UnknownSyntaxElementType()),
        })
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

type WindowSequence = c_uint;
const LONG_STOP_SEQUENCE: WindowSequence = 3;
const EIGHT_SHORT_SEQUENCE: WindowSequence = 2;
const LONG_START_SEQUENCE: WindowSequence = 1;
const ONLY_LONG_SEQUENCE: WindowSequence = 0;

#[ffmpeg_src(file = "libavcodec/aac.h", lines = 169..=191)]
#[derive(Default, Copy, Clone)]
pub(crate) struct IndividualChannelStream {
    /// number of scalefactor bands per group
    max_sfb: c_uchar,
    window_sequence: [WindowSequence; 2],
    /// If set, use Kaiser-Bessel window, otherwise use a sine window.
    use_kb_window: [c_uchar; 2],
    group_len: [c_uchar; 8],
    ltp: LongTermPrediction,
    /// table of offsets to the lowest spectral coefficient of a scalefactor
    /// band, sfb, for a particular window
    swb_offset: &'static [c_ushort],
    /// table of scalefactor band sizes for a particular window
    swb_sizes: &'static [c_uchar],
    /// number of scalefactor window bands
    num_swb: c_int,
    num_windows: c_int,
    tns_max_bands: c_int,
    predictor_present: c_int,
    prediction_used: Array<c_uchar, 41>,
    /// set if a certain window is near clipping
    window_clipping: [c_uchar; 8],
    /// set if any window is near clipping to the necessary atennuation factor
    /// to avoid it
    clip_avoidance_factor: c_float,
}

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
}
