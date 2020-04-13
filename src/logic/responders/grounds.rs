use crate::logic::models::ground::Ground;
use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
    pub items: Vec<Ground>,
}

impl Responder for Data {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .header("Access-Control-Allow-Origin", "*")
            .body(body)))
    }
}
