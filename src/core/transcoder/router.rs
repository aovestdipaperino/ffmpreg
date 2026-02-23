use rustc_hash::FxHashMap;

use super::Transcoder;
use crate::core::resolver::CodecResolver;
use crate::core::track::{Format, Tracks};
use crate::core::traits::Transform;
use crate::core::{Selector, Track};
use crate::io::{Input, Output};
use crate::message::Result;

pub struct Router {
	streams: FxHashMap<usize, Transcoder>,
}

impl Router {
	pub fn new() -> Self {
		Self { streams: FxHashMap::default() }
	}

	pub fn register(&mut self, track: &Track, codecs: &CodecResolver, format: &Format) -> Result<()> {
		if self.streams.contains_key(&track.id) {
			return Ok(());
		}
		let transcoder = Transcoder::from_track(track, codecs, format)?;
		self.streams.insert(track.id, transcoder);
		Ok(())
	}

	pub fn apply(
		&mut self,
		selector: Selector,
		transform: Box<dyn Transform>,
		tracks: &Tracks,
	) -> Result<()> {
		for track in tracks.audio_selector(&selector)? {
			if let Some(transcoder) = self.streams.get_mut(&track.id) {
				transcoder.add_transform(selector, transform);
				break;
			}
		}
		Ok(())
	}

	pub fn run(&mut self, input: &mut Input, output: &mut Output) -> Result<()> {
		while let Some(in_packet) = input.read_packet()? {
			if let Some(transcoder) = self.streams.get_mut(&in_packet.track_id) {
				transcoder.transcode(in_packet, &mut |out_packet| output.write_packet(out_packet))?;
			}
		}

		for transcoder in self.streams.values_mut() {
			transcoder.flush(&mut |packet| output.write_packet(packet))?;
		}
		output.finalize()
	}
}
