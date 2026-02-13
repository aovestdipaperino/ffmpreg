pub mod audio;
pub mod subtitle;
pub mod transform;
pub mod video;
use std::path::PathBuf;

pub use audio::*;
pub use subtitle::*;
pub use transform::*;
pub use video::*;

use crate::{cli::BaseOptions, message};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Options {
	pub input: PathBuf,
	pub output: PathBuf,
	pub audios: Vec<AudioOption>,
	pub videos: Vec<VideoOption>,
	pub subtitles: Vec<SubtitleOption>,
	pub transforms: Vec<TransformOption>,
}

impl BaseOptions {
	pub fn audios(&self) -> message::Result<Vec<AudioOption>> {
		parse_list(&self.audio)
	}

	pub fn videos(&self) -> message::Result<Vec<VideoOption>> {
		parse_list(&self.video)
	}

	pub fn subtitles(&self) -> message::Result<Vec<SubtitleOption>> {
		parse_list(&self.subtitle)
	}

	pub fn transforms(&self) -> message::Result<Vec<TransformOption>> {
		parse_list(&self.apply)
	}
}

fn parse_list<T>(list: &[String]) -> message::Result<Vec<T>>
where
	T: for<'a> TryFrom<&'a str, Error = message::Message>,
{
	list.iter().map(|v| T::try_from(v.as_str())).collect()
}
