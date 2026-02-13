use std::path::Path;
pub mod color;
pub mod kv;
use crate::{error, message};

pub fn extension(path: &str) -> message::Result<String> {
	std::path::Path::new(path)
		.extension()
		.and_then(|e| e.to_str())
		.map(|s| s.to_lowercase())
		.ok_or_else(|| error!("no file extension"))
}

pub fn extension_from_path(path: &Path) -> message::Result<String> {
	path
		.extension()
		.and_then(|e| e.to_str())
		.map(|s| s.to_lowercase())
		.ok_or_else(|| error!("no file extension"))
}

pub fn filename(path: &str) -> message::Result<String> {
	std::path::Path::new(path)
		.file_name()
		.and_then(|e| e.to_str())
		.map(|s| s.to_lowercase())
		.ok_or_else(|| error!("no file name"))
}

pub fn filename_from_path(path: &Path) -> message::Result<String> {
	path
		.file_name()
		.and_then(|e| e.to_str())
		.map(|s| s.to_lowercase())
		.ok_or_else(|| error!("no file name"))
}
