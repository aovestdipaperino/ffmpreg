pub mod format;
pub mod resolution;

use crate::core::frame::Frame;
use crate::core::frame::video::PixelFormat;
use crate::core::track::VideoFormat;
use crate::core::traits::Scaler;
use crate::message::Result;

use format::FormatConverter;
use resolution::ResolutionScaler;

pub struct VideoScaler {
	input: VideoFormat,
	output: VideoFormat,
	resolution: ResolutionScaler,
	format_conv: FormatConverter,
}

impl VideoScaler {
	pub fn new(
		input: VideoFormat,
		output: VideoFormat,
		input_pixel: PixelFormat,
		output_pixel: PixelFormat,
	) -> Self {
		let resolution = ResolutionScaler::new(input.width, input.height, output.width, output.height);
		let format_conv = FormatConverter::new(input_pixel.format, output_pixel.format);
		Self { input, output, resolution, format_conv }
	}
}

impl Scaler for VideoScaler {
	fn input_format(&self) -> VideoFormat {
		self.input
	}

	fn output_format(&self) -> VideoFormat {
		self.output
	}

	fn needed(&self) -> bool {
		self.resolution.is_needed() || self.format_conv.is_needed()
	}

	fn scale(&mut self, frame: Frame) -> Result<Frame> {
		let video = match frame.video() {
			Some(video) => video,
			None => return Ok(frame),
		};

		let needs_res = self.resolution.is_needed();
		let needs_fmt = self.format_conv.is_needed();

		if !needs_res && !needs_fmt {
			return Ok(frame);
		}

		let track_id = frame.track_id;

		let mut current = video.clone();

		if needs_fmt {
			current = self.format_conv.convert(&current);
		}

		if needs_res {
			current = self.resolution.scale(&current);
		}

		Ok(Frame::new_video(current, track_id))
	}
}
