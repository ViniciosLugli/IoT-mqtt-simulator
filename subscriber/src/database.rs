use serde::{Deserialize, Deserializer};

#[allow(warnings, unused)]
use crate::models::*;
use std::error::Error;

fn parse_float_from_string<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
	D: Deserializer<'de>,
{
	let s: String = Deserialize::deserialize(deserializer)?;
	s.parse::<f64>().map_err(serde::de::Error::custom)
}

#[derive(Deserialize)]
pub struct SensorData {
	sensor: String,
	#[serde(deserialize_with = "parse_float_from_string")]
	value: f64,
	unit: String,
}

pub struct Database {
	client: PrismaClient,
}

impl Database {
	pub async fn new() -> Result<Self, Box<dyn Error>> {
		let client = PrismaClient::_builder().build().await.unwrap();

		Ok(Self { client })
	}

	pub async fn add_sensor_reading(&self, data: SensorData) -> Result<(), Box<dyn Error>> {
		self.client.sensor_reading().create(data.sensor, data.unit, data.value, vec![]).exec().await.unwrap();
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn test_integrity() {
		let db = Database::new().await.expect("Failed to create database client");
		let test_message = r#"{"sensor":"SPS30","value":42.0,"unit":"μg/m³"}"#;

		db.add_sensor_reading(serde_json::from_str(test_message).unwrap()).await.expect("Failed to add sensor reading");

		let sensor_readings = db.client.sensor_reading().find_many(vec![]).exec().await.expect("Failed to fetch sensor readings");

		assert_eq!(sensor_readings.len(), 1);
		assert_eq!(sensor_readings[0].name, "SPS30");
		assert_eq!(sensor_readings[0].value, 42.0);
		assert_eq!(sensor_readings[0].unit, "μg/m³");
	}
}
