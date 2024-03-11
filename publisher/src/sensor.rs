use common::utils::RoundToTwoDecimals;
use rand::Rng;
use std::{thread::sleep, time::Duration};

pub trait Sensor {
	fn read(&mut self) -> f32;
	fn unit(&self) -> &str;
	fn name(&self) -> &str;
}

pub struct SPS30 {
	last_read: f32,
} // value range: 0.3μm - 1.0μm / Read interval: 1s

impl SPS30 {
	const VALUE_RANGE: (f32, f32) = (0.3, 1.0);
	const READ_INTERVAL: u32 = 1;

	pub fn new() -> Self {
		SPS30 { last_read: 0.65 }
	}
}

impl Sensor for SPS30 {
	fn read(&mut self) -> f32 {
		let mut rng = rand::thread_rng();
		let change: f32 = rng.gen_range(-0.05..0.05);

		self.last_read = (self.last_read + change).clamp(Self::VALUE_RANGE.0, Self::VALUE_RANGE.1).round_to_two_decimals();

		sleep(Duration::from_secs(Self::READ_INTERVAL.into()));

		self.last_read
	}

	fn unit(&self) -> &str {
		"μg/m³"
	}

	fn name(&self) -> &str {
		"SPS30"
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::time::Instant;

	#[test]
	fn test_sps30_sensor_data() {
		let mut sensor = SPS30::new();
		assert_eq!(sensor.name(), "SPS30");
		assert_eq!(sensor.unit(), "μg/m³");
		let read_value = sensor.read();
		assert!(
			read_value >= SPS30::VALUE_RANGE.0 && read_value <= SPS30::VALUE_RANGE.1,
			"Sensor value out of range: {}",
			read_value
		);
	}

	#[test]
	fn test_sps30_sensor_timing() {
		let mut sensor = SPS30::new();
		let start = Instant::now();
		sensor.read();
		let duration = start.elapsed();

		let expected_duration = Duration::from_secs(SPS30::READ_INTERVAL.into());

		let leeway = Duration::from_millis(100);

		assert!(
			duration >= expected_duration && duration <= expected_duration + leeway,
			"Expected read to take about {:?}, but took {:?}",
			expected_duration,
			duration
		);
	}
}
