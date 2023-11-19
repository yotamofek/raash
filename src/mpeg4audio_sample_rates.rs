#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

pub(crate) static mut ff_mpeg4audio_sample_rates: [libc::c_int; 16] = [
    96000 as libc::c_int,
    88200 as libc::c_int,
    64000 as libc::c_int,
    48000 as libc::c_int,
    44100 as libc::c_int,
    32000 as libc::c_int,
    24000 as libc::c_int,
    22050 as libc::c_int,
    16000 as libc::c_int,
    12000 as libc::c_int,
    11025 as libc::c_int,
    8000 as libc::c_int,
    7350 as libc::c_int,
    0,
    0,
    0,
];
