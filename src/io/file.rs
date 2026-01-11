use super::{MediaRead, MediaSeek, MediaWrite, SeekFrom};
use crate::error;
use crate::message::Result;
use std::io::{Read, Seek, Write};

#[derive(Debug)]
pub struct File {
	file: std::fs::File,
	extension: Option<String>,
}
impl File {
	pub fn open(path: &str) -> Result<Self> {
		let file = mapper_error(std::fs::File::open(path), path)?;
		let temp_file = std::path::Path::new(path);
		if let Some(extension) = temp_file.extension() {
			let extension = extension.to_str().map(|s| s.to_owned());
			return Ok(Self { file, extension });
		}
		Ok(Self { file, extension: None })
	}

	pub fn create(path: &str) -> Result<Self> {
		let file = mapper_error(std::fs::File::create(path), path)?;
		let temp_file = std::path::Path::new(path);
		if let Some(extension) = temp_file.extension() {
			let extension = extension.to_str().map(|s| s.to_owned());
			return Ok(Self { file, extension });
		}
		Ok(Self { file, extension: None })
	}
}

impl MediaRead for File {
	fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
		self.file.read(buf).map_err(Into::into)
	}
	fn extension(&self) -> Option<String> {
		self.extension.clone()
	}
}

impl MediaWrite for File {
	fn write(&mut self, buf: &[u8]) -> Result<usize> {
		self.file.write(buf).map_err(Into::into)
	}

	fn flush(&mut self) -> Result<()> {
		self.file.flush().map_err(Into::into)
	}
}

impl MediaSeek for File {
	fn seek(&mut self, position: SeekFrom) -> Result<u64> {
		let std_position = match position {
			SeekFrom::Start(offset) => std::io::SeekFrom::Start(offset),
			SeekFrom::Current(offset) => std::io::SeekFrom::Current(offset),
			SeekFrom::End(offset) => std::io::SeekFrom::End(offset),
		};
		self.file.seek(std_position).map_err(Into::into)
	}
}

impl Read for File {
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
		self.file.read(buf)
	}
}

impl Seek for File {
	fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
		self.file.seek(pos)
	}
}

impl Write for File {
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
		self.file.write(buf)
	}

	fn flush(&mut self) -> std::io::Result<()> {
		self.file.flush()
	}
}

pub fn mapper_error<T>(result: std::io::Result<T>, path: &str) -> Result<T> {
	match result {
		Ok(file) => Ok(file),
		Err(error) => match error.kind() {
			std::io::ErrorKind::NotFound => Err(error!("'{}' not found", path)),
			std::io::ErrorKind::AlreadyExists => Err(error!("'{}' already exists", path)),
			std::io::ErrorKind::PermissionDenied => Err(error!("permission denied for '{}'", path)),
			std::io::ErrorKind::InvalidInput => Err(error!("invalid path for '{}'", path)),
			std::io::ErrorKind::InvalidData => Err(error!("invalid data for '{}'", path)),
			std::io::ErrorKind::IsADirectory => Err(error!("is a directory for '{}'", path)),
			std::io::ErrorKind::NotADirectory => Err(error!("not a directory for '{}'", path)),
			std::io::ErrorKind::DirectoryNotEmpty => Err(error!("directory not empty for '{}'", path)),
			std::io::ErrorKind::ReadOnlyFilesystem => Err(error!("read-only filesystem for '{}'", path)),
			std::io::ErrorKind::StorageFull => Err(error!("no space left on device for '{}'", path)),
			std::io::ErrorKind::QuotaExceeded => Err(error!("quota exceeded for '{}'", path)),
			std::io::ErrorKind::FileTooLarge => Err(error!("file too large for '{}'", path)),
			std::io::ErrorKind::TimedOut => Err(error!("filesystem timeout for '{}'", path)),
			std::io::ErrorKind::Interrupted => Err(error!("filesystem interrupted for '{}'", path)),
			_ => Err(error!("unknown error for '{}', {}", path, error)),
		},
	}
}
