#![allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Indenter {
	pub unit: &'static str, // string used for one level of indent
	pub level: usize,       // current indent level
	pub default: usize,     // default/base indent level
}

impl Default for Indenter {
	fn default() -> Self {
		Self { unit: "  ", level: 2, default: 2 }
	}
}

impl Indenter {
	pub fn new(unit: &'static str, level: usize) -> Self {
		Self { unit, level, default: level }
	}

	pub fn from_level(level: usize) -> Self {
		Self { unit: "  ", level, default: level }
	}

	pub const fn over(&mut self) -> &mut Self {
		self.level += 1;
		self
	}

	pub const fn under(&mut self) -> &mut Self {
		self.level = self.level.saturating_sub(1);
		self
	}

	pub const fn reset(&mut self) -> &mut Self {
		self.level = self.default;
		self
	}

	#[inline(always)]
	pub fn indent(&self) -> String {
		self.unit.repeat(self.level)
	}
}

impl std::fmt::Display for Indenter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.indent())
	}
}
