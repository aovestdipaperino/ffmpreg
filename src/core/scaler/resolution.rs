use crate::core::frame::video::FrameVideo;

pub struct ResolutionScaler {
	input_width: u32,
	input_height: u32,
	output_width: u32,
	output_height: u32,
}

impl ResolutionScaler {
	pub fn new(
		input_width: u32,
		input_height: u32,
		output_width: u32,
		output_height: u32,
	) -> Self {
		Self { input_width, input_height, output_width, output_height }
	}

	pub fn is_needed(&self) -> bool {
		self.input_width != self.output_width || self.input_height != self.output_height
	}

	pub fn scale(&self, video: &FrameVideo) -> FrameVideo {
		if !self.is_needed() {
			return video.clone();
		}

		let y = self.scale_plane(
			video.y_plane(),
			video.width as usize,
			video.height as usize,
			self.output_width as usize,
			self.output_height as usize,
		);

		let in_chroma = chroma_dims(video.pixel.format, video.width, video.height);
		let out_chroma = chroma_dims(video.pixel.format, self.output_width, self.output_height);

		let u = self.scale_plane(
			video.u_plane(),
			in_chroma.0,
			in_chroma.1,
			out_chroma.0,
			out_chroma.1,
		);

		let v = self.scale_plane(
			video.v_plane(),
			in_chroma.0,
			in_chroma.1,
			out_chroma.0,
			out_chroma.1,
		);

		FrameVideo::new(y, u, v, self.output_width, self.output_height, video.keyframe, video.pixel)
			.with_pts(video.pts)
	}

	fn scale_plane(
		&self,
		src: &[u8],
		src_w: usize,
		src_h: usize,
		dst_w: usize,
		dst_h: usize,
	) -> Vec<u8> {
		if src_w == dst_w && src_h == dst_h {
			return src.to_vec();
		}

		let mut dst = Vec::with_capacity(dst_w * dst_h);

		let x_ratio = src_w as f64 / dst_w as f64;
		let y_ratio = src_h as f64 / dst_h as f64;

		for row in 0..dst_h {
			let src_y = row as f64 * y_ratio;
			let y0 = src_y as usize;
			let y1 = (y0 + 1).min(src_h - 1);
			let y_frac = src_y - y0 as f64;

			for col in 0..dst_w {
				let src_x = col as f64 * x_ratio;
				let x0 = src_x as usize;
				let x1 = (x0 + 1).min(src_w - 1);
				let x_frac = src_x - x0 as f64;

				let tl = src[y0 * src_w + x0] as f64;
				let tr = src[y0 * src_w + x1] as f64;
				let bl = src[y1 * src_w + x0] as f64;
				let br = src[y1 * src_w + x1] as f64;

				let top = tl + (tr - tl) * x_frac;
				let bottom = bl + (br - bl) * x_frac;
				let value = top + (bottom - top) * y_frac;

				dst.push(value.round().clamp(0.0, 255.0) as u8);
			}
		}

		dst
	}
}

fn chroma_dims(format: crate::core::frame::video::Format, width: u32, height: u32) -> (usize, usize) {
	use crate::core::frame::video::Format;
	match format {
		Format::YUV444 => (width as usize, height as usize),
		Format::YUV422 => ((width as usize + 1) / 2, height as usize),
		Format::YUV420 => ((width as usize + 1) / 2, (height as usize + 1) / 2),
	}
}
