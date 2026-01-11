use super::format;
use crate::{core::frame::Channels, error, message::Result};
#[derive(Debug)]
pub struct WavHeader {
	pub channels: Channels,
	pub sample_rate: u32,
	pub byte_rate: u32,
	pub block_align: u16,
	pub bits_per_sample: u16,
	pub format_code: u16,
}

impl Default for WavHeader {
	fn default() -> Self {
		Self {
			channels: Channels::Mono,
			sample_rate: 44100,
			byte_rate: 0,
			block_align: 0,
			bits_per_sample: 16,
			format_code: 1, // PCM
		}
	}
}

impl WavHeader {
	pub fn to_format(&self) -> format::WavFormat {
		format::WavFormat {
			channels: self.channels,
			sample_rate: self.sample_rate,
			bit_depth: self.bits_per_sample,
			format_code: self.format_code,
		}
	}

	pub fn validate(&self) -> Result<()> {
		self.validate_common()?;
		self.validate_layout()?;
		self.validate_codec()
	}

	fn validate_common(&self) -> Result<()> {
		if self.channels.count() == 0 {
			return Err(error!("channels must be non-zero"));
		}
		if self.sample_rate == 0 {
			return Err(error!("sample rate must be non-zero"));
		}
		Ok(())
	}

	fn validate_layout(&self) -> Result<()> {
		if self.block_align == 0 {
			return Err(error!("block align must be non-zero"));
		}
		if self.byte_rate == 0 {
			return Err(error!("byte rate must be non-zero"));
		}
		Ok(())
	}

	fn validate_codec(&self) -> Result<()> {
		match self.format_code {
			1 | 3 => self.validate_pcm(),
			0x11 => self.validate_ima_adpcm(),
			code => Err(error!("unsupported audio format code {}", code)),
		}
	}

	fn validate_pcm(&self) -> Result<()> {
		if self.bits_per_sample == 0 {
			return Err(error!("bits per sample must be non-zero"));
		}
		if self.bits_per_sample % 8 != 0 {
			return Err(error!("bits per sample must be multiple of 8"));
		}
		Ok(())
	}

	fn validate_ima_adpcm(&self) -> Result<()> {
		if self.bits_per_sample != 4 {
			return Err(error!("ima adpcm requires 4 bits per sample"));
		}
		Ok(())
	}
}
