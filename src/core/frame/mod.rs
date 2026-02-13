use crate::core::time::Timestamp;

pub mod audio;
pub mod iter;
pub mod subtitle;
pub mod video;
pub use audio::*;
pub use iter::*;
pub use subtitle::*;
pub use video::*;

#[derive(Debug, Clone)]
pub enum FrameData {
	Audio(FrameAudio),
	Video(FrameVideo),
}

#[derive(Debug, Clone)]
pub struct Frame {
	pub track_id: usize,
	pub data: FrameData,
}

impl Frame {
	pub fn new_audio(audio: FrameAudio, track_id: usize) -> Self {
		Self { track_id, data: FrameData::Audio(audio) }
	}

	pub fn new_video(video: FrameVideo, track_id: usize) -> Self {
		Self { track_id, data: FrameData::Video(video) }
	}

	#[inline(always)]
	pub fn size(&self) -> usize {
		match &self.data {
			FrameData::Audio(audio) => audio.data.len(),
			FrameData::Video(video) => video.size(),
		}
	}

	#[inline(always)]
	pub fn is_empty(&self) -> bool {
		self.size() == 0
	}

	#[inline(always)]
	pub fn pts(&self) -> Timestamp {
		match &self.data {
			FrameData::Audio(audio) => audio.pts,
			FrameData::Video(video) => video.pts,
		}
	}

	#[inline(always)]
	pub fn audio(&self) -> Option<&FrameAudio> {
		match &self.data {
			FrameData::Audio(audio) => Some(audio),
			_ => None,
		}
	}

	#[inline(always)]
	pub fn audio_mut(&mut self) -> Option<&mut FrameAudio> {
		match &mut self.data {
			FrameData::Audio(audio) => Some(audio),
			_ => None,
		}
	}

	#[inline(always)]
	pub fn video(&self) -> Option<&FrameVideo> {
		match &self.data {
			FrameData::Video(video) => Some(video),
			_ => None,
		}
	}

	#[inline(always)]
	pub fn into_audio(self) -> Option<FrameAudio> {
		match self.data {
			FrameData::Audio(audio) => Some(audio),
			_ => None,
		}
	}
}
