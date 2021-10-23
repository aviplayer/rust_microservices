use std::borrow::Borrow;

use actix_web::{App, HttpServer, middleware, web};
use log::info;

use crate::accessors::db;
use std::sync::Mutex;
use num_cpus;
use crate::server::handlers::get_users;
use crate::server::routes::Router;

pub async fn start_server(host: &str, port: i16, db_worker: &'static db::Db) -> std::io::Result<()> {
    let server_address = format!("{}:{}", host, port);
    let cores = num_cpus::get();
    info!("Server is running {} on cores {}", server_address, cores);
    let storage = web::Data::new(Mutex::new(db_worker));
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(storage.clone())
            .configure(Router::init_routes)
    })
        .workers(cores - 3)
        .bind(server_address)?
        .run()
        .await
}
