use core::fmt;
use std::fmt::Display;

use crate::core::time::Time;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StreamKind {
	Audio,
	Video,
	Subtitle,
}

impl Display for StreamKind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let text = match self {
			StreamKind::Audio => "audio",
			StreamKind::Video => "video",
			StreamKind::Subtitle => "subtitle",
		};
		write!(f, "{}", text)
	}
}

#[derive(Debug, Clone)]
pub struct Stream {
	pub id: u32,
	pub kind: StreamKind,
	pub codec: String,
	pub time: Time,
	pub codec_private: Vec<u8>,
}

impl Stream {
	pub fn new(id: u32, kind: StreamKind, codec: String, time: Time) -> Self {
		Self { id, kind, codec, time, codec_private: Vec::new() }
	}

	pub fn new_audio(id: u32, codec: String, time: Time) -> Self {
		Self::new(id, StreamKind::Audio, codec, time)
	}

	pub fn new_video(id: u32, codec: String, time: Time) -> Self {
		Self::new(id, StreamKind::Video, codec, time)
	}

	pub fn new_subtitle(id: u32, codec: String, time: Time) -> Self {
		Self::new(id, StreamKind::Subtitle, codec, time)
	}

	pub fn with_codec_private(mut self, codec_private: Vec<u8>) -> Self {
		self.codec_private = codec_private;
		self
	}

	#[inline(always)]
	pub fn audio_kind(&self) -> bool {
		matches!(self.kind, StreamKind::Audio)
	}

	#[inline(always)]
	pub fn video_kind(&self) -> bool {
		matches!(self.kind, StreamKind::Video)
	}

	#[inline(always)]
	pub fn subtitle_kind(&self) -> bool {
		matches!(self.kind, StreamKind::Subtitle)
	}
}

#[derive(Debug, Clone)]
pub struct Streams {
	inner: Vec<Stream>,
}

impl Streams {
	pub fn new(inner: Vec<Stream>) -> Self {
		Self { inner }
	}

	pub fn new_empty() -> Self {
		Self { inner: Vec::new() }
	}

	pub fn add(&mut self, stream: Stream) -> u32 {
		let index = self.inner.len() as u32;
		self.inner.push(stream);
		index
	}

	pub fn next_id(&self) -> u32 {
		self.inner.len() as u32
	}

	pub fn all(&self) -> &[Stream] {
		&self.inner
	}

	pub fn get(&self, id: u32) -> Option<&Stream> {
		self.inner.get(id as usize)
	}

	pub fn audio(&self) -> impl Iterator<Item = &Stream> {
		self.inner.iter().filter(|s| s.audio_kind())
	}

	pub fn video(&self) -> impl Iterator<Item = &Stream> {
		self.inner.iter().filter(|s| s.video_kind())
	}

	pub fn subtitle(&self) -> impl Iterator<Item = &Stream> {
		self.inner.iter().filter(|s| s.subtitle_kind())
	}

	pub fn count_audio(&self) -> usize {
		self.audio().count()
	}
}

impl Display for Stream {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "stream id: {} kind: {} codec: {}", self.id, self.kind, self.codec)
	}
}

impl Display for Streams {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for stream in &self.inner {
			write!(f, "{}\n", stream)?;
		}
		Ok(())
	}
}
