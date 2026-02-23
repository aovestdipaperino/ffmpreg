use crate::core::frame::Frame;

pub struct FrameIter {
	inner: std::vec::IntoIter<Frame>,
}

impl FrameIter {
	pub fn new(frames: Vec<Frame>) -> Self {
		Self { inner: frames.into_iter() }
	}

	pub fn empty() -> Self {
		Self { inner: Vec::new().into_iter() }
	}
}

impl Iterator for FrameIter {
	type Item = Frame;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item> {
		self.inner.next()
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.inner.size_hint()
	}
}
