use super::Input;
use crate::core::packet::Packet;
use crate::message::Result;

pub struct InputPacketIter<'a> {
	input: &'a mut Input,
}

impl<'a> InputPacketIter<'a> {
	pub fn new(input: &'a mut Input) -> Self {
		Self { input }
	}
}

impl<'a> Iterator for InputPacketIter<'a> {
	type Item = Result<Packet>;

	fn next(&mut self) -> Option<Self::Item> {
		match self.input.read_packet() {
			Ok(Some(packet)) => Some(Ok(packet)),
			Ok(None) => None,
			Err(message) => Some(Err(message)),
		}
	}
}
