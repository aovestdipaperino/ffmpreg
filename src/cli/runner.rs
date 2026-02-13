use crate::cli::args::Command;
use crate::cli::{Cli, args};
use crate::{converter, message, play, probe};

pub fn runner(cli: args::Cli) -> message::Result<()> {
	match cli {
		Cli::Command(command) => match command {
			Command::Probe(options) => probe::runner(&options),
			Command::Play(options) => play::runner(&options),
		},
		Cli::Convert(options) => converter::runner(&options),
	}
}
