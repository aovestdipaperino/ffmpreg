use crate::core::Transform;
use crate::core::frame::Frame;
use crate::message::Result;

pub struct Normalize {}

impl Normalize {}

impl Transform for Normalize {
	fn apply(&mut self, frame: Frame) -> Result<Frame> {
		Ok(frame)
	}

	fn name(&self) -> &'static str {
		"normalize"
	}
}
