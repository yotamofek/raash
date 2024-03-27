use std::{mem::size_of, ptr, slice};

use itertools::izip;
use libc::{c_float, c_int, c_uint, c_ulong, c_void};

use super::ctx::AACEncContext;
use crate::{
    aac::{
        tables::{KBD_LONG, KBD_SHORT},
        EIGHT_SHORT_SEQUENCE,
    },
    sinewin::{SINE_WIN_1024, SINE_WIN_128},
    types::{ptrdiff_t, SingleChannelElement},
};

unsafe fn apply_only_long_window(mut sce: *mut SingleChannelElement, mut audio: *const c_float) {
    let audio = slice::from_raw_parts(audio, 1024 * 2);
    let out = &mut *(*sce).ret_buf;

    let [lwindow, pwindow] = (*sce).ics.use_kb_window.map(|use_kb| {
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

unsafe fn apply_long_start_window(mut sce: *mut SingleChannelElement, mut audio: *const c_float) {
    let audio = slice::from_raw_parts(audio, 1024 * 2);
    let out = &mut *(*sce).ret_buf;

    let mut lwindow = if (*sce).ics.use_kb_window[1] != 0 {
        &*KBD_LONG
    } else {
        &SINE_WIN_1024
    };
    let mut swindow = if (*sce).ics.use_kb_window[0] != 0 {
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

unsafe fn apply_long_stop_window(mut sce: *mut SingleChannelElement, mut audio: *const c_float) {
    let audio = slice::from_raw_parts(audio, 1024 * 2);
    let out = &mut *(*sce).ret_buf;

    let mut lwindow = if (*sce).ics.use_kb_window[0] != 0 {
        &*KBD_LONG
    } else {
        &SINE_WIN_1024
    };
    let mut swindow = if (*sce).ics.use_kb_window[1] != 0 {
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

unsafe fn apply_eight_short_window(mut sce: *mut SingleChannelElement, mut audio: *const c_float) {
    let audio = slice::from_raw_parts(audio, 1024 * 2);
    let out = &mut *(*sce).ret_buf;

    let mut swindow = if (*sce).ics.use_kb_window[0] != 0 {
        &*KBD_SHORT
    } else {
        &SINE_WIN_128
    };
    let mut pwindow = if (*sce).ics.use_kb_window[1] != 0 {
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

pub(super) const APPLY_WINDOW: [unsafe fn(*mut SingleChannelElement, *const c_float) -> (); 4] = [
    apply_only_long_window,
    apply_long_start_window,
    apply_eight_short_window,
    apply_long_stop_window,
];

pub(super) unsafe fn apply_window_and_mdct(
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
    mut audio: *mut c_float,
) {
    let mut i: c_int = 0;
    let mut output: *mut c_float = ((*sce).ret_buf).as_mut_ptr();
    APPLY_WINDOW[(*sce).ics.window_sequence[0] as usize](sce, audio);
    if (*sce).ics.window_sequence[0] as c_uint != EIGHT_SHORT_SEQUENCE as c_int as c_uint {
        ((*s).mdct1024_fn).expect("non-null function pointer")(
            (*s).mdct1024,
            ((*sce).coeffs).as_mut_ptr() as *mut c_void,
            output as *mut c_void,
            size_of::<c_float>() as c_ulong as ptrdiff_t,
        );
    } else {
        i = 0;
        while i < 1024 {
            ((*s).mdct128_fn).expect("non-null function pointer")(
                (*s).mdct128,
                &mut *((*sce).coeffs).as_mut_ptr().offset(i as isize) as *mut c_float
                    as *mut c_void,
                output.offset((i * 2) as isize) as *mut c_void,
                size_of::<c_float>() as c_ulong as ptrdiff_t,
            );
            i += 128;
        }
    }
    ptr::copy_nonoverlapping(audio.offset(1024), audio, 1024);
    (*sce).pcoeffs = (*sce).coeffs;
}
