use crate::core::packet::Packet;
use crate::core::resolver::ContainerResolver;
use crate::core::track::Metadata;
use crate::core::{Muxer, track::Format};
use crate::io::File;
use crate::message::Result;
use crate::utils;
use std::path::{Path, PathBuf};

pub struct Output {
	#[allow(dead_code)]
	pub path: PathBuf,
	pub extension: String,
	format: Format,
	muxer: Box<dyn Muxer>,
}

impl Output {
	pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
		let resolver = ContainerResolver::new();
		Self::from_resolver(path, &resolver)
	}

	#[inline]
	pub fn from_resolver<P: AsRef<Path>>(path: P, resolver: &ContainerResolver) -> Result<Self> {
		let path_ref = path.as_ref();
		let extension = utils::extension_from_path(path_ref)?;
		let path_str = path_ref.to_string_lossy();

		let container = resolver.resolver_for(&extension)?;
		let format = Format::from_container(container)?;

		let file = File::create(&path_str)?;
		let muxer = resolver.open_muxer(&extension, file, &format)?;

		Ok(Self { path: path_ref.to_path_buf(), extension, format, muxer })
	}

	pub const fn format(&self) -> &Format {
		&self.format
	}

	#[inline]
	pub fn with_metadata(&mut self, metadata: impl Into<Metadata>) -> &mut Self {
		self.muxer.set_metadata(Some(metadata.into()));
		self
	}

	#[inline(always)]
	pub fn write_packet(&mut self, packet: Packet) -> Result<()> {
		self.muxer.write(packet)
	}

	#[inline(always)]
	pub fn flush(&mut self) -> Result<()> {
		self.muxer.finalize()
	}

	#[inline(always)]
	pub fn finalize(&mut self) -> Result<()> {
		self.muxer.finalize()
	}

	#[inline]
	pub fn write_all<I>(&mut self, packets: I) -> Result<()>
	where
		I: IntoIterator<Item = Packet>,
	{
		for packet in packets {
			self.write_packet(packet)?;
		}
		Ok(())
	}
}
