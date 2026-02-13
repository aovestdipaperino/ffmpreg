use crate::core::Selector;
use crate::message;
use crate::utils::kv::Kv;

#[derive(Debug, Clone)]
pub struct TransformOption {
	pub track: Selector,
	pub normalize: Option<String>,
	pub trim: Option<String>,
	pub fade: Option<String>,
	pub reverse: Option<String>,
	pub speed: Option<String>,
	pub rotate: Option<String>,
	pub filter_chain: Option<String>,
}

impl TryFrom<&str> for TransformOption {
	type Error = message::Message;

	fn try_from(text: &str) -> message::Result<Self> {
		let kv = Kv::parse(text)?;
		let selector = Selector::from_kv(&kv)?.unwrap_or_default();
		Ok(TransformOption {
			track: selector,
			normalize: kv.get("normalize"),
			trim: kv.get("trim"),
			fade: kv.get("fade"),
			reverse: kv.get("reverse"),
			speed: kv.get("speed"),
			rotate: kv.get("rotate"),
			filter_chain: kv.get("filter_chain"),
		})
	}
}
