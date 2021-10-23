use rdkafka::producer::FutureProducer;
use rdkafka::ClientConfig;
use rdkafka::admin::{AdminClient, AdminOptions};
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::StreamConsumer;

pub fn create_producer(brokers: &str, ) -> FutureProducer {

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");
    producer
}
