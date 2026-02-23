use crate::codecs::pcm;
use crate::codecs::yuv;
use crate::core::resolver;
use rustc_hash::FxHashMap;

pub fn register_codecs() -> resolver::CodecFactoryMapper {
	let mut mapper = FxHashMap::default();
	pcm::register::register_pcm_codecs(&mut mapper);
	yuv::register::register_yuv_codecs(&mut mapper);
	mapper
}

#[inline(always)]
pub fn register_codecs_ids() -> (resolver::CodecIdMapper, resolver::CodecExtMapper) {
	use crate::codecs::*;

	let mut codecs_mapper = FxHashMap::default();
	let mut extension_mapper = FxHashMap::default();

	#[rustfmt::skip]
		let codecs = [
    // audio
    AAC, MP3, MP2, OPUS, VORBIS, AMR_NB,
    AMR_WB, AC3, EAC3, WMA, ATRAC3, FLAC,
    ALAC, WAVPACK, TTA, APE,
    DSD_LSBF, DSD_MSBF,
    DSD_LSBF_PLANAR, DSD_MSBF_PLANAR,

    // important order
      PCM_F32LE, PCM_S24LE, PCM_S16LE,

    // vídeo
    H265, H264, VP8, VP9, AV1,
    MPEG2, MPEG4, THEORA, VP6,
    WMV1, WMV2, WMV3, MJPEG, JPEG2000,
    PRORES, DNXHD, DNXHR, VP10,
    YUV420P
		];

	codecs.iter().for_each(|&codec| {
		codecs_mapper.insert(codec.name, codec);
		if let Some(extension) = codec.primary_extension() {
			extension_mapper.insert(extension, codec);
		}
		codec.aliases.iter().for_each(|alias| {
			codecs_mapper.insert(alias, codec);
		});
	});

	(codecs_mapper, extension_mapper)
}
