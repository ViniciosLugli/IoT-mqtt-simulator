mod sensor;
use common::mqtt::MqttClient;
use sensor::{Sensor, SPS30};
use std::{error::Error, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
	let mut mqtt_client = MqttClient::new("SPS30_PUBLISHER")?;
	mqtt_client.connect(Duration::from_secs(60), true)?;

	let mut sensor = SPS30::new();

	loop {
		let value = sensor.read();
		let payload = serde_json::json!({
			"sensor": sensor.name(),
			"value": format!("{:.2}", value),
			"unit": sensor.unit()
		});

		mqtt_client.publish("sensors", payload.to_string().as_str(), 1).unwrap();
	}
}
