use crate::container::wav::format::WavFormat;
use crate::core::frame::{AudioFormat, Channels, Frame, FrameAudio};
use crate::core::packet::Packet;
use crate::core::traits::Decoder;
use crate::message::Result;

pub struct PcmDecoder {
	sample_rate: u32,
	channels: Channels,
	bytes_per_sample: usize,
}

impl PcmDecoder {
	pub fn new(sample_rate: u32, channels: Channels, bytes_per_sample: usize) -> Self {
		Self { sample_rate, channels, bytes_per_sample }
	}

	pub fn new_from_metadata(metadata: &WavFormat) -> Self {
		Self::new(metadata.sample_rate, metadata.channels, metadata.bytes_per_sample())
	}
}

impl Decoder for PcmDecoder {
	fn decode(&mut self, packet: Packet) -> Result<Option<Frame>> {
		if packet.is_empty() {
			return Ok(None);
		}

		let nb_samples = packet.data.len() / (self.channels.count() as usize * self.bytes_per_sample);

		let audio_format = match self.bytes_per_sample {
			2 => AudioFormat::PCM16,
			3 => AudioFormat::PCM24,
			4 => AudioFormat::PCM32,
			_ => AudioFormat::PCM16,
		};

		let audio = FrameAudio::new(packet.data, self.sample_rate, self.channels, audio_format);
		let audio = audio.with_nb_samples(nb_samples);

		let frame = Frame::new_audio(audio, packet.stream_id);

		Ok(Some(frame.with_pts(packet.pts)))
	}

	fn flush(&mut self) -> Result<Option<Frame>> {
		Ok(None)
	}
}
