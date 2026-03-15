use ffmpreg::codecs::video::h264::H264Encoder;
use ffmpreg::container::avi::{AviAudioTrack, AviMuxer, AviVideoTrack};
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

fn generate_video_frame(frame_idx: u32) -> FrameVideo {
	let w = WIDTH as usize;
	let h = HEIGHT as usize;
	let y_size = w * h;
	let uv_w = w / 2;
	let uv_h = h / 2;
	let uv_size = uv_w * uv_h;
	let mut data = vec![0u8; y_size + 2 * uv_size];
	let phase = (frame_idx as f32 / TOTAL_FRAMES as f32) * std::f32::consts::TAU;

	for row in 0..h {
		for col in 0..w {
			let offset = ((frame_idx * 2) as usize + col + row) % 256;
			data[row * w + col] = (16 + (offset * 219) / 255) as u8;
		}
	}
	for row in 0..uv_h {
		for col in 0..uv_w {
			let u_val = 128.0 + 100.0 * (phase + col as f32 / uv_w as f32 * 3.0).sin();
			data[y_size + row * uv_w + col] = u_val.clamp(0.0, 255.0) as u8;
		}
	}
	for row in 0..uv_h {
		for col in 0..uv_w {
			let v_val = 128.0 + 100.0 * (phase + row as f32 / uv_h as f32 * 3.0).cos();
			data[y_size + uv_size + row * uv_w + col] = v_val.clamp(0.0, 255.0) as u8;
		}
	}

	FrameVideo::new(data, WIDTH, HEIGHT, VideoFormat::YUV420, frame_idx == 0)
}

fn generate_audio_chunk(frame_idx: u32) -> Vec<u8> {
	let samples_per_frame = SAMPLE_RATE / FPS;
	let start_sample = frame_idx * samples_per_frame;
	let mut pcm = Vec::with_capacity(samples_per_frame as usize * 2);
	for i in 0..samples_per_frame {
		let t = (start_sample + i) as f64 / SAMPLE_RATE as f64;
		let val = ((t * 440.0 * std::f64::consts::TAU).sin() * i16::MAX as f64) as i16;
		pcm.extend_from_slice(&val.to_le_bytes());
	}
	pcm
}

fn main() {
	let output_path = "demo_output.avi";
	println!("=== ffmpreg AVI demo ===");
	println!("Video: {WIDTH}x{HEIGHT} H264 @ {FPS}fps, {DURATION_SECS}s");
	println!("Audio: 440Hz sine, {SAMPLE_RATE}Hz, 16-bit, mono");

	let mut h264_encoder = H264Encoder::new(FPS).expect("H264 encoder");
	let writer = ffmpreg::io::File::create(output_path).expect("create file");

	let mut muxer = AviMuxer::new(
		writer,
		Some(AviVideoTrack { width: WIDTH, height: HEIGHT, fps: FPS, fourcc: *b"H264" }),
		Some(AviAudioTrack { sample_rate: SAMPLE_RATE, channels: 1, bit_depth: 16, format_tag: 1 }),
	)
	.expect("AVI muxer");

	let video_time = Time::new(1, FPS);
	let audio_time = Time::new(1, FPS);

	for i in 0..TOTAL_FRAMES {
		let video = generate_video_frame(i);
		let frame = Frame::new_video(video, 0).with_pts(i as i64);
		if let Some(pkt) = h264_encoder.encode(frame).expect("encode") {
			let avi_pkt = Packet::new(pkt.data, 0, video_time)
				.with_pts(i as i64)
				.with_keyframe(pkt.keyframe);
			muxer.write(avi_pkt).expect("video write");
		}

		let audio_data = generate_audio_chunk(i);
		let audio_pkt = Packet::new(audio_data, 1, audio_time)
			.with_pts(i as i64)
			.with_keyframe(true);
		muxer.write(audio_pkt).expect("audio write");
	}

	muxer.finalize().expect("finalize");
	println!("Wrote {output_path}");
}
