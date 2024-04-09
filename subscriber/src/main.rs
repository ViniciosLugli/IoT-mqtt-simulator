mod database;
mod kafka;

#[allow(warnings, unused)]
pub mod models;

use crate::database::{Database, SensorData};
use crate::kafka::Kafka;
use common::utils::dotenv;
use std::error::Error;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let db = Arc::new(Database::new().await?);
	let kafka: Kafka = Kafka::new()?;
	let topic = dotenv::get_var("KAFKA_TOPIC").unwrap();

	kafka.subscribe(&topic)?;

	kafka
		.consume(move |payload| {
			let db: Arc<Database> = Arc::clone(&db);
			let payload = payload.to_owned();
			async move {
				match serde_json::from_str::<SensorData>(&payload) {
					Ok(sensor_data) => {
						if let Err(e) = db.add_sensor_reading(sensor_data).await {
							eprintln!("Failed to add sensor reading to the database: {}", e);
						} else {
							println!("Sensor reading added to the database: {}", payload);
						}
					}
					Err(e) => eprintln!("Failed to deserialize message: {}", e),
				}
				Ok(())
			}
		})
		.await?;

	Ok(())
}
