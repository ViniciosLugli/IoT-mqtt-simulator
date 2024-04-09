mod database;

#[allow(warnings, unused)]
pub mod models;

use common::mqtt::MqttClient;
use database::{Database, SensorData};
use std::{error::Error, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let mut mqtt_client = MqttClient::new("SUBSCRIBER")?;
	mqtt_client.connect(Duration::from_secs(60), true)?;
	mqtt_client.subscribe("sensors", 1)?;
	let rx = mqtt_client.start_consuming();

	let db = Database::new().await?;

	for msg in rx.iter() {
		if let Some(msg) = msg {
			println!("{}", msg);

			match serde_json::from_str::<SensorData>(&msg.payload_str()) {
				Ok(sensor_data) => {
					if let Err(e) = db.add_sensor_reading(sensor_data).await {
						eprintln!("Failed to add sensor reading to the database: {}", e);
					} else {
						println!("Sensor reading added to the database.");
					}
				}
				Err(e) => eprintln!("Failed to deserialize message: {}", e),
			}
		}
	}

	Ok(())
}
