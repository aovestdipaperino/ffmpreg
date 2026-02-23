use crate::core::resolver::{CodecResolver, ContainerResolver, TransformResolver};
use crate::core::transcoder::Router;
use crate::io::{Input, Output};
use crate::{cli, message::Result};

pub fn runner(options: &cli::ConvertOptions) -> Result<()> {
	let containers = ContainerResolver::new();
	let codecs = CodecResolver::new();
	let transforms = TransformResolver::new();

	let mut input = Input::from_resolver(&options.input, &containers)?;
	let mut builder = Output::builder(&options.output, &containers)?;

	let audio_options = options.base.audios_default()?;
	let video_options = options.base.videos_default()?;

	let transform_options = options.base.transforms()?;

	let mut router = Router::new();

	for option in &audio_options {
		if let Some(codec) = option.codec.as_ref() {
			builder.format_mut().apply_codec(codec)?;
		}

		for track in input.tracks.audio_selector(&option.selector)? {
			router.register(track, &codecs, builder.format_mut())?;
		}
	}

	for option in &transform_options {
		for (selector, transform) in transforms.from_transform(option)? {
			router.apply(selector, transform, &input.tracks)?;
		}
	}

	for option in &audio_options {
		for (selector, transform) in transforms.from_audio(option)? {
			router.apply(selector, transform, &input.tracks)?;
		}
	}

	let mut output = builder.build()?;

	router.run(&mut input, &mut output)
}
