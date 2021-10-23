use actix_web::Error;
use futures::future::join_all;
use log::info;
use tokio::runtime::Runtime;

use crate::accessors::{kafka, db};
use crate::config::env::EnvConfig;

pub async fn start_consumer(cfg: EnvConfig, db_worker: &'static db::Db) -> Result<(), Error> {
    let futures = (0..3)
        .map(|_| {
            tokio::spawn({
                kafka::KafkaConsumer::receive_messages(cfg.clone(), db_worker)
            })
        });
    info!("Spawning  consumers in  {} threads", futures.len() as i32);
    //join_all(futures).await;
    for thread in futures {
        thread.await;
    }

    Ok(())
}
