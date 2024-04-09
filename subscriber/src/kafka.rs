use common::utils::dotenv;
use futures_util::stream::StreamExt;
use rdkafka::config::ClientConfig;
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::error::KafkaError;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::Message;
use std::error::Error;
use std::future::Future;
use std::time::Duration;
pub struct Kafka {
	consumer: StreamConsumer,
	producer: FutureProducer,
}

impl Kafka {
	pub fn new() -> Result<Self, Box<dyn Error>> {
		let brokers = dotenv::get_var("KAFKA_BROKER").unwrap();
		let group_id = dotenv::get_var("KAFKA_GROUP_ID").unwrap();

		let consumer: StreamConsumer = ClientConfig::new()
			.set("group.id", &group_id)
			.set("bootstrap.servers", &brokers)
			.set("enable.partition.eof", "false")
			.set("session.timeout.ms", "6000")
			.set("enable.auto.commit", "true")
			.set("security.protocol", "sasl_ssl")
			.set("sasl.mechanisms", "SCRAM-SHA-256")
			.set("sasl.username", &dotenv::get_var("KAFKA_USERNAME").unwrap())
			.set("sasl.password", &dotenv::get_var("KAFKA_PASSWORD").unwrap())
			.set_log_level(RDKafkaLogLevel::Debug)
			.create()?;

		let producer: FutureProducer = ClientConfig::new()
			.set("bootstrap.servers", &brokers)
			.set("security.protocol", "sasl_ssl")
			.set("sasl.mechanisms", "SCRAM-SHA-256")
			.set("sasl.username", &dotenv::get_var("KAFKA_USERNAME").unwrap())
			.set("sasl.password", &dotenv::get_var("KAFKA_PASSWORD").unwrap())
			.create()?;

		Ok(Kafka { consumer, producer })
	}

	pub fn subscribe(&self, topic: &str) -> Result<(), KafkaError> {
		self.consumer.subscribe(&[topic])
	}

	pub async fn consume<F, Fut>(&self, mut callback: F) -> Result<(), Box<dyn Error>>
	where
		F: FnMut(&str) -> Fut,
		Fut: Future<Output = Result<(), Box<dyn Error>>>,
	{
		let mut message_stream = self.consumer.stream();

		while let Some(message) = message_stream.next().await {
			let message = message?;
			let payload = match message.payload_view::<str>() {
				None => "",
				Some(Ok(s)) => s,
				Some(Err(_)) => {
					println!("Error while deserializing message payload");
					""
				}
			};

			callback(payload).await?;
		}

		Ok(())
	}

	pub async fn produce(&self, topic: &str, payload: &str) -> Result<(), KafkaError> {
		let result = self.producer.send(FutureRecord::to(topic).payload(payload).key("sensors"), Duration::from_secs(0)).await;
		match result {
			Ok(_) => Ok(()),
			Err((error, _)) => Err(error),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn test_integrity() {
		let kafka = Kafka::new().expect("Failed to create Kafka client");
		let topic = dotenv::get_var("KAFKA_TOPIC_TESTS").unwrap();
		let test_message = "Hello NicoNicoNii";

		kafka.subscribe(&topic).expect("Failed to subscribe to topic");

		tokio::time::sleep(Duration::from_secs(1)).await;

		kafka.produce(&topic, test_message).await.expect("Failed to produce message");

		let mut consumed_message = None;

		let consume_future = kafka.consume(|payload| {
			consumed_message = Some(payload.to_owned());
			async move { Ok(()) }
		});

		let consume_timeout = tokio::time::timeout(Duration::from_secs(5), consume_future);
		if let Err(_) = consume_timeout.await {
			panic!("Consuming timed out without receiving a message");
		}

		kafka.consumer.unsubscribe();

		assert_eq!(
			consumed_message.map(|s| s.to_owned()),
			Some(test_message.to_owned()),
			"Consumed message does not match produced message"
		);
	}

	#[tokio::test]
	async fn test_produce() {
		let kafka = Kafka::new().expect("Failed to create Kafka client");
		let topic = dotenv::get_var("KAFKA_TOPIC_TESTS").unwrap();
		let test_message = "Hello NicoNicoNii";

		kafka.produce(&topic, test_message).await.expect("Failed to produce message");
	}

	#[tokio::test]
	async fn test_consume() {
		let kafka = Kafka::new().expect("Failed to create Kafka client");
		let topic = dotenv::get_var("KAFKA_TOPIC_TESTS").unwrap();
		let test_message = "Hello NicoNicoNii";

		kafka.subscribe(&topic).expect("Failed to subscribe to topic");

		tokio::time::sleep(Duration::from_secs(1)).await;

		kafka.produce(&topic, test_message).await.expect("Failed to produce message");

		let mut consumed_message = None;

		kafka
			.consume(|payload| {
				consumed_message = Some(payload.to_owned());
				async move { Ok(()) }
			})
			.await
			.expect("Failed to consume message");

		kafka.consumer.unsubscribe();

		assert_eq!(
			consumed_message.map(|s| s.to_owned()),
			Some(test_message.to_owned()),
			"Consumed message does not match produced message"
		);
	}

	#[tokio::test]
	async fn test_subscribe() {
		let kafka = Kafka::new().expect("Failed to create Kafka client");
		let topic = dotenv::get_var("KAFKA_TOPIC_TESTS").unwrap();

		kafka.subscribe(&topic).expect("Failed to subscribe to topic");
	}
}
