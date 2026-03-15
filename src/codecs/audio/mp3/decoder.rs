use nanomp3::{Decoder as NanoDecoder, MAX_SAMPLES_PER_FRAME};

use crate::core::frame::{AudioFormat, Channels, Frame, FrameAudio};
use crate::core::packet::Packet;
use crate::core::traits::Decoder;
use crate::message::Result;

pub struct Mp3Decoder {
	inner: NanoDecoder,
	/// Leftover MP3 bytes that weren't consumed by the previous decode call.
	buffer: Vec<u8>,
}

impl Mp3Decoder {
	pub fn new() -> Self {
		Self { inner: NanoDecoder::new(), buffer: Vec::new() }
	}

	/// Decode all complete MP3 frames from the buffer, returning PCM S16LE data.
	fn decode_buffered(&mut self) -> Result<Option<(Vec<u8>, u32, Channels, usize)>> {
		let mut pcm_out: Vec<u8> = Vec::new();
		let mut sample_rate = 0u32;
		let mut channels = Channels::Stereo;
		let mut total_samples = 0usize;

		let mut pcm_buf = [0.0f32; MAX_SAMPLES_PER_FRAME];

		loop {
			if self.buffer.is_empty() {
				break;
			}

			let (consumed, info) = self.inner.decode(&self.buffer, &mut pcm_buf);

			if consumed == 0 && info.is_none() {
				// Not enough data for a frame
				break;
			}

			if consumed > 0 {
				self.buffer.drain(..consumed);
			}

			if let Some(frame_info) = info {
				sample_rate = frame_info.sample_rate;
				let ch_count = frame_info.channels.num();
				channels = Channels::from_count(ch_count);
				let num_samples = frame_info.samples_produced;

				// Convert interleaved f32 samples to PCM S16LE
				for &sample in &pcm_buf[..num_samples] {
					let clamped = sample.clamp(-1.0, 1.0);
					let val = (clamped * i16::MAX as f32) as i16;
					pcm_out.extend_from_slice(&val.to_le_bytes());
				}

				// num_samples is total (interleaved), so per-channel = num_samples / ch_count
				total_samples += num_samples / ch_count as usize;
			}
		}

		if pcm_out.is_empty() {
			return Ok(None);
		}

		Ok(Some((pcm_out, sample_rate, channels, total_samples)))
	}
}

impl Decoder for Mp3Decoder {
	fn decode(&mut self, packet: Packet) -> Result<Option<Frame>> {
		if packet.is_empty() {
			return Ok(None);
		}

		self.buffer.extend_from_slice(&packet.data);

		let (pcm_data, sample_rate, channels, nb_samples) = match self.decode_buffered()? {
			Some(result) => result,
			None => return Ok(None),
		};

		let audio = FrameAudio::new(pcm_data, sample_rate, channels, AudioFormat::PCM16)
			.with_nb_samples(nb_samples);
		let frame = Frame::new_audio(audio, packet.stream_id).with_pts(packet.pts);

		Ok(Some(frame))
	}

	fn flush(&mut self) -> Result<Option<Frame>> {
		if self.buffer.is_empty() {
			return Ok(None);
		}
		// Try to decode any remaining data
		self.decode_buffered()?;
		Ok(None)
	}
}
