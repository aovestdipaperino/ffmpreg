pub mod channels;
pub mod pcm;
pub mod rate;

use crate::core::frame::{Frame, FrameAudio};
use crate::core::track::AudioFormat;
use crate::core::traits::Resampler;
use crate::message::Result;

use channels::ChannelConverter;
use pcm::PcmResampler;
use rate::RateConverter;

pub struct AudioResampler {
	input: AudioFormat,
	output: AudioFormat,
	pcm: PcmResampler,
	channels: ChannelConverter,
	rate: RateConverter,
}

impl AudioResampler {
	pub fn new(input: AudioFormat, output: AudioFormat) -> Self {
		let pcm = PcmResampler::new(input.bit_depth, output.bit_depth);
		let channels = ChannelConverter::new(input.channels, output.channels);
		let rate = RateConverter::new(input.sample_rate, output.sample_rate, output.channels.count());
		Self { input, output, pcm, channels, rate }
	}
}

impl Resampler for AudioResampler {
	fn input_format(&self) -> AudioFormat {
		self.input
	}

	fn output_format(&self) -> AudioFormat {
		self.output
	}

	fn needed(&self) -> bool {
		self.pcm.is_needed() || self.channels.is_needed() || self.rate.is_needed()
	}

	fn resample(&mut self, frame: Frame) -> Result<Frame> {
		let audio = match frame.audio() {
			Some(_) => frame.audio().unwrap(),
			None => return Ok(frame),
		};

		let needs_bit = self.pcm.is_needed();
		let needs_ch = self.channels.is_needed();
		let needs_rate = self.rate.is_needed();

		if !needs_bit && !needs_ch && !needs_rate {
			return Ok(frame);
		}

		let track_id = frame.track_id;
		let pts = audio.pts;

		let mut samples = self.pcm.decode(&audio.data)?;

		if needs_ch {
			samples = self.channels.convert(&samples);
		}

		if needs_rate {
			samples = self.rate.convert(&samples);
		}

		let data = self.pcm.encode(&samples)?;

		let new_audio =
			FrameAudio::new(data, self.output.sample_rate, self.output.channels, self.output.bit_depth)
				.with_pts(pts);

		Ok(Frame::new_audio(new_audio, track_id))
	}
}
