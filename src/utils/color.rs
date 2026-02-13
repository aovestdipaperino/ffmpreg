#![allow(non_upper_case_globals)]
pub struct Paint {
	pub code: &'static str,
}

impl Paint {
	pub fn text(&self, content: impl Into<String>) -> String {
		format!("{}{}{}", self.code, content.into(), "\x1b[0m")
	}

	pub fn char(&self, c: char) -> String {
		format!("{}{}{}", self.code, c, "\x1b[0m")
	}
}

pub const red: Paint = Paint { code: "\x1b[31m" };
pub const green: Paint = Paint { code: "\x1b[32m" };
pub const yellow: Paint = Paint { code: "\x1b[33m" };
pub const blue: Paint = Paint { code: "\x1b[34m" };
pub const magenta: Paint = Paint { code: "\x1b[35m" };
pub const cyan: Paint = Paint { code: "\x1b[36m" };
pub const white: Paint = Paint { code: "\x1b[37m" };
pub const dark_gray: Paint = Paint { code: "\x1b[90m" };
pub const light_gray: Paint = Paint { code: "\x1b[97m" };
pub const light_red: Paint = Paint { code: "\x1b[91m" };
pub const light_green: Paint = Paint { code: "\x1b[92m" };
pub const light_blue: Paint = Paint { code: "\x1b[94m" };
pub const light_magenta: Paint = Paint { code: "\x1b[95m" };
pub const light_cyan: Paint = Paint { code: "\x1b[96m" };
pub const light_white: Paint = Paint { code: "\x1b[97m" };
