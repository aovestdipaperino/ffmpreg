use crate::core::frame::Frame;
use crate::message::Result;

trait Swresample {
	fn new(input: &Frame, output: &Frame) -> Self;
	fn resample(&mut self, input: &Frame, output: &mut Frame) -> Result<()>;
}
