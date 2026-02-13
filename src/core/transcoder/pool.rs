use rustc_hash::FxHashMap;

use super::Transcoder;
use crate::core::packet::Packet;
use crate::core::track::Format;
use crate::core::{Selector, resolver::CodecResolver};
use crate::io::{Input, Output};
use crate::message::Result;

pub struct Transcoders<'codec> {
	resolver: &'codec CodecResolver,
	streams: FxHashMap<usize, Transcoder>,
}

impl<'codec> Transcoders<'codec> {
	pub fn new(resolver: &'codec CodecResolver) -> Self {
		Self { resolver, streams: FxHashMap::default() }
	}

	#[rustfmt::skip]
	pub fn ensure(&mut self, selector: Selector, input: &mut Input, format: &Format) -> Result<()> {
		for track in input.tracks.audio_selector(&selector)? {
			if self.streams.contains_key(&track.id) {
				// ok?
				continue;
			}

			let decoder = self.resolver.decoder_for(track)?;
			let encoder = self.resolver.encoder_for(track, format)?;
			let transcoder = Transcoder { decoder, encoder };
			self.streams.insert(track.id, transcoder);
		}
		Ok(())
	}

	pub fn ensure_default(&mut self, input: &mut Input, format: &Format) -> Result<()> {
		self.ensure(Selector::All, input, format)
	}

	pub fn write_packet(&mut self, packet: Packet, output: &mut Output) -> Result<()> {
		if let Some(transcoder) = self.streams.get_mut(&packet.track_id) {
			for frame in transcoder.decoder.decode(packet)? {
				for packeted in transcoder.encoder.encode(frame)? {
					output.write_packet(packeted)?;
				}
			}
		}
		Ok(())
	}

	pub fn finish(&mut self, output: &mut Output) -> Result<()> {
		for transcoder in self.streams.values_mut() {
			for frame in transcoder.decoder.finish()? {
				for packeted in transcoder.encoder.encode(frame)? {
					output.write_packet(packeted)?;
				}
			}
			for packeted in transcoder.encoder.finish()? {
				output.write_packet(packeted)?;
			}
		}

		return output.flush();
	}
}
