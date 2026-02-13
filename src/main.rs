use osaka::{cli, message::*};

fn main() {
	let args = cli::parse_cli().report();
	cli::runner(args).report();
}
