use actix_web::{HttpRequest, HttpResponse, Responder, web};
use common::User;
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};

type Error = actix_web::Error;
type Future = Ready<Result<HttpResponse, Error>>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Users {
    pub(crate) users: Vec<User>,
}

impl Responder for Users {
    fn respond_to(self, _: &HttpRequest) -> HttpResponse {
        let body = serde_json::to_string(&self);
        match body {
            Ok(body) => HttpResponse::Ok()
                .content_type("application/json")
                .body(body),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}
