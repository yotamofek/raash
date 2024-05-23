#![allow(
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use ffi::codec::AVCodecContext;
use ffmpeg_src_macro::ffmpeg_src;
use libc::{c_int, c_uchar};

use crate::{aac::psy_model::AacPsyContext, types::*};

impl FFPsyContext {
    #[cold]
    #[ffmpeg_src(file = "libavcodec/psymodel.c", lines = 31..=71, name = "ff_psy_init")]
    pub(crate) unsafe fn init(
        avctx: *const AVCodecContext,
        bands: &[&'static [c_uchar]],
        num_bands: &[c_int],
        num_groups: c_int,
        group_map: &[c_uchar; 16],
    ) -> Self {
        assert_eq!(bands.len(), num_bands.len());
        let mut ctx = FFPsyContext {
            avctx,
            ch: vec![FFPsyChannel::default(); (*avctx).ch_layout.nb_channels as usize * 2]
                .into_boxed_slice(),
            group: group_map[..num_groups as usize]
                .iter()
                .map(|&group| FFPsyChannelGroup { num_ch: group + 1 })
                .collect(),
            cutoff: (*avctx).cutoff,
            bands: bands.to_vec().into_boxed_slice(),
            num_bands: num_bands.to_vec().into_boxed_slice(),
            bitres: BitResolution::default(),
            model_priv_data: Default::default(),
        };
        assert_eq!((*ctx.avctx).codec_id, AV_CODEC_ID_AAC);
        *ctx.model_priv_data = AacPsyContext::init(&mut ctx);
        ctx
    }

    #[ffmpeg_src(file = "libavcodec/psymodel.c", lines = 73..=81, name = "ff_psy_find_group")]
    pub(crate) fn find_group(&self, channel: c_int) -> &FFPsyChannelGroup {
        let pos = self
            .group
            .iter()
            .scan(0, |channels, &FFPsyChannelGroup { num_ch }| {
                *channels += c_int::from(num_ch);
                Some(*channels)
            })
            .position(|ch| ch > channel)
            .unwrap();

        &self.group[pos]
    }
}
