use crate::{error, message};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Time {
	pub num: u32,
	pub den: u32,
}

impl Time {
	pub fn new(num: u32, den: u32) -> message::Result<Self> {
		if num == 0 || den == 0 {
			return Err(error!("numerator and denominator must be > 0"));
		}
		Ok(Self { num, den })
	}

	pub fn to_seconds(&self, pts: i64) -> f64 {
		pts as f64 * (self.num as f64) / (self.den as f64)
	}

	pub fn from_seconds(&self, seconds: f64) -> i64 {
		(seconds * self.den as f64 / self.num as f64) as i64
	}

	pub fn scale_pts(&self, pts: i64, target: Time) -> Option<i64> {
		let mut pts128 = pts as i128;

		pts128 = pts128.checked_mul(target.num as i128)?;
		pts128 = pts128.checked_div(target.den as i128)?;
		pts128 = pts128.checked_mul(self.den as i128)?;
		pts128 = pts128.checked_div(self.num as i128)?;

		pts128.try_into().ok()
	}

	pub fn gcd(&self) -> u32 {
		fn gcd(a: u32, b: u32) -> u32 {
			if b == 0 { a } else { gcd(b, a % b) }
		}
		gcd(self.num, self.den)
	}

	pub fn simplify(&self) -> Time {
		let g = self.gcd();
		Time { num: self.num / g, den: self.den / g }
	}
}

#[derive(Debug, Clone, Default, Copy)]
pub struct Timestamp {
	pub time: Time,
	pub pts: i64,
}

impl Timestamp {
	pub fn new(time: Time, pts: i64) -> Self {
		Self { time, pts }
	}

	pub fn zero(time: Time) -> Self {
		Self { pts: 0, time }
	}

	pub fn advance(&mut self, delta: i64) {
		self.pts += delta;
	}

	pub fn as_seconds(&self) -> f64 {
		self.time.to_seconds(self.pts)
	}

	pub fn scale_to(&self, target: Time) -> Option<Timestamp> {
		let scaled_pts = self.time.scale_pts(self.pts, target)?;
		Some(Timestamp { pts: scaled_pts, time: target })
	}
}

impl From<Timestamp> for i64 {
	fn from(timestamp: Timestamp) -> Self {
		timestamp.pts
	}
}
