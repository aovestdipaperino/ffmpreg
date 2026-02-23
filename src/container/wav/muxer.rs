use super::format::WavFormat;
use crate::core::Muxer;
use crate::core::packet::Packet;
use crate::core::track::{Format, Metadata};
use crate::io::{BinaryWrite, MediaSeek, MediaWrite, SeekFrom};
use crate::{error, message::Result};

pub struct WavMuxer<W: MediaWrite + MediaSeek> {
	writer: W,
	_format: WavFormat,
	metadata: Option<Metadata>,
	data_size: u32,
	data_size_pos: u64,
	file_size_pos: u64,
}

impl<W: MediaWrite + MediaSeek> WavMuxer<W> {
	pub fn new(mut writer: W, format: WavFormat) -> Result<Self> {
		let (file_size_pos, data_size_pos) = Self::write_header(&mut writer, &format)?;
		writer.flush()?;

		Ok(Self { writer, _format: format, metadata: None, data_size: 0, data_size_pos, file_size_pos })
	}

	pub fn from_format(writer: W, format: &Format) -> Result<Self> {
		let audio_format = match format {
			Format::Wav(audio) => audio,
			_ => return Err(error!("wav does not support non-audio tracks")),
		};
		let muxer = Self::new(writer, *audio_format)?;
		Ok(muxer)
	}

	fn write_header(writer: &mut W, format: &WavFormat) -> Result<(u64, u64)> {
		writer.write_all(b"RIFF")?;
		let file_size_pos = writer.stream_position()?;
		writer.write_u32_le(0)?;
		writer.write_all(b"WAVE")?;
		writer.write_all(b"fmt ")?;

		let fmt_size = match format.format_code {
			3 => 18,
			0x11 => 20,
			_ => 16,
		};
		writer.write_u32_le(fmt_size)?;
		writer.write_u16_le(format.format_code)?;
		writer.write_u16_le(format.channels.count() as u16)?;
		writer.write_u32_le(format.sample_rate.value())?;
		writer.write_u32_le(format.byte_rate())?;
		writer.write_u16_le(format.block_align())?;
		writer.write_u16_le(format.bit_depth.bits() as u16)?;

		if format.format_code == 3 {
			writer.write_u16_le(0)?;
		} else if format.format_code == 0x11 {
			writer.write_u16_le(4)?;
			let spb = ((512 - 4 * format.channels.count() as usize) * 2 + 1) as u16;
			writer.write_u16_le(spb)?;
		}

		writer.write_all(b"data")?;
		let data_size_pos = writer.stream_position()?;
		writer.write_u32_le(0)?;
		Ok((file_size_pos, data_size_pos))
	}

	pub fn write_packet(&mut self, packet: Packet) -> Result<()> {
		self.writer.write_all(&packet.data)?;
		self.data_size += packet.data.len() as u32;
		Ok(())
	}

	pub fn finalize_muxer(&mut self) -> Result<()> {
		self.writer.seek(SeekFrom::Start(self.data_size_pos))?;
		self.writer.write_u32_le(self.data_size)?;

		let mut file_size = self.data_size + 36;

		if let Some(meta) = &self.metadata {
			if !meta.is_empty() {
				file_size += Self::calc_list_size(meta) as u32;
				self.writer.seek(SeekFrom::End(0))?;
				Self::write_list_chunk(&mut self.writer, meta)?;
			}
		}

		self.writer.seek(SeekFrom::Start(self.file_size_pos))?;
		self.writer.write_u32_le(file_size)?;
		self.writer.flush()?;
		Ok(())
	}

	fn calc_list_size(metadata: &Metadata) -> u64 {
		let fields = metadata.export_fields();
		fields.values().fold(8, |acc, v| {
			let byte_len = v.as_bytes().len();
			let mut size = acc + 8 + byte_len as u64 + 1;
			if (byte_len + 1) % 2 == 1 {
				size += 1;
			}
			size
		})
	}

	fn write_list_chunk(writer: &mut W, metadata: &Metadata) -> Result<()> {
		if metadata.is_empty() {
			return Ok(());
		}

		let list_size = Self::calc_list_size(metadata) - 8;
		writer.write_all(b"LIST")?;
		writer.write_u32_le(list_size as u32)?;
		writer.write_all(b"INFO")?;

		let fields = metadata.export_fields();
		for (field, value) in fields {
			let id: &[u8; 4] = match field.as_str() {
				"artist" => b"IART",
				"title" => b"INAM",
				"comment" => b"ICOM",
				"copyright" => b"ICOP",
				"software" => b"ISFT",
				"genre" => b"IGNR",
				"track" => b"ITRK",
				_ => continue,
			};
			Self::write_wav_tag(writer, id, &value)?;
		}
		Ok(())
	}

	fn write_wav_tag(writer: &mut W, id: &[u8; 4], value: &str) -> Result<()> {
		let value_len = value.len() + 1;
		writer.write_all(id)?;
		writer.write_u32_le(value_len as u32)?;
		writer.write_all(value.as_bytes())?;
		writer.write_u8(0)?;
		if value_len % 2 == 1 {
			writer.write_u8(0)?;
		}
		Ok(())
	}
}

impl<W: MediaWrite + MediaSeek> Muxer for WavMuxer<W> {
	fn write(&mut self, packet: Packet) -> Result<()> {
		self.write_packet(packet)
	}

	fn finalize(&mut self) -> Result<()> {
		self.finalize_muxer()
	}

	fn set_metadata(&mut self, metadata: Option<Metadata>) {
		self.metadata = metadata;
	}
}
