use crate::config::env::EnvConfig;
use crate::accessors::github;
use reqwest::Response;
use crate::models::errors::ProducerError;
use crate::accessors::kafka;
use rdkafka::producer::FutureRecord;
use rdkafka::message::OwnedMessage;
use rdkafka::error::KafkaError;
use log::{debug, info, error};
use std::time::Duration;

extern crate common;

pub async fn start_processing(cfg: EnvConfig) {
    let mut num_of_users = cfg.num_of_users;
    let per_page = cfg.num_of_users_per_request;
    let mut curr_position: u8 = 0;

    let producer = kafka::create_producer(&cfg.kafka_brokers);
    let topic = cfg.kafka_topic.to_owned();

    loop {
        let curr_per_page = if num_of_users > (per_page + curr_position) {
            per_page
        } else {
            num_of_users - curr_position
        };
        let response = github::get_users(curr_position, curr_per_page).await;
        match response {
            Ok(resp) => {
                let users = resp.json::<Vec<common::User>>()
                    .await.unwrap_or(vec![]);  // Parsing error should be validated
                let users_str_rep = serde_json::to_string(&users).unwrap_or("[]".into());
                let record = FutureRecord::to(&topic)
                    .key("users".into())
                    .payload(&users_str_rep);

                let produce_future = producer.send(
                    record, rdkafka::util::Timeout::from(Duration::from_millis(0)),
                );
                match produce_future.await {
                    Ok(res) => debug!("Sent: {:?}", res),
                    Err(err) => error!("Future cancelled: {:?}", err)
                }
            }
            Err(err) => {
                error!("Error while response {:?}", err);
                // Possibly retry to be implemented!!!
            }
        }


        if num_of_users <= (per_page + curr_position) {
            break;
        }
        curr_position = curr_position + per_page;
    }
}
