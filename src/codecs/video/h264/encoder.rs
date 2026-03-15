use openh264::encoder::{Encoder as OpenH264Encoder, EncoderConfig};
use openh264::formats::YUVSlices;

use crate::core::frame::{Frame, VideoFormat};
use crate::core::packet::Packet;
use crate::core::time::Time;
use crate::core::traits::Encoder;
use crate::message::Result;

pub struct H264Encoder {
	encoder: OpenH264Encoder,
	fps: u32,
}

impl H264Encoder {
	pub fn new(fps: u32) -> Result<Self> {
		let encoder =
			OpenH264Encoder::new().map_err(|e| crate::error!("failed to create H264 encoder: {e}"))?;
		Ok(Self { encoder, fps })
	}

	pub fn with_config(config: EncoderConfig, fps: u32) -> Result<Self> {
		let api = openh264::OpenH264API::from_source();
		let encoder = OpenH264Encoder::with_api_config(api, config)
			.map_err(|e| crate::error!("failed to create H264 encoder: {e}"))?;
		Ok(Self { encoder, fps })
	}
}

impl Encoder for H264Encoder {
	fn encode(&mut self, frame: Frame) -> Result<Option<Packet>> {
		let video = match frame.video() {
			Some(v) => v,
			None => return Ok(None),
		};

		if video.format != VideoFormat::YUV420 {
			return Err(crate::error!("H264 encoder requires YUV420 input, got {:?}", video.format));
		}

		let w = video.width as usize;
		let h = video.height as usize;
		let y_size = w * h;
		let uv_size = (w / 2) * (h / 2);

		if video.data.len() < y_size + 2 * uv_size {
			return Err(crate::error!("H264 encoder: frame data too small for {}x{}", w, h));
		}

		let y = &video.data[..y_size];
		let u = &video.data[y_size..y_size + uv_size];
		let v = &video.data[y_size + uv_size..y_size + 2 * uv_size];

		let yuv = YUVSlices::new((y, u, v), (w, h), (w, w / 2, w / 2));

		let bitstream = self
			.encoder
			.encode(&yuv)
			.map_err(|e| crate::error!("H264 encode error: {e}"))?;

		let data = bitstream.to_vec();
		if data.is_empty() {
			return Ok(None);
		}

		let time = Time::new(1, self.fps);
		let packet = Packet::new(data, frame.stream_id, time)
			.with_pts(frame.pts)
			.with_keyframe(video.keyframe);

		Ok(Some(packet))
	}

	fn flush(&mut self) -> Result<Option<Packet>> {
		Ok(None)
	}
}
