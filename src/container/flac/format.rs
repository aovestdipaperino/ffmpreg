use std::collections::HashMap;

use crate::container::raw;
use crate::core::frame::{AudioFormat, Channels};

#[derive(Debug, Clone, Copy)]
pub struct FlacFormat {
	pub channels: Channels,
	pub sample_rate: u32,
	pub bit_depth: u16,
}

impl Default for FlacFormat {
	fn default() -> Self {
		Self { channels: Channels::Stereo, sample_rate: 44100, bit_depth: 16 }
	}
}

impl FlacFormat {
	pub fn new(channels: Channels, sample_rate: u32, bit_depth: u16) -> Self {
		Self { channels, sample_rate, bit_depth }
	}

	pub fn to_codec_string(&self) -> &'static str {
		"flac"
	}

	pub fn bytes_per_frame(&self) -> usize {
		(self.bit_depth as usize / 8) * self.channels.count() as usize
	}

	pub fn block_align(&self) -> u16 {
		((self.bit_depth / 8) as u16) * self.channels.count() as u16
	}

	pub fn to_raw_format(&self) -> raw::RawPcmFormat {
		raw::RawPcmFormat { channels: self.channels, sample_rate: self.sample_rate, bit_depth: self.bit_depth }
	}

	pub fn audio_format(&self) -> AudioFormat {
		match self.bit_depth {
			16 => AudioFormat::PCM16,
			24 => AudioFormat::PCM24,
			32 => AudioFormat::PCM32,
			_ => AudioFormat::PCM16,
		}
	}

	pub fn apply_codec(&mut self, codec: &str) -> Result<(), String> {
		match codec {
			"pcm_s16le" => self.bit_depth = 16,
			"pcm_s24le" => self.bit_depth = 24,
			"pcm_f32le" => self.bit_depth = 32,
			_ => return Err(format!("flac codec '{}' is not supported", codec)),
		}
		Ok(())
	}
}

#[derive(Debug, Clone)]
pub struct FlacMetadata {
	fields: HashMap<String, String>,
}

impl FlacMetadata {
	pub fn new() -> Self {
		Self { fields: HashMap::new() }
	}

	pub fn set(&mut self, key: &str, value: String) {
		self.fields.insert(key.to_string(), value);
	}

	pub fn get(&self, key: &str) -> Option<&String> {
		self.fields.get(key)
	}

	pub fn all_fields(&self) -> &HashMap<String, String> {
		&self.fields
	}

	pub fn is_empty(&self) -> bool {
		self.fields.is_empty()
	}
}
