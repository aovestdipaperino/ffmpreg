use super::format;
use crate::core::Muxer;
use crate::core::packet::Packet;
use crate::core::stream::{self, Stream};
use crate::core::time::Time;
use crate::io::{BinaryWrite, MediaSeek, MediaWrite};
use crate::message::Result;

pub struct FlacMuxer<W: MediaWrite + MediaSeek> {
	writer: W,
	#[allow(dead_code)]
	format: format::FlacFormat,
	streams: stream::Streams,
	metadata: Option<format::FlacMetadata>,
	#[allow(dead_code)]
	data_size: u32,
}

impl<W: MediaWrite + MediaSeek> FlacMuxer<W> {
	pub fn new(mut writer: W, format: format::FlacFormat) -> Result<Self> {
		Self::write_header(&mut writer, &format)?;
		writer.flush()?;

		let codec_name = format.to_codec_string().to_string();
		let time = Time::new(1, format.sample_rate)?;
		let mut streams = stream::Streams::new_empty();
		let stream_id = streams.next_id();
		let stream = Stream::new_audio(stream_id, codec_name, time);

		streams.add(stream);

		Ok(Self { writer, format, streams, metadata: None, data_size: 0 })
	}

	pub fn with_metadata(&mut self, metadata: Option<format::FlacMetadata>) {
		self.metadata = metadata;
	}

	fn write_header(writer: &mut W, format: &format::FlacFormat) -> Result<()> {
		writer.write_all(b"fLaC")?;

		// Write STREAMINFO metadata block
		let mut streaminfo = vec![0u8; 34];

		// Set min/max block sizes (4 bytes each, initially 0)
		streaminfo[0..4].copy_from_slice(&[0, 16, 0xff, 0xff]);

		// Set min/max frame sizes (24-bit each, initially 0)
		streaminfo[4..10].copy_from_slice(&[0, 0, 0, 0, 0, 0]);

		// Sample rate (20 bits) | channels (3 bits) | bit depth (5 bits) | samples (36 bits)
		let sample_rate = format.sample_rate;
		let channels = (format.channels.count() as u8) - 1;
		let bit_depth = (format.bit_depth - 1) as u8;

		streaminfo[10] = ((sample_rate >> 12) & 0xff) as u8;
		streaminfo[11] = ((sample_rate >> 4) & 0xff) as u8;
		streaminfo[12] = ((sample_rate & 0xf) << 4) as u8 | ((channels & 0x7) << 1);
		streaminfo[12] |= ((bit_depth >> 4) & 0x1) as u8;
		streaminfo[13] = ((bit_depth & 0xf) << 4) as u8;

		// Write metadata block header
		let is_last = true;
		let block_type = 0u8; // STREAMINFO

		let mut block_header = if is_last { 0x80 } else { 0x00 };
		block_header |= block_type & 0x7f;
		writer.write_u8(block_header)?;

		// Write block size (24-bit big-endian)
		let size = 34u32;
		writer.write_u8((size >> 16) as u8)?;
		writer.write_u8((size >> 8) as u8)?;
		writer.write_u8(size as u8)?;

		writer.write_all(&streaminfo)?;

		Ok(())
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

impl<W: MediaWrite + MediaSeek> Muxer for FlacMuxer<W> {
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
