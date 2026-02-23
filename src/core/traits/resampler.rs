use crate::core::frame::Frame;
use crate::core::track::AudioFormat;
use crate::message::Result;

pub trait Resampler {
	fn input_format(&self) -> AudioFormat;
	fn output_format(&self) -> AudioFormat;
	fn needed(&self) -> bool;
	fn resample(&mut self, frame: Frame) -> Result<Frame>;
}
