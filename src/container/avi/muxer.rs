use crate::core::Muxer;
use crate::core::packet::Packet;
use crate::core::stream::{Stream, StreamKind, Streams};
use crate::core::time::Time;
use crate::io::{MediaSeek, MediaWrite, SeekFrom, WritePrimitives};
use crate::message::Result;

pub struct AviVideoTrack {
	pub width: u32,
	pub height: u32,
	pub fps: u32,
	/// FourCC codec tag (e.g. b"H264", b"MJPG").
	pub fourcc: [u8; 4],
}

pub struct AviAudioTrack {
	pub sample_rate: u32,
	pub channels: u16,
	pub bit_depth: u16,
	/// wFormatTag: 1 = PCM, 0x0055 = MP3, 0x00FF = AAC.
	pub format_tag: u16,
}

struct IndexEntry {
	chunk_id: [u8; 4],
	flags: u32,
	offset: u32,
	size: u32,
}

pub struct AviMuxer<W: MediaWrite + MediaSeek> {
	writer: W,
	streams: Streams,
	#[allow(dead_code)]
	video: Option<AviVideoTrack>,
	#[allow(dead_code)]
	audio: Option<AviAudioTrack>,
	index: Vec<IndexEntry>,
	movi_start: u64,
	video_frames: u32,
	total_data_size: u32,
	/// Position of the RIFF file size field for patching.
	riff_size_pos: u64,
	/// Position of the movi list size field for patching.
	movi_size_pos: u64,
	/// Position of dwTotalFrames in avih for patching.
	total_frames_pos: u64,
}

impl<W: MediaWrite + MediaSeek> AviMuxer<W> {
	pub fn new(
		mut writer: W,
		video: Option<AviVideoTrack>,
		audio: Option<AviAudioTrack>,
	) -> Result<Self> {
		let mut streams = Streams::new_empty();
		let mut track_idx = 0usize;

		if video.is_some() {
			let stream = Stream::new(0, track_idx, StreamKind::Video, "h264".into(), Time::new(1, 1000));
			streams.add(stream);
			track_idx += 1;
		}
		if audio.is_some() {
			let stream =
				Stream::new(1, track_idx, StreamKind::Audio, "pcm_s16le".into(), Time::new(1, 1000));
			streams.add(stream);
		}

		// RIFF header
		writer.write_all(b"RIFF")?;
		let riff_size_pos = writer.stream_position()?;
		writer.write_u32_le(0)?; // patched in finalize
		writer.write_all(b"AVI ")?;

		let total_frames_pos = Self::write_headers(&mut writer, &video, &audio)?;

		// movi list
		writer.write_all(b"LIST")?;
		let movi_size_pos = writer.stream_position()?;
		writer.write_u32_le(0)?; // patched in finalize
		writer.write_all(b"movi")?;
		let movi_start = writer.stream_position()?;

		Ok(Self {
			writer,
			streams,
			video,
			audio,
			index: Vec::new(),
			movi_start,
			video_frames: 0,
			total_data_size: 0,
			riff_size_pos,
			movi_size_pos,
			total_frames_pos,
		})
	}

	fn write_headers(
		w: &mut W,
		video: &Option<AviVideoTrack>,
		audio: &Option<AviAudioTrack>,
	) -> Result<u64> {
		// Build hdrl in memory to know size
		let mut hdrl: Vec<u8> = Vec::new();

		let num_streams =
			if video.is_some() { 1u32 } else { 0 } + if audio.is_some() { 1u32 } else { 0 };
		let fps = video.as_ref().map(|v| v.fps).unwrap_or(30);
		let width = video.as_ref().map(|v| v.width).unwrap_or(0);
		let height = video.as_ref().map(|v| v.height).unwrap_or(0);

		// avih (main AVI header) - 56 bytes
		let mut avih = Vec::new();
		avih.extend_from_slice(&(1_000_000u32 / fps).to_le_bytes()); // dwMicroSecPerFrame
		avih.extend_from_slice(&0u32.to_le_bytes()); // dwMaxBytesPerSec
		avih.extend_from_slice(&0u32.to_le_bytes()); // dwPaddingGranularity
		avih.extend_from_slice(&0x10u32.to_le_bytes()); // dwFlags (AVIF_HASINDEX)
		// dwTotalFrames - we'll record its offset to patch later
		let total_frames_offset_in_hdrl = 8 + 8 + avih.len(); // "avih" + size + current pos
		avih.extend_from_slice(&0u32.to_le_bytes()); // patched in finalize
		avih.extend_from_slice(&0u32.to_le_bytes()); // dwInitialFrames
		avih.extend_from_slice(&num_streams.to_le_bytes()); // dwStreams
		avih.extend_from_slice(&0u32.to_le_bytes()); // dwSuggestedBufferSize
		avih.extend_from_slice(&width.to_le_bytes()); // dwWidth
		avih.extend_from_slice(&height.to_le_bytes()); // dwHeight
		avih.extend_from_slice(&[0u8; 16]); // dwReserved[4]

		hdrl.extend_from_slice(b"avih");
		hdrl.extend_from_slice(&(avih.len() as u32).to_le_bytes());
		hdrl.extend_from_slice(&avih);

		// Video stream list (strl)
		if let Some(v) = video {
			let mut strl: Vec<u8> = Vec::new();

			// strh (stream header)
			let mut strh = Vec::new();
			strh.extend_from_slice(b"vids"); // fccType
			strh.extend_from_slice(&v.fourcc); // fccHandler
			strh.extend_from_slice(&0u32.to_le_bytes()); // dwFlags
			strh.extend_from_slice(&0u16.to_le_bytes()); // wPriority
			strh.extend_from_slice(&0u16.to_le_bytes()); // wLanguage
			strh.extend_from_slice(&0u32.to_le_bytes()); // dwInitialFrames
			strh.extend_from_slice(&1u32.to_le_bytes()); // dwScale
			strh.extend_from_slice(&v.fps.to_le_bytes()); // dwRate
			strh.extend_from_slice(&0u32.to_le_bytes()); // dwStart
			strh.extend_from_slice(&0u32.to_le_bytes()); // dwLength (patched? or leave 0)
			strh.extend_from_slice(&0u32.to_le_bytes()); // dwSuggestedBufferSize
			strh.extend_from_slice(&0xFFFFFFFFu32.to_le_bytes()); // dwQuality
			strh.extend_from_slice(&0u32.to_le_bytes()); // dwSampleSize
			strh.extend_from_slice(&0u16.to_le_bytes()); // rcFrame left
			strh.extend_from_slice(&0u16.to_le_bytes()); // rcFrame top
			strh.extend_from_slice(&(v.width as u16).to_le_bytes()); // rcFrame right
			strh.extend_from_slice(&(v.height as u16).to_le_bytes()); // rcFrame bottom

			strl.extend_from_slice(b"strh");
			strl.extend_from_slice(&(strh.len() as u32).to_le_bytes());
			strl.extend_from_slice(&strh);

			// strf (stream format) - BITMAPINFOHEADER (40 bytes)
			let mut strf = Vec::new();
			strf.extend_from_slice(&40u32.to_le_bytes()); // biSize
			strf.extend_from_slice(&(v.width as i32).to_le_bytes()); // biWidth
			strf.extend_from_slice(&(v.height as i32).to_le_bytes()); // biHeight
			strf.extend_from_slice(&1u16.to_le_bytes()); // biPlanes
			strf.extend_from_slice(&24u16.to_le_bytes()); // biBitCount
			strf.extend_from_slice(&v.fourcc); // biCompression (FourCC)
			strf.extend_from_slice(&(v.width * v.height * 3).to_le_bytes()); // biSizeImage
			strf.extend_from_slice(&0u32.to_le_bytes()); // biXPelsPerMeter
			strf.extend_from_slice(&0u32.to_le_bytes()); // biYPelsPerMeter
			strf.extend_from_slice(&0u32.to_le_bytes()); // biClrUsed
			strf.extend_from_slice(&0u32.to_le_bytes()); // biClrImportant

			strl.extend_from_slice(b"strf");
			strl.extend_from_slice(&(strf.len() as u32).to_le_bytes());
			strl.extend_from_slice(&strf);

			hdrl.extend_from_slice(b"LIST");
			hdrl.extend_from_slice(&((strl.len() + 4) as u32).to_le_bytes());
			hdrl.extend_from_slice(b"strl");
			hdrl.extend_from_slice(&strl);
		}

		// Audio stream list (strl)
		if let Some(a) = audio {
			let mut strl: Vec<u8> = Vec::new();

			// strh
			let mut strh = Vec::new();
			strh.extend_from_slice(b"auds"); // fccType
			strh.extend_from_slice(&a.format_tag.to_le_bytes()); // fccHandler (low 2 bytes)
			strh.extend_from_slice(&0u16.to_le_bytes()); // fccHandler (high 2 bytes)
			strh.extend_from_slice(&0u32.to_le_bytes()); // dwFlags
			strh.extend_from_slice(&0u16.to_le_bytes()); // wPriority
			strh.extend_from_slice(&0u16.to_le_bytes()); // wLanguage
			strh.extend_from_slice(&0u32.to_le_bytes()); // dwInitialFrames
			let block_align = a.channels * (a.bit_depth / 8);
			strh.extend_from_slice(&(block_align as u32).to_le_bytes()); // dwScale
			let byte_rate = a.sample_rate * block_align as u32;
			strh.extend_from_slice(&byte_rate.to_le_bytes()); // dwRate
			strh.extend_from_slice(&0u32.to_le_bytes()); // dwStart
			strh.extend_from_slice(&0u32.to_le_bytes()); // dwLength
			strh.extend_from_slice(&0u32.to_le_bytes()); // dwSuggestedBufferSize
			strh.extend_from_slice(&0xFFFFFFFFu32.to_le_bytes()); // dwQuality
			strh.extend_from_slice(&(block_align as u32).to_le_bytes()); // dwSampleSize
			strh.extend_from_slice(&0u16.to_le_bytes()); // rcFrame left
			strh.extend_from_slice(&0u16.to_le_bytes()); // rcFrame top
			strh.extend_from_slice(&0u16.to_le_bytes()); // rcFrame right
			strh.extend_from_slice(&0u16.to_le_bytes()); // rcFrame bottom

			strl.extend_from_slice(b"strh");
			strl.extend_from_slice(&(strh.len() as u32).to_le_bytes());
			strl.extend_from_slice(&strh);

			// strf (WAVEFORMATEX - 18 bytes)
			let mut strf = Vec::new();
			strf.extend_from_slice(&a.format_tag.to_le_bytes()); // wFormatTag
			strf.extend_from_slice(&a.channels.to_le_bytes()); // nChannels
			strf.extend_from_slice(&a.sample_rate.to_le_bytes()); // nSamplesPerSec
			strf.extend_from_slice(&byte_rate.to_le_bytes()); // nAvgBytesPerSec
			strf.extend_from_slice(&block_align.to_le_bytes()); // nBlockAlign
			strf.extend_from_slice(&a.bit_depth.to_le_bytes()); // wBitsPerSample
			strf.extend_from_slice(&0u16.to_le_bytes()); // cbSize

			strl.extend_from_slice(b"strf");
			strl.extend_from_slice(&(strf.len() as u32).to_le_bytes());
			strl.extend_from_slice(&strf);

			hdrl.extend_from_slice(b"LIST");
			hdrl.extend_from_slice(&((strl.len() + 4) as u32).to_le_bytes());
			hdrl.extend_from_slice(b"strl");
			hdrl.extend_from_slice(&strl);
		}

		// Write hdrl LIST
		w.write_all(b"LIST")?;
		w.write_u32_le((hdrl.len() + 4) as u32)?; // +4 for "hdrl"
		let hdrl_start = w.stream_position()?;
		w.write_all(b"hdrl")?;
		w.write_all(&hdrl)?;

		// The total_frames field position in the file
		let total_frames_pos = hdrl_start + 4 + total_frames_offset_in_hdrl as u64;

		Ok(total_frames_pos)
	}

	fn chunk_id_for_stream(&self, stream_id: u32) -> [u8; 4] {
		let stream = self.streams.get(stream_id);
		let is_video = stream.map(|s| s.video_kind()).unwrap_or(false);
		let idx = stream.map(|s| s.index).unwrap_or(0);
		let d0 = b'0' + (idx / 10) as u8;
		let d1 = b'0' + (idx % 10) as u8;
		if is_video {
			[d0, d1, b'd', b'c'] // compressed video
		} else {
			[d0, d1, b'w', b'b'] // audio
		}
	}
}

impl<W: MediaWrite + MediaSeek> Muxer for AviMuxer<W> {
	fn streams(&self) -> &Streams {
		&self.streams
	}

	fn write(&mut self, packet: Packet) -> Result<()> {
		let chunk_id = self.chunk_id_for_stream(packet.stream_id);
		let data_len = packet.data.len() as u32;
		let offset = (self.writer.stream_position()? - self.movi_start) as u32;

		self.writer.write_all(&chunk_id)?;
		self.writer.write_u32_le(data_len)?;
		self.writer.write_all(&packet.data)?;

		// AVI chunks must be word-aligned
		if data_len % 2 != 0 {
			self.writer.write_u8(0)?;
		}

		let flags = if packet.keyframe { 0x10 } else { 0x00 }; // AVIIF_KEYFRAME
		self.index.push(IndexEntry { chunk_id, flags, offset, size: data_len });
		self.total_data_size += 8 + data_len + (data_len % 2);

		let stream = self.streams.get(packet.stream_id);
		if stream.map(|s| s.video_kind()).unwrap_or(false) {
			self.video_frames += 1;
		}

		Ok(())
	}

	fn finalize(&mut self) -> Result<()> {
		// Write idx1 index
		let idx1_data_size = (self.index.len() * 16) as u32;
		self.writer.write_all(b"idx1")?;
		self.writer.write_u32_le(idx1_data_size)?;
		for entry in &self.index {
			self.writer.write_all(&entry.chunk_id)?;
			self.writer.write_u32_le(entry.flags)?;
			self.writer.write_u32_le(entry.offset)?;
			self.writer.write_u32_le(entry.size)?;
		}

		let file_end = self.writer.stream_position()?;

		// Patch movi LIST size
		let movi_data_size = (self.movi_start - self.movi_size_pos - 4) + self.total_data_size as u64;
		self.writer.seek(SeekFrom::Start(self.movi_size_pos))?;
		self.writer.write_u32_le(movi_data_size as u32)?;

		// Patch RIFF size
		let riff_size = (file_end - self.riff_size_pos - 4) as u32;
		self.writer.seek(SeekFrom::Start(self.riff_size_pos))?;
		self.writer.write_u32_le(riff_size)?;

		// Patch total frames
		self.writer.seek(SeekFrom::Start(self.total_frames_pos))?;
		self.writer.write_u32_le(self.video_frames)?;

		self.writer.flush()
	}
}
