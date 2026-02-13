use crate::core::frame::Frame;
use crate::core::packet::PacketIter;
use crate::message::Result;

pub trait Encoder {
	fn encode(&mut self, frame: Frame) -> Result<PacketIter>;
	fn finish(&mut self) -> Result<PacketIter>;
}
