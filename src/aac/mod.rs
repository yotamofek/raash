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
