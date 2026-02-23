use super::format::YuvFormat;
use crate::core::Muxer;
use crate::core::packet::Packet;
use crate::core::track::{Format, Metadata};
use crate::io::MediaWrite;
use crate::{error, message::Result};

pub struct YuvMuxer<W: MediaWrite> {
	writer: W,
	_format: YuvFormat,
}

impl<W: MediaWrite> YuvMuxer<W> {
	pub fn new(writer: W, format: YuvFormat) -> Result<Self> {
		Ok(Self { writer, _format: format })
	}

	pub fn from_format(writer: W, format: &Format) -> Result<Self> {
		let yuv_format = match format {
			Format::Yuv(yuv) => yuv,
			_ => return Err(error!("yuv does not support non-video formats")),
		};
		Self::new(writer, *yuv_format)
	}
}

impl<W: MediaWrite> Muxer for YuvMuxer<W> {
	fn write(&mut self, packet: Packet) -> Result<()> {
		let mut written = 0;
		while written < packet.data.len() {
			let n = self.writer.write(&packet.data[written..])?;
			if n == 0 {
				return Err(error!("write returned zero"));
			}
			written += n;
		}
		Ok(())
	}

	fn finalize(&mut self) -> Result<()> {
		self.writer.flush()
	}

	fn set_metadata(&mut self, _metadata: Option<Metadata>) {}
}
