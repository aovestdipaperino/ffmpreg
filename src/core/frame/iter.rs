use crate::core::frame::Frame;

pub struct FrameIter {
	frames: Vec<Frame>,
}

impl FrameIter {
	pub fn new(frames: Vec<Frame>) -> Self {
		Self { frames }
	}

	pub fn empty() -> Self {
		Self { frames: Vec::new() }
	}
}

impl Iterator for FrameIter {
	type Item = Frame;

	fn next(&mut self) -> Option<Self::Item> {
		self.frames.pop()
	}
}
