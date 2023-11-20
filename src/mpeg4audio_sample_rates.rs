#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use libc::c_int;

pub(crate) static mut ff_mpeg4audio_sample_rates: [c_int; 16] = [
    96000 as c_int,
    88200 as c_int,
    64000 as c_int,
    48000 as c_int,
    44100 as c_int,
    32000 as c_int,
    24000 as c_int,
    22050 as c_int,
    16000 as c_int,
    12000 as c_int,
    11025 as c_int,
    8000 as c_int,
    7350 as c_int,
    0,
    0,
    0,
];
