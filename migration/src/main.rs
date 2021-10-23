mod config;

use tokio_postgres::{NoTls};
use crate::config::env::EnvConfig;
use structopt::StructOpt;

type Error = Box<dyn std::error::Error>;

mod embedded {
    use refinery::embed_migrations;

    embed_migrations!("migrations");
}

async fn run_migrations() -> std::result::Result<(), Error> {
    println!("Running DB migrations...");
    let cfg: EnvConfig = EnvConfig::from_args();
    let db_string =  &format!("host={} port={} user={} password={} dbname={}",
                              cfg.host, cfg.port,
                              cfg.user, cfg.password, cfg.dbname);
    let (mut client, con) =
        tokio_postgres::connect(db_string, NoTls)
            .await.expect("Connection failed");

    tokio::spawn(async move {
        if let Err(e) = con.await {
            eprintln!("connection error: {}", e);
        }
    });
    let migration_report = embedded::migrations::runner()
        .run_async(&mut client)
        .await?;
    for migration in migration_report.applied_migrations() {
        println!(
            "Migration Applied -  Name: {}, Version: {}",
            migration.name(),
            migration.version()
        );
    }
    println!("DB migrations finished!");

    Ok(())
}

#[tokio::main]
async fn main() {
    run_migrations().await.expect("can't run DB migrations: {}");
}
