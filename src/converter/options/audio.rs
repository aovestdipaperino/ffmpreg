use crate::core::Selector;
use crate::message;
use crate::utils::kv::Kv;

#[derive(Debug, Clone)]
pub struct AudioOption {
	pub selector: Selector,
	pub codec: Option<String>,
	pub channels: Option<String>,
	pub sample_rate: Option<String>,
	pub volume: Option<String>,
}

impl TryFrom<&str> for AudioOption {
	type Error = message::Message;

	fn try_from(text: &str) -> message::Result<Self> {
		let kv = Kv::parse(text)?;
		let selector = Selector::from_kv(&kv)?.unwrap_or_default();
		Ok(AudioOption {
			selector,
			codec: kv.get("codec"),
			channels: kv.get("channels"),
			sample_rate: kv.get("rate"),
			volume: kv.get("volume"),
		})
	}
}
