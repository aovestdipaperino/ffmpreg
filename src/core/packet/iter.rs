use crate::core::packet::Packet;

pub struct PacketIter {
	packets: Vec<Packet>,
}

impl PacketIter {
	pub fn new(packets: Vec<Packet>) -> Self {
		Self { packets }
	}

	pub fn empty() -> Self {
		Self { packets: Vec::new() }
	}
}

impl Iterator for PacketIter {
	type Item = Packet;

	fn next(&mut self) -> Option<Self::Item> {
		self.packets.pop()
	}
}
