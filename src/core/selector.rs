use crate::{error, message, utils::kv::Kv};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Selector {
	#[default]
	All,
	Id(usize),
}

impl Selector {
	pub fn matches(&self, id: usize) -> bool {
		match self {
			Selector::All => true,
			Selector::Id(selector) => id == *selector,
		}
	}

	pub fn from_kv(kv: &Kv) -> message::Result<Option<Self>> {
		let selector = match kv.get("track") {
			None => return Ok(None),
			Some(track) => match track.as_str() {
				"all" | "*" => Some(Selector::All),
				track => {
					let value = track.parse().map_err(|_| error!("unable to parse track '{}'", track))?;
					Some(Selector::Id(value))
				}
			},
		};
		Ok(selector)
	}
}

impl From<usize> for Selector {
	fn from(id: usize) -> Self {
		Selector::Id(id)
	}
}

impl From<Selector> for Vec<usize> {
	fn from(selector: Selector) -> Self {
		match selector {
			Selector::All => Vec::new(),
			Selector::Id(id) => vec![id],
		}
	}
}

impl Display for Selector {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Selector::All => write!(f, "all"),
			Selector::Id(id) => write!(f, "{}", id),
		}
	}
}
