use crate::container::ContainerId;
// ISO Base Media File Format (ISOBMFF)
pub const MP4: ContainerId = ContainerId::new("mp4");
pub const MOV: ContainerId = ContainerId::new("mov");
pub const M4A: ContainerId = ContainerId::new("m4a");
pub const M4V: ContainerId = ContainerId::new("m4v");
pub const FMP4: ContainerId = ContainerId::new("fmp4");

// matroska
pub const MKV: ContainerId = ContainerId::new("mkv");
pub const WEBM: ContainerId = ContainerId::new("webm");

// MPEG
pub const MPEG_PS: ContainerId = ContainerId::new("mpeg");
pub const MPEG_TS: ContainerId = ContainerId::new("ts");

// AVI
pub const AVI: ContainerId = ContainerId::new("avi");

// flash
pub const FLV: ContainerId = ContainerId::new("flv");

// ogg
pub const OGG: ContainerId = ContainerId::new("ogg");
pub const OGA: ContainerId = ContainerId::new("oga");
pub const OGV: ContainerId = ContainerId::new("ogv");

// Aadio-specific
pub const WAV: ContainerId = ContainerId::new("wav");
pub const AIFF: ContainerId = ContainerId::new("aiff");
pub const CAF: ContainerId = ContainerId::new("caf");

// lossless / codec-wrapped
pub const FLAC: ContainerId = ContainerId::new("flac");

// MP3 container
pub const MP3: ContainerId = ContainerId::new("mp3");

// Images
pub const PNG: ContainerId = ContainerId::new("png");
pub const JPEG: ContainerId = ContainerId::new("jpeg");
pub const JPG: ContainerId = ContainerId::new("jpg");
pub const GIF: ContainerId = ContainerId::new("gif");
pub const BMP: ContainerId = ContainerId::new("bmp");
pub const TIFF: ContainerId = ContainerId::new("tiff"); // também .tif
pub const WEBP: ContainerId = ContainerId::new("webp");
pub const ICO: ContainerId = ContainerId::new("ico");
pub const HEIF: ContainerId = ContainerId::new("heif");
pub const AVIF: ContainerId = ContainerId::new("avif");

// chunk size limit for demuxer packet reading (20KB)
// pub const CHUNK_SIZE_LIMIT: usize = 20000;
pub const CHUNK_SIZE_LIMIT: usize = 65536;
