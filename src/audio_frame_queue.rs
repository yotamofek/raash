use ffi::{
    codec::{frame::AVFrame, AVCodecContext},
    num::AVRational,
};
use libc::{c_char, c_int, c_long, c_uint, c_void};

use crate::avutil::mathematics::av_rescale_q;
extern "C" {
    fn av_log(avcl: *mut c_void, level: c_int, fmt: *const c_char, _: ...);
}

#[derive(Copy, Clone)]
struct AudioFrame {
    pts: c_long,
    duration: c_int,
}

#[derive(Copy, Clone)]
pub(crate) struct AudioRemoved {
    pub(crate) pts: c_long,
    pub(crate) duration: c_long,
}

#[derive(Clone)]
pub(crate) struct AudioFrameQueue {
    avctx: *const AVCodecContext,
    remaining_delay: c_int,
    remaining_samples: c_int,
    frames: Vec<AudioFrame>,
    last_pts: c_long,
}

#[inline(always)]
fn ff_samples_to_time_base(avctx: &AVCodecContext, samples: c_long) -> c_long {
    if samples == c_long::MIN {
        return c_long::MIN;
    }
    av_rescale_q(
        samples,
        {
            AVRational {
                num: 1,
                den: (*avctx).sample_rate,
            }
        },
        (*avctx).time_base,
    )
}

impl AudioFrameQueue {
    /// Source: [libavcodec/audio_frame_queue.c](https://github.com/ffmpeg/ffmpeg/blob/59eadb5060acd07ad2d4dc5dbb354ee81f034222/libavcodec/audio_frame_queue.c#L28-L34)
    pub unsafe fn new(avctx: *const AVCodecContext) -> Self {
        Self {
            avctx,
            remaining_delay: (*avctx).initial_padding,
            remaining_samples: (*avctx).initial_padding,
            frames: vec![],
            last_pts: c_long::MIN,
        }
    }

    /// Source: [libavcodec/audio_frame_queue.c](https://github.com/ffmpeg/ffmpeg/blob/59eadb5060acd07ad2d4dc5dbb354ee81f034222/libavcodec/audio_frame_queue.c#L44-L73)
    pub unsafe fn add_frame(&mut self, f: &AVFrame) {
        let new = AudioFrame {
            pts: if f.pts != c_long::MIN {
                let pts = av_rescale_q(f.pts, (*self.avctx).time_base, {
                    AVRational {
                        num: 1,
                        den: (*self.avctx).sample_rate,
                    }
                }) - self.remaining_delay as c_long;

                if let Some(last_frame) = self.frames.last()
                    && last_frame.pts >= pts
                {
                    av_log(
                        self.avctx as *mut c_void,
                        24,
                        c"Queue input is backward in time\n".as_ptr(),
                    );
                }

                pts
            } else {
                c_long::MIN
            },
            duration: f.nb_samples + self.remaining_delay,
        };

        self.frames.push(new);

        self.remaining_delay = 0;
        self.remaining_samples += f.nb_samples;
    }

    /// Source: [libavcodec/audio_frame_queue.c](https://github.com/ffmpeg/ffmpeg/blob/59eadb5060acd07ad2d4dc5dbb354ee81f034222/libavcodec/audio_frame_queue.c#L75-L113)
    pub unsafe fn remove(&mut self, mut nb_samples: c_int) -> AudioRemoved {
        if self.frames.is_empty() {
            av_log(
                self.avctx as *mut c_void,
                24,
                b"Trying to remove %d samples, but the queue is empty\n\0" as *const u8
                    as *const c_char,
                nb_samples,
            );
        }

        let mut removed_samples: c_int = 0;

        let pts = ff_samples_to_time_base(
            &*self.avctx,
            self.frames
                .first()
                .map(|&AudioFrame { pts, .. }| pts)
                .unwrap_or(self.last_pts),
        );

        let mut i = 0;
        while nb_samples != 0 && (i as c_uint) < self.frames.len() as c_uint {
            let n: c_int = self.frames[i as usize].duration.min(nb_samples);
            self.frames[i as usize].duration -= n;
            nb_samples -= n;
            removed_samples += n;
            if self.frames[i as usize].pts != c_long::MIN {
                let fresh0 = &mut self.frames[i as usize].pts;
                *fresh0 += n as c_long;
            }
            i += 1;
            i;
        }
        self.remaining_samples -= removed_samples;
        i -= (i != 0 && self.frames[(i - 1) as usize].duration != 0) as c_int;

        if let Some(AudioFrame { pts, .. }) = self.frames.drain(..i as usize).next() {
            self.last_pts = pts;
        }

        if nb_samples != 0 {
            assert!(self.frames.is_empty());
            assert_eq!(self.remaining_samples, self.remaining_delay);

            let last_pts = self
                .frames
                .first_mut()
                .map(|AudioFrame { pts, .. }| pts)
                .unwrap_or(&mut self.last_pts);
            if *last_pts != c_long::MIN {
                *last_pts += nb_samples as c_long;
            }

            av_log(
                self.avctx as *mut c_void,
                48,
                c"Trying to remove %d more samples than there are in the queue\n".as_ptr(),
                nb_samples,
            );
        }

        AudioRemoved {
            pts,
            duration: ff_samples_to_time_base(&*self.avctx, removed_samples as c_long),
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.remaining_samples == 0 || self.frames.is_empty()
    }
}

impl Drop for AudioFrameQueue {
    /// Source: [libavcodec/audio_frame_queue.c](https://github.com/ffmpeg/ffmpeg/blob/59eadb5060acd07ad2d4dc5dbb354ee81f034222/libavcodec/audio_frame_queue.c#L36-L42)
    fn drop(&mut self) {
        if !self.frames.is_empty() {
            unsafe {
                av_log(
                    self.avctx as *mut c_void,
                    24,
                    b"%d frames left in the queue on closing\n\0" as *const u8 as *const c_char,
                    self.frames.len() as c_uint,
                )
            };
        }
    }
}
