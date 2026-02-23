use crate::core::frame::Frame;
use crate::core::track::VideoFormat;
use crate::message::Result;

pub trait Scaler {
	fn input_format(&self) -> VideoFormat;
	fn output_format(&self) -> VideoFormat;
	fn needed(&self) -> bool;
	fn scale(&mut self, frame: Frame) -> Result<Frame>;
}
