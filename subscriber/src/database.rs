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
	// As funções permanecem as mesmas, mas agora utilizam a instância concreta.
	pub async fn new() -> Result<Self, Box<dyn Error>> {
		let client = PrismaClient::_builder().build().await.unwrap();

		Ok(Self { client })
	}

	pub async fn add_sensor_reading(&self, data: SensorData) -> Result<(), Box<dyn Error>> {
		self.client.sensor_reading().create(data.sensor, data.value, data.unit, vec![]).exec().await.unwrap();
		Ok(())
	}
}
