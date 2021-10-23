use actix_web::web;

use crate::server::handlers::{get_user, get_users};

pub struct Router {}

impl Router {
    pub fn init_routes(config: &mut web::ServiceConfig) {
        config
            .route("/users", web::get().to(get_users))
            .route("/user/{id}", web::get().to(get_user));
    }
}


