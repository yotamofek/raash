pub mod option;

use libc::{c_char, c_int, c_uint, c_void};

use self::option::{AVOption, AVOptionRanges};

pub type AVClassCategory = c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVClass {
    pub class_name: *const c_char,
    pub item_name: Option<unsafe extern "C" fn(*mut c_void) -> *const c_char>,
    pub option: *const AVOption,
    pub version: c_int,
    pub log_level_offset_offset: c_int,
    pub parent_log_context_offset: c_int,
    pub category: AVClassCategory,
    pub get_category: Option<unsafe extern "C" fn(*mut c_void) -> AVClassCategory>,
    pub query_ranges: Option<
        unsafe extern "C" fn(*mut *mut AVOptionRanges, *mut c_void, *const c_char, c_int) -> c_int,
    >,
    pub child_next: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> *mut c_void>,
    pub child_class_iterate: Option<unsafe extern "C" fn(*mut *mut c_void) -> *const AVClass>,
}
