use crate::converter::options::{AudioOption, TransformOption};
use crate::core::Selector;
use crate::core::traits::Transform;
use crate::transforms::register_transforms;
use crate::{error, message};

pub struct TransformResolver {
	entries: super::TransformMapper,
}

impl TransformResolver {
	pub fn new() -> Self {
		let entries = register_transforms();
		Self { entries }
	}

	pub fn resolve(&self, name: &str, value: &str) -> message::Result<Box<dyn Transform>> {
		let factory = self.entries.get(name);
		match factory {
			Some(factory) => factory(value),
			None => Err(error!("transform '{}' is not supported", name)),
		}
	}

	pub fn from_transform(
		&self,
		option: &TransformOption,
	) -> message::Result<Vec<(Selector, Box<dyn Transform>)>> {
		let mut resolved = Vec::new();

		if let Some(value) = &option.normalize {
			let transform = self.resolve("normalize", value)?;
			resolved.push((option.track, transform));
		}

		if let Some(value) = &option.speed {
			let transform = self.resolve("speed", value)?;
			resolved.push((option.track, transform));
		}

		if let Some(value) = &option.trim {
			let transform = self.resolve("trim", value)?;
			resolved.push((option.track, transform));
		}

		if let Some(value) = &option.fade {
			let transform = self.resolve("fade", value)?;
			resolved.push((option.track, transform));
		}

		if let Some(value) = &option.reverse {
			let transform = self.resolve("reverse", value)?;
			resolved.push((option.track, transform));
		}

		if let Some(value) = &option.rotate {
			let transform = self.resolve("rotate", value)?;
			resolved.push((option.track, transform));
		}

		Ok(resolved)
	}

	pub fn from_audio(
		&self,
		option: &AudioOption,
	) -> message::Result<Vec<(Selector, Box<dyn Transform>)>> {
		let mut resolved = Vec::new();

		if let Some(value) = &option.volume {
			let transform = self.resolve("volume", value)?;
			resolved.push((option.selector, transform));
		}

		Ok(resolved)
	}
}
