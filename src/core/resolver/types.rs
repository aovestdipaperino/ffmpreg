use rustc_hash::FxHashMap;

use crate::container::ContainerId;
use crate::core::track::Format;
use crate::core::{CodecId, Decoder, Encoder, traits};
use crate::core::{Demuxer, Muxer, Track};
use crate::io::File;
use crate::message::Result;

pub type DecoderFactory = fn(&Track) -> Result<Box<dyn Decoder>>;
pub type EncoderFactory = fn(&Track, &Format) -> Result<Box<dyn Encoder>>;

pub type CodecFactoryMapper = FxHashMap<CodecId, CodecEntry>;
pub type CodecRegister = fn(&mut CodecFactoryMapper);

pub type CodecIdMapper = FxHashMap<&'static str, CodecId>;
pub type CodecIdMapperRegister = fn(&mut CodecIdMapper);

pub type CodecExtMapper = FxHashMap<&'static str, CodecId>;
pub type CodecExtMapperRegister = fn(&mut CodecExtMapper);

#[derive(Clone, Copy, Default)]
pub struct CodecEntry {
	pub decoder: Option<DecoderFactory>,
	pub encoder: Option<EncoderFactory>,
}

// containers

pub type DemuxerFactory = fn(File) -> Result<Box<dyn Demuxer>>;
pub type MuxerFactory = fn(File, &Format) -> Result<Box<dyn Muxer>>;

#[derive(Clone, Copy, Default)]
pub struct ContainerEntry {
	pub demuxer: Option<DemuxerFactory>,
	pub muxer: Option<MuxerFactory>,
}

pub type ContainerMapper = FxHashMap<ContainerId, ContainerEntry>;
pub type ContainerExtMapper = FxHashMap<&'static str, ContainerId>;

// transforms

pub type TransformFactory = fn(&str) -> Result<Box<dyn traits::Transform>>;

pub type TransformMapper = FxHashMap<&'static str, TransformFactory>;
pub type TransformRegister = fn(&mut TransformMapper);
