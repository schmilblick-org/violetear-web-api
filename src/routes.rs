use actix_web::{Error as AWError, HttpResponse};
use futures::{future::ok, Future};

pub fn index() -> impl Future<Item = HttpResponse, Error = AWError> {
    ok(HttpResponse::Ok().body("You have reached a Violetear Web API."))
}
