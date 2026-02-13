#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameSubtitle {
	pub text: String,
	pub start: u64,
	pub end: u64,
}

impl FrameSubtitle {
	pub fn new(text: impl Into<String>, start: u64, end: u64) -> Self {
		Self { text: text.into(), start, end }
	}

	pub fn empty(start: u64, end: u64) -> Self {
		Self { text: String::new(), start, end }
	}

	pub fn duration(&self) -> u64 {
		self.end.saturating_sub(self.start)
	}

	pub fn is_valid(&self) -> bool {
		self.start < self.end
	}

	pub fn with_text(mut self, text: impl Into<String>) -> Self {
		self.text = text.into();
		self
	}

	pub fn with_timestamps(mut self, start: u64, end: u64) -> Self {
		self.start = start;
		self.end = end;
		self
	}
}
