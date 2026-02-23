use osaka::{cli, message::Report};

fn main() {
	let args = cli::Cli::parse().report();
	println!("{:?}", args);
	// cli::runner(args).report();
}
