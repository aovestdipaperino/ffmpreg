use crate::core::CodecId;

#[rustfmt::skip]
pub const H265: CodecId = CodecId::new("h265").aliases(&["hevc"])
  .exts(&["mp4", "mkv", "mov"]);

#[rustfmt::skip]
pub const H264: CodecId = CodecId::new("h264")
  .exts(&["mp4", "mkv", "mov"]);

#[rustfmt::skip]
pub const VP8: CodecId = CodecId::new("vp8")
  .exts(&["webm", "mkv"]);

#[rustfmt::skip]
pub const VP9: CodecId = CodecId::new("vp9")
  .exts(&["webm", "mkv"]);

#[rustfmt::skip]
pub const AV1: CodecId = CodecId::new("av1")
  .exts(&["mkv", "mp4", "webm"]);

// older / legacy codecs
#[rustfmt::skip]
pub const MPEG2: CodecId = CodecId::new("mpeg2")
  .exts(&["mpg", "mpeg", "ts"]);

#[rustfmt::skip]
pub const MPEG4: CodecId = CodecId::new("mpeg4")
  .exts(&["mp4", "avi"]);

#[rustfmt::skip]
pub const THEORA: CodecId = CodecId::new("theora")
  .exts(&["ogv", "mkv"]);

#[rustfmt::skip]
pub const VP6: CodecId = CodecId::new("vp6")
  .exts(&["flv"]);

#[rustfmt::skip]
pub const WMV1: CodecId = CodecId::new("wmv1")
  .exts(&["wmv"]);

#[rustfmt::skip]
pub const WMV2: CodecId = CodecId::new("wmv2")
  .exts(&["wmv"]);

#[rustfmt::skip]
pub const WMV3: CodecId = CodecId::new("wmv3")
  .exts(&["wmv"]);

// motion JPEG
#[rustfmt::skip]
pub const MJPEG: CodecId = CodecId::new("mjpeg")
  .exts(&["avi", "mjpg"]);

#[rustfmt::skip]
pub const JPEG2000: CodecId = CodecId::new("jpeg2000")
  .exts(&["jp2", "mxf"]);

// specialized / less common
#[rustfmt::skip]
pub const PRORES: CodecId = CodecId::new("prores")
  .exts(&["mov", "mxf"]);

#[rustfmt::skip]
pub const DNXHD: CodecId = CodecId::new("dnxhd")
  .exts(&["mxf"]);

#[rustfmt::skip]
pub const DNXHR: CodecId = CodecId::new("dnxhr")
  .exts(&["mxf"]);

// experimental / emerging
#[rustfmt::skip]
pub const VP10: CodecId = CodecId::new("vp10")
  .exts(&["webm", "mkv"]);

// raw video
#[rustfmt::skip]
pub const YUV420P: CodecId = CodecId::new("yuv420p")
  .exts(&["yuv"]);
