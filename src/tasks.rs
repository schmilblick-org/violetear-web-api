use actix_web::error::{BlockingError, PayloadError};
use actix_web::{http::header, web, Error as AWError, HttpRequest, HttpResponse};
use bytes::BytesMut;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    Connection, PgConnection,
};
use futures::{
    future::{ok, Either},
    Future, IntoFuture, Stream,
};
use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Serialize)]
pub struct ListResponse {
    pub tasks: Vec<models::Task>,
}

#[derive(Deserialize)]
pub struct ByIdPath {
    pub report_id: i64,
}

pub fn list(
    req: HttpRequest,
    path: web::Path<ByIdPath>,
    db: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    if let Some(token) = req.headers().get(header::AUTHORIZATION) {
        let token = token.to_str().unwrap().to_string();

        Either::A(
            web::block(move || {
                let conn = &db.get().unwrap();

                let user = models::Token::user_by_token(conn, &token)?;
                let report = models::Report::by_id_check_user(conn, path.report_id, user.id)?;

                Ok((db, report))
            })
            .map_err(
                |_: actix_web::error::BlockingError<diesel::result::Error>| {
                    actix_web::error::ErrorInternalServerError(String::new())
                },
            )
            .and_then(move |(db, report)| {
                web::block(move || models::Task::list_for_report(&db.get().unwrap(), report.id))
                    .map_err(
                        |_: actix_web::error::BlockingError<diesel::result::Error>| {
                            actix_web::error::ErrorInternalServerError(String::new())
                        },
                    )
                    .and_then(|tasks| HttpResponse::Ok().json(ListResponse { tasks }))
                    .or_else(|_| HttpResponse::InternalServerError().finish())
            })
            .or_else(|_| HttpResponse::Unauthorized().finish()),
        )
    } else {
        Either::B(ok(HttpResponse::Unauthorized().finish()))
    }
}
