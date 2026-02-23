mod ext;
mod magic;

pub use ext::{from_extension, from_path};

use crate::container::ContainerId;
use magic::{AudioMatcher, ImageMatcher, VideoMatcher};
use std::io::Read;

const MAGIC_DETECTION_BUFFER_SIZE: usize = 256; // max offset used in magic detection is 38 (mkv)

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MediaType {
	Audio,
	Video,
	Image,
}

#[derive(Debug)]
pub struct Detector {
	pub media_type: MediaType,
	pub container: ContainerId,
}

impl Detector {
	pub fn from_audio(container: ContainerId) -> Self {
		Self { media_type: MediaType::Audio, container }
	}

	pub fn from_video(container: ContainerId) -> Self {
		Self { media_type: MediaType::Video, container }
	}

	pub fn from_image(container: ContainerId) -> Self {
		Self { media_type: MediaType::Image, container }
	}
	pub fn detect(buf: &[u8]) -> Option<Detector> {
		AudioMatcher::detect(buf)
			.map(Detector::from_audio)
			.or_else(|| VideoMatcher::detect(buf).map(Detector::from_video))
			.or_else(|| ImageMatcher::detect(buf).map(Detector::from_image))
	}

	pub fn detect_from_path<P: AsRef<std::path::Path>>(path: P) -> Option<Detector> {
		let path = path.as_ref();
		let mut file = std::fs::File::open(path).ok()?;
		let mut buff = vec![0u8; MAGIC_DETECTION_BUFFER_SIZE];
		let bytes_read = file.read(&mut buff).ok()?;
		buff.truncate(bytes_read);
		from_path(path)
			.and_then(|container| Self::detect_by_container(container, &buff))
			.or_else(|| Self::detect(&buff))
	}

	fn detect_by_container(container: ContainerId, buf: &[u8]) -> Option<Detector> {
		match container.name {
			// Audio
			"flac" if AudioMatcher::flac(buf) => Some(Detector::from_audio(container)),
			"wav" if AudioMatcher::wav(buf) => Some(Detector::from_audio(container)),
			"mp3" if AudioMatcher::mp3(buf) => Some(Detector::from_audio(container)),
			"m4a" if AudioMatcher::m4a(buf) => Some(Detector::from_audio(container)),
			"aiff" if AudioMatcher::aiff(buf) => Some(Detector::from_audio(container)),
			"ogg" | "oga" if AudioMatcher::ogg_opus(buf) || AudioMatcher::ogg(buf) => {
				Some(Detector::from_audio(container))
			}

			// Video
			"mp4" if VideoMatcher::mp4(buf) => Some(Detector::from_video(container)),
			"mkv" if VideoMatcher::mkv(buf) => Some(Detector::from_video(container)),
			"webm" if VideoMatcher::webm(buf) => Some(Detector::from_video(container)),
			"m4v" if VideoMatcher::m4v(buf) => Some(Detector::from_video(container)),
			"mov" if VideoMatcher::mov(buf) => Some(Detector::from_video(container)),
			"avi" if VideoMatcher::avi(buf) => Some(Detector::from_video(container)),
			"flv" if VideoMatcher::flv(buf) => Some(Detector::from_video(container)),
			"mpeg" if VideoMatcher::mpeg(buf) => Some(Detector::from_video(container)),
			"yuv" => Some(Detector::from_video(container)),

			// Image
			"png" if ImageMatcher::png(buf) => Some(Detector::from_image(container)),
			"jpg" | "jpeg" if ImageMatcher::jpeg(buf) => Some(Detector::from_image(container)),
			"gif" if ImageMatcher::gif(buf) => Some(Detector::from_image(container)),
			"bmp" if ImageMatcher::bmp(buf) => Some(Detector::from_image(container)),
			"webp" if ImageMatcher::webp(buf) => Some(Detector::from_image(container)),
			"tiff" if ImageMatcher::tiff(buf) => Some(Detector::from_image(container)),
			"ico" if ImageMatcher::ico(buf) => Some(Detector::from_image(container)),
			"heif" if ImageMatcher::heif(buf) => Some(Detector::from_image(container)),
			"avif" if ImageMatcher::avif(buf) => Some(Detector::from_image(container)),

			_ => None,
		}
	}
}
