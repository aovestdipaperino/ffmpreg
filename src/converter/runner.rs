use crate::core::Transcoders;
use crate::core::resolver::*;
use crate::io::{Input, Output};
use crate::{cli, message};

pub fn runner(options: &cli::ConvertOptions) -> message::Result<()> {
	let formats = ContainerResolver::new();
	let codecs = CodecResolver::new();

	let mut input = Input::from_resolver(&options.input, &formats)?;
	let mut output = Output::from_resolver(&options.output, &formats)?;

	let mut transcoders = Transcoders::new(&codecs);

	let audio_options = options.base.audios()?;
	for audio in audio_options.iter() {
		transcoders.ensure(audio.selector, &mut input, output.format())?;
	}

	if audio_options.is_empty() {
		transcoders.ensure_default(&mut input, output.format())?;
	}

	while let Some(packet) = input.read_packet()? {
		transcoders.write_packet(packet, &mut output)?;
	}

	return transcoders.finish(&mut output);
}
