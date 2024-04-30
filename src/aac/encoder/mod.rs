#![allow(
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

mod channel_layout;
pub(super) mod ctx;
mod dsp;
mod options;
mod pb;
pub(super) mod pow;
mod tables;
mod window;

mod intensity_stereo;
mod temporal_noise_shaping;

use core::panic;
use std::{
    ffi::CStr,
    iter::zip,
    mem::{self, MaybeUninit},
    ptr::{self, addr_of, addr_of_mut, null, null_mut, NonNull},
};

use array_util::{WindowedArray, W};
use arrayvec::ArrayVec;
use encoder::{encoder, Class, Encoder, PacketBuilder};
use ffi::{
    class::option::AVOption,
    codec::{
        channel::AVChannelLayout, frame::AVFrame, profile, AVCodecContext, AVCodecID, FFCodec,
        FFCodecDefault,
    },
};
use ffmpeg_src_macro::ffmpeg_src;
use itertools::Itertools as _;
use izip::izip;
use libc::{c_char, c_double, c_float, c_int, c_long, c_uchar, c_uint, c_ulong, c_void};
use lpc::LPCContext;

pub(crate) use self::temporal_noise_shaping::TemporalNoiseShaping;
use self::{
    channel_layout::pce,
    ctx::AACEncContext,
    options::OPTIONS,
    pb::*,
    tables::{SWB_SIZE_1024, SWB_SIZE_128},
    temporal_noise_shaping as tns,
    window::apply_window_and_mdct,
};
use super::{
    psy_model::{psy_3gpp_analyze, psy_lame_window},
    IndividualChannelStream, SyntaxElementType, WindowSequence, WindowedIteration,
    EIGHT_SHORT_SEQUENCE, ONLY_LONG_SEQUENCE,
};
use crate::{
    aac::{
        coder::{
            mid_side as ms, perceptual_noise_substitution as pns,
            quantize_and_encode_band::quantize_and_encode_band, quantizers,
            set_special_band_scalefactors, trellis,
        },
        tables::{
            ff_aac_num_swb_1024, ff_aac_num_swb_128, SCALEFACTOR_BITS, SCALEFACTOR_CODE,
            SWB_OFFSET_1024, SWB_OFFSET_128, TNS_MAX_BANDS_1024, TNS_MAX_BANDS_128,
        },
        SCALE_DIFF_ZERO,
    },
    audio_frame_queue::{AudioFrameQueue, AudioRemoved},
    avutil::tx::av_tx_uninit,
    common::*,
    mpeg4audio_sample_rates::ff_mpeg4audio_sample_rates,
    psy_model::{ff_psy_end, ff_psy_init},
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
    fn av_mallocz(size: c_ulong) -> *mut c_void;
    fn av_log(avcl: *mut c_void, level: c_int, fmt: *const c_char, _: ...);
}

#[ffmpeg_src(file = "libavcodec/aacenc_utils.h", lines = 65..=78)]
pub(crate) fn quantize_bands(
    in_: &[c_float],
    scaled: &[c_float],
    is_signed: bool,
    maxval: c_int,
    Q34: c_float,
    rounding: c_float,
) -> ArrayVec<c_int, 96> {
    debug_assert_eq!(in_.len(), scaled.len());

    zip(in_, scaled)
        .map(|(&in_0, &scaled)| {
            let qc = scaled * Q34;
            let mut out = (qc + rounding).min(maxval as c_float) as c_int;
            if is_signed && in_0 < 0. {
                out = -out;
            }
            out
        })
        .collect()
}

unsafe extern "C" fn put_pce(
    mut pb: *mut PutBitContext,
    mut avctx: *mut AVCodecContext,
    mut s: *mut AACEncContext,
) {
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut pce: *mut pce::Info = &mut (*s).pce.unwrap();
    let mut aux_data = if (*avctx).flags.bit_exact() {
        c"Lavc"
    } else {
        c"Lavc60.33.100"
    };
    put_bits(pb, 4, 0 as BitBuf);
    put_bits(pb, 2, (*avctx).profile as BitBuf);
    put_bits(pb, 4, (*s).samplerate_index as BitBuf);
    put_bits(pb, 4, (*pce).num_ele[0] as BitBuf);
    put_bits(pb, 4, (*pce).num_ele[1] as BitBuf);
    put_bits(pb, 4, (*pce).num_ele[2] as BitBuf);
    put_bits(pb, 2, (*pce).num_ele[3] as BitBuf);
    put_bits(pb, 3, 0 as BitBuf);
    put_bits(pb, 4, 0 as BitBuf);
    put_bits(pb, 1, 0 as BitBuf);
    put_bits(pb, 1, 0 as BitBuf);
    put_bits(pb, 1, 0 as BitBuf);
    i = 0;
    while i < 4 {
        j = 0;
        while j < (*pce).num_ele[i as usize] {
            if i < 3 {
                put_bits(pb, 1, (*pce).pairing[i as usize][j as usize] as BitBuf);
            }
            put_bits(pb, 4, (*pce).index[i as usize][j as usize] as BitBuf);
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    align_put_bits(pb);
    put_bits(pb, 8, aux_data.to_bytes().len() as BitBuf);
    for c in aux_data.to_bytes() {
        put_bits(pb, 8, *c as u32);
    }
}

unsafe fn put_audio_specific_config(
    mut avctx: *mut AVCodecContext,
    s: &mut AACEncContext,
) -> c_int {
    let mut pb: PutBitContext = PutBitContext {
        bit_buf: 0,
        bit_left: 0,
        buf: ptr::null_mut::<c_uchar>(),
        buf_ptr: ptr::null_mut::<c_uchar>(),
        buf_end: ptr::null_mut::<c_uchar>(),
    };

    let mut channels: c_int =
        (s.needs_pce == 0) as c_int * (s.channels - (if s.channels == 8 { 1 } else { 0 }));
    let max_size: c_int = 32;
    (*avctx).extradata = av_mallocz(max_size as c_ulong) as *mut c_uchar;
    if ((*avctx).extradata).is_null() {
        return -12;
    }
    init_put_bits(&mut pb, (*avctx).extradata, max_size);
    put_bits(&mut pb, 5, (s.profile + 1) as BitBuf);
    put_bits(&mut pb, 4, s.samplerate_index as BitBuf);
    put_bits(&mut pb, 4, channels as BitBuf);
    put_bits(&mut pb, 1, 0 as BitBuf);
    put_bits(&mut pb, 1, 0 as BitBuf);
    put_bits(&mut pb, 1, 0 as BitBuf);
    if s.needs_pce != 0 {
        put_pce(&mut pb, avctx, s);
    }
    put_bits(&mut pb, 11, 0x2b7 as c_int as BitBuf);
    put_bits(&mut pb, 5, AOT_SBR as c_int as BitBuf);
    put_bits(&mut pb, 1, 0 as BitBuf);
    flush_put_bits(&mut pb);
    (*avctx).extradata_size = put_bytes_output(&mut pb);
    0
}

/// Encode ics_info element.
///
/// @see Table 4.6 (syntax of ics_info)
#[ffmpeg_src(file = "libavcodec/aacenc.c", lines = 491..=510)]
unsafe extern "C" fn put_ics_info(s: *mut AACEncContext, info: *const IndividualChannelStream) {
    let pb = addr_of_mut!((*s).pb);
    let IndividualChannelStream {
        window_sequence: [window_sequence, _],
        use_kb_window: [use_kb_window, _],
        max_sfb,
        predictor_present,
        group_len,
        ..
    } = *info;
    put_bits(pb, 1, 0); // ics_reserved bit
    put_bits(pb, 2, window_sequence);
    put_bits(pb, 1, use_kb_window.into());
    if window_sequence != EIGHT_SHORT_SEQUENCE {
        put_bits(pb, 6, max_sfb.into());
        put_bits(pb, 1, predictor_present.into());
    } else {
        put_bits(pb, 4, max_sfb.into());
        for &group_len in &group_len[1..] {
            put_bits(pb, 1, (group_len == 0).into());
        }
    };
}

unsafe extern "C" fn encode_ms_info(mut pb: *mut PutBitContext, mut cpe: *mut ChannelElement) {
    let mut i: c_int = 0;
    put_bits(pb, 2, (*cpe).ms_mode as BitBuf);
    if (*cpe).ms_mode == 1 {
        for WindowedIteration { w, .. } in (*cpe).ch[0].ics.iter_windows() {
            i = 0;
            while i < (*cpe).ch[0].ics.max_sfb as c_int {
                put_bits(pb, 1, (*cpe).ms_mask[W(w)][i as usize] as BitBuf);
                i += 1;
                i;
            }
        }
    }
}

#[ffmpeg_src(file = "libavcodec/aacenc.c", lines = 527..=578)]
unsafe fn adjust_frame_information(cpe: *mut ChannelElement, chans: c_int) {
    let ChannelElement {
        ref mut ch,
        common_window,
        ref mut ms_mode,
        ref ms_mask,
        ..
    } = *cpe;

    for SingleChannelElement {
        ics,
        pulse: Pulse { num_pulse, .. },
        zeroes,
        ..
    } in &mut ch[..chans as usize]
    {
        *num_pulse = 0;
        ics.max_sfb = ics
            .iter_windows()
            .map(|WindowedIteration { w, .. }| {
                zeroes[W(w)][..ics.num_swb as usize]
                    .iter()
                    .rposition(|&zero| !zero)
                    .map(|maxsfb| maxsfb + 1)
                    .unwrap_or_default() as c_uchar
            })
            .max()
            .unwrap();

        let zeroes = zeroes.as_array_of_cells_deref();
        for WindowedIteration { w, group_len } in ics.iter_windows() {
            let zeroes = &zeroes[W(w)];
            for (g, zero) in zeroes.iter().take(ics.max_sfb.into()).enumerate() {
                zero.set(
                    WindowedArray::<_, 16>::from_ref(zeroes)
                        .into_iter()
                        .take(group_len.into())
                        .all(|zeroes| zeroes[g].get()),
                );
            }
        }
    }
    if chans > 1 && common_window != 0 {
        let [SingleChannelElement { ics: ics0, .. }, SingleChannelElement { ics: ics1, .. }] = ch;
        ics0.max_sfb = c_uchar::max(ics0.max_sfb, ics1.max_sfb);
        ics1.max_sfb = ics0.max_sfb;
        let msc = ms_mask
            .into_iter()
            .take(ics0.num_windows as usize)
            .flat_map(|ms_masks| &ms_masks[..ics0.max_sfb.into()])
            .filter(|&&ms_mask| ms_mask)
            .count();
        *ms_mode = if msc == 0 || ics0.max_sfb == 0 {
            0
        } else if msc < usize::from(ics0.max_sfb) * ics0.num_windows as usize {
            1
        } else {
            2
        };
    }
}

unsafe fn encode_band_info(mut s: *mut AACEncContext, mut sce: *mut SingleChannelElement) {
    set_special_band_scalefactors(sce);
    for WindowedIteration { w, group_len } in (*sce).ics.iter_windows() {
        trellis::codebook_rate(s, sce, w, group_len.into());
    }
}

/// preamble for [`NOISE_BT`], put in bitstream with the first noise band
const NOISE_PRE: c_int = 256;
/// length of preamble
const NOISE_PRE_BITS: c_int = 9;
/// subtracted from global gain, used as offset for the preamble
const NOISE_OFFSET: c_int = 90;

#[ffmpeg_src(file = "libavcodec/aac.h", lines = 52)]
const CLIP_AVOIDANCE_FACTOR: c_float = 0.95;

#[ffmpeg_src(file = "libavcodec/aacenc.c", lines = 655..=689)]
unsafe fn encode_scale_factors(
    mut _avctx: *mut AVCodecContext,
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
) {
    let SingleChannelElement {
        ref sf_idx,
        ics: ref ics @ IndividualChannelStream { max_sfb, .. },
        ref zeroes,
        ref band_type,
        ..
    } = *sce;

    let pb = addr_of_mut!((*s).pb);

    let mut diff: c_int = 0;
    let mut off_sf: c_int = (**sf_idx)[0];
    let mut off_pns: c_int = (**sf_idx)[0] - NOISE_OFFSET;
    let mut off_is: c_int = 0;
    let mut noise_flag = true;

    for WindowedIteration { w, .. } in ics.iter_windows() {
        for (_, &band_type, &sf_idx) in izip!(&zeroes[W(w)], &band_type[W(w)], &sf_idx[W(w)])
            .take(max_sfb.into())
            .filter(|(&zero, ..)| !zero)
        {
            let offset = match band_type {
                NOISE_BT => &mut off_pns,
                INTENSITY_BT | INTENSITY_BT2 => &mut off_is,
                _ => &mut off_sf,
            };

            diff = sf_idx - *offset;
            *offset = sf_idx;

            if band_type == NOISE_BT && mem::take(&mut noise_flag) {
                put_bits(pb, NOISE_PRE_BITS, (diff + NOISE_PRE) as BitBuf);
                continue;
            }

            diff += SCALE_DIFF_ZERO as c_int;
            put_bits(
                pb,
                SCALEFACTOR_BITS[diff as usize] as c_int,
                SCALEFACTOR_CODE[diff as usize],
            );
        }
    }
}

/// Encode pulse data.
#[ffmpeg_src(file = "libavcodec/aacenc.c", lines = 691..=708)]
unsafe extern "C" fn encode_pulses(s: *mut AACEncContext, pulse: *const Pulse) {
    let pb = addr_of_mut!((*s).pb);
    let Pulse {
        num_pulse,
        start,
        ref pos,
        ref amp,
    } = *pulse;
    put_bits(pb, 1, (num_pulse != 0) as c_int as BitBuf);
    if num_pulse == 0 {
        return;
    }
    put_bits(pb, 2, (num_pulse - 1) as BitBuf);
    put_bits(pb, 6, start as BitBuf);
    for (&pos, &amp) in zip(pos, amp).take(num_pulse as usize) {
        put_bits(pb, 5, pos as BitBuf);
        put_bits(pb, 4, amp as BitBuf);
    }
}

#[ffmpeg_src(file = "libavcodec/aacenc.c", lines = 710..=736)]
unsafe extern "C" fn encode_spectral_coeffs(
    s: *mut AACEncContext,
    sce: *const SingleChannelElement,
) {
    let lambda = (*s).lambda;
    let pb = NonNull::new(addr_of_mut!((*s).pb)).unwrap();

    let SingleChannelElement {
        ics:
            ref ics @ IndividualChannelStream {
                ref window_clipping,
                max_sfb,
                ..
            },
        ref zeroes,
        ref sf_idx,
        ref band_type,
        ref coeffs,
        ..
    } = *sce;

    for WindowedIteration { w, group_len } in ics.iter_windows() {
        let window_clipping = window_clipping[w as usize];

        for ((swb_size, offset), _, &sf_idx, &band_type) in izip!(
            ics.iter_swb_sizes_sum(),
            &zeroes[W(w)],
            &sf_idx[W(w)],
            &band_type[W(w)]
        )
        .take(max_sfb.into())
        .filter(|&(_, &zero, ..)| !zero)
        {
            let coeffs = WindowedArray::<_, 128>::from_ref(&coeffs[W(w)]);
            for coeffs in coeffs.into_iter().take(group_len.into()) {
                quantize_and_encode_band(
                    pb,
                    &coeffs[offset.into()..][..swb_size.into()],
                    sf_idx,
                    band_type as c_int,
                    lambda,
                    window_clipping.into(),
                );
            }
        }
    }
}

/// Downscale spectral coefficients for near-clipping windows to avoid artifacts
#[ffmpeg_src(file = "libavcodec/aacenc.c", lines = 738..=756)]
fn avoid_clipping(mut sce: &mut SingleChannelElement) {
    if sce.ics.clip_avoidance_factor < 1. {
        for w in 0..sce.ics.num_windows {
            let mut start = 0;

            for i in 0..sce.ics.max_sfb {
                sce.coeffs[W(w)][start.try_into().unwrap()..]
                    [..sce.ics.swb_sizes[usize::from(i)].into()]
                    .iter_mut()
                    .for_each(|swb_coeff| {
                        *swb_coeff *= sce.ics.clip_avoidance_factor;
                    });

                start += sce.ics.swb_sizes[usize::from(i)] as c_int;
            }
        }
    }
}

unsafe extern "C" fn encode_individual_channel(
    mut avctx: *mut AVCodecContext,
    mut s: *mut AACEncContext,
    mut sce: *mut SingleChannelElement,
    mut common_window: c_int,
) -> c_int {
    let pb = addr_of_mut!((*s).pb);
    put_bits(pb, 8, (*sce).sf_idx[W(0)][0] as BitBuf);
    if common_window == 0 {
        put_ics_info(s, addr_of!((*sce).ics));
    }
    encode_band_info(s, sce);
    encode_scale_factors(avctx, s, sce);
    encode_pulses(s, &mut (*sce).pulse);
    put_bits(pb, 1, ((*sce).tns.present != 0) as c_int as BitBuf);
    tns::encode_info(s, sce);
    put_bits(pb, 1, 0 as BitBuf);
    encode_spectral_coeffs(s, sce);
    0
}
unsafe fn put_bitstream_info(mut s: *mut AACEncContext, mut name: &CStr) {
    let mut i: c_int = 0;
    let mut namelen: c_int = 0;
    let mut padbits: c_int = 0;
    namelen = name.to_bytes().len().wrapping_add(2) as c_int;
    put_bits(
        &mut (*s).pb,
        3,
        SyntaxElementType::FillElement as c_int as BitBuf,
    );
    put_bits(
        &mut (*s).pb,
        4,
        (if namelen > 15 { 15 } else { namelen }) as BitBuf,
    );
    if namelen >= 15 {
        put_bits(&mut (*s).pb, 8, (namelen - 14) as BitBuf);
    }
    put_bits(&mut (*s).pb, 4, 0 as BitBuf);
    padbits = -put_bits_count(&mut (*s).pb) & 7;
    align_put_bits(&mut (*s).pb);
    i = 0;
    while i < namelen - 2 {
        put_bits(&mut (*s).pb, 8, name.to_bytes()[i as usize] as BitBuf);
        i += 1;
        i;
    }
    put_bits(&mut (*s).pb, 12 - padbits, 0 as BitBuf);
}

/// Copy input samples.
///
/// Channels are reordered from libavcodec's default order to AAC order.
#[ffmpeg_src(file = "libavcodec/aacenc.c", lines = 804..=828)]
unsafe extern "C" fn copy_input_samples(s: *mut AACEncContext, frame: *const AVFrame) {
    let end: c_int = 2048
        + if !frame.is_null() {
            (*frame).nb_samples
        } else {
            0
        };

    let AACEncContext {
        reorder_map,
        ref mut planar_samples,
        channels,
        ..
    } = *s;

    // copy and remap input samples
    for (&reorder, planar_samples) in
        zip(reorder_map, &mut **planar_samples).take(channels as usize)
    {
        // copy last 1024 samples of previous frame to the start of the current frame
        planar_samples.copy_within(2048..2048 + 1024, 1024);

        // copy new samples and zero any remaining samples
        if !frame.is_null() {
            ptr::copy_nonoverlapping(
                *((*frame).extended_data).offset(reorder.into()) as *mut c_float,
                planar_samples[2048..][..(*frame).nb_samples as usize].as_mut_ptr(),
                (*frame).nb_samples as usize,
            );
        }
        planar_samples[end as usize..].fill(0.);
    }
}

impl IndividualChannelStream {
    #[ffmpeg_src(file = "libavcodec/aacenc.c", lines = 893..=909)]
    fn apply_psy_window_info(
        &mut self,
        tag: c_uchar,
        &FFPsyWindowInfo {
            window_type: [window_type, ..],
            window_shape,
            num_windows,
            ref grouping,
            ..
        }: &FFPsyWindowInfo,
        psy: &FFPsyContext,
        samplerate_index: c_int,
    ) {
        let Self {
            window_sequence: [window_sequence, _],
            use_kb_window: [use_kb_window, _],
            max_sfb,
            num_swb,
            ..
        } = *self;

        let num_swb = if c_int::from(tag) == SyntaxElementType::LowFrequencyEffects as c_int {
            num_swb
        } else {
            psy.num_bands[usize::from(num_windows == 8)]
        };

        *self = IndividualChannelStream {
            window_sequence: [window_type as WindowSequence, window_sequence],
            use_kb_window: [window_shape as c_uchar, use_kb_window],
            num_windows,
            swb_sizes: psy.bands[usize::from(num_windows == 8)],
            num_swb,
            max_sfb: max_sfb.min(num_swb as c_uchar),
            group_len: grouping.map(|grouping| grouping as c_uchar),
            ..if window_type == EIGHT_SHORT_SEQUENCE as c_int {
                IndividualChannelStream {
                    swb_offset: SWB_OFFSET_128[samplerate_index as usize],
                    tns_max_bands: TNS_MAX_BANDS_128[samplerate_index as usize].into(),
                    ..*self
                }
            } else {
                IndividualChannelStream {
                    swb_offset: SWB_OFFSET_1024[samplerate_index as usize],
                    tns_max_bands: TNS_MAX_BANDS_1024[samplerate_index as usize].into(),
                    ..*self
                }
            }
        };
    }
}

#[ffmpeg_src(file = "libavcodec/aacenc.c", lines = 830..=1183)]
unsafe fn aac_encode_frame(
    mut avctx: *mut AVCodecContext,
    mut ctx: *mut AACEncContext,
    mut frame: *const AVFrame,
    mut packet_builder: PacketBuilder,
) -> c_int {
    let mut i: c_int = 0;
    let mut its: c_int = 0;
    let mut ch: c_int = 0;
    let mut w: c_int = 0;
    let mut chans: c_int = 0;
    let mut tag: c_int = 0;
    let mut start_ch: c_int = 0;
    let mut frame_bits: c_int = 0;
    let mut rate_bits: c_int = 0;
    let mut too_many_bits: c_int = 0;
    let mut too_few_bits: c_int = 0;
    let mut ms_mode: c_int = 0;
    let mut is_mode: c_int = 0;
    let mut tns_mode: c_int = 0;
    let mut pred_mode: c_int = 0;
    let mut windows = [FFPsyWindowInfo::default(); 16];
    if !frame.is_null() {
        (*ctx).afq.add_frame(&*frame)
    } else if (*ctx).afq.is_empty() {
        return 0;
    }
    copy_input_samples(ctx, frame);
    if (*avctx).frame_num == 0 {
        return 0;
    }
    start_ch = 0;
    let chan_map = {
        let mut chan_map = (*ctx).chan_map;
        let len = *chan_map.take_first().unwrap();
        &chan_map[..len.into()]
    };

    for (&tag, cpe) in zip(chan_map, &mut *(*ctx).cpe) {
        let mut wi = &mut windows[start_ch.try_into().unwrap()..];
        let chans = if c_int::from(tag) == SyntaxElementType::ChannelPairElement as c_int {
            2
        } else {
            1
        };
        for ch in 0..chans {
            let sce = &mut cpe.ch[usize::try_from(ch).unwrap()];
            let ics = &mut sce.ics;
            (*ctx).cur_channel = start_ch + ch;
            let overlap = &mut (*ctx).planar_samples[(*ctx).cur_channel as usize];
            let la = (!frame.is_null()).then(|| &overlap[1024 + 448 + 64..]);
            let mut wi = &mut wi[usize::try_from(ch).unwrap()];
            if c_uint::from(tag) == SyntaxElementType::LowFrequencyEffects as c_uint {
                let fresh2 = &mut wi.window_type[1];
                *fresh2 = ONLY_LONG_SEQUENCE as c_int;
                wi.window_type[0] = *fresh2;
                wi.window_shape = 0;
                wi.num_windows = 1;
                wi.grouping[0] = 1;
                wi.clipping[0] = 0.;

                // Only the lowest 12 coefficients are used in a LFE channel.
                // The expression below results in only the bottom 8 coefficients
                // being used for 11.025kHz to 16kHz sample rates.
                ics.num_swb = if (*ctx).samplerate_index >= 8 { 1 } else { 3 };
            } else {
                *wi = psy_lame_window(
                    &mut (*ctx).psy,
                    la.map_or_else(null, |la| la.as_ptr()),
                    (*ctx).cur_channel,
                    ics.window_sequence[0] as c_int,
                );
            }

            ics.apply_psy_window_info(tag, wi, &(*ctx).psy, (*ctx).samplerate_index);

            for (clipping, wbuf) in zip(
                &mut wi.clipping,
                WindowedArray::<_, 128>::from_ref(&*overlap),
            )
            .take(ics.num_windows as usize)
            {
                let wlen: c_int = 2048 / ics.num_windows;
                // mdct input is 2 * output
                *clipping = wbuf[..wlen as usize]
                    .iter()
                    .copied()
                    .map(c_float::abs)
                    .max_by(c_float::total_cmp)
                    .unwrap();
            }

            let clip_avoidance_factor = zip(&wi.clipping, &mut ics.window_clipping)
                .take(ics.num_windows as usize)
                .filter_map(|(&clipping, window_clipping)| {
                    if clipping > CLIP_AVOIDANCE_FACTOR {
                        *window_clipping = 1;
                        Some(clipping)
                    } else {
                        *window_clipping = 0;
                        None
                    }
                })
                .max_by(c_float::total_cmp)
                .unwrap_or_default();
            if clip_avoidance_factor > CLIP_AVOIDANCE_FACTOR {
                ics.clip_avoidance_factor = CLIP_AVOIDANCE_FACTOR / clip_avoidance_factor;
            } else {
                ics.clip_avoidance_factor = 1.;
            }
            apply_window_and_mdct(ctx, sce, overlap.as_mut_ptr());

            if sce
                .coeffs
                .iter()
                .any(|coeff| coeff.partial_cmp(&1E16) != Some(std::cmp::Ordering::Less))
            {
                av_log(
                    avctx as *mut c_void,
                    16,
                    c"Input contains (near) NaN/+-Inf\n".as_ptr(),
                );
                return -22;
            }

            avoid_clipping(sce);
        }
        start_ch += chans;
    }
    let mut avpkt = packet_builder.allocate((8192 * (*ctx).channels) as c_long);

    its = 0;
    frame_bits = its;
    let pb = addr_of_mut!((*ctx).pb);
    loop {
        init_put_bits(
            pb,
            avpkt.data_mut().as_mut_ptr(),
            avpkt.data().len() as c_int,
        );
        if (*avctx).frame_num & 0xff as c_int as c_long == 1 as c_long
            && !(*avctx).flags.bit_exact()
        {
            put_bitstream_info(ctx, c"Lavc60.33.100");
        }
        let mut start_ch = 0;
        let mut target_bits = 0;
        let mut chan_el_counter = [0; 4];
        i = 0;
        while i < (*ctx).chan_map[0] as c_int {
            let mut wi_0: *mut FFPsyWindowInfo = windows.as_mut_ptr().offset(start_ch as isize);
            let mut coeffs: [*const c_float; 2] = [ptr::null::<c_float>(); 2];
            tag = (*ctx).chan_map[(i + 1) as usize] as c_int;
            chans = if tag == SyntaxElementType::ChannelPairElement as c_int {
                2
            } else {
                1
            };
            let cpe = &mut (*ctx).cpe[i as usize] as *mut ChannelElement;
            (*cpe).common_window = 0;
            (*cpe).is_mask.fill(false);
            (*cpe).ms_mask.fill(false);
            put_bits(pb, 3, tag as BitBuf);
            let fresh3 = chan_el_counter[tag as usize];
            chan_el_counter[tag as usize] += 1;
            put_bits(pb, 4, fresh3 as BitBuf);
            for (sce, coeffs) in zip(&mut (*cpe).ch, &mut coeffs).take(chans as usize) {
                *coeffs = (sce.coeffs).as_mut_ptr();
                sce.ics.predictor_present = false;
                sce.tns = TemporalNoiseShaping::default();
                for band_type in (*sce.band_type)[..128]
                    .iter_mut()
                    .filter(|&&mut band_type| band_type > RESERVED_BT)
                {
                    *band_type = ZERO_BT;
                }
            }
            (*ctx).psy.bitres.alloc = -1;
            (*ctx).psy.bitres.bits = (*ctx).last_frame_pb_count / (*ctx).channels;
            psy_3gpp_analyze(&mut (*ctx).psy, start_ch, coeffs.as_mut_ptr(), wi_0);
            if (*ctx).psy.bitres.alloc > 0 {
                target_bits = (target_bits as c_float
                    + (*ctx).psy.bitres.alloc as c_float
                        * ((*ctx).lambda
                            / (if (*avctx).global_quality != 0 {
                                (*avctx).global_quality
                            } else {
                                120
                            }) as c_float)) as c_int;
                (*ctx).psy.bitres.alloc /= chans;
            }
            (*ctx).cur_type = (tag as u32).try_into().unwrap();
            ch = 0;
            while ch < chans {
                (*ctx).cur_channel = start_ch + ch;
                if (*ctx).options.pns != 0 {
                    pns::mark(
                        ctx,
                        avctx,
                        &mut *((*cpe).ch).as_mut_ptr().offset(ch as isize),
                    );
                }
                quantizers::twoloop::search(
                    avctx,
                    ctx,
                    &mut *((*cpe).ch).as_mut_ptr().offset(ch as isize),
                    (*ctx).lambda,
                );
                ch += 1;
                ch;
            }
            if chans > 1
                && (*wi_0.offset(0)).window_type[0] == (*wi_0.offset(1)).window_type[0]
                && (*wi_0.offset(0)).window_shape == (*wi_0.offset(1)).window_shape
            {
                (*cpe).common_window = 1;
                w = 0;
                while w < (*wi_0.offset(0)).num_windows {
                    if (*wi_0.offset(0)).grouping[w as usize]
                        != (*wi_0.offset(1)).grouping[w as usize]
                    {
                        (*cpe).common_window = 0;
                        break;
                    } else {
                        w += 1;
                        w;
                    }
                }
            }
            ch = 0;
            while ch < chans {
                let sce =
                    &mut *((*cpe).ch).as_mut_ptr().offset(ch as isize) as *mut SingleChannelElement;
                (*ctx).cur_channel = start_ch + ch;
                if (*ctx).options.tns != 0 {
                    tns::search(ctx, sce);
                }
                if (*ctx).options.tns != 0 {
                    tns::apply(sce);
                }
                if (*sce).tns.present != 0 {
                    tns_mode = 1;
                }
                if (*ctx).options.pns != 0 {
                    pns::search(ctx, avctx, sce);
                }
                ch += 1;
                ch;
            }
            (*ctx).cur_channel = start_ch;
            if (*ctx).options.intensity_stereo != 0 {
                intensity_stereo::search(ctx, avctx, cpe);
                if (*cpe).is_mode {
                    is_mode = 1;
                }
                intensity_stereo::apply(cpe);
            }
            if (*ctx).options.pred != 0 {
                ch = 0;
                while ch < chans {
                    (*ctx).cur_channel = start_ch + ch;
                    if (*ctx).options.pred != 0 {
                        unimplemented!("main pred is unimplemented");
                    }
                    if (*cpe).ch[ch as usize].ics.predictor_present {
                        pred_mode = 1;
                    }
                    ch += 1;
                    ch;
                }

                ch = 0;
                while ch < chans {
                    (*ctx).cur_channel = start_ch + ch;
                    if (*ctx).options.pred != 0 {
                        unimplemented!("main pred is unimplemented");
                    }
                    ch += 1;
                    ch;
                }
                (*ctx).cur_channel = start_ch;
            }
            if (*ctx).options.mid_side != 0 {
                if (*ctx).options.mid_side == -1 {
                    ms::search(ctx, cpe);
                } else if (*cpe).common_window != 0 {
                    (*cpe).ms_mask.fill(true);
                }
                ms::apply(cpe);
            }
            adjust_frame_information(cpe, chans);

            if chans == 2 {
                put_bits(pb, 1, (*cpe).common_window as BitBuf);
                if (*cpe).common_window != 0 {
                    put_ics_info(ctx, addr_of!((*cpe).ch[0].ics));

                    encode_ms_info(pb, cpe);
                    if (*cpe).ms_mode != 0 {
                        ms_mode = 1;
                    }
                }
            }
            for (ch, sce) in (*cpe).ch[..chans as usize].iter_mut().enumerate() {
                (*ctx).cur_channel = start_ch + ch as c_int;
                encode_individual_channel(avctx, ctx, sce, (*cpe).common_window);
            }
            start_ch += chans;
            i += 1;
        }
        if (*avctx).flags.qscale() {
            break;
        }
        frame_bits = put_bits_count(pb);
        rate_bits = ((*avctx).bit_rate * 1024 as c_long / (*avctx).sample_rate as c_long) as c_int;
        rate_bits = if rate_bits > 6144 * (*ctx).channels - 3 {
            6144 * (*ctx).channels - 3
        } else {
            rate_bits
        };
        too_many_bits = if target_bits > rate_bits {
            target_bits
        } else {
            rate_bits
        };
        too_many_bits = if too_many_bits > 6144 * (*ctx).channels - 3 {
            6144 * (*ctx).channels - 3
        } else {
            too_many_bits
        };
        too_few_bits = if (if rate_bits - rate_bits / 4 > target_bits {
            rate_bits - rate_bits / 4
        } else {
            target_bits
        }) > too_many_bits
        {
            too_many_bits
        } else if rate_bits - rate_bits / 4 > target_bits {
            rate_bits - rate_bits / 4
        } else {
            target_bits
        };
        if (*avctx).bit_rate_tolerance == 0 {
            if rate_bits < frame_bits {
                let mut ratio: c_float = rate_bits as c_float / frame_bits as c_float;
                (*ctx).lambda *= if 0.9 > ratio { ratio } else { 0.9 };
            } else {
                (*ctx).lambda = (if (*avctx).global_quality > 0 {
                    (*avctx).global_quality
                } else {
                    120
                }) as c_float;
                break;
            }
        } else {
            too_few_bits = too_few_bits - too_few_bits / 8;
            too_many_bits = too_many_bits + too_many_bits / 2;
            if !(its == 0
                || its < 5 && (frame_bits < too_few_bits || frame_bits > too_many_bits)
                || frame_bits >= 6144 * (*ctx).channels - 3)
            {
                break;
            }
            let mut ratio_0: c_float = rate_bits as c_float / frame_bits as c_float;
            if frame_bits >= too_few_bits && frame_bits <= too_many_bits {
                ratio_0 = sqrtf(sqrtf(ratio_0));
                ratio_0 = av_clipf_c(ratio_0, 0.9, 1.1);
            } else {
                ratio_0 = sqrtf(ratio_0);
            }
            (*ctx).lambda = av_clipf_c((*ctx).lambda * ratio_0, 1.192_092_9e-7_f32, 65536.);
            if ratio_0 > 0.9 && ratio_0 < 1.1 {
                break;
            }
            if is_mode != 0 || ms_mode != 0 || tns_mode != 0 || pred_mode != 0 {
                for (_, ChannelElement { ch, .. }) in zip(chan_map, &mut *(*ctx).cpe) {
                    chans = if tag == SyntaxElementType::ChannelPairElement as c_int {
                        2
                    } else {
                        1
                    };
                    for sce in &mut ch[..chans as usize] {
                        sce.coeffs = sce.pcoeffs;
                    }
                }
            }
            its += 1;
        }
    }
    put_bits(pb, 3, SyntaxElementType::End as BitBuf);
    flush_put_bits(pb);
    (*ctx).last_frame_pb_count = put_bits_count(pb);
    avpkt.truncate(put_bytes_output(pb) as usize);
    (*ctx).lambda_sum += (*ctx).lambda;
    (*ctx).lambda_count += 1;

    {
        let AudioRemoved { pts, duration } = (*ctx).afq.remove((*avctx).frame_size);
        avpkt.set_pts(pts);
        avpkt.set_duration(duration);
    }

    0
}

const aac_encode_defaults: [FFCodecDefault; 2] =
    [FFCodecDefault::new(c"b", c"0"), FFCodecDefault::null()];

struct AACEncoder();

impl Class for AACEncoder {
    const NAME: &'static CStr = c"AAC encoder";

    const OPTIONS: &'static [AVOption] = &OPTIONS;
}

impl Encoder for AACEncoder {
    const NAME: &'static CStr = c"aac";
    const LONG_NAME: &'static CStr = c"AAC (Advanced Audio Coding)";

    const ID: AVCodecID = AV_CODEC_ID_AAC;

    const SUPPORTED_SAMPLERATES: &'static [c_int] = &ff_mpeg4audio_sample_rates;
    const SAMPLE_FMTS: &'static [ffi::codec::AVSampleFormat] = &[8, -1];
    const DEFAULTS: &'static [FFCodecDefault] = &aac_encode_defaults;

    type Ctx = AACEncContext;
    type Options = AACEncOptions;

    fn init(avctx: *mut AVCodecContext, options: &Self::Options) -> Box<Self::Ctx> {
        unsafe {
            (*avctx).frame_size = 1024;
            (*avctx).initial_padding = 1024;
            let ch_layout = addr_of!((*avctx).ch_layout);

            let needs_pce = if channel_layout::NORMAL_LAYOUTS
                .iter()
                .any(|layout| av_channel_layout_compare(ch_layout, layout) == 0)
            {
                options.pce
            } else {
                1
            };

            let channels = (*ch_layout).nb_channels;

            let samplerate_index = ff_mpeg4audio_sample_rates
                .iter()
                .find_position(|&&sample_rate| (*avctx).sample_rate == sample_rate)
                .map(|(i, _)| i)
                .filter(|&i| i < SWB_SIZE_1024.len() && i < SWB_SIZE_128.len())
                .unwrap_or_else(|| panic!("Unsupported sample rate {}", (*avctx).sample_rate))
                as c_int;

            let (pce, reorder_map, chan_map) = if needs_pce != 0 {
                let mut buf: [c_char; 64] = [0; 64];
                av_channel_layout_describe(ch_layout, buf.as_mut_ptr(), buf.len() as c_ulong);

                let config = pce::CONFIGS
                    .iter()
                    .find(|config| av_channel_layout_compare(ch_layout, &config.layout) == 0)
                    .unwrap_or_else(|| {
                        panic!(
                            "Unsupported channel layout {}",
                            CStr::from_ptr(buf.as_ptr()).to_string_lossy()
                        )
                    });

                av_log(
                    avctx.cast(),
                    32,
                    c"Using a PCE to encode channel layout \"%s\"\n".as_ptr(),
                    buf.as_mut_ptr(),
                );

                (
                    Some(*config),
                    &config.reorder_map,
                    config.config_map.as_slice(),
                )
            } else {
                (
                    None,
                    &channel_layout::REORDER_MAPS[(channels - 1) as usize],
                    channel_layout::CONFIGS[(channels - 1) as usize].as_slice(),
                )
            };

            assert_eq!(
                options.coder as c_uint, AAC_CODER_TWOLOOP,
                "only twoloop coder is supported"
            );

            let mut ctx = Box::new(AACEncContext {
                options: *options,
                pb: PutBitContext::zero(),
                mdct1024: null_mut(),
                mdct1024_fn: None,
                mdct128: null_mut(),
                mdct128_fn: None,
                pce,
                planar_samples: vec![[0.; _]; (*ch_layout).nb_channels as usize].into_boxed_slice(),
                profile: 0,
                needs_pce,
                lpc: LPCContext::new(2 * (*avctx).frame_size, 20, lpc::Type::Levinson),
                samplerate_index,
                channels: (*ch_layout).nb_channels,
                reorder_map,
                chan_map,
                cpe: vec![ChannelElement::default(); chan_map[0] as usize].into_boxed_slice(),
                psy: FFPsyContext::zero(),
                cur_channel: 0,
                random_state: 0x1f2e3d4c,
                lambda: if (*avctx).global_quality > 0 {
                    (*avctx).global_quality as c_float
                } else {
                    120.
                },
                last_frame_pb_count: 0,
                lambda_sum: 0.,
                lambda_count: 0,
                cur_type: SyntaxElementType::SingleChannelElement,
                afq: AudioFrameQueue::new(avctx),
                scaled_coeffs: Default::default(),
                quantize_band_cost_cache: Default::default(),
            });

            let mut i: c_int = 0;
            let mut ret: c_int = 0;
            let mut grouping: [c_uchar; 16] = [0; 16];

            if (*avctx).bit_rate == 0 {
                i = 1;
                while i <= ctx.chan_map[0] as c_int {
                    (*avctx).bit_rate += match u32::from(ctx.chan_map[i as usize]).try_into() {
                        Ok(SyntaxElementType::ChannelPairElement) => 128000,
                        Ok(SyntaxElementType::LowFrequencyEffects) => 16000,
                        _ => 69000,
                    } as c_long;

                    i += 1;
                    i;
                }
            }

            if 1024. * (*avctx).bit_rate as c_double / (*avctx).sample_rate as c_double
                > (6144 * ctx.channels) as c_double
            {
                av_log(
                    avctx.cast(),
                    24,
                    c"Too many bits %f > %d per frame requested, clamping to max\n".as_ptr(),
                    1024. * (*avctx).bit_rate as c_double / (*avctx).sample_rate as c_double,
                    6144 * ctx.channels,
                );
            }
            (*avctx).bit_rate = (((6144 * ctx.channels) as c_double / 1024.
                * (*avctx).sample_rate as c_double) as c_long)
                .min((*avctx).bit_rate);

            (*avctx).profile = if (*avctx).profile == profile::UNKNOWN {
                profile::AAC_LOW
            } else {
                (*avctx).profile
            };

            if (*avctx).profile == profile::MPEG2_AAC_LOW as c_int {
                (*avctx).profile = profile::AAC_LOW;
                assert_eq!(
                    ctx.options.pred, 0,
                    "Main prediction unavailable in the \"mpeg2_aac_low\" profile"
                );
                assert_eq!(
                    ctx.options.ltp, 0,
                    "LTP prediction unavailable in the \"mpeg2_aac_low\" profile"
                );
                if ctx.options.pns != 0 {
                    av_log(
                        avctx.cast(),
                        24,
                        c"PNS unavailable in the \"mpeg2_aac_low\" profile, turning off\n".as_ptr(),
                    );
                }
                ctx.options.pns = 0;
            } else if (*avctx).profile == profile::AAC_LTP {
                ctx.options.ltp = 1;
                assert_eq!(
                    ctx.options.pred, 0,
                    "Main prediction unavailable in the \"aac_ltp\" profile"
                );
            } else if (*avctx).profile == profile::AAC_MAIN {
                ctx.options.pred = 1;
                assert_eq!(
                    ctx.options.ltp, 0,
                    "LTP prediction unavailable in the \"aac_main\" profile"
                );
            } else if ctx.options.ltp != 0 {
                (*avctx).profile = profile::AAC_LTP;
                av_log(
                    avctx.cast(),
                    24,
                    c"Chainging profile to \"aac_ltp\"\n".as_ptr(),
                );
                assert_eq!(
                    ctx.options.pred, 0,
                    "Main prediction unavailable in the \"aac_ltp\" profile"
                );
            } else if ctx.options.pred != 0 {
                (*avctx).profile = profile::AAC_MAIN;
                av_log(
                    avctx.cast(),
                    24,
                    c"Chainging profile to \"aac_main\"\n".as_ptr(),
                );
                assert_eq!(
                    ctx.options.ltp, 0,
                    "LTP prediction unavailable in the \"aac_main\" profile"
                );
            }
            ctx.profile = (*avctx).profile;
            let experimental = (*avctx).strict_std_compliance <= -2;
            if ctx.options.coder == AAC_CODER_ANMR as c_int {
                if !experimental {
                    panic!("The ANMR coder is considered experimental, add -strict -2 to enable!");
                }
                ctx.options.intensity_stereo = 0;
                ctx.options.pns = 0;
            }
            if ctx.options.ltp != 0 && !experimental {
                panic!(
                    "The LPT profile requires experimental compliance, add -strict -2 to enable!"
                );
            }
            if ctx.channels > 3 {
                ctx.options.mid_side = 0;
            }

            ret = dsp::init(avctx, &mut *ctx);
            assert!(ret >= 0, "dsp::init failed");
            ret = put_audio_specific_config(avctx, &mut ctx);
            assert!(ret >= 0, "put_audio_specific_config failed");
            let mut sizes = [
                SWB_SIZE_1024[ctx.samplerate_index as usize],
                SWB_SIZE_128[ctx.samplerate_index as usize],
            ];
            let mut lengths = [
                ff_aac_num_swb_1024[ctx.samplerate_index as usize] as c_int,
                ff_aac_num_swb_128[ctx.samplerate_index as usize] as c_int,
            ];
            i = 0;
            while i < ctx.chan_map[0] as c_int {
                grouping[i as usize] = (ctx.chan_map[(i + 1) as usize] as c_int
                    == SyntaxElementType::ChannelPairElement as c_int)
                    as c_int as c_uchar;
                i += 1;
                i;
            }

            ret = ff_psy_init(
                &mut ctx.psy,
                avctx,
                &sizes,
                &lengths,
                ctx.chan_map[0] as c_int,
                grouping.as_mut_ptr(),
            );
            assert!(ret >= 0, "ff_psy_init failed");

            ctx
        }
    }

    fn encode_frame(
        avctx: *mut AVCodecContext,
        ctx: &mut Self::Ctx,
        _: &Self::Options,
        frame: &AVFrame,
        packet_builder: PacketBuilder,
    ) {
        let ret = unsafe { aac_encode_frame(avctx, ctx, frame, packet_builder) };
        assert!(ret >= 0, "aac_encode_frame failed");
    }

    fn close(avctx: *mut AVCodecContext, mut ctx: Box<Self::Ctx>) {
        unsafe {
            av_log(
                avctx.cast(),
                32,
                c"Qavg: %.3f\n".as_ptr(),
                if ctx.lambda_count != 0 {
                    ctx.lambda_sum as c_double / ctx.lambda_count as c_double
                } else {
                    c_double::NAN
                },
            );
            av_tx_uninit(&mut ctx.mdct1024);
            av_tx_uninit(&mut ctx.mdct128);
            ff_psy_end(&mut ctx.psy);
        }
    }
}

#[no_mangle]
pub static mut ff_aac_encoder: FFCodec = encoder::<AACEncoder>();
