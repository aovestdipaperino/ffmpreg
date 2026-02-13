use super::{AudioFormat, TrackFormat};
use crate::core::time::Timestamp;
use crate::core::{CodecId, Selector};
use crate::error;
use crate::message::Result;

#[derive(Debug, Clone, Copy)]
pub struct Track {
	pub id: usize,
	pub codec_in: CodecId,
	pub codec_out: CodecId,
	pub timestamp: Timestamp,
	pub format: TrackFormat,
}

impl Track {
	pub fn add_codec_out(&mut self, codec_out: CodecId) {
		self.codec_out = codec_out
	}

	pub fn is_audio(&self) -> bool {
		matches!(self.format, TrackFormat::Audio(_))
	}

	pub fn is_video(&self) -> bool {
		matches!(self.format, TrackFormat::Video(_))
	}

	pub fn audio_format(&self) -> Option<&AudioFormat> {
		match &self.format {
			TrackFormat::Audio(format) => Some(format),
			_ => None,
		}
	}

	pub fn audio_format_mut(&mut self) -> Option<&mut AudioFormat> {
		match &mut self.format {
			TrackFormat::Audio(format) => Some(format),
			_ => None,
		}
	}
}

#[derive(Debug)]
pub struct Tracks {
	tracks: Vec<Track>,
}

impl Tracks {
	pub fn new(tracks: Vec<Track>) -> Self {
		Self { tracks }
	}

	pub fn empty() -> Self {
		Self { tracks: vec![] }
	}

	pub fn primary_audio(&self) -> Result<&Track> {
		let audio_tracks: Vec<&Track> = self.iter().filter(|track| track.is_audio()).collect();
		if audio_tracks.is_empty() {
			return Err(error!("no audio tracks found"));
		}
		Ok(audio_tracks[0])
	}

	pub fn by_id(&self, id: usize) -> Option<&Track> {
		self.iter().find(|track| track.id == id)
	}

	pub fn by_id_mut(&mut self, id: usize) -> Option<&mut Track> {
		self.iter_mut().find(|track| track.id == id)
	}
	pub fn audio_selector(&mut self, selector: &Selector) -> Result<Vec<&Track>> {
		if let Selector::Id(track_id) = selector {
			let track = self.by_id(*track_id).ok_or_else(|| error!("track={} not found", track_id))?;
			if track.is_audio() {
				return Ok(vec![track]);
			}
			return Err(error!("track={} is not audio", track_id));
		}

		let tracks = self.audio_tracks();

		if tracks.is_empty() { Err(error!("no audio tracks found")) } else { Ok(tracks) }
	}

	pub fn video_selector(&self, selector: &Selector) -> Result<Vec<&Track>> {
		if let Selector::Id(track_id) = selector {
			let track = self.by_id(*track_id).ok_or_else(|| error!("track={} not found", track_id))?;
			if track.is_video() {
				return Ok(vec![track]);
			}
			return Err(error!("track={} is not video", track_id));
		}

		let tracks = self.video_tracks();

		if tracks.is_empty() { Err(error!("no video tracks found")) } else { Ok(tracks) }
	}

	pub fn audio_tracks(&mut self) -> Vec<&Track> {
		self.iter().filter(|track| track.is_audio()).collect()
	}

	pub fn video_tracks(&self) -> Vec<&Track> {
		self.iter().filter(|track| track.is_video()).collect()
	}

	#[inline]
	pub fn len(&self) -> usize {
		self.tracks.len()
	}

	#[inline]
	pub fn is_empty(&self) -> bool {
		self.tracks.is_empty()
	}

	#[inline]
	pub fn iter(&self) -> impl Iterator<Item = &Track> {
		self.tracks.iter()
	}
	#[inline]
	pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Track> {
		self.tracks.iter_mut()
	}
}
