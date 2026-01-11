use super::common::Pipeline;
use crate::{error, message::Result};

pub fn run(_pipeline: Pipeline) -> Result<()> {
	Err(error!("ts pipeline not implemented"))
}
