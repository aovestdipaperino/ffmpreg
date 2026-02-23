use crate::core::packet::Packet;

pub struct PacketIter {
	inner: std::vec::IntoIter<Packet>,
}

impl PacketIter {
	pub fn new(packets: Vec<Packet>) -> Self {
		Self { inner: packets.into_iter() }
	}

	pub fn empty() -> Self {
		Self { inner: Vec::new().into_iter() }
	}
}

impl Iterator for PacketIter {
	type Item = Packet;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item> {
		self.inner.next()
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.inner.size_hint()
	}
}
