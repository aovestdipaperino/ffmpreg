use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::slice::ParallelSlice;

use super::format::WavFormat;
use super::utils;
use crate::{error, message};

pub fn to_f32(data: &[u8], format: &WavFormat) -> message::Result<Vec<f32>> {
	match format.bit_depth.bits() {
		16 => decode_pcm16(data),
		24 => decode_pcm24(data),
		32 => decode_pcm32(data),
		_ => Err(error!("unsupported bit depth")),
	}
}

pub fn from_f32(samples: &[f32], format: &WavFormat) -> message::Result<Vec<u8>> {
	match format.bit_depth.bits() {
		16 => encode_pcm16(samples),
		24 => encode_pcm24(samples),
		32 => encode_pcm32(samples),
		_ => Err(error!("unsupported bit depth")),
	}
}

fn decode_pcm16(data: &[u8]) -> message::Result<Vec<f32>> {
	if data.len() % 2 != 0 {
		return Err(error!("invalid pcm16 length"));
	}
	let samples = data
		.par_chunks_exact(2)
		.map(|b| {
			let sample = i16::from_le_bytes([b[0], b[1]]);
			utils::normalize_pcm16(sample)
		})
		.collect();
	Ok(samples)
}

fn decode_pcm24(data: &[u8]) -> message::Result<Vec<f32>> {
	if data.len() % 3 != 0 {
		return Err(error!("invalid pcm24 length"));
	}
	let samples = data
		.par_chunks_exact(3)
		.map(|chunk| {
			let value = (chunk[0] as i32) | ((chunk[1] as i32) << 8) | ((chunk[2] as i32) << 16);
			let value = if value & 0x800000 != 0 { value | (0xFF000000u32 as i32) } else { value };
			utils::normalize_pcm24(value)
		})
		.collect();
	Ok(samples)
}

fn decode_pcm32(data: &[u8]) -> message::Result<Vec<f32>> {
	if data.len() % 4 != 0 {
		return Err(error!("invalid pcm32 length"));
	}
	let samples = data
		.par_chunks_exact(4)
		.map(|b| {
			let sample = f32::from_le_bytes([b[0], b[1], b[2], b[3]]);
			utils::normalize_pcm32(sample)
		})
		.collect();
	Ok(samples)
}

fn encode_pcm16(samples: &[f32]) -> message::Result<Vec<u8>> {
	let encoded = samples
		.par_iter()
		.flat_map(|&s| {
			let sample = utils::denormalize_pcm16(s);
			sample.to_le_bytes()
		})
		.collect();
	Ok(encoded)
}

fn encode_pcm24(samples: &[f32]) -> message::Result<Vec<u8>> {
	let encoded = samples
		.par_iter()
		.flat_map(|&s| {
			let val = utils::denormalize_pcm24(s);
			let val_24 = val & 0xFFFFFF;
			vec![
				(val_24 & 0xFF) as u8,
				((val_24 >> 8) & 0xFF) as u8,
				((val_24 >> 16) & 0xFF) as u8,
			]
		})
		.collect();
	Ok(encoded)
}

fn encode_pcm32(samples: &[f32]) -> message::Result<Vec<u8>> {
	let encoded = samples
		.par_iter()
		.flat_map(|&s| {
			let sample = utils::denormalize_pcm32(s);
			sample.to_le_bytes()
		})
		.collect();
	Ok(encoded)
}
