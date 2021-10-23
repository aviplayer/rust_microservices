use structopt::StructOpt;
use log::Level;

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "producer")]
pub struct EnvConfig {
    #[structopt(short="t", long, default_value = "users")]
    pub kafka_topic: String,

    #[structopt(short="b", long, default_value = "127.0.0.1:11002")]
    pub kafka_brokers: String,

    #[structopt(short="l", long, default_value = "info")]
    pub log_level: Level,

    #[structopt(short="u", long, default_value = "40")]
    pub num_of_users: u8,

    #[structopt(short="r", long, default_value = "10")]
    pub num_of_users_per_request: u8,
}
