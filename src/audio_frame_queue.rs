use std::mem::size_of;

use ::libc;
use ffi::{
    codec::{frame::AVFrame, AVCodecContext},
    num::AVRational,
};
use libc::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

use crate::{avutil::mathematics::av_rescale_q, types::*};
extern "C" {
    fn av_fast_realloc(ptr: *mut c_void, size: *mut c_uint, min_size: c_ulong) -> *mut c_void;
    fn memmove(_: *mut c_void, _: *const c_void, _: c_ulong) -> *mut c_void;
    fn memset(_: *mut c_void, _: c_int, _: c_ulong) -> *mut c_void;
    fn av_freep(ptr: *mut c_void);
    fn av_log(avcl: *mut c_void, level: c_int, fmt: *const c_char, _: ...);
}

#[inline(always)]
unsafe extern "C" fn ff_samples_to_time_base(
    avctx: *const AVCodecContext,
    samples: c_long,
) -> c_long {
    if samples == 0x8000000000000000 as c_ulong as c_long {
        return 0x8000000000000000 as c_ulong as c_long;
    }
    av_rescale_q(
        samples,
        {
            AVRational {
                num: 1 as c_int,
                den: (*avctx).sample_rate,
            }
        },
        (*avctx).time_base,
    )
}

#[cold]
pub unsafe extern "C" fn ff_af_queue_init(avctx: *mut AVCodecContext, afq: *mut AudioFrameQueue) {
    (*afq).avctx = avctx;
    (*afq).remaining_delay = (*avctx).initial_padding;
    (*afq).remaining_samples = (*avctx).initial_padding;
    (*afq).frame_count = 0 as c_int as c_uint;
}

pub unsafe extern "C" fn ff_af_queue_close(afq: *mut AudioFrameQueue) {
    if (*afq).frame_count != 0 {
        av_log(
            (*afq).avctx as *mut c_void,
            24 as c_int,
            b"%d frames left in the queue on closing\n\0" as *const u8 as *const c_char,
            (*afq).frame_count,
        );
    }
    av_freep(&mut (*afq).frames as *mut *mut AudioFrame as *mut c_void);
    memset(
        afq as *mut c_void,
        0 as c_int,
        size_of::<AudioFrameQueue>() as c_ulong,
    );
}

pub unsafe extern "C" fn ff_af_queue_add(afq: *mut AudioFrameQueue, f: *const AVFrame) -> c_int {
    let mut new: *mut AudioFrame = av_fast_realloc(
        (*afq).frames as *mut c_void,
        &mut (*afq).frame_alloc,
        (size_of::<AudioFrame>() as c_ulong)
            .wrapping_mul(((*afq).frame_count).wrapping_add(1 as c_int as c_uint) as c_ulong),
    ) as *mut AudioFrame;
    if new.is_null() {
        return -(12 as c_int);
    }
    (*afq).frames = new;
    new = new.offset((*afq).frame_count as isize);
    (*new).duration = (*f).nb_samples;
    (*new).duration += (*afq).remaining_delay;
    if (*f).pts != 0x8000000000000000 as c_ulong as c_long {
        (*new).pts = av_rescale_q((*f).pts, (*(*afq).avctx).time_base, {
            AVRational {
                num: 1 as c_int,
                den: (*(*afq).avctx).sample_rate,
            }
        });
        (*new).pts -= (*afq).remaining_delay as c_long;
        if (*afq).frame_count != 0 && (*new.offset(-(1 as c_int) as isize)).pts >= (*new).pts {
            av_log(
                (*afq).avctx as *mut c_void,
                24 as c_int,
                b"Queue input is backward in time\n\0" as *const u8 as *const c_char,
            );
        }
    } else {
        (*new).pts = 0x8000000000000000 as c_ulong as c_long;
    }
    (*afq).remaining_delay = 0 as c_int;
    (*afq).remaining_samples += (*f).nb_samples;
    (*afq).frame_count = ((*afq).frame_count).wrapping_add(1);
    (*afq).frame_count;
    0 as c_int
}

pub unsafe extern "C" fn ff_af_queue_remove(
    afq: *mut AudioFrameQueue,
    mut nb_samples: c_int,
    pts: *mut c_long,
    duration: *mut c_long,
) {
    let mut out_pts: c_long = 0x8000000000000000 as c_ulong as c_long;
    let mut removed_samples: c_int = 0 as c_int;
    let mut i: c_int = 0;
    if ((*afq).frame_count != 0 || (*afq).frame_alloc != 0)
        && (*(*afq).frames).pts != 0x8000000000000000 as c_ulong as c_long
    {
        out_pts = (*(*afq).frames).pts;
    }
    if (*afq).frame_count == 0 {
        av_log(
            (*afq).avctx as *mut c_void,
            24 as c_int,
            b"Trying to remove %d samples, but the queue is empty\n\0" as *const u8
                as *const c_char,
            nb_samples,
        );
    }
    if !pts.is_null() {
        *pts = ff_samples_to_time_base((*afq).avctx, out_pts);
    }
    i = 0 as c_int;
    while nb_samples != 0 && (i as c_uint) < (*afq).frame_count {
        let n: c_int = if (*((*afq).frames).offset(i as isize)).duration > nb_samples {
            nb_samples
        } else {
            (*((*afq).frames).offset(i as isize)).duration
        };
        (*((*afq).frames).offset(i as isize)).duration -= n;
        nb_samples -= n;
        removed_samples += n;
        if (*((*afq).frames).offset(i as isize)).pts != 0x8000000000000000 as c_ulong as c_long {
            let fresh0 = &mut (*((*afq).frames).offset(i as isize)).pts;
            *fresh0 += n as c_long;
        }
        i += 1;
        i;
    }
    (*afq).remaining_samples -= removed_samples;
    i -= (i != 0 && (*((*afq).frames).offset((i - 1 as c_int) as isize)).duration != 0) as c_int;
    memmove(
        (*afq).frames as *mut c_void,
        ((*afq).frames).offset(i as isize) as *const c_void,
        (size_of::<AudioFrame>() as c_ulong)
            .wrapping_mul(((*afq).frame_count).wrapping_sub(i as c_uint) as c_ulong),
    );
    (*afq).frame_count = ((*afq).frame_count).wrapping_sub(i as c_uint);
    if nb_samples != 0 {
        assert_eq!((*afq).frame_count, 0);
        assert_eq!((*afq).remaining_samples, (*afq).remaining_delay);
        if !((*afq).frames).is_null()
            && (*((*afq).frames).offset(0 as c_int as isize)).pts
                != 0x8000000000000000 as c_ulong as c_long
        {
            let fresh1 = &mut (*((*afq).frames).offset(0 as c_int as isize)).pts;
            *fresh1 += nb_samples as c_long;
        }
        av_log(
            (*afq).avctx as *mut c_void,
            48 as c_int,
            b"Trying to remove %d more samples than there are in the queue\n\0" as *const u8
                as *const c_char,
            nb_samples,
        );
    }
    if !duration.is_null() {
        *duration = ff_samples_to_time_base((*afq).avctx, removed_samples as c_long);
    }
}
