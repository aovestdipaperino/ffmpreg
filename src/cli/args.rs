use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Cli {
	Command(Command),
	Convert(ConvertOptions),
}

#[derive(Debug, Clone)]
pub enum Command {
	Probe(ProbeCommand),
	Play(PlayCommand),
}

#[derive(Debug, Clone)]
pub struct PlayCommand {
	pub input: PathBuf,
	pub output: Option<PathBuf>,
	pub base: BaseOptions,
}

#[derive(Debug, Clone)]
pub struct ProbeCommand {
	pub input: PathBuf,
	pub output: Option<PathBuf>,
	pub json: Option<JsonOption>,

	pub base: BaseOptions,
}

#[derive(Clone, Debug, Default)]
pub enum JsonOption {
	Pretty,
	#[default]
	Raw,
}

#[derive(Debug, Clone)]
pub struct ConvertOptions {
	pub input: PathBuf,
	pub output: PathBuf,

	pub base: BaseOptions,
}

#[derive(Debug, Clone, Default)]
pub struct BaseOptions {
	pub audio: Vec<String>,
	pub video: Vec<String>,
	pub subtitle: Vec<String>,
	pub apply: Vec<String>,
}
