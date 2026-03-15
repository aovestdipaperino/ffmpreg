use crate::core::Muxer;
use crate::core::packet::Packet;
use crate::core::stream::{self, Stream, StreamKind, Streams};
use crate::io::{MediaSeek, MediaWrite, SeekFrom, WritePrimitives};
use crate::message::Result;

use super::ebml;

// EBML Header element IDs
const EBML: u32 = 0x1A45_DFA3;
const EBML_VERSION: u32 = 0x4286;
const EBML_READ_VERSION: u32 = 0x42F7;
const EBML_MAX_ID_LENGTH: u32 = 0x42F2;
const EBML_MAX_SIZE_LENGTH: u32 = 0x42F3;
const DOC_TYPE: u32 = 0x4282;
const DOC_TYPE_VERSION: u32 = 0x4287;
const DOC_TYPE_READ_VERSION: u32 = 0x4285;

// Segment
const SEGMENT: u32 = 0x1853_8067;

// Info
const INFO: u32 = 0x1549_A966;
const TIMECODE_SCALE: u32 = 0x2AD7B1;
const MUXING_APP: u32 = 0x4D80;
const WRITING_APP: u32 = 0x5741;
const DURATION: u32 = 0x4489;

// Tracks
const TRACKS: u32 = 0x1654_AE6B;
const TRACK_ENTRY: u32 = 0xAE;
const TRACK_NUMBER: u32 = 0xD7;
const TRACK_UID: u32 = 0x73C5;
const TRACK_TYPE: u32 = 0x83;
const CODEC_ID: u32 = 0x86;
const CODEC_PRIVATE: u32 = 0x63A2;

// Video
const VIDEO: u32 = 0xE0;
const PIXEL_WIDTH: u32 = 0xB0;
const PIXEL_HEIGHT: u32 = 0xBA;

// Audio
const AUDIO: u32 = 0xE1;
const SAMPLING_FREQUENCY: u32 = 0xB5;
const CHANNELS: u32 = 0x9F;
const BIT_DEPTH: u32 = 0x6264;

// Cluster
const CLUSTER: u32 = 0x1F43_B675;
const TIMECODE: u32 = 0xE7;
const SIMPLE_BLOCK: u32 = 0xA3;

/// Timecode scale in nanoseconds (1ms = 1_000_000ns).
const DEFAULT_TIMECODE_SCALE: u64 = 1_000_000;

pub struct VideoTrackInfo {
	pub width: u32,
	pub height: u32,
	pub codec_private: Vec<u8>,
}

pub struct AudioTrackInfo {
	pub sample_rate: u32,
	pub channels: u16,
	pub bit_depth: u16,
}

pub struct MkvMuxer<W: MediaWrite + MediaSeek> {
	writer: W,
	streams: Streams,
	/// Current cluster timecode in ms.
	cluster_timecode: Option<i64>,
	/// Maximum cluster duration before starting a new one (ms).
	cluster_max_duration: i64,
	/// Duration position in the file for patching.
	duration_pos: u64,
	/// Track of the maximum timecode seen (ms) for final duration.
	max_timecode_ms: i64,
}

impl<W: MediaWrite + MediaSeek> MkvMuxer<W> {
	pub fn new(
		mut writer: W,
		video: Option<VideoTrackInfo>,
		audio: Option<AudioTrackInfo>,
	) -> Result<Self> {
		let mut streams = Streams::new_empty();
		let mut track_idx = 0usize;

		if let Some(v) = &video {
			let mut stream =
				Stream::new(1, track_idx, StreamKind::Video, "V_MS/VFW/FOURCC".into(), crate::core::time::Time::new(1, 1000));
			if !v.codec_private.is_empty() {
				stream = stream.with_codec_private(v.codec_private.clone());
			}
			streams.add(stream);
			track_idx += 1;
		}
		if audio.is_some() {
			let stream =
				Stream::new(2, track_idx, StreamKind::Audio, "A_PCM/INT/LIT".into(), crate::core::time::Time::new(1, 1000));
			streams.add(stream);
		}

		Self::write_ebml_header(&mut writer)?;
		// Segment with unknown size (streaming-style)
		ebml::write_id(&mut writer, SEGMENT)?;
		ebml::write_unknown_size(&mut writer)?;

		let duration_pos = Self::write_info(&mut writer)?;
		Self::write_tracks(&mut writer, video.as_ref(), audio.as_ref())?;

		writer.flush()?;

		Ok(Self {
			writer,
			streams,
			cluster_timecode: None,
			cluster_max_duration: 5000,
			duration_pos,
			max_timecode_ms: 0,
		})
	}

	fn write_ebml_header(w: &mut W) -> Result<()> {
		// Build the EBML header body in memory first to know its size
		let mut body: Vec<u8> = Vec::new();
		ebml::write_uint(&mut body, EBML_VERSION, 1)?;
		ebml::write_uint(&mut body, EBML_READ_VERSION, 1)?;
		ebml::write_uint(&mut body, EBML_MAX_ID_LENGTH, 4)?;
		ebml::write_uint(&mut body, EBML_MAX_SIZE_LENGTH, 8)?;
		ebml::write_string(&mut body, DOC_TYPE, "matroska")?;
		ebml::write_uint(&mut body, DOC_TYPE_VERSION, 4)?;
		ebml::write_uint(&mut body, DOC_TYPE_READ_VERSION, 2)?;

		ebml::write_id(w, EBML)?;
		ebml::write_size(w, body.len() as u64)?;
		w.write_all(&body)
	}

	fn write_info(w: &mut W) -> Result<u64> {
		let mut body: Vec<u8> = Vec::new();
		ebml::write_uint(&mut body, TIMECODE_SCALE, DEFAULT_TIMECODE_SCALE)?;
		ebml::write_utf8(&mut body, MUXING_APP, "ffmpreg")?;
		ebml::write_utf8(&mut body, WRITING_APP, "ffmpreg")?;

		// Reserve space for duration (f64 = 8 bytes element + id + size)
		// We'll record the position of the float payload to patch later
		ebml::write_id(&mut body, DURATION)?;
		ebml::write_size(&mut body, 8)?;
		let duration_offset_in_body = body.len();
		body.extend_from_slice(&0.0f64.to_be_bytes());

		ebml::write_id(w, INFO)?;
		ebml::write_size(w, body.len() as u64)?;
		let info_start = w.stream_position()?;
		w.write_all(&body)?;

		Ok(info_start + duration_offset_in_body as u64)
	}

	fn write_tracks(
		w: &mut W,
		video: Option<&VideoTrackInfo>,
		audio: Option<&AudioTrackInfo>,
	) -> Result<()> {
		let mut body: Vec<u8> = Vec::new();

		if let Some(v) = video {
			let mut entry: Vec<u8> = Vec::new();
			ebml::write_uint(&mut entry, TRACK_NUMBER, 1)?;
			ebml::write_uint(&mut entry, TRACK_UID, 1)?;
			ebml::write_uint(&mut entry, TRACK_TYPE, 1)?; // video
			ebml::write_string(&mut entry, CODEC_ID, "V_MPEG4/ISO/AVC")?;
			if !v.codec_private.is_empty() {
				ebml::write_binary(&mut entry, CODEC_PRIVATE, &v.codec_private)?;
			}

			// Video sub-element
			let mut video_body: Vec<u8> = Vec::new();
			ebml::write_uint(&mut video_body, PIXEL_WIDTH, v.width as u64)?;
			ebml::write_uint(&mut video_body, PIXEL_HEIGHT, v.height as u64)?;
			ebml::write_id(&mut entry, VIDEO)?;
			ebml::write_size(&mut entry, video_body.len() as u64)?;
			entry.extend_from_slice(&video_body);

			ebml::write_id(&mut body, TRACK_ENTRY)?;
			ebml::write_size(&mut body, entry.len() as u64)?;
			body.extend_from_slice(&entry);
		}

		if let Some(a) = audio {
			let mut entry: Vec<u8> = Vec::new();
			ebml::write_uint(&mut entry, TRACK_NUMBER, 2)?;
			ebml::write_uint(&mut entry, TRACK_UID, 2)?;
			ebml::write_uint(&mut entry, TRACK_TYPE, 2)?; // audio
			ebml::write_string(&mut entry, CODEC_ID, "A_PCM/INT/LIT")?;

			// Audio sub-element
			let mut audio_body: Vec<u8> = Vec::new();
			ebml::write_float(&mut audio_body, SAMPLING_FREQUENCY, a.sample_rate as f64)?;
			ebml::write_uint(&mut audio_body, CHANNELS, a.channels as u64)?;
			ebml::write_uint(&mut audio_body, BIT_DEPTH, a.bit_depth as u64)?;
			ebml::write_id(&mut entry, AUDIO)?;
			ebml::write_size(&mut entry, audio_body.len() as u64)?;
			entry.extend_from_slice(&audio_body);

			ebml::write_id(&mut body, TRACK_ENTRY)?;
			ebml::write_size(&mut body, entry.len() as u64)?;
			body.extend_from_slice(&entry);
		}

		ebml::write_id(w, TRACKS)?;
		ebml::write_size(w, body.len() as u64)?;
		w.write_all(&body)
	}

	fn start_cluster(&mut self, timecode_ms: i64) -> Result<()> {
		ebml::write_id(&mut self.writer, CLUSTER)?;
		ebml::write_unknown_size(&mut self.writer)?;
		ebml::write_uint(&mut self.writer, TIMECODE, timecode_ms as u64)?;
		self.cluster_timecode = Some(timecode_ms);
		Ok(())
	}

	/// Write a SimpleBlock for the given track.
	fn write_simple_block(
		&mut self,
		track_number: u8,
		timecode_rel: i16,
		keyframe: bool,
		data: &[u8],
	) -> Result<()> {
		// SimpleBlock: track_number (VINT) + timecode (i16 BE) + flags (u8) + data
		let block_size = 1 + 2 + 1 + data.len(); // track num as 1-byte VINT
		ebml::write_id(&mut self.writer, SIMPLE_BLOCK)?;
		ebml::write_size(&mut self.writer, block_size as u64)?;

		// Track number as VINT (1-byte for tracks 1-127)
		self.writer.write_u8(0x80 | track_number)?;
		self.writer.write_i16_be(timecode_rel)?;
		let flags: u8 = if keyframe { 0x80 } else { 0x00 };
		self.writer.write_u8(flags)?;
		self.writer.write_all(data)?;

		Ok(())
	}

	pub fn write_packet(&mut self, packet: Packet) -> Result<()> {
		// Determine track number from stream_id
		let track_number = packet.stream_id as u8;
		if track_number == 0 {
			return Err(crate::error!("MKV: track number must be >= 1"));
		}

		// Convert pts to milliseconds using the packet's time base
		let timecode_ms = packet.time.to_seconds(packet.pts) * 1000.0;
		let timecode_ms = timecode_ms as i64;

		if timecode_ms > self.max_timecode_ms {
			self.max_timecode_ms = timecode_ms;
		}

		// Start a new cluster if needed
		let need_new_cluster = match self.cluster_timecode {
			None => true,
			Some(ct) => (timecode_ms - ct) >= self.cluster_max_duration,
		};
		if need_new_cluster {
			self.start_cluster(timecode_ms)?;
		}

		let cluster_tc = self.cluster_timecode.unwrap();
		let rel = (timecode_ms - cluster_tc) as i16;

		self.write_simple_block(track_number, rel, packet.keyframe, &packet.data)
	}
}

impl<W: MediaWrite + MediaSeek> Muxer for MkvMuxer<W> {
	fn streams(&self) -> &stream::Streams {
		&self.streams
	}

	fn write(&mut self, packet: Packet) -> Result<()> {
		self.write_packet(packet)
	}

	fn finalize(&mut self) -> Result<()> {
		// Patch duration
		let duration_ms = self.max_timecode_ms as f64;
		self.writer.seek(SeekFrom::Start(self.duration_pos))?;
		self.writer.write_f64_be(duration_ms)?;
		self.writer.flush()
	}
}
