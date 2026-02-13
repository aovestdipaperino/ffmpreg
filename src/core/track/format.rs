use crate::{
	container::wav::WavFormat,
	core::frame::{BitDepth, Channels, SampleRate},
};
use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioFormat {
	pub channels: Channels,
	pub bit_depth: BitDepth,
	pub sample_rate: SampleRate,
}

impl AudioFormat {
	pub fn bytes_per_frame(&self) -> usize {
		let bytes_per_sample = self.bit_depth.bytes() as usize;
		let channels = self.channels.count() as usize;
		bytes_per_sample.saturating_mul(channels)
	}

	pub fn block_align(&self) -> u16 {
		let bytes_per_sample = self.bit_depth.bytes() as u16;
		let channels = self.channels.count() as u16;
		bytes_per_sample.saturating_mul(channels)
	}

	pub fn byte_rate(&self) -> u32 {
		let channels = self.channels.count() as u32;
		let bit_depth = self.bit_depth.bytes() as u32;
		let value = self.sample_rate.value();
		value.saturating_mul(channels).saturating_mul(bit_depth)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VideoFormat {
	pub width: u32,
	pub height: u32,
	pub pixel: Pixel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pixel {
	pub depth: BitDepth,
	pub color_space: ColorSpace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSpace {
	YUV,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackFormat {
	Audio(AudioFormat),
	Video(VideoFormat),
}

#[derive(Debug, Clone)]
pub enum Format {
	Wav(WavFormat),
	Flac,
	Ogg,
	Vorbis,
	Mp3,
	Aac,
}

impl Format {
	pub fn wav() -> Self {
		Self::Wav(WavFormat::default())
	}

	pub fn flac() -> Self {
		Self::Flac
	}

	pub fn ogg() -> Self {
		Self::Ogg
	}

	pub fn vorbis() -> Self {
		Self::Vorbis
	}

	pub fn mp3() -> Self {
		Self::Mp3
	}

	pub fn aac() -> Self {
		Self::Aac
	}

	pub fn from_container(container: crate::container::ContainerId) -> crate::message::Result<Self> {
		match container.name {
			"wav" => Ok(Self::wav()),
			"flac" => Ok(Self::flac()),
			"ogg" | "oga" => Ok(Self::ogg()),
			"mp3" => Ok(Self::mp3()),
			_ => Err(crate::error!("unsupported container '{}'", container.name)),
		}
	}
}

impl fmt::Display for Format {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Format::Wav(_) => write!(f, "wav"),
			Format::Flac => write!(f, "flac"),
			Format::Ogg => write!(f, "ogg"),
			Format::Vorbis => write!(f, "vorbis"),
			Format::Mp3 => write!(f, "mp3"),
			Format::Aac => write!(f, "aac"),
		}
	}
}

impl fmt::Display for TrackFormat {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Audio(_) => write!(f, "audio"),
			Self::Video(_) => write!(f, "video"),
		}
	}
}
