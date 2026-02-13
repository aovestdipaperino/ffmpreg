use crate::probe::render::text::TextRender;
use crate::probe::{Builder, render};
use crate::{cli, io::Input, io::Output, message};

pub fn runner(cmd: &cli::ProbeCommand) -> message::Result<()> {
	let input = Input::open(&cmd.input)?;
	let output = cmd.output.clone().map(|output| Output::new(output)).transpose()?;

	let builder = Builder::new(input, output);

	let media = builder.media_file()?;

	let output = match cmd.json {
		Some(cli::JsonOption::Pretty) => render::json::render_pretty(&media)?,
		Some(cli::JsonOption::Raw) => render::json::render_raw(&media)?,
		None => {
			let mut renderer = TextRender::default();
			renderer.render(&media)
		}
	};
	println!("{}", output);
	Ok(())
}
