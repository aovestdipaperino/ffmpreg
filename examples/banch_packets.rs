#![allow(dead_code, unused_imports)]
use osaka::Result;
use osaka::io::Input;
use osaka::message::Report;
use std::time::Instant;

fn format_human(n: f64) -> String {
	if n >= 1_000_000.0 {
		format!("{:.1}M", n / 1_000_000.0)
	} else if n >= 1_000.0 {
		format!("{:.1}k", n / 1_000.0)
	} else {
		format!("{:.0}", n)
	}
}

fn main() -> Result<()> {
	let start = Instant::now();
	let mut input = Input::open("./playground/sparkle.wav")?;
	let mut packets = 0;
	for packet in input.iter_packets() {
		packet.report();
		packets += 1;
	}
	let duration = start.elapsed().as_secs_f64();
	let pps = packets as f64 / duration;

	println!("{} packets/s.", format_human(pps),);
	Ok(())
}
