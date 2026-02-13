use rustc_hash::FxHashMap;

use crate::{error, message};

pub struct Kv<'a> {
	map: FxHashMap<&'a str, &'a str>,
}

impl<'a> Kv<'a> {
	pub fn parse(line: &'a str) -> message::Result<Self> {
		let parts = line.split_whitespace();
		let mut map = FxHashMap::default();
		for part in parts {
			let mut it = part.splitn(2, '=');
			let key = it.next().unwrap();
			if key.is_empty() {
				return Err(error!("invalid key=value"));
			}
			let value = it.next().unwrap_or("true");
			map.insert(key, value);
		}

		Ok(Self { map })
	}

	pub fn get(&self, key: &str) -> Option<String> {
		self.map.get(key).map(|v| (*v).to_string())
	}
}
