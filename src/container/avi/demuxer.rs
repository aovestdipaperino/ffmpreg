use crate::core::packet::Packet;
use crate::core::stream::{Stream, StreamKind, Streams};
use crate::core::time::Time;
use crate::core::traits::Demuxer;
use crate::io::{MediaRead, ReadPrimitives};
use crate::{error, message::Result};

struct StreamInfo {
	id: u32,
	kind: StreamKind,
	codec: String,
	/// For video: fps. For audio: sample_rate.
	rate: u32,
	scale: u32,
	#[allow(dead_code)]
	block_align: u16,
}

pub struct AviDemuxer<R: MediaRead> {
	reader: R,
	streams: Streams,
	stream_info: Vec<StreamInfo>,
	data_remaining: u64,
	packet_count: u64,
}

impl<R: MediaRead> AviDemuxer<R> {
	pub fn new(mut reader: R) -> Result<Self> {
		Self::check_fourcc(&mut reader, "RIFF")?;
		let _file_size = reader.read_u32_le()?;
		Self::check_fourcc(&mut reader, "AVI ")?;

		let mut stream_info: Vec<StreamInfo> = Vec::new();
		let mut movi_size = 0u64;

		// Parse top-level chunks until we find movi
		loop {
			let chunk_id = match Self::read_fourcc(&mut reader) {
				Ok(id) => id,
				Err(_) => break,
			};
			let chunk_size = reader.read_u32_le()? as u64;

			match chunk_id.as_str() {
				"LIST" => {
					let list_type = Self::read_fourcc(&mut reader)?;
					match list_type.as_str() {
						"hdrl" => {
							Self::parse_hdrl(&mut reader, chunk_size - 4, &mut stream_info)?;
						}
						"movi" => {
							movi_size = chunk_size - 4;
							break;
						}
						_ => Self::skip_bytes(&mut reader, chunk_size - 4)?,
					}
				}
				_ => Self::skip_bytes(&mut reader, chunk_size)?,
			}
		}

		let mut streams = Streams::new_empty();
		for (i, info) in stream_info.iter().enumerate() {
			let time = if info.scale > 0 {
				Time::new(info.scale, info.rate)
			} else {
				Time::new(1, 1000)
			};
			let stream = Stream::new(info.id, i, info.kind, info.codec.clone(), time);
			streams.add(stream);
		}

		Ok(Self {
			reader,
			streams,
			stream_info,
			data_remaining: movi_size,
			packet_count: 0,
		})
	}

	fn parse_hdrl(reader: &mut R, size: u64, infos: &mut Vec<StreamInfo>) -> Result<()> {
		let mut remaining = size;

		while remaining >= 8 {
			let chunk_id = Self::read_fourcc(reader)?;
			let chunk_size = reader.read_u32_le()? as u64;
			remaining -= 8;

			match chunk_id.as_str() {
				"avih" => {
					Self::skip_bytes(reader, chunk_size)?;
					remaining -= chunk_size;
				}
				"LIST" => {
					let list_type = Self::read_fourcc(reader)?;
					let inner_size = chunk_size - 4;
					remaining -= chunk_size;
					if list_type == "strl" {
						Self::parse_strl(reader, inner_size, infos)?;
					} else {
						Self::skip_bytes(reader, inner_size)?;
					}
				}
				_ => {
					Self::skip_bytes(reader, chunk_size)?;
					remaining -= chunk_size;
				}
			}
		}
		if remaining > 0 {
			Self::skip_bytes(reader, remaining)?;
		}
		Ok(())
	}

	fn parse_strl(reader: &mut R, size: u64, infos: &mut Vec<StreamInfo>) -> Result<()> {
		let mut remaining = size;
		let mut kind = StreamKind::Video;
		let mut codec = String::new();
		let mut rate = 30u32;
		let mut scale = 1u32;
		let mut block_align = 0u16;

		while remaining >= 8 {
			let chunk_id = Self::read_fourcc(reader)?;
			let chunk_size = reader.read_u32_le()? as u64;
			remaining -= 8;

			match chunk_id.as_str() {
				"strh" => {
					if chunk_size >= 8 {
						let fcc_type = Self::read_fourcc(reader)?;
						let fcc_handler_bytes = Self::read_bytes(reader, 4)?;
						kind = match fcc_type.as_str() {
							"vids" => StreamKind::Video,
							"auds" => StreamKind::Audio,
							"txts" => StreamKind::Subtitle,
							_ => StreamKind::Video,
						};
						if kind == StreamKind::Video {
							codec = String::from_utf8_lossy(&fcc_handler_bytes)
								.trim_end_matches('\0')
								.to_lowercase();
						}
						// Skip to dwScale (offset 20) and dwRate (offset 24) from strh start
						if chunk_size >= 28 {
							Self::skip_bytes(reader, 12)?; // flags, priority, language, initialFrames
							scale = reader.read_u32_le()?;
							rate = reader.read_u32_le()?;
							Self::skip_bytes(reader, chunk_size - 28)?;
						} else {
							Self::skip_bytes(reader, chunk_size - 8)?;
						}
					} else {
						Self::skip_bytes(reader, chunk_size)?;
					}
					remaining -= chunk_size;
				}
				"strf" => {
					if kind == StreamKind::Audio && chunk_size >= 16 {
						let format_tag = reader.read_u16_le()?;
						let channels = reader.read_u16_le()?;
						let sample_rate = reader.read_u32_le()?;
						let _avg_bytes_per_sec = reader.read_u32_le()?;
						block_align = reader.read_u16_le()?;
						let bits_per_sample = reader.read_u16_le()?;
						let _ = (channels, bits_per_sample);
						rate = sample_rate;
						scale = block_align as u32;
						codec = match format_tag {
							1 => "pcm_s16le".into(),
							3 => "pcm_f32le".into(),
							0x0055 => "mp3".into(),
							0x00FF => "aac".into(),
							_ => format!("audio_0x{:04x}", format_tag),
						};
						Self::skip_bytes(reader, chunk_size - 16)?;
					} else if kind == StreamKind::Video && chunk_size >= 40 {
						// BITMAPINFOHEADER
						let _bi_size = reader.read_u32_le()?;
						let _width = reader.read_u32_le()?; // biWidth (as i32)
						let _height = reader.read_u32_le()?; // biHeight (as i32)
						let _planes = reader.read_u16_le()?;
						let _bit_count = reader.read_u16_le()?;
						let compression = Self::read_bytes(reader, 4)?;
						codec = String::from_utf8_lossy(&compression)
							.trim_end_matches('\0')
							.to_lowercase();
						Self::skip_bytes(reader, chunk_size - 20)?;
					} else {
						Self::skip_bytes(reader, chunk_size)?;
					}
					remaining -= chunk_size;
				}
				_ => {
					Self::skip_bytes(reader, chunk_size)?;
					remaining -= chunk_size;
				}
			}
		}
		if remaining > 0 {
			Self::skip_bytes(reader, remaining)?;
		}

		let id = infos.len() as u32;
		infos.push(StreamInfo { id, kind, codec, rate, scale, block_align });
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
			return Err(error!("expected '{}', found '{}'", expected, actual));
		}
		Ok(())
	}

	fn read_bytes(reader: &mut R, size: u64) -> Result<Vec<u8>> {
		let mut buf = vec![0u8; size as usize];
		reader.read_exact(&mut buf)?;
		Ok(buf)
	}

	fn skip_bytes(reader: &mut R, size: u64) -> Result<()> {
		let mut remaining = size;
		let mut buf = [0u8; 4096];
		while remaining > 0 {
			let to_read = std::cmp::min(remaining as usize, buf.len());
			reader.read_exact(&mut buf[..to_read])?;
			remaining -= to_read as u64;
		}
		Ok(())
	}

	/// Map a chunk ID like "00dc" or "01wb" to a stream_id.
	fn chunk_id_to_stream(&self, id: &[u8; 4]) -> Option<(u32, bool)> {
		if id[0] < b'0' || id[0] > b'9' || id[1] < b'0' || id[1] > b'9' {
			return None;
		}
		let idx = ((id[0] - b'0') as u32) * 10 + (id[1] - b'0') as u32;
		let is_video = id[2] == b'd' && (id[3] == b'c' || id[3] == b'b');
		if (idx as usize) < self.stream_info.len() {
			Some((idx, is_video))
		} else {
			None
		}
	}
}

impl<R: MediaRead> Demuxer for AviDemuxer<R> {
	fn streams(&self) -> &Streams {
		&self.streams
	}

	fn read_packet(&mut self) -> Result<Option<Packet>> {
		loop {
			if self.data_remaining < 8 {
				return Ok(None);
			}

			let mut id_buf = [0u8; 4];
			if self.reader.read_exact(&mut id_buf).is_err() {
				return Ok(None);
			}
			let chunk_size = self.reader.read_u32_le()? as u64;
			self.data_remaining -= 8;

			if let Some((stream_id, _is_video)) = self.chunk_id_to_stream(&id_buf) {
				let mut data = vec![0u8; chunk_size as usize];
				self.reader.read_exact(&mut data)?;
				self.data_remaining -= chunk_size;

				// Skip padding byte
				if chunk_size % 2 != 0 {
					let mut pad = [0u8; 1];
					let _ = self.reader.read_exact(&mut pad);
					if self.data_remaining > 0 {
						self.data_remaining -= 1;
					}
				}

				let info = &self.stream_info[stream_id as usize];
				let time = if info.scale > 0 {
					Time::new(info.scale, info.rate)
				} else {
					Time::new(1, 1000)
				};
				let packet = Packet::new(data, stream_id, time).with_pts(self.packet_count as i64);
				self.packet_count += 1;
				return Ok(Some(packet));
			}

			// Unknown chunk, skip
			Self::skip_bytes(&mut self.reader, chunk_size)?;
			self.data_remaining -= chunk_size;
			if chunk_size % 2 != 0 && self.data_remaining > 0 {
				Self::skip_bytes(&mut self.reader, 1)?;
				self.data_remaining -= 1;
			}
		}
	}
}
