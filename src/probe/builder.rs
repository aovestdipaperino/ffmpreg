use crate::core::Track;
use crate::core::track::{AudioFormat, ColorSpace, TrackFormat, VideoFormat};
use crate::io::{Input, Output};
use crate::message::Result;
use crate::probe::model;
use crate::utils;

pub struct Builder {
	input: Input,
	output: Option<Output>,
}

impl Builder {
	pub fn new(input: Input, output: Option<Output>) -> Self {
		Self { input, output }
	}

	pub fn media_file(&self) -> Result<model::MediaFile> {
		let input_file = self.input_file()?;
		let outputs = self.output_files(&input_file.tracks)?;

		Ok(model::MediaFile::new(input_file, outputs))
	}

	fn input_file(&self) -> Result<model::InputFile> {
		let format = Some(utils::extension_from_path(&self.input.path)?);
		let size = std::fs::metadata(&self.input.path).ok().map(|meta| meta.len());
		let duration = self.input.duration().map(|seconds| seconds.round() as u64);
		let tracks = self.tracks_from_core(self.input.tracks.iter())?;
		let summary = self.summary_from_tracks(&tracks);
		let path = self.input.path.to_str().unwrap().to_string();
		Ok(model::InputFile { path, duration, size, summary, tracks, format })
	}

	fn output_files(&self, tracks: &[model::Track]) -> Result<Vec<model::OutputFile>> {
		let mut outputs = Vec::new();

		let Some(output) = &self.output else {
			return Ok(outputs);
		};

		let path = output.path.to_str().unwrap();
		outputs.push(model::OutputFile {
			path: path.to_string(),
			tracks: tracks.to_vec(),
			metrics: None,
		});

		Ok(outputs)
	}

	fn tracks_from_core<'a>(
		&self,
		tracks: impl Iterator<Item = &'a Track>,
	) -> Result<Vec<model::Track>> {
		let mut items = Vec::new();
		for track in tracks {
			let item = self.track_from_core(track)?;
			items.push(item);
		}
		Ok(items)
	}

	fn track_from_core(&self, track: &Track) -> Result<model::Track> {
		let id = track.id as u32;
		let codec = Some(track.codec_in.to_string());

		let (kind, video, audio) = match &track.format {
			TrackFormat::Audio(format) => {
				(model::TrackKind::Audio, None, Some(self.audio_params_from_format(format)))
			}
			TrackFormat::Video(format) => {
				(model::TrackKind::Video, Some(self.video_params_from_format(format)?), None)
			}
		};

		Ok(model::Track { id, kind, codec, duration: None, bitrate: None, video, audio, tags: None })
	}

	fn audio_params_from_format(&self, format: &AudioFormat) -> model::AudioParams {
		model::AudioParams {
			channel_layout: None,
			sample_rate: Some(format.sample_rate),
			channels: Some(format.channels),
			bit_depth: Some(format.bit_depth),
		}
	}

	fn video_params_from_format(&self, format: &VideoFormat) -> Result<model::VideoParams> {
		let color_space = match format.pixel.color_space {
			ColorSpace::YUV => "yuv",
		};

		let pixel_format = format!("{}-bit {}", format.pixel.depth, color_space);

		Ok(model::VideoParams {
			width: Some(format.width),
			height: Some(format.height),
			frame_rate: None,
			pixel_format: Some(pixel_format),
			color_space: Some(color_space.to_string()),
		})
	}

	fn summary_from_tracks(&self, tracks: &[model::Track]) -> Option<model::Summary> {
		let mut summary = model::Summary {
			width: None,
			height: None,
			frame_rate: None,
			sample_rate: None,
			channels: None,
			bit_depth: None,
		};

		for track in tracks {
			if let Some(audio) = &track.audio {
				summary.sample_rate = audio.sample_rate;
				summary.channels = audio.channels;
				summary.bit_depth = audio.bit_depth;
				return Some(summary);
			}
		}

		for track in tracks {
			if let Some(video) = &track.video {
				summary.width = video.width;
				summary.height = video.height;
				return Some(summary);
			}
		}

		None
	}
}
