use std::sync::Once;

use crate::common::*;

pub(crate) static mut ff_sine_4096: [libc::c_float; 4096] = [0.; 4096];
static mut init_sine_window_once: [Once; 9] = [
    Once::new(),
    Once::new(),
    Once::new(),
    Once::new(),
    Once::new(),
    Once::new(),
    Once::new(),
    Once::new(),
    Once::new(),
];
unsafe fn init_ff_sine_window_5() {
    ff_sine_window_init(
        ff_sine_windows[5 as libc::c_int as usize],
        (1 as libc::c_int) << 5 as libc::c_int,
    );
}
unsafe fn init_ff_sine_window_6() {
    ff_sine_window_init(
        ff_sine_windows[6 as libc::c_int as usize],
        (1 as libc::c_int) << 6 as libc::c_int,
    );
}
unsafe fn init_ff_sine_window_7() {
    ff_sine_window_init(
        ff_sine_windows[7 as libc::c_int as usize],
        (1 as libc::c_int) << 7 as libc::c_int,
    );
}
unsafe fn init_ff_sine_window_8() {
    ff_sine_window_init(
        ff_sine_windows[8 as libc::c_int as usize],
        (1 as libc::c_int) << 8 as libc::c_int,
    );
}
unsafe fn init_ff_sine_window_9() {
    ff_sine_window_init(
        ff_sine_windows[9 as libc::c_int as usize],
        (1 as libc::c_int) << 9 as libc::c_int,
    );
}
unsafe fn init_ff_sine_window_10() {
    ff_sine_window_init(
        ff_sine_windows[10 as libc::c_int as usize],
        (1 as libc::c_int) << 10 as libc::c_int,
    );
}
unsafe fn init_ff_sine_window_11() {
    ff_sine_window_init(
        ff_sine_windows[11 as libc::c_int as usize],
        (1 as libc::c_int) << 11 as libc::c_int,
    );
}
unsafe fn init_ff_sine_window_12() {
    ff_sine_window_init(
        ff_sine_windows[12 as libc::c_int as usize],
        (1 as libc::c_int) << 12 as libc::c_int,
    );
}

pub(crate) static mut ff_sine_32: [libc::c_float; 32] = [0.; 32];

pub(crate) static mut ff_sine_64: [libc::c_float; 64] = [0.; 64];

pub(crate) static mut ff_sine_128: [libc::c_float; 128] = [0.; 128];

pub(crate) static mut ff_sine_256: [libc::c_float; 256] = [0.; 256];

pub(crate) static mut ff_sine_512: [libc::c_float; 512] = [0.; 512];

pub(crate) static mut ff_sine_1024: [libc::c_float; 1024] = [0.; 1024];

pub(crate) static mut ff_sine_2048: [libc::c_float; 2048] = [0.; 2048];

pub(crate) static mut ff_sine_windows: [*mut libc::c_float; 14] = unsafe {
    [
        0 as *const libc::c_float as *mut libc::c_float,
        0 as *const libc::c_float as *mut libc::c_float,
        0 as *const libc::c_float as *mut libc::c_float,
        0 as *const libc::c_float as *mut libc::c_float,
        0 as *const libc::c_float as *mut libc::c_float,
        ff_sine_32.as_ptr() as *mut _,
        ff_sine_64.as_ptr() as *mut _,
        ff_sine_128.as_ptr() as *mut _,
        ff_sine_256.as_ptr() as *mut _,
        ff_sine_512.as_ptr() as *mut _,
        ff_sine_1024.as_ptr() as *mut _,
        ff_sine_2048.as_ptr() as *mut _,
        ff_sine_4096.as_ptr() as *mut _,
        ff_sine_8192.as_ptr() as *mut _,
    ]
};

pub(crate) static mut ff_sine_8192: [libc::c_float; 8192] = [0.; 8192];

#[cold]
pub(crate) unsafe fn ff_sine_window_init(window: *mut libc::c_float, n: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        *window.offset(i as isize) = sinf(
            ((i as libc::c_double + 0.5f64)
                * (3.141_592_653_589_793_f64 / (2.0f64 * n as libc::c_double)))
                as libc::c_float,
        );
        i += 1;
        i;
    }
}

#[cold]
pub(crate) unsafe fn ff_init_ff_sine_windows(index: libc::c_int) {
    init_sine_window_once[(index - 5 as libc::c_int) as usize]
        .call_once(|| sine_window_init_func_array[(index - 5 as libc::c_int) as usize].unwrap()());
}
unsafe fn init_ff_sine_window_13() {
    ff_sine_window_init(
        ff_sine_windows[13 as libc::c_int as usize],
        (1 as libc::c_int) << 13 as libc::c_int,
    );
}
static mut sine_window_init_func_array: [Option<unsafe fn() -> ()>; 9] = unsafe {
    [
        Some(init_ff_sine_window_5 as unsafe fn() -> ()),
        Some(init_ff_sine_window_6 as unsafe fn() -> ()),
        Some(init_ff_sine_window_7 as unsafe fn() -> ()),
        Some(init_ff_sine_window_8 as unsafe fn() -> ()),
        Some(init_ff_sine_window_9 as unsafe fn() -> ()),
        Some(init_ff_sine_window_10 as unsafe fn() -> ()),
        Some(init_ff_sine_window_11 as unsafe fn() -> ()),
        Some(init_ff_sine_window_12 as unsafe fn() -> ()),
        Some(init_ff_sine_window_13 as unsafe fn() -> ()),
    ]
};
