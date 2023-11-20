use crate::{avutil::mathematics::av_rescale_q, types::*};

use ::libc;
extern "C" {
    fn av_fast_realloc(
        ptr: *mut libc::c_void,
        size: *mut libc::c_uint,
        min_size: size_t,
    ) -> *mut libc::c_void;
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn av_freep(ptr: *mut libc::c_void);
    fn av_log(avcl: *mut libc::c_void, level: libc::c_int, fmt: *const libc::c_char, _: ...);
}

#[inline(always)]
unsafe extern "C" fn ff_samples_to_time_base(
    mut avctx: *const AVCodecContext,
    mut samples: int64_t,
) -> int64_t {
    if samples == 0x8000000000000000 as libc::c_ulong as int64_t {
        return 0x8000000000000000 as libc::c_ulong as int64_t;
    }
    return av_rescale_q(
        samples,
        {
            let mut init = AVRational {
                num: 1 as libc::c_int,
                den: (*avctx).sample_rate,
            };
            init
        },
        (*avctx).time_base,
    );
}

#[cold]
pub unsafe extern "C" fn ff_af_queue_init(
    mut avctx: *mut AVCodecContext,
    mut afq: *mut AudioFrameQueue,
) {
    (*afq).avctx = avctx;
    (*afq).remaining_delay = (*avctx).initial_padding;
    (*afq).remaining_samples = (*avctx).initial_padding;
    (*afq).frame_count = 0 as libc::c_int as libc::c_uint;
}

pub unsafe extern "C" fn ff_af_queue_close(mut afq: *mut AudioFrameQueue) {
    if (*afq).frame_count != 0 {
        av_log(
            (*afq).avctx as *mut libc::c_void,
            24 as libc::c_int,
            b"%d frames left in the queue on closing\n\0" as *const u8 as *const libc::c_char,
            (*afq).frame_count,
        );
    }
    av_freep(&mut (*afq).frames as *mut *mut AudioFrame as *mut libc::c_void);
    memset(
        afq as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<AudioFrameQueue>() as libc::c_ulong,
    );
}

pub unsafe extern "C" fn ff_af_queue_add(
    mut afq: *mut AudioFrameQueue,
    mut f: *const AVFrame,
) -> libc::c_int {
    let mut new: *mut AudioFrame = av_fast_realloc(
        (*afq).frames as *mut libc::c_void,
        &mut (*afq).frame_alloc,
        (::core::mem::size_of::<AudioFrame>() as libc::c_ulong).wrapping_mul(
            ((*afq).frame_count).wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
        ),
    ) as *mut AudioFrame;
    if new.is_null() {
        return -(12 as libc::c_int);
    }
    (*afq).frames = new;
    new = new.offset((*afq).frame_count as isize);
    (*new).duration = (*f).nb_samples;
    (*new).duration += (*afq).remaining_delay;
    if (*f).pts != 0x8000000000000000 as libc::c_ulong as int64_t {
        (*new).pts = av_rescale_q((*f).pts, (*(*afq).avctx).time_base, {
            let mut init = AVRational {
                num: 1 as libc::c_int,
                den: (*(*afq).avctx).sample_rate,
            };
            init
        });
        (*new).pts -= (*afq).remaining_delay as libc::c_long;
        if (*afq).frame_count != 0 && (*new.offset(-(1 as libc::c_int) as isize)).pts >= (*new).pts
        {
            av_log(
                (*afq).avctx as *mut libc::c_void,
                24 as libc::c_int,
                b"Queue input is backward in time\n\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        (*new).pts = 0x8000000000000000 as libc::c_ulong as int64_t;
    }
    (*afq).remaining_delay = 0 as libc::c_int;
    (*afq).remaining_samples += (*f).nb_samples;
    (*afq).frame_count = ((*afq).frame_count).wrapping_add(1);
    (*afq).frame_count;
    return 0 as libc::c_int;
}

pub unsafe extern "C" fn ff_af_queue_remove(
    mut afq: *mut AudioFrameQueue,
    mut nb_samples: libc::c_int,
    mut pts: *mut int64_t,
    mut duration: *mut int64_t,
) {
    let mut out_pts: int64_t = 0x8000000000000000 as libc::c_ulong as int64_t;
    let mut removed_samples: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    if (*afq).frame_count != 0 || (*afq).frame_alloc != 0 {
        if (*(*afq).frames).pts != 0x8000000000000000 as libc::c_ulong as int64_t {
            out_pts = (*(*afq).frames).pts;
        }
    }
    if (*afq).frame_count == 0 {
        av_log(
            (*afq).avctx as *mut libc::c_void,
            24 as libc::c_int,
            b"Trying to remove %d samples, but the queue is empty\n\0" as *const u8
                as *const libc::c_char,
            nb_samples,
        );
    }
    if !pts.is_null() {
        *pts = ff_samples_to_time_base((*afq).avctx, out_pts);
    }
    i = 0 as libc::c_int;
    while nb_samples != 0 && (i as libc::c_uint) < (*afq).frame_count {
        let mut n: libc::c_int = if (*((*afq).frames).offset(i as isize)).duration > nb_samples {
            nb_samples
        } else {
            (*((*afq).frames).offset(i as isize)).duration
        };
        (*((*afq).frames).offset(i as isize)).duration -= n;
        nb_samples -= n;
        removed_samples += n;
        if (*((*afq).frames).offset(i as isize)).pts
            != 0x8000000000000000 as libc::c_ulong as int64_t
        {
            let ref mut fresh0 = (*((*afq).frames).offset(i as isize)).pts;
            *fresh0 += n as libc::c_long;
        }
        i += 1;
        i;
    }
    (*afq).remaining_samples -= removed_samples;
    i -= (i != 0 && (*((*afq).frames).offset((i - 1 as libc::c_int) as isize)).duration != 0)
        as libc::c_int;
    memmove(
        (*afq).frames as *mut libc::c_void,
        ((*afq).frames).offset(i as isize) as *const libc::c_void,
        (::core::mem::size_of::<AudioFrame>() as libc::c_ulong)
            .wrapping_mul(((*afq).frame_count).wrapping_sub(i as libc::c_uint) as libc::c_ulong),
    );
    (*afq).frame_count = ((*afq).frame_count).wrapping_sub(i as libc::c_uint);
    if nb_samples != 0 {
        assert_eq!((*afq).frame_count, 0);
        assert_eq!((*afq).remaining_samples, (*afq).remaining_delay);
        if !((*afq).frames).is_null()
            && (*((*afq).frames).offset(0 as libc::c_int as isize)).pts
                != 0x8000000000000000 as libc::c_ulong as int64_t
        {
            let ref mut fresh1 = (*((*afq).frames).offset(0 as libc::c_int as isize)).pts;
            *fresh1 += nb_samples as libc::c_long;
        }
        av_log(
            (*afq).avctx as *mut libc::c_void,
            48 as libc::c_int,
            b"Trying to remove %d more samples than there are in the queue\n\0" as *const u8
                as *const libc::c_char,
            nb_samples,
        );
    }
    if !duration.is_null() {
        *duration = ff_samples_to_time_base((*afq).avctx, removed_samples as int64_t);
    }
}
