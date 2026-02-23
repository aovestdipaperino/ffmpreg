use crate::message::Result;

pub trait Transform: Send {
	fn apply(&mut self, samples: &mut [f32]) -> Result<()>;
	fn name(&self) -> &'static str;
}
