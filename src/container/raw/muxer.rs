use super::format::{RawPcmFormat, RawPcmMetadata};
use crate::core::Muxer;
use crate::core::packet::Packet;
use crate::core::stream::{self, Stream};
use crate::core::time::Time;
use crate::io::{BinaryWrite, MediaSeek, MediaWrite};
use crate::message::Result;

pub struct RawPcmMuxer<W: MediaWrite + MediaSeek> {
	writer: W,
	#[allow(dead_code)]
	format: RawPcmFormat,
	streams: stream::Streams,
	metadata: Option<RawPcmMetadata>,
	#[allow(dead_code)]
	data_size: u32,
}

impl<W: MediaWrite + MediaSeek> RawPcmMuxer<W> {
	pub fn new(writer: W, format: RawPcmFormat) -> Result<Self> {
		let codec_name = format.to_codec_string().to_string();
		let time = Time::new(1, format.sample_rate)?;
		let mut streams = stream::Streams::new_empty();
		let stream_id = streams.next_id();
		let stream = Stream::new_audio(stream_id, codec_name, time);

		streams.add(stream);

		Ok(Self { writer, format, streams, metadata: None, data_size: 0 })
	}

	pub fn with_metadata(&mut self, metadata: Option<RawPcmMetadata>) {
		self.metadata = metadata;
	}

	pub fn write_packet(&mut self, packet: Packet) -> Result<()> {
		self.writer.write_all(&packet.data)?;
		self.data_size += packet.data.len() as u32;
		Ok(())
	}

	pub fn finalize(&mut self) -> Result<()> {
		self.writer.flush()?;
		Ok(())
	}
}

impl<W: MediaWrite + MediaSeek> Muxer for RawPcmMuxer<W> {
	fn streams(&self) -> &stream::Streams {
		&self.streams
	}
	fn write(&mut self, packet: Packet) -> Result<()> {
		self.write_packet(packet)
	}
	fn finalize(&mut self) -> Result<()> {
		self.finalize()
	}
}
