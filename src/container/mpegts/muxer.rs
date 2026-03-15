use crate::core::Muxer;
use crate::core::packet::Packet;
use crate::core::stream::{Stream, StreamKind, Streams};
use crate::core::time::Time;
use crate::io::{MediaWrite, WritePrimitives};
use crate::message::Result;

const TS_PACKET_SIZE: usize = 188;
const SYNC_BYTE: u8 = 0x47;
const PAT_PID: u16 = 0x0000;
const PMT_PID: u16 = 0x1000;
const VIDEO_PID: u16 = 0x0100;
const AUDIO_PID: u16 = 0x0101;

/// MPEG-TS stream types.
const STREAM_TYPE_H264: u8 = 0x1B;
const STREAM_TYPE_H265: u8 = 0x24;
const STREAM_TYPE_MPEG2_VIDEO: u8 = 0x02;
const STREAM_TYPE_ADTS_AAC: u8 = 0x0F;
const STREAM_TYPE_MPEG2_AUDIO: u8 = 0x04;
const STREAM_TYPE_PRIVATE: u8 = 0x06; // for PCM or other private data

/// 90kHz clock used by MPEG-TS for PTS/DTS.
const TS_CLOCK: u64 = 90_000;

pub struct TsVideoTrack {
	pub stream_type: u8,
}

pub struct TsAudioTrack {
	pub stream_type: u8,
}

impl TsVideoTrack {
	pub fn h264() -> Self {
		Self { stream_type: STREAM_TYPE_H264 }
	}
	pub fn h265() -> Self {
		Self { stream_type: STREAM_TYPE_H265 }
	}
	pub fn mpeg2() -> Self {
		Self { stream_type: STREAM_TYPE_MPEG2_VIDEO }
	}
}

impl TsAudioTrack {
	pub fn aac() -> Self {
		Self { stream_type: STREAM_TYPE_ADTS_AAC }
	}
	pub fn mp2() -> Self {
		Self { stream_type: STREAM_TYPE_MPEG2_AUDIO }
	}
	pub fn pcm() -> Self {
		Self { stream_type: STREAM_TYPE_PRIVATE }
	}
}

pub struct TsMuxer<W: MediaWrite> {
	writer: W,
	streams: Streams,
	video: Option<TsVideoTrack>,
	audio: Option<TsAudioTrack>,
	pat_cc: u8,
	pmt_cc: u8,
	video_cc: u8,
	audio_cc: u8,
	has_video: bool,
	#[allow(dead_code)]
	has_audio: bool,
	packet_count: u64,
}

impl<W: MediaWrite> TsMuxer<W> {
	pub fn new(
		writer: W,
		video: Option<TsVideoTrack>,
		audio: Option<TsAudioTrack>,
	) -> Result<Self> {
		let mut streams = Streams::new_empty();
		let mut track_idx = 0usize;
		let has_video = video.is_some();
		let has_audio = audio.is_some();

		if has_video {
			let stream = Stream::new(
				VIDEO_PID as u32,
				track_idx,
				StreamKind::Video,
				"h264".into(),
				Time::new(1, 90000),
			);
			streams.add(stream);
			track_idx += 1;
		}
		if has_audio {
			let stream = Stream::new(
				AUDIO_PID as u32,
				track_idx,
				StreamKind::Audio,
				"aac".into(),
				Time::new(1, 90000),
			);
			streams.add(stream);
		}

		let mut muxer = Self {
			writer,
			streams,
			video,
			audio,
			pat_cc: 0,
			pmt_cc: 0,
			video_cc: 0,
			audio_cc: 0,
			has_video,
			has_audio,
			packet_count: 0,
		};

		// Write initial PAT + PMT
		muxer.write_pat()?;
		muxer.write_pmt()?;

		Ok(muxer)
	}

	/// Build and write one 188-byte TS packet.
	fn write_ts_packet(
		&mut self,
		pid: u16,
		cc: u8,
		payload_unit_start: bool,
		adaptation: Option<&[u8]>,
		payload: &[u8],
	) -> Result<()> {
		let mut pkt = [0xFFu8; TS_PACKET_SIZE];
		pkt[0] = SYNC_BYTE;

		let pusi_bit = if payload_unit_start { 0x40 } else { 0x00 };
		pkt[1] = pusi_bit | ((pid >> 8) as u8 & 0x1F);
		pkt[2] = pid as u8;

		let has_adapt = adaptation.is_some();
		let adapt_field = if has_adapt { 0x20 } else { 0x00 };
		let payload_field = if !payload.is_empty() { 0x10 } else { 0x00 };
		pkt[3] = adapt_field | payload_field | (cc & 0x0F);

		let mut pos = 4;

		if let Some(adapt) = adaptation {
			pkt[pos] = adapt.len() as u8; // adaptation_field_length
			pos += 1;
			pkt[pos..pos + adapt.len()].copy_from_slice(adapt);
			pos += adapt.len();
		}

		let payload_space = TS_PACKET_SIZE - pos;
		let copy_len = std::cmp::min(payload.len(), payload_space);
		pkt[pos..pos + copy_len].copy_from_slice(&payload[..copy_len]);

		self.writer.write_all(&pkt)?;
		self.packet_count += 1;
		Ok(())
	}

	fn write_pat(&mut self) -> Result<()> {
		// PAT payload: table_id=0x00, section
		let mut section = Vec::new();
		section.push(0x00); // table_id
		// section_syntax_indicator=1, 0, reserved=11
		section.extend_from_slice(&[0x00, 0x00]); // section_length (patched)
		section.extend_from_slice(&[0x00, 0x01]); // transport_stream_id
		section.push(0xC1); // reserved, version=0, current_next=1
		section.push(0x00); // section_number
		section.push(0x00); // last_section_number
		// Program 1 -> PMT PID
		section.extend_from_slice(&[0x00, 0x01]); // program_number = 1
		section.push(0xE0 | ((PMT_PID >> 8) as u8 & 0x1F));
		section.push(PMT_PID as u8);

		// Patch section length (from after section_length field to end, +4 for CRC)
		let section_data_len = section.len() - 3 + 4; // -3 for header before length, +4 for CRC
		section[1] = 0xB0 | ((section_data_len >> 8) as u8 & 0x0F);
		section[2] = section_data_len as u8;

		// CRC32
		let crc = crc32_mpeg2(&section);
		section.extend_from_slice(&crc.to_be_bytes());

		// Prepend pointer_field=0x00 for PSI
		let mut payload = vec![0x00];
		payload.extend_from_slice(&section);

		self.write_ts_packet(PAT_PID, self.pat_cc, true, None, &payload)?;
		self.pat_cc = (self.pat_cc + 1) & 0x0F;
		Ok(())
	}

	fn write_pmt(&mut self) -> Result<()> {
		let mut section = Vec::new();
		section.push(0x02); // table_id = PMT
		let section_len_pos = section.len();
		section.extend_from_slice(&[0x00, 0x00]); // section_length (patched)
		section.extend_from_slice(&[0x00, 0x01]); // program_number = 1
		section.push(0xC1); // reserved, version=0, current_next=1
		section.push(0x00); // section_number
		section.push(0x00); // last_section_number

		// PCR PID
		let pcr_pid = if self.has_video { VIDEO_PID } else { AUDIO_PID };
		section.push(0xE0 | ((pcr_pid >> 8) as u8 & 0x1F));
		section.push(pcr_pid as u8);

		// Program info length = 0
		section.extend_from_slice(&[0xF0, 0x00]);

		// Video stream entry
		if let Some(v) = &self.video {
			section.push(v.stream_type);
			section.push(0xE0 | ((VIDEO_PID >> 8) as u8 & 0x1F));
			section.push(VIDEO_PID as u8);
			section.extend_from_slice(&[0xF0, 0x00]); // ES info length = 0
		}

		// Audio stream entry
		if let Some(a) = &self.audio {
			section.push(a.stream_type);
			section.push(0xE0 | ((AUDIO_PID >> 8) as u8 & 0x1F));
			section.push(AUDIO_PID as u8);
			section.extend_from_slice(&[0xF0, 0x00]); // ES info length = 0
		}

		// Patch section length
		let section_data_len = section.len() - 3 + 4;
		section[section_len_pos] = 0xB0 | ((section_data_len >> 8) as u8 & 0x0F);
		section[section_len_pos + 1] = section_data_len as u8;

		let crc = crc32_mpeg2(&section);
		section.extend_from_slice(&crc.to_be_bytes());

		let mut payload = vec![0x00]; // pointer_field
		payload.extend_from_slice(&section);

		self.write_ts_packet(PMT_PID, self.pmt_cc, true, None, &payload)?;
		self.pmt_cc = (self.pmt_cc + 1) & 0x0F;
		Ok(())
	}

	fn write_pes(&mut self, pid: u16, cc: &mut u8, stream_id_byte: u8, pts_90k: u64, data: &[u8]) -> Result<()> {
		// Build PES header
		let mut pes = Vec::new();
		pes.extend_from_slice(&[0x00, 0x00, 0x01]); // start code
		pes.push(stream_id_byte);

		// PES packet length (0 = unbounded for video, or actual for audio)
		let pes_data_len = 3 + 5 + data.len(); // header_data_length fields + PTS + data
		if pes_data_len <= 0xFFFF {
			pes.extend_from_slice(&(pes_data_len as u16).to_be_bytes());
		} else {
			pes.extend_from_slice(&[0x00, 0x00]); // unbounded
		}

		// PES header flags
		pes.push(0x80); // marker bits, no scrambling, no priority, no alignment, no copyright
		pes.push(0x80); // PTS present, no DTS
		pes.push(0x05); // PES header data length (5 bytes for PTS)

		// PTS encoding (5 bytes)
		let pts = pts_90k;
		pes.push(0x21 | (((pts >> 29) & 0x0E) as u8)); // '0010' + PTS[32..30] + marker
		pes.push(((pts >> 22) & 0xFF) as u8);
		pes.push((((pts >> 14) & 0xFE) | 0x01) as u8); // PTS[22..15] + marker
		pes.push(((pts >> 7) & 0xFF) as u8);
		pes.push((((pts << 1) & 0xFE) | 0x01) as u8); // PTS[7..0] + marker

		pes.extend_from_slice(data);

		// Split PES into 188-byte TS packets
		let mut offset = 0;
		let mut first = true;
		while offset < pes.len() {
			let remaining = pes.len() - offset;
			let max_payload = TS_PACKET_SIZE - 4; // 184 bytes

			if remaining >= max_payload {
				// Full payload, no adaptation needed
				self.write_ts_packet(pid, *cc, first, None, &pes[offset..offset + max_payload])?;
				offset += max_payload;
			} else {
				// Last packet: need adaptation field stuffing to fill 188 bytes
				// Layout: 4 (header) + 1 (adapt_length) + adapt_data + payload = 188
				// So adapt_data length = 184 - 1 - remaining = 183 - remaining
				let adapt_data_len = 183 - remaining;
				if adapt_data_len == 0 {
					// Exactly 1 byte for adaptation_field_length (value 0), rest is payload
					self.write_ts_packet(pid, *cc, first, Some(&[]), &pes[offset..])?;
				} else {
					// First byte is flags (0x00), rest is 0xFF stuffing
					let mut adapt = vec![0xFFu8; adapt_data_len];
					adapt[0] = 0x00; // flags byte
					self.write_ts_packet(pid, *cc, first, Some(&adapt), &pes[offset..])?;
				}
				offset = pes.len();
			}
			*cc = (*cc + 1) & 0x0F;
			first = false;
		}

		Ok(())
	}
}

impl<W: MediaWrite> Muxer for TsMuxer<W> {
	fn streams(&self) -> &Streams {
		&self.streams
	}

	fn write(&mut self, packet: Packet) -> Result<()> {
		// Convert PTS from packet time base to 90kHz
		let pts_sec = packet.time.to_seconds(packet.pts);
		let pts_90k = (pts_sec * TS_CLOCK as f64) as u64;

		let stream = self.streams.get(packet.stream_id);
		let is_video = stream.map(|s| s.video_kind()).unwrap_or(false);

		if is_video {
			// Periodically re-emit PAT/PMT before keyframes
			if packet.keyframe {
				self.write_pat()?;
				self.write_pmt()?;
			}
			let mut cc = self.video_cc;
			self.write_pes(VIDEO_PID, &mut cc, 0xE0, pts_90k, &packet.data)?;
			self.video_cc = cc;
		} else {
			let mut cc = self.audio_cc;
			self.write_pes(AUDIO_PID, &mut cc, 0xC0, pts_90k, &packet.data)?;
			self.audio_cc = cc;
		}

		Ok(())
	}

	fn finalize(&mut self) -> Result<()> {
		self.writer.flush()
	}
}

/// CRC-32 for MPEG-2 sections (polynomial 0x04C11DB7, init 0xFFFFFFFF).
fn crc32_mpeg2(data: &[u8]) -> u32 {
	let mut crc: u32 = 0xFFFF_FFFF;
	for &byte in data {
		crc ^= (byte as u32) << 24;
		for _ in 0..8 {
			if crc & 0x8000_0000 != 0 {
				crc = (crc << 1) ^ 0x04C1_1DB7;
			} else {
				crc <<= 1;
			}
		}
	}
	crc
}
