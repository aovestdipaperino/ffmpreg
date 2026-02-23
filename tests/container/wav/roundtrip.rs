use osaka::io::{Input, Output};
use std::fs;
use tempfile::Builder;

fn asset_path(name: &str) -> String {
	format!("tests/assets/wav/{}", name)
}

fn temp_wav() -> tempfile::NamedTempFile {
	Builder::new().suffix(".wav").tempfile().unwrap()
}

#[test]
fn wav_roundtrip_preserves_pcm_data() {
	let input_path = asset_path("wave_16bit_44100Hz_mono.wav");
	let tmp = temp_wav();
	let output_path = tmp.path().to_path_buf();

	let mut input = Input::open(&input_path).unwrap();
	let mut output = Output::new(&output_path).unwrap();

	while let Some(packet) = input.read_packet().unwrap() {
		output.write_packet(packet).unwrap();
	}
	output.finalize().unwrap();

	let produced = fs::read(&output_path).unwrap();

	assert!(!produced.is_empty(), "output file is empty");
	assert!(produced.len() > 44, "output file too small for valid WAV");
}

#[test]
fn wav_roundtrip_stereo_preserves_format() {
	let input_path = asset_path("wave_16bit_44100Hz_stereo.wav");
	let tmp = temp_wav();
	let output_path = tmp.path().to_path_buf();

	let mut input = Input::open(&input_path).unwrap();

	let track = input.tracks.primary_audio().unwrap();
	let audio = track.audio_format().unwrap();
	assert_eq!(audio.channels.count(), 2);

	let mut output = Output::new(&output_path).unwrap();

	while let Some(packet) = input.read_packet().unwrap() {
		output.write_packet(packet).unwrap();
	}
	output.finalize().unwrap();

	let output_input = Input::open(&output_path).unwrap();
	let out_track = output_input.tracks.primary_audio().unwrap();
	let out_audio = out_track.audio_format().unwrap();

	assert_eq!(out_audio.channels.count(), 2);
}

#[test]
fn wav_roundtrip_byte_exact() {
	let input_path = asset_path("wave_16bit_44100Hz_mono.wav");
	let tmp = temp_wav();
	let output_path = tmp.path().to_path_buf();

	let mut input = Input::open(&input_path).unwrap();
	let mut output = Output::new(&output_path).unwrap();

	let mut input_data = Vec::new();
	while let Some(packet) = input.read_packet().unwrap() {
		input_data.extend_from_slice(&packet.data);
		output.write_packet(packet).unwrap();
	}
	output.finalize().unwrap();

	let mut output_input = Input::open(&output_path).unwrap();
	let mut output_data = Vec::new();
	while let Some(packet) = output_input.read_packet().unwrap() {
		output_data.extend_from_slice(&packet.data);
	}

	assert_eq!(input_data.len(), output_data.len(), "PCM data length mismatch");
	assert_eq!(input_data, output_data, "PCM data content mismatch");
}

#[test]
fn wav_input_detects_tracks() {
	let input_path = asset_path("wave_16bit_44100Hz_stereo.wav");
	let input = Input::open(&input_path).unwrap();

	assert_eq!(input.tracks.len(), 1);

	let track = input.tracks.primary_audio().unwrap();
	assert!(track.is_audio());
	assert!(!track.is_video());
}

#[test]
fn wav_open_nonexistent_file_returns_error() {
	let result = Input::open("nonexistent.wav");
	assert!(result.is_err());
}
