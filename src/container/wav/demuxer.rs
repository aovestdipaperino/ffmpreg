use super::format::{WavFormat, WavMetadata};
use super::header::WavHeader;
use crate::container::constants::CHUNK_SIZE_LIMIT;
use crate::core::Demuxer;
use crate::core::frame::Channels;
use crate::core::packet::Packet;
use crate::core::stream::{Stream, Streams};
use crate::core::time::Time;
use crate::io::{BinaryRead, MediaRead};
use crate::{error, message::Result};

pub struct WavDemuxer<R: MediaRead> {
	reader: R,
	format: WavFormat,
	streams: Streams,
	metadata: WavMetadata,
	remaining_bytes: u64,
	samples_read: u64,
	packet_count: u64,
	time: Time,
}

impl<R: MediaRead> WavDemuxer<R> {

	pub fn new(mut reader: R) -> Result<Self> {
		let (header, metadata, data_size) = Self::read_wav_and_find_data(&mut reader)?;
		header.validate()?;
		let mut streams = Streams::new_empty();

		let format = header.to_format();
		let time = Time::new(1, header.sample_rate)?;
		let codec = format.to_codec_string().to_string();
		let stream_id = streams.next_id();
		let stream = Stream::new_audio(stream_id, codec, time);
		streams.add(stream);

		Ok(Self {
			reader,
			format,
			streams,
			metadata,
			remaining_bytes: data_size,
			samples_read: 0,
			packet_count: 0,
			time,
		})
	}

	fn read_wav_and_find_data(reader: &mut R) -> Result<(WavHeader, WavMetadata, u64)> {
		Self::check_fourcc(reader, "RIFF")?;
		let _file_size = reader.read_u32_le()?;
		Self::check_fourcc(reader, "WAVE")?;

		let mut header = WavHeader::default();
		let mut metadata = WavMetadata::new();

		loop {
			let chunk_id = Self::read_fourcc(reader)?;
			let chunk_size = reader.read_u32_le()? as u64;

			match chunk_id.as_str() {
				"fmt " => Self::read_fmt_chunk(reader, chunk_size, &mut header)?,
				"LIST" => Self::read_list_chunk(reader, chunk_size, &mut metadata)?,
				"data" => return Ok((header, metadata, chunk_size)),
				_ => Self::skip_bytes(reader, chunk_size)?,
			}
		}
	}

	fn read_fmt_chunk(reader: &mut R, chunk_size: u64, header: &mut WavHeader) -> Result<()> {
		if chunk_size < 16 {
			return Err(error!("fmt chunk too small"));
		}

		header.format_code = reader.read_u16_le()?;
		header.channels = Channels::from_count(reader.read_u16_le()? as u8);
		header.sample_rate = reader.read_u32_le()?;
		header.byte_rate = reader.read_u32_le()?;
		header.block_align = reader.read_u16_le()?;
		header.bits_per_sample = reader.read_u16_le()?;

		if chunk_size > 16 {
			Self::skip_bytes(reader, chunk_size - 16)?;
		}
		Ok(())
	}

	fn read_list_chunk(reader: &mut R, chunk_size: u64, metadata: &mut WavMetadata) -> Result<()> {
		if chunk_size < 4 {
			return Ok(());
		}

		let form_type = Self::read_fourcc(reader)?;
		if form_type != "INFO" {
			return Self::skip_bytes(reader, chunk_size - 4);
		}

		let mut pos = 4u64;
		while pos + 8 <= chunk_size {
			let id = Self::read_fourcc(reader)?;
			let size = reader.read_u32_le()? as u64;
			pos += 8;

			let data = Self::read_bytes(reader, size)?;
			pos += size;

			let value = String::from_utf8_lossy(&data).trim_end_matches('\0').to_string();
			match id.as_str() {
				"IART" => metadata.set("artist", value),
				"INAM" => metadata.set("title", value),
				"ICOM" => metadata.set("comment", value),
				"ICOP" => metadata.set("copyright", value),
				"ISFT" => metadata.set("software", value),
				"IGNR" => metadata.set("genre", value),
				"ITRK" => metadata.set("track", value),
				_ => {}
			}

			if size % 2 == 1 {
				reader.read_u8()?;
				pos += 1;
			}
		}
		Ok(())
	}

	fn read_fourcc(reader: &mut R) -> Result<String> {
		let mut buf = [0u8; 4];
		reader.read_exact(&mut buf)?;
		Ok(String::from_utf8_lossy(&buf).to_string())
	}

	fn check_fourcc(reader: &mut R, expected: &str) -> Result<()> {
		let actual = Self::read_fourcc(reader)?;
		if actual != expected {
			return Err(error!("expected {}, found {}", expected, actual));
		}
		Ok(())
	}

	fn read_bytes(reader: &mut R, size: u64) -> Result<Vec<u8>> {
		let mut buf = vec![0u8; size as usize];
		reader.read_exact(&mut buf)?;
		Ok(buf)
	}

	fn skip_bytes(reader: &mut R, mut size: u64) -> Result<()> {
		const BUF_SIZE: usize = 8192;
		let mut buf = [0u8; BUF_SIZE];
		while size > 0 {
			let read_size = std::cmp::min(size, BUF_SIZE as u64) as usize;
			reader.read_exact(&mut buf[..read_size])?;
			size -= read_size as u64;
		}
		Ok(())
	}

	pub fn read_packet(&mut self) -> Result<Option<Packet>> {
		if self.remaining_bytes == 0 {
			return Ok(None);
		}

		let block_align = self.format.block_align() as u64;
		let max_chunk = (CHUNK_SIZE_LIMIT as u64 / block_align) * block_align;
		let chunk_size = std::cmp::min(self.remaining_bytes, max_chunk) as usize;

		let mut data = vec![0u8; chunk_size];
		let bytes_read = self.reader.read(&mut data)?;
		if bytes_read == 0 {
			return Ok(None);
		}

		data.truncate(bytes_read);
		self.remaining_bytes -= bytes_read as u64;

		let packet = Packet::new(data, 0, self.time).with_pts(self.samples_read as i64);

		self.samples_read += (bytes_read / self.format.bytes_per_frame()) as u64;
		self.packet_count += 1;

		Ok(Some(packet))
	}

	pub fn format(&self) -> WavFormat {
		self.format
	}
	pub fn metadata(&self) -> &WavMetadata {
		&self.metadata
	}
}

impl<R: MediaRead> Demuxer for WavDemuxer<R> {
	fn streams(&self) -> &Streams {
		&self.streams
	}
	fn read_packet(&mut self) -> Result<Option<Packet>> {
		self.read_packet()
	}
}
