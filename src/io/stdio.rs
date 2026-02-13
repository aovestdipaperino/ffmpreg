use crate::message::Result;
use std::io::{Read, Write};

pub struct StdinAdapter;

impl StdinAdapter {
	pub fn new() -> Self {
		Self
	}
}

impl crate::io::MediaRead for StdinAdapter {
	fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
		std::io::stdin().read(buf).map_err(|e| crate::message::Message::from(e))
	}
}

pub struct StdoutAdapter;

impl StdoutAdapter {
	pub fn new() -> Self {
		Self
	}
}

impl crate::io::MediaWrite for StdoutAdapter {
	fn write(&mut self, buf: &[u8]) -> Result<usize> {
		std::io::stdout().write(buf).map_err(|e| crate::message::Message::from(e))
	}

	fn flush(&mut self) -> Result<()> {
		std::io::stdout().flush().map_err(|e| crate::message::Message::from(e))
	}
}

pub enum StdioSource {
	Stdin(StdinAdapter),
	File(std::fs::File),
}

impl crate::io::MediaRead for StdioSource {
	fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
		match self {
			StdioSource::Stdin(stdin) => stdin.read(buf),
			StdioSource::File(file) => {
				use std::io::Read;
				file.read(buf).map_err(|e| crate::message::Message::from(e))
			}
		}
	}
}

pub enum StdioSink {
	Stdout(StdoutAdapter),
	File(std::fs::File),
}

impl crate::io::MediaWrite for StdioSink {
	fn write(&mut self, buf: &[u8]) -> Result<usize> {
		match self {
			StdioSink::Stdout(stdout) => stdout.write(buf),
			StdioSink::File(file) => {
				use std::io::Write;
				file.write(buf).map_err(|e| crate::message::Message::from(e))
			}
		}
	}

	fn flush(&mut self) -> Result<()> {
		match self {
			StdioSink::Stdout(stdout) => stdout.flush(),
			StdioSink::File(file) => {
				use std::io::Write;
				file.flush().map_err(|e| crate::message::Message::from(e))
			}
		}
	}
}
