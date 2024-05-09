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
    array,
    ffi::CStr,
    iter::zip,
    mem::{self, MaybeUninit},
    ptr::{self, addr_of, addr_of_mut, null_mut, NonNull},
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
};
use super::{IndividualChannelStream, SyntaxElementType, WindowSequence, WindowedIteration};
use crate::{
    aac::{
        coder::{
            mid_side as ms, perceptual_noise_substitution as pns,
            quantize_and_encode_band::quantize_and_encode_band, quantizers,
            set_special_band_scalefactors, trellis,
        },
        encoder::ctx::MdctContext,
        tables::{
            ff_aac_num_swb_1024, ff_aac_num_swb_128, SCALEFACTOR_BITS, SCALEFACTOR_CODE,
            SWB_OFFSET_1024, SWB_OFFSET_128, TNS_MAX_BANDS_1024, TNS_MAX_BANDS_128,
        },
        SCALE_DIFF_ZERO,
    },
    audio_frame_queue::{AudioFrameQueue, AudioRemoved},
    avutil::tx::av_tx_uninit,
    mpeg4audio_sample_rates::ff_mpeg4audio_sample_rates,
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
        buf: null_mut(),
        buf_ptr: null_mut(),
        buf_end: null_mut(),
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
    put_bits(pb, 2, (window_sequence as u8).into());
    put_bits(pb, 1, use_kb_window.into());
    if window_sequence != WindowSequence::EightShort {
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
                    window_clipping,
                );
            }
        }
    }
}

impl SingleChannelElement {
    /// Downscale spectral coefficients for near-clipping windows to avoid
    /// artifacts
    #[ffmpeg_src(file = "libavcodec/aacenc.c", lines = 738..=756)]
    fn avoid_clipping(&mut self) {
        let Self {
            ics:
                IndividualChannelStream {
                    clip_avoidance_factor,
                    max_sfb,
                    num_windows,
                    swb_sizes,
                    ..
                },
            ref mut coeffs,
            ..
        } = *self;

        if clip_avoidance_factor >= 1. {
            return;
        }

        for coeffs in coeffs
            .as_array_of_cells_deref()
            .into_iter()
            .take(num_windows as usize)
        {
            let mut start = 0;

            for &swb_size in swb_sizes.iter().take(max_sfb.into()) {
                for coeff in &coeffs[start..][..swb_size.into()] {
                    coeff.update(|coeff| coeff * clip_avoidance_factor);
                }

                start += usize::from(swb_size);
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
unsafe extern "C" fn copy_input_samples(s: &mut AACEncContext, frame: *const AVFrame) {
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
                *(*frame).extended_data.offset(reorder.into()) as *mut c_float,
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
        tag: SyntaxElementType,
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

        let num_swb = if tag == SyntaxElementType::LowFrequencyEffects {
            num_swb
        } else {
            psy.num_bands[usize::from(num_windows == 8)]
        };

        *self = IndividualChannelStream {
            window_sequence: [window_type as WindowSequence, window_sequence],
            use_kb_window: [window_shape != 0, use_kb_window],
            num_windows,
            swb_sizes: psy.bands[usize::from(num_windows == 8)],
            num_swb,
            max_sfb: max_sfb.min(num_swb as c_uchar),
            group_len: grouping.map(|grouping| grouping as c_uchar),
            ..if window_type == WindowSequence::EightShort {
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

    /// Calculate input sample maximums and evaluate clipping risk
    #[ffmpeg_src(file = "libavcodec/aacenc.c", lines = 911..=935)]
    fn calc_clip_avoidance_factor(&mut self, overlap: &[c_float]) {
        let mut clipping = MaybeUninit::uninit_array::<8>();
        let wlen = 2048 / self.num_windows;

        let (clipping, _) = MaybeUninit::fill_from(
            &mut clipping,
            WindowedArray::<_, 128>::from_ref(overlap)
                .into_iter()
                .take(self.num_windows as usize)
                .map(|wbuf| {
                    // mdct input is 2 * output
                    wbuf[..wlen as usize]
                        .iter()
                        .copied()
                        .map(c_float::abs)
                        .max_by(c_float::total_cmp)
                        .unwrap()
                }),
        );

        let clip_avoidance_factor = zip(&*clipping, &mut self.window_clipping)
            .filter_map(|(&clipping, window_clipping)| {
                if clipping > CLIP_AVOIDANCE_FACTOR {
                    *window_clipping = true;
                    Some(clipping)
                } else {
                    *window_clipping = false;
                    None
                }
            })
            .max_by(c_float::total_cmp)
            .unwrap_or_default();

        self.clip_avoidance_factor = if clip_avoidance_factor > CLIP_AVOIDANCE_FACTOR {
            CLIP_AVOIDANCE_FACTOR / clip_avoidance_factor
        } else {
            1.
        };
    }
}

unsafe fn analyze_psy_windows(
    ctx: &mut AACEncContext,
    cpe: &mut Box<[ChannelElement]>,
    frame: *const AVFrame,
    windows: &mut [FFPsyWindowInfo; 16],
) {
    let mut windows = windows.as_mut_slice();
    let mut planar_samples = ctx.planar_samples.as_mut();
    let mut start_ch = 0;
    for (&tag, cpe) in zip(ctx.chan_map, &mut **cpe) {
        let chans = tag.channels();
        for (ch, (sce, wi, overlap)) in izip!(
            &mut cpe.ch,
            windows.take_mut(..chans).unwrap(),
            planar_samples.take_mut(..chans).unwrap(),
        )
        .enumerate()
        {
            let ics = &mut sce.ics;
            ctx.cur_channel = (start_ch + ch) as c_int;
            let la = (!frame.is_null()).then(|| &overlap[1024 + 448 + 64..]);
            if tag == SyntaxElementType::LowFrequencyEffects {
                wi.window_type[0] = WindowSequence::OnlyLong;
                wi.window_type[1] = WindowSequence::OnlyLong;
                wi.window_shape = 0;
                wi.num_windows = 1;
                wi.grouping[0] = 1;

                // Only the lowest 12 coefficients are used in a LFE channel.
                // The expression below results in only the bottom 8 coefficients
                // being used for 11.025kHz to 16kHz sample rates.
                ics.num_swb = if ctx.samplerate_index >= 8 { 1 } else { 3 };
            } else {
                *wi = ctx.psy.window(la, ctx.cur_channel, ics.window_sequence[0]);
            }

            ics.apply_psy_window_info(tag, wi, &ctx.psy, ctx.samplerate_index);
            ics.calc_clip_avoidance_factor(overlap);

            sce.apply_window_and_mdct(&ctx.mdct, overlap);

            assert!(
                !sce.coeffs
                    .iter()
                    .any(|coeff| coeff.partial_cmp(&1E16) != Some(std::cmp::Ordering::Less)),
                "Input contains (near) NaN/+-Inf"
            );

            sce.avoid_clipping();
        }
        start_ch += chans;
    }
}

#[ffmpeg_src(file = "libavcodec/aacenc.c", lines = 830..=1183)]
unsafe fn aac_encode_frame(
    mut avctx: *mut AVCodecContext,
    mut ctx: &mut AACEncContext,
    mut cpe: &mut Box<[ChannelElement]>,
    mut frame: *const AVFrame,
    mut packet_builder: PacketBuilder,
) -> c_int {
    let mut ms_mode = false;
    let mut is_mode = false;
    let mut tns_mode = false;
    let mut pred_mode = false;
    let mut windows = [FFPsyWindowInfo::default(); 16];

    if !frame.is_null() {
        ctx.afq.add_frame(frame)
    } else if ctx.afq.is_empty() {
        return 0;
    }

    copy_input_samples(ctx, frame);
    if (*avctx).frame_num == 0 {
        return 0;
    }

    analyze_psy_windows(ctx, cpe, frame, &mut windows);

    let mut avpkt = packet_builder.allocate((8192 * ctx.channels) as c_long);

    let pb = addr_of_mut!(ctx.pb);
    for iterations in 0.. {
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
        let mut windows = windows.as_slice();
        let mut start_ch = 0;
        let mut target_bits = 0;
        let mut chan_el_counter = [0; 4];
        for (&tag, cpe) in zip(ctx.chan_map, &mut **cpe) {
            let windows = windows.take(..tag.channels()).unwrap();
            let mut coeffs: [&[c_float]; 2] = [&[]; 2];

            *cpe = ChannelElement {
                common_window: 0,
                is_mask: Default::default(),
                ms_mask: Default::default(),
                ..*cpe
            };

            put_bits(pb, 3, tag as BitBuf);
            let chan_el_counter = &mut chan_el_counter[tag as usize];
            put_bits(pb, 4, *chan_el_counter as BitBuf);
            *chan_el_counter += 1;

            for (sce, coeffs) in zip(&mut cpe.ch, &mut coeffs).take(windows.len()) {
                *coeffs = &**sce.coeffs;
                sce.ics.predictor_present = false;
                sce.tns = TemporalNoiseShaping::default();
                for band_type in (*sce.band_type)[..128]
                    .iter_mut()
                    .filter(|&&mut band_type| band_type > RESERVED_BT)
                {
                    *band_type = ZERO_BT;
                }
            }
            ctx.psy.bitres.alloc = -1;
            ctx.psy.bitres.bits = ctx.last_frame_pb_count / ctx.channels;
            ctx.psy.analyze(start_ch, &coeffs, windows);
            if ctx.psy.bitres.alloc > 0 {
                target_bits = (target_bits as c_float
                    + ctx.psy.bitres.alloc as c_float
                        * (ctx.lambda
                            / (if (*avctx).global_quality != 0 {
                                (*avctx).global_quality
                            } else {
                                120
                            }) as c_float)) as c_int;
                ctx.psy.bitres.alloc /= windows.len() as c_int;
            }

            ctx.cur_type = tag;

            for (ch, sce) in cpe.ch.iter_mut().take(windows.len()).enumerate() {
                ctx.cur_channel = start_ch + ch as c_int;
                if ctx.options.pns != 0 {
                    pns::mark(ctx, avctx, sce);
                }
                quantizers::twoloop::search(avctx, ctx, sce, ctx.lambda);
            }
            if let [wi0, wi1] = windows
                && wi0.window_type[0] == wi1.window_type[0]
                && wi0.window_shape == wi1.window_shape
            {
                cpe.common_window = zip(&wi0.grouping, &wi1.grouping)
                    .take(wi0.num_windows as usize)
                    .all(|(grouping0, grouping1)| grouping0 == grouping1)
                    .into();
            }
            for (ch, sce) in cpe.ch.iter_mut().take(windows.len()).enumerate() {
                ctx.cur_channel = start_ch + ch as c_int;
                if ctx.options.tns != 0 {
                    tns::search(ctx, sce);
                }
                if ctx.options.tns != 0 {
                    tns::apply(sce);
                }
                if sce.tns.present != 0 {
                    tns_mode = true;
                }
                if ctx.options.pns != 0 {
                    pns::search(ctx, avctx, sce);
                }
            }
            ctx.cur_channel = start_ch;
            if ctx.options.intensity_stereo != 0 {
                intensity_stereo::search(ctx, avctx, cpe);
                if cpe.is_mode {
                    is_mode = true;
                }
                intensity_stereo::apply(cpe);
            }
            if ctx.options.pred != 0 {
                for (ch, sce) in cpe.ch.iter().take(windows.len()).enumerate() {
                    ctx.cur_channel = start_ch + ch as c_int;
                    if ctx.options.pred != 0 {
                        unimplemented!("main pred is unimplemented");
                    }
                    if sce.ics.predictor_present {
                        pred_mode = true;
                    }
                }

                ctx.cur_channel = start_ch;
            }
            if ctx.options.mid_side != 0 {
                if ctx.options.mid_side == -1 {
                    ms::search(ctx, cpe);
                } else if cpe.common_window != 0 {
                    cpe.ms_mask.fill(true);
                }
                ms::apply(cpe);
            }
            adjust_frame_information(cpe, windows.len() as c_int);

            if let [_, _] = windows {
                put_bits(pb, 1, cpe.common_window as BitBuf);
                if cpe.common_window != 0 {
                    put_ics_info(ctx, addr_of!(cpe.ch[0].ics));

                    encode_ms_info(pb, cpe);
                    if cpe.ms_mode != 0 {
                        ms_mode = true;
                    }
                }
            }
            for (ch, sce) in cpe.ch.iter_mut().take(windows.len()).enumerate() {
                ctx.cur_channel = start_ch + ch as c_int;
                encode_individual_channel(avctx, ctx, sce, cpe.common_window);
            }
            start_ch += windows.len() as c_int;
        }

        if (*avctx).flags.qscale() {
            // When using a constant Q-scale, don't mess with lambda
            break;
        }

        // rate control stuff
        // allow between the nominal bitrate, and what psy's bit reservoir says to
        // target but drift towards the nominal bitrate always
        let frame_bits = put_bits_count(pb);
        let rate_bits = (((*avctx).bit_rate * 1024 / (*avctx).sample_rate as c_long) as c_int)
            .min(6144 * ctx.channels - 3);
        let mut too_many_bits = target_bits.max(rate_bits).min(6144 * ctx.channels - 3);
        let mut too_few_bits = (rate_bits - rate_bits / 4)
            .max(target_bits)
            .min(too_many_bits);

        // When strict bit-rate control is demanded
        if (*avctx).bit_rate_tolerance == 0 {
            if rate_bits < frame_bits {
                let mut ratio = rate_bits as c_float / frame_bits as c_float;
                ctx.lambda *= ratio.min(0.9);
            }

            // reset lambda when solution is found
            ctx.lambda = if (*avctx).global_quality > 0 {
                (*avctx).global_quality
            } else {
                120
            } as c_float;
            break;
        }

        // When using ABR, be strict (but only for increasing)
        too_few_bits -= too_few_bits / 8;
        too_many_bits += too_many_bits / 2;

        let bits_range = too_few_bits..=too_many_bits;

        if !(iterations == 0 // for steady-state Q-scale tracking 
            || iterations < 5 && !bits_range.contains(&frame_bits)
            || frame_bits >= 6144 * ctx.channels - 3)
        {
            break;
        }

        let ratio = {
            let ratio = rate_bits as c_float / frame_bits as c_float;
            if bits_range.contains(&frame_bits) {
                // This path is for steady-state Q-scale tracking
                // When frame bits fall within the stable range, we still need to adjust
                // lambda to maintain it like so in a stable fashion (large jumps in lambda
                // create artifacts and should be avoided), but slowly
                ratio.sqrt().sqrt().clamp(0.9, 1.1)
            } else {
                // Not so fast though
                ratio.sqrt()
            }
        };

        ctx.lambda = (ctx.lambda * ratio).clamp(c_float::EPSILON, 65536.);

        // Keep iterating if we must reduce and lambda is in the sky
        if ratio > 0.9 && ratio < 1.1 {
            break;
        }

        if is_mode || ms_mode || tns_mode || pred_mode {
            let chans = ctx.chan_map.last().unwrap().channels();
            // Must restore coeffs
            for sce in cpe
                .iter_mut()
                .flat_map(|ChannelElement { ch, .. }| &mut ch[..chans])
            {
                sce.coeffs = sce.pcoeffs;
            }
        }
    }

    put_bits(pb, 3, SyntaxElementType::End as BitBuf);
    flush_put_bits(pb);
    ctx.last_frame_pb_count = put_bits_count(pb);
    avpkt.truncate(put_bytes_output(pb) as usize);
    ctx.lambda_sum += ctx.lambda;
    ctx.lambda_count += 1;

    {
        let AudioRemoved { pts, duration } = ctx.afq.remove((*avctx).frame_size);
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

    type Ctx = (AACEncContext, Box<[ChannelElement]>);
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

                (Some(*config), &config.reorder_map, config.config_map)
            } else {
                (
                    None,
                    &channel_layout::REORDER_MAPS[(channels - 1) as usize],
                    channel_layout::CONFIGS[(channels - 1) as usize],
                )
            };

            assert_eq!(
                options.coder as c_uint, AAC_CODER_TWOLOOP,
                "only twoloop coder is supported"
            );

            let mut res = Box::new((
                AACEncContext {
                    options: *options,
                    pb: PutBitContext::zero(),
                    mdct: MdctContext {
                        mdct1024: null_mut(),
                        mdct1024_fn: None,
                        mdct128: null_mut(),
                        mdct128_fn: None,
                    },
                    pce,
                    planar_samples: vec![[0.; _]; (*ch_layout).nb_channels as usize]
                        .into_boxed_slice(),
                    profile: 0,
                    needs_pce,
                    lpc: LPCContext::new(2 * (*avctx).frame_size, 20, lpc::Type::Levinson),
                    samplerate_index,
                    channels: (*ch_layout).nb_channels,
                    reorder_map,
                    chan_map,
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
                },
                vec![ChannelElement::default(); chan_map.len()].into_boxed_slice(),
            ));
            let (ctx, _) = &mut *res;

            let mut ret: c_int = 0;

            if (*avctx).bit_rate == 0 {
                (*avctx).bit_rate = ctx
                    .chan_map
                    .iter()
                    .map(|&tag| match tag {
                        SyntaxElementType::ChannelPairElement => 128000,
                        SyntaxElementType::LowFrequencyEffects => 16000,
                        _ => 69000,
                    })
                    .sum();
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

            ret = dsp::init(ctx);
            assert!(ret >= 0, "dsp::init failed");
            ret = put_audio_specific_config(avctx, ctx);
            assert!(ret >= 0, "put_audio_specific_config failed");
            let mut sizes = [
                SWB_SIZE_1024[ctx.samplerate_index as usize],
                SWB_SIZE_128[ctx.samplerate_index as usize],
            ];
            let mut lengths = [
                ff_aac_num_swb_1024[ctx.samplerate_index as usize] as c_int,
                ff_aac_num_swb_128[ctx.samplerate_index as usize] as c_int,
            ];
            let grouping = array::from_fn::<c_uchar, 16, _>(|i| {
                ctx.chan_map
                    .get(i)
                    .map(|&tag| (tag == SyntaxElementType::ChannelPairElement).into())
                    .unwrap_or_default()
            });

            ctx.psy = FFPsyContext::init(
                avctx,
                &sizes,
                &lengths,
                ctx.chan_map.len() as c_int,
                &grouping,
            );

            res
        }
    }

    fn encode_frame(
        avctx: *mut AVCodecContext,
        ctx: &mut Self::Ctx,
        _: &Self::Options,
        frame: *const AVFrame,
        packet_builder: PacketBuilder,
    ) {
        let (ctx, cpe) = &mut *ctx;
        let ret = unsafe { aac_encode_frame(avctx, ctx, cpe, frame, packet_builder) };
        assert!(ret >= 0, "aac_encode_frame failed");
    }

    fn close(avctx: *mut AVCodecContext, mut ctx: Box<Self::Ctx>) {
        let (ctx, _) = &mut *ctx;
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
            av_tx_uninit(&mut ctx.mdct.mdct1024);
            av_tx_uninit(&mut ctx.mdct.mdct128);
        }
    }
}

#[no_mangle]
pub static mut ff_aac_encoder: FFCodec = encoder::<AACEncoder>();
