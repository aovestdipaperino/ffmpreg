use super::format::{RawPcmFormat, RawPcmMetadata};
use crate::container::constants::CHUNK_SIZE_LIMIT;
use crate::core::packet::Packet;
use crate::core::{Demuxer, stream, time};
use crate::io::MediaRead;
use crate::message::Result;

pub struct RawPcmDemuxer<R: MediaRead> {
	reader: R,
	format: RawPcmFormat,
	streams: stream::Streams,
	metadata: RawPcmMetadata,
	remaining_bytes: u64,
	samples_read: u64,
	packet_count: u64,
	time: time::Time,
}

impl<R: MediaRead> RawPcmDemuxer<R> {
	pub fn new(reader: R, format: RawPcmFormat) -> Result<Self> {
		let codec_name = format.to_codec_string().to_string();
		let time = time::Time::new(1, format.sample_rate)?;
		let mut streams = stream::Streams::new_empty();
		let stream_id = streams.next_id();
		let stream = stream::Stream::new_audio(stream_id, codec_name, time);
		streams.add(stream);

		Ok(Self {
			reader,
			format,
			streams,
			metadata: RawPcmMetadata::new(),
			remaining_bytes: u64::MAX,
			samples_read: 0,
			packet_count: 0,
			time,
		})
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

	pub fn format(&self) -> RawPcmFormat {
		self.format
	}

	pub fn metadata(&self) -> &RawPcmMetadata {
		&self.metadata
	}
}

impl<R: MediaRead> Demuxer for RawPcmDemuxer<R> {
	fn streams(&self) -> &stream::Streams {
		&self.streams
	}
	fn read_packet(&mut self) -> Result<Option<Packet>> {
		self.read_packet()
	}
}
