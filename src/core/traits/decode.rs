use crate::core::frame::FrameIter;
use crate::core::packet::Packet;
use crate::message::Result;

pub trait Decoder {
	fn decode(&mut self, packet: Packet) -> Result<FrameIter>;
	fn finish(&mut self) -> Result<FrameIter>;
}
