use crate::io::WritePrimitives;
use crate::message::Result;

/// Write an EBML element ID (1-4 bytes, big-endian, variable-length).
pub fn write_id<W: WritePrimitives>(w: &mut W, id: u32) -> Result<()> {
	if id <= 0xFF {
		w.write_u8(id as u8)
	} else if id <= 0xFFFF {
		w.write_u16_be(id as u16)
	} else if id <= 0xFF_FFFF {
		w.write_u8((id >> 16) as u8)?;
		w.write_u16_be(id as u16)
	} else {
		w.write_u32_be(id)
	}
}

/// Encode a size as EBML variable-length integer (VINT).
/// Returns the encoded bytes.
fn encode_vint_size(size: u64) -> (u8, [u8; 8]) {
	let mut buf = [0u8; 8];
	if size < 0x7F {
		buf[0] = 0x80 | size as u8;
		(1, buf)
	} else if size < 0x3FFF {
		let val = 0x4000 | size as u16;
		buf[..2].copy_from_slice(&val.to_be_bytes());
		(2, buf)
	} else if size < 0x1F_FFFF {
		buf[0] = 0x20 | (size >> 16) as u8;
		buf[1] = (size >> 8) as u8;
		buf[2] = size as u8;
		(3, buf)
	} else if size < 0x0FFF_FFFF {
		let val = 0x1000_0000 | size as u32;
		buf[..4].copy_from_slice(&val.to_be_bytes());
		(4, buf)
	} else {
		// Use 8-byte encoding for unknown/large sizes
		let val = 0x0100_0000_0000_0000u64 | size;
		buf = val.to_be_bytes();
		(8, buf)
	}
}

/// Write an EBML VINT size field.
pub fn write_size<W: WritePrimitives>(w: &mut W, size: u64) -> Result<()> {
	let (len, buf) = encode_vint_size(size);
	w.write_all(&buf[..len as usize])
}

/// Write the "unknown size" marker (8-byte VINT with all data bits set).
pub fn write_unknown_size<W: WritePrimitives>(w: &mut W) -> Result<()> {
	// 0x01FF_FFFF_FFFF_FFFF = unknown size in 8-byte VINT
	w.write_all(&[0x01, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF])
}

/// Write an EBML unsigned integer element.
pub fn write_uint<W: WritePrimitives>(w: &mut W, id: u32, value: u64) -> Result<()> {
	write_id(w, id)?;
	let bytes = uint_encoded_len(value);
	write_size(w, bytes as u64)?;
	// Write only the significant bytes, big-endian
	let be = value.to_be_bytes();
	w.write_all(&be[8 - bytes..])
}

/// Write an EBML float element (always 8 bytes / f64).
pub fn write_float<W: WritePrimitives>(w: &mut W, id: u32, value: f64) -> Result<()> {
	write_id(w, id)?;
	write_size(w, 8)?;
	w.write_f64_be(value)
}

/// Write an EBML string (ASCII) element.
pub fn write_string<W: WritePrimitives>(w: &mut W, id: u32, value: &str) -> Result<()> {
	write_id(w, id)?;
	write_size(w, value.len() as u64)?;
	w.write_all(value.as_bytes())
}

/// Write an EBML UTF-8 string element.
pub fn write_utf8<W: WritePrimitives>(w: &mut W, id: u32, value: &str) -> Result<()> {
	write_string(w, id, value)
}

/// Write an EBML binary element.
pub fn write_binary<W: WritePrimitives>(w: &mut W, id: u32, value: &[u8]) -> Result<()> {
	write_id(w, id)?;
	write_size(w, value.len() as u64)?;
	w.write_all(value)
}

/// Number of bytes needed to represent a uint value.
fn uint_encoded_len(value: u64) -> usize {
	if value == 0 {
		1
	} else {
		let bits = 64 - value.leading_zeros() as usize;
		(bits + 7) / 8
	}
}
