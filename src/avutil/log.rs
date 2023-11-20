use libc::{c_char, c_void};

use crate::types::AVClass;

pub(crate) unsafe extern "C" fn av_default_item_name(mut ptr: *mut c_void) -> *const c_char {
    return (**(ptr as *mut *mut AVClass)).class_name;
}
