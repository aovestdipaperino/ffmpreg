use crate::message::{Message, Result};
use std::iter::Peekable;

pub struct ArgIter<I: Iterator<Item = String>> {
	iter: Peekable<I>,
}

impl<I: Iterator<Item = String>> ArgIter<I> {
	pub fn new(iter: I) -> Self {
		Self { iter: iter.peekable() }
	}

	pub fn next(&mut self) -> Option<String> {
		self.iter.next()
	}

	pub fn peek(&mut self) -> Option<&str> {
		self.iter.peek().map(String::as_str)
	}

	pub fn next_required(&mut self, msg: &str) -> Result<String> {
		self.next().ok_or(Message::error(msg))
	}

	pub fn read_values(&mut self) -> String {
		let mut out = String::new();

		while let Some(value) = self.peek() {
			if value.starts_with('-') {
				break;
			}
			out.push_str(&self.next().unwrap());
			out.push(' ');
		}

		out.trim().to_string()
	}
}
