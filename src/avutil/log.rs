use libc::{c_char, c_void};

use crate::types::AVClass;

pub(crate) unsafe extern "C" fn av_default_item_name(ptr: *mut c_void) -> *const c_char {
    (**(ptr as *mut *mut AVClass)).class_name
}
