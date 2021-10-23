use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "migration")]
pub struct EnvConfig {
    #[structopt(short="h", long, default_value = "localhost")]
    pub host: String,

    #[structopt(short="port", long, default_value = "11001")]
    pub port: u16,

    #[structopt(short="u", long, default_value = "local")]
    pub user: String,

    #[structopt(short="cred", long, default_value = "local")]
    pub password: String,

    #[structopt(short="d", long, default_value = "user_db")]
    pub dbname: String,
}
