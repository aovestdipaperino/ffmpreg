pub mod pool;

use crate::core::packet::Packet;
use crate::core::traits::{Decoder, Encoder};
use crate::message::Result;

pub use pool::Transcoders;

pub struct Transcoder {
	decoder: Box<dyn Decoder>,
	encoder: Box<dyn Encoder>,
}

impl Transcoder {
	pub fn new(decoder: Box<dyn Decoder>, encoder: Box<dyn Encoder>) -> Self {
		Self { decoder, encoder }
	}

	pub fn transcode(&mut self, packet: Packet) -> Result<Vec<Packet>> {
		let mut output = vec![];
		for frame in self.decoder.decode(packet)? {
			for packet in self.encoder.encode(frame)? {
				output.push(packet);
			}
		}
		Ok(output)
	}

	pub fn flush(&mut self) -> Result<Vec<Packet>> {
		let mut output = vec![];

		for frame in self.decoder.finish()? {
			for packet in self.encoder.encode(frame)? {
				output.push(packet);
			}
		}

		for packet in self.encoder.finish()? {
			output.push(packet);
		}

		Ok(output)
	}
}
