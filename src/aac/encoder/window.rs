use std::{iter::zip, mem::size_of};

use izip::izip;
use libc::{c_float, c_uint};

use super::ctx::MdctContext;
use crate::{
    aac::{
        tables::{KBD_LONG, KBD_SHORT},
        EIGHT_SHORT_SEQUENCE, LONG_START_SEQUENCE, LONG_STOP_SEQUENCE, ONLY_LONG_SEQUENCE,
    },
    sinewin::{SINE_WIN_1024, SINE_WIN_128},
    types::{ptrdiff_t, SingleChannelElement},
};

fn apply_only_long_window(sce: &mut SingleChannelElement, audio: &[c_float; 3 * 1024]) {
    let out = &mut *sce.ret_buf;

    let [lwindow, pwindow] = sce.ics.use_kb_window.map(|use_kb| {
        if use_kb != 0 {
            &*KBD_LONG
        } else {
            &SINE_WIN_1024
        }
    });

    for (out, audio, lwindow) in izip!(&mut out[..1024], &audio[..1024], &lwindow[..1024]) {
        *out = *audio * *lwindow;
    }
    for (out, audio, pwindow) in izip!(
        &mut out[1024..][..1024],
        &audio[1024..][..1024],
        pwindow[..1024].iter().rev()
    ) {
        *out = *audio * *pwindow;
    }
}

fn apply_long_start_window(sce: &mut SingleChannelElement, audio: &[c_float; 3 * 1024]) {
    let out = &mut *sce.ret_buf;

    let mut lwindow = if sce.ics.use_kb_window[1] != 0 {
        &*KBD_LONG
    } else {
        &SINE_WIN_1024
    };
    let mut swindow = if sce.ics.use_kb_window[0] != 0 {
        &*KBD_SHORT
    } else {
        &SINE_WIN_128
    };

    for (out, audio, lwindow) in izip!(&mut out[..1024], &audio[..1024], &lwindow[..1024]) {
        *out = *audio * *lwindow;
    }

    out[1024..][..448].copy_from_slice(&audio[1024..][..448]);

    for (out, audio, swindow) in izip!(
        &mut out[1024..][448..][..128],
        &audio[1024..][448..],
        swindow[..128].iter().rev()
    ) {
        *out = *audio * *swindow;
    }

    out[1024..][576..][..448].fill(0.);
}

fn apply_long_stop_window(sce: &mut SingleChannelElement, audio: &[c_float; 3 * 1024]) {
    let out = &mut *sce.ret_buf;

    let mut lwindow = if sce.ics.use_kb_window[0] != 0 {
        &*KBD_LONG
    } else {
        &SINE_WIN_1024
    };
    let mut swindow = if sce.ics.use_kb_window[1] != 0 {
        &*KBD_SHORT
    } else {
        &SINE_WIN_128
    };

    out[..448].fill(0.);

    for (out, audio, swindow) in izip!(
        &mut out[448..][..128],
        &audio[448..][..128],
        &swindow[..128]
    ) {
        *out = *audio * *swindow;
    }

    out[576..][..448].copy_from_slice(&audio[576..][..448]);

    for (out, audio, lwindow) in izip!(
        &mut out[1024..][..1024],
        &audio[1024..][..1024],
        lwindow[..1024].iter().rev()
    ) {
        *out = *audio * *lwindow;
    }
}

fn apply_eight_short_window(sce: &mut SingleChannelElement, audio: &[c_float; 3 * 1024]) {
    let out = &mut *sce.ret_buf;

    let mut swindow = if sce.ics.use_kb_window[0] != 0 {
        &*KBD_SHORT
    } else {
        &SINE_WIN_128
    };
    let mut pwindow = if sce.ics.use_kb_window[1] != 0 {
        &*KBD_SHORT
    } else {
        &SINE_WIN_128
    };

    let mut in_ = &audio[448..];
    let mut out = &mut out[..];
    for w in 0..8 {
        for (out, in_, window) in izip!(
            &mut out[..128],
            &in_[..128],
            &(if w != 0 { pwindow } else { swindow })[..128]
        ) {
            *out = *in_ * *window;
        }
        out = &mut out[128..];
        in_ = &in_[128..];
        for (out, in_, swindow) in izip!(&mut out[..128], &in_[..128], swindow[..128].iter().rev())
        {
            *out = *in_ * *swindow;
        }
        out = &mut out[128..];
    }
}

pub(super) fn apply_window_and_mdct(
    mut mdct: &MdctContext,
    mut sce: &mut SingleChannelElement,
    mut audio: &mut [c_float; 3 * 1024],
) {
    match sce.ics.window_sequence[0] {
        ONLY_LONG_SEQUENCE => apply_only_long_window(sce, audio),
        LONG_START_SEQUENCE => apply_long_start_window(sce, audio),
        EIGHT_SHORT_SEQUENCE => apply_eight_short_window(sce, audio),
        LONG_STOP_SEQUENCE => apply_long_stop_window(sce, audio),
        _ => unreachable!(),
    }

    if sce.ics.window_sequence[0] as c_uint != EIGHT_SHORT_SEQUENCE {
        unsafe {
            mdct.mdct1024_fn.expect("non-null function pointer")(
                mdct.mdct1024,
                sce.coeffs.as_mut_ptr().cast(),
                sce.ret_buf.as_mut_ptr().cast(),
                size_of::<c_float>() as ptrdiff_t,
            )
        };
    } else {
        for (coeffs, output) in zip(
            sce.coeffs.array_chunks_mut::<128>(),
            sce.ret_buf.array_chunks_mut::<256>(),
        ) {
            unsafe {
                mdct.mdct128_fn.expect("non-null function pointer")(
                    mdct.mdct128,
                    coeffs.as_mut_ptr().cast(),
                    output.as_mut_ptr().cast(),
                    size_of::<c_float>() as ptrdiff_t,
                )
            };
        }
    }

    audio.copy_within(1024..1024 * 2, 0);
    sce.pcoeffs = sce.coeffs;
}
