use ffmpreg::codecs::audio::mp3::Mp3Decoder;
use ffmpreg::codecs::audio::pcm::PcmEncoder;
use ffmpreg::container::mp3::Mp3Demuxer;
use ffmpreg::container::wav::{WavFormat, WavMuxer};
use ffmpreg::core::Muxer;
use ffmpreg::core::traits::{Decoder, Encoder};

fn main() {
	let mp3_path = "demo_input.mp3";
	let wav_path = "demo_mp3_output.wav";

	// Generate a test MP3 file using ffmpeg
	println!("=== ffmpreg MP3 decode demo ===");
	println!("Generating test MP3 with ffmpeg...");
	let status = std::process::Command::new("ffmpeg")
		.args([
			"-y",
			"-f",
			"lavfi",
			"-i",
			"sine=frequency=440:duration=2",
			"-codec:a",
			"libmp3lame",
			"-b:a",
			"128k",
			mp3_path,
		])
		.stdout(std::process::Stdio::null())
		.stderr(std::process::Stdio::null())
		.status()
		.expect("ffmpeg not found");

	if !status.success() {
		eprintln!("Failed to generate test MP3");
		std::process::exit(1);
	}

	// Decode MP3 -> WAV using the library
	println!("Decoding {mp3_path} -> {wav_path}...");

	let mp3_file = ffmpreg::io::File::open(mp3_path).expect("open mp3");
	let mut demuxer = Mp3Demuxer::new(mp3_file).expect("mp3 demuxer");
	let sample_rate = demuxer.sample_rate();

	let mut mp3_decoder = Mp3Decoder::new();
	let mut pcm_encoder = PcmEncoder::new(sample_rate);

	let wav_format = WavFormat { channels: ffmpreg::core::frame::Channels::Mono, sample_rate, bit_depth: 16, format_code: 1 };
	let wav_file = ffmpreg::io::File::create(wav_path).expect("create wav");
	let mut wav_muxer = WavMuxer::new(wav_file, wav_format).expect("wav muxer");

	let mut total_frames = 0u32;
	let mut total_samples = 0usize;

	while let Some(packet) = ffmpreg::core::Demuxer::read_packet(&mut demuxer).expect("read") {
		if let Some(frame) = mp3_decoder.decode(packet).expect("decode") {
			if let Some(audio) = frame.audio() {
				total_samples += audio.nb_samples;
				if total_frames == 0 {
					println!(
						"  First frame: {}Hz, {:?}, {} samples",
						audio.sample_rate, audio.channels, audio.nb_samples
					);
				}
			}
			if let Some(out_packet) = pcm_encoder.encode(frame).expect("encode") {
				wav_muxer.write(out_packet).expect("write wav");
			}
			total_frames += 1;
		}
	}

	wav_muxer.finalize().expect("finalize wav");

	println!("  Decoded {total_frames} MP3 frames ({total_samples} samples)");
	println!("Wrote {wav_path}");

	// Clean up input
	let _ = std::fs::remove_file(mp3_path);
	println!("\nVerify with: ffprobe {wav_path}");
	println!("  or: ffplay {wav_path}");
}
