use crate::cli;
use crate::core::graph::nodes::{DecoderNode, EncoderNode, ResamplerNode};
use crate::core::graph::Graph;
use crate::core::resampler::AudioResampler;
use crate::core::resolver::{CodecResolver, ContainerResolver};
use crate::core::track::TrackFormat;
use crate::core::traits::Resampler;
use crate::io::{Input, Output};
use crate::message::Result;

pub fn runner(options: &cli::RunArgs) -> Result<()> {
	let containers = ContainerResolver::new();
	let codecs = CodecResolver::new();

	let mut input = Input::from_resolver(&options.input, &containers)?;
	let mut builder = Output::builder(&options.output, &containers)?;

	let stream_options = options.stream_options()?;
	let mut graph = Graph::new();

	for option in &stream_options {
		if let Some(codec) = option.codec.as_ref() {
			builder.format_mut().apply_codec(codec)?;
		}

		for track in input.tracks.audio_selector(&option.selector)? {
			let decoder = codecs.decoder_for(track)?;
			let encoder = codecs.encoder_for(track, builder.format_mut())?;

			let decoder_id = graph.add(DecoderNode::new(decoder));

			let resampler_node = build_resampler_node(track, &*encoder);
			let after_decode = if let Some(resampler) = resampler_node {
				let resample_id = graph.add(resampler);
				graph.link(decoder_id, resample_id);
				resample_id
			} else {
				decoder_id
			};

			let encoder_id = graph.add(EncoderNode::new(encoder));
			graph.link(after_decode, encoder_id);

			graph.set_entry(track.id, decoder_id);
		}
	}

	let mut output = builder.build()?;

	while let Some(packet) = input.read_packet()? {
		let packets = graph.process(packet)?;
		output.write_all(packets)?;
	}

	let flushed = graph.flush()?;
	for packet in flushed {
		output.write_packet(packet)?;
	}

	output.finalize()
}

fn build_resampler_node(
	track: &crate::core::Track,
	encoder: &dyn crate::core::traits::Encoder,
) -> Option<ResamplerNode> {
	let input_format = track.audio_format()?;
	let output_format = match encoder.input_format() {
		TrackFormat::Audio(fmt) => fmt,
		_ => return None,
	};
	let resampler = AudioResampler::new(*input_format, output_format);
	if resampler.needed() {
		Some(ResamplerNode::new(Box::new(resampler)))
	} else {
		None
	}
}
