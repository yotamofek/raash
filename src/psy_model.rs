use encoder::CodecContext;
use ffmpeg_src_macro::ffmpeg_src;
use libc::{c_int, c_uchar};

use crate::{aac::psy_model::AacPsyContext, types::*};

impl PsyContext {
    #[cold]
    #[ffmpeg_src(file = "libavcodec/psymodel.c", lines = 31..=71, name = "ff_psy_init")]
    pub(crate) fn init(
        avctx: &CodecContext,
        bands: &[&'static [c_uchar]],
        num_bands: &[c_int],
        num_groups: c_int,
        group_map: &[c_uchar; 16],
    ) -> Self {
        assert_eq!(bands.len(), num_bands.len());
        let mut ctx = PsyContext {
            ch: vec![FFPsyChannel::default(); avctx.ch_layout().get().nb_channels as usize * 2]
                .into_boxed_slice(),
            group: group_map[..num_groups as usize]
                .iter()
                .map(|&group| FFPsyChannelGroup { num_ch: group + 1 })
                .collect(),
            cutoff: avctx.cutoff().get(),
            bands: bands.to_vec().into_boxed_slice(),
            num_bands: num_bands.to_vec().into_boxed_slice(),
            bitres: BitResolution::default(),
            model_priv_data: Default::default(),
        };
        assert_eq!(avctx.codec_id().get(), AV_CODEC_ID_AAC);
        *ctx.model_priv_data = AacPsyContext::init(avctx, &mut ctx);
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
