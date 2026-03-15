# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

FFmpReg is a Rust-native multimedia toolkit that decodes, transforms, and encodes audio/video without requiring FFmpeg. It provides both a CLI and a library API.

## Commands

```bash
cargo build                # Debug build
cargo build --release      # Release build
cargo run -- [ARGS]        # Run CLI (e.g., cargo run -- -i input.wav -o output.wav)
cargo test                 # Run tests
cargo fmt                  # Format (uses hard tabs, tab_spaces=2, see rustfmt.toml)
cargo clippy               # Lint
```

## Architecture

Data flows through a pipeline: **Demuxer → Decoder → Transform → Encoder → Muxer**

Key abstractions (defined as traits in `src/core/traits/`):
- **Demuxer/Muxer** — read/write container formats (WAV, RAW, MKV, WebM)
- **Decoder/Encoder** — codec-level encode/decode (PCM, ADPCM, FLAC, MP3, SRT, ASS)
- **Transform** — frame-level modifications (volume/gain, normalize)

Core data types (`src/core/`):
- **Packet** — encoded data from a container
- **Frame** — decoded data (AudioFrame, VideoFrame, SubtitleFrame)
- **Timebase** — rational time representation (num/den)

Source layout:
- `src/cli/` — CLI entry point, argument parsing (clap derive), pipeline runners, transcoders
- `src/core/` — Core types, traits, compatibility checks
- `src/codecs/` — Audio/video/subtitle codec implementations
- `src/container/` — Container format demuxers and muxers
- `src/transform/` — Audio/video transform implementations
- `src/io/` — I/O abstractions (file, cursor, stdio)

Format-specific pipelines live in `src/cli/pipeline/` and are dispatched by the executor (`src/cli/executor.rs`) based on input/output container types. The compatibility matrix in `src/core/compatible.rs` determines valid container↔codec combinations.

## Dependencies

- **clap** (derive) — CLI argument parsing
- **nom** — binary format parsing
- **glob** — batch file processing via wildcard patterns
- **rustc-hash** — FxHashMap for performance
