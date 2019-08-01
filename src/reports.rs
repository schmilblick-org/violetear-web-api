use actix_web::{http::header, web, Error as AWError, HttpRequest, HttpResponse};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use futures::{
    future::{ok, Either},
    Future,Stream
};
use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Serialize, Deserialize)]
pub struct ListResponse {
    reports: Vec<models::Report>,
}

pub fn list(
    req: HttpRequest,
    db: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    if let Some(token) = req.headers().get(header::AUTHORIZATION) {
        let token = token.to_str().unwrap().to_string();

        Either::A(
            web::block(move || {
                let user = models::Token::user_by_token(&db.get().unwrap(), &token)?;

                Ok((db, user))
            })
            .and_then(move |(db, user)| {
                web::block(move || models::Report::list_for_user(&db.get().unwrap(), user.id))
                    .map(|reports| HttpResponse::Ok().json(ListResponse { reports }))
            })
            .or_else(|_| HttpResponse::InternalServerError().finish()),
        )
    } else {
        Either::B(ok(HttpResponse::Unauthorized().finish()))
    }
}
