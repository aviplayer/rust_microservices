use deadpool_postgres::{Client, Config, Pool};
use tokio_postgres::tls::NoTls;

use crate::config::env::EnvConfig;

#[derive(Clone)]
pub struct Connection {}

impl Connection {
    fn create_config(env_cfg: EnvConfig) -> Config {
        let mut cfg = Config::new();
        cfg.dbname = Some(env_cfg.db_name);
        cfg.port = Some(env_cfg.db_port);
        cfg.host = Some(env_cfg.db_host);
        cfg.user = Some(env_cfg.db_user);
        cfg.password = Some(env_cfg.db_pwd);
        cfg
    }

    pub fn new(env_cfg: EnvConfig) -> Pool {
        let config = Connection::create_config(env_cfg);
        match config.create_pool(NoTls)
        {
            Ok(pool) => {
                pool
            }
            Err(error) => {
                panic!("Connection Pool Critical Error: \n{}", error);
            }
        }
    }
}
