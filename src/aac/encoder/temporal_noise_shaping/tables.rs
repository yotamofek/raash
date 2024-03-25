#![allow(clippy::excessive_precision)]

use libc::{c_float, c_uchar};

const tns_tmp2_map_1_3: [c_float; 4] = [0.00000000, -0.43388373, 0.64278758, 0.34202015];
const tns_tmp2_map_0_3: [c_float; 8] = [
    0.00000000,
    -0.43388373,
    -0.78183150,
    -0.97492790,
    0.98480773,
    0.86602539,
    0.64278758,
    0.34202015,
];
const tns_tmp2_map_1_4: [c_float; 8] = [
    0.00000000,
    -0.20791170,
    -0.40673664,
    -0.58778524,
    0.67369562,
    0.52643216,
    0.36124167,
    0.18374951,
];
const tns_tmp2_map_0_4: [c_float; 16] = [
    0.00000000,
    -0.20791170,
    -0.40673664,
    -0.58778524,
    -0.74314481,
    -0.86602539,
    -0.95105654,
    -0.99452192,
    0.99573416,
    0.96182561,
    0.89516330,
    0.79801720,
    0.67369562,
    0.52643216,
    0.36124167,
    0.18374951,
];

pub(super) const tns_tmp2_map: [&[c_float]; 4] = [
    &tns_tmp2_map_0_3,
    &tns_tmp2_map_0_4,
    &tns_tmp2_map_1_3,
    &tns_tmp2_map_1_4,
];

const tns_min_sfb_short: [c_uchar; 16] = [2, 2, 2, 3, 3, 4, 6, 6, 8, 10, 10, 12, 12, 12, 12, 12];
const tns_min_sfb_long: [c_uchar; 16] = [
    12, 13, 15, 16, 17, 20, 25, 26, 24, 28, 30, 31, 31, 31, 31, 31,
];

pub(super) const tns_min_sfb: [[c_uchar; 16]; 2] = [tns_min_sfb_long, tns_min_sfb_short];
