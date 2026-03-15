use crate::core::packet::Packet;
use crate::core::stream::{Stream, StreamKind, Streams};
use crate::core::time::Time;
use crate::core::traits::Demuxer;
use crate::io::{MediaRead, ReadPrimitives};
use crate::message::Result;

/// Reads MP3 files, skipping ID3 tags, and emitting chunks of raw MPEG audio data.
pub struct Mp3Demuxer<R: MediaRead> {
	reader: R,
	streams: Streams,
	sample_rate: u32,
	packet_count: u64,
	eof: bool,
	/// Buffer for data read during probing that hasn't been returned yet.
	stashed: Vec<u8>,
}

impl<R: MediaRead> Mp3Demuxer<R> {
	/// Chunk size for reading MP3 data. Large enough to contain several MP3 frames.
	const CHUNK_SIZE: usize = 16384;

	pub fn new(mut reader: R) -> Result<Self> {
		// Skip ID3v2 tag if present
		let mut header = [0u8; 10];
		reader.read_exact(&mut header)?;

		let mut preamble = Vec::new();

		if &header[..3] == b"ID3" {
			// ID3v2: size is 4 bytes syncsafe integer at offset 6
			let size = ((header[6] as u32 & 0x7F) << 21)
				| ((header[7] as u32 & 0x7F) << 14)
				| ((header[8] as u32 & 0x7F) << 7)
				| (header[9] as u32 & 0x7F);
			let mut skip = vec![0u8; size as usize];
			reader.read_exact(&mut skip)?;
		} else {
			// Not an ID3 tag, this is audio data
			preamble.extend_from_slice(&header);
		}

		// Probe the first chunk to detect sample rate
		let mut probe = vec![0u8; Self::CHUNK_SIZE];
		let probe_read = reader.read(&mut probe)?;
		probe.truncate(probe_read);

		let mut all_data = preamble;
		all_data.extend_from_slice(&probe);

		let sample_rate = detect_sample_rate(&all_data).unwrap_or(44100);

		let time = Time::new(1, sample_rate);
		let stream = Stream::new(0, 0, StreamKind::Audio, "mp3".into(), time);
		let streams = Streams::new(vec![stream]);

		Ok(Self {
			reader,
			streams,
			sample_rate,
			packet_count: 0,
			eof: false,
			stashed: all_data,
		})
	}

	pub fn sample_rate(&self) -> u32 {
		self.sample_rate
	}
}

impl<R: MediaRead> Demuxer for Mp3Demuxer<R> {
	fn streams(&self) -> &Streams {
		&self.streams
	}

	fn read_packet(&mut self) -> Result<Option<Packet>> {
		if self.eof {
			return Ok(None);
		}

		let data = if !self.stashed.is_empty() {
			std::mem::take(&mut self.stashed)
		} else {
			let mut buf = vec![0u8; Self::CHUNK_SIZE];
			let n = self.reader.read(&mut buf)?;
			if n == 0 {
				self.eof = true;
				return Ok(None);
			}
			buf.truncate(n);
			buf
		};

		let time = Time::new(1, self.sample_rate);
		let packet = Packet::new(data, 0, time).with_pts(self.packet_count as i64);
		self.packet_count += 1;

		Ok(Some(packet))
	}
}

/// Detect sample rate from the first MPEG audio sync word in the data.
fn detect_sample_rate(data: &[u8]) -> Option<u32> {
	for i in 0..data.len().saturating_sub(4) {
		if data[i] == 0xFF && (data[i + 1] & 0xE0) == 0xE0 {
			let version_bits = (data[i + 1] >> 3) & 0x03;
			let sr_index = (data[i + 2] >> 2) & 0x03;

			if sr_index == 3 {
				continue; // reserved
			}

			let rate = match version_bits {
				0x03 => [44100, 48000, 32000][sr_index as usize], // MPEG1
				0x02 => [22050, 24000, 16000][sr_index as usize], // MPEG2
				0x00 => [11025, 12000, 8000][sr_index as usize],  // MPEG2.5
				_ => continue,
			};

			return Some(rate);
		}
	}
	None
}
