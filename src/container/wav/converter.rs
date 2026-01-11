use super::format::WavFormat;
use super::utils;
use crate::{error, message};
pub fn to_f32(data: &[u8], format: &WavFormat) -> message::Result<Vec<f32>> {
	match format.bit_depth {
		16 => from_pcm16(data),
		24 => from_pcm24(data),
		32 => from_pcm32(data),
		_ => Err(error!("unsupported bit depth")),
	}
}

pub fn from_f32(samples: &[f32], format: &WavFormat) -> message::Result<Vec<u8>> {
	match format.bit_depth {
		16 => to_pcm16(samples),
		24 => to_pcm24(samples),
		32 => to_pcm32(samples),
		_ => Err(error!("unsupported bit depth")),
	}
}

fn from_pcm16(data: &[u8]) -> message::Result<Vec<f32>> {
	if data.len() % 2 != 0 {
		return Err(error!("invalid pcm16 length"));
	}

	let iter = data.chunks_exact(2);
	let value = iter.map(|b| i16::from_le_bytes([b[0], b[1]]));

	Ok(value.map(utils::normalize_pcm16).collect())
}

fn from_pcm24(data: &[u8]) -> message::Result<Vec<f32>> {
	if data.len() % 3 != 0 {
		return Err(error!("invalid pcm24 length"));
	}
	let mut result = Vec::with_capacity(data.len() / 3);
	for chunk in data.chunks_exact(3) {
		let value = ((chunk[0] as u32) | ((chunk[1] as u32) << 8) | ((chunk[2] as u32) << 16)) as i32;
		result.push(utils::normalize_pcm24(value << 8));
	}
	Ok(result)
}

fn from_pcm32(data: &[u8]) -> message::Result<Vec<f32>> {
	if data.len() % 4 != 0 {
		return Err(error!("invalid pcm32 length"));
	}
	let iter = data.chunks_exact(4);
	let value = iter.map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]));

	Ok(value.map(utils::normalize_pcm32).collect())
}

fn to_pcm16(samples: &[f32]) -> message::Result<Vec<u8>> {
	Ok(samples.iter().flat_map(|&s| utils::denormalize_pcm16(s).to_le_bytes()).collect())
}

fn to_pcm24(samples: &[f32]) -> message::Result<Vec<u8>> {
	let mut buf = Vec::with_capacity(samples.len() * 3);
	for &s in samples {
		let val = utils::denormalize_pcm24(s);
		let val_24 = (val >> 8) as u32 & 0xFFFFFF;
		buf.push((val_24 & 0xFF) as u8);
		buf.push(((val_24 >> 8) & 0xFF) as u8);
		buf.push(((val_24 >> 16) & 0xFF) as u8);
	}
	Ok(buf)
}

fn to_pcm32(samples: &[f32]) -> message::Result<Vec<u8>> {
	Ok(samples.iter().flat_map(|&s| utils::denormalize_pcm32(s).to_le_bytes()).collect())
}
