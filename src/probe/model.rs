use crate::core::frame;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaFile {
	pub schema: String,
	pub input: InputFile,
	pub outputs: Vec<OutputFile>,
}
impl MediaFile {
	pub fn new(input: InputFile, outputs: Vec<OutputFile>) -> Self {
		let version = env!("CARGO_PKG_VERSION");
		let schema = format!("osaka/{}", version);
		Self { schema, input, outputs }
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputFile {
	pub path: String,
	pub format: Option<String>,
	pub duration: Option<u64>,
	pub size: Option<u64>,
	pub summary: Option<Summary>,
	pub tracks: Vec<Track>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputFile {
	pub path: String,
	pub tracks: Vec<Track>,
	pub metrics: Option<Metrics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
	pub width: Option<u32>,
	pub height: Option<u32>,
	pub frame_rate: Option<f32>,

	pub sample_rate: Option<frame::SampleRate>,
	pub channels: Option<frame::Channels>,
	pub bit_depth: Option<frame::BitDepth>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
	pub id: u32,
	pub kind: TrackKind,
	pub codec: Option<String>,

	pub duration: Option<u64>,
	pub bitrate: Option<u32>,

	pub video: Option<VideoParams>,
	pub audio: Option<AudioParams>,

	pub tags: Option<Vec<Tag>>,
	// pub verified: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TrackKind {
	Video,
	Audio,
	Subtitle,
	Image,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoParams {
	pub width: Option<u32>,
	pub height: Option<u32>,
	pub frame_rate: Option<f32>,
	pub pixel_format: Option<String>,
	pub color_space: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioParams {
	pub channel_layout: Option<String>,
	pub sample_rate: Option<frame::SampleRate>,
	pub channels: Option<frame::Channels>,
	pub bit_depth: Option<frame::BitDepth>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metrics {
	pub size: Option<u64>,
	pub bitrate: Option<u32>,
	pub ratio: Option<f32>,
	pub duration_delta: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
	pub key: String,
	pub value: String,
}

impl std::fmt::Display for TrackKind {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			TrackKind::Video => write!(f, "video"),
			TrackKind::Audio => write!(f, "audio"),
			TrackKind::Subtitle => write!(f, "subtitle"),
			TrackKind::Image => write!(f, "image"),
		}
	}
}
