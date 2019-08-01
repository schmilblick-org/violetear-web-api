use actix_web::{http::header, web, Error as AWError, HttpRequest, HttpResponse};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use futures::{
    future::{ok, Either},
    Future,
};
use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Serialize, Deserialize)]
pub struct ListResponse {
    profiles: Vec<models::Profile>,
}

pub fn list(
    req: HttpRequest,
    db: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    if let Some(token) = req.headers().get(header::AUTHORIZATION) {
        let token = token.to_str().unwrap().to_string();

        Either::A(
            web::block(move || {
                models::Token::user_by_token(&db.get().unwrap(), &token)?;

                Ok(db)
            })
            .and_then(move |db| {
                web::block(move || models::Profile::list(&db.get().unwrap()))
                    .map(|profiles| HttpResponse::Ok().json(ListResponse { profiles }))
            })
            .or_else(|_| HttpResponse::InternalServerError().finish()),
        )
    } else {
        Either::B(ok(HttpResponse::Unauthorized().finish()))
    }
}
