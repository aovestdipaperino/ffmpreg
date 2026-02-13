use crate::core::time::Timestamp;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
	YUV420,
	YUV422,
	YUV444,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyframe {
	Key,
	NonKey,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PixelFormat {
	pub depth: u8, // bits per channel
	pub format: Format,
}

#[derive(Debug, Clone)]
pub struct FrameVideo {
	y: Vec<u8>, // size = width * height
	u: Vec<u8>, // size = (width/2) * (height/2)
	v: Vec<u8>, // size = (width/2) * (height/2)
	pub width: u32,
	pub height: u32,
	pub keyframe: Keyframe,
	pub pixel: PixelFormat,
	pub pts: Timestamp,
}

impl PixelFormat {
	pub fn plane_sizes(&self, width: u32, height: u32) -> [usize; 3] {
		match self.format {
			Format::YUV444 => [
				(width * height * self.depth as u32 / 8) as usize,
				(width * height * self.depth as u32 / 8) as usize,
				(width * height * self.depth as u32 / 8) as usize,
			],
			Format::YUV422 => [
				(width * height * self.depth as u32 / 8) as usize,
				(width * height * self.depth as u32 / 16) as usize,
				(width * height * self.depth as u32 / 16) as usize,
			],
			Format::YUV420 => [
				(width * height * self.depth as u32 / 8) as usize,
				(width * height * self.depth as u32 / 32) as usize,
				(width * height * self.depth as u32 / 32) as usize,
			],
		}
	}
}

impl FrameVideo {
	pub fn new_empty(width: u32, height: u32, keyframe: Keyframe, pixel: PixelFormat) -> Self {
		let sizes = pixel.plane_sizes(width, height);
		Self {
			y: Vec::with_capacity(sizes[0]),
			u: Vec::with_capacity(sizes[1]),
			v: Vec::with_capacity(sizes[2]),
			width,
			height,
			keyframe,
			pixel,
			pts: Timestamp::default(),
		}
	}

	pub fn with_pts(mut self, pts: Timestamp) -> Self {
		self.pts = pts;
		self
	}

	pub fn size(&self) -> usize {
		let sizes = self.pixel.plane_sizes(self.width, self.height);
		sizes.iter().sum()
	}

	pub fn is_valid(&self) -> bool {
		let sizes = self.pixel.plane_sizes(self.width, self.height);
		self.y.len() == sizes[0] && self.u.len() == sizes[1] && self.v.len() == sizes[2]
	}

	pub fn duration_seconds(&self, fps: f64) -> f64 {
		1.0 / fps
	}
}

impl fmt::Display for Format {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::YUV420 => write!(f, "YUV420"),
			Self::YUV422 => write!(f, "YUV422"),
			Self::YUV444 => write!(f, "YUV444"),
		}
	}
}

impl fmt::Display for Keyframe {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Key => write!(f, "Key"),
			Self::NonKey => write!(f, "Non-Key"),
		}
	}
}

impl fmt::Display for PixelFormat {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}-bit {}", self.depth, self.format)
	}
}
