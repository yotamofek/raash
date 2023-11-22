#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

mod channel_layout;
pub(crate) mod ctx;
mod dsp;
pub mod options;
mod pb;
pub(crate) mod pow;
mod window;

use std::{
    ffi::CStr,
    iter::zip,
    mem::size_of,
    ptr::{self, addr_of_mut, null, null_mut},
    slice,
};

use ffi::{
    class::AVClass,
    codec::{
        channel::{self, AVChannelLayout, ChannelLayoutMaskOrMap},
        frame::AVFrame,
        AVCodec, AVCodecContext, AVCodecHWConfigInternal, AVPacket, AVPixelFormat, AVProfile,
        CodecCallback, FFCodec, FFCodecDefault, FF_CODEC_CB_TYPE_ENCODE,
    },
    num::AVRational,
};
use itertools::{izip, Itertools};
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_schar, c_uchar, c_uint, c_ulong, c_ulonglong,
    c_ushort, c_void,
};

use self::{
    channel_layout::pce,
    ctx::{AACEncContext, PrivData},
    options::OPTIONS,
    pb::*,
    pow::Pow34,
    window::{apply_window_and_mdct, APPLY_WINDOW},
};
use crate::{
    aaccoder::{
        coder, encode_window_bands_info, ms::search_for_ms, pns,
        quantize_and_encode_band::quantize_and_encode_band, set_special_band_scalefactors,
    },
    aacenc_is::search_for_is,
    aacenc_ltp::{
        adjust_common_ltp, encode_ltp_info, ltp_insert_new_frame, search_for_ltp, update_ltp,
    },
    aacenc_pred::{adjust_common_pred, apply_main_pred, encode_main_pred, search_for_pred},
    aacenc_tns::{apply_tns, encode_tns_info, search_for_tns},
    aacenctab::{ff_aac_swb_size_1024, ff_aac_swb_size_128},
    aactab::{
        ff_aac_float_common_init, ff_aac_num_swb_1024, ff_aac_num_swb_128, ff_aac_scalefactor_bits,
        ff_aac_scalefactor_code, ff_swb_offset_1024, ff_swb_offset_128, ff_tns_max_bands_1024,
        ff_tns_max_bands_128,
    },
    audio_frame_queue::{ff_af_queue_add, ff_af_queue_close, ff_af_queue_init, ff_af_queue_remove},
    avutil::{log::av_default_item_name, tx::av_tx_uninit},
    common::*,
    lpc::{ff_lpc_end, ff_lpc_init},
    mpeg4audio_sample_rates::ff_mpeg4audio_sample_rates,
    psymodel::{
        ff_psy_end, ff_psy_init, ff_psy_preprocess, ff_psy_preprocess_end, ff_psy_preprocess_init,
    },
    types::*,
};

extern "C" {
    fn av_channel_layout_describe(
        channel_layout: *const AVChannelLayout,
        buf: *mut c_char,
        buf_size: c_ulong,
    ) -> c_int;
    fn av_channel_layout_compare(
        chl: *const AVChannelLayout,
        chl1: *const AVChannelLayout,
    ) -> c_int;
    fn avpriv_float_dsp_alloc(strict: c_int) -> *mut AVFloatDSPContext;
    fn av_mallocz(size: c_ulong) -> *mut c_void;
    fn av_calloc(nmemb: c_ulong, size: c_ulong) -> *mut c_void;
    fn av_freep(ptr: *mut c_void);
    fn av_log(avcl: *mut c_void, level: c_int, fmt: *const c_char, _: ...);
    fn ff_alloc_packet(avctx: *mut AVCodecContext, avpkt: *mut AVPacket, size: c_long) -> c_int;
}

static mut aacenc_profiles: [c_int; 4] = [0 as c_int, 1 as c_int, 3 as c_int, 128 as c_int];

#[inline]
pub(crate) unsafe fn abs_pow34_v(mut out: *mut c_float, mut in_0: *const c_float, size: c_int) {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < size {
        *out.offset(i as isize) = (*in_0.offset(i as isize)).abs_pow34();
        i += 1;
        i;
    }
}

pub(crate) unsafe fn quantize_bands(
    mut out: *mut c_int,
    mut in_0: *const c_float,
    mut scaled: *const c_float,
    mut size: c_int,
    mut is_signed: bool,
    mut maxval: c_int,
    Q34: c_float,
    rounding: c_float,
) {
    let out = slice::from_raw_parts_mut::<c_int>(out, size as usize);
    let in_0 = slice::from_raw_parts::<c_float>(in_0, size as usize);
    let scaled = slice::from_raw_parts::<c_float>(scaled, size as usize);
    zip(in_0, scaled)
        .map(|(&in_0, &scaled)| {
            let qc = scaled * Q34;
            let mut out = (qc + rounding).min(maxval as c_float) as c_int;
            if is_signed && in_0 < 0.0f32 {
                out = -out;
            }
            out
        })
        .zip(out)
        .for_each(|(val, out)| *out = val);
}

unsafe extern "C" fn put_pce(mut pb: *mut PutBitContext, mut avctx: *mut AVCodecContext) {
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut s: *mut AACEncContext = (*((*avctx).priv_data as *mut PrivData)).ctx;
    let mut pce: *mut pce::Info = &mut (*s).pce.unwrap();
    let bitexact: c_int = (*avctx).flags & (1 as c_int) << 23 as c_int;
    let mut aux_data = if bitexact != 0 {
        c"Lavc"
    } else {
        c"Lavc60.33.100"
    };
    put_bits(pb, 4 as c_int, 0 as c_int as BitBuf);
    put_bits(pb, 2 as c_int, (*avctx).profile as BitBuf);
    put_bits(pb, 4 as c_int, (*s).samplerate_index as BitBuf);
    put_bits(pb, 4 as c_int, (*pce).num_ele[0] as BitBuf);
    put_bits(pb, 4 as c_int, (*pce).num_ele[1] as BitBuf);
    put_bits(pb, 4 as c_int, (*pce).num_ele[2] as BitBuf);
    put_bits(pb, 2 as c_int, (*pce).num_ele[3] as BitBuf);
    put_bits(pb, 3 as c_int, 0 as c_int as BitBuf);
    put_bits(pb, 4 as c_int, 0 as c_int as BitBuf);
    put_bits(pb, 1 as c_int, 0 as c_int as BitBuf);
    put_bits(pb, 1 as c_int, 0 as c_int as BitBuf);
    put_bits(pb, 1 as c_int, 0 as c_int as BitBuf);
    i = 0 as c_int;
    while i < 4 as c_int {
        j = 0 as c_int;
        while j < (*pce).num_ele[i as usize] {
            if i < 3 as c_int {
                put_bits(
                    pb,
                    1 as c_int,
                    (*pce).pairing[i as usize][j as usize] as BitBuf,
                );
            }
            put_bits(
                pb,
                4 as c_int,
                (*pce).index[i as usize][j as usize] as BitBuf,
            );
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    align_put_bits(pb);
    put_bits(pb, 8 as c_int, aux_data.to_bytes().len() as BitBuf);
    for c in aux_data.to_bytes() {
        put_bits(pb, 8, *c as u32);
    }
}
unsafe extern "C" fn put_audio_specific_config(mut avctx: *mut AVCodecContext) -> c_int {
    let mut pb: PutBitContext = PutBitContext {
        bit_buf: 0,
        bit_left: 0,
        buf: ptr::null_mut::<c_uchar>(),
        buf_ptr: ptr::null_mut::<c_uchar>(),
        buf_end: ptr::null_mut::<c_uchar>(),
    };

    let mut s: *mut AACEncContext = (*((*avctx).priv_data as *mut PrivData)).ctx;
    let mut channels: c_int = ((*s).needs_pce == 0) as c_int
        * ((*s).channels
            - (if (*s).channels == 8 as c_int {
                1 as c_int
            } else {
                0 as c_int
            }));
    let max_size: c_int = 32 as c_int;
    (*avctx).extradata = av_mallocz(max_size as c_ulong) as *mut c_uchar;
    if ((*avctx).extradata).is_null() {
        return -(12 as c_int);
    }
    init_put_bits(&mut pb, (*avctx).extradata, max_size);
    put_bits(&mut pb, 5 as c_int, ((*s).profile + 1 as c_int) as BitBuf);
    put_bits(&mut pb, 4 as c_int, (*s).samplerate_index as BitBuf);
    put_bits(&mut pb, 4 as c_int, channels as BitBuf);
    put_bits(&mut pb, 1 as c_int, 0 as c_int as BitBuf);
    put_bits(&mut pb, 1 as c_int, 0 as c_int as BitBuf);
    put_bits(&mut pb, 1 as c_int, 0 as c_int as BitBuf);
    if (*s).needs_pce != 0 {
        put_pce(&mut pb, avctx);
    }
    put_bits(&mut pb, 11 as c_int, 0x2b7 as c_int as BitBuf);
    put_bits(&mut pb, 5 as c_int, AOT_SBR as c_int as BitBuf);
    put_bits(&mut pb, 1 as c_int, 0 as c_int as BitBuf);
    flush_put_bits(&mut pb);
    (*avctx).extradata_size = put_bytes_output(&mut pb);
    0 as c_int
}

pub(crate) unsafe fn ff_quantize_band_cost_cache_init(mut s: *mut AACEncContext) {
    (*s).quantize_band_cost_cache_generation =
        ((*s).quantize_band_cost_cache_generation).wrapping_add(1);
    (*s).quantize_band_cost_cache_generation;
    if (*s).quantize_band_cost_cache_generation as c_int == 0 as c_int {
        (*s).quantize_band_cost_cache = [[AACQuantizeBandCostCacheEntry::default(); 128]; 256];
        (*s).quantize_band_cost_cache_generation = 1 as c_int as c_ushort;
    }
}

unsafe extern "C" fn put_ics_info(
    mut s: *mut AACEncContext,
    mut info: *mut IndividualChannelStream,
) {
    let mut w: c_int = 0;
    put_bits(&mut (*s).pb, 1 as c_int, 0 as c_int as BitBuf);
    put_bits(
        &mut (*s).pb,
        2 as c_int,
        (*info).window_sequence[0] as BitBuf,
    );
    put_bits(&mut (*s).pb, 1 as c_int, (*info).use_kb_window[0] as BitBuf);
    if (*info).window_sequence[0] as c_uint != EIGHT_SHORT_SEQUENCE as c_int as c_uint {
        put_bits(&mut (*s).pb, 6 as c_int, (*info).max_sfb as BitBuf);
        put_bits(
            &mut (*s).pb,
            1 as c_int,
            ((*info).predictor_present != 0) as c_int as BitBuf,
        );
    } else {
        put_bits(&mut (*s).pb, 4 as c_int, (*info).max_sfb as BitBuf);
        w = 1 as c_int;
        while w < 8 as c_int {
            put_bits(
                &mut (*s).pb,
                1 as c_int,
                ((*info).group_len[w as usize] == 0) as c_int as BitBuf,
            );
            w += 1;
            w;
        }
    };
}
unsafe extern "C" fn encode_ms_info(mut pb: *mut PutBitContext, mut cpe: *mut ChannelElement) {
    let mut i: c_int = 0;
    let mut w: c_int = 0;
    put_bits(pb, 2 as c_int, (*cpe).ms_mode as BitBuf);
    if (*cpe).ms_mode == 1 as c_int {
        w = 0 as c_int;
        while w < (*cpe).ch[0].ics.num_windows {
            i = 0 as c_int;
            while i < (*cpe).ch[0].ics.max_sfb as c_int {
                put_bits(
                    pb,
                    1 as c_int,
                    (*cpe).ms_mask[(w * 16 as c_int + i) as usize] as BitBuf,
                );
                i += 1;
                i;
            }
            w += (*cpe).ch[0].ics.group_len[w as usize] as c_int;
        }
    }
}
unsafe extern "C" fn adjust_frame_information(mut cpe: *mut ChannelElement, mut chans: c_int) {
    let mut i: c_int = 0;
    let mut w: c_int = 0;
    let mut w2: c_int = 0;
    let mut g: c_int = 0;
    let mut ch: c_int = 0;
    let mut maxsfb: c_int = 0;
    let mut cmaxsfb: c_int = 0;
    ch = 0 as c_int;
    while ch < chans {
        let mut ics: *mut IndividualChannelStream =
            &mut (*((*cpe).ch).as_mut_ptr().offset(ch as isize)).ics;
        maxsfb = 0 as c_int;
        (*cpe).ch[ch as usize].pulse.num_pulse = 0 as c_int;
        w = 0 as c_int;
        while w < (*ics).num_windows {
            w2 = 0 as c_int;
            while w2 < (*ics).group_len[w as usize] as c_int {
                cmaxsfb = (*ics).num_swb;
                while cmaxsfb > 0 as c_int
                    && (*cpe).ch[ch as usize].zeroes
                        [(w * 16 as c_int + cmaxsfb - 1 as c_int) as usize]
                        as c_int
                        != 0
                {
                    cmaxsfb -= 1;
                    cmaxsfb;
                }
                maxsfb = if maxsfb > cmaxsfb { maxsfb } else { cmaxsfb };
                w2 += 1;
                w2;
            }
            w += (*ics).group_len[w as usize] as c_int;
        }
        (*ics).max_sfb = maxsfb as c_uchar;
        w = 0 as c_int;
        while w < (*ics).num_windows {
            g = 0 as c_int;
            while g < (*ics).max_sfb as c_int {
                i = 1 as c_int;
                w2 = w;
                while w2 < w + (*ics).group_len[w as usize] as c_int {
                    if (*cpe).ch[ch as usize].zeroes[(w2 * 16 as c_int + g) as usize] == 0 {
                        i = 0 as c_int;
                        break;
                    } else {
                        w2 += 1;
                        w2;
                    }
                }
                (*cpe).ch[ch as usize].zeroes[(w * 16 as c_int + g) as usize] = i as c_uchar;
                g += 1;
                g;
            }
            w += (*ics).group_len[w as usize] as c_int;
        }
        ch += 1;
        ch;
    }
    if chans > 1 as c_int && (*cpe).common_window != 0 {
        let mut ics0: *mut IndividualChannelStream =
            &mut (*((*cpe).ch).as_mut_ptr().offset(0 as c_int as isize)).ics;
        let mut ics1: *mut IndividualChannelStream =
            &mut (*((*cpe).ch).as_mut_ptr().offset(1 as c_int as isize)).ics;
        let mut msc: c_int = 0 as c_int;
        (*ics0).max_sfb = (if (*ics0).max_sfb as c_int > (*ics1).max_sfb as c_int {
            (*ics0).max_sfb as c_int
        } else {
            (*ics1).max_sfb as c_int
        }) as c_uchar;
        (*ics1).max_sfb = (*ics0).max_sfb;
        w = 0 as c_int;
        while w < (*ics0).num_windows * 16 as c_int {
            i = 0 as c_int;
            while i < (*ics0).max_sfb as c_int {
                if (*cpe).ms_mask[(w + i) as usize] != 0 {
                    msc += 1;
                    msc;
                }
                i += 1;
                i;
            }
            w += 16 as c_int;
        }
        if msc == 0 as c_int || (*ics0).max_sfb as c_int == 0 as c_int {
            (*cpe).ms_mode = 0 as c_int;
        } else {
            (*cpe).ms_mode = if msc < (*ics0).max_sfb as c_int * (*ics0).num_windows {
                1 as c_int
            } else {
                2 as c_int
            };
        }
    }
}
unsafe extern "C" fn apply_intensity_stereo(mut cpe: *mut ChannelElement) {
    let mut w: c_int = 0;
    let mut w2: c_int = 0;
    let mut g: c_int = 0;
    let mut i: c_int = 0;
    let mut ics: *mut IndividualChannelStream =
        &mut (*((*cpe).ch).as_mut_ptr().offset(0 as c_int as isize)).ics;
    if (*cpe).common_window == 0 {
        return;
    }
    w = 0 as c_int;
    while w < (*ics).num_windows {
        w2 = 0 as c_int;
        while w2 < (*ics).group_len[w as usize] as c_int {
            let mut start: c_int = (w + w2) * 128 as c_int;
            g = 0 as c_int;
            while g < (*ics).num_swb {
                let mut p: c_int = (-(1 as c_int) as c_uint).wrapping_add(
                    (2 as c_int as c_uint).wrapping_mul(
                        ((*cpe).ch[1].band_type[(w * 16 as c_int + g) as usize] as c_uint)
                            .wrapping_sub(14 as c_int as c_uint),
                    ),
                ) as c_int;
                let mut scale: c_float = (*cpe).ch[0].is_ener[(w * 16 as c_int + g) as usize];
                if (*cpe).is_mask[(w * 16 as c_int + g) as usize] == 0 {
                    start += *((*ics).swb_sizes).offset(g as isize) as c_int;
                } else {
                    if (*cpe).ms_mask[(w * 16 as c_int + g) as usize] != 0 {
                        p *= -(1 as c_int);
                    }
                    i = 0 as c_int;
                    while i < *((*ics).swb_sizes).offset(g as isize) as c_int {
                        let mut sum: c_float = ((*cpe).ch[0].coeffs[(start + i) as usize]
                            + p as c_float * (*cpe).ch[1].coeffs[(start + i) as usize])
                            * scale;
                        (*cpe).ch[0].coeffs[(start + i) as usize] = sum;
                        (*cpe).ch[1].coeffs[(start + i) as usize] = 0.0f32;
                        i += 1;
                        i;
                    }
                    start += *((*ics).swb_sizes).offset(g as isize) as c_int;
                }
                g += 1;
                g;
            }
            w2 += 1;
            w2;
        }
        w += (*ics).group_len[w as usize] as c_int;
    }
}
unsafe extern "C" fn apply_mid_side_stereo(mut cpe: *mut ChannelElement) {
    let mut w: c_int = 0;
    let mut w2: c_int = 0;
    let mut g: c_int = 0;
    let mut i: c_int = 0;
    let mut ics: *mut IndividualChannelStream =
        &mut (*((*cpe).ch).as_mut_ptr().offset(0 as c_int as isize)).ics;
    if (*cpe).common_window == 0 {
        return;
    }
    w = 0 as c_int;
    while w < (*ics).num_windows {
        w2 = 0 as c_int;
        while w2 < (*ics).group_len[w as usize] as c_int {
            let mut start: c_int = (w + w2) * 128 as c_int;
            g = 0 as c_int;
            while g < (*ics).num_swb {
                if (*cpe).ms_mask[(w * 16 as c_int + g) as usize] == 0
                    || (*cpe).is_mask[(w * 16 as c_int + g) as usize] as c_int != 0
                    || (*cpe).ch[0].band_type[(w * 16 as c_int + g) as usize] as c_uint
                        >= NOISE_BT as c_int as c_uint
                    || (*cpe).ch[1].band_type[(w * 16 as c_int + g) as usize] as c_uint
                        >= NOISE_BT as c_int as c_uint
                {
                    start += *((*ics).swb_sizes).offset(g as isize) as c_int;
                } else {
                    i = 0 as c_int;
                    while i < *((*ics).swb_sizes).offset(g as isize) as c_int {
                        let mut L: c_float = ((*cpe).ch[0].coeffs[(start + i) as usize]
                            + (*cpe).ch[1].coeffs[(start + i) as usize])
                            * 0.5f32;
                        let mut R: c_float = L - (*cpe).ch[1].coeffs[(start + i) as usize];
                        (*cpe).ch[0].coeffs[(start + i) as usize] = L;
                        (*cpe).ch[1].coeffs[(start + i) as usize] = R;
                        i += 1;
                        i;
                    }
                    start += *((*ics).swb_sizes).offset(g as isize) as c_int;
                }
                g += 1;
                g;
            }
            w2 += 1;
            w2;
        }
        w += (*ics).group_len[w as usize] as c_int;
    }
}
unsafe extern "C" fn encode_band_info(
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
) {
    let mut w: c_int = 0;
    set_special_band_scalefactors(s, sce);
    w = 0 as c_int;
    while w < (*sce).ics.num_windows {
        encode_window_bands_info(
            s,
            sce,
            w,
            (*sce).ics.group_len[w as usize] as c_int,
            (*s).lambda,
        );
        w += (*sce).ics.group_len[w as usize] as c_int;
    }
}
unsafe extern "C" fn encode_scale_factors(
    mut _avctx: *mut AVCodecContext,
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
) {
    let mut diff: c_int = 0;
    let mut off_sf: c_int = (*sce).sf_idx[0];
    let mut off_pns: c_int = (*sce).sf_idx[0] - 90 as c_int;
    let mut off_is: c_int = 0 as c_int;
    let mut noise_flag: c_int = 1 as c_int;
    let mut i: c_int = 0;
    let mut w: c_int = 0;
    w = 0 as c_int;
    while w < (*sce).ics.num_windows {
        let mut current_block_19: u64;
        i = 0 as c_int;
        while i < (*sce).ics.max_sfb as c_int {
            if (*sce).zeroes[(w * 16 as c_int + i) as usize] == 0 {
                if (*sce).band_type[(w * 16 as c_int + i) as usize] as c_uint
                    == NOISE_BT as c_int as c_uint
                {
                    diff = (*sce).sf_idx[(w * 16 as c_int + i) as usize] - off_pns;
                    off_pns = (*sce).sf_idx[(w * 16 as c_int + i) as usize];
                    let fresh1 = noise_flag;
                    noise_flag -= 1;
                    if fresh1 > 0 as c_int {
                        put_bits(&mut (*s).pb, 9 as c_int, (diff + 256 as c_int) as BitBuf);
                        current_block_19 = 10680521327981672866;
                    } else {
                        current_block_19 = 7976072742316086414;
                    }
                } else {
                    if (*sce).band_type[(w * 16 as c_int + i) as usize] as c_uint
                        == INTENSITY_BT as c_int as c_uint
                        || (*sce).band_type[(w * 16 as c_int + i) as usize] as c_uint
                            == INTENSITY_BT2 as c_int as c_uint
                    {
                        diff = (*sce).sf_idx[(w * 16 as c_int + i) as usize] - off_is;
                        off_is = (*sce).sf_idx[(w * 16 as c_int + i) as usize];
                    } else {
                        diff = (*sce).sf_idx[(w * 16 as c_int + i) as usize] - off_sf;
                        off_sf = (*sce).sf_idx[(w * 16 as c_int + i) as usize];
                    }
                    current_block_19 = 7976072742316086414;
                }
                match current_block_19 {
                    10680521327981672866 => {}
                    _ => {
                        diff += 60 as c_int;
                        assert!(diff >= 0 as c_int && diff <= 120 as c_int);
                        put_bits(
                            &mut (*s).pb,
                            ff_aac_scalefactor_bits[diff as usize] as c_int,
                            ff_aac_scalefactor_code[diff as usize],
                        );
                    }
                }
            }
            i += 1;
            i;
        }
        w += (*sce).ics.group_len[w as usize] as c_int;
    }
}
unsafe extern "C" fn encode_pulses(mut s: *mut AACEncContext, mut pulse: *mut Pulse) {
    let mut i: c_int = 0;
    put_bits(
        &mut (*s).pb,
        1 as c_int,
        ((*pulse).num_pulse != 0) as c_int as BitBuf,
    );
    if (*pulse).num_pulse == 0 {
        return;
    }
    put_bits(
        &mut (*s).pb,
        2 as c_int,
        ((*pulse).num_pulse - 1 as c_int) as BitBuf,
    );
    put_bits(&mut (*s).pb, 6 as c_int, (*pulse).start as BitBuf);
    i = 0 as c_int;
    while i < (*pulse).num_pulse {
        put_bits(&mut (*s).pb, 5 as c_int, (*pulse).pos[i as usize] as BitBuf);
        put_bits(&mut (*s).pb, 4 as c_int, (*pulse).amp[i as usize] as BitBuf);
        i += 1;
        i;
    }
}
unsafe extern "C" fn encode_spectral_coeffs(
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
) {
    let mut start: c_int = 0;
    let mut i: c_int = 0;
    let mut w: c_int = 0;
    let mut w2: c_int = 0;
    w = 0 as c_int;
    while w < (*sce).ics.num_windows {
        start = 0 as c_int;
        i = 0 as c_int;
        while i < (*sce).ics.max_sfb as c_int {
            if (*sce).zeroes[(w * 16 as c_int + i) as usize] != 0 {
                start += *((*sce).ics.swb_sizes).offset(i as isize) as c_int;
            } else {
                w2 = w;
                while w2 < w + (*sce).ics.group_len[w as usize] as c_int {
                    quantize_and_encode_band(
                        s,
                        &mut (*s).pb,
                        &mut *((*sce).coeffs)
                            .as_mut_ptr()
                            .offset((start + w2 * 128 as c_int) as isize),
                        ptr::null_mut::<c_float>(),
                        *((*sce).ics.swb_sizes).offset(i as isize) as c_int,
                        (*sce).sf_idx[(w * 16 as c_int + i) as usize],
                        (*sce).band_type[(w * 16 as c_int + i) as usize] as c_int,
                        (*s).lambda,
                        (*sce).ics.window_clipping[w as usize] as c_int,
                    );
                    w2 += 1;
                    w2;
                }
                start += *((*sce).ics.swb_sizes).offset(i as isize) as c_int;
            }
            i += 1;
            i;
        }
        w += (*sce).ics.group_len[w as usize] as c_int;
    }
}
unsafe extern "C" fn avoid_clipping(
    mut _s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
) {
    let mut start: c_int = 0;
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut w: c_int = 0;
    if (*sce).ics.clip_avoidance_factor < 1.0f32 {
        w = 0 as c_int;
        while w < (*sce).ics.num_windows {
            start = 0 as c_int;
            i = 0 as c_int;
            while i < (*sce).ics.max_sfb as c_int {
                let mut swb_coeffs: *mut c_float = &mut *((*sce).coeffs)
                    .as_mut_ptr()
                    .offset((start + w * 128 as c_int) as isize)
                    as *mut c_float;
                j = 0 as c_int;
                while j < *((*sce).ics.swb_sizes).offset(i as isize) as c_int {
                    *swb_coeffs.offset(j as isize) *= (*sce).ics.clip_avoidance_factor;
                    j += 1;
                    j;
                }
                start += *((*sce).ics.swb_sizes).offset(i as isize) as c_int;
                i += 1;
                i;
            }
            w += 1;
            w;
        }
    }
}
unsafe extern "C" fn encode_individual_channel(
    mut avctx: *mut AVCodecContext,
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
    mut common_window: c_int,
) -> c_int {
    put_bits(&mut (*s).pb, 8 as c_int, (*sce).sf_idx[0] as BitBuf);
    if common_window == 0 {
        put_ics_info(s, &mut (*sce).ics);
        encode_main_pred(s, sce);
        encode_ltp_info(s, sce, 0 as c_int);
    }
    encode_band_info(s, sce);
    encode_scale_factors(avctx, s, sce);
    encode_pulses(s, &mut (*sce).pulse);
    put_bits(
        &mut (*s).pb,
        1 as c_int,
        ((*sce).tns.present != 0) as c_int as BitBuf,
    );
    encode_tns_info(s, sce);
    put_bits(&mut (*s).pb, 1 as c_int, 0 as c_int as BitBuf);
    encode_spectral_coeffs(s, sce);
    0 as c_int
}
unsafe fn put_bitstream_info(mut s: *mut AACEncContext, mut name: &CStr) {
    let mut i: c_int = 0;
    let mut namelen: c_int = 0;
    let mut padbits: c_int = 0;
    namelen = name.to_bytes().len().wrapping_add(2) as c_int;
    put_bits(&mut (*s).pb, 3 as c_int, TYPE_FIL as c_int as BitBuf);
    put_bits(
        &mut (*s).pb,
        4 as c_int,
        (if namelen > 15 as c_int {
            15 as c_int
        } else {
            namelen
        }) as BitBuf,
    );
    if namelen >= 15 as c_int {
        put_bits(&mut (*s).pb, 8 as c_int, (namelen - 14 as c_int) as BitBuf);
    }
    put_bits(&mut (*s).pb, 4 as c_int, 0 as c_int as BitBuf);
    padbits = -put_bits_count(&mut (*s).pb) & 7 as c_int;
    align_put_bits(&mut (*s).pb);
    i = 0 as c_int;
    while i < namelen - 2 as c_int {
        put_bits(
            &mut (*s).pb,
            8 as c_int,
            name.to_bytes()[i as usize] as BitBuf,
        );
        i += 1;
        i;
    }
    put_bits(&mut (*s).pb, 12 as c_int - padbits, 0 as c_int as BitBuf);
}
unsafe extern "C" fn copy_input_samples(mut s: *mut AACEncContext, mut frame: *const AVFrame) {
    let mut ch: c_int = 0;
    let mut end: c_int = 2048 as c_int
        + (if !frame.is_null() {
            (*frame).nb_samples
        } else {
            0 as c_int
        });
    let mut channel_map: *const c_uchar = (*s).reorder_map;
    ch = 0 as c_int;
    while ch < (*s).channels {
        let mut planar_samples = &mut (*s).planar_samples[ch as usize];

        planar_samples.copy_within(2048..2048 + 1024, 1024);
        if !frame.is_null() {
            ptr::copy_nonoverlapping(
                *((*frame).extended_data).offset(*channel_map.offset(ch as isize) as isize)
                    as *mut c_float,
                planar_samples[2048..][..(*frame).nb_samples as usize].as_mut_ptr(),
                (*frame).nb_samples as usize,
            );
        }

        planar_samples[end as usize..].fill(0.);
        ch += 1;
        ch;
    }
}
unsafe extern "C" fn aac_encode_frame(
    mut avctx: *mut AVCodecContext,
    mut avpkt: *mut AVPacket,
    mut frame: *const AVFrame,
    mut got_packet_ptr: *mut c_int,
) -> c_int {
    let mut s: *mut AACEncContext = (*((*avctx).priv_data as *mut PrivData)).ctx;
    // let mut samples: *mut *mut c_float = ((*s).planar_samples).as_mut_ptr();
    let mut samples2: *mut c_float = ptr::null_mut::<c_float>();
    let mut la: *mut c_float = ptr::null_mut::<c_float>();
    let mut overlap: *mut c_float = ptr::null_mut::<c_float>();
    let mut cpe: *mut ChannelElement = ptr::null_mut::<ChannelElement>();
    let mut sce: *mut SingleChannelElement = ptr::null_mut::<SingleChannelElement>();
    let mut ics: *mut IndividualChannelStream = ptr::null_mut::<IndividualChannelStream>();
    let mut i: c_int = 0;
    let mut its: c_int = 0;
    let mut ch: c_int = 0;
    let mut w: c_int = 0;
    let mut chans: c_int = 0;
    let mut tag: c_int = 0;
    let mut start_ch: c_int = 0;
    let mut ret: c_int = 0;
    let mut frame_bits: c_int = 0;
    let mut target_bits: c_int = 0;
    let mut rate_bits: c_int = 0;
    let mut too_many_bits: c_int = 0;
    let mut too_few_bits: c_int = 0;
    let mut ms_mode: c_int = 0 as c_int;
    let mut is_mode: c_int = 0 as c_int;
    let mut tns_mode: c_int = 0 as c_int;
    let mut pred_mode: c_int = 0 as c_int;
    let mut chan_el_counter: [c_int; 4] = [0; 4];
    let mut windows: [FFPsyWindowInfo; 16] = [FFPsyWindowInfo {
        window_type: [0; 3],
        window_shape: 0,
        num_windows: 0,
        grouping: [0; 8],
        clipping: [0.; 8],
        window_sizes: ptr::null_mut::<c_int>(),
    }; 16];
    if !frame.is_null() {
        ret = ff_af_queue_add(&mut (*s).afq, frame);
        if ret < 0 as c_int {
            return ret;
        }
    } else if (*s).afq.remaining_samples == 0
        || (*s).afq.frame_alloc == 0 && (*s).afq.frame_count == 0
    {
        return 0 as c_int;
    }
    copy_input_samples(s, frame);
    if !((*s).psypp).is_null() {
        ff_psy_preprocess((*s).psypp, &mut (*s).planar_samples);
    }
    if (*avctx).frame_num == 0 {
        return 0 as c_int;
    }
    start_ch = 0 as c_int;
    i = 0 as c_int;
    while i < *((*s).chan_map).offset(0 as c_int as isize) as c_int {
        let mut wi: *mut FFPsyWindowInfo = windows.as_mut_ptr().offset(start_ch as isize);
        tag = *((*s).chan_map).offset((i + 1 as c_int) as isize) as c_int;
        chans = if tag == TYPE_CPE as c_int {
            2 as c_int
        } else {
            1 as c_int
        };
        cpe = &mut *((*s).cpe).offset(i as isize) as *mut ChannelElement;
        ch = 0 as c_int;
        while ch < chans {
            let mut k: c_int = 0;
            let mut clip_avoidance_factor: c_float = 0.;
            sce = &mut *((*cpe).ch).as_mut_ptr().offset(ch as isize) as *mut SingleChannelElement;
            ics = &mut (*sce).ics;
            (*s).cur_channel = start_ch + ch;
            overlap = &mut *(*s).planar_samples[(*s).cur_channel as usize]
                .as_mut_ptr()
                .offset(0 as c_int as isize) as *mut c_float;
            samples2 = overlap.offset(1024 as c_int as isize);
            la = samples2.offset((448 as c_int + 64 as c_int) as isize);
            if frame.is_null() {
                la = ptr::null_mut::<c_float>();
            }
            if tag == TYPE_LFE as c_int {
                let fresh2 = &mut (*wi.offset(ch as isize)).window_type[1];
                *fresh2 = ONLY_LONG_SEQUENCE as c_int;
                (*wi.offset(ch as isize)).window_type[0] = *fresh2;
                (*wi.offset(ch as isize)).window_shape = 0 as c_int;
                (*wi.offset(ch as isize)).num_windows = 1 as c_int;
                (*wi.offset(ch as isize)).grouping[0] = 1 as c_int;
                (*wi.offset(ch as isize)).clipping[0] = 0 as c_int as c_float;
                (*ics).num_swb = if (*s).samplerate_index >= 8 as c_int {
                    1 as c_int
                } else {
                    3 as c_int
                };
            } else {
                *wi.offset(ch as isize) = ((*(*s).psy.model).window)
                    .expect("non-null function pointer")(
                    &mut (*s).psy,
                    samples2,
                    la,
                    (*s).cur_channel,
                    (*ics).window_sequence[0] as c_int,
                );
            }
            (*ics).window_sequence[1] = (*ics).window_sequence[0];
            (*ics).window_sequence[0] = (*wi.offset(ch as isize)).window_type[0] as WindowSequence;
            (*ics).use_kb_window[1] = (*ics).use_kb_window[0];
            (*ics).use_kb_window[0] = (*wi.offset(ch as isize)).window_shape as c_uchar;
            (*ics).num_windows = (*wi.offset(ch as isize)).num_windows;
            (*ics).swb_sizes =
                *((*s).psy.bands).offset(((*ics).num_windows == 8 as c_int) as c_int as isize);
            (*ics).num_swb = if tag == TYPE_LFE as c_int {
                (*ics).num_swb
            } else {
                *((*s).psy.num_bands).offset(((*ics).num_windows == 8 as c_int) as c_int as isize)
            };
            (*ics).max_sfb = (if (*ics).max_sfb as c_int > (*ics).num_swb {
                (*ics).num_swb
            } else {
                (*ics).max_sfb as c_int
            }) as c_uchar;
            (*ics).swb_offset =
                if (*wi.offset(ch as isize)).window_type[0] == EIGHT_SHORT_SEQUENCE as c_int {
                    ff_swb_offset_128[(*s).samplerate_index as usize].as_ptr()
                } else {
                    ff_swb_offset_1024[(*s).samplerate_index as usize].as_ptr()
                };
            (*ics).tns_max_bands =
                if (*wi.offset(ch as isize)).window_type[0] == EIGHT_SHORT_SEQUENCE as c_int {
                    ff_tns_max_bands_128[(*s).samplerate_index as usize] as c_int
                } else {
                    ff_tns_max_bands_1024[(*s).samplerate_index as usize] as c_int
                };
            w = 0 as c_int;
            while w < (*ics).num_windows {
                (*ics).group_len[w as usize] =
                    (*wi.offset(ch as isize)).grouping[w as usize] as c_uchar;
                w += 1;
                w;
            }
            clip_avoidance_factor = 0.0f32;
            w = 0 as c_int;
            while w < (*ics).num_windows {
                let mut wbuf: *const c_float = overlap.offset((w * 128 as c_int) as isize);
                let wlen: c_int = 2048 as c_int / (*ics).num_windows;
                let mut max: c_float = 0 as c_int as c_float;
                let mut j: c_int = 0;
                j = 0 as c_int;
                while j < wlen {
                    max = if max > fabsf(*wbuf.offset(j as isize)) {
                        max
                    } else {
                        fabsf(*wbuf.offset(j as isize))
                    };
                    j += 1;
                    j;
                }
                (*wi.offset(ch as isize)).clipping[w as usize] = max;
                w += 1;
                w;
            }
            w = 0 as c_int;
            while w < (*ics).num_windows {
                if (*wi.offset(ch as isize)).clipping[w as usize] > 0.95f32 {
                    (*ics).window_clipping[w as usize] = 1 as c_int as c_uchar;
                    clip_avoidance_factor =
                        if clip_avoidance_factor > (*wi.offset(ch as isize)).clipping[w as usize] {
                            clip_avoidance_factor
                        } else {
                            (*wi.offset(ch as isize)).clipping[w as usize]
                        };
                } else {
                    (*ics).window_clipping[w as usize] = 0 as c_int as c_uchar;
                }
                w += 1;
                w;
            }
            if clip_avoidance_factor > 0.95f32 {
                (*ics).clip_avoidance_factor = 0.95f32 / clip_avoidance_factor;
            } else {
                (*ics).clip_avoidance_factor = 1.0f32;
            }
            apply_window_and_mdct(s, sce, overlap);
            if (*s).options.ltp != 0 {
                update_ltp(s, sce);
                APPLY_WINDOW[(*sce).ics.window_sequence[0] as usize](
                    (*s).fdsp,
                    sce,
                    &mut *((*sce).ltp_state).as_mut_ptr().offset(0 as c_int as isize),
                );
                ((*s).mdct1024_fn).expect("non-null function pointer")(
                    (*s).mdct1024,
                    ((*sce).lcoeffs).as_mut_ptr() as *mut c_void,
                    ((*sce).ret_buf).as_mut_ptr() as *mut c_void,
                    size_of::<c_float>() as c_ulong as ptrdiff_t,
                );
            }
            k = 0 as c_int;
            while k < 1024 as c_int {
                if !(fabs((*cpe).ch[ch as usize].coeffs[k as usize] as c_double) < 1E16f64) {
                    av_log(
                        avctx as *mut c_void,
                        16 as c_int,
                        b"Input contains (near) NaN/+-Inf\n\0" as *const u8 as *const c_char,
                    );
                    return -(22 as c_int);
                }
                k += 1;
                k;
            }
            avoid_clipping(s, sce);
            ch += 1;
            ch;
        }
        start_ch += chans;
        i += 1;
        i;
    }
    ret = ff_alloc_packet(avctx, avpkt, (8192 as c_int * (*s).channels) as c_long);
    if ret < 0 as c_int {
        return ret;
    }
    its = 0 as c_int;
    frame_bits = its;
    loop {
        init_put_bits(&mut (*s).pb, (*avpkt).data, (*avpkt).size);
        if (*avctx).frame_num & 0xff as c_int as c_long == 1 as c_int as c_long
            && (*avctx).flags & (1 as c_int) << 23 as c_int == 0
        {
            put_bitstream_info(s, c"Lavc60.33.100");
        }
        start_ch = 0 as c_int;
        target_bits = 0 as c_int;
        chan_el_counter.fill(0);
        i = 0 as c_int;
        while i < *((*s).chan_map).offset(0 as c_int as isize) as c_int {
            let mut wi_0: *mut FFPsyWindowInfo = windows.as_mut_ptr().offset(start_ch as isize);
            let mut coeffs: [*const c_float; 2] = [ptr::null::<c_float>(); 2];
            tag = *((*s).chan_map).offset((i + 1 as c_int) as isize) as c_int;
            chans = if tag == TYPE_CPE as c_int {
                2 as c_int
            } else {
                1 as c_int
            };
            cpe = &mut *((*s).cpe).offset(i as isize) as *mut ChannelElement;
            (*cpe).common_window = 0 as c_int;
            (*cpe).is_mask.fill(0);
            (*cpe).ms_mask.fill(0);
            put_bits(&mut (*s).pb, 3 as c_int, tag as BitBuf);
            let fresh3 = chan_el_counter[tag as usize];
            chan_el_counter[tag as usize] += 1;
            put_bits(&mut (*s).pb, 4 as c_int, fresh3 as BitBuf);
            ch = 0 as c_int;
            while ch < chans {
                sce =
                    &mut *((*cpe).ch).as_mut_ptr().offset(ch as isize) as *mut SingleChannelElement;
                coeffs[ch as usize] = ((*sce).coeffs).as_mut_ptr();
                (*sce).ics.predictor_present = 0 as c_int;
                (*sce).ics.ltp.present = 0 as c_int as c_schar;
                (*sce).ics.ltp.used.fill(0);
                (*sce).ics.prediction_used.fill(0);
                (*sce).tns = TemporalNoiseShaping::default();
                w = 0 as c_int;
                while w < 128 as c_int {
                    if (*sce).band_type[w as usize] as c_uint > RESERVED_BT as c_int as c_uint {
                        (*sce).band_type[w as usize] = ZERO_BT;
                    }
                    w += 1;
                    w;
                }
                ch += 1;
                ch;
            }
            (*s).psy.bitres.alloc = -(1 as c_int);
            (*s).psy.bitres.bits = (*s).last_frame_pb_count / (*s).channels;
            ((*(*s).psy.model).analyze).expect("non-null function pointer")(
                &mut (*s).psy,
                start_ch,
                coeffs.as_mut_ptr(),
                wi_0,
            );
            if (*s).psy.bitres.alloc > 0 as c_int {
                target_bits = (target_bits as c_float
                    + (*s).psy.bitres.alloc as c_float
                        * ((*s).lambda
                            / (if (*avctx).global_quality != 0 {
                                (*avctx).global_quality
                            } else {
                                120 as c_int
                            }) as c_float)) as c_int;
                (*s).psy.bitres.alloc /= chans;
            }
            (*s).cur_type = tag as RawDataBlockType;
            ch = 0 as c_int;
            while ch < chans {
                (*s).cur_channel = start_ch + ch;
                if (*s).options.pns != 0 {
                    pns::mark(s, avctx, &mut *((*cpe).ch).as_mut_ptr().offset(ch as isize));
                }
                (*(*s).coder).search_for_quantizers(
                    avctx,
                    s,
                    &mut *((*cpe).ch).as_mut_ptr().offset(ch as isize),
                    (*s).lambda,
                );
                ch += 1;
                ch;
            }
            if chans > 1 as c_int
                && (*wi_0.offset(0 as c_int as isize)).window_type[0]
                    == (*wi_0.offset(1 as c_int as isize)).window_type[0]
                && (*wi_0.offset(0 as c_int as isize)).window_shape
                    == (*wi_0.offset(1 as c_int as isize)).window_shape
            {
                (*cpe).common_window = 1 as c_int;
                w = 0 as c_int;
                while w < (*wi_0.offset(0 as c_int as isize)).num_windows {
                    if (*wi_0.offset(0 as c_int as isize)).grouping[w as usize]
                        != (*wi_0.offset(1 as c_int as isize)).grouping[w as usize]
                    {
                        (*cpe).common_window = 0 as c_int;
                        break;
                    } else {
                        w += 1;
                        w;
                    }
                }
            }
            ch = 0 as c_int;
            while ch < chans {
                sce =
                    &mut *((*cpe).ch).as_mut_ptr().offset(ch as isize) as *mut SingleChannelElement;
                (*s).cur_channel = start_ch + ch;
                if (*s).options.tns != 0 {
                    search_for_tns(s, sce);
                }
                if (*s).options.tns != 0 {
                    apply_tns(s, sce);
                }
                if (*sce).tns.present != 0 {
                    tns_mode = 1 as c_int;
                }
                if (*s).options.pns != 0 {
                    pns::search(s, avctx, sce);
                }
                ch += 1;
                ch;
            }
            (*s).cur_channel = start_ch;
            if (*s).options.intensity_stereo != 0 {
                search_for_is(s, avctx, cpe);
                if (*cpe).is_mode != 0 {
                    is_mode = 1 as c_int;
                }
                apply_intensity_stereo(cpe);
            }
            if (*s).options.pred != 0 {
                ch = 0 as c_int;
                while ch < chans {
                    sce = &mut *((*cpe).ch).as_mut_ptr().offset(ch as isize)
                        as *mut SingleChannelElement;
                    (*s).cur_channel = start_ch + ch;
                    if (*s).options.pred != 0 {
                        search_for_pred(s, sce);
                    }
                    if (*cpe).ch[ch as usize].ics.predictor_present != 0 {
                        pred_mode = 1 as c_int;
                    }
                    ch += 1;
                    ch;
                }

                adjust_common_pred(s, cpe);

                ch = 0 as c_int;
                while ch < chans {
                    sce = &mut *((*cpe).ch).as_mut_ptr().offset(ch as isize)
                        as *mut SingleChannelElement;
                    (*s).cur_channel = start_ch + ch;
                    if (*s).options.pred != 0 {
                        apply_main_pred(s, sce);
                    }
                    ch += 1;
                    ch;
                }
                (*s).cur_channel = start_ch;
            }
            if (*s).options.mid_side != 0 {
                if (*s).options.mid_side == -(1 as c_int) {
                    search_for_ms(s, cpe);
                } else if (*cpe).common_window != 0 {
                    (*cpe).ms_mask.fill(1);
                }
                apply_mid_side_stereo(cpe);
            }
            adjust_frame_information(cpe, chans);
            if (*s).options.ltp != 0 {
                ch = 0 as c_int;
                while ch < chans {
                    sce = &mut *((*cpe).ch).as_mut_ptr().offset(ch as isize)
                        as *mut SingleChannelElement;
                    (*s).cur_channel = start_ch + ch;

                    search_for_ltp(s, sce, (*cpe).common_window);

                    if (*sce).ics.ltp.present != 0 {
                        pred_mode = 1 as c_int;
                    }
                    ch += 1;
                    ch;
                }
                (*s).cur_channel = start_ch;

                adjust_common_ltp(s, cpe);
            }
            if chans == 2 as c_int {
                put_bits(&mut (*s).pb, 1 as c_int, (*cpe).common_window as BitBuf);
                if (*cpe).common_window != 0 {
                    put_ics_info(
                        s,
                        &mut (*((*cpe).ch).as_mut_ptr().offset(0 as c_int as isize)).ics,
                    );

                    encode_main_pred(
                        s,
                        &mut *((*cpe).ch).as_mut_ptr().offset(0 as c_int as isize),
                    );

                    encode_ltp_info(
                        s,
                        &mut *((*cpe).ch).as_mut_ptr().offset(0 as c_int as isize),
                        1 as c_int,
                    );

                    encode_ms_info(&mut (*s).pb, cpe);
                    if (*cpe).ms_mode != 0 {
                        ms_mode = 1 as c_int;
                    }
                }
            }
            ch = 0 as c_int;
            while ch < chans {
                (*s).cur_channel = start_ch + ch;
                encode_individual_channel(
                    avctx,
                    s,
                    &mut *((*cpe).ch).as_mut_ptr().offset(ch as isize),
                    (*cpe).common_window,
                );
                ch += 1;
                ch;
            }
            start_ch += chans;
            i += 1;
            i;
        }
        if (*avctx).flags & AV_CODEC_FLAG_QSCALE != 0 {
            break;
        }
        frame_bits = put_bits_count(&mut (*s).pb);
        rate_bits =
            ((*avctx).bit_rate * 1024 as c_int as c_long / (*avctx).sample_rate as c_long) as c_int;
        rate_bits = if rate_bits > 6144 as c_int * (*s).channels - 3 as c_int {
            6144 as c_int * (*s).channels - 3 as c_int
        } else {
            rate_bits
        };
        too_many_bits = if target_bits > rate_bits {
            target_bits
        } else {
            rate_bits
        };
        too_many_bits = if too_many_bits > 6144 as c_int * (*s).channels - 3 as c_int {
            6144 as c_int * (*s).channels - 3 as c_int
        } else {
            too_many_bits
        };
        too_few_bits = if (if rate_bits - rate_bits / 4 as c_int > target_bits {
            rate_bits - rate_bits / 4 as c_int
        } else {
            target_bits
        }) > too_many_bits
        {
            too_many_bits
        } else if rate_bits - rate_bits / 4 as c_int > target_bits {
            rate_bits - rate_bits / 4 as c_int
        } else {
            target_bits
        };
        if (*avctx).bit_rate_tolerance == 0 as c_int {
            if rate_bits < frame_bits {
                let mut ratio: c_float = rate_bits as c_float / frame_bits as c_float;
                (*s).lambda *= if 0.9f32 > ratio { ratio } else { 0.9f32 };
            } else {
                (*s).lambda = (if (*avctx).global_quality > 0 as c_int {
                    (*avctx).global_quality
                } else {
                    120 as c_int
                }) as c_float;
                break;
            }
        } else {
            too_few_bits = too_few_bits - too_few_bits / 8 as c_int;
            too_many_bits = too_many_bits + too_many_bits / 2 as c_int;
            if !(its == 0 as c_int
                || its < 5 as c_int && (frame_bits < too_few_bits || frame_bits > too_many_bits)
                || frame_bits >= 6144 as c_int * (*s).channels - 3 as c_int)
            {
                break;
            }
            let mut ratio_0: c_float = rate_bits as c_float / frame_bits as c_float;
            if frame_bits >= too_few_bits && frame_bits <= too_many_bits {
                ratio_0 = sqrtf(sqrtf(ratio_0));
                ratio_0 = av_clipf_c(ratio_0, 0.9f32, 1.1f32);
            } else {
                ratio_0 = sqrtf(ratio_0);
            }
            (*s).lambda = av_clipf_c((*s).lambda * ratio_0, 1.192_092_9e-7_f32, 65536.0f32);
            if ratio_0 > 0.9f32 && ratio_0 < 1.1f32 {
                break;
            }
            if is_mode != 0 || ms_mode != 0 || tns_mode != 0 || pred_mode != 0 {
                i = 0 as c_int;
                while i < *((*s).chan_map).offset(0 as c_int as isize) as c_int {
                    chans = if tag == TYPE_CPE as c_int {
                        2 as c_int
                    } else {
                        1 as c_int
                    };
                    cpe = &mut *((*s).cpe).offset(i as isize) as *mut ChannelElement;
                    ch = 0 as c_int;
                    while ch < chans {
                        (*cpe).ch[ch as usize].coeffs = (*cpe).ch[ch as usize].pcoeffs;
                        ch += 1;
                        ch;
                    }
                    i += 1;
                    i;
                }
            }
            its += 1;
            its;
        }
    }
    if (*s).options.ltp != 0 {
        ltp_insert_new_frame(s);
    }
    put_bits(&mut (*s).pb, 3 as c_int, TYPE_END as c_int as BitBuf);
    flush_put_bits(&mut (*s).pb);
    (*s).last_frame_pb_count = put_bits_count(&mut (*s).pb);
    (*avpkt).size = put_bytes_output(&mut (*s).pb);
    (*s).lambda_sum += (*s).lambda;
    (*s).lambda_count += 1;
    (*s).lambda_count;
    ff_af_queue_remove(
        &mut (*s).afq,
        (*avctx).frame_size,
        &mut (*avpkt).pts,
        &mut (*avpkt).duration,
    );
    *got_packet_ptr = 1 as c_int;
    0 as c_int
}

#[cold]
unsafe extern "C" fn aac_encode_end(mut avctx: *mut AVCodecContext) -> c_int {
    let mut s: *mut AACEncContext = (*((*avctx).priv_data as *mut PrivData)).ctx;
    av_log(
        avctx as *mut c_void,
        32 as c_int,
        b"Qavg: %.3f\n\0" as *const u8 as *const c_char,
        (if (*s).lambda_count != 0 {
            (*s).lambda_sum / (*s).lambda_count as c_float
        } else {
            ::core::f32::NAN
        }) as c_double,
    );
    av_tx_uninit(&mut (*s).mdct1024);
    av_tx_uninit(&mut (*s).mdct128);
    ff_psy_end(&mut (*s).psy);
    ff_lpc_end(&mut (*s).lpc);
    if !((*s).psypp).is_null() {
        ff_psy_preprocess_end((*s).psypp);
    }
    av_freep(&mut (*s).cpe as *mut *mut ChannelElement as *mut c_void);
    av_freep(&mut (*s).fdsp as *mut *mut AVFloatDSPContext as *mut c_void);
    ff_af_queue_close(&mut (*s).afq);
    drop(Box::from_raw(s));
    0 as c_int
}

#[cold]
unsafe extern "C" fn alloc_buffers(
    mut _avctx: *mut AVCodecContext,
    mut s: *mut AACEncContext,
) -> c_int {
    (*s).cpe = av_calloc(
        *((*s).chan_map).offset(0 as c_int as isize) as c_ulong,
        size_of::<ChannelElement>() as c_ulong,
    ) as *mut ChannelElement;
    if ((*s).cpe).is_null() {
        return -(12 as c_int);
    }
    0 as c_int
}

#[cold]
unsafe extern "C" fn aac_encode_init(mut avctx: *mut AVCodecContext) -> c_int {
    let priv_data = (*avctx).priv_data as *mut PrivData;
    debug_assert!((*priv_data).ctx.is_null());

    let needs_pce = if channel_layout::NORMAL_LAYOUTS
        .iter()
        .any(|layout| av_channel_layout_compare(&(*avctx).ch_layout, layout) == 0)
    {
        (*priv_data).options.pce
    } else {
        1
    };

    let samplerate_index = ff_mpeg4audio_sample_rates
        .iter()
        .find_position(|&&sample_rate| (*avctx).sample_rate == sample_rate)
        .map(|(i, _)| i)
        .filter(|&i| i < ff_aac_swb_size_1024.len() && i < ff_aac_swb_size_128.len())
        .unwrap_or_else(|| panic!("Unsupported sample rate {}", (*avctx).sample_rate))
        as c_int;

    let ctx = Box::new(AACEncContext {
        av_class: (*priv_data).class,
        options: (*priv_data).options,
        pb: PutBitContext::zero(),
        mdct1024: null_mut(),
        mdct1024_fn: None,
        mdct128: null_mut(),
        mdct128_fn: None,
        fdsp: null_mut(),
        pce: None,
        planar_samples: vec![[0.; _]; (*avctx).ch_layout.nb_channels as usize].into_boxed_slice(),
        profile: 0,
        needs_pce,
        lpc: LPCContext::zero(),
        samplerate_index,
        channels: (*avctx).ch_layout.nb_channels,
        reorder_map: null(),
        chan_map: null(),
        cpe: null_mut(),
        psy: FFPsyContext::zero(),
        psypp: null_mut(),
        coder: match (*priv_data).options.coder as c_uint {
            AAC_CODER_ANMR => &coder::Anmr,
            AAC_CODER_TWOLOOP => &coder::TwoLoop,
            AAC_CODER_FAST => &coder::Fast,
            _ => panic!("Unknown coder"),
        },
        cur_channel: 0,
        random_state: 0x1f2e3d4c,
        lambda: if (*avctx).global_quality > 0 as c_int {
            (*avctx).global_quality as c_float
        } else {
            120.
        },
        last_frame_pb_count: 0,
        lambda_sum: 0.,
        lambda_count: 0,
        cur_type: 0,
        afq: AudioFrameQueue::zero(),
        qcoefs: [0; _],
        scoefs: [0.; _],
        quantize_band_cost_cache_generation: 0,
        quantize_band_cost_cache: [[AACQuantizeBandCostCacheEntry::default(); _]; _],
    });
    (*priv_data).ctx = Box::into_raw(ctx);

    let mut s: *mut AACEncContext = (*priv_data).ctx;

    let mut i: c_int = 0;
    let mut ret: c_int = 0 as c_int;
    let mut sizes: [*const c_uchar; 2] = [ptr::null::<c_uchar>(); 2];
    let mut grouping: [c_uchar; 16] = [0; 16];
    let mut lengths: [c_int; 2] = [0; 2];
    (*avctx).frame_size = 1024 as c_int;
    (*avctx).initial_padding = 1024 as c_int;

    if (*s).needs_pce != 0 {
        let mut buf: [c_char; 64] = [0; 64];
        av_channel_layout_describe(
            &mut (*avctx).ch_layout,
            buf.as_mut_ptr(),
            buf.len() as c_ulong,
        );

        let config = pce::CONFIGS
            .iter()
            .find(|config| av_channel_layout_compare(&(*avctx).ch_layout, &config.layout) == 0)
            .unwrap_or_else(|| {
                panic!(
                    "Unsupported channel layout {}",
                    CStr::from_ptr(buf.as_ptr()).to_string_lossy()
                )
            });

        av_log(
            avctx as *mut c_void,
            32 as c_int,
            b"Using a PCE to encode channel layout \"%s\"\n\0" as *const u8 as *const c_char,
            buf.as_mut_ptr(),
        );

        (*s).pce = Some(*config);
        (*s).reorder_map = (config.reorder_map).as_ptr();
        (*s).chan_map = (config.config_map).as_ptr();
    } else {
        (*s).reorder_map =
            (channel_layout::REORDER_MAPS[((*s).channels - 1 as c_int) as usize]).as_ptr();
        (*s).chan_map = (channel_layout::CONFIGS[((*s).channels - 1 as c_int) as usize]).as_ptr();
    }

    if (*avctx).bit_rate == 0 {
        i = 1 as c_int;
        while i <= *((*s).chan_map).offset(0 as c_int as isize) as c_int {
            (*avctx).bit_rate +=
                (if *((*s).chan_map).offset(i as isize) as c_int == TYPE_CPE as c_int {
                    128000 as c_int
                } else if *((*s).chan_map).offset(i as isize) as c_int == TYPE_LFE as c_int {
                    16000 as c_int
                } else {
                    69000 as c_int
                }) as c_long;
            i += 1;
            i;
        }
    }

    if 1024.0f64 * (*avctx).bit_rate as c_double / (*avctx).sample_rate as c_double
        > (6144 as c_int * (*s).channels) as c_double
    {
        av_log(
            avctx as *mut c_void,
            24 as c_int,
            b"Too many bits %f > %d per frame requested, clamping to max\n\0" as *const u8
                as *const c_char,
            1024.0f64 * (*avctx).bit_rate as c_double / (*avctx).sample_rate as c_double,
            6144 as c_int * (*s).channels,
        );
    }
    (*avctx).bit_rate = (if (6144 as c_int * (*s).channels) as c_double / 1024.0f64
        * (*avctx).sample_rate as c_double
        > (*avctx).bit_rate as c_double
    {
        (*avctx).bit_rate as c_double
    } else {
        (6144 as c_int * (*s).channels) as c_double / 1024.0f64 * (*avctx).sample_rate as c_double
    }) as c_long;
    (*avctx).profile = if (*avctx).profile == -(99 as c_int) {
        1 as c_int
    } else {
        (*avctx).profile
    };
    i = 0 as c_int;
    while (i as c_ulong)
        < (size_of::<[c_int; 4]>() as c_ulong).wrapping_div(size_of::<c_int>() as c_ulong)
    {
        if (*avctx).profile == aacenc_profiles[i as usize] {
            break;
        }
        i += 1;
        i;
    }
    if (*avctx).profile == 128 as c_int {
        (*avctx).profile = 1 as c_int;
        if (*s).options.pred != 0 {
            av_log(
                avctx as *mut c_void,
                16 as c_int,
                b"Main prediction unavailable in the \"mpeg2_aac_low\" profile\n\0" as *const u8
                    as *const c_char,
            );
            return -(22 as c_int);
        }
        if (*s).options.ltp != 0 {
            av_log(
                avctx as *mut c_void,
                16 as c_int,
                b"LTP prediction unavailable in the \"mpeg2_aac_low\" profile\n\0" as *const u8
                    as *const c_char,
            );
            return -(22 as c_int);
        }
        if (*s).options.pns != 0 {
            av_log(
                avctx as *mut c_void,
                24 as c_int,
                b"PNS unavailable in the \"mpeg2_aac_low\" profile, turning off\n\0" as *const u8
                    as *const c_char,
            );
        }
        (*s).options.pns = 0 as c_int;
    } else if (*avctx).profile == 3 as c_int {
        (*s).options.ltp = 1 as c_int;
        if (*s).options.pred != 0 {
            av_log(
                avctx as *mut c_void,
                16 as c_int,
                b"Main prediction unavailable in the \"aac_ltp\" profile\n\0" as *const u8
                    as *const c_char,
            );
            return -(22 as c_int);
        }
    } else if (*avctx).profile == 0 as c_int {
        (*s).options.pred = 1 as c_int;
        if (*s).options.ltp != 0 {
            av_log(
                avctx as *mut c_void,
                16 as c_int,
                b"LTP prediction unavailable in the \"aac_main\" profile\n\0" as *const u8
                    as *const c_char,
            );
            return -(22 as c_int);
        }
    } else if (*s).options.ltp != 0 {
        (*avctx).profile = 3 as c_int;
        av_log(
            avctx as *mut c_void,
            24 as c_int,
            b"Chainging profile to \"aac_ltp\"\n\0" as *const u8 as *const c_char,
        );
        if (*s).options.pred != 0 {
            av_log(
                avctx as *mut c_void,
                16 as c_int,
                b"Main prediction unavailable in the \"aac_ltp\" profile\n\0" as *const u8
                    as *const c_char,
            );
            return -(22 as c_int);
        }
    } else if (*s).options.pred != 0 {
        (*avctx).profile = 0 as c_int;
        av_log(
            avctx as *mut c_void,
            24 as c_int,
            b"Chainging profile to \"aac_main\"\n\0" as *const u8 as *const c_char,
        );
        if (*s).options.ltp != 0 {
            av_log(
                avctx as *mut c_void,
                16 as c_int,
                b"LTP prediction unavailable in the \"aac_main\" profile\n\0" as *const u8
                    as *const c_char,
            );
            return -(22 as c_int);
        }
    }
    (*s).profile = (*avctx).profile;
    if (*s).options.coder == AAC_CODER_ANMR as c_int {
        if (*avctx).strict_std_compliance > -(2 as c_int) {
            panic!("The ANMR coder is considered experimental, add -strict -2 to enable!");
        }
        (*s).options.intensity_stereo = 0 as c_int;
        (*s).options.pns = 0 as c_int;
    }
    if (*s).options.ltp != 0 && (*avctx).strict_std_compliance > -(2 as c_int) {
        panic!("The LPT profile requires experimental compliance, add -strict -2 to enable!");
    }
    if (*s).channels > 3 as c_int {
        (*s).options.mid_side = 0 as c_int;
    }
    ff_aac_float_common_init();
    ret = dsp::init(avctx, s);
    if ret < 0 as c_int {
        return ret;
    }
    ret = alloc_buffers(avctx, s);
    if ret < 0 as c_int {
        return ret;
    }
    ret = put_audio_specific_config(avctx);
    if ret != 0 {
        return ret;
    }
    sizes[0] = *ff_aac_swb_size_1024
        .as_ptr()
        .offset((*s).samplerate_index as isize);
    sizes[1] = *ff_aac_swb_size_128
        .as_ptr()
        .offset((*s).samplerate_index as isize);
    lengths[0] = *ff_aac_num_swb_1024
        .as_ptr()
        .offset((*s).samplerate_index as isize) as c_int;
    lengths[1] = *ff_aac_num_swb_128
        .as_ptr()
        .offset((*s).samplerate_index as isize) as c_int;
    i = 0 as c_int;
    while i < *((*s).chan_map).offset(0 as c_int as isize) as c_int {
        grouping[i as usize] = (*((*s).chan_map).offset((i + 1 as c_int) as isize) as c_int
            == TYPE_CPE as c_int) as c_int as c_uchar;
        i += 1;
        i;
    }
    ret = ff_psy_init(
        &mut (*s).psy,
        avctx,
        2 as c_int,
        sizes.as_mut_ptr(),
        lengths.as_mut_ptr(),
        *((*s).chan_map).offset(0 as c_int as isize) as c_int,
        grouping.as_mut_ptr(),
    );
    if ret < 0 as c_int {
        return ret;
    }
    (*s).psypp = ff_psy_preprocess_init(avctx);
    ff_lpc_init(
        &mut (*s).lpc,
        2 as c_int * (*avctx).frame_size,
        20 as c_int,
        FF_LPC_TYPE_LEVINSON,
    );
    ff_af_queue_init(avctx, &mut (*s).afq);
    0 as c_int
}

const ENCODER_CLASS: AVClass = AVClass {
    class_name: c"AAC encoder".as_ptr(),
    item_name: Some(av_default_item_name),
    option: OPTIONS.as_ptr(),
    version: (58 << 16) | (32 << 8) | 100,
    log_level_offset_offset: 0,
    parent_log_context_offset: 0,
    category: AV_CLASS_CATEGORY_NA,
    get_category: None,
    query_ranges: None,
    child_next: None,
    child_class_iterate: None,
};

const aac_encode_defaults: [FFCodecDefault; 2] =
    [FFCodecDefault::new(c"b", c"0"), FFCodecDefault::null()];

#[no_mangle]
pub static mut ff_aac_encoder: FFCodec = FFCodec {
    p: AVCodec {
        name: c"aac".as_ptr(),
        long_name: c"AAC (Advanced Audio Coding)".as_ptr(),
        type_0: AVMEDIA_TYPE_AUDIO,
        id: AV_CODEC_ID_AAC,
        capabilities: AV_CODEC_FLAG_QSCALE
            | (1 as c_int) << 5 as c_int
            | (1 as c_int) << 6 as c_int,
        max_lowres: 0,
        supported_framerates: ptr::null::<AVRational>(),
        pix_fmts: ptr::null::<AVPixelFormat>(),
        supported_samplerates: ff_mpeg4audio_sample_rates.as_ptr(),
        sample_fmts: [8, -1].as_ptr(),
        channel_layouts: ptr::null::<c_ulong>(),
        priv_class: &ENCODER_CLASS,
        profiles: ptr::null::<AVProfile>(),
        wrapper_name: ptr::null::<c_char>(),
        ch_layouts: ptr::null::<AVChannelLayout>(),
    },
    caps_internal_cb_type: {
        (AV_CODEC_FLAG_QSCALE as c_uint | FF_CODEC_CB_TYPE_ENCODE << 29).to_le_bytes()
    },
    priv_data_size: size_of::<PrivData>() as c_int,
    update_thread_context: None,
    update_thread_context_for_user: None,
    defaults: aac_encode_defaults.as_ptr(),
    init_static_data: None,
    init: Some(aac_encode_init),
    cb: CodecCallback {
        encode: Some(aac_encode_frame),
    },
    close: Some(aac_encode_end),
    flush: None,
    bsfs: ptr::null::<c_char>(),
    hw_configs: ptr::null::<*const AVCodecHWConfigInternal>(),
    codec_tags: ptr::null::<c_uint>(),
};
