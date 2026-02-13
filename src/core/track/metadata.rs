use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Default)]
pub struct Metadata {
	pub title: Option<String>,
	pub description: Option<String>,
	pub artist: Option<String>,
	pub album: Option<String>,
	pub album_artist: Option<String>,
	pub track_number: Option<u32>,
	pub tracks_total: Option<u32>,
	pub disc_number: Option<u32>,
	pub discs_total: Option<u32>,
	pub genre: Option<String>,
	pub date: Option<String>,
	pub lyrics: Option<String>,
	pub comment: Option<String>,
	pub images: Option<Vec<AttachedImage>>,
	pub raw: Option<HashMap<String, RawValue>>,
}

#[derive(Debug, Clone)]
pub struct AttachedImage {
	pub data: Vec<u8>,
	pub mime_type: String,
	pub kind: ImageKind,
	pub name: Option<String>,
	pub description: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub enum ImageKind {
	CoverFront,
	CoverBack,
	#[default]
	Unknown,
}

#[derive(Debug, Clone)]
pub enum RawValue {
	String(String),
	Bytes(Vec<u8>),
	RichImageData(AttachedImage),
	File(AttachedFile),
}

#[derive(Debug, Clone, Default)]
pub struct AttachedFile {
	pub data: Vec<u8>,
	pub name: Option<String>,
	pub mime_type: Option<String>,
}

impl Metadata {
	pub fn set(&mut self, key: &str, value: String) {
		if value.is_empty() || key.is_empty() {
			return;
		}

		match key {
			"title" => self.title = Some(value),
			"description" => self.description = Some(value),
			"artist" => self.artist = Some(value),
			"album" => self.album = Some(value),
			"album_artist" => self.album_artist = Some(value),
			"track" | "track_number" => self.track_number = value.parse::<u32>().ok(),
			"tracks_total" => self.tracks_total = value.parse::<u32>().ok(),
			"disc_number" | "disc" => self.disc_number = value.parse::<u32>().ok(),
			"discs_total" | "disc_total" => self.discs_total = value.parse::<u32>().ok(),
			"genre" => self.genre = Some(value),
			"lyrics" => self.lyrics = Some(value),
			"date" | "release_date" | "year" => self.date = Some(value),
			"comment" => self.comment = Some(value),
			_ => self.set_raw(key, RawValue::String(value)),
		}
	}

	pub fn set_raw(&mut self, key: &str, value: RawValue) {
		if self.raw.is_none() {
			self.raw = Some(HashMap::new());
		}
		if let Some(ref mut raw) = self.raw {
			raw.insert(key.to_string(), value);
		}
	}

	pub const fn is_empty(&self) -> bool {
		self.title.is_none()
			&& self.description.is_none()
			&& self.artist.is_none()
			&& self.album.is_none()
			&& self.album_artist.is_none()
			&& self.track_number.is_none()
			&& self.tracks_total.is_none()
			&& self.disc_number.is_none()
			&& self.discs_total.is_none()
			&& self.genre.is_none()
			&& self.date.is_none()
			&& self.lyrics.is_none()
			&& self.comment.is_none()
			&& self.images.is_none()
			&& self.raw.is_none()
	}

	pub fn export_fields(&self) -> HashMap<String, String> {
		let mut fields = HashMap::new();

		if let Some(ref value) = self.title {
			fields.insert("title".to_string(), value.clone());
		}

		if let Some(ref value) = self.description {
			fields.insert("description".to_string(), value.clone());
		}

		if let Some(ref value) = self.artist {
			fields.insert("artist".to_string(), value.clone());
		}
		if let Some(ref value) = self.album {
			fields.insert("album".to_string(), value.clone());
		}
		if let Some(ref value) = self.album_artist {
			fields.insert("album_artist".to_string(), value.clone());
		}

		if let Some(v) = self.track_number {
			fields.insert("track".to_string(), v.to_string());
		}

		if let Some(v) = self.tracks_total {
			fields.insert("track_total".to_string(), v.to_string());
		}

		if let Some(v) = self.disc_number {
			fields.insert("disc".to_string(), v.to_string());
		}

		if let Some(v) = self.discs_total {
			fields.insert("disc_total".to_string(), v.to_string());
		}

		if let Some(ref value) = self.genre {
			fields.insert("genre".to_string(), value.clone());
		}

		fields
	}
}

// for debug

impl fmt::Display for Metadata {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut parts = Vec::new();

		if let Some(ref v) = self.title {
			parts.push(format!("Title: {}", v));
		}
		if let Some(ref v) = self.description {
			parts.push(format!("Description: {}", v));
		}
		if let Some(ref v) = self.artist {
			parts.push(format!("Artist: {}", v));
		}
		if let Some(ref v) = self.album {
			parts.push(format!("Album: {}", v));
		}
		if let Some(ref v) = self.album_artist {
			parts.push(format!("Album Artist: {}", v));
		}
		if let Some(v) = self.track_number {
			parts.push(format!("Track: {}", v));
		}
		if let Some(v) = self.tracks_total {
			parts.push(format!("Tracks Total: {}", v));
		}
		if let Some(v) = self.disc_number {
			parts.push(format!("Disc: {}", v));
		}
		if let Some(v) = self.discs_total {
			parts.push(format!("Discs Total: {}", v));
		}
		if let Some(ref v) = self.genre {
			parts.push(format!("Genre: {}", v));
		}
		if let Some(ref v) = self.date {
			parts.push(format!("Date: {}", v));
		}
		if let Some(ref v) = self.lyrics {
			parts.push(format!("Lyrics: {}", v));
		}
		if let Some(ref v) = self.comment {
			parts.push(format!("Comment: {}", v));
		}

		if let Some(ref images) = self.images {
			for (i, img) in images.iter().enumerate() {
				parts.push(format!(
					"Image[{}]: {} ({} bytes, kind: {:?})",
					i,
					img.name.as_deref().unwrap_or("<unnamed>"),
					img.data.len(),
					img.kind
				));
			}
		}

		if let Some(ref raw) = self.raw {
			for (k, v) in raw {
				let val = match v {
					RawValue::String(s) => s.clone(),
					RawValue::Bytes(b) => format!("{:?}", b),
					RawValue::RichImageData(img) => format!(
						"RichImageData(name: {}, size: {} bytes, kind: {:?})",
						img.name.as_deref().unwrap_or("<unnamed>"),
						img.data.len(),
						img.kind
					),
					RawValue::File(f) => format!(
						"File(name: {}, size: {} bytes, mime: {:?})",
						f.name.as_deref().unwrap_or("<unnamed>"),
						f.data.len(),
						f.mime_type.as_deref().unwrap_or("<unknown>")
					),
				};
				parts.push(format!("Raw[{}]: {}", k, val));
			}
		}

		if parts.is_empty() {
			write!(f, "(no metadata)")?;
		} else {
			write!(f, "{}", parts.join(" | "))?;
		}

		Ok(())
	}
}
