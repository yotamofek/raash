use libc::{c_char, c_int, c_uint, c_ulong, c_void};

pub type AVChannel = c_int;
pub type AVChannelOrder = c_uint;

pub const WIDE_RIGHT: AVChannel = 32;
pub const WIDE_LEFT: AVChannel = 31;
pub const TOP_BACK_RIGHT: AVChannel = 17;
pub const TOP_BACK_CENTER: AVChannel = 16;
pub const TOP_BACK_LEFT: AVChannel = 15;
pub const TOP_FRONT_RIGHT: AVChannel = 14;
pub const TOP_FRONT_CENTER: AVChannel = 13;
pub const TOP_FRONT_LEFT: AVChannel = 12;
pub const TOP_CENTER: AVChannel = 11;
pub const SIDE_RIGHT: AVChannel = 10;
pub const SIDE_LEFT: AVChannel = 9;
pub const BACK_CENTER: AVChannel = 8;
pub const FRONT_RIGHT_OF_CENTER: AVChannel = 7;
pub const FRONT_LEFT_OF_CENTER: AVChannel = 6;
pub const BACK_RIGHT: AVChannel = 5;
pub const BACK_LEFT: AVChannel = 4;
pub const LOW_FREQUENCY: AVChannel = 3;
pub const FRONT_CENTER: AVChannel = 2;
pub const FRONT_RIGHT: AVChannel = 1;
pub const FRONT_LEFT: AVChannel = 0;

pub const ORDER_NATIVE: AVChannelOrder = 1;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVChannelCustom {
    pub id: AVChannel,
    pub name: [c_char; 16],
    pub opaque: *mut c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVChannelLayout {
    pub order: AVChannelOrder,
    pub nb_channels: c_int,
    pub u: ChannelLayoutMaskOrMap,
    pub opaque: *mut c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union ChannelLayoutMaskOrMap {
    pub mask: c_ulong,
    pub map: *mut AVChannelCustom,
}
