use futures::future::join;
use log::info;
use structopt::StructOpt;

use config::env::EnvConfig;

use crate::accessors::db;
use crate::config::{connection, kafka};
use crate::server::server::start_server;

mod config;
mod server;
mod accessors;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref DB: db::Db = {
        let cfg: EnvConfig = EnvConfig::from_args();
        let conn = connection::Connection::new(cfg.clone());
        db::Db::new(conn)
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg: EnvConfig = EnvConfig::from_args();
    env_logger::init_from_env(
        env_logger::Env::default()
            .default_filter_or(cfg.log_level.as_str())
    );
    info!("Logging initialized with level {}", cfg.log_level);

    join(
        kafka::start_consumer(cfg.clone(), &DB),
        start_server(cfg.host.as_str(), cfg.port, &DB),
    ).await;

    Ok(())
}
