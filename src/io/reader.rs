#![allow(dead_code)]
pub const DEFAULT_BUFFER_SIZE: usize = 8192;
use crate::{error, message::Result};

pub trait MediaRead {
	fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
	fn extension(&self) -> Option<String>;
}

pub trait BinaryRead: MediaRead {
	fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
		let mut filled = 0;
		while filled < buf.len() {
			match self.read(&mut buf[filled..]) {
				Ok(0) => return Err(error!("unexpected EOF")),
				Ok(n) => filled += n,
				Err(e) => return Err(e),
			}
		}
		Ok(())
	}

	#[inline]
	fn read_u8(&mut self) -> Result<u8> {
		let mut buf = [0u8; 1];
		self.read_exact(&mut buf)?;
		Ok(buf[0])
	}

	#[inline]
	fn read_u16_be(&mut self) -> Result<u16> {
		let mut buf = [0u8; 2];
		self.read_exact(&mut buf)?;
		Ok(u16::from_be_bytes(buf))
	}

	#[inline]
	fn read_u16_le(&mut self) -> Result<u16> {
		let mut buf = [0u8; 2];
		self.read_exact(&mut buf)?;
		Ok(u16::from_le_bytes(buf))
	}

	#[inline]
	fn read_u32_be(&mut self) -> Result<u32> {
		let mut buf = [0u8; 4];
		self.read_exact(&mut buf)?;
		Ok(u32::from_be_bytes(buf))
	}

	#[inline]
	fn read_u32_le(&mut self) -> Result<u32> {
		let mut buf = [0u8; 4];
		self.read_exact(&mut buf)?;
		Ok(u32::from_le_bytes(buf))
	}

	#[inline]
	fn read_u64_be(&mut self) -> Result<u64> {
		let mut buf = [0u8; 8];
		self.read_exact(&mut buf)?;
		Ok(u64::from_be_bytes(buf))
	}

	#[inline]
	fn read_u64_le(&mut self) -> Result<u64> {
		let mut buf = [0u8; 8];
		self.read_exact(&mut buf)?;
		Ok(u64::from_le_bytes(buf))
	}

	#[inline]
	fn read_i8(&mut self) -> Result<i8> {
		let mut buf = [0u8; 1];
		self.read_exact(&mut buf)?;
		Ok(buf[0] as i8)
	}

	#[inline]
	fn read_i16_be(&mut self) -> Result<i16> {
		let mut buf = [0u8; 2];
		self.read_exact(&mut buf)?;
		Ok(i16::from_be_bytes(buf))
	}

	#[inline]
	fn read_i16_le(&mut self) -> Result<i16> {
		let mut buf = [0u8; 2];
		self.read_exact(&mut buf)?;
		Ok(i16::from_le_bytes(buf))
	}

	#[inline]
	fn read_i32_be(&mut self) -> Result<i32> {
		let mut buf = [0u8; 4];
		self.read_exact(&mut buf)?;
		Ok(i32::from_be_bytes(buf))
	}

	#[inline]
	fn read_i32_le(&mut self) -> Result<i32> {
		let mut buf = [0u8; 4];
		self.read_exact(&mut buf)?;
		Ok(i32::from_le_bytes(buf))
	}

	#[inline]
	fn read_i64_be(&mut self) -> Result<i64> {
		let mut buf = [0u8; 8];
		self.read_exact(&mut buf)?;
		Ok(i64::from_be_bytes(buf))
	}

	#[inline]
	fn read_i64_le(&mut self) -> Result<i64> {
		let mut buf = [0u8; 8];
		self.read_exact(&mut buf)?;
		Ok(i64::from_le_bytes(buf))
	}

	#[inline]
	fn read_f32_be(&mut self) -> Result<f32> {
		let mut buf = [0u8; 4];
		self.read_exact(&mut buf)?;
		Ok(f32::from_be_bytes(buf))
	}

	#[inline]
	fn read_f32_le(&mut self) -> Result<f32> {
		let mut buf = [0u8; 4];
		self.read_exact(&mut buf)?;
		Ok(f32::from_le_bytes(buf))
	}

	#[inline]
	fn read_f64_be(&mut self) -> Result<f64> {
		let mut buf = [0u8; 8];
		self.read_exact(&mut buf)?;
		Ok(f64::from_be_bytes(buf))
	}

	#[inline]
	fn read_f64_le(&mut self) -> Result<f64> {
		let mut buf = [0u8; 8];
		self.read_exact(&mut buf)?;
		Ok(f64::from_le_bytes(buf))
	}
}

impl<T: MediaRead> BinaryRead for T {}

pub struct StdReadAdapter<R> {
	inner: R,
}

impl<R> StdReadAdapter<R> {
	#[inline]
	pub const fn new(inner: R) -> Self {
		Self { inner }
	}

	#[inline]
	pub fn into_inner(self) -> R {
		self.inner
	}

	#[inline]
	pub const fn get_ref(&self) -> &R {
		&self.inner
	}

	#[inline]
	pub fn get_mut(&mut self) -> &mut R {
		&mut self.inner
	}
}

impl<R: std::io::Read> MediaRead for StdReadAdapter<R> {
	#[inline]
	fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
		self.inner.read(buf).map_err(|e| crate::message::Message::from(e))
	}
	fn extension(&self) -> Option<String> {
		None
	}
}

pub struct BufferedReader<R, const N: usize = DEFAULT_BUFFER_SIZE> {
	inner: R,
	buffer: [u8; N],
	pos: usize,
	filled: usize,
}

impl<R, const N: usize> BufferedReader<R, N> {
	pub fn new(inner: R) -> Self {
		Self { inner, buffer: [0u8; N], pos: 0, filled: 0 }
	}

	#[inline]
	pub fn into_inner(self) -> R {
		self.inner
	}

	#[inline]
	pub const fn get_ref(&self) -> &R {
		&self.inner
	}

	#[inline]
	pub fn get_mut(&mut self) -> &mut R {
		&mut self.inner
	}

	#[inline]
	pub fn buffer(&self) -> &[u8] {
		&self.buffer[self.pos..self.filled]
	}

	#[inline]
	pub const fn capacity(&self) -> usize {
		N
	}

	#[inline]
	fn consume(&mut self, amt: usize) {
		self.pos = core::cmp::min(self.pos + amt, self.filled);
	}

	#[inline]
	fn discard_buffer(&mut self) {
		self.pos = 0;
		self.filled = 0;
	}
}

impl<R: MediaRead, const N: usize> BufferedReader<R, N> {
	fn fill_buf(&mut self) -> Result<&[u8]> {
		if self.pos >= self.filled {
			self.discard_buffer();
			self.filled = self.inner.read(&mut self.buffer)?;
		}
		Ok(&self.buffer[self.pos..self.filled])
	}
}

impl<R: MediaRead, const N: usize> MediaRead for BufferedReader<R, N> {
	fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
		if buf.len() >= N && self.pos >= self.filled {
			self.discard_buffer();
			return self.inner.read(buf);
		}

		let available = self.fill_buf()?;
		let amt = core::cmp::min(available.len(), buf.len());
		buf[..amt].copy_from_slice(&available[..amt]);
		self.consume(amt);
		Ok(amt)
	}
	fn extension(&self) -> Option<String> {
		None
	}
}

impl MediaRead for &[u8] {
	fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
		let amt = core::cmp::min(self.len(), buf.len());
		let (a, b) = self.split_at(amt);
		buf[..amt].copy_from_slice(a);
		*self = b;
		Ok(amt)
	}
	fn extension(&self) -> Option<String> {
		None
	}
}

pub trait MediaWrite {
	fn write(&mut self, buf: &[u8]) -> Result<usize>;
	fn flush(&mut self) -> Result<()>;
}

pub trait BinaryWrite: MediaWrite {
	fn write_all(&mut self, buf: &[u8]) -> Result<()> {
		let mut written = 0;
		while written < buf.len() {
			match self.write(&buf[written..]) {
				Ok(0) => return Err(crate::error!("write returned zero")),
				Ok(n) => written += n,
				Err(e) => return Err(e),
			}
		}
		Ok(())
	}

	#[inline]
	fn write_u8(&mut self, value: u8) -> Result<()> {
		self.write_all(&[value])
	}
	#[inline]
	fn write_u16_be(&mut self, value: u16) -> Result<()> {
		self.write_all(&value.to_be_bytes())
	}
	#[inline]
	fn write_u16_le(&mut self, value: u16) -> Result<()> {
		self.write_all(&value.to_le_bytes())
	}
	#[inline]
	fn write_u32_be(&mut self, value: u32) -> Result<()> {
		self.write_all(&value.to_be_bytes())
	}
	#[inline]
	fn write_u32_le(&mut self, value: u32) -> Result<()> {
		self.write_all(&value.to_le_bytes())
	}
	#[inline]
	fn write_u64_be(&mut self, value: u64) -> Result<()> {
		self.write_all(&value.to_be_bytes())
	}
	#[inline]
	fn write_u64_le(&mut self, value: u64) -> Result<()> {
		self.write_all(&value.to_le_bytes())
	}
	#[inline]
	fn write_i8(&mut self, value: i8) -> Result<()> {
		self.write_all(&[value as u8])
	}
	#[inline]
	fn write_i16_be(&mut self, value: i16) -> Result<()> {
		self.write_all(&value.to_be_bytes())
	}
	#[inline]
	fn write_i16_le(&mut self, value: i16) -> Result<()> {
		self.write_all(&value.to_le_bytes())
	}
	#[inline]
	fn write_i32_be(&mut self, value: i32) -> Result<()> {
		self.write_all(&value.to_be_bytes())
	}
	#[inline]
	fn write_i32_le(&mut self, value: i32) -> Result<()> {
		self.write_all(&value.to_le_bytes())
	}
	#[inline]
	fn write_i64_be(&mut self, value: i64) -> Result<()> {
		self.write_all(&value.to_be_bytes())
	}
	#[inline]
	fn write_i64_le(&mut self, value: i64) -> Result<()> {
		self.write_all(&value.to_le_bytes())
	}
	#[inline]
	fn write_f32_be(&mut self, value: f32) -> Result<()> {
		self.write_all(&value.to_be_bytes())
	}
	#[inline]
	fn write_f32_le(&mut self, value: f32) -> Result<()> {
		self.write_all(&value.to_le_bytes())
	}
	#[inline]
	fn write_f64_be(&mut self, value: f64) -> Result<()> {
		self.write_all(&value.to_be_bytes())
	}
	#[inline]
	fn write_f64_le(&mut self, value: f64) -> Result<()> {
		self.write_all(&value.to_le_bytes())
	}
}

impl<T: MediaWrite> BinaryWrite for T {}

pub struct StdWriteAdapter<W> {
	inner: W,
}

impl<W> StdWriteAdapter<W> {
	#[inline]
	pub const fn new(inner: W) -> Self {
		Self { inner }
	}
	#[inline]
	pub fn into_inner(self) -> W {
		self.inner
	}
	#[inline]
	pub const fn get_ref(&self) -> &W {
		&self.inner
	}
	#[inline]
	pub fn get_mut(&mut self) -> &mut W {
		&mut self.inner
	}
}

impl<W: std::io::Write> MediaWrite for StdWriteAdapter<W> {
	#[inline]
	fn write(&mut self, buf: &[u8]) -> Result<usize> {
		self.inner.write(buf).map_err(|e| crate::message::Message::from(e))
	}
	#[inline]
	fn flush(&mut self) -> Result<()> {
		self.inner.flush().map_err(|e| crate::message::Message::from(e))
	}
}

impl MediaWrite for Vec<u8> {
	#[inline]
	fn write(&mut self, buf: &[u8]) -> Result<usize> {
		self.extend_from_slice(buf);
		Ok(buf.len())
	}
	#[inline]
	fn flush(&mut self) -> Result<()> {
		Ok(())
	}
}

pub struct BufferedWriter<W, const N: usize = DEFAULT_BUFFER_SIZE> {
	inner: W,
	buffer: Vec<u8>,
}

impl<W, const N: usize> BufferedWriter<W, N> {
	pub fn new(inner: W) -> Self {
		Self { inner, buffer: Vec::with_capacity(N) }
	}
	#[inline]
	pub fn into_inner(self) -> W {
		self.inner
	}
	#[inline]
	pub const fn get_ref(&self) -> &W {
		&self.inner
	}
	#[inline]
	pub fn get_mut(&mut self) -> &mut W {
		&mut self.inner
	}
	#[inline]
	pub const fn capacity(&self) -> usize {
		N
	}
}

impl<W: crate::io::MediaWrite, const N: usize> BufferedWriter<W, N> {
	fn flush_buf(&mut self) -> Result<()> {
		if !self.buffer.is_empty() {
			let mut written = 0;
			while written < self.buffer.len() {
				match self.inner.write(&self.buffer[written..]) {
					Ok(0) => return Err(crate::error!("write returned zero")),
					Ok(n) => written += n,
					Err(e) => return Err(e),
				}
			}
			self.buffer.clear();
		}
		Ok(())
	}
}

impl<W: crate::io::MediaWrite, const N: usize> MediaWrite for BufferedWriter<W, N> {
	fn write(&mut self, buf: &[u8]) -> Result<usize> {
		if self.buffer.len() + buf.len() > N {
			self.flush_buf()?;
		}
		if buf.len() >= N {
			return self.inner.write(buf);
		}
		self.buffer.extend_from_slice(buf);
		Ok(buf.len())
	}

	fn flush(&mut self) -> Result<()> {
		self.flush_buf()?;
		self.inner.flush()
	}
}
