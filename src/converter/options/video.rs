use crate::core::Selector;
use crate::message;
use crate::utils::kv::Kv;

#[derive(Debug, Clone)]
pub struct VideoOption {
	pub selector: Selector,
	pub codec: Option<String>,
	pub scale: Option<String>,
	pub width: Option<String>,
	pub height: Option<String>,
	pub fps: Option<String>,
	pub bitrate: Option<String>,
	pub aspect_ratio: Option<String>,
	pub rotate: Option<String>,
	pub brightness: Option<String>,
	pub contrast: Option<String>,
}

impl TryFrom<&str> for VideoOption {
	type Error = message::Message;

	fn try_from(text: &str) -> message::Result<Self> {
		let kv = Kv::parse(text)?;
		let selector = Selector::from_kv(&kv)?.unwrap_or_default();
		Ok(VideoOption {
			selector,
			codec: kv.get("codec"),
			scale: kv.get("scale"),
			width: kv.get("width"),
			height: kv.get("height"),
			fps: kv.get("fps"),
			bitrate: kv.get("bitrate"),
			aspect_ratio: kv.get("aspect_ratio"),
			rotate: kv.get("rotate"),
			brightness: kv.get("brightness"),
			contrast: kv.get("contrast"),
		})
	}
}
