#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct CodecId {
	pub name: &'static str,
	pub aliases: &'static [&'static str],
	pub extensions: &'static [&'static str],
}

impl std::hash::Hash for CodecId {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.name.hash(state);
	}
}

impl CodecId {
	pub const fn new(name: &'static str) -> Self {
		Self { name, aliases: &[], extensions: &[] }
	}

	pub const fn aliases(mut self, aliases: &'static [&'static str]) -> Self {
		self.aliases = aliases;
		self
	}

	pub const fn exts(mut self, exts: &'static [&'static str]) -> Self {
		self.extensions = exts;
		self
	}

	pub fn supports_extension(&self, ext: &str) -> bool {
		self.extensions.iter().any(|e| *e == ext)
	}

	pub fn primary_extension(&self) -> Option<&'static str> {
		self.extensions.first().copied()
	}
}

impl From<&'static str> for CodecId {
	fn from(name: &'static str) -> Self {
		Self::new(name)
	}
}

impl From<CodecId> for &'static str {
	fn from(id: CodecId) -> Self {
		id.name
	}
}

impl std::fmt::Display for CodecId {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.name)
	}
}
