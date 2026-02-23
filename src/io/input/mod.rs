pub mod iter;

use crate::core::packet::Packet;
use crate::core::resolver::ContainerResolver;
use crate::core::track::Metadata;
use crate::core::{Demuxer, Tracks};
use crate::io::File;
use crate::message::Result;
use crate::utils;
pub use iter::InputPacketIter;
use std::path::{Path, PathBuf};

pub struct Input {
	pub path: PathBuf,
	demuxer: Box<dyn Demuxer>,
	pub tracks: Tracks,
}

impl Input {
	pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
		let resolver = ContainerResolver::new();
		Self::from_resolver(path, &resolver)
	}

	#[inline]
	pub fn from_resolver<P: AsRef<Path>>(path: P, resolver: &ContainerResolver) -> Result<Self> {
		let path_ref = path.as_ref();
		let extension = utils::extension_from_path(path_ref)?;
		let path_str = path_ref.to_string_lossy();

		let file = File::open(&path_str)?;
		let demuxer = resolver.open_demuxer(&extension, file)?;
		let tracks = demuxer.tracks();
		Ok(Self { path: path_ref.to_path_buf(), demuxer, tracks })
	}

	#[inline(always)]
	pub fn read_packet(&mut self) -> Result<Option<Packet>> {
		self.demuxer.read()
	}

	#[inline(always)]
	pub fn iter_packets(&mut self) -> InputPacketIter<'_> {
		InputPacketIter::new(self)
	}

	#[inline]
	pub fn metadata(&self) -> &Metadata {
		self.demuxer.metadata()
	}

	pub fn duration(&self) -> Option<f64> {
		self.demuxer.duration()
	}
}

impl Iterator for Input {
	type Item = Result<Packet>;

	fn next(&mut self) -> Option<Self::Item> {
		match self.read_packet() {
			Ok(Some(packet)) => Some(Ok(packet)),
			Ok(None) => None,
			Err(e) => Some(Err(e)),
		}
	}
}
