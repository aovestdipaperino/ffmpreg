use crate::core::Transform;
use crate::core::frame::Frame;
use crate::message::Result;

#[allow(dead_code)]
pub struct Volume {
	factor: f32,
}

impl Volume {
	pub fn new(factor: f32) -> Self {
		Self { factor }
	}
}

impl Transform for Volume {
	fn apply(&mut self, frame: Frame) -> Result<Frame> {
		Ok(frame)
	}

	fn name(&self) -> &'static str {
		"volume"
	}
}
