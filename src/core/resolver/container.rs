use crate::container::{ContainerId, register_containers};
use crate::core::track::Format;
use crate::core::{Demuxer, Muxer};
use crate::error;
use crate::io::File;
use crate::message::Result;

pub struct ContainerResolver {
	entries: super::ContainerMapper,
	extensions: super::ContainerExtMapper,
}

impl ContainerResolver {
	pub fn new() -> Self {
		let (entries, extensions) = register_containers();
		Self { entries, extensions }
	}

	pub fn resolver_for(&self, extension: &str) -> Result<ContainerId> {
		let container = self.extensions.get(extension).copied();
		container.ok_or_else(|| error!("container '{}' is not supported", extension))
	}

	pub fn register_demuxer(&mut self, container_id: ContainerId, factory: super::DemuxerFactory) {
		let entry = self.entries.entry(container_id).or_default();
		entry.demuxer = Some(factory);
	}

	pub fn register_muxer(&mut self, container_id: ContainerId, factory: super::MuxerFactory) {
		let entry = self.entries.entry(container_id).or_default();
		entry.muxer = Some(factory);
	}

	pub fn demuxer_for(&self, container: ContainerId, file: File) -> Result<Box<dyn Demuxer>> {
		let factory = self.entries.get(&container).map(|entry| entry.demuxer);
		if let Some(Some(factory)) = factory {
			return factory(file);
		}
		Err(error!("demuxer '{}' is not supported", container.name))
	}

	pub fn muxer_for(
		&self,
		container: ContainerId,
		file: File,
		format: &Format,
	) -> Result<Box<dyn Muxer>> {
		let factory = self.entries.get(&container).map(|entry| entry.muxer);
		if let Some(Some(factory)) = factory {
			return factory(file, format);
		}
		Err(error!("muxer '{}' is not supported", container.name))
	}

	pub fn open_demuxer(&self, extension: &str, file: File) -> Result<Box<dyn Demuxer>> {
		let container = self.resolver_for(extension)?;
		self.demuxer_for(container, file)
	}

	pub fn open_muxer(&self, extension: &str, file: File, format: &Format) -> Result<Box<dyn Muxer>> {
		let container = self.resolver_for(extension)?;
		self.muxer_for(container, file, format)
	}
}

impl Default for ContainerResolver {
	fn default() -> Self {
		Self::new()
	}
}
