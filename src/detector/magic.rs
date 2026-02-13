#![allow(dead_code, unused_variables)]
use crate::container::ContainerId;

pub trait MagicMatcher {
	fn matches(&self, buf: &[u8]) -> bool;
}

pub struct AudioMatcher;
pub struct VideoMatcher;
pub struct ImageMatcher;

impl AudioMatcher {
	pub fn detect(buf: &[u8]) -> Option<ContainerId> {
		use crate::container::{AIFF, FLAC, M4A, MP3, OGG, WAV};

		if Self::flac(buf) {
			return Some(FLAC);
		}
		if Self::wav(buf) {
			return Some(WAV);
		}
		if Self::mp3(buf) {
			return Some(MP3);
		}
		if Self::m4a(buf) {
			return Some(M4A);
		}
		if Self::aiff(buf) {
			return Some(AIFF);
		}
		if Self::ogg_opus(buf) || Self::ogg(buf) {
			return Some(OGG);
		}
		if Self::aac(buf) {
			return Some(M4A);
		}

		None
	}

	pub fn flac(buf: &[u8]) -> bool {
		buf.len() > 3 && buf[0] == 0x66 && buf[1] == 0x4C && buf[2] == 0x61 && buf[3] == 0x43
	}

	pub fn wav(buf: &[u8]) -> bool {
		buf.len() > 11
			&& buf[0] == 0x52
			&& buf[1] == 0x49
			&& buf[2] == 0x46
			&& buf[3] == 0x46
			&& buf[8] == 0x57
			&& buf[9] == 0x41
			&& buf[10] == 0x56
			&& buf[11] == 0x45
	}

	pub fn mp3(buf: &[u8]) -> bool {
		if buf.len() <= 2 {
			return false;
		}

		let id3v2 = buf[0] == 0x49 && buf[1] == 0x44 && buf[2] == 0x33;
		let sync = buf[0] == 0xFF && buf[1] == 0xFB;

		id3v2 || sync
	}

	pub fn m4a(buf: &[u8]) -> bool {
		if buf.len() <= 10 {
			return false;
		}

		let ftyp = buf[4] == 0x66
			&& buf[5] == 0x74
			&& buf[6] == 0x79
			&& buf[7] == 0x70
			&& buf[8] == 0x4D
			&& buf[9] == 0x34
			&& buf[10] == 0x41;

		let brand = buf[0] == 0x4D && buf[1] == 0x34 && buf[2] == 0x41 && buf[3] == 0x20;

		ftyp || brand
	}

	pub fn aiff(buf: &[u8]) -> bool {
		buf.len() > 11
			&& buf[0] == 0x46
			&& buf[1] == 0x4F
			&& buf[2] == 0x52
			&& buf[3] == 0x4D
			&& buf[8] == 0x41
			&& buf[9] == 0x49
			&& buf[10] == 0x46
			&& buf[11] == 0x46
	}

	pub fn ogg(buf: &[u8]) -> bool {
		buf.len() > 3 && buf[0] == 0x4F && buf[1] == 0x67 && buf[2] == 0x67 && buf[3] == 0x53
	}

	pub fn ogg_opus(buf: &[u8]) -> bool {
		if !Self::ogg(buf) || buf.len() <= 35 {
			return false;
		}

		buf[28] == 0x4F
			&& buf[29] == 0x70
			&& buf[30] == 0x75
			&& buf[31] == 0x73
			&& buf[32] == 0x48
			&& buf[33] == 0x65
			&& buf[34] == 0x61
			&& buf[35] == 0x44
	}

	pub fn aac(buf: &[u8]) -> bool {
		buf.len() > 1 && buf[0] == 0xFF && (buf[1] == 0xF1 || buf[1] == 0xF9)
	}
}

impl VideoMatcher {
	pub fn detect(buf: &[u8]) -> Option<ContainerId> {
		use crate::container::{AVI, FLV, M4V, MKV, MOV, MP4, MPEG_PS, WEBM};

		if Self::mp4(buf) {
			return Some(MP4);
		}
		if Self::mkv(buf) {
			return Some(MKV);
		}
		if Self::webm(buf) {
			return Some(WEBM);
		}
		if Self::m4v(buf) {
			return Some(M4V);
		}
		if Self::mov(buf) {
			return Some(MOV);
		}
		if Self::avi(buf) {
			return Some(AVI);
		}
		if Self::flv(buf) {
			return Some(FLV);
		}
		if Self::mpeg(buf) {
			return Some(MPEG_PS);
		}

		None
	}

	pub fn mp4(buf: &[u8]) -> bool {
		if buf.len() <= 11 || !Self::is_ftyp(buf) {
			return false;
		}

		let major = &buf[8..12];
		matches!(
			major,
			b"avc1"
				| b"dash"
				| b"iso2"
				| b"iso3"
				| b"iso4"
				| b"iso5"
				| b"iso6"
				| b"isom"
				| b"mmp4"
				| b"mp41"
				| b"mp42"
				| b"mp4v"
				| b"mp71"
				| b"MSNV"
				| b"NDAS"
				| b"NDSC"
				| b"NSDC"
				| b"NDSH"
				| b"NDSM"
				| b"NDSP"
				| b"NDSS"
				| b"NDXC"
				| b"NDXH"
				| b"NDXM"
				| b"NDXP"
				| b"NDXS"
				| b"F4V "
				| b"F4P "
		)
	}

	pub fn mkv(buf: &[u8]) -> bool {
		let ebml_early = buf.len() > 15
			&& buf[0] == 0x1A
			&& buf[1] == 0x45
			&& buf[2] == 0xDF
			&& buf[3] == 0xA3
			&& buf[4..=15] == [0x93, 0x42, 0x82, 0x88, 0x6D, 0x61, 0x74, 0x72, 0x6F, 0x73, 0x6B, 0x61];

		if ebml_early {
			return true;
		}

		buf.len() > 38 && buf[31..=38] == [0x6D, 0x61, 0x74, 0x72, 0x6f, 0x73, 0x6B, 0x61]
	}

	pub fn webm(buf: &[u8]) -> bool {
		buf.len() > 3 && buf[0] == 0x1A && buf[1] == 0x45 && buf[2] == 0xDF && buf[3] == 0xA3
	}

	pub fn m4v(buf: &[u8]) -> bool {
		buf.len() > 10
			&& buf[4] == 0x66
			&& buf[5] == 0x74
			&& buf[6] == 0x79
			&& buf[7] == 0x70
			&& buf[8] == 0x4D
			&& buf[9] == 0x34
			&& buf[10] == 0x56
	}

	pub fn mov(buf: &[u8]) -> bool {
		if buf.len() <= 15 {
			return false;
		}

		let ftyp_qt = buf[4] == b'f'
			&& buf[5] == b't'
			&& buf[6] == b'y'
			&& buf[7] == b'p'
			&& buf[8] == b'q'
			&& buf[9] == b't'
			&& buf[10] == b' '
			&& buf[11] == b' ';

		if ftyp_qt {
			return true;
		}

		let mdat = buf[4] == 0x6d && buf[5] == 0x64 && buf[6] == 0x61 && buf[7] == 0x74;
		let moov = buf[4] == 0x6d && buf[5] == 0x6f && buf[6] == 0x6f && buf[7] == 0x76;
		let mdat_at_12 = buf[12] == 0x6d && buf[13] == 0x64 && buf[14] == 0x61 && buf[15] == 0x74;

		mdat || moov || mdat_at_12
	}

	pub fn avi(buf: &[u8]) -> bool {
		buf.len() > 10 && buf[0..4] == [0x52, 0x49, 0x46, 0x46] && buf[8..11] == [0x41, 0x56, 0x49]
	}

	pub fn flv(buf: &[u8]) -> bool {
		buf.len() > 3 && buf[0] == 0x46 && buf[1] == 0x4C && buf[2] == 0x56 && buf[3] == 0x01
	}

	pub fn mpeg(buf: &[u8]) -> bool {
		buf.len() > 3
			&& buf[0] == 0x00
			&& buf[1] == 0x00
			&& buf[2] == 0x01
			&& buf[3] >= 0xb0
			&& buf[3] <= 0xbf
	}

	fn is_ftyp(buf: &[u8]) -> bool {
		buf.len() >= 8 && buf[4] == b'f' && buf[5] == b't' && buf[6] == b'y' && buf[7] == b'p'
	}
}

impl ImageMatcher {
	pub fn detect(buf: &[u8]) -> Option<ContainerId> {
		use crate::container::{AVIF, BMP, GIF, HEIF, ICO, JPEG, PNG, TIFF, WEBP};

		if Self::png(buf) {
			return Some(PNG);
		}
		if Self::jpeg(buf) {
			return Some(JPEG);
		}
		if Self::gif(buf) {
			return Some(GIF);
		}
		if Self::bmp(buf) {
			return Some(BMP);
		}
		if Self::webp(buf) {
			return Some(WEBP);
		}
		if Self::tiff(buf) {
			return Some(TIFF);
		}
		if Self::ico(buf) {
			return Some(ICO);
		}
		if Self::heif(buf) {
			return Some(HEIF);
		}
		if Self::avif(buf) {
			return Some(AVIF);
		}

		None
	}

	pub fn png(buf: &[u8]) -> bool {
		buf.len() > 3 && buf[0..4] == [0x89, 0x50, 0x4E, 0x47]
	}

	pub fn jpeg(buf: &[u8]) -> bool {
		buf.len() > 2 && buf[0] == 0xFF && buf[1] == 0xD8 && buf[2] == 0xFF
	}

	pub fn gif(buf: &[u8]) -> bool {
		buf.len() > 2 && buf[0..3] == [0x47, 0x49, 0x46]
	}

	pub fn bmp(buf: &[u8]) -> bool {
		buf.len() > 1 && buf[0] == 0x42 && buf[1] == 0x4D
	}

	pub fn webp(buf: &[u8]) -> bool {
		buf.len() > 11
			&& buf[0..4] == [0x52, 0x49, 0x46, 0x46]
			&& buf[8..12] == [0x57, 0x45, 0x42, 0x50]
	}

	pub fn tiff(buf: &[u8]) -> bool {
		if buf.len() <= 3 {
			return false;
		}

		let little = buf[0..4] == [0x49, 0x49, 0x2A, 0x00];
		let big = buf[0..4] == [0x4D, 0x4D, 0x00, 0x2A];

		little || big
	}

	pub fn ico(buf: &[u8]) -> bool {
		buf.len() > 3 && buf[0..4] == [0x00, 0x00, 0x01, 0x00]
	}

	pub fn heif(buf: &[u8]) -> bool {
		if buf.len() < 16 || !Self::is_isobmff(buf) {
			return false;
		}

		let major = &buf[8..12];
		major == b"heic" || major == b"heix"
	}

	pub fn avif(buf: &[u8]) -> bool {
		if buf.len() < 16 || !Self::is_isobmff(buf) {
			return false;
		}

		let major = &buf[8..12];
		major == b"avif" || major == b"avis"
	}

	fn is_isobmff(buf: &[u8]) -> bool {
		buf.len() >= 16 && buf[4..8] == *b"ftyp"
	}
}
