use crate::container::wav::{converter, format};
use crate::core::Encoder;
use crate::core::frame::{BitDepth, Channels, Frame, FrameAudio, SampleRate};
use crate::core::packet::{Packet, PacketIter};
use crate::core::time::Time;
use crate::message::Result;

pub struct PcmEncoder {
	sample_rate: SampleRate,
	target_bit_depth: Option<BitDepth>,
}

impl PcmEncoder {
	pub fn new(sample_rate: SampleRate) -> Self {
		Self { sample_rate, target_bit_depth: None }
	}

	#[inline(always)]
	pub fn from_format(format: &format::WavFormat) -> Self {
		Self::new(format.sample_rate).with_bit_depth(format.bit_depth)
	}

	pub const fn with_bit_depth(mut self, bit_depth: BitDepth) -> Self {
		self.target_bit_depth = Some(bit_depth);
		self
	}

	const fn make_wav_format(
		bit_depth: BitDepth,
		channels: Channels,
		sample_rate: SampleRate,
	) -> format::WavFormat {
		let format_code = if bit_depth.bits() == 32 { 3 } else { 1 };
		format::WavFormat { channels, sample_rate, bit_depth, format_code }
	}
}

impl Encoder for PcmEncoder {
	fn encode(&mut self, mut frame: Frame) -> Result<PacketIter> {
		let audio = frame.audio_mut().ok_or_else(|| crate::error!("no audio data"))?;
		let time = Time::new(1, self.sample_rate.value())?;

		let packet = if let Some(bit_depth) = self.target_bit_depth {
			let data = self.transcode_audio(&audio, bit_depth)?;
			Packet::new(data, frame.track_id, time).with_pts(frame.pts().pts)
		} else {
			// todo: is good to take here?
			let data = std::mem::take(&mut audio.data);
			Packet::new(data, frame.track_id, time).with_pts(frame.pts().pts)
		};

		Ok(PacketIter::new(vec![packet]))
	}

	fn finish(&mut self) -> Result<PacketIter> {
		Ok(PacketIter::empty())
	}
}

impl PcmEncoder {
	fn transcode_audio(&self, audio: &FrameAudio, bit_depth: BitDepth) -> Result<Vec<u8>> {
		let src_format = Self::make_wav_format(audio.bit_depth, audio.channels, self.sample_rate);
		let dst_format = Self::make_wav_format(bit_depth, audio.channels, self.sample_rate);

		let samples = converter::to_f32(&audio.data, &src_format)?;
		converter::from_f32(&samples, &dst_format)
	}
}
