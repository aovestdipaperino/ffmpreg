use std::fmt;

use crate::{EXIT_FAILURE, EXIT_SUCCESS};

pub const RED: &str = "\x1b[1;31m";
pub const YELLOW: &str = "\x1b[1;33m";
pub const BLUE: &str = "\x1b[1;34m";
pub const RESET: &str = "\x1b[0m";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageKind {
	Error,
	Warning,
	Info,
}

impl MessageKind {
	pub fn name(self) -> &'static str {
		match self {
			MessageKind::Error => "error",
			MessageKind::Warning => "warning",
			MessageKind::Info => "info",
		}
	}
}

impl fmt::Display for MessageKind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.name())
	}
}

impl From<std::io::Error> for Message {
	fn from(err: std::io::Error) -> Self {
		Message::error(err.to_string())
	}
}

#[derive(Debug)]
pub struct Message {
	pub kind: MessageKind,
	pub text: String,
	pub note: Option<String>,
}

impl Message {
	pub fn error(text: impl Into<String>) -> Self {
		Self { kind: MessageKind::Error, text: text.into(), note: None }
	}

	pub fn warning(text: impl Into<String>) -> Self {
		Self { kind: MessageKind::Warning, text: text.into(), note: None }
	}

	pub fn info(text: impl Into<String>) -> Self {
		Self { kind: MessageKind::Info, text: text.into(), note: None }
	}

	pub fn with_note(mut self, note: impl Into<String>) -> Self {
		self.note = Some(note.into());
		self
	}

	pub fn render(&self) {
		match self.kind {
			MessageKind::Error => println!("{}{}:{} {}", RED, self.kind, RESET, self.text),
			MessageKind::Warning => println!("{}{}:{} {}", YELLOW, self.kind, RESET, self.text),
			MessageKind::Info => println!("{}{}:{} {}", BLUE, self.kind, RESET, self.text),
		}
	}

	pub fn render_and_exit(&self) -> ! {
		self.render();
		if self.kind == MessageKind::Error {
			std::process::exit(EXIT_FAILURE);
		}
		std::process::exit(EXIT_SUCCESS);
	}
}

pub type Result<T> = std::result::Result<T, Message>;

#[macro_export]
macro_rules! error {
  ($($arg:tt)*) => {
    $crate::message::Message::error(format!( $($arg)* ))
  }
}

#[macro_export]
macro_rules! warning {
  ($($arg:tt)*) => {
    $crate::message::Message::warning(format!( $($arg)* ))
  }
}

#[macro_export]
macro_rules! info {
  ($($arg:tt)*) => {
    $crate::message::Message::info(format!( $($arg)* ))
  }
}
