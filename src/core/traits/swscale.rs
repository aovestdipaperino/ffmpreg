use crate::core::frame::Frame;
use crate::message::Result;

pub trait Swscale {
	fn new(input: &Frame, output: &Frame) -> Self;
	fn scale(&mut self, input: &Frame, output: &mut Frame) -> Result<()>;
}
