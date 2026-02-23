use crate::container;
use std::path::Path;

pub fn from_path<P: AsRef<Path>>(path: P) -> Option<container::ContainerId> {
	let path = path.as_ref();
	let ext = path.extension()?.to_str()?.to_lowercase();

	from_extension(&ext)
}

pub fn from_extension(ext: &str) -> Option<container::ContainerId> {
	let lower = ext.to_lowercase();
	let ext_ref = lower.as_str();

	match ext_ref {
		// Audio
		"flac" => Some(container::FLAC),
		"wav" => Some(container::WAV),
		"mp3" => Some(container::MP3),
		"m4a" => Some(container::M4A),
		"aiff" | "aif" => Some(container::AIFF),
		"ogg" | "oga" => Some(container::OGG),
		"opus" => Some(container::OGG),

		// Video
		"mp4" => Some(container::MP4),
		"mkv" => Some(container::MKV),
		"webm" => Some(container::WEBM),
		"m4v" => Some(container::M4V),
		"mov" | "qt" => Some(container::MOV),
		"avi" => Some(container::AVI),
		"flv" => Some(container::FLV),
		"mpeg" | "mpg" | "mpe" => Some(container::MP4),
		"yuv" => Some(container::YUV),

		// Image
		"png" => Some(container::PNG),
		"jpg" | "jpeg" => Some(container::JPG),
		"gif" => Some(container::GIF),
		"bmp" | "dib" => Some(container::BMP),
		"webp" => Some(container::WEBP),
		"tiff" | "tif" => Some(container::TIFF),
		"ico" => Some(container::ICO),
		"heif" | "heic" => Some(container::HEIF),
		"avif" => Some(container::AVIF),

		_ => None,
	}
}
