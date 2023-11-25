use libc::c_int;

/**
 * Codec uses get_buffer() or get_encode_buffer() for allocating buffers and
 * supports custom allocators.
 * If not set, it might not use get_buffer() or get_encode_buffer() at all,
 * or use operations that assume the buffer was allocated by
 * avcodec_default_get_buffer2 or avcodec_default_get_encode_buffer.
 */
pub(super) const AV_CODEC_CAP_DR1: c_int = 1 << 1;
/**
 * Encoder or decoder requires flushing with NULL input at the end in order
 * to give the complete and correct output.
 *
 * NOTE: If this flag is not set, the codec is guaranteed to never be fed
 * with       with NULL data. The user can still send NULL data to the
 * public encode       or decode function, but libavcodec will not pass it
 * along to the codec       unless this flag is set.
 *
 * Decoders:
 * The decoder has a non-zero delay and needs to be fed with
 * avpkt->data=NULL, avpkt->size=0 at the end to get the delayed data until
 * the decoder no longer returns frames.
 *
 * Encoders:
 * The encoder needs to be fed with NULL data at the end of encoding until
 * the encoder no longer returns data.
 *
 * NOTE: For encoders implementing the AVCodec.encode2() function, setting
 * this       flag also means that the encoder must set the pts and duration
 * for       each output packet. If this flag is not set, the pts and
 * duration will       be determined by libavcodec from the input frame.
 */
pub(super) const AV_CODEC_CAP_DELAY: c_int = 1 << 5;
/**
 * Codec can be fed a final frame with a smaller size.
 * This can be used to prevent truncation of the last audio samples.
 */
pub(super) const AV_CODEC_CAP_SMALL_LAST_FRAME: c_int = 1 << 6;
