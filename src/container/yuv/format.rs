use crate::core::frame::video::PixelFormat;
use crate::{codecs, core};

#[derive(Debug, Clone, Copy)]
pub struct YuvFormat {
	pub width: u32,
	pub height: u32,
	pub pixel: PixelFormat,
	pub fps: f64,
}

impl Default for YuvFormat {
	fn default() -> Self {
		Self {
			width: 1920,
			height: 1080,
			pixel: PixelFormat::yuv420(8),
			fps: 30.0,
		}
	}
}

impl YuvFormat {
	pub fn new(width: u32, height: u32, pixel: PixelFormat, fps: f64) -> Self {
		Self { width, height, pixel, fps }
	}

	pub fn frame_size(&self) -> usize {
		self.pixel.total_size(self.width, self.height)
	}

	pub fn to_codec_id(&self) -> core::CodecId {
		codecs::YUV420P
	}
}
