use crate::core::frame::Channels;

pub struct ChannelConverter {
	input: Channels,
	output: Channels,
}

impl ChannelConverter {
	pub fn new(input: Channels, output: Channels) -> Self {
		Self { input, output }
	}

	pub fn is_needed(&self) -> bool {
		self.input != self.output
	}

	pub fn convert(&self, samples: &[f32]) -> Vec<f32> {
		if !self.is_needed() {
			return samples.to_vec();
		}

		let in_ch = self.input.count() as usize;
		let out_ch = self.output.count() as usize;

		match (in_ch, out_ch) {
			(1, 2) => Self::mono_to_stereo(samples),
			(2, 1) => Self::stereo_to_mono(samples),
			(i, o) if i < o => Self::upmix(samples, i, o),
			(i, o) => Self::downmix(samples, i, o),
		}
	}

	fn mono_to_stereo(samples: &[f32]) -> Vec<f32> {
		let mut out = Vec::with_capacity(samples.len() * 2);
		for &s in samples {
			out.push(s);
			out.push(s);
		}
		out
	}

	fn stereo_to_mono(samples: &[f32]) -> Vec<f32> {
		let mut out = Vec::with_capacity(samples.len() / 2);
		for pair in samples.chunks_exact(2) {
			let mixed = (pair[0] + pair[1]) * 0.5;
			out.push(mixed);
		}
		out
	}

	fn upmix(samples: &[f32], in_ch: usize, out_ch: usize) -> Vec<f32> {
		let num_frames = samples.len() / in_ch;
		let mut out = Vec::with_capacity(num_frames * out_ch);
		for frame in samples.chunks_exact(in_ch) {
			for &s in frame {
				out.push(s);
			}
			for _ in in_ch..out_ch {
				out.push(0.0);
			}
		}
		out
	}

	fn downmix(samples: &[f32], in_ch: usize, out_ch: usize) -> Vec<f32> {
		let num_frames = samples.len() / in_ch;
		let mut out = Vec::with_capacity(num_frames * out_ch);
		let scale = 1.0 / in_ch as f32;
		for frame in samples.chunks_exact(in_ch) {
			for o in 0..out_ch {
				let mut sum = 0.0;
				let sources = in_ch / out_ch;
				let start = o * sources;
				let end = (start + sources).min(in_ch);
				for i in start..end {
					sum += frame[i];
				}
				out.push(sum * scale * out_ch as f32);
			}
		}
		out
	}
}
