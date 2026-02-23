use super::{YuvDemuxer, YuvFormat, YuvMuxer};
use crate::container;
use crate::core::resolver;

use resolver::{ContainerEntry, ContainerExtMapper, ContainerMapper};

pub fn yuv_container(mapper: &mut ContainerMapper, ext_mapper: &mut ContainerExtMapper) {
	let demuxer: resolver::DemuxerFactory = |file| {
		let format = YuvFormat::default();
		let demuxer = YuvDemuxer::new(file, format)?;
		Ok(Box::new(demuxer))
	};

	let muxer: resolver::MuxerFactory = |file, format| {
		let muxer = YuvMuxer::from_format(file, format)?;
		Ok(Box::new(muxer))
	};

	let entry = ContainerEntry { demuxer: Some(demuxer), muxer: Some(muxer) };

	mapper.insert(container::YUV, entry);
	ext_mapper.insert("yuv", container::YUV);
}
