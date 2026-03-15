use openh264::decoder::Decoder as OpenH264Decoder;
use openh264::formats::YUVSource;

use crate::core::frame::{Frame, FrameVideo, VideoFormat};
use crate::core::packet::Packet;
use crate::core::traits::Decoder;
use crate::message::Result;

pub struct H264Decoder {
	decoder: OpenH264Decoder,
}

impl H264Decoder {
	pub fn new() -> Result<Self> {
		let decoder =
			OpenH264Decoder::new().map_err(|e| crate::error!("failed to create H264 decoder: {e}"))?;
		Ok(Self { decoder })
	}
}

impl Decoder for H264Decoder {
	fn decode(&mut self, packet: Packet) -> Result<Option<Frame>> {
		if packet.is_empty() {
			return Ok(None);
		}

		let decoded = self
			.decoder
			.decode(&packet.data)
			.map_err(|e| crate::error!("H264 decode error: {e}"))?;

		let yuv = match decoded {
			Some(yuv) => yuv,
			None => return Ok(None),
		};

		let (width, height) = yuv.dimensions();
		let (y_stride, u_stride, v_stride) = yuv.strides();
		let y_plane = yuv.y();
		let u_plane = yuv.u();
		let v_plane = yuv.v();

		let uv_height = height / 2;
		let uv_width = width / 2;

		// Pack YUV420 planes into a contiguous buffer, stripping padding
		let mut data = Vec::with_capacity(width * height + 2 * uv_width * uv_height);
		for row in 0..height {
			let start = row * y_stride;
			data.extend_from_slice(&y_plane[start..start + width]);
		}
		for row in 0..uv_height {
			let start = row * u_stride;
			data.extend_from_slice(&u_plane[start..start + uv_width]);
		}
		for row in 0..uv_height {
			let start = row * v_stride;
			data.extend_from_slice(&v_plane[start..start + uv_width]);
		}

		let video = FrameVideo::new(data, width as u32, height as u32, VideoFormat::YUV420, packet.keyframe);
		let frame = Frame::new_video(video, packet.stream_id).with_pts(packet.pts);

		Ok(Some(frame))
	}

	fn flush(&mut self) -> Result<Option<Frame>> {
		Ok(None)
	}
}
