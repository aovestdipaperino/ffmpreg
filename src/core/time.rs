use crate::{error, message};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Time {
	pub numerator: u32,
	pub denominator: u32,
}

impl Time {
	pub fn new(numerator: u32, denominator: u32) -> message::Result<Self> {
		if numerator == 0 || denominator == 0 {
			return Err(error!("numerator and denominator must be > 0"));
		}
		Ok(Self { numerator, denominator })
	}

	pub fn to_seconds(&self, pts: i64) -> f64 {
		pts as f64 * (self.numerator as f64) / (self.denominator as f64)
	}

	pub fn from_seconds(&self, seconds: f64) -> i64 {
		(seconds * self.denominator as f64 / self.numerator as f64) as i64
	}

	pub fn scale_pts(&self, pts: i64, target: Time) -> Option<i64> {
		let mut pts128 = pts as i128;

		pts128 = pts128.checked_mul(target.numerator as i128)?;
		pts128 = pts128.checked_div(target.denominator as i128)?;
		pts128 = pts128.checked_mul(self.denominator as i128)?;
		pts128 = pts128.checked_div(self.numerator as i128)?;

		pts128.try_into().ok()
	}

	pub fn gcd(&self) -> u32 {
		fn gcd(a: u32, b: u32) -> u32 {
			if b == 0 { a } else { gcd(b, a % b) }
		}
		gcd(self.numerator, self.denominator)
	}

	pub fn simplify(&self) -> Time {
		let g = self.gcd();
		Time { numerator: self.numerator / g, denominator: self.denominator / g }
	}
}
