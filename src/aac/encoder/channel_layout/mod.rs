pub(super) mod pce;

use std::ptr::null_mut;

use ffi::codec::channel::{self, AVChannel, AVChannelLayout, ChannelLayoutMaskOrMap, ORDER_NATIVE};
use libc::{c_uchar, c_ulong};

use crate::aac::SyntaxElementType::{
    self, ChannelPairElement as CPE, LowFrequencyEffects as LFE, SingleChannelElement as SCE,
};

const fn channel_mask(channels: &[AVChannel]) -> ChannelLayoutMaskOrMap {
    let mut mask: c_ulong = 0;
    let mut i = 0;
    while i < channels.len() {
        mask |= 1 << channels[i] as c_ulong;
        i += 1;
    }
    ChannelLayoutMaskOrMap { mask }
}

const fn channel_layout(channels: &[AVChannel]) -> AVChannelLayout {
    AVChannelLayout {
        order: ORDER_NATIVE,
        nb_channels: channels.len() as i32,
        u: channel_mask(channels),
        opaque: null_mut(),
    }
}

const fn config_map<const N: usize>(types: &[SyntaxElementType]) -> [c_uchar; N] {
    let mut map: [u8; N] = [0; N];
    let mut i = 0;
    map[0] = types.len() as c_uchar;
    while i < types.len() {
        map[i + 1] = types[i] as c_uchar;
        i += 1;
    }
    map
}

pub(super) const NORMAL_LAYOUTS: [AVChannelLayout; 7] = [
    channel_layout(&[channel::FRONT_CENTER]),
    channel_layout(&[channel::FRONT_LEFT, channel::FRONT_RIGHT]),
    channel_layout(&[
        channel::FRONT_LEFT,
        channel::FRONT_RIGHT,
        channel::FRONT_CENTER,
    ]),
    channel_layout(&[
        channel::FRONT_LEFT,
        channel::FRONT_RIGHT,
        channel::FRONT_CENTER,
        channel::BACK_CENTER,
    ]),
    channel_layout(&[
        channel::FRONT_LEFT,
        channel::FRONT_RIGHT,
        channel::FRONT_CENTER,
        channel::BACK_LEFT,
        channel::BACK_RIGHT,
    ]),
    channel_layout(&[
        channel::FRONT_LEFT,
        channel::FRONT_RIGHT,
        channel::FRONT_CENTER,
        channel::BACK_LEFT,
        channel::BACK_RIGHT,
        channel::LOW_FREQUENCY,
    ]),
    channel_layout(&[
        channel::FRONT_LEFT,
        channel::FRONT_RIGHT,
        channel::FRONT_CENTER,
        channel::SIDE_LEFT,
        channel::SIDE_RIGHT,
        channel::LOW_FREQUENCY,
        channel::BACK_LEFT,
        channel::BACK_RIGHT,
    ]),
];

pub(super) const CONFIGS: [[c_uchar; 6]; 16] = [
    config_map(&[SCE]),
    config_map(&[CPE]),
    config_map(&[SCE, CPE]),
    config_map(&[SCE, CPE, SCE]),
    config_map(&[SCE, CPE, CPE]),
    config_map(&[SCE, CPE, CPE, LFE]),
    config_map(&[]),
    config_map(&[SCE, CPE, CPE, CPE, LFE]),
    config_map(&[]),
    config_map(&[]),
    config_map(&[]),
    config_map(&[]),
    config_map(&[]),
    config_map(&[]),
    config_map(&[]),
    config_map(&[]),
];

pub(super) const REORDER_MAPS: [[c_uchar; 16]; 16] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [2, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [2, 0, 1, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [2, 0, 1, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [2, 0, 1, 4, 5, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [2, 0, 1, 6, 7, 4, 5, 3, 0, 0, 0, 0, 0, 0, 0, 0],
    [0; 16],
    [0; 16],
    [0; 16],
    [0; 16],
    [0; 16],
    [0; 16],
    [0; 16],
    [0; 16],
];
