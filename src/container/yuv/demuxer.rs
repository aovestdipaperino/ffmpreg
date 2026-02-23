use super::format::YuvFormat;
use crate::core::frame::BitDepth;
use crate::core::packet::Packet;
use crate::core::time::{Time, Timestamp};
use crate::core::track::{ColorSpace, Metadata, Pixel, TrackFormat, VideoFormat};
use crate::core::{Demuxer, Track, Tracks};
use crate::io::MediaRead;
use crate::{error, message::Result};

pub struct YuvDemuxer<R: MediaRead> {
	reader: R,
	format: YuvFormat,
	track: Track,
	metadata: Metadata,
	frame_count: u64,
	time: Time,
	read_buf: Vec<u8>,
}

impl<R: MediaRead> YuvDemuxer<R> {
	pub fn new(reader: R, format: YuvFormat) -> Result<Self> {
		let frame_size = format.frame_size();
		if frame_size == 0 {
			return Err(error!("yuv frame size must be non-zero"));
		}

		let fps_num = (format.fps * 1000.0) as u32;
		let fps_den = 1000u32;
		let time = Time::new(fps_den, fps_num)?;

		let codec = format.to_codec_id();

		let video_format = VideoFormat {
			width: format.width,
			height: format.height,
			pixel: Pixel {
				depth: BitDepth::from_bits_any(format.pixel.depth),
				color_space: ColorSpace::YUV,
			},
		};

		let track = Track {
			id: 0,
			codec_in: codec,
			codec_out: codec,
			timestamp: Timestamp::zero(time),
			format: TrackFormat::Video(video_format),
		};

		Ok(Self {
			reader,
			format,
			track,
			metadata: Metadata::default(),
			frame_count: 0,
			time,
			read_buf: vec![0u8; frame_size],
		})
	}

	pub fn format(&self) -> YuvFormat {
		self.format
	}
}

impl<R: MediaRead> Demuxer for YuvDemuxer<R> {
	fn read(&mut self) -> Result<Option<Packet>> {
		let frame_size = self.format.frame_size();
		let mut filled = 0;

		while filled < frame_size {
			let n = self.reader.read(&mut self.read_buf[filled..])?;
			if n == 0 {
				if filled == 0 {
					return Ok(None);
				}
				break;
			}
			filled += n;
		}

		if filled < frame_size {
			return Ok(None);
		}

		let data = self.read_buf[..frame_size].to_vec();
		let pts = self.frame_count as i64;
		let is_key = self.frame_count == 0;

		let packet = Packet::new(data, 0, self.time)
			.with_pts(pts)
			.with_keyframe(is_key);

		self.frame_count += 1;

		Ok(Some(packet))
	}

	fn seek(&mut self, _time: f64) -> Result<()> {
		Err(error!("seek not implemented for yuv"))
	}

	fn duration(&self) -> Option<f64> {
		None
	}

	fn tracks(&self) -> Tracks {
		Tracks::new(vec![self.track])
	}

	fn metadata(&self) -> &Metadata {
		&self.metadata
	}
}
