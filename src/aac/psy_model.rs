use std::{
    array,
    cell::Cell,
    iter::{successors, zip},
    mem,
};

use array_util::{Array, WindowedArray};
use encoder::CodecContext;
use ffmpeg_src_macro::ffmpeg_src;
use izip::izip;
use libc::{c_double, c_float, c_int, c_long, c_uchar, c_ushort};
use reductor::{Reduce as _, Sum};

use super::WindowSequence;
use crate::{common::*, types::*};

/// -1dB
#[ffmpeg_src(file = "libavcodec/aacpsy.c", lines = 64, name = "PSY_SNR_1DB")]
const SNR_1DB: c_float = 7.943_282e-1;

/// -25dB
#[ffmpeg_src(file = "libavcodec/aacpsy.c", lines = 65, name = "PSY_SNR_25DB")]
const SNR_25DB: c_float = 3.162_277_6e-3;

/// long block size
#[ffmpeg_src(file = "libavcodec/aacpsy.c", lines = 96, name = "AAC_BLOCK_SIZE_LONG")]
const BLOCK_SIZE_LONG: c_ushort = 1024;

/// spreading factor for low-to-hi energy spreading, long block, >
/// 22kbps/channel (20dB/Bark)
#[ffmpeg_src(file = "libavcodec/aacpsy.c", lines = 46..=47, name = "PSY_3GPP_EN_SPREAD_HI_L1")]
const EN_SPREAD_HI_L1: c_float = 2.;
/// spreading factor for low-to-hi energy spreading, short block (15 dB/Bark)
#[ffmpeg_src(file = "libavcodec/aacpsy.c", lines = 50..=51, name = "PSY_3GPP_EN_SPREAD_HI_S")]
const EN_SPREAD_HI_S: c_float = 1.5;
/// spreading factor for hi-to-low energy spreading, long block (30dB/Bark)
#[ffmpeg_src(file = "libavcodec/aacpsy.c", lines = 52..=53, name = "PSY_3GPP_EN_SPREAD_LOW_L")]
const EN_SPREAD_LOW_L: c_float = 3.;
/// spreading factor for hi-to-low energy spreading, short block (20dB/Bark)
#[ffmpeg_src(file = "libavcodec/aacpsy.c", lines = 54..=55, name = "PSY_3GPP_EN_SPREAD_LOW_S")]
const EN_SPREAD_LOW_S: c_float = 2.;

trait Bits {
    fn bits_to_pe(self) -> Self;
    fn pe_to_bits(self) -> Self;
}

impl Bits for c_float {
    #[ffmpeg_src(file = "libavcodec/aacpsy.c", lines = 91, name = "PSY_3GPP_BITS_TO_PE")]
    fn bits_to_pe(self) -> Self {
        self * 1.18
    }

    #[ffmpeg_src(file = "libavcodec/aacpsy.c", lines = 92, name = "PSY_3GPP_PE_TO_BITS")]
    fn pe_to_bits(self) -> Self {
        self / 1.18
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum AvoidHoles {
    Active,
    Inactive,
    #[default]
    None,
}

/// information for single band used by 3GPP TS26.403-inspired psychoacoustic
/// model
#[ffmpeg_src(file = "libavcodec/aacpsy.c", lines = 105..=118, name = "AacPsyBand")]
#[derive(Copy, Clone, Default)]
struct Band {
    /// band energy
    energy: c_float,
    /// energy threshold
    threshold: c_float,
    /// threshold in quiet
    thr_quiet: c_float,
    /// number of non-zero spectral lines
    nz_lines: c_float,
    /// number of active spectral lines
    active_lines: c_float,
    /// perceptual entropy
    pe: c_float,
    /// constant part of the PE calculation
    pe_const: c_float,
    /// normalization factor for linearization
    norm_fac: c_float,
    /// hole avoidance flag
    avoid_holes: AvoidHoles,
}

/// single/pair channel context for psychoacoustic model
#[ffmpeg_src(file = "libavcodec/aacpsy.c", lines = 123..=135, name = "AacPsyChannel")]
#[derive(Copy, Clone, Default)]
struct Channel {
    /// bands information
    band: WindowedArray<Array<Band, 128>, 16>,
    /// bands information from the previous frame
    prev_band: WindowedArray<Array<Band, 128>, 16>,
    /// stored grouping scheme for the next frame (in case of 8 short window
    /// sequence)
    next_grouping: c_uchar,
    /// window sequence to be used in the next frame
    next_window_seq: WindowSequence,
    /// attack threshold for this channel
    attack_threshold: c_float,
    prev_energy_subshort: [c_float; 24],
    /// attack value for the last short block in the previous sequence
    prev_attack: c_int,
}

/// psychoacoustic model frame type-dependent coefficients
#[ffmpeg_src(file = "libavcodec/aacpsy.c", lines = 137..=146, name = "AacPsyCoeffs")]
#[derive(Copy, Clone, Default)]
struct Coeffs {
    /// absolute threshold of hearing per bands
    ath: c_float,
    /// Bark value for each spectral band in long frame
    barks: c_float,
    /// spreading factor for low-to-high threshold spreading in long frame
    spread_low: [c_float; 2],
    /// spreading factor for high-to-low threshold spreading in long frame
    spread_hi: [c_float; 2],
    /// minimal SNR
    min_snr: c_float,
}

/// 3GPP TS26.403-inspired psychoacoustic model specific data
#[ffmpeg_src(file = "libavcodec/aacpsy.c", lines = 148..=164)]
#[derive(Clone, Default)]
pub(crate) struct AacPsyContext {
    /// bitrate per channel
    chan_bitrate: c_int,
    pe: PEContext,
    psy_coef: [Array<Coeffs, 64>; 2],
    ch: Box<[Channel]>,
}

/// (yotam): Split out from [`AacPsyContext`] to appease borrow checker
#[derive(Copy, Clone, Default)]
struct PEContext {
    /// average bits per frame
    frame_bits: c_int,
    /// bit reservoir fill level
    fill_level: c_int,
    pe: PEState,
}

/// Perceptual entropy state
#[derive(Copy, Clone, Default)]
struct PEState {
    /// minimum allowed PE for bit factor calculation
    min: c_float,
    /// maximum allowed PE for bit factor calculation
    max: c_float,
    /// allowed PE of the previous frame
    previous: c_float,
}

/// LAME psy model preset struct
#[ffmpeg_src(file = "libavcodec/aacpsy.c", lines = 166..=175, name = "PsyLamePreset")]
#[derive(Copy, Clone)]
struct LamePreset {
    /// Quality to map the rest of the vaules to.
    ///
    /// This is overloaded to be both kbps per channel in ABR mode, and
    /// requested quality in constant quality mode.
    quality: c_int,
    /// short threshold for L, R, and M channels
    st_lrm: c_float,
}

impl LamePreset {
    const fn new(quality: c_int, st_lrm: c_float) -> Self {
        Self { quality, st_lrm }
    }
}

/// LAME psy model preset table for ABR
#[ffmpeg_src(file = "libavcodec/aacpsy.c", lines = 177..=196, name = "psy_abr_map")]
static ABR_MAP: [LamePreset; 13] = [
    LamePreset::new(8, 6.60),
    LamePreset::new(16, 6.60),
    LamePreset::new(24, 6.60),
    LamePreset::new(32, 6.60),
    LamePreset::new(40, 6.60),
    LamePreset::new(48, 6.60),
    LamePreset::new(56, 6.60),
    LamePreset::new(64, 6.40),
    LamePreset::new(80, 6.00),
    LamePreset::new(96, 5.60),
    LamePreset::new(112, 5.20),
    LamePreset::new(128, 5.20),
    LamePreset::new(160, 5.20),
];

/// LAME psy model preset table for constant quality
#[ffmpeg_src(file = "libavcodec/aacpsy.c", lines = 198..=214, name = "psy_vbr_map")]
static VBR_MAP: [LamePreset; 11] = [
    LamePreset::new(0, 4.20),
    LamePreset::new(1, 4.20),
    LamePreset::new(2, 4.20),
    LamePreset::new(3, 4.20),
    LamePreset::new(4, 4.20),
    LamePreset::new(5, 4.20),
    LamePreset::new(6, 4.20),
    LamePreset::new(7, 4.20),
    LamePreset::new(8, 4.20),
    LamePreset::new(9, 4.20),
    LamePreset::new(10, 4.20),
];

/// LAME psy model FIR coefficient table
#[ffmpeg_src(file = "libavcodec/aacpsy.c", lines = 216..=223, name = "psy_fir_coeffs")]
static FIR_COEFFS: [c_float; 10] = [
    -8.65163e-18 * 2.,
    -0.00851586 * 2.,
    -6.74764e-18 * 2.,
    0.0209036 * 2.,
    -3.36639e-17 * 2.,
    -0.0438162 * 2.,
    -1.54175e-17 * 2.,
    0.0931738 * 2.,
    -5.52212e-17 * 2.,
    -0.313819 * 2.,
];

#[ffmpeg_src(file = "libavcodec/psymodel.h", lines = 41..=45, name = "AAC_CUTOFF")]
fn cutoff(ctx: &CodecContext) -> c_int {
    if ctx.flags().get().qscale() {
        ctx.sample_rate().get() / 2
    } else {
        cutoff_from_bitrate(
            ctx.bit_rate().get().try_into().unwrap(),
            ctx.ch_layout().get().nb_channels,
            ctx.sample_rate().get(),
        )
    }
}

/// cutoff for VBR is purposely increased, since LP filtering actually
/// hinders VBR performance rather than the opposite
#[ffmpeg_src(file = "libavcodec/psymodel.h", lines = 35..=40, name = "AAC_CUTOFF_FROM_BITRATE")]
pub(crate) fn cutoff_from_bitrate(bit_rate: c_int, channels: c_int, sample_rate: c_int) -> c_int {
    if bit_rate == 0 {
        return sample_rate / 2;
    }

    (bit_rate / channels / 5)
        .max(bit_rate / channels * 15 / 32 - 5500)
        .min(3000 + bit_rate / channels / 4)
        .min(12000 + bit_rate / channels / 16)
        .min(22000)
        .min(sample_rate / 2)
}

/// Calculate the ABR attack threshold from the above LAME psymodel table.
#[ffmpeg_src(file = "libavcodec/aacpsy.c", lines = 229..=257)]
fn lame_calc_attack_threshold(bitrate: c_int) -> c_float {
    let mut lower_range: c_int = 12;
    let mut upper_range: c_int = 12;
    let mut lower_range_kbps: c_int = ABR_MAP[12].quality;
    let mut upper_range_kbps: c_int = ABR_MAP[12].quality;
    let mut i = 1;
    while i < 13 {
        if (if bitrate > ABR_MAP[i as usize].quality {
            bitrate
        } else {
            ABR_MAP[i as usize].quality
        }) != bitrate
        {
            upper_range = i;
            upper_range_kbps = ABR_MAP[i as usize].quality;
            lower_range = i - 1;
            lower_range_kbps = ABR_MAP[(i - 1) as usize].quality;
            break;
        } else {
            i += 1;
        }
    }
    if upper_range_kbps - bitrate > bitrate - lower_range_kbps {
        return ABR_MAP[lower_range as usize].st_lrm;
    }
    ABR_MAP[upper_range as usize].st_lrm
}

#[cold]
fn lame_window_init(avctx: &CodecContext) -> Channel {
    Channel {
        attack_threshold: if avctx.flags().get().qscale() {
            VBR_MAP[(avctx.global_quality().get() / 118).clamp(0, 10) as usize].st_lrm
        } else {
            lame_calc_attack_threshold(
                (avctx.bit_rate().get() / c_long::from(avctx.ch_layout().get().nb_channels) / 1000)
                    as c_int,
            )
        },
        prev_energy_subshort: [10.; _],
        ..Default::default()
    }
}

#[cold]
fn calc_bark(f: c_float) -> c_float {
    13.3 * (0.00076 * f).atan() + 3.5 * (f / 7500. * (f / 7500.)).atan()
}

#[ffmpeg_src(file = "libavcodec/aacpsy.c", lines = 287)]
const ATH_ADD: c_float = 4.;

#[cold]
/// Calculate ATH (Absolute Threshold of Hearing) value for given frequency.
/// Borrowed from Lame.
fn ath(f: c_float, add: c_float) -> c_float {
    let f = c_double::from(f / 1000.);
    let add = c_double::from(add);
    (3.64 * f.powf(-0.8) - 6.8 * (-0.6 * (f - 3.4).powi(2)).exp()
        + 6. * (-0.15 * (f - 8.7).powi(2)).exp()
        + (0.6 + 0.04 * add) * 0.001 * f.powi(4)) as c_float
}

trait ChannelBitrate {
    fn channel_bitrate(&self) -> c_int;
}

impl ChannelBitrate for CodecContext {
    #[ffmpeg_src(file = "libavcodec/aacpsy.c", lines = 306)]
    fn channel_bitrate(&self) -> c_int {
        let bit_rate = self.bit_rate().get();
        let flags = self.flags().get();
        let nb_channels = self.ch_layout().get().nb_channels;
        let global_quality = self.global_quality().get();

        let mut chan_bitrate = (bit_rate as c_float
            / if flags.qscale() {
                2.
            } else {
                nb_channels as c_float
            }) as c_int;

        if flags.qscale() {
            // Use the target average bitrate to compute spread parameters
            chan_bitrate = (chan_bitrate as c_double / 120.
                * if global_quality != 0 {
                    global_quality as c_double
                } else {
                    120.
                }) as c_int;
        }

        chan_bitrate
    }
}

impl AacPsyContext {
    #[cold]
    #[ffmpeg_src(file = "libavcodec/aacpsy.c", lines = 301..=382, name = "psy_3gpp_init")]
    pub(crate) fn init(avctx: &CodecContext, ctx: &mut FFPsyContext) -> Self {
        let bandwidth = if ctx.cutoff != 0 {
            ctx.cutoff
        } else {
            cutoff(avctx)
        };
        assert!(bandwidth > 0);

        let sample_rate = avctx.sample_rate().get();

        let chan_bitrate = avctx.channel_bitrate();
        let frame_bits = 2560.min(chan_bitrate * c_int::from(BLOCK_SIZE_LONG) / sample_rate);

        ctx.bitres.size = 6144 - frame_bits;
        ctx.bitres.size -= ctx.bitres.size % 8;

        let psy_coeffs = {
            let mut coeffs = [Array([Coeffs::default(); 64]); 2];
            let num_bark = calc_bark(bandwidth as c_float);

            let minath = ath((3410. - 0.733 * ATH_ADD) as c_float, ATH_ADD);
            for (j, (coeffs, &band_sizes, &num_bands)) in
                izip!(&mut coeffs, &*ctx.bands, &*ctx.num_bands)
                    .take(2)
                    .enumerate()
            {
                let line_to_frequency = sample_rate as c_float / if j != 0 { 256. } else { 2048. };
                let avg_chan_bits = chan_bitrate as c_float * if j != 0 { 128. } else { 1024. }
                    / sample_rate as c_float;
                // reference encoder uses 2.4% here instead of 60% like the spec says
                let bark_pe = 0.024 * avg_chan_bits.bits_to_pe() / num_bark;
                let en_spread_low = if j != 0 {
                    EN_SPREAD_LOW_S
                } else {
                    EN_SPREAD_LOW_L
                };
                // High energy spreading for long blocks <= 22kbps/channel and short blocks are
                // the same.
                let en_spread_hi = if j != 0 || chan_bitrate as c_float <= 22. {
                    EN_SPREAD_HI_S
                } else {
                    EN_SPREAD_HI_L1
                };

                {
                    let mut i = 0;
                    let mut prev = 0.;
                    for (&band_size, coeff) in
                        zip(band_sizes, &mut *coeffs).take(num_bands as usize)
                    {
                        i += c_int::from(band_size);
                        let bark = calc_bark((i - 1) as c_float * line_to_frequency);
                        coeff.barks = ((bark + prev) as c_double / 2.) as c_float;
                        prev = bark;
                    }
                }

                {
                    let coeffs = Cell::from_mut(&mut **coeffs).as_array_of_cells();
                    for ([coeff0, coeff1], &band_size) in
                        zip(coeffs.array_windows(), band_sizes).take(num_bands as usize - 1)
                    {
                        let bark_width: c_float = coeff1.get().barks - coeffs[0].get().barks;
                        coeff0.update(|coeff| Coeffs {
                            spread_low: [-bark_width * 3., -bark_width * en_spread_low]
                                .map(Exp10::exp10),
                            spread_hi: [-bark_width * 1.5, -bark_width * en_spread_hi]
                                .map(Exp10::exp10),
                            min_snr: {
                                let pe_min = bark_pe * bark_width;
                                let minsnr = (pe_min / c_float::from(band_size)).exp2() - 1.5;
                                minsnr.recip().clamp(SNR_25DB, SNR_1DB)
                            },
                            ..coeff
                        });
                    }
                }

                {
                    let mut start = 0;
                    for (coeff, &band_size) in
                        zip(&mut *coeffs, band_sizes).take(num_bands as usize)
                    {
                        let minscale = (0..c_int::from(band_size))
                            .map(|i| ath((start + i) as c_float * line_to_frequency, 4.))
                            .min_by(c_float::total_cmp)
                            .unwrap();
                        coeff.ath = minscale - minath;
                        start += band_size as c_int;
                    }
                }
            }

            coeffs
        };

        Self {
            chan_bitrate,
            pe: PEContext {
                frame_bits,
                pe: {
                    let pe_state = |c| {
                        c * c_float::from(BLOCK_SIZE_LONG) * bandwidth as c_float
                            / (sample_rate as c_float * 2.)
                    };
                    PEState {
                        min: pe_state(8.),
                        max: pe_state(12.),
                        ..Default::default()
                    }
                },
                fill_level: ctx.bitres.size,
            },
            psy_coef: psy_coeffs,
            ch: vec![lame_window_init(avctx); avctx.ch_layout().get().nb_channels as usize]
                .into_boxed_slice(),
        }
    }
}

const WINDOW_GROUPING: [c_uchar; 9] = [0xb6, 0x6c, 0xd8, 0xb2, 0x66, 0xc6, 0x96, 0x36, 0x36];

impl PEContext {
    /// 5.6.1.2 "Calculation of Bit Demand"
    #[ffmpeg_src(file = "libavcodec/aacpsy.c", lines = 493..=535)]
    fn calc_bit_demand(
        &mut self,
        pe: c_float,
        bits: c_int,
        size: c_int,
        short_window: bool,
    ) -> c_int {
        #[derive(Default)]
        struct SlopeAdd {
            slope: c_float,
            add: c_float,
        }

        #[derive(Default)]
        struct Info {
            bit_save: SlopeAdd,
            bit_spend: SlopeAdd,
            clip_low: c_float,
            clip_high: c_float,
        }

        fn get_info(short_window: bool) -> Info {
            Info {
                clip_low: 0.2,
                ..if short_window {
                    Info {
                        bit_save: SlopeAdd {
                            slope: -0.363_636_37,
                            add: -0.75,
                        },
                        bit_spend: SlopeAdd {
                            slope: 0.818_181_8,
                            add: -0.261_111_1,
                        },
                        clip_high: 0.75,
                        ..Default::default()
                    }
                } else {
                    Info {
                        bit_save: SlopeAdd {
                            slope: -0.466_666_67,
                            add: -0.842_857_1,
                        },
                        bit_spend: SlopeAdd {
                            slope: 0.666_666_7,
                            add: -0.35,
                        },
                        clip_high: 0.95,
                        ..Default::default()
                    }
                }
            }
        }

        let Info {
            bit_save:
                SlopeAdd {
                    slope: bitsave_slope,
                    add: bitsave_add,
                },
            bit_spend:
                SlopeAdd {
                    slope: bitspend_slope,
                    add: bitspend_add,
                },
            clip_low,
            clip_high,
        } = get_info(short_window);

        self.fill_level += self.frame_bits - bits;
        self.fill_level = self.fill_level.clamp(0, size);
        let fill_level = (self.fill_level as c_float / size as c_float).clamp(clip_low, clip_high);
        let clipped_pe = pe.clamp(self.pe.min, self.pe.max);
        let bit_save = (fill_level + bitsave_add) * bitsave_slope;
        let bit_spend = (fill_level + bitspend_add) * bitspend_slope;
        // The bit factor graph in the spec is obviously incorrect.
        //      bit_spend + ((bit_spend - bit_spend))...
        // The reference encoder subtracts everything from 1, but also seems incorrect.
        //      1 - bit_save + ((bit_spend + bit_save))...
        // Hopefully below is correct.
        let bit_factor = 1. - bit_save
            + (bit_spend - bit_save) / (self.pe.max - self.pe.min) * (clipped_pe - self.pe.min);
        // NOTE: The reference encoder attempts to center pe max/min around the current
        // pe. Here we do that by slowly forgetting pe.min when pe stays in a range that
        // makes it unlikely (ie: above the mean)
        self.pe.max = pe.max(self.pe.max);
        let forgetful_min_pe =
            (self.pe.min * 511. + self.pe.min.max(pe * (pe / self.pe.max))) / (511 + 1) as c_float;
        self.pe.min = pe.min(forgetful_min_pe);

        // NOTE: allocate a minimum of 1/8th average frame bits, to avoid
        // reservoir starvation from producing zero-bit frames
        (self.frame_bits as c_float * bit_factor)
            .min((self.frame_bits + size - bits).max(self.frame_bits / 8) as c_float)
            as c_int
    }
}

fn calc_pe_3gpp(band: &mut Band) -> c_float {
    band.pe = 0.;
    band.pe_const = 0.;
    band.active_lines = 0.;
    if band.energy > band.threshold {
        let mut a = band.energy.log2();
        let mut pe = a - band.threshold.log2();
        band.active_lines = band.nz_lines;
        if pe < 3. {
            pe = pe * 0.559_357_3_f32 + 1.3219281;
            a = a * 0.559_357_3_f32 + 1.3219281;
            band.active_lines *= 0.559_357_3_f32;
        }
        band.pe = pe * band.nz_lines;
        band.pe_const = a * band.nz_lines;
    }
    band.pe
}

#[ffmpeg_src(file = "libavcodec/aacpsy.c", lines = 560..=572)]
fn calc_reduction_3gpp(
    a: c_float,
    desired_pe: c_float,
    pe: c_float,
    active_lines: c_float,
) -> c_float {
    if active_lines == 0. {
        return 0.;
    }

    let thr_avg = ((a - pe) / (4. * active_lines)).exp2();
    let reduction = ((a - desired_pe) / (4. * active_lines)).exp2() - thr_avg;

    reduction.max(0.)
}

impl Band {
    #[ffmpeg_src(file = "libavcodec/aacpsy.c", lines = 574..=597, name = "calc_reduced_thr_3gpp")]
    fn calc_reduced_threshold(&mut self, min_snr: c_float, reduction: c_float) -> c_float {
        let mut thr = self.threshold;
        if self.energy > thr {
            thr = thr.sqrt();
            thr = thr.sqrt() + reduction;
            thr *= thr;
            thr *= thr;
            if thr > self.energy * min_snr && self.avoid_holes != AvoidHoles::None {
                thr = if self.threshold > self.energy * min_snr {
                    self.threshold
                } else {
                    self.energy * min_snr
                };
                self.avoid_holes = AvoidHoles::Active;
            }
        }
        thr
    }
}

#[ffmpeg_src(file = "libavcodec/aacpsy.c", lines = 600..=627)]
fn calc_thr_3gpp(
    wi: &PsyWindowInfo,
    num_bands: c_int,
    pch: &mut Channel,
    band_sizes: &[c_uchar],
    coefs: &[c_float],
    cutoff: c_int,
) {
    let mut start = 0;
    for bands in pch
        .band
        .as_array_of_cells_deref()
        .into_iter()
        .take(c_uchar::from(wi.num_windows).into())
    {
        let mut wstart = 0;
        for (band, &band_size) in zip(bands, band_sizes).take(num_bands as usize) {
            band.update(|mut band| {
                let form_factor: c_float;
                (Sum(band.energy), Sum(form_factor)) = if wstart < cutoff {
                    coefs
                        .iter()
                        .skip(start as usize)
                        .take(band_size.into())
                        .map(|&coef| (coef.powi(2), coef.abs().sqrt()))
                        .reduce_with()
                } else {
                    Default::default()
                };
                band.threshold = band.energy * 0.001258925;
                band.nz_lines = form_factor
                    * if band.energy > 0. {
                        (c_float::from(band_size) / band.energy).sqrt()
                    } else {
                        0.
                    }
                    .sqrt();
                start += c_int::from(band_size);
                wstart += c_int::from(band_size);
                band
            });
        }
    }
}

#[ffmpeg_src(file = "libavcodec/aacpsy.c", lines = 631..=646, name = "psy_hp_filter")]
fn hp_filter(firbuf: &[c_float], psy_fir_coeffs_0: &[c_float; 10]) -> [c_float; 1024] {
    array::from_fn(|i| {
        let firbuf = &firbuf[i..];

        let (Sum::<c_float>(sum1), Sum::<c_float>(sum2)) = izip!(
            psy_fir_coeffs_0.array_chunks(),
            firbuf.array_chunks(),
            firbuf[..22].array_chunks().rev()
        )
        .map(|(&[coeff0, coeff1], &[fir0, fir1], &[rfir0, rfir1])| {
            (coeff0 * (fir0 + rfir1), coeff1 * (fir1 + rfir0))
        })
        .reduce_with();

        // NOTE: The LAME psymodel expects it's input in the range -32768 to 32768.
        //       Tuning this for normalized floats would be difficult.
        (sum1 + firbuf[(21 - 1) / 2] + sum2) * 32768.
    })
}

impl FFPsyContext {
    /// Calculate band thresholds as suggested in 3GPP TS26.403
    #[ffmpeg_src(file = "libavcodec/aacpsy.c", lines = 649..=846, name = "psy_3gpp_analyze_channel")]
    fn analyze_channel(
        &mut self,
        avctx: &CodecContext,
        channel: c_int,
        coefs: &[c_float],
        wi: &PsyWindowInfo,
    ) {
        let pctx = &mut *self.model_priv_data;
        let pch = &mut pctx.ch[channel as usize];
        let ch = &mut self.ch[channel as usize];
        let mut desired_bits: c_float;
        let mut desired_pe: c_float;
        let mut delta_pe: c_float = 0.;
        let mut reduction: c_float = f32::NAN;
        let spread_en = WindowedArray::<_, 16>([const { Cell::new(0.) }; 128]);
        let mut a: c_float = 0.;
        let mut active_lines: c_float = 0.;
        let mut norm_fac: c_float = 0.;
        let mut pe = if pctx.chan_bitrate > 32000 {
            0.
        } else {
            c_float::max(50., 100. - pctx.chan_bitrate as c_float * 100. / 32000.)
        };
        let num_bands = self.num_bands[usize::from(wi.num_windows == WindowCount::Eight)];
        let band_sizes = self.bands[usize::from(wi.num_windows == WindowCount::Eight)];
        let coeffs = &mut pctx.psy_coef[usize::from(wi.num_windows == WindowCount::Eight)];
        let avoid_hole_thr = if wi.num_windows == WindowCount::Eight {
            0.63
        } else {
            0.5
        };
        let bandwidth = if self.cutoff != 0 {
            self.cutoff
        } else {
            cutoff(avctx)
        };
        let cutoff: c_int = bandwidth * 2048
            / c_int::from(c_uchar::from(wi.num_windows))
            / avctx.sample_rate().get();

        // calculate energies, initial thresholds and related values -
        // 5.4.2 "Threshold Calculation"
        calc_thr_3gpp(wi, num_bands, pch, band_sizes, coefs, cutoff);

        // modify thresholds and energies - spread, threshold in quiet, pre-echo control
        for (w, (bands, spread_ens, prev_bands)) in izip!(
            pch.band.as_array_of_cells_deref(),
            &spread_en,
            &pch.prev_band
        )
        .take(c_uchar::from(wi.num_windows).into())
        .enumerate()
        {
            // 5.4.2.3 "Spreading" & 5.4.3 "Spread Energy Calculation"
            (*spread_en)[0].set(bands[0].get().energy);
            for ([band0, band1], [spread_en0, spread_en1], coeff) in izip!(
                bands.array_windows(),
                spread_ens.array_windows(),
                &coeffs[1..num_bands as usize]
            ) {
                band1.update(|band| Band {
                    threshold: c_float::max(
                        band.threshold,
                        band0.get().threshold * coeff.spread_hi[0],
                    ),
                    ..band
                });
                spread_en1.set(c_float::max(
                    band1.get().energy,
                    spread_en0.get() * coeff.spread_hi[1],
                ));
            }
            for ([band0, band1], [spread_en0, spread_en1], coeff) in izip!(
                bands.array_windows(),
                spread_ens.array_windows(),
                coeffs[..num_bands as usize].iter(),
            )
            .rev()
            {
                band0.update(|band| Band {
                    threshold: c_float::max(
                        band.threshold,
                        band1.get().threshold * coeff.spread_low[0],
                    ),
                    ..band
                });
                spread_en0.update(|en| c_float::max(en, spread_en1.get() * coeff.spread_low[1]));
            }

            // 5.4.2.4 "Threshold in quiet"
            for (band, prev_band, coeffs, spread_en) in
                izip!(bands, prev_bands, &*coeffs, spread_ens).take(num_bands as usize)
            {
                band.update(|mut band| {
                    band.threshold = c_float::max(band.threshold, coeffs.ath);
                    band.thr_quiet = band.threshold;

                    // 5.4.2.5 "Pre-echo control"
                    if !(wi.window_type[0] == WindowSequence::LongStop
                        || w == 0 && wi.window_type[1] == WindowSequence::LongStart)
                    {
                        band.threshold = c_float::max(
                            0.01 * band.threshold,
                            c_float::min(band.threshold, 2. * prev_band.thr_quiet),
                        );
                    }

                    // 5.6.1.3.1 "Preparatory steps of the perceptual entropy calculation"
                    pe += calc_pe_3gpp(&mut band);
                    a += band.pe_const;
                    active_lines += band.active_lines;

                    // 5.6.1.3.3 "Selection of the bands for avoidance of holes"
                    band.avoid_holes =
                        if spread_en.get() * avoid_hole_thr > band.energy || coeffs.min_snr > 1. {
                            AvoidHoles::None
                        } else {
                            AvoidHoles::Inactive
                        };

                    band
                });
            }
        }

        // 5.6.1.3.2 "Calculation of the desired perceptual entropy"
        ch.entropy = pe;
        if avctx.flags().get().qscale() {
            // (2.5 * 120) achieves almost transparent rate, and we want to give
            // ample room downwards, so we make that equivalent to QSCALE=2.4
            desired_pe =
                pe * if avctx.global_quality().get() != 0 {
                    avctx.global_quality().get() as c_float
                } else {
                    120.
                } / (2. * 2.5 * 120.);
            desired_bits = c_float::min(2560., desired_pe / 1.18);
            desired_pe = desired_bits.bits_to_pe(); // reflect clipping

            // PE slope smoothing
            if self.bitres.bits > 0 {
                desired_bits = c_float::min(2560., desired_pe.pe_to_bits());
                desired_pe = desired_bits.bits_to_pe();
            }

            pctx.pe.pe.max = pe.max(pctx.pe.pe.max);
            pctx.pe.pe.min = pe.min(pctx.pe.pe.min);
        } else {
            desired_bits = pctx.pe.calc_bit_demand(
                pe,
                self.bitres.bits,
                self.bitres.size,
                wi.num_windows == WindowCount::Eight,
            ) as c_float;
            desired_pe = desired_bits.bits_to_pe();

            // NOTE: PE correction is kept simple. During initial testing it had very
            //       little effect on the final bitrate. Probably a good idea to come
            //       back and do more testing later.
            if self.bitres.bits > 0 {
                desired_pe *= (pctx.pe.pe.previous / (self.bitres.bits as c_float).bits_to_pe())
                    .clamp(0.85, 1.15);
            }
        }
        pctx.pe.pe.previous = desired_bits.bits_to_pe();
        self.bitres.alloc = desired_bits as c_int;

        if desired_pe < pe {
            // 5.6.1.3.4 "First Estimation of the reduction value"
            for bands in pch
                .band
                .as_array_of_cells_deref()
                .into_iter()
                .take(c_uchar::from(wi.num_windows).into())
            {
                reduction = calc_reduction_3gpp(a, desired_pe, pe, active_lines);

                (Sum(pe), Sum(a), Sum(active_lines)) = zip(bands, &*coeffs)
                    .take(num_bands as usize)
                    .map(|(band, coeffs)| {
                        let Band {
                            active_lines,
                            pe,
                            pe_const,
                            ..
                        } = band.update(|mut band| {
                            band.threshold = band.calc_reduced_threshold(coeffs.min_snr, reduction);
                            calc_pe_3gpp(&mut band);
                            band
                        });
                        (pe, pe_const, active_lines)
                    })
                    .reduce_with();
            }

            // 5.6.1.3.5 "Second Estimation of the reduction value"
            for _ in 0..2 {
                let (Sum::<c_float>(pe_no_ah), Sum::<c_float>(a), Sum::<c_float>(active_lines)) =
                    pch.band
                        .as_array_of_cells_deref()
                        .into_iter()
                        .take(c_uchar::from(wi.num_windows).into())
                        .flat_map(|bands| bands.iter().take(num_bands as usize))
                        .map(Cell::get)
                        .filter(|&Band { avoid_holes, .. }| avoid_holes != AvoidHoles::Active)
                        .map(
                            |Band {
                                 active_lines,
                                 pe,
                                 pe_const,
                                 ..
                             }| { (pe, pe_const, active_lines) },
                        )
                        .reduce_with();

                if active_lines > 0. {
                    let desired_pe_no_ah = c_float::max(desired_pe - (pe - pe_no_ah), 0.);
                    reduction = calc_reduction_3gpp(a, desired_pe_no_ah, pe_no_ah, active_lines);
                }

                pe = 0.;
                for bands in pch
                    .band
                    .as_array_of_cells_deref()
                    .into_iter()
                    .take(c_uchar::from(wi.num_windows).into())
                {
                    for (band, coeffs) in zip(bands, &*coeffs).take(num_bands as usize) {
                        band.update(|mut band| {
                            if active_lines > 0. {
                                band.threshold =
                                    band.calc_reduced_threshold(coeffs.min_snr, reduction);
                            }
                            pe += calc_pe_3gpp(&mut band);
                            band.norm_fac = if band.threshold > 0. {
                                band.active_lines / band.threshold
                            } else {
                                0.
                            };
                            norm_fac += band.norm_fac;

                            band
                        });
                    }
                }
                delta_pe = desired_pe - pe;
                if delta_pe.abs() > 0.05 * desired_pe {
                    break;
                }
            }

            if pe < 1.15 * desired_pe {
                // 6.6.1.3.6 "Final threshold modification by linearization"
                norm_fac = if norm_fac != 0. { norm_fac.recip() } else { 0. };
                for bands in pch
                    .band
                    .as_array_of_cells_deref()
                    .into_iter()
                    .take(c_uchar::from(wi.num_windows).into())
                {
                    for (band, coeffs) in zip(bands, &*coeffs)
                        .take(num_bands as usize)
                        .filter(|(band, _)| band.get().active_lines > 0.5)
                    {
                        band.update(|mut band| {
                            let delta_sfb_pe = band.norm_fac * norm_fac * delta_pe;
                            let mut thr = band.threshold;

                            thr *= (delta_sfb_pe / band.active_lines).exp2();
                            if thr > coeffs.min_snr * band.energy
                                && band.avoid_holes == AvoidHoles::Inactive
                            {
                                thr = c_float::max(band.threshold, coeffs.min_snr * band.energy);
                            }
                            band.threshold = thr;
                            band
                        });
                    }
                }
            } else {
                // 5.6.1.3.7 "Further perceptual entropy reduction"
                for (g, coeffs) in coeffs[..num_bands as usize]
                    .iter_mut()
                    .map(Cell::from_mut)
                    .enumerate()
                    .rev()
                {
                    for band in pch
                        .band
                        .as_array_of_cells_deref()
                        .into_iter()
                        .take(c_uchar::from(wi.num_windows).into())
                        .map(|bands| &bands[g])
                        .filter(|band| {
                            band.get().avoid_holes != AvoidHoles::None
                                && coeffs.get().min_snr < SNR_1DB
                        })
                    {
                        coeffs.update(|coeffs| Coeffs {
                            min_snr: SNR_1DB,
                            ..coeffs
                        });
                        band.update(|mut band| {
                            band.threshold = band.energy * SNR_1DB;
                            pe += band.active_lines * 1.5 - band.pe;
                            band
                        });
                    }

                    if pe <= desired_pe {
                        break;
                    }
                }
                /* TODO: allow more holes (unused without mid/side) */
            }
        }

        for (psy_bands, aac_bands) in zip(ch.psy_bands.as_array_of_cells_deref(), &pch.band)
            .take(c_uchar::from(wi.num_windows).into())
        {
            for (
                psy_band,
                &Band {
                    threshold,
                    energy,
                    active_lines,
                    ..
                },
                &band_size,
            ) in izip!(psy_bands, aac_bands, band_sizes).take(num_bands as usize)
            {
                psy_band.set(FFPsyBand {
                    threshold,
                    energy,
                    spread: active_lines * 2. / c_float::from(band_size),
                });
            }
        }

        pch.prev_band = pch.band;
    }

    #[ffmpeg_src(file = "libavcodec/aacpsy.c", lines = 848..=856, name = "psy_3gpp_analyze")]
    pub(super) fn analyze(
        &mut self,
        avctx: &CodecContext,
        channel: c_int,
        coeffs: &[&[c_float]; 2],
        wi: &[PsyWindowInfo],
    ) {
        let &FFPsyChannelGroup { num_ch } = self.find_group(channel);
        for (ch, (&coeffs, wi)) in zip(coeffs, wi).take(num_ch.into()).enumerate() {
            self.analyze_channel(avctx, channel + ch as c_int, coeffs, wi);
        }
    }

    #[ffmpeg_src(file = "libavcodec/aacpsy.c", lines = 884..=1020, name = "psy_lame_window")]
    pub(super) fn window(
        &mut self,
        la: Option<&[c_float]>,
        channel: c_int,
        prev_type: WindowSequence,
    ) -> PsyWindowInfo {
        let pctx = &mut *self.model_priv_data;
        let pch = &mut pctx.ch[channel as usize];
        let mut use_long_block = true;
        let mut attacks: [c_int; 9] = [0; 9];

        if let Some(la) = la {
            let mut attack_intensity = [0.; 27];
            let mut energy_subshort = [0.; 27];
            let mut energy_short = [0.; 9];
            let firbuf = &la[128 / 4 - 21..];
            let hpf_samples = hp_filter(firbuf, &FIR_COEFFS);
            for (energy_subshort, attack_intensity, prev_energy_subshort) in izip!(
                &mut energy_subshort,
                &mut attack_intensity,
                successors(pch.prev_energy_subshort.get(..), |s| s.get(1..))
            )
            .take(3)
            {
                *energy_subshort = prev_energy_subshort[(8 - 1) * 3];
                *attack_intensity = *energy_subshort / prev_energy_subshort[(8 - 2) * 3 + 1];
                energy_short[0] += *energy_subshort;
            }
            for (i, (pf, prev_energy_subshort, attack_intensity)) in izip!(
                hpf_samples.array_chunks::<{ 1024 / (8 * 3) }>(),
                &mut pch.prev_energy_subshort,
                &mut attack_intensity[3..]
            )
            .enumerate()
            {
                let p = pf.iter().fold(1., |p, pf| c_float::max(p, pf.abs()));
                energy_subshort[i + 3] = p;
                *prev_energy_subshort = energy_subshort[i + 3];
                energy_short[1 + i / 3] += p;

                *attack_intensity = if p > energy_subshort[i + 1] {
                    p / energy_subshort[i + 1]
                } else if energy_subshort[i + 1] > p * 10. {
                    energy_subshort[i + 1] / (p * 10.)
                } else {
                    0.
                };
            }
            for (attack, attack_intensities) in
                zip(&mut attacks, attack_intensity.array_chunks::<3>())
            {
                if *attack == 0
                    && let Some(i) = attack_intensities
                        .iter()
                        .position(|&attack_intensity| attack_intensity > pch.attack_threshold)
                {
                    *attack = i as c_int + 1;
                }
            }
            {
                let ([first_attack], attacks) = attacks.split_array_mut();
                for (i, (_, attack)) in zip(energy_short.array_windows(), attacks)
                    .enumerate()
                    .filter(|(_, (&[u, v], _))| {
                        c_float::max(u, v) < 40000. && u < 1.7 * v && v < 1.7 * u
                    })
                {
                    if i == 1 && *first_attack < *attack {
                        *first_attack = 0;
                    }
                    *attack = 0;
                }
            }
            {
                let [attack, ..] = &mut attacks;
                if *attack <= pch.prev_attack {
                    *attack = 0;
                }
            }
            let attack_sum: c_int = attacks.iter().sum();
            if pch.prev_attack == 3 || attack_sum != 0 {
                use_long_block = false;
                for [_, attack1] in Cell::from_mut(&mut attacks)
                    .as_array_of_cells()
                    .array_windows()
                    .filter(|attacks| attacks.iter().map(Cell::get).all(|attack| attack != 0))
                {
                    attack1.set(0);
                }
            }
        } else {
            use_long_block = !(prev_type == WindowSequence::EightShort);
        }

        let window_type = pch.apply_block_type(use_long_block);
        let wi = PsyWindowInfo {
            window_type: [window_type, prev_type, Default::default()],
            ..match window_type {
                WindowSequence::EightShort => PsyWindowInfo {
                    window_shape: WindowShape::Sine,
                    num_windows: WindowCount::Eight,
                    grouping: calc_grouping(pch.next_grouping),
                    ..Default::default()
                },
                window_type => PsyWindowInfo {
                    window_shape: match window_type {
                        WindowSequence::LongStart => WindowShape::Sine,
                        _ => WindowShape::Kbd,
                    },
                    num_windows: WindowCount::One,
                    grouping: [1, 0, 0, 0, 0, 0, 0, 0],
                    ..Default::default()
                },
            }
        };

        let grouping = attacks
            .iter()
            .position(|&attack| attack != 0)
            .unwrap_or_default();
        pch.next_grouping = WINDOW_GROUPING[grouping];
        [.., pch.prev_attack] = attacks;

        wi
    }
}

impl Channel {
    #[ffmpeg_src(file = "libavcodec/aacpsy.c", lines = 866..=882, name = "lame_apply_block_type")]
    fn apply_block_type(&mut self, use_long_block: bool) -> WindowSequence {
        use WindowSequence::*;

        let block_type;
        (block_type, self.next_window_seq) = match (use_long_block, self.next_window_seq) {
            (true, seq @ EightShort) => (LongStop, seq),
            (false, OnlyLong) => (EightShort, LongStart),
            (false, LongStop) => (EightShort, EightShort),
            (false, seq) => (EightShort, seq),
            (_, seq) => (Default::default(), seq),
        };

        mem::replace(&mut self.next_window_seq, block_type)
    }
}

#[ffmpeg_src(file = "libavcodec/aacpsy.c", lines = 996..=1000)]
fn calc_grouping(next_grouping: c_uchar) -> [c_uchar; 8] {
    let mut grouping = [0; 8];
    let mut last = 0;
    for i in 0..8 {
        if next_grouping >> i & 1 == 0 {
            last = i;
        }
        grouping[last] += 1;
    }
    grouping
}
