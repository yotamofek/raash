use libc::{c_double, c_float, c_int};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVRational {
    pub num: c_int,
    pub den: c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVComplexDouble {
    pub re: c_double,
    pub im: c_double,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVComplexFloat {
    pub re: c_float,
    pub im: c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVComplexInt32 {
    pub re: c_int,
    pub im: c_int,
}
