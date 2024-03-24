use ffmpeg_src_macro::ffmpeg_src;
use libc::c_uchar;

pub mod coder;
pub mod encoder;
pub mod psy_model;
pub mod tables;

/// Tag for AAC syntax elements which comprise an element.
///
/// <https://wiki.multimedia.cx/index.php/Understanding_AAC#Frames_And_Syntax_Elements>
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
