use crate::error;
use crate::message::*;
use std::iter::Peekable;

pub struct Iter {
	args: Peekable<std::env::Args>,
}

impl Iter {
	pub fn new() -> Self {
		let mut iterator = std::env::args().peekable();
		iterator.next(); // skip bin
		Iter { args: iterator }
	}

	pub fn next(&mut self) -> Option<String> {
		self.args.next().map(|s| s.trim().to_string())
	}

	pub fn peek(&mut self) -> Option<&str> {
		self.args.peek().map(|s| s.trim())
	}

	pub fn expect(&mut self, expected: &str) -> Result<()> {
		match self.next() {
			Some(arg) if arg == expected => Ok(()),
			Some(arg) => Err(error!("expected '{}', found '{}'", expected, arg)),
			None => Err(error!("expected '{}', found nothing", expected)),
		}
	}

	pub fn take_until_flag(&mut self) -> Result<String> {
		let mut args = String::new();
		loop {
			let is_flag = self.peek().map_or(true, |p| p.starts_with('-'));
			if is_flag {
				break;
			}
			let value = self.next().unwrap();
			args = format!("{} {}", args, value).trim().to_string();
		}
		Ok(args)
	}
}
