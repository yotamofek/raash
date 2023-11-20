use crate::types::AVClass;

pub(crate) unsafe extern "C" fn av_default_item_name(
    mut ptr: *mut libc::c_void,
) -> *const libc::c_char {
    return (**(ptr as *mut *mut AVClass)).class_name;
}
