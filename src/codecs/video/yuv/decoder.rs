use crate::container::yuv::YuvFormat;
use crate::core::frame::video::{Keyframe, PixelFormat};
use crate::core::frame::{Frame, FrameIter, FrameVideo};
use crate::core::packet::Packet;
use crate::core::track::VideoFormat;
use crate::core::traits::Decoder;
use crate::message::Result;

pub struct YuvDecoder {
	width: u32,
	height: u32,
	pixel: PixelFormat,
}

impl YuvDecoder {
	pub fn new(width: u32, height: u32, pixel: PixelFormat) -> Self {
		Self { width, height, pixel }
	}

	pub fn from_format(format: &VideoFormat) -> Self {
		let pixel = PixelFormat::yuv420(format.pixel.depth.bits());
		Self::new(format.width, format.height, pixel)
	}

	pub fn from_yuv(yuv: &YuvFormat) -> Self {
		Self::new(yuv.width, yuv.height, yuv.pixel)
	}
}

impl Decoder for YuvDecoder {
	fn decode(&mut self, packet: Packet) -> Result<FrameIter> {
		if packet.is_empty() {
			return Ok(FrameIter::empty());
		}

		let keyframe = if packet.keyframe { Keyframe::Key } else { Keyframe::NonKey };

		let video =
			FrameVideo::from_interleaved(&packet.data, self.width, self.height, keyframe, self.pixel);

		let frame = Frame::new_video(video, packet.track_id);

		Ok(FrameIter::new(vec![frame]))
	}

	fn finish(&mut self) -> Result<FrameIter> {
		Ok(FrameIter::empty())
	}
}
