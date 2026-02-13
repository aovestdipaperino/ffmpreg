use crate::core::Tracks;
use crate::core::packet::Packet;
use crate::core::track::Metadata;
use crate::message::Result;

pub trait Demuxer {
	fn read(&mut self) -> Result<Option<Packet>>;

	fn seek(&mut self, time: f64) -> Result<()>;

	fn duration(&self) -> Option<f64>;

	fn tracks(&self) -> Tracks;

	fn metadata(&self) -> &Metadata;
}
