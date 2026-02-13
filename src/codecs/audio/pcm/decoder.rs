use crate::container::wav::format::WavFormat;
use crate::core::frame::{BitDepth, Channels, Frame, FrameAudio, FrameIter, SampleRate};
use crate::core::packet::Packet;
use crate::core::track::AudioFormat;
use crate::core::traits::Decoder;
use crate::message::Result;

pub struct PcmDecoder {
	sample_rate: SampleRate,
	channels: Channels,
	bit_depth: BitDepth,
}

impl PcmDecoder {
	pub fn new(sample_rate: SampleRate, channels: Channels, bit_depth: BitDepth) -> Self {
		Self { sample_rate, channels, bit_depth }
	}

	pub fn from_format(format: &AudioFormat) -> Self {
		Self::new(format.sample_rate, format.channels, format.bit_depth)
	}

	pub fn from_wav(wav: &WavFormat) -> Self {
		let bit_depth = BitDepth::from_bits_any((wav.bytes_per_sample() * 8) as u8);
		Self::new(wav.sample_rate, wav.channels, bit_depth)
	}
}

impl Decoder for PcmDecoder {
	fn decode(&mut self, packet: Packet) -> Result<FrameIter> {
		if packet.is_empty() {
			return Ok(FrameIter::empty());
		}

		let audio = FrameAudio::new(packet.data, self.sample_rate, self.channels, self.bit_depth);
		let frame = Frame::new_audio(audio, packet.track_id);

		Ok(FrameIter::new(vec![frame]))
	}

	fn finish(&mut self) -> Result<FrameIter> {
		Ok(FrameIter::empty())
	}
}
