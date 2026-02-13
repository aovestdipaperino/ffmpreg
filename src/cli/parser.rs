use std::env;
use std::path::PathBuf;

use crate::cli::ArgIter;
use crate::cli::Builder;
use crate::cli::Flag;
use crate::cli::*;
use crate::error;
use crate::message::*;

pub fn parse_cli() -> Result<Cli> {
	let args = env::args();
	let mut iter = ArgIter::new(args.into_iter());
	iter.next();

	match iter.peek() {
		Some("play") => parse_play(&mut iter),
		Some("probe") => parse_probe(&mut iter),
		_ => parse_convert(&mut iter),
	}
}

pub fn parse_convert<I: Iterator<Item = String>>(iter: &mut ArgIter<I>) -> Result<Cli> {
	let (input, output, base, _) = parse_command_base(iter, false)?;
	let output = output.ok_or(error!("-o required"))?;
	Ok(Cli::Convert(ConvertOptions { input, output, base }))
}

pub fn parse_probe<I: Iterator<Item = String>>(iter: &mut ArgIter<I>) -> Result<Cli> {
	iter.next();

	let (input, output, base, json) = parse_command_base(iter, true)?;
	Ok(Cli::Command(Command::Probe(ProbeCommand { input, output, base, json })))
}

pub fn parse_play<I: Iterator<Item = String>>(iter: &mut ArgIter<I>) -> Result<Cli> {
	iter.next();

	let (input, output, base, _) = parse_command_base(iter, false)?;
	Ok(Cli::Command(Command::Play(PlayCommand { input, output, base })))
}

pub fn parse_command_base<I: Iterator<Item = String>>(
	iter: &mut ArgIter<I>,
	json_allowed: bool,
) -> Result<(PathBuf, Option<PathBuf>, BaseOptions, Option<JsonOption>)> {
	let mut builder = Builder::new();

	while let Some(token) = iter.next() {
		let flag = Flag::parse(&token).ok_or(error!("unknown '{}'", token))?;
		builder.apply(flag, iter, json_allowed)?;
	}

	builder.build()
}
