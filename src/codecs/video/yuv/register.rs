use crate::core::track::Format;
use crate::{codecs, core::resolver, error};

pub fn register_yuv_codecs(codec_mapper: &mut resolver::CodecFactoryMapper) {
	use crate::codecs::yuv::{YuvDecoder, YuvEncoder};

	let decoder: resolver::DecoderFactory = |track| match track.video_format() {
		Some(video_format) => Ok(Box::new(YuvDecoder::from_format(video_format))),
		None => Err(error!("expected 'video' format, got '{}'", track.format)),
	};

	let encoder: resolver::EncoderFactory = |_, format| match format {
		Format::Yuv(yuv_format) => Ok(Box::new(YuvEncoder::from_format(yuv_format))),
		_ => Err(error!("expected 'yuv' format, got '{}'", format)),
	};

	let codecs = resolver::CodecEntry { decoder: Some(decoder), encoder: Some(encoder) };

	codec_mapper.insert(codecs::YUV420P, codecs);
}
