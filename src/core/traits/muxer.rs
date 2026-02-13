use crate::core::packet::Packet;
use crate::core::track::Metadata;
use crate::message::Result;

pub trait Muxer {
	fn write(&mut self, packet: Packet) -> Result<()>;

	fn finalize(&mut self) -> Result<()>;

	/// Set metadata for the output container.
	/// Default implementation does nothing (not all containers support metadata).
	fn set_metadata(&mut self, _metadata: Option<Metadata>) {
		// Optional: containers that support metadata override this
	}
}
