use crate::codecs::{register_codecs, register_codecs_ids};
use crate::core::track::Format;
use crate::core::{CodecId, Decoder, Encoder, Track};
use crate::error;
use crate::message::Result;

pub struct CodecResolver {
	entries: super::CodecFactoryMapper,
	codec_ids: super::CodecIdMapper,
	extensions: super::CodecExtMapper,
}

impl CodecResolver {
	pub fn new() -> Self {
		let (codec_ids, extensions) = register_codecs_ids();
		let entries = register_codecs();
		Self { entries, codec_ids, extensions }
	}

	pub fn codec_id_for(&self, name: &str) -> Result<CodecId> {
		let codec = self.codec_ids.get(name).copied();
		codec.ok_or_else(|| error!("codec '{}' is not supported", name))
	}

	pub fn codec_for_extension(&self, extension: &str) -> Result<CodecId> {
		let codec = self.extensions.get(extension).copied();
		codec.ok_or_else(|| error!("extension '{}' is not supported", extension))
	}

	pub fn codec_resolver(&self, name: Option<String>, extension: &str) -> Result<CodecId> {
		if let Some(name) = name {
			return self.codec_id_for(name.as_str());
		}
		self.codec_for_extension(extension)
	}

	pub fn register_decoder(&mut self, codec_id: CodecId, factory: super::DecoderFactory) {
		let entry = self.entries.entry(codec_id).or_default();
		entry.decoder = Some(factory);
	}

	pub fn register_encoder(&mut self, codec_id: CodecId, factory: super::EncoderFactory) {
		let entry = self.entries.entry(codec_id).or_default();
		entry.encoder = Some(factory);
	}

	pub fn decoder_for(&self, track: &Track) -> Result<Box<dyn Decoder>> {
		let entry = self.entries.get(&track.codec_in);
		match entry {
			Some(entry) => match entry.decoder {
				Some(factory) => factory(track),
				None => Err(error!("decoder for '{}' is not yet implemented", track.codec_in)),
			},
			None => Err(error!("unknown codec '{}'", track.codec_in)),
		}
	}

	pub fn encoder_for(&self, track: &Track, format: &Format) -> Result<Box<dyn Encoder>> {
		let entry = self.entries.get(&track.codec_out);
		match entry {
			Some(entry) => match entry.encoder {
				Some(factory) => factory(track, format),
				None => Err(error!("encoder for '{}' is not yet implemented", track.codec_out)),
			},
			None => Err(error!("unknown codec '{}'", track.codec_out)),
		}
	}
}
