use std::collections::HashMap;

use crate::core::frame::Channels;

#[derive(Debug, Clone, Copy)]
pub struct RawPcmFormat {
	pub channels: Channels,
	pub sample_rate: u32,
	pub bit_depth: u16,
}

impl RawPcmFormat {
	pub fn new(channels: Channels, sample_rate: u32, bit_depth: u16) -> Self {
		Self { channels, sample_rate, bit_depth }
	}

	pub fn default_pcm16() -> Self {
		Self { channels: Channels::Stereo, sample_rate: 44100, bit_depth: 16 }
	}

	pub fn to_codec_string(&self) -> &'static str {
		"pcm"
	}

	pub fn bytes_per_frame(&self) -> usize {
		(self.bit_depth as usize / 8) * self.channels.count() as usize
	}

	pub fn block_align(&self) -> u16 {
		((self.bit_depth / 8) as u16) * self.channels.count() as u16
	}

	pub fn byte_rate(&self) -> u32 {
		self.sample_rate.saturating_mul(self.channels.count() as u32).saturating_mul((self.bit_depth / 8) as u32)
	}
}

#[derive(Debug, Clone)]
pub struct RawPcmMetadata {
	fields: HashMap<String, String>,
}

impl RawPcmMetadata {
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

impl Default for RawPcmFormat {
	fn default() -> Self {
		Self::default_pcm16()
	}
}
