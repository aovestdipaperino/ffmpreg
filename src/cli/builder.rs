use super::flag::Flag;
use super::iter::ArgIter;
use crate::cli::*;
use crate::error;
use crate::message::Result;
use std::path::PathBuf;

pub struct Builder {
	input: Option<PathBuf>,
	output: Option<PathBuf>,
	base: BaseOptions,
	json: Option<JsonOption>,
}

impl Builder {
	pub fn new() -> Self {
		Self { input: None, output: None, base: BaseOptions::default(), json: None }
	}

	pub fn apply<I: Iterator<Item = String>>(
		&mut self,
		flag: Flag,
		iter: &mut ArgIter<I>,
		json_allowed: bool,
	) -> Result<()> {
		match flag {
			Flag::Input => self.input = Some(iter.next_required("missing input")?.into()),
			Flag::Output => self.output = Some(iter.next_required("missing output")?.into()),
			Flag::Audio => self.base.audio.push(iter.read_values()),
			Flag::Video => self.base.video.push(iter.read_values()),
			Flag::Subtitle => self.base.subtitle.push(iter.read_values()),
			Flag::Apply => self.base.apply.push(iter.read_values()),
			Flag::Json if json_allowed => {
				let value = iter.next_required("missing json")?;
				let json_value = match value.as_str() {
					"pretty" => JsonOption::Pretty,
					_ => JsonOption::Raw,
				};
				self.json = Some(json_value);
			}
			Flag::Json => return Err(error!("json not allowed")),
		}

		Ok(())
	}

	pub fn build(self) -> Result<(PathBuf, Option<PathBuf>, BaseOptions, Option<JsonOption>)> {
		Ok((self.input.ok_or(error!("-i required"))?, self.output, self.base, self.json))
	}
}
