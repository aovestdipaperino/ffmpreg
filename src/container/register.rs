use crate::{container::wav, core::resolver};
use rustc_hash::FxHashMap;

pub fn register_containers() -> (resolver::ContainerMapper, resolver::ContainerExtMapper) {
	let mut containers = FxHashMap::default();
	let mut extensions = FxHashMap::default();

	wav::wav_container(&mut containers, &mut extensions);
	(containers, extensions)
}
