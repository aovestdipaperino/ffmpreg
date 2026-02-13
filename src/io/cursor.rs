use crate::message::Result;
use std::io::{Read as StdRead, Seek as StdSeek, SeekFrom, Write as StdWrite};

pub struct Cursor<T> {
	buffer: T,
	offset: u64,
}

impl<T> Cursor<T> {
	#[inline]
	pub const fn new(buffer: T) -> Self {
		Self { buffer, offset: 0 }
	}

	#[inline]
	pub fn into_inner(self) -> T {
		self.buffer
	}

	#[inline]
	pub const fn get_ref(&self) -> &T {
		&self.buffer
	}

	#[inline]
	pub fn get_mut(&mut self) -> &mut T {
		&mut self.buffer
	}

	#[inline]
	pub const fn offset(&self) -> u64 {
		self.offset
	}

	#[inline]
	pub fn set_offset(&mut self, new_offset: u64) {
		self.offset = new_offset;
	}
}
impl<T: AsRef<[u8]>> crate::io::MediaRead for Cursor<T> {
	fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
		let slice = self.buffer.as_ref();
		let current_offset = self.offset as usize;
		if current_offset >= slice.len() {
			return Ok(0);
		}
		let remaining = &slice[current_offset..];
		let bytes_to_read = remaining.len().min(buf.len());
		if let Some(src) = remaining.get(..bytes_to_read) {
			buf[..bytes_to_read].copy_from_slice(src);
		}
		self.offset += bytes_to_read as u64;
		Ok(bytes_to_read)
	}
}

impl crate::io::MediaWrite for Cursor<Vec<u8>> {
	fn write(&mut self, buf: &[u8]) -> Result<usize> {
		let current_offset = self.offset as usize;
		if current_offset > self.buffer.len() {
			self.buffer.resize(current_offset, 0);
		}
		let available_space = self.buffer.len().saturating_sub(current_offset);
		let overwrite = available_space.min(buf.len());
		if let Some(dst) = self.buffer.get_mut(current_offset..current_offset + overwrite) {
			dst.copy_from_slice(&buf[..overwrite]);
		}
		if buf.len() > overwrite {
			self.buffer.extend_from_slice(&buf[overwrite..]);
		}
		self.offset += buf.len() as u64;
		Ok(buf.len())
	}

	#[inline(always)]
	fn flush(&mut self) -> Result<()> {
		Ok(())
	}
}

impl<T: AsRef<[u8]>> crate::io::MediaSeek for Cursor<T> {
	fn seek(&mut self, seek_form: crate::io::SeekFrom) -> Result<u64> {
		let len = self.buffer.as_ref().len() as i64;
		let new_offset = match seek_form {
			crate::io::SeekFrom::Start(n) => n as i64,
			crate::io::SeekFrom::End(n) => len + n,
			crate::io::SeekFrom::Current(n) => self.offset as i64 + n,
		};
		let new_offset = new_offset.max(0).min(len) as u64;
		self.offset = new_offset;
		Ok(self.offset)
	}
}

impl<T: AsRef<[u8]>> StdRead for Cursor<T> {
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
		crate::io::MediaRead::read(self, buf)
			.map_err(|message| std::io::Error::new(std::io::ErrorKind::Other, message.text))
	}
}

impl StdWrite for Cursor<Vec<u8>> {
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
		crate::io::MediaWrite::write(self, buf)
			.map_err(|message| std::io::Error::new(std::io::ErrorKind::Other, message.text))
	}
	fn flush(&mut self) -> std::io::Result<()> {
		crate::io::MediaWrite::flush(self)
			.map_err(|message| std::io::Error::new(std::io::ErrorKind::Other, message.text))
	}
}

impl<T: AsRef<[u8]>> StdSeek for Cursor<T> {
	fn seek(&mut self, seek_from: SeekFrom) -> std::io::Result<u64> {
		let seek_from = match seek_from {
			SeekFrom::Start(n) => crate::io::SeekFrom::Start(n),
			SeekFrom::End(n) => crate::io::SeekFrom::End(n),
			SeekFrom::Current(n) => crate::io::SeekFrom::Current(n),
		};
		crate::io::MediaSeek::seek(self, seek_from)
			.map_err(|message| std::io::Error::new(std::io::ErrorKind::Other, message.text))
	}
}
