use super::format::{FlacFormat, FlacMetadata};
use crate::container::CHUNK_SIZE_LIMIT;
use crate::core::Demuxer;
use crate::core::frame::Channels;
use crate::core::packet::Packet;
use crate::core::stream::{Stream, Streams};
use crate::core::time::Time;
use crate::io::{BinaryRead, MediaRead};
use crate::{error, message::Result};

pub struct FlacDemuxer<R: MediaRead> {
	reader: R,
	format: FlacFormat,
	streams: Streams,
	metadata: FlacMetadata,
	remaining_bytes: u64,
	samples_read: u64,
	packet_count: u64,
	time: Time,
}

impl<R: MediaRead> FlacDemuxer<R> {
	pub fn new(mut reader: R) -> Result<Self> {
		let (format, metadata, data_size) = Self::read_flac_and_find_data(&mut reader)?;

		let time = Time::new(1, format.sample_rate)?;
		let codec = format.to_codec_string().to_string();
		let mut streams = Streams::new_empty();
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

	fn read_flac_and_find_data(reader: &mut R) -> Result<(FlacFormat, FlacMetadata, u64)> {
		Self::check_fourcc(reader, "fLaC")?;

		let mut format = FlacFormat::new(Channels::Stereo, 44100, 16);
		let metadata = FlacMetadata::new();

		loop {
			let last_block_byte = reader.read_u8()?;
			let is_last = (last_block_byte & 0x80) != 0;
			let block_type = last_block_byte & 0x7f;

			let mut size_bytes = [0u8; 3];
			reader.read_exact(&mut size_bytes)?;
			let block_size = u32::from_be_bytes([0, size_bytes[0], size_bytes[1], size_bytes[2]]) as u64;

			match block_type {
				0 => Self::read_streaminfo(reader, block_size, &mut format)?,
				1..=6 => Self::skip_bytes(reader, block_size)?,
				_ => return Err(error!("unknown metadata block type: {}", block_type)),
			}

			if is_last {
				let data_size = Self::calculate_remaining_size(reader)?;
				return Ok((format, metadata, data_size));
			}
		}
	}

	fn read_streaminfo(reader: &mut R, size: u64, format: &mut FlacFormat) -> Result<()> {
		if size < 34 {
			return Err(error!("streaminfo block too small"));
		}

		let mut buf = [0u8; 34];
		reader.read_exact(&mut buf)?;

		let _min_block_size = u16::from_be_bytes([buf[0], buf[1]]);
		let _max_block_size = u16::from_be_bytes([buf[2], buf[3]]);
		let _min_frame_size = u32::from_be_bytes([0, buf[4], buf[5], buf[6]]);
		let _max_frame_size = u32::from_be_bytes([0, buf[7], buf[8], buf[9]]);

		let sample_rate_bytes = [buf[10], buf[11], buf[12], 0];
		let sample_rate = (u32::from_be_bytes(sample_rate_bytes) >> 4) as u32;

		let channels_bits = (buf[12] >> 1) & 0x7;
		let channels = Channels::from_count((channels_bits as u8) + 1);

		let bit_depth_bits = ((buf[12] & 1) << 4) | ((buf[13] >> 4) & 0xf);
		let bit_depth = (bit_depth_bits as u16) + 1;

		*format = FlacFormat::new(channels, sample_rate, bit_depth);

		if size > 34 {
			Self::skip_bytes(reader, size - 34)?;
		}

		Ok(())
	}

	fn calculate_remaining_size(_reader: &mut R) -> Result<u64> {
		// This is a simplified implementation
		// In a real scenario, you'd need to seek to end and calculate
		Ok(0)
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

	pub fn format(&self) -> FlacFormat {
		self.format
	}

	pub fn metadata(&self) -> &FlacMetadata {
		&self.metadata
	}
}

impl<R: MediaRead> Demuxer for FlacDemuxer<R> {
	fn streams(&self) -> &Streams {
		&self.streams
	}
	fn read_packet(&mut self) -> Result<Option<Packet>> {
		self.read_packet()
	}
}
