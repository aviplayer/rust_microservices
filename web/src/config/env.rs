use log::Level;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "web")]
pub struct EnvConfig {
    #[structopt(short = "t", long, default_value = "users")]
    pub kafka_topic: String,

    #[structopt(short = "b", long, default_value = "127.0.0.1:11002")]
    pub kafka_brokers: String,

    #[structopt(short, long, default_value = "consumer_group")]
    pub kafka_consumer_group: String,

    #[structopt(short = "l", long, default_value = "info")]
    pub log_level: Level,

    #[structopt(short = "h", long, default_value = "localhost")]
    pub host: String,

    #[structopt(short = "p", long, default_value = "11007")]
    pub port: i16,

    #[structopt(short = "H", long, default_value = "localhost")]
    pub db_host: String,

    #[structopt(short = "P", long, default_value = "11001")]
    pub db_port: u16,

    #[structopt(short = "n", long, default_value = "user_db")]
    pub db_name: String,

    #[structopt(short = "u", long, default_value = "local")]
    pub db_user: String,

    #[structopt(short = "c", long, default_value = "local")]
    pub db_pwd: String,
}
