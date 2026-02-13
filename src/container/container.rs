#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct ContainerId {
	pub name: &'static str,
	pub extensions: &'static [&'static str],
}

impl ContainerId {
	pub const fn new(name: &'static str) -> Self {
		Self { name, extensions: &[] }
	}

	pub const fn exts(mut self, exts: &'static [&'static str]) -> Self {
		self.extensions = exts;
		self
	}
}

impl std::hash::Hash for ContainerId {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.name.hash(state);
	}
}
