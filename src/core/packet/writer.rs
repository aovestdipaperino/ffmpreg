use crate::core::frame::Frame;
use crate::core::traits::{Encoder, Muxer};
use crate::message::Result;

pub struct PacketWriter {
	encoder: Box<dyn Encoder>,
	muxer: Box<dyn Muxer>,
}

impl PacketWriter {
	pub fn new(encoder: Box<dyn Encoder>, muxer: Box<dyn Muxer>) -> Self {
		Self { encoder, muxer }
	}

	pub fn write_frame(&mut self, frame: Frame) -> Result<()> {
		for packet in self.encoder.encode(frame)? {
			self.muxer.write(packet)?;
		}
		Ok(())
	}

	pub fn flush(&mut self) -> Result<()> {
		for packet in self.encoder.finish()? {
			self.muxer.write(packet)?;
		}
		self.muxer.finalize()?;
		Ok(())
	}
}
