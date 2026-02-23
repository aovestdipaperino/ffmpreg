use super::args::*;
use crate::{cli::iter, error, message::*};
use std::path::PathBuf;

impl Cli {
	pub fn parse() -> Result<Cli> {
		let mut iterator = iter::Iter::new();
		let command = match iterator.peek() {
			Some("probe") => parse_probe_subcommand(&mut iterator)?,
			Some("play") => parse_play_subcommand(&mut iterator)?,
			_ => parse_default_subcommand(&mut iterator)?,
		};
		Ok(Cli { command })
	}
}

fn parse_probe_subcommand(iterator: &mut iter::Iter) -> Result<Commands> {
	iterator.expect("probe")?;
	let raw = parse_common(iterator, true)?;
	let input = raw.input.ok_or_else(|| error!("missing required input path (-i)"))?;
	let args = ProbeArgs {
		input,
		output: raw.output,
		audio: raw.audio,
		video: raw.video,
		subtitle: raw.subtitle,
		apply: raw.apply,
	};
	Ok(Commands::Probe(args))
}

fn parse_play_subcommand(iterator: &mut iter::Iter) -> Result<Commands> {
	iterator.expect("play")?;
	let raw = parse_common(iterator, false)?;
	let input = raw.input.ok_or_else(|| error!("missing required input path (-i)"))?;
	let args = PlayArgs {
		input,
		audio: raw.audio,
		video: raw.video,
		subtitle: raw.subtitle,
		apply: raw.apply,
	};
	Ok(Commands::Play(args))
}

fn parse_default_subcommand(iterator: &mut iter::Iter) -> Result<Commands> {
	let raw = parse_common(iterator, true)?;
	let input = raw.input.ok_or_else(|| error!("missing required input path (-i)"))?;
	let output = raw.output.ok_or_else(|| error!("missing required output path (-o)"))?;
	let args = RunArgs {
		input,
		output,
		audio: raw.audio,
		video: raw.video,
		subtitle: raw.subtitle,
		apply: raw.apply,
	};
	Ok(Commands::Run(args))
}

fn parse_path(iterator: &mut iter::Iter, required_exists: bool) -> Result<PathBuf> {
	let path = iterator.next();
	let path = path.ok_or_else(|| error!("expected path, found nothing"))?;
	let path = PathBuf::from(path);
	if required_exists && !path.exists() {
		return Err(error!("{} does not exist", path.display()));
	}
	Ok(path)
}

struct RawArgs {
	input: Option<PathBuf>,
	output: Option<PathBuf>,
	audio: Vec<String>,
	video: Vec<String>,
	subtitle: Vec<String>,
	apply: Vec<String>,
}

fn parse_common(iterator: &mut iter::Iter, accepts_output: bool) -> Result<RawArgs> {
	let mut raw = RawArgs {
		input: None,
		output: None,
		audio: Vec::new(),
		video: Vec::new(),
		subtitle: Vec::new(),
		apply: Vec::new(),
	};

	loop {
		let arg = match iterator.peek() {
			Some(a) => a.to_string(),
			None => break,
		};
		match arg.as_str() {
			"-i" => {
				iterator.next();
				raw.input = Some(parse_path(iterator, true)?);
			}
			"-o" if accepts_output => {
				iterator.next();
				raw.output = Some(parse_path(iterator, false)?);
			}
			"--audio" => {
				iterator.expect("--audio")?;
				raw.audio.push(iterator.take_until_flag()?);
			}
			"--video" => {
				iterator.expect("--video")?;
				raw.video.push(iterator.take_until_flag()?);
			}
			"--subtitle" => {
				iterator.expect("--subtitle")?;
				raw.subtitle.push(iterator.take_until_flag()?);
			}
			"--apply" => {
				iterator.expect("--apply")?;
				raw.apply.push(iterator.take_until_flag()?);
			}
			_ => {
				return Err(error!("unexpected argument '{}'", arg));
			}
		}
	}

	Ok(raw)
}
