//! PCM normalization and denormalization utilities.

const PCM16_SCALE: f32 = 32768.0;
const PCM16_MAX: f32 = 32767.0;
const PCM24_SCALE: f32 = 8388608.0;
const PCM24_MAX: f32 = 8388607.0;

pub const fn normalize_pcm16(sample: i16) -> f32 {
	sample as f32 / PCM16_SCALE
}

pub const fn denormalize_pcm16(normalized: f32) -> i16 {
	(normalized * PCM16_MAX).clamp(-PCM16_SCALE, PCM16_MAX) as i16
}

pub const fn normalize_pcm24(sample: i32) -> f32 {
	sample as f32 / PCM24_SCALE
}

pub const fn denormalize_pcm24(normalized: f32) -> i32 {
	(normalized * PCM24_MAX).clamp(-PCM24_SCALE, PCM24_MAX) as i32
}

pub const fn normalize_pcm32(sample: f32) -> f32 {
	sample.clamp(-1.0, 1.0)
}

pub const fn denormalize_pcm32(normalized: f32) -> f32 {
	normalized.clamp(-1.0, 1.0)
}
