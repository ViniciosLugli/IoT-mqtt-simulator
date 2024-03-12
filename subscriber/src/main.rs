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

			// Aqui, vamos tentar deserializar a mensagem MQTT em SensorData.
			match serde_json::from_str::<SensorData>(&msg.payload_str()) {
				Ok(sensor_data) => {
					// Se a deserialização for bem-sucedida, insira os dados no banco de dados.
					match db.add_sensor_reading(sensor_data).await {
						Ok(_) => println!("Sensor reading added to the database."),
						Err(e) => eprintln!("Failed to add sensor reading to the database: {}", e),
					}
				}
				Err(e) => eprintln!("Failed to deserialize message: {}", e),
			}
		}
	}

	Ok(())
}
