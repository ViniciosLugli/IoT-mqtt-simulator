pub trait RoundToTwoDecimals {
	fn round_to_two_decimals(self) -> Self;
}

impl RoundToTwoDecimals for f32 {
	fn round_to_two_decimals(self) -> Self {
		(self * 100.0).round() / 100.0
	}
}

pub mod dotenv {
	pub fn get_var(key: &str) -> Result<String, dotenvy::Error> {
		dotenvy::dotenv()?;

		match dotenvy::var(key) {
			Ok(value) => Ok(value),
			Err(e) => Err(e),
		}
	}
}
