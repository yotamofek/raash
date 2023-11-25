use libc::c_int;

pub type ProfileId = c_int;

pub const UNKNOWN: ProfileId = -99;
pub const RESERVED: ProfileId = -100;

pub const AAC_MAIN: ProfileId = 0;
pub const AAC_LOW: ProfileId = 1;
pub const AAC_SSR: ProfileId = 2;
pub const AAC_LTP: ProfileId = 3;
pub const AAC_HE: ProfileId = 4;
pub const AAC_HE_V2: ProfileId = 28;
pub const AAC_LD: ProfileId = 22;
pub const AAC_ELD: ProfileId = 38;
pub const MPEG2_AAC_LOW: ProfileId = 128;
pub const MPEG2_AAC_HE: ProfileId = 131;
