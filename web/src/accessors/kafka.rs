use rdkafka::{ClientConfig, Message};
use rdkafka::consumer::{StreamConsumer, Consumer};
use rdkafka::config::RDKafkaLogLevel;
use crate::config::env::EnvConfig;
use futures::StreamExt;
use std::str;
use log::{info, error, debug};
use crate::accessors::db::Db;
use serde::{Serialize, Deserialize};
use crate::accessors::db;

pub struct KafkaConsumer {}

impl KafkaConsumer {
    pub fn create_consumer(brokers: &str, group_id: &str, topic: &str) -> StreamConsumer {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", group_id)
            .set("bootstrap.servers", brokers)
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "false")
            .set("auto.commit.interval.ms", "1000")
            .set("enable.auto.offset.store", "false")
            .set_log_level(RDKafkaLogLevel::Debug)
            .create()
            .expect("Consumer creation failed");

        consumer
            .subscribe(&[topic])
            .expect("Can't subscribe to specified topic");
        consumer
    }

    pub async fn receive_messages(cfg: EnvConfig, db_worker: &'static db::Db) {
        let consumer = KafkaConsumer::create_consumer(&cfg.kafka_brokers,
                             &cfg.kafka_consumer_group,
                             &cfg.kafka_topic);
        let mut msg_stream = consumer.start();
        // iterate over all messages blocking
        while let Some(msg) = msg_stream.next().await {
            // we cant borrow cfg to the async function, since that would require static a lifetime
            // therefore, we copy it
            info!("Got the message {:?}", msg);
            let cfg = cfg.clone();

            match msg {
                Ok(msg) => {
                    // tha payload can be empty
                    match msg.payload() {
                        Some(payload) => {
                            let str_msg = str::from_utf8(payload).unwrap();
                            let users: Vec<common::User> = serde_json::from_str(&str_msg).unwrap();
                            debug!("Received message: {}", str_msg);
                            info!("Received users: {}", users.len());
                            for user in users {
                                db_worker.insert_user(user).await;
                            }
                        }
                        None => {
                            error!("Message with empty payload");
                        }
                    }
                    // now we can store the offset to be committed in the next auto-commit so this
                    // message will never be processed again
                    let res = consumer.store_offset(&msg);
                    match res {
                        Ok(()) => {}
                        Err(e) => error!("Could not commit message: {} ", e)
                    }
                }
                Err(e) => {
                    error!("Could not receive and will not process message: {}",e)
                }
            };
        }
    }
}


