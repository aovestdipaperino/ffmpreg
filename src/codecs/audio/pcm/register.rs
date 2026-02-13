use crate::core::track::Format;
use crate::{codecs, core::resolver, error};

pub fn register_pcm_codecs(codec_mapper: &mut resolver::CodecFactoryMapper) {
	use crate::codecs::pcm::PcmDecoder;
	use crate::codecs::pcm::PcmEncoder;

	let decoder: resolver::DecoderFactory = |track| match track.audio_format() {
		Some(audio_format) => Ok(Box::new(PcmDecoder::from_format(audio_format))),
		None => Err(error!("expected 'audio' format, got '{}'", track.format)),
	};
	let encoder: resolver::EncoderFactory = |_, format| match format {
		Format::Wav(wav_format) => Ok(Box::new(PcmEncoder::from_format(wav_format))),
		_ => Err(error!("expected 'wav' format, got '{}'", format)),
	};

	let codecs = resolver::CodecEntry { decoder: Some(decoder), encoder: Some(encoder) };

	codec_mapper.insert(codecs::PCM_S16LE, codecs);
	codec_mapper.insert(codecs::PCM_S24LE, codecs);
	codec_mapper.insert(codecs::PCM_F32LE, codecs);
}
