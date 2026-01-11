use crate::cli::{config, pipeline, utils};
use crate::container;
use crate::core::compatible;
use crate::{cli, error, message};

pub fn execute(cli: cli::Cli) -> message::Result<()> {
	let mut pipe = pipeline::Pipeline::new(&cli.input, &cli.output);

	let audio = config::parse_audio(cli.audio)?;
	let video = config::parse_video(cli.video)?;
	let subtitle = config::parse_subtitle(cli.subtitle)?;
	let transform = config::parse_transform(cli.apply)?;
	pipe.with_transform(transform);

	let input_ext = utils::get_extension(&cli.input)?;
	let output_ext = utils::get_extension(&cli.output)?;

	let compat = compatible::Compatible::new();
	compat.assert_container_supported(&input_ext)?;
	compat.assert_container_supported(&output_ext)?;

	if let Some(codec) = &audio.codec {
		compat.assert_audio_supported(&input_ext, codec)?;
		pipe.with_audio(audio);
	}

	if let Some(codec) = &video.codec {
		compat.assert_video_supported(&input_ext, codec)?;
		pipe.with_video(video);
	}

	if let Some(codec) = &subtitle.codec {
		compat.assert_subtitle_supported(&input_ext, codec)?;
		pipe.with_subtitle(subtitle);
	}

	match output_ext.as_str() {
		// audio
		container::MP3 => pipeline::mp3::run(pipe),
		container::WAV => pipeline::wav::run(pipe),
		container::AAC => pipeline::aac::run(pipe),
		container::OPUS => pipeline::opus::run(pipe),
		container::FLAC => pipeline::flac::run(pipe),
		container::ALAC => pipeline::alac::run(pipe),
		container::M4A => pipeline::m4a::run(pipe),
		container::OGG => pipeline::ogg::run(pipe),

		// video
		container::MP4 => pipeline::mp4::run(pipe),
		container::AVI => pipeline::avi::run(pipe),
		container::MOV => pipeline::mov::run(pipe),
		container::WEBM => pipeline::webm::run(pipe),
		container::MKV => pipeline::mkv::run(pipe),
		container::OGV => pipeline::ogv::run(pipe),
		container::FLV => pipeline::flv::run(pipe),
		container::MXF => pipeline::mxf::run(pipe),
		container::TS => pipeline::ts::run(pipe),

		// images
		container::JPEG | container::JPG => pipeline::jpeg::run(pipe),
		container::PNG => pipeline::png::run(pipe),
		container::BMP => pipeline::bmp::run(pipe),
		container::WEBP => pipeline::webp::run(pipe),
		container::TIFF => pipeline::tiff::run(pipe),
		_ => Err(error!("unsupported format '{}'", output_ext)),
	}
}
