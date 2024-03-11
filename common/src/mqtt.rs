use paho_mqtt as mqtt;
use std::time::Duration;

pub struct MqttClient {
	client: mqtt::Client,
}

impl MqttClient {
	pub fn new(client_id: &str, broker: &str) -> Result<Self, mqtt::Error> {
		let create_opts = mqtt::CreateOptionsBuilder::new().server_uri(broker).client_id(client_id).finalize();

		let client = mqtt::Client::new(create_opts)?;
		Ok(MqttClient { client })
	}

	pub fn connect(&mut self, keep_alive_interval: Duration, clean_session: bool) -> Result<(), mqtt::Error> {
		let conn_opts =
			mqtt::ConnectOptionsBuilder::new().keep_alive_interval(keep_alive_interval).clean_session(clean_session).finalize();

		self.client.connect(conn_opts).map_err(|e| e)?;
		Ok(())
	}

	pub fn publish(&self, topic: &str, payload: &str, qos: i32) -> Result<(), mqtt::Error> {
		let msg = mqtt::MessageBuilder::new().topic(topic).payload(payload).qos(qos).finalize();

		self.client.publish(msg)
	}

	pub fn disconnect(&self) -> Result<(), mqtt::Error> {
		self.client.disconnect(None)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::time::Duration;

	#[test]
	fn test_mqtt_client() -> Result<(), mqtt::Error> {
		let client_id = "test_mqtt_client";
		let broker = "tcp://localhost:1891";
		let topic = "test";
		let payload = "test_message";
		let keep_alive = Duration::from_secs(60);

		let mut mqtt_client = MqttClient::new(client_id, broker)?;
		mqtt_client.connect(keep_alive, true)?;
		mqtt_client.publish(topic, payload, 2)?; // Example QoS value
		mqtt_client.disconnect()?;
		Ok(())
	}
}
