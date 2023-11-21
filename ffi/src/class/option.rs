use libc::{c_char, c_double, c_int, c_long, c_uint};

use crate::num::AVRational;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVOption {
    pub name: *const c_char,
    pub help: *const c_char,
    pub offset: c_int,
    pub type_0: AVOptionType,
    pub default_val: DefaultValue,
    pub min: c_double,
    pub max: c_double,
    pub flags: c_int,
    pub unit: *const c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union DefaultValue {
    pub i64_0: c_long,
    pub dbl: c_double,
    pub str_0: *const c_char,
    pub q: AVRational,
}

pub type AVOptionType = c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVOptionRange {
    pub str_0: *const c_char,
    pub value_min: c_double,
    pub value_max: c_double,
    pub component_min: c_double,
    pub component_max: c_double,
    pub is_range: c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVOptionRanges {
    pub range: *mut *mut AVOptionRange,
    pub nb_ranges: c_int,
    pub nb_components: c_int,
}
