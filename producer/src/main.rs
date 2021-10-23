mod config;
mod accessors;
mod models;
mod services;

use crate::config::env::EnvConfig;
use crate::services::process_users;
use log::{info};
use tokio::runtime::Runtime;
use structopt::StructOpt;

fn main() {
    let cfg: EnvConfig = EnvConfig::from_args();
    simple_logger::init_with_level(cfg.log_level).expect("Could not init logger");
    info!("Logging initialized with level {}", cfg.log_level);
    Runtime::new()
        .expect("Failed to create Tokyo runtime")
        .block_on(
            process_users::start_processing(cfg.clone())
        );
}
