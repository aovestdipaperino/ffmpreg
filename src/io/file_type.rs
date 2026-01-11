#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileType {
	// Video containers
	Mp4,
	Mkv,
	Mov,
	Webm,
	Avi,
	Ogv,
	Flv,
	Mxf,
	Ts,
	Ogg,
	// Audio codecs/streams
	Mp3,
	Aac,
	Opus,
	Flac,
	Wav,
	Raw,
	Pcm,
	M4a,
	Alac,
	// Image formats
	Jpeg,
	Jpg,
	Png,
	Bmp,
	Tiff,
	Webp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileCategory {
	Video,
	Audio,
	Image,
}

impl FileType {
	pub fn extension(&self) -> &'static str {
		match self {
			FileType::Mp4 => "mp4",
			FileType::Mkv => "mkv",
			FileType::Mov => "mov",
			FileType::Webm => "webm",
			FileType::Avi => "avi",
			FileType::Ogv => "ogv",
			FileType::Flv => "flv",
			FileType::Mxf => "mxf",
			FileType::Ts => "ts",
			FileType::Ogg => "ogg",
			FileType::Mp3 => "mp3",
			FileType::Aac => "aac",
			FileType::Opus => "opus",
			FileType::Flac => "flac",
			FileType::Wav => "wav",
			FileType::Raw => "raw",
			FileType::Pcm => "pcm",
			FileType::M4a => "m4a",
			FileType::Alac => "alac",
			FileType::Jpeg => "jpeg",
			FileType::Jpg => "jpg",
			FileType::Png => "png",
			FileType::Bmp => "bmp",
			FileType::Tiff => "tiff",
			FileType::Webp => "webp",
		}
	}

	pub fn category(&self) -> FileCategory {
		match self {
			FileType::Mp4
			| FileType::Mkv
			| FileType::Mov
			| FileType::Webm
			| FileType::Avi
			| FileType::Ogv
			| FileType::Flv
			| FileType::Mxf
			| FileType::Ts
			| FileType::Ogg => FileCategory::Video,

			FileType::Mp3
			| FileType::Aac
			| FileType::Opus
			| FileType::Flac
			| FileType::Wav
			| FileType::Raw
			| FileType::Pcm
			| FileType::M4a
			| FileType::Alac => FileCategory::Audio,

			FileType::Jpeg
			| FileType::Jpg
			| FileType::Png
			| FileType::Bmp
			| FileType::Tiff
			| FileType::Webp => FileCategory::Image,
		}
	}

	pub fn is_audio(&self) -> bool {
		matches!(self.category(), FileCategory::Audio)
	}

	pub fn is_video(&self) -> bool {
		matches!(self.category(), FileCategory::Video)
	}

	pub fn is_image(&self) -> bool {
		matches!(self.category(), FileCategory::Image)
	}
}
