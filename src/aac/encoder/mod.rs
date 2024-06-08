mod channel_layout;
pub(super) mod ctx;
mod dsp;
mod options;
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
    ptr::{self, addr_of, null_mut},
};

use array_util::{WindowedArray, W, W2};
use arrayvec::ArrayVec;
use bit_writer::{BitBuf, BitWriter};
use encoder::{encoder, Class, CodecContext, Encoder, Frame, PacketBuilder};
use ffi::{
    class::option::AVOption,
    codec::{channel::AVChannelLayout, profile, AVCodecID, FFCodec, FFCodecDefault},
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
    tables::{SWB_SIZE_1024, SWB_SIZE_128},
    temporal_noise_shaping as tns,
};
use super::{IndividualChannelStream, SyntaxElementType, WindowSequence, WindowedIteration};
use crate::{
    aac::{
        coder::{
            mid_side as ms, perceptual_noise_substitution as pns,
            quantization::quantize_and_encode_band, quantizers, set_special_band_scalefactors,
            trellis,
        },
        encoder::ctx::MdctContext,
        tables::{
            NUM_SWB_1024, NUM_SWB_128, SCALEFACTOR_BITS, SCALEFACTOR_CODE, SWB_OFFSET_1024,
            SWB_OFFSET_128, TNS_MAX_BANDS_1024, TNS_MAX_BANDS_128,
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
    fn av_log(avcl: *mut c_void, level: c_int, fmt: *const c_char, _: ...);
}

#[ffmpeg_src(file = "libavcodec/aacenc_utils.h", lines = 65..=78)]
pub(crate) fn quantize_bands(
    in_: &[c_float],
    scaled: &[c_float],
    is_signed: bool,
    maxval: c_int,
    q34: c_float,
    rounding: c_float,
) -> ArrayVec<c_int, 96> {
    debug_assert_eq!(in_.len(), scaled.len());

    zip(in_, scaled)
        .map(|(&in_0, &scaled)| {
            let qc = scaled * q34;
            let mut out = (qc + rounding).min(maxval as c_float) as c_int;
            if is_signed && in_0 < 0. {
                out = -out;
            }
            out
        })
        .collect()
}

fn put_pce(avctx: &CodecContext, s: &mut AACEncContext, pb: &mut BitWriter) {
    let pce = &s.pce.unwrap();
    let aux_data = if avctx.flags().get().bit_exact() {
        c"Lavc"
    } else {
        c"Lavc60.33.100"
    };
    pb.put(4, 0);
    pb.put(2, avctx.profile().get() as _);
    pb.put(4, s.samplerate_index as _);
    pb.put(4, pce.num_ele[0] as _);
    pb.put(4, pce.num_ele[1] as _);
    pb.put(4, pce.num_ele[2] as _);
    pb.put(2, pce.num_ele[3] as _);
    pb.put(3, 0);
    pb.put(4, 0);
    pb.put(1, 0);
    pb.put(1, 0);
    pb.put(1, 0);
    let mut i = 0;
    while i < 4 {
        let mut j = 0;
        while j < pce.num_ele[i as usize] {
            if i < 3 {
                pb.put(1, pce.pairing[i as usize][j as usize] as _);
            }
            pb.put(4, pce.index[i as usize][j as usize] as _);
            j += 1;
        }
        i += 1;
    }
    pb.align();
    pb.put(8, aux_data.to_bytes().len() as _);
    for c in aux_data.to_bytes() {
        pb.put(8, (*c).into());
    }
}

fn put_audio_specific_config(avctx: &mut CodecContext, s: &mut AACEncContext) {
    const MAX_SIZE: usize = 32;

    let channels =
        (s.needs_pce == 0) as c_int * (s.channels - (if s.channels == 8 { 1 } else { 0 }));

    let mut extradata = vec![0; MAX_SIZE].into_boxed_slice();
    let mut pb = BitWriter::new(&mut extradata);

    pb.put(5, (s.profile + 1) as _);
    pb.put(4, s.samplerate_index as _);
    pb.put(4, channels as _);
    pb.put(1, 0);
    pb.put(1, 0);
    pb.put(1, 0);
    if s.needs_pce != 0 {
        put_pce(avctx, s, &mut pb);
    }
    pb.put(11, 0x2b7);
    pb.put(5, AOT_SBR as _);
    pb.put(1, 0);
    pb.flush();

    avctx.extradata_size().set(pb.total_bytes_written() as _);
    avctx.extradata().set(Box::into_raw(extradata).as_mut_ptr());
}

/// Encode ics_info element.
///
/// @see Table 4.6 (syntax of ics_info)
#[ffmpeg_src(file = "libavcodec/aacenc.c", lines = 491..=510)]
fn put_ics_info(pb: &mut BitWriter, info: &IndividualChannelStream) {
    let IndividualChannelStream {
        window_sequence: [window_sequence, _],
        use_kb_window: [use_kb_window, _],
        max_sfb,
        predictor_present,
        group_len,
        ..
    } = *info;

    pb.put(1, 0); // ics_reserved bit
    pb.put(2, (window_sequence as u8).into());
    pb.put(1, use_kb_window.into());
    if window_sequence != WindowSequence::EightShort {
        pb.put(6, max_sfb.into());
        pb.put(1, predictor_present.into());
    } else {
        pb.put(4, max_sfb.into());
        for &group_len in &group_len[1..] {
            pb.put(1, (group_len == 0).into());
        }
    };
}

/// Encode MS data.
///
/// See 4.6.8.1 "Joint Coding - M/S Stereo"
#[ffmpeg_src(file = "libavcodec/aacenc.c", lines = 513..=526)]
fn encode_ms_info(pb: &mut BitWriter, cpe: &ChannelElement) {
    pb.put(2, cpe.ms_mode as _);

    if cpe.ms_mode != 1 {
        return;
    }

    let ChannelElement {
        ch: [SingleChannelElement { ics, .. }, _],
        ms_mask,
        ..
    } = cpe;

    for WindowedIteration { w, .. } in ics.iter_windows() {
        for &ms_mask in ms_mask[W(w)].iter().take(ics.max_sfb.into()) {
            pb.put(1, ms_mask.into());
        }
    }
}

#[ffmpeg_src(file = "libavcodec/aacenc.c", lines = 527..=578)]
fn adjust_frame_information(cpe: &mut ChannelElement, chans: c_int) {
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
            let zeroes = &zeroes[W2(w)];
            for (g, zero) in zeroes.iter().take(ics.max_sfb.into()).enumerate() {
                zero.set(
                    zeroes
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
            .take(c_uchar::from(ics0.num_windows).into())
            .flat_map(|ms_masks| &ms_masks[..ics0.max_sfb.into()])
            .filter(|&&ms_mask| ms_mask)
            .count();
        *ms_mode = if msc == 0 || ics0.max_sfb == 0 {
            0
        } else if msc < usize::from(ics0.max_sfb) * usize::from(c_uchar::from(ics0.num_windows)) {
            1
        } else {
            2
        };
    }
}

fn encode_band_info(s: &mut AACEncContext, sce: &mut SingleChannelElement, pb: &mut BitWriter) {
    set_special_band_scalefactors(sce);
    for WindowedIteration { w, group_len } in sce.ics.iter_windows() {
        trellis::codebook_rate(s, sce, pb, w, group_len.into());
    }
}

/// preamble for [`NOISE_BT`], put in bitstream with the first noise band
const NOISE_PRE: c_int = 256;
/// length of preamble
const NOISE_PRE_BITS: u8 = 9;
/// subtracted from global gain, used as offset for the preamble
const NOISE_OFFSET: c_int = 90;

#[ffmpeg_src(file = "libavcodec/aac.h", lines = 52)]
const CLIP_AVOIDANCE_FACTOR: c_float = 0.95;

#[ffmpeg_src(file = "libavcodec/aacenc.c", lines = 655..=689)]
fn encode_scale_factors(sce: &mut SingleChannelElement, pb: &mut BitWriter) {
    let SingleChannelElement {
        ref sf_idx,
        ics: ref ics @ IndividualChannelStream { max_sfb, .. },
        ref zeroes,
        ref band_type,
        ..
    } = *sce;

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

            let mut diff = sf_idx - *offset;
            *offset = sf_idx;

            if band_type == NOISE_BT && mem::take(&mut noise_flag) {
                pb.put(NOISE_PRE_BITS, (diff + NOISE_PRE) as _);
                continue;
            }

            diff += SCALE_DIFF_ZERO as c_int;
            pb.put(
                SCALEFACTOR_BITS[diff as usize],
                SCALEFACTOR_CODE[diff as usize] as _,
            );
        }
    }
}

/// Encode pulse data.
#[ffmpeg_src(file = "libavcodec/aacenc.c", lines = 691..=708)]
fn encode_pulses(pb: &mut BitWriter, pulse: &Pulse) {
    let Pulse {
        num_pulse,
        start,
        ref pos,
        ref amp,
    } = *pulse;

    pb.put(1, (num_pulse != 0).into());
    if num_pulse == 0 {
        return;
    }
    pb.put(2, (num_pulse - 1) as BitBuf);
    pb.put(6, start as BitBuf);
    for (&pos, &amp) in zip(pos, amp).take(num_pulse as usize) {
        pb.put(5, pos as BitBuf);
        pb.put(4, amp as BitBuf);
    }
}

#[ffmpeg_src(file = "libavcodec/aacenc.c", lines = 710..=736)]
fn encode_spectral_coeffs(s: &mut AACEncContext, sce: &SingleChannelElement, pb: &mut BitWriter) {
    let lambda = s.lambda;

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
            let coeffs = &coeffs[W2(w)];
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
            .take(c_uchar::from(num_windows).into())
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

fn encode_individual_channel(
    s: &mut AACEncContext,
    sce: &mut SingleChannelElement,
    pb: &mut BitWriter,
    common_window: c_int,
) -> c_int {
    pb.put(8, sce.sf_idx[W(0)][0] as _);
    if common_window == 0 {
        put_ics_info(pb, &sce.ics);
    }
    encode_band_info(s, sce, pb);
    encode_scale_factors(sce, pb);
    encode_pulses(pb, &sce.pulse);
    pb.put(1, sce.tns.present.into());
    tns::encode_info(pb, sce);
    pb.put(1, 0);
    encode_spectral_coeffs(s, sce, pb);
    0
}

fn put_bitstream_info(pb: &mut BitWriter, name: &CStr) {
    let namelen = name.to_bytes().len().wrapping_add(2) as c_int;
    pb.put(3, (SyntaxElementType::FillElement as u8).into());
    pb.put(4, (if namelen > 15 { 15 } else { namelen }) as _);
    if namelen >= 15 {
        pb.put(8, (namelen - 14) as _);
    }
    pb.put(4, 0);
    let padbits = -(pb.bits_written() as c_int) & 7;
    pb.align();
    let mut i = 0;
    while i < namelen - 2 {
        pb.put(8, name.to_bytes()[i as usize].into());
        i += 1;
    }
    pb.put(12 - padbits as u8, 0);
}

/// Copy input samples.
///
/// Channels are reordered from libavcodec's default order to AAC order.
#[ffmpeg_src(file = "libavcodec/aacenc.c", lines = 804..=828)]
fn copy_input_samples(s: &mut AACEncContext, frame: Option<&Frame>) {
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
        let ([_, mid, end], []) = planar_samples.as_chunks_mut::<1024>() else {
            unreachable!();
        };

        // copy last 1024 samples of previous frame to the start of the current frame
        *mid = *end;

        let mut end = end.as_mut_slice();

        // copy new samples...
        if let Some(frame) = frame {
            unsafe {
                let extended_data = frame.get_extended_data_unchecked::<c_float>(reorder.into());

                let ext;
                (ext, end) = end.split_at_mut_unchecked(extended_data.len());

                ptr::copy_nonoverlapping(
                    extended_data.as_ptr(),
                    ext.as_mut_ptr(),
                    extended_data.len(),
                );
            }
        }

        // ...and zero any remaining samples
        end.fill(0.);
    }
}

impl IndividualChannelStream {
    #[ffmpeg_src(file = "libavcodec/aacenc.c", lines = 893..=909)]
    fn apply_psy_window_info(
        &mut self,
        tag: SyntaxElementType,
        &PsyWindowInfo {
            window_type: [window_type, ..],
            window_shape,
            num_windows,
            grouping,
            ..
        }: &PsyWindowInfo,
        psy: &PsyContext,
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
            psy.num_bands[usize::from(num_windows == WindowCount::Eight)]
        };

        *self = IndividualChannelStream {
            window_sequence: [window_type as WindowSequence, window_sequence],
            use_kb_window: [window_shape == WindowShape::Kbd, use_kb_window],
            num_windows,
            swb_sizes: psy.bands[usize::from(num_windows == WindowCount::Eight)],
            num_swb,
            max_sfb: max_sfb.min(num_swb as c_uchar),
            group_len: grouping,
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
        let wlen = 2048 / c_int::from(c_uchar::from(self.num_windows));

        let (clipping, _) = MaybeUninit::fill_from(
            &mut clipping,
            WindowedArray::<_, 128>::from_ref(overlap)
                .into_iter()
                .take(c_uchar::from(self.num_windows).into())
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

fn analyze_psy_windows(
    ctx: &mut AACEncContext,
    cpe: &mut Box<[ChannelElement]>,
    frame: Option<&Frame>,
    windows: &mut [PsyWindowInfo; 16],
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
            let la = (frame.is_some()).then(|| &overlap[1024 + 448 + 64..]);
            if tag == SyntaxElementType::LowFrequencyEffects {
                wi.window_type[0] = WindowSequence::OnlyLong;
                wi.window_type[1] = WindowSequence::OnlyLong;
                wi.window_shape = WindowShape::Sine;
                wi.num_windows = WindowCount::One;
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
    avctx: &CodecContext,
    ctx: &mut AACEncContext,
    cpe: &mut Box<[ChannelElement]>,
    frame: Option<&Frame>,
    packet_builder: PacketBuilder,
) -> c_int {
    let mut ms_mode = false;
    let mut is_mode = false;
    let mut tns_mode = false;
    let mut pred_mode = false;
    let mut windows = [PsyWindowInfo::default(); 16];

    if let Some(frame) = frame {
        ctx.afq.add_frame(avctx, frame)
    } else if ctx.afq.is_empty() {
        return 0;
    }

    copy_input_samples(ctx, frame);
    if avctx.frame_num().get() == 0 {
        return 0;
    }

    analyze_psy_windows(ctx, cpe, frame, &mut windows);

    let mut avpkt = packet_builder.allocate((8192 * ctx.channels) as c_long);

    let mut pb = avpkt.bit_writer();
    for iterations in 0.. {
        pb.clear();
        if avctx.frame_num().get() & 0xff == 1 && !avctx.flags().get().bit_exact() {
            put_bitstream_info(&mut pb, c"Lavc60.33.100");
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

            pb.put(3, tag as _);
            let chan_el_counter = &mut chan_el_counter[tag as usize];
            pb.put(4, *chan_el_counter);
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
            ctx.psy.analyze(avctx, start_ch, &coeffs, windows);
            if ctx.psy.bitres.alloc > 0 {
                target_bits = (target_bits as c_float
                    + ctx.psy.bitres.alloc as c_float
                        * (ctx.lambda
                            / (if avctx.global_quality().get() != 0 {
                                avctx.global_quality().get()
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
                    .take(c_uchar::from(wi0.num_windows).into())
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
                if sce.tns.present {
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
                pb.put(1, cpe.common_window as _);
                if cpe.common_window != 0 {
                    put_ics_info(&mut pb, &cpe.ch[0].ics);

                    encode_ms_info(&mut pb, cpe);
                    if cpe.ms_mode != 0 {
                        ms_mode = true;
                    }
                }
            }
            for (ch, sce) in cpe.ch.iter_mut().take(windows.len()).enumerate() {
                ctx.cur_channel = start_ch + ch as c_int;
                encode_individual_channel(ctx, sce, &mut pb, cpe.common_window);
            }
            start_ch += windows.len() as c_int;
        }

        if avctx.flags().get().qscale() {
            // When using a constant Q-scale, don't mess with lambda
            break;
        }

        // rate control stuff
        // allow between the nominal bitrate, and what psy's bit reservoir says to
        // target but drift towards the nominal bitrate always
        let frame_bits = pb.bits_written() as c_int;
        let rate_bits = ((avctx.bit_rate().get() * 1024 / avctx.sample_rate().get() as c_long)
            as c_int)
            .min(6144 * ctx.channels - 3);
        let mut too_many_bits = target_bits.max(rate_bits).min(6144 * ctx.channels - 3);
        let mut too_few_bits = (rate_bits - rate_bits / 4)
            .max(target_bits)
            .min(too_many_bits);

        // When strict bit-rate control is demanded
        if avctx.bit_rate_tolerance().get() == 0 {
            if rate_bits < frame_bits {
                let ratio = rate_bits as c_float / frame_bits as c_float;
                ctx.lambda *= ratio.min(0.9);
            }

            // reset lambda when solution is found
            ctx.lambda = if avctx.global_quality().get() > 0 {
                avctx.global_quality().get()
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

    pb.put(3, (SyntaxElementType::End as u8).into());
    pb.flush();
    ctx.last_frame_pb_count = pb.bits_written() as c_int;
    let bytes_output = pb.total_bytes_written();
    avpkt.truncate(bytes_output);
    ctx.lambda_sum += ctx.lambda;
    ctx.lambda_count += 1;

    {
        let AudioRemoved { pts, duration } = ctx.afq.remove(avctx);
        avpkt.pts().set(pts);
        avpkt.duration().set(duration);
    }

    0
}

const ENCODE_DEFAULTS: [FFCodecDefault; 2] =
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
    const DEFAULTS: &'static [FFCodecDefault] = &ENCODE_DEFAULTS;

    type Ctx = (AACEncContext, Box<[ChannelElement]>);
    type Options = AACEncOptions;

    fn init(avctx: &mut CodecContext, options: &Self::Options) -> Box<Self::Ctx> {
        unsafe {
            avctx.frame_size().set(1024);
            avctx.initial_padding().set(1024);
            let ch_layout = addr_of!((*avctx.as_ptr()).ch_layout);

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
                .find_position(|&&sample_rate| avctx.sample_rate().get() == sample_rate)
                .map(|(i, _)| i)
                .filter(|&i| i < SWB_SIZE_1024.len() && i < SWB_SIZE_128.len())
                .unwrap_or_else(|| panic!("Unsupported sample rate {}", avctx.sample_rate().get()))
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
                    avctx.as_ptr().cast(),
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
                    lpc: LPCContext::new(2 * avctx.frame_size().get(), 20, lpc::Type::Levinson),
                    samplerate_index,
                    channels: (*ch_layout).nb_channels,
                    reorder_map,
                    chan_map,
                    psy: PsyContext::default(),
                    cur_channel: 0,
                    random_state: 0x1f2e3d4c,
                    lambda: if avctx.global_quality().get() > 0 {
                        avctx.global_quality().get() as c_float
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

            if avctx.bit_rate().get() == 0 {
                avctx.bit_rate().set(
                    ctx.chan_map
                        .iter()
                        .map(|&tag| match tag {
                            SyntaxElementType::ChannelPairElement => 128000,
                            SyntaxElementType::LowFrequencyEffects => 16000,
                            _ => 69000,
                        })
                        .sum(),
                );
            }

            if 1024. * avctx.bit_rate().get() as c_double / avctx.sample_rate().get() as c_double
                > (6144 * ctx.channels) as c_double
            {
                av_log(
                    avctx.as_ptr().cast(),
                    24,
                    c"Too many bits %f > %d per frame requested, clamping to max\n".as_ptr(),
                    1024. * avctx.bit_rate().get() as c_double
                        / avctx.sample_rate().get() as c_double,
                    6144 * ctx.channels,
                );
            }
            (*avctx).bit_rate().set(
                (((6144 * ctx.channels) as c_double / 1024. * avctx.sample_rate().get() as c_double)
                    as c_long)
                    .min(avctx.bit_rate().get()),
            );

            avctx
                .profile()
                .set(if avctx.profile().get() == profile::UNKNOWN {
                    profile::AAC_LOW
                } else {
                    avctx.profile().get()
                });

            if avctx.profile().get() == profile::MPEG2_AAC_LOW {
                avctx.profile().set(profile::AAC_LOW);
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
                        avctx.as_ptr().cast(),
                        24,
                        c"PNS unavailable in the \"mpeg2_aac_low\" profile, turning off\n".as_ptr(),
                    );
                }
                ctx.options.pns = 0;
            } else if avctx.profile().get() == profile::AAC_LTP {
                ctx.options.ltp = 1;
                assert_eq!(
                    ctx.options.pred, 0,
                    "Main prediction unavailable in the \"aac_ltp\" profile"
                );
            } else if avctx.profile().get() == profile::AAC_MAIN {
                ctx.options.pred = 1;
                assert_eq!(
                    ctx.options.ltp, 0,
                    "LTP prediction unavailable in the \"aac_main\" profile"
                );
            } else if ctx.options.ltp != 0 {
                avctx.profile().set(profile::AAC_LTP);
                av_log(
                    avctx.as_ptr().cast(),
                    24,
                    c"Chainging profile to \"aac_ltp\"\n".as_ptr(),
                );
                assert_eq!(
                    ctx.options.pred, 0,
                    "Main prediction unavailable in the \"aac_ltp\" profile"
                );
            } else if ctx.options.pred != 0 {
                avctx.profile().set(profile::AAC_MAIN);
                av_log(
                    avctx.as_ptr().cast(),
                    24,
                    c"Chainging profile to \"aac_main\"\n".as_ptr(),
                );
                assert_eq!(
                    ctx.options.ltp, 0,
                    "LTP prediction unavailable in the \"aac_main\" profile"
                );
            }
            ctx.profile = avctx.profile().get();
            let experimental = avctx.strict_std_compliance().get() <= -2;
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

            dsp::init(ctx).expect("dsp::init failed");
            put_audio_specific_config(avctx, ctx);

            let sizes = [
                SWB_SIZE_1024[ctx.samplerate_index as usize],
                SWB_SIZE_128[ctx.samplerate_index as usize],
            ];
            let lengths = [
                c_int::from(NUM_SWB_1024[ctx.samplerate_index as usize]),
                c_int::from(NUM_SWB_128[ctx.samplerate_index as usize]),
            ];
            let grouping = array::from_fn(|i| {
                ctx.chan_map
                    .get(i)
                    .map(|&tag| (tag == SyntaxElementType::ChannelPairElement).into())
                    .unwrap_or_default()
            });

            ctx.psy = PsyContext::init(
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
        avctx: &CodecContext,
        ctx: &mut Self::Ctx,
        _: &Self::Options,
        frame: Option<&Frame>,
        packet_builder: PacketBuilder,
    ) {
        let (ctx, cpe) = &mut *ctx;
        let ret = unsafe { aac_encode_frame(avctx, ctx, cpe, frame, packet_builder) };
        assert!(ret >= 0, "aac_encode_frame failed");
    }

    fn close(avctx: &mut CodecContext, mut ctx: Box<Self::Ctx>) {
        // TODO(yotam): de-dup
        const MAX_SIZE: usize = 32;

        unsafe {
            let _ = Box::from_raw(ptr::slice_from_raw_parts_mut(
                avctx.extradata().get(),
                MAX_SIZE,
            ));
            avctx.extradata().set(null_mut());
            avctx.extradata_size().set(0);
        }

        let (ctx, _) = &mut *ctx;
        unsafe {
            av_log(
                avctx.as_ptr().cast(),
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
