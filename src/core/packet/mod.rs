use crate::core::time::Time;
mod iter;
mod writer;
pub use iter::*;
pub use writer::*;

#[derive(Debug, Clone)]
pub struct Packet {
	pub data: Vec<u8>,
	pub pts: i64,
	pub dts: i64,
	pub time: Time,
	pub track_id: usize,
	pub keyframe: bool,
	pub discard: bool,
	pub samples: Option<u64>,
}

impl Packet {
	pub fn new(data: Vec<u8>, track_id: usize, time: Time) -> Self {
		Self { data, pts: 0, dts: 0, time, track_id, keyframe: false, discard: false, samples: None }
	}

	pub fn with_pts(mut self, pts: i64) -> Self {
		self.pts = pts;
		self
	}

	pub fn with_dts(mut self, dts: i64) -> Self {
		self.dts = dts;
		self
	}

	pub fn with_keyframe(mut self, keyframe: bool) -> Self {
		self.keyframe = keyframe;
		self
	}

	pub fn with_samples(mut self, samples: u64) -> Self {
		self.samples = Some(samples);
		self
	}

	pub fn size(&self) -> usize {
		self.data.len()
	}

	pub fn is_empty(&self) -> bool {
		self.data.is_empty()
	}
}
