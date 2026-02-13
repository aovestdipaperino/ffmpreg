use super::{WavDemuxer, WavMuxer};
use crate::container;
use crate::core::resolver;

use resolver::{ContainerEntry, ContainerExtMapper, ContainerMapper};

pub fn wav_container(mapper: &mut ContainerMapper, ext_mapper: &mut ContainerExtMapper) {
	let demuxer: resolver::DemuxerFactory = |file| {
		let demuxer = WavDemuxer::new(file)?;
		Ok(Box::new(demuxer))
	};

	let muxer: resolver::MuxerFactory = |file, format| {
		let muxer = WavMuxer::from_format(file, format)?;
		Ok(Box::new(muxer))
	};

	let entry = ContainerEntry { demuxer: Some(demuxer), muxer: Some(muxer) };

	mapper.insert(container::WAV, entry);
	ext_mapper.insert("wav", container::WAV);
}
