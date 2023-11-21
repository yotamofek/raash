use libc::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVRational {
    pub num: c_int,
    pub den: c_int,
}
