use crate::core::frame::BitDepth;
use crate::{error, message};

const PCM16_SCALE: f32 = 32768.0;
const PCM16_MAX: f32 = 32767.0;
const PCM24_SCALE: f32 = 8388608.0;
const PCM24_MAX: f32 = 8388607.0;

pub struct PcmResampler {
	input: BitDepth,
	output: BitDepth,
}

impl PcmResampler {
	pub fn new(input: BitDepth, output: BitDepth) -> Self {
		Self { input, output }
	}

	pub fn is_needed(&self) -> bool {
		self.input != self.output
	}

	pub fn resample(&self, data: &[u8]) -> message::Result<Vec<u8>> {
		if !self.is_needed() {
			return Ok(data.to_vec());
		}
		let samples = self.decode(data)?;
		let converted = self.encode(&samples)?;
		Ok(converted)
	}

	pub fn decode(&self, data: &[u8]) -> message::Result<Vec<f32>> {
		match self.input.bits() {
			16 => Self::decode_pcm16(data),
			24 => Self::decode_pcm24(data),
			32 => Self::decode_pcm32(data),
			_ => Err(error!("unsupported input bit depth: {}", self.input)),
		}
	}

	pub fn encode(&self, samples: &[f32]) -> message::Result<Vec<u8>> {
		match self.output.bits() {
			16 => Self::encode_pcm16(samples),
			24 => Self::encode_pcm24(samples),
			32 => Self::encode_pcm32(samples),
			_ => Err(error!("unsupported output bit depth: {}", self.output)),
		}
	}

	fn decode_pcm16(data: &[u8]) -> message::Result<Vec<f32>> {
		if data.len() % 2 != 0 {
			return Err(error!("invalid pcm16 length"));
		}
		let samples = data
			.chunks_exact(2)
			.map(|b| {
				let sample = i16::from_le_bytes([b[0], b[1]]);
				sample as f32 / PCM16_SCALE
			})
			.collect();
		Ok(samples)
	}

	fn decode_pcm24(data: &[u8]) -> message::Result<Vec<f32>> {
		if data.len() % 3 != 0 {
			return Err(error!("invalid pcm24 length"));
		}
		let samples = data
			.chunks_exact(3)
			.map(|chunk| {
				let value = (chunk[0] as i32) | ((chunk[1] as i32) << 8) | ((chunk[2] as i32) << 16);
				let value = if value & 0x800000 != 0 { value | (0xFF000000u32 as i32) } else { value };
				value as f32 / PCM24_SCALE
			})
			.collect();
		Ok(samples)
	}

	fn decode_pcm32(data: &[u8]) -> message::Result<Vec<f32>> {
		if data.len() % 4 != 0 {
			return Err(error!("invalid pcm32 length"));
		}
		let samples = data
			.chunks_exact(4)
			.map(|b| {
				let sample = f32::from_le_bytes([b[0], b[1], b[2], b[3]]);
				sample.clamp(-1.0, 1.0)
			})
			.collect();
		Ok(samples)
	}

	fn encode_pcm16(samples: &[f32]) -> message::Result<Vec<u8>> {
		let encoded = samples
			.iter()
			.flat_map(|&s| {
				let sample = (s * PCM16_MAX).clamp(-PCM16_SCALE, PCM16_MAX) as i16;
				sample.to_le_bytes()
			})
			.collect();
		Ok(encoded)
	}

	fn encode_pcm24(samples: &[f32]) -> message::Result<Vec<u8>> {
		let encoded = samples
			.iter()
			.flat_map(|&s| {
				let val = (s * PCM24_MAX).clamp(-PCM24_SCALE, PCM24_MAX) as i32;
				let val_24 = val & 0xFFFFFF;
				[(val_24 & 0xFF) as u8, ((val_24 >> 8) & 0xFF) as u8, ((val_24 >> 16) & 0xFF) as u8]
			})
			.collect();
		Ok(encoded)
	}

	fn encode_pcm32(samples: &[f32]) -> message::Result<Vec<u8>> {
		let encoded = samples
			.iter()
			.flat_map(|&s| {
				let sample = s.clamp(-1.0, 1.0);
				sample.to_le_bytes()
			})
			.collect();
		Ok(encoded)
	}
}
