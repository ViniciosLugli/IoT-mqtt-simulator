use common::{mqtt::MqttClient, utils::dotenv};
use std::{error::Error, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
	let mut mqtt_client = MqttClient::new("SUBSCRIBER", dotenv::get_var("BROKER").unwrap().as_str())?;
	mqtt_client.connect(Duration::from_secs(60), true).unwrap();

	mqtt_client.subscribe("sensors", 1)?;
	let rx = mqtt_client.start_consuming();
	for msg in rx.iter() {
		if let Some(msg) = msg {
			println!("{}", msg);
		}
	}

	Ok(())
}
