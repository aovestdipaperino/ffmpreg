use crate::core::frame::Frame;
use crate::core::packet::Packet;
use crate::error;
use crate::message::Result;

pub enum Media {
	Packet(Packet),
	Frame(Frame),
}

impl Media {
	pub fn into_packet(self) -> Result<Packet> {
		match self {
			Media::Packet(p) => Ok(p),
			Media::Frame(_) => Err(error!("expected packet, got frame")),
		}
	}

	pub fn into_frame(self) -> Result<Frame> {
		match self {
			Media::Frame(f) => Ok(f),
			Media::Packet(_) => Err(error!("expected frame, got packet")),
		}
	}
}
