use crate::core::CodecId;

// lossy audio codecs

#[rustfmt::skip]
pub const AAC: CodecId = CodecId::new("aac")
  .exts(&["aac", "m4a"]);

#[rustfmt::skip]
pub const MP3: CodecId = CodecId::new("mp3")
  .exts(&["mp3"]);

#[rustfmt::skip]
pub const MP2: CodecId = CodecId::new("mp2")
  .exts(&["mp2"]);

#[rustfmt::skip]
pub const OPUS: CodecId = CodecId::new("opus")
  .exts(&["opus", "ogg"]);

#[rustfmt::skip]
pub const VORBIS: CodecId = CodecId::new("vorbis")
  .exts(&["ogg"]);

#[rustfmt::skip]
pub const AMR_NB: CodecId = CodecId::new("amr_nb")
  .exts(&["amr"]);

#[rustfmt::skip]
pub const AMR_WB: CodecId = CodecId::new("amr_wb")
  .exts(&["amr"]);

#[rustfmt::skip]
pub const AC3: CodecId = CodecId::new("ac3")
  .exts(&["ac3"]);

#[rustfmt::skip]
pub const EAC3: CodecId = CodecId::new("eac3")
  .exts(&["eac3"]);

#[rustfmt::skip]
pub const WMA: CodecId = CodecId::new("wma")
  .exts(&["wma"]);

#[rustfmt::skip]
pub const ATRAC3: CodecId = CodecId::new("atrac3")
  .exts(&["aa3"]);

// lossless audio codecs
#[rustfmt::skip]
pub const FLAC: CodecId = CodecId::new("flac")
  .exts(&["flac"]);

#[rustfmt::skip]
pub const ALAC: CodecId = CodecId::new("alac")
  .exts(&["m4a"]);

#[rustfmt::skip]
pub const WAVPACK: CodecId = CodecId::new("wavpack")
  .exts(&["wv"]);

#[rustfmt::skip]
pub const TTA: CodecId = CodecId::new("tta")
  .exts(&["tta"]);

#[rustfmt::skip]
pub const APE: CodecId = CodecId::new("ape");

// pcm / uncompressed audio
#[rustfmt::skip]
pub const PCM_S16LE: CodecId = CodecId::new("pcm_s16le")
  .aliases(&["pcm"]).exts(&["wav"]);

#[rustfmt::skip]
pub const PCM_S24LE: CodecId = CodecId::new("pcm_s24le")
  .exts(&["wav"]);

#[rustfmt::skip]
pub const PCM_F32LE: CodecId = CodecId::new("pcm_f32le")
  .exts(&["wav"]);

// misc / special audio
#[rustfmt::skip]
pub const DSD_LSBF: CodecId = CodecId::new("dsd_lsbf")
  .exts(&["dsf"]);

#[rustfmt::skip]
pub const DSD_MSBF: CodecId = CodecId::new("dsd_msbf")
  .exts(&["dsf"]);

#[rustfmt::skip]
pub const DSD_LSBF_PLANAR: CodecId = CodecId::new("dsd_lsbf_planar")
  .exts(&["dff"]);

#[rustfmt::skip]
pub const DSD_MSBF_PLANAR: CodecId = CodecId::new("dsd_msbf_planar")
  .exts(&["dff"]);
