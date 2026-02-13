use crate::core::Selector;
use crate::message;
use crate::utils::kv::Kv;

#[derive(Debug, Clone)]
pub struct SubtitleOption {
	pub selector: Selector,
	pub language: Option<String>,
	pub codec: Option<String>,
	pub default: Option<String>,
	pub shift: Option<String>,
	pub font_size: Option<String>,
	pub color: Option<String>,
	pub position: Option<String>,
	pub fps: Option<String>,
	pub encoding: Option<String>,
	pub translate: Option<String>,
}

impl TryFrom<&str> for SubtitleOption {
	type Error = message::Message;

	fn try_from(text: &str) -> message::Result<Self> {
		let kv = Kv::parse(text)?;
		let selector = Selector::from_kv(&kv)?.unwrap_or_default();
		Ok(SubtitleOption {
			selector,
			language: kv.get("language"),
			codec: kv.get("codec"),
			default: kv.get("default"),
			shift: kv.get("shift"),
			font_size: kv.get("font_size"),
			color: kv.get("color"),
			position: kv.get("position"),
			fps: kv.get("fps"),
			encoding: kv.get("encoding"),
			translate: kv.get("translate"),
		})
	}
}
