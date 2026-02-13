pub enum Flag {
	Input,
	Output,
	Audio,
	Video,
	Subtitle,
	Apply,
	Json,
}

impl Flag {
	pub fn parse(token: &str) -> Option<Flag> {
		match token {
			"-i" | "--input" => Some(Flag::Input),
			"-o" | "--output" => Some(Flag::Output),
			"-a" | "--audio" => Some(Flag::Audio),
			"-v" | "--video" => Some(Flag::Video),
			"-s" | "--subtitle" => Some(Flag::Subtitle),
			"-t" | "--apply" => Some(Flag::Apply),
			"--json" => Some(Flag::Json),
			_ => None,
		}
	}
}
