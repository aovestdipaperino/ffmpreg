use crate::core::packet::Packet;
use crate::core::stream::{Stream, StreamKind, Streams};
use crate::core::time::Time;
use crate::core::traits::Demuxer;
use crate::io::{MediaRead, ReadPrimitives};
use crate::message::Result;

const TS_PACKET_SIZE: usize = 188;
const SYNC_BYTE: u8 = 0x47;
const PAT_PID: u16 = 0x0000;

struct PesState {
	/// Accumulated PES payload across TS packets.
	buffer: Vec<u8>,
	/// Parsed PTS from PES header (in 90kHz ticks).
	pts: Option<u64>,
	/// Stream id in our Streams list.
	stream_id: u32,
}

pub struct TsDemuxer<R: MediaRead> {
	reader: R,
	streams: Streams,
	/// PMT PID discovered from PAT.
	pmt_pid: Option<u16>,
	/// Streams discovered from PMT (pid -> stream info).
	pid_map: Vec<(u16, u32, StreamKind)>, // (pid, stream_id, kind)
	/// PES reassembly state per PID.
	pes_state: Vec<(u16, PesState)>,
	/// Queued output packets (from completed PES).
	output_queue: Vec<Packet>,
	/// Whether PMT has been parsed.
	pmt_parsed: bool,
	streams_built: bool,
}

impl<R: MediaRead> TsDemuxer<R> {
	pub fn new(reader: R) -> Result<Self> {
		let mut demuxer = Self {
			reader,
			streams: Streams::new_empty(),
			pmt_pid: None,
			pid_map: Vec::new(),
			pes_state: Vec::new(),
			output_queue: Vec::new(),
			pmt_parsed: false,
			streams_built: false,
		};

		// Read enough packets to discover PAT and PMT
		for _ in 0..100 {
			if demuxer.pmt_parsed {
				break;
			}
			demuxer.read_one_ts_packet()?;
		}

		demuxer.build_streams();
		Ok(demuxer)
	}

	fn build_streams(&mut self) {
		if self.streams_built {
			return;
		}
		let time = Time::new(1, 90000);
		for (i, &(_pid, stream_id, kind)) in self.pid_map.iter().enumerate() {
			let codec = match kind {
				StreamKind::Video => "h264".to_string(),
				StreamKind::Audio => "aac".to_string(),
				StreamKind::Subtitle => "subtitle".to_string(),
			};
			let stream = Stream::new(stream_id, i, kind, codec, time);
			self.streams.add(stream);
		}
		self.streams_built = true;
	}

	fn get_pes_state(&mut self, pid: u16) -> Option<&mut PesState> {
		self.pes_state.iter_mut().find(|(p, _)| *p == pid).map(|(_, s)| s)
	}

	fn ensure_pes_state(&mut self, pid: u16) {
		if !self.pes_state.iter().any(|(p, _)| *p == pid) {
			let stream_id = self.pid_map.iter().find(|(p, _, _)| *p == pid).map(|(_, id, _)| *id).unwrap_or(0);
			self.pes_state.push((pid, PesState {
				buffer: Vec::new(),
				pts: None,
				stream_id,
			}));
		}
	}

	fn read_one_ts_packet(&mut self) -> Result<bool> {
		let mut pkt = [0u8; TS_PACKET_SIZE];
		match self.reader.read_exact(&mut pkt) {
			Ok(()) => {}
			Err(_) => return Ok(false),
		}

		if pkt[0] != SYNC_BYTE {
			// Try to resync
			return Ok(true);
		}

		let payload_unit_start = (pkt[1] & 0x40) != 0;
		let pid = ((pkt[1] as u16 & 0x1F) << 8) | pkt[2] as u16;
		let adaptation_field_control = (pkt[3] >> 4) & 0x03;
		let _cc = pkt[3] & 0x0F;

		let mut pos = 4;

		// Adaptation field
		if adaptation_field_control & 0x02 != 0 {
			let adapt_len = pkt[pos] as usize;
			pos += 1 + adapt_len;
		}

		// No payload
		if adaptation_field_control & 0x01 == 0 || pos >= TS_PACKET_SIZE {
			return Ok(true);
		}

		let payload = &pkt[pos..];

		if pid == PAT_PID {
			self.parse_pat(payload, payload_unit_start);
		} else if Some(pid) == self.pmt_pid {
			self.parse_pmt(payload, payload_unit_start);
		} else if self.pid_map.iter().any(|(p, _, _)| *p == pid) {
			self.handle_pes_data(pid, payload, payload_unit_start)?;
		}

		Ok(true)
	}

	fn parse_pat(&mut self, payload: &[u8], pusi: bool) {
		let data = if pusi && !payload.is_empty() {
			let ptr = payload[0] as usize;
			if 1 + ptr < payload.len() {
				&payload[1 + ptr..]
			} else {
				return;
			}
		} else {
			payload
		};

		if data.len() < 8 {
			return;
		}
		// table_id should be 0x00
		if data[0] != 0x00 {
			return;
		}

		let section_length = ((data[1] as usize & 0x0F) << 8) | data[2] as usize;
		let end = std::cmp::min(3 + section_length, data.len());

		// Skip 5 bytes (transport_stream_id, version, section numbers)
		let mut i = 8;
		while i + 4 <= end - 4 {
			// -4 for CRC
			let program_num = ((data[i] as u16) << 8) | data[i + 1] as u16;
			let pid = ((data[i + 2] as u16 & 0x1F) << 8) | data[i + 3] as u16;
			if program_num != 0 {
				self.pmt_pid = Some(pid);
			}
			i += 4;
		}
	}

	fn parse_pmt(&mut self, payload: &[u8], pusi: bool) {
		let data = if pusi && !payload.is_empty() {
			let ptr = payload[0] as usize;
			if 1 + ptr < payload.len() {
				&payload[1 + ptr..]
			} else {
				return;
			}
		} else {
			payload
		};

		if data.len() < 12 || data[0] != 0x02 {
			return;
		}

		let section_length = ((data[1] as usize & 0x0F) << 8) | data[2] as usize;
		let end = std::cmp::min(3 + section_length, data.len());

		// program_info_length
		let prog_info_len = ((data[10] as usize & 0x0F) << 8) | data[11] as usize;
		let mut i = 12 + prog_info_len;

		let mut stream_idx = 0u32;
		while i + 5 <= end - 4 {
			let stream_type = data[i];
			let es_pid = ((data[i + 1] as u16 & 0x1F) << 8) | data[i + 2] as u16;
			let es_info_len = ((data[i + 3] as usize & 0x0F) << 8) | data[i + 4] as usize;
			i += 5 + es_info_len;

			let kind = match stream_type {
				0x01 | 0x02 | 0x1B | 0x24 | 0x10 => StreamKind::Video,
				0x03 | 0x04 | 0x0F | 0x11 => StreamKind::Audio,
				0x06 => StreamKind::Audio, // private data, often audio
				_ => continue,
			};

			if !self.pid_map.iter().any(|(p, _, _)| *p == es_pid) {
				self.pid_map.push((es_pid, stream_idx, kind));
				stream_idx += 1;
			}
		}

		self.pmt_parsed = true;
	}

	fn handle_pes_data(&mut self, pid: u16, payload: &[u8], pusi: bool) -> Result<()> {
		self.ensure_pes_state(pid);

		if pusi {
			// Flush the previous PES if we have data
			self.flush_pes(pid)?;

			// Parse PES header
			if payload.len() >= 9 && payload[0] == 0x00 && payload[1] == 0x00 && payload[2] == 0x01 {
				let _stream_id = payload[3];
				let _pes_length = ((payload[4] as u16) << 8) | payload[5] as u16;
				let pts_dts_flags = (payload[7] >> 6) & 0x03;
				let header_data_len = payload[8] as usize;
				let header_end = 9 + header_data_len;

				let pts = if pts_dts_flags >= 2 && payload.len() >= 14 {
					Some(parse_pts(&payload[9..14]))
				} else {
					None
				};

				let state = self.get_pes_state(pid).unwrap();
				state.pts = pts;
				state.buffer.clear();
				if header_end < payload.len() {
					state.buffer.extend_from_slice(&payload[header_end..]);
				}
			}
		} else {
			let state = self.get_pes_state(pid).unwrap();
			state.buffer.extend_from_slice(payload);
		}

		Ok(())
	}

	fn flush_pes(&mut self, pid: u16) -> Result<()> {
		let entry = self.pes_state.iter_mut().find(|(p, _)| *p == pid);
		if let Some((_, state)) = entry {
			if !state.buffer.is_empty() {
				let time = Time::new(1, 90000);
				let pts = state.pts.unwrap_or(0) as i64;
				let data = std::mem::take(&mut state.buffer);
				let packet = Packet::new(data, state.stream_id, time).with_pts(pts);
				self.output_queue.push(packet);
			}
		}
		Ok(())
	}

	fn flush_all_pes(&mut self) -> Result<()> {
		let pids: Vec<u16> = self.pes_state.iter().map(|(p, _)| *p).collect();
		for pid in pids {
			self.flush_pes(pid)?;
		}
		Ok(())
	}
}

impl<R: MediaRead> Demuxer for TsDemuxer<R> {
	fn streams(&self) -> &Streams {
		&self.streams
	}

	fn read_packet(&mut self) -> Result<Option<Packet>> {
		loop {
			if let Some(pkt) = self.output_queue.pop() {
				return Ok(Some(pkt));
			}

			if !self.read_one_ts_packet()? {
				// EOF - flush remaining PES data
				self.flush_all_pes()?;
				return Ok(self.output_queue.pop());
			}
		}
	}
}

/// Parse a 5-byte MPEG PTS field into a 33-bit timestamp.
fn parse_pts(data: &[u8]) -> u64 {
	let b0 = data[0] as u64;
	let b1 = data[1] as u64;
	let b2 = data[2] as u64;
	let b3 = data[3] as u64;
	let b4 = data[4] as u64;

	((b0 >> 1) & 0x07) << 30
		| (b1 << 22)
		| ((b2 >> 1) << 15)
		| (b3 << 7)
		| (b4 >> 1)
}
