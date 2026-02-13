use std::fmt::{self, Display};

use crate::{EXIT_FAILURE, EXIT_SUCCESS, utils::color::*};
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
impl From<String> for Message {
	fn from(message: String) -> Self {
		Message::error(message)
	}
}

impl From<Message> for std::io::Error {
	fn from(message: Message) -> Self {
		std::io::Error::other(message.text)
	}
}

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

	fn render_value(&self) -> String {
		let name = self.kind.name();
		match self.kind {
			MessageKind::Error => format!("{}: {}", red.text(name), self.text),
			MessageKind::Warning => format!("{}: {}", yellow.text(name), self.text),
			MessageKind::Info => format!("{}: {}", blue.text(name), self.text),
		}
	}

	pub fn render(&self) {
		let name = self.kind.name();
		match self.kind {
			MessageKind::Error => println!("{}: {}", red.text(name), self.text),
			MessageKind::Warning => println!("{}: {}", yellow.text(name), self.text),
			MessageKind::Info => println!("{}: {}", blue.text(name), self.text),
		}
	}

	pub fn report(&self) -> ! {
		self.render();
		if self.kind == MessageKind::Error {
			std::process::exit(EXIT_FAILURE);
		}
		std::process::exit(EXIT_SUCCESS);
	}
}

impl Display for Message {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.render_value())
	}
}
impl fmt::Debug for Message {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.text)
	}
}
#[derive(Debug)]
pub struct Messages {
	pub list: Vec<Message>,
}

impl Messages {
	pub fn new() -> Self {
		Self { list: vec![] }
	}

	pub fn message(&mut self, message: Message) {
		self.list.push(message);
	}

	pub fn has_errors(&self) -> bool {
		self.list.iter().any(|m| m.kind == MessageKind::Error)
	}
}

pub type Result<T> = std::result::Result<T, Message>;

pub trait Report<T> {
	fn report(self) -> T;
}

impl<T> Report<T> for Result<T> {
	fn report(self) -> T {
		match self {
			Ok(value) => value,
			Err(message) => message.report(),
		}
	}
}

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
