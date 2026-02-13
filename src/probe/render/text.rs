use crate::core::frame;
use crate::message::Report;
use crate::probe::model;
use crate::probe::render::Indenter;
use crate::utils::{self, color};

#[derive(Debug, Clone)]
pub struct TextRender {
	indent: Indenter,
	output: String,
	track_symbol: char,
}

impl Default for TextRender {
	fn default() -> Self {
		Self { indent: Indenter::from_level(0), output: String::new(), track_symbol: '*' }
	}
}

impl TextRender {
	#[allow(dead_code)]
	pub fn track_symbol(mut self, symbol: char) -> Self {
		self.track_symbol = symbol;
		self
	}

	pub fn render(&mut self, media: &model::MediaFile) -> String {
		self.output.clear();
		self.indent.reset();
		self.render_media(media);
		std::mem::take(&mut self.output)
	}

	fn render_media(&mut self, media: &model::MediaFile) {
		self.render_input(&media.input);

		for output in &media.outputs {
			self.render_output(output);
		}
	}

	fn render_input(&mut self, input: &model::InputFile) {
		let filename = utils::filename(&input.path).report();

		self.output.push_str(&format!("{}▶ {}\n", self.indent, color::cyan.text(&filename)));

		self.indent.over();

		for track in &input.tracks {
			self.render_input_track(track);
		}

		self.indent.under();
	}

	fn render_input_track(&mut self, track: &model::Track) {
		let symbol = color::red.char(self.track_symbol);
		let track_str = color::red.text(track.id.to_string());
		let mut parts = vec![format!("{} track={}", symbol, track_str), format!("{}", track.kind)];

		if let Some(codec) = &track.codec {
			parts.push(color::red.text(codec));
		}

		if let Some(audio) = &track.audio {
			if let Some(ch) = audio.channels {
				parts.push(format!("{}", ch));
			}
			if let Some(sample_rate) = audio.sample_rate {
				parts.push(sample_rate.to_string());
			}
			if let Some(bd) = audio.bit_depth {
				parts.push(self.format_bit_depth(&bd));
			}
		}

		if let Some(video) = &track.video {
			if let (Some(width), Some(height)) = (video.width, video.height) {
				parts.push(format!("{}x{}", width, height));
			}
			if let Some(fps) = video.frame_rate {
				parts.push(format!("{}fps", fps));
			}
		}

		let line = parts.join("  ");
		self.output.push_str(&format!("{}{}\n", self.indent, line));
	}

	fn render_output(&mut self, output: &model::OutputFile) {
		let filename = utils::filename(&output.path).report();
		self.output.push_str(&format!("{}→ {}\n", self.indent, filename));

		self.indent.over();

		for track in &output.tracks {
			self.render_output_track(track);
		}

		if let Some(metrics) = &output.metrics {
			self.indent.over();
			self.render_metrics(metrics);
			self.indent.under();
		}

		self.indent.under();
	}

	fn render_output_track(&mut self, track: &model::Track) {
		let symbol = color::red.char(self.track_symbol);
		let track_str = color::red.text(track.id.to_string());

		let mut parts = vec![format!("{} track={}", symbol, track_str), format!("{}", track.kind)];

		if let Some(codec) = &track.codec {
			parts.push(color::red.text(codec));
		}

		if let Some(audio) = &track.audio {
			if let Some(ch) = audio.channels {
				parts.push(format!("{}", ch));
			}
			if let Some(sr) = audio.sample_rate {
				parts.push(format!("{}", sr));
			}
			if let Some(bd) = audio.bit_depth {
				parts.push(self.format_bit_depth(&bd));
			}
		}

		if let Some(video) = &track.video {
			if let (Some(width), Some(height)) = (video.width, video.height) {
				parts.push(format!("{}x{}", width, height));
			}
			if let Some(fps) = video.frame_rate {
				parts.push(format!("{}fps", fps));
			}
		}
		let line = parts.join("  ");
		self.output.push_str(&format!("{}{}\n", self.indent, line));
	}

	fn render_metrics(&mut self, metrics: &model::Metrics) {
		let size = self.render_size(metrics.size.unwrap_or(0));
		let bitrate = metrics.bitrate.unwrap_or(0);
		let ratio = metrics.ratio.unwrap_or(0.0);
		let delta = metrics.duration_delta.unwrap_or(0.0);

		let line =
			format!("{}{}   {} kb/s   {:.0}x   {:+.2}%", self.indent, size, bitrate, ratio, delta);
		self.output.push_str(&line);
		self.output.push('\n');
	}

	#[allow(dead_code)]
	fn format_duration(&self, ms: u64) -> String {
		let total_secs = ms / 1000;
		let hours = total_secs / 3600;
		let minutes = (total_secs % 3600) / 60;
		let secs = total_secs % 60;
		format!("{:02}:{:02}:{:02}", hours, minutes, secs)
	}

	fn format_bit_depth(&self, bd: &frame::BitDepth) -> String {
		let formatted = format!("{}", bd);
		formatted.replace("-bit", "b")
	}

	pub fn render_size(&self, bytes: u64) -> String {
		const KB: f64 = 1024.0;
		const MB: f64 = KB * 1024.0;
		const GB: f64 = MB * 1024.0;
		const TB: f64 = GB * 1024.0;

		let bytes = bytes as f64;

		if bytes >= TB {
			return format!("{:.1}TiB", bytes / TB);
		}
		if bytes >= GB {
			return format!("{:.1}GiB", bytes / GB);
		}
		if bytes >= MB {
			return format!("{:.1}MiB", bytes / MB);
		}
		if bytes >= KB {
			return format!("{:.1}KiB", bytes / KB);
		}

		format!("{}B", bytes)
	}
}
