use crate::core::traits::Transform;
use crate::core::transcoder::Transcoder;
use crate::io::{Input, Output};

#[allow(dead_code)]
pub struct Converter {
	input: Input,
	output: Output,
	transcoder: Transcoder,
	transforms: Vec<(usize, Box<dyn Transform>)>,
}
