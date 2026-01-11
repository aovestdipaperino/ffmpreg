use crate::container::wav::{converter, format};
use crate::core::Encoder;
use crate::core::frame::{AudioFormat, Channels, Frame};
use crate::core::packet::Packet;
use crate::core::time::Time;
use crate::message::Result;

pub struct PcmEncoder {
	sample_rate: u32,
	target_format: Option<AudioFormat>,
}

impl PcmEncoder {
	pub fn new(sample_rate: u32) -> Self {
		Self { sample_rate, target_format: None }
	}

	pub fn with_target_format(mut self, format: AudioFormat) -> Self {
		self.target_format = Some(format);
		self
	}

	fn wav_format(format: AudioFormat, channels: Channels, sample_rate: u32) -> format::WavFormat {
		let bit_depth = match format {
			AudioFormat::PCM16 => 16,
			AudioFormat::PCM24 => 24,
			AudioFormat::PCM32 => 32,
			_ => 16,
		};
		let format_code = if bit_depth == 32 { 3 } else { 1 };
		format::WavFormat { channels, sample_rate, bit_depth, format_code }
	}
}

impl Encoder for PcmEncoder {
	fn encode(&mut self, frame: Frame) -> Result<Option<Packet>> {
		let audio = match frame.audio() {
			Some(audio) => audio,
			None => return Ok(None),
		};

		let time = Time::new(1, self.sample_rate)?;

		if let Some(target) = self.target_format {
			let format = Self::wav_format(audio.format, audio.channels, self.sample_rate);
			let target_format = Self::wav_format(target, audio.channels, self.sample_rate);

			let samples = converter::to_f32(&audio.data, &format)?;

			let data = converter::from_f32(&samples, &target_format)?;
			let packet = Packet::new(data, frame.stream_id, time);
			return Ok(Some(packet.with_pts(frame.pts)));
		}

		let packet = Packet::new(audio.data.clone(), frame.stream_id, time);
		Ok(Some(packet.with_pts(frame.pts)))
	}

	fn flush(&mut self) -> Result<Option<Packet>> {
		Ok(None)
	}
}
