use crate::core::selector::Selector;
use crate::core::{Transform, frame::Frame};
use crate::message::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
	Video,
	Audio,
	Subtitle,
}

pub struct Metadata {
	pub transform: Box<dyn Transform>,
	pub selector: Selector,
	pub kind: Option<Kind>,
}

impl Metadata {
	pub fn matches(&self, frame: &Frame) -> bool {
		match self.selector {
			Selector::All => true,
			Selector::Id(id) => frame.track_id == id,
		}
	}
}

pub struct Transforms {
	list: Vec<Metadata>,
}

impl Transforms {
	pub fn new() -> Self {
		Self { list: Vec::new() }
	}

	pub fn add(&mut self, transform: Box<dyn Transform>, selector: Selector) {
		self.list.push(Metadata { transform, selector, kind: None });
	}
	pub fn add_video(&mut self, transform: Box<dyn Transform>, selector: Selector) {
		self.list.push(Metadata { transform, selector, kind: Some(Kind::Video) });
	}
	pub fn add_audio(&mut self, transform: Box<dyn Transform>, selector: Selector) {
		self.list.push(Metadata { transform, selector, kind: Some(Kind::Audio) });
	}
	pub fn add_subtitle(&mut self, transform: Box<dyn Transform>, selector: Selector) {
		self.list.push(Metadata { transform, selector, kind: Some(Kind::Subtitle) });
	}

	pub fn clear(&mut self) {
		self.list.clear();
	}
	pub fn reverse(&mut self) {
		self.list.reverse();
	}

	pub fn apply(&mut self, mut frame: Frame) -> Result<Frame> {
		for meta in self.list.iter_mut() {
			if !meta.matches(&frame) {
				continue;
			}
			frame = meta.transform.apply(frame)?;
		}
		Ok(frame)
	}
}
