use crate::core::frame::SampleRate;

pub struct RateConverter {
	input: SampleRate,
	output: SampleRate,
	channels: u8,
}

impl RateConverter {
	pub fn new(input: SampleRate, output: SampleRate, channels: u8) -> Self {
		Self { input, output, channels }
	}

	pub fn is_needed(&self) -> bool {
		self.input != self.output
	}

	pub fn convert(&self, samples: &[f32]) -> Vec<f32> {
		if !self.is_needed() {
			return samples.to_vec();
		}

		let in_rate = self.input.value() as f64;
		let out_rate = self.output.value() as f64;
		let ch = self.channels as usize;
		let in_frames = samples.len() / ch;
		let ratio = in_rate / out_rate;
		let out_frames = ((in_frames as f64) / ratio).ceil() as usize;

		let mut out = Vec::with_capacity(out_frames * ch);

		for i in 0..out_frames {
			let src_pos = i as f64 * ratio;
			let idx = src_pos as usize;
			let frac = (src_pos - idx as f64) as f32;

			let idx_next = (idx + 1).min(in_frames - 1);

			for c in 0..ch {
				let a = samples[idx * ch + c];
				let b = samples[idx_next * ch + c];
				let interpolated = a + (b - a) * frac;
				out.push(interpolated);
			}
		}

		out
	}
}
