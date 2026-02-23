use crate::core::frame::video::{Format, FrameVideo, PixelFormat};

pub struct FormatConverter {
	input: Format,
	output: Format,
}

impl FormatConverter {
	pub fn new(input: Format, output: Format) -> Self {
		Self { input, output }
	}

	pub fn is_needed(&self) -> bool {
		self.input != self.output
	}

	pub fn convert(&self, video: &FrameVideo) -> FrameVideo {
		if !self.is_needed() {
			return video.clone();
		}

		match (self.input, self.output) {
			(Format::YUV420, Format::YUV422) => self.yuv420_to_yuv422(video),
			(Format::YUV420, Format::YUV444) => self.yuv420_to_yuv444(video),
			(Format::YUV422, Format::YUV420) => self.yuv422_to_yuv420(video),
			(Format::YUV422, Format::YUV444) => self.yuv422_to_yuv444(video),
			(Format::YUV444, Format::YUV420) => self.yuv444_to_yuv420(video),
			(Format::YUV444, Format::YUV422) => self.yuv444_to_yuv422(video),
			_ => video.clone(),
		}
	}

	fn yuv420_to_yuv422(&self, video: &FrameVideo) -> FrameVideo {
		let w = video.width as usize;
		let h = video.height as usize;
		let chroma_w = (w + 1) / 2;
		let chroma_h = (h + 1) / 2;

		let y = video.y_plane().to_vec();

		let u_in = video.u_plane();
		let v_in = video.v_plane();
		let out_uv_size = chroma_w * h;
		let mut u_out = Vec::with_capacity(out_uv_size);
		let mut v_out = Vec::with_capacity(out_uv_size);

		for row in 0..h {
			let src_row = (row / 2).min(chroma_h - 1);
			let src_offset = src_row * chroma_w;
			u_out.extend_from_slice(&u_in[src_offset..src_offset + chroma_w]);
			v_out.extend_from_slice(&v_in[src_offset..src_offset + chroma_w]);
		}

		let pixel = PixelFormat::yuv422(video.pixel.depth);
		FrameVideo::new(y, u_out, v_out, video.width, video.height, video.keyframe, pixel)
			.with_pts(video.pts)
	}

	fn yuv420_to_yuv444(&self, video: &FrameVideo) -> FrameVideo {
		let w = video.width as usize;
		let h = video.height as usize;
		let chroma_w = (w + 1) / 2;
		let chroma_h = (h + 1) / 2;

		let y = video.y_plane().to_vec();

		let u_in = video.u_plane();
		let v_in = video.v_plane();
		let out_size = w * h;
		let mut u_out = Vec::with_capacity(out_size);
		let mut v_out = Vec::with_capacity(out_size);

		for row in 0..h {
			let src_row = (row / 2).min(chroma_h - 1);
			for col in 0..w {
				let src_col = (col / 2).min(chroma_w - 1);
				let src_idx = src_row * chroma_w + src_col;
				u_out.push(u_in[src_idx]);
				v_out.push(v_in[src_idx]);
			}
		}

		let pixel = PixelFormat::yuv444(video.pixel.depth);
		FrameVideo::new(y, u_out, v_out, video.width, video.height, video.keyframe, pixel)
			.with_pts(video.pts)
	}

	fn yuv422_to_yuv420(&self, video: &FrameVideo) -> FrameVideo {
		let w = video.width as usize;
		let h = video.height as usize;
		let chroma_w = (w + 1) / 2;
		let chroma_h = (h + 1) / 2;

		let y = video.y_plane().to_vec();

		let u_in = video.u_plane();
		let v_in = video.v_plane();
		let out_uv_size = chroma_w * chroma_h;
		let mut u_out = Vec::with_capacity(out_uv_size);
		let mut v_out = Vec::with_capacity(out_uv_size);

		for row in 0..chroma_h {
			let row0 = row * 2;
			let row1 = (row0 + 1).min(h - 1);
			let off0 = row0 * chroma_w;
			let off1 = row1 * chroma_w;
			for col in 0..chroma_w {
				let u_avg = ((u_in[off0 + col] as u16 + u_in[off1 + col] as u16) / 2) as u8;
				let v_avg = ((v_in[off0 + col] as u16 + v_in[off1 + col] as u16) / 2) as u8;
				u_out.push(u_avg);
				v_out.push(v_avg);
			}
		}

		let pixel = PixelFormat::yuv420(video.pixel.depth);
		FrameVideo::new(y, u_out, v_out, video.width, video.height, video.keyframe, pixel)
			.with_pts(video.pts)
	}

	fn yuv422_to_yuv444(&self, video: &FrameVideo) -> FrameVideo {
		let w = video.width as usize;
		let h = video.height as usize;
		let chroma_w = (w + 1) / 2;

		let y = video.y_plane().to_vec();

		let u_in = video.u_plane();
		let v_in = video.v_plane();
		let out_size = w * h;
		let mut u_out = Vec::with_capacity(out_size);
		let mut v_out = Vec::with_capacity(out_size);

		for row in 0..h {
			let src_offset = row * chroma_w;
			for col in 0..w {
				let src_col = (col / 2).min(chroma_w - 1);
				u_out.push(u_in[src_offset + src_col]);
				v_out.push(v_in[src_offset + src_col]);
			}
		}

		let pixel = PixelFormat::yuv444(video.pixel.depth);
		FrameVideo::new(y, u_out, v_out, video.width, video.height, video.keyframe, pixel)
			.with_pts(video.pts)
	}

	fn yuv444_to_yuv420(&self, video: &FrameVideo) -> FrameVideo {
		let w = video.width as usize;
		let h = video.height as usize;
		let chroma_w = (w + 1) / 2;
		let chroma_h = (h + 1) / 2;

		let y = video.y_plane().to_vec();

		let u_in = video.u_plane();
		let v_in = video.v_plane();
		let out_uv_size = chroma_w * chroma_h;
		let mut u_out = Vec::with_capacity(out_uv_size);
		let mut v_out = Vec::with_capacity(out_uv_size);

		for row in 0..chroma_h {
			let row0 = row * 2;
			let row1 = (row0 + 1).min(h - 1);
			for col in 0..chroma_w {
				let col0 = col * 2;
				let col1 = (col0 + 1).min(w - 1);

				let u_sum = u_in[row0 * w + col0] as u16
					+ u_in[row0 * w + col1] as u16
					+ u_in[row1 * w + col0] as u16
					+ u_in[row1 * w + col1] as u16;
				let v_sum = v_in[row0 * w + col0] as u16
					+ v_in[row0 * w + col1] as u16
					+ v_in[row1 * w + col0] as u16
					+ v_in[row1 * w + col1] as u16;

				u_out.push((u_sum / 4) as u8);
				v_out.push((v_sum / 4) as u8);
			}
		}

		let pixel = PixelFormat::yuv420(video.pixel.depth);
		FrameVideo::new(y, u_out, v_out, video.width, video.height, video.keyframe, pixel)
			.with_pts(video.pts)
	}

	fn yuv444_to_yuv422(&self, video: &FrameVideo) -> FrameVideo {
		let w = video.width as usize;
		let h = video.height as usize;
		let chroma_w = (w + 1) / 2;

		let y = video.y_plane().to_vec();

		let u_in = video.u_plane();
		let v_in = video.v_plane();
		let out_uv_size = chroma_w * h;
		let mut u_out = Vec::with_capacity(out_uv_size);
		let mut v_out = Vec::with_capacity(out_uv_size);

		for row in 0..h {
			let src_offset = row * w;
			for col in 0..chroma_w {
				let col0 = col * 2;
				let col1 = (col0 + 1).min(w - 1);
				let u_avg = ((u_in[src_offset + col0] as u16 + u_in[src_offset + col1] as u16) / 2) as u8;
				let v_avg = ((v_in[src_offset + col0] as u16 + v_in[src_offset + col1] as u16) / 2) as u8;
				u_out.push(u_avg);
				v_out.push(v_avg);
			}
		}

		let pixel = PixelFormat::yuv422(video.pixel.depth);
		FrameVideo::new(y, u_out, v_out, video.width, video.height, video.keyframe, pixel)
			.with_pts(video.pts)
	}
}
