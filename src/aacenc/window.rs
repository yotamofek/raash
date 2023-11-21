use std::{mem::size_of, ptr};

use libc::{c_float, c_int, c_uint, c_ulong, c_void};

use super::ctx::AACEncContext;
use crate::{
    aactab::{KBD_LONG, KBD_SHORT},
    sinewin::{ff_sine_1024, ff_sine_128},
    types::{ptrdiff_t, AVFloatDSPContext, SingleChannelElement, EIGHT_SHORT_SEQUENCE},
};

unsafe fn apply_only_long_window(
    mut fdsp: *mut AVFloatDSPContext,
    mut sce: *mut SingleChannelElement,
    mut audio: *const c_float,
) {
    let mut lwindow: *const c_float = if (*sce).ics.use_kb_window[0] as c_int != 0 {
        KBD_LONG.as_ptr()
    } else {
        ff_sine_1024.as_ptr()
    };
    let mut pwindow: *const c_float = if (*sce).ics.use_kb_window[1] as c_int != 0 {
        KBD_LONG.as_ptr()
    } else {
        ff_sine_1024.as_ptr()
    };
    let mut out: *mut c_float = ((*sce).ret_buf).as_mut_ptr();
    ((*fdsp).vector_fmul).expect("non-null function pointer")(out, audio, lwindow, 1024 as c_int);
    ((*fdsp).vector_fmul_reverse).expect("non-null function pointer")(
        out.offset(1024 as c_int as isize),
        audio.offset(1024 as c_int as isize),
        pwindow,
        1024 as c_int,
    );
}
unsafe fn apply_long_start_window(
    mut fdsp: *mut AVFloatDSPContext,
    mut sce: *mut SingleChannelElement,
    mut audio: *const c_float,
) {
    let mut lwindow: *const c_float = if (*sce).ics.use_kb_window[1] as c_int != 0 {
        KBD_LONG.as_ptr()
    } else {
        ff_sine_1024.as_ptr()
    };
    let mut swindow: *const c_float = if (*sce).ics.use_kb_window[0] as c_int != 0 {
        KBD_SHORT.as_ptr()
    } else {
        ff_sine_128.as_ptr()
    };
    let mut out: *mut c_float = ((*sce).ret_buf).as_mut_ptr();
    ((*fdsp).vector_fmul).expect("non-null function pointer")(out, audio, lwindow, 1024 as c_int);
    ptr::copy_nonoverlapping(
        audio.offset(1024 as c_int as isize),
        out.offset(1024 as c_int as isize),
        448,
    );
    ((*fdsp).vector_fmul_reverse).expect("non-null function pointer")(
        out.offset(1024 as c_int as isize)
            .offset(448 as c_int as isize),
        audio
            .offset(1024 as c_int as isize)
            .offset(448 as c_int as isize),
        swindow,
        128 as c_int,
    );
    ptr::write_bytes(
        out.offset(1024 as c_int as isize)
            .offset(576 as c_int as isize),
        0,
        448,
    );
}
unsafe fn apply_long_stop_window(
    mut fdsp: *mut AVFloatDSPContext,
    mut sce: *mut SingleChannelElement,
    mut audio: *const c_float,
) {
    let mut lwindow: *const c_float = if (*sce).ics.use_kb_window[0] as c_int != 0 {
        KBD_LONG.as_ptr()
    } else {
        ff_sine_1024.as_ptr()
    };
    let mut swindow: *const c_float = if (*sce).ics.use_kb_window[1] as c_int != 0 {
        KBD_SHORT.as_ptr()
    } else {
        ff_sine_128.as_ptr()
    };
    let mut out: *mut c_float = ((*sce).ret_buf).as_mut_ptr();
    ptr::write_bytes(out, 0, 448);
    ((*fdsp).vector_fmul).expect("non-null function pointer")(
        out.offset(448 as c_int as isize),
        audio.offset(448 as c_int as isize),
        swindow,
        128 as c_int,
    );
    ptr::copy_nonoverlapping(
        audio.offset(576 as c_int as isize),
        out.offset(576 as c_int as isize),
        448,
    );
    ((*fdsp).vector_fmul_reverse).expect("non-null function pointer")(
        out.offset(1024 as c_int as isize),
        audio.offset(1024 as c_int as isize),
        lwindow,
        1024 as c_int,
    );
}
unsafe fn apply_eight_short_window(
    mut fdsp: *mut AVFloatDSPContext,
    mut sce: *mut SingleChannelElement,
    mut audio: *const c_float,
) {
    let mut swindow: *const c_float = if (*sce).ics.use_kb_window[0] as c_int != 0 {
        KBD_SHORT.as_ptr()
    } else {
        ff_sine_128.as_ptr()
    };
    let mut pwindow: *const c_float = if (*sce).ics.use_kb_window[1] as c_int != 0 {
        KBD_SHORT.as_ptr()
    } else {
        ff_sine_128.as_ptr()
    };
    let mut in_0: *const c_float = audio.offset(448 as c_int as isize);
    let mut out: *mut c_float = ((*sce).ret_buf).as_mut_ptr();
    let mut w: c_int = 0;
    w = 0 as c_int;
    while w < 8 as c_int {
        ((*fdsp).vector_fmul).expect("non-null function pointer")(
            out,
            in_0,
            if w != 0 { pwindow } else { swindow },
            128 as c_int,
        );
        out = out.offset(128 as c_int as isize);
        in_0 = in_0.offset(128 as c_int as isize);
        ((*fdsp).vector_fmul_reverse).expect("non-null function pointer")(
            out,
            in_0,
            swindow,
            128 as c_int,
        );
        out = out.offset(128 as c_int as isize);
        w += 1;
        w;
    }
}

pub(super) const APPLY_WINDOW: [unsafe fn(
    *mut AVFloatDSPContext,
    *mut SingleChannelElement,
    *const c_float,
) -> (); 4] = [
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
    APPLY_WINDOW[(*sce).ics.window_sequence[0] as usize]((*s).fdsp, sce, audio);
    if (*sce).ics.window_sequence[0] as c_uint != EIGHT_SHORT_SEQUENCE as c_int as c_uint {
        ((*s).mdct1024_fn).expect("non-null function pointer")(
            (*s).mdct1024,
            ((*sce).coeffs).as_mut_ptr() as *mut c_void,
            output as *mut c_void,
            size_of::<c_float>() as c_ulong as ptrdiff_t,
        );
    } else {
        i = 0 as c_int;
        while i < 1024 as c_int {
            ((*s).mdct128_fn).expect("non-null function pointer")(
                (*s).mdct128,
                &mut *((*sce).coeffs).as_mut_ptr().offset(i as isize) as *mut c_float
                    as *mut c_void,
                output.offset((i * 2 as c_int) as isize) as *mut c_void,
                size_of::<c_float>() as c_ulong as ptrdiff_t,
            );
            i += 128 as c_int;
        }
    }
    ptr::copy_nonoverlapping(audio.offset(1024 as c_int as isize), audio, 1024);
    (*sce).pcoeffs = (*sce).coeffs;
}
