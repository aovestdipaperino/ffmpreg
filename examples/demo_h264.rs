use ffmpreg::codecs::video::h264::H264Encoder;
use ffmpreg::container::mkv::{AudioTrackInfo, MkvMuxer, VideoTrackInfo};
use ffmpreg::core::Muxer;
use ffmpreg::core::frame::{Frame, FrameVideo, VideoFormat};
use ffmpreg::core::packet::Packet;
use ffmpreg::core::time::Time;
use ffmpreg::core::traits::Encoder;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const FPS: u32 = 30;
const DURATION_SECS: u32 = 3;
const TOTAL_FRAMES: u32 = FPS * DURATION_SECS;
const SAMPLE_RATE: u32 = 44100;
const CHANNELS: u16 = 1;
const BIT_DEPTH: u16 = 16;

/// Generate a YUV420 frame with a moving color gradient.
fn generate_video_frame(frame_idx: u32) -> FrameVideo {
	let w = WIDTH as usize;
	let h = HEIGHT as usize;
	let y_size = w * h;
	let uv_w = w / 2;
	let uv_h = h / 2;
	let uv_size = uv_w * uv_h;

	let mut data = vec![0u8; y_size + 2 * uv_size];
	let phase = (frame_idx as f32 / TOTAL_FRAMES as f32) * std::f32::consts::TAU;

	// Y plane: moving diagonal gradient
	for row in 0..h {
		for col in 0..w {
			let offset = ((frame_idx * 2) as usize + col + row) % 256;
			data[row * w + col] = (16 + (offset * 219) / 255) as u8;
		}
	}

	// U plane: horizontal color sweep
	for row in 0..uv_h {
		for col in 0..uv_w {
			let u_val = 128.0 + 100.0 * (phase + col as f32 / uv_w as f32 * 3.0).sin();
			data[y_size + row * uv_w + col] = u_val.clamp(0.0, 255.0) as u8;
		}
	}

	// V plane: vertical color sweep
	for row in 0..uv_h {
		for col in 0..uv_w {
			let v_val = 128.0 + 100.0 * (phase + row as f32 / uv_h as f32 * 3.0).cos();
			data[y_size + uv_size + row * uv_w + col] = v_val.clamp(0.0, 255.0) as u8;
		}
	}

	let keyframe = frame_idx == 0;
	FrameVideo::new(data, WIDTH, HEIGHT, VideoFormat::YUV420, keyframe)
}

/// Generate one frame's worth of 440Hz sine wave as PCM S16LE.
fn generate_audio_chunk(frame_idx: u32) -> Vec<u8> {
	let samples_per_frame = SAMPLE_RATE / FPS;
	let start_sample = frame_idx * samples_per_frame;
	let mut pcm = Vec::with_capacity(samples_per_frame as usize * 2);

	for i in 0..samples_per_frame {
		let t = (start_sample + i) as f64 / SAMPLE_RATE as f64;
		let sample = (t * 440.0 * std::f64::consts::TAU).sin();
		let val = (sample * i16::MAX as f64) as i16;
		pcm.extend_from_slice(&val.to_le_bytes());
	}

	pcm
}

fn main() {
	let output_path = "demo_output.mkv";
	println!("=== ffmpreg H264 + audio demo ===");
	println!(
		"Video: {WIDTH}x{HEIGHT} @ {FPS}fps, {DURATION_SECS}s ({TOTAL_FRAMES} frames)"
	);
	println!("Audio: 440Hz sine, {SAMPLE_RATE}Hz, {BIT_DEPTH}-bit, mono");

	// --- Set up H264 encoder ---
	let mut h264_encoder = H264Encoder::new(FPS).expect("failed to create H264 encoder");

	// --- Set up MKV muxer ---
	let writer = ffmpreg::io::File::create(output_path).expect("failed to create output file");

	let mut muxer = MkvMuxer::new(
		writer,
		Some(VideoTrackInfo {
			width: WIDTH,
			height: HEIGHT,
			codec_private: Vec::new(),
		}),
		Some(AudioTrackInfo {
			sample_rate: SAMPLE_RATE,
			channels: CHANNELS,
			bit_depth: BIT_DEPTH,
		}),
	)
	.expect("failed to create MKV muxer");

	// --- Encode and mux frame by frame ---
	let video_time = Time::new(1, FPS);
	let audio_time = Time::new(1, FPS); // one audio chunk per video frame

	for i in 0..TOTAL_FRAMES {
		// Encode video frame
		let video = generate_video_frame(i);
		let frame = Frame::new_video(video, 1).with_pts(i as i64);

		if let Some(pkt) = h264_encoder.encode(frame).expect("video encode failed") {
			let mkv_packet = Packet::new(pkt.data, 1, video_time)
				.with_pts(i as i64)
				.with_keyframe(pkt.keyframe);
			muxer.write(mkv_packet).expect("video mux failed");
		}

		// Generate and mux audio chunk
		let audio_data = generate_audio_chunk(i);
		let audio_packet = Packet::new(audio_data, 2, audio_time)
			.with_pts(i as i64)
			.with_keyframe(true);
		muxer.write(audio_packet).expect("audio mux failed");
	}

	muxer.finalize().expect("finalize failed");

	println!("Wrote {output_path}");
	println!("\nVerify with:");
	println!("  ffprobe {output_path}");
	println!("  ffmpeg -i {output_path} -f null -");
	println!("  ffplay {output_path}");
}
