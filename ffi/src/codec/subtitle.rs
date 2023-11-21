use libc::{c_char, c_int, c_long, c_uchar, c_uint, c_ushort};

pub type AVSubtitleType = c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVSubtitleRect {
    pub x: c_int,
    pub y: c_int,
    pub w: c_int,
    pub h: c_int,
    pub nb_colors: c_int,
    pub data: [*mut c_uchar; 4],
    pub linesize: [c_int; 4],
    pub type_0: AVSubtitleType,
    pub text: *mut c_char,
    pub ass: *mut c_char,
    pub flags: c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVSubtitle {
    pub format: c_ushort,
    pub start_display_time: c_uint,
    pub end_display_time: c_uint,
    pub num_rects: c_uint,
    pub rects: *mut *mut AVSubtitleRect,
    pub pts: c_long,
}
