use std::sync::Mutex;

use actix_web::{HttpRequest, HttpResponse, Responder, web};
use actix_web::body::Body;
use common::User;
use deadpool_postgres::tokio_postgres::Error;
use futures::future::{ready, Ready};
use log::error;

use crate::accessors::db;
use crate::server::model::Users;

pub async fn get_users(data: web::Data<Mutex<&db::Db>>) -> impl Responder {
    let mut data = data.lock().unwrap();
    match data.get_users().await {
        Ok(users) => {
            let usrs = Users {
                users
            };
            usrs
        }
        Err(err) => {
            error!("Error in get  users {}", err);
            Users { users: vec![] }
        }
    }
}

pub async fn get_user(
    path: web::Path<String>,
    data: web::Data<Mutex<&db::Db>>) -> HttpResponse<Body> {
    let id: i32 = path.into_inner().parse().unwrap();
    let mut data = data.lock().unwrap();
    match data.get_user(id).await {
        Ok(user) => {
            HttpResponse::Ok().json(user)
        }
        Err(err) => {
            eprintln!("Error in get  user with id{} ann error {}", id, err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}
