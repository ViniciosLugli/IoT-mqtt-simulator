use mqtt::Receiver;
use paho_mqtt as mqtt;
use std::time::{Duration, Instant};

pub struct MqttClient {
	client: mqtt::Client,
	broker: String,
}

impl MqttClient {
	pub fn new(client_id: &str, broker: &str) -> Result<Self, mqtt::Error> {
		let create_opts = mqtt::CreateOptionsBuilder::new().server_uri(broker).client_id(client_id).finalize();

		let client = mqtt::Client::new(create_opts)?;
		Ok(MqttClient { client, broker: broker.to_string() })
	}

	pub fn connect(&mut self, keep_alive_interval: Duration, clean_session: bool) -> Result<(), mqtt::Error> {
		let conn_opts =
			mqtt::ConnectOptionsBuilder::new().keep_alive_interval(keep_alive_interval).clean_session(clean_session).finalize();

		self.client.connect(conn_opts).map_err(|e| e)?;
		println!("Connected to the broker at {}", self.broker);
		Ok(())
	}

	pub fn publish(&self, topic: &str, payload: &str, qos: i32) -> Result<(), mqtt::Error> {
		let msg = mqtt::MessageBuilder::new().topic(topic).payload(payload).qos(qos).finalize();

		self.client.publish(msg)
	}

	pub fn subscribe(&self, topic: &str, qos: i32) -> Result<(), mqtt::Error> {
		self.client.subscribe(topic, qos).map_err(|e| e)?;
		Ok(())
	}

	pub fn unsubscribe(&self, topic: &str) -> Result<(), mqtt::Error> {
		self.client.unsubscribe(topic).map_err(|e| e)?;
		Ok(())
	}

	pub fn disconnect(&self) -> Result<(), mqtt::Error> {
		self.client.disconnect(None)
	}

	pub fn start_consuming(&self) -> Receiver<Option<mqtt::Message>> {
		self.client.start_consuming()
	}

	pub fn collect_messages(&self, duration: Duration) -> Vec<String> {
		let rx = self.start_consuming();
		let end_time = Instant::now() + duration;

		let mut messages = Vec::new();
		while Instant::now() < end_time {
			if let Ok(Some(message)) = rx.try_recv() {
				messages.push(message.payload_str().to_string());
			}
		}
		messages
	}

	pub fn wait_for_message(&self, duration: Duration) -> Option<String> {
		let rx = self.start_consuming();
		let end_time = Instant::now() + duration;

		while Instant::now() < end_time {
			if let Ok(Some(message)) = rx.try_recv() {
				return Some(message.payload_str().to_string());
			}
		}
		None
	}

	pub fn measure_rate(&self, duration: Duration) -> (usize, Duration) {
		let rx = self.start_consuming();
		let start_time = Instant::now();
		let mut message_count = 0;

		while Instant::now() - start_time < duration {
			if let Ok(Some(_)) = rx.try_recv() {
				message_count += 1;
			}
		}
		(message_count, Instant::now() - start_time)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::utils::dotenv;
	use rand;
	use std::{error::Error, sync::Arc, thread, time::Duration};
	const CLIENT_ID: &str = "test_mqtt_client";
	const MESSAGE_RATE: u64 = 10;

	fn setup_mqtt_client() -> Result<MqttClient, Box<dyn Error>> {
		let mut client = MqttClient::new(
			format!("{}-{}", CLIENT_ID, rand::random::<u16>()).as_str(),
			dotenv::get_var("BROKER").unwrap().as_str(),
		)?;
		client.connect(Duration::from_secs(60), true)?;
		Ok(client)
	}

	fn simulate_publishing(
		client: Arc<MqttClient>,
		topic: &'static str,
		duration: Duration,
		rate: u64,
	) -> thread::JoinHandle<()> {
		let sleep_time = Duration::from_secs_f64(1.0 / rate as f64);
		let handle = thread::spawn(move || {
			let start_time = Instant::now();
			while Instant::now() - start_time < duration {
				client.publish(topic, "Test message", 0).expect("Failed to publish message");
				thread::sleep(sleep_time);
			}
		});
		handle
	}

	#[test]
	fn test_collect_messages() -> Result<(), Box<dyn Error>> {
		let client = Arc::new(setup_mqtt_client()?);
		let topic = "test_collect_messages";
		client.subscribe(topic, 1)?;

		let sim_duration = Duration::from_secs(3);
		let publishing_handle = simulate_publishing(client.clone(), topic, sim_duration, MESSAGE_RATE);

		let messages = client.collect_messages(sim_duration);
		publishing_handle.join().unwrap();

		assert!(
			messages.len() >= MESSAGE_RATE as usize * sim_duration.as_secs() as usize,
			"Received messages: {}, Expected messages: {}",
			messages.len(),
			MESSAGE_RATE * sim_duration.as_secs()
		);

		client.unsubscribe(topic)?;
		client.disconnect()?;
		Ok(())
	}

	#[test]
	fn test_wait_for_message() -> Result<(), Box<dyn Error>> {
		let client = Arc::new(setup_mqtt_client()?);
		let topic = "test_wait_for_message";
		client.subscribe(topic, 1)?;

		client.publish(topic, "Test message", 0)?;
		let received_message = client.wait_for_message(Duration::from_secs(1));
		assert!(received_message.is_some());

		client.unsubscribe(topic)?;
		client.disconnect()?;
		Ok(())
	}

	#[test]
	fn test_measure_rate() -> Result<(), Box<dyn Error>> {
		let client = Arc::new(setup_mqtt_client()?);
		let topic = "test_measure_rate";
		client.subscribe(topic, 1)?;

		let measurement_duration = Duration::from_secs(5);
		let publishing_handle = simulate_publishing(client.clone(), topic, measurement_duration, MESSAGE_RATE);

		let (message_count, duration) = client.measure_rate(measurement_duration);
		publishing_handle.join().unwrap();
		let actual_rate = message_count as u64 / duration.as_secs();
		assert!((actual_rate as i64 - MESSAGE_RATE as i64).abs() <= (MESSAGE_RATE as i64 / 10));

		client.unsubscribe(topic)?;
		client.disconnect()?;
		Ok(())
	}
}
