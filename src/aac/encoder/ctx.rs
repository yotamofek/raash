use array_util::{Array, WindowedArray};
use ffmpeg_src_macro::ffmpeg_src;
use libc::{c_float, c_int, c_uchar, c_ushort};
use lpc::LPCContext;

use super::channel_layout::pce;
use crate::{aac::SyntaxElementType, audio_frame_queue::AudioFrameQueue, types::*};

#[derive(Default, Clone)]
pub(crate) struct QuantizeBandCostCache {
    pub cache_generation: c_ushort,
    pub cache: Array<Array<AACQuantizeBandCostCacheEntry, 128>, 256>,
}

impl QuantizeBandCostCache {
    #[ffmpeg_src(file = "libavcodec/aacenc.c", lines = 400..=407, name = "ff_quantize_band_cost_cache_init")]
    pub(crate) fn init(&mut self) {
        self.cache_generation = self.cache_generation.wrapping_add(1);
        if self.cache_generation == 0 {
            self.cache = Default::default();
            self.cache_generation = 1;
        }
    }
}

pub struct MdctContext {
    pub mdct1024: *mut AVTXContext,
    pub mdct1024_fn: av_tx_fn,
    pub mdct128: *mut AVTXContext,
    pub mdct128_fn: av_tx_fn,
}

pub(crate) struct AACEncContext {
    pub options: AACEncOptions,

    pub pb: PutBitContext,
    pub mdct: MdctContext,
    pub pce: Option<pce::Info>,
    pub planar_samples: Box<[[c_float; 3 * 1024]]>,
    pub profile: c_int,
    pub needs_pce: c_int,
    pub lpc: LPCContext,
    pub samplerate_index: c_int,
    pub channels: c_int,
    pub reorder_map: &'static [c_uchar],
    pub chan_map: &'static [c_uchar],
    pub cpe: Box<[ChannelElement]>,
    pub psy: FFPsyContext,
    pub cur_channel: c_int,
    pub random_state: c_int,
    pub lambda: c_float,
    pub last_frame_pb_count: c_int,
    pub lambda_sum: c_float,
    pub lambda_count: c_int,
    pub cur_type: SyntaxElementType,
    pub afq: AudioFrameQueue,
    pub scaled_coeffs: WindowedArray<Array<c_float, 1024>, 128>,
    pub quantize_band_cost_cache: QuantizeBandCostCache,
}
