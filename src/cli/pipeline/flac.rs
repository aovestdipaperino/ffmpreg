use super::common::Pipeline;
use crate::cli::transcoder::media;
use crate::cli::utils;
use crate::codecs::audio::pcm::{PcmDecoder, PcmEncoder};
use crate::container::{self, flac, raw, wav};
use crate::core::{Demuxer, Muxer};
use crate::io::{Error, File};
use crate::{error, message::Result};

pub fn run(pipeline: Pipeline) -> Result<()> {
	let mut input_file = File::open(&pipeline.input)?;
	let input_extension = utils::get_extension(&pipeline.input)?;

	let mut format = flac::FlacFormat::default();

	// Extract format from input if WAV
	if input_extension == container::WAV {
		let file = File::open(&pipeline.input)?;
		let demuxer = wav::WavDemuxer::new(file)?;
		format = flac::FlacFormat {
			channels: demuxer.format().channels,
			sample_rate: demuxer.format().sample_rate,
			bit_depth: demuxer.format().bit_depth,
		};
	}

	let mut target_format = format;
	if let Some(codec) = &pipeline.audio.codec {
		target_format.apply_codec(codec).map_err(Error::invalid_data)?;
	}

	let output_file = File::create(&pipeline.output)?;
	let mut muxer = flac::FlacMuxer::new(output_file, target_format)?;
	muxer.with_metadata(None);

	let mut demuxer = create_demuxer(&pipeline.input, &input_extension, format)?;
	let mut transcoder = create_transcoder(format, target_format);

	while let Some(packet) = demuxer.read_packet()? {
		for output_packet in transcoder.transcode(packet)? {
			muxer.write(output_packet)?;
		}
	}

	for packet in transcoder.flush()? {
		muxer.write(packet)?;
	}

	muxer.finalize()
}

fn create_demuxer(
	path: &str,
	extension: &str,
	format: flac::FlacFormat,
) -> Result<Box<dyn Demuxer>> {
	let file = File::open(path)?;
	match extension {
		container::WAV => Ok(Box::new(wav::WavDemuxer::new(file)?)),
		container::RAW => {
			let demuxer = raw::RawPcmDemuxer::new(file, format.to_raw_format())?;
			Ok(Box::new(demuxer))
		}
		_ => Err(error!("unsupported input '{}' for flac output", extension)),
	}
}

fn create_transcoder(
	format: flac::FlacFormat,
	target_format: flac::FlacFormat,
) -> media::Transcoder {
	let wav_format = wav::WavFormat {
		channels: format.channels,
		sample_rate: format.sample_rate,
		bit_depth: format.bit_depth,
		format_code: 1,
	};
	let decoder = PcmDecoder::new_from_metadata(&wav_format);

	if format.audio_format() != target_format.audio_format() {
		let encoder = PcmEncoder::new(target_format.sample_rate);
		let encoder = encoder.with_target_format(target_format.audio_format());
		return media::Transcoder::new(Box::new(decoder), Box::new(encoder));
	}

	let encoder = PcmEncoder::new(target_format.sample_rate);
	media::Transcoder::new(Box::new(decoder), Box::new(encoder))
}
