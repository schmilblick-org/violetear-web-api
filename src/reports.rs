use actix_web::error::PayloadError;
use actix_web::{http::header, web, Error as AWError, HttpRequest, HttpResponse};
use bytes::BytesMut;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    Connection, PgConnection, RunQueryDsl,
};
use futures::{
    future::{ok, Either},
    Future, Stream,
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

#[derive(Deserialize)]
pub struct CreateQuery {
    profiles: String,
}

const MAX_SIZE: usize = 104_857_600;

#[derive(Serialize)]
pub struct CreateResponse {
    report_id: i64,
}

pub fn create(
    req: HttpRequest,
    query: web::Query<CreateQuery>,
    payload: web::Payload,
    db: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    if let Some(token) = req.headers().get(header::AUTHORIZATION) {
        let token = token.to_str().unwrap().to_string();

        Either::A(
            web::block(move || {
                let user = models::Token::user_by_token(&db.get().unwrap(), &token)?;

                Ok((db, user))
            })
            .and_then(|(db, user)| {
                payload
                    .fold(
                        BytesMut::new(),
                        |mut body, chunk| -> Result<_, PayloadError> {
                            if body.len() + chunk.len() > MAX_SIZE {
                                Err(PayloadError::Overflow)
                            } else {
                                body.extend_from_slice(&chunk);
                                Ok(body)
                            }
                        },
                    )
                    .and_then(|body| {
                        web::block(move || {
                            let conn = &db.get().unwrap();

                            let trans = conn.transaction(|| -> Result<_, diesel::result::Error> {
                                let report_id =
                                    models::Report::create(conn, user.id, body.to_vec())?;

                                let mut profile_names: Vec<&str> =
                                    query.profiles.split(',').collect();

                                profile_names.dedup();

                                for profile_machine_name in profile_names {
                                    let profile_id = models::Profile::id_for_machine_name(
                                        conn,
                                        profile_machine_name,
                                    )?;

                                    models::Task::create(conn, report_id, profile_id)?;
                                }

                                Ok(report_id)
                            });

                            conn.execute("NOTIFY tasks_created").unwrap();

                            trans
                        })
                        .and_then(|report_id| {
                            Ok(HttpResponse::Ok().json(CreateResponse { report_id }))
                        })
                        .or_else(
                            |e: actix_web::error::BlockingError<diesel::result::Error>| {
                                Ok(HttpResponse::InternalServerError().finish())
                            },
                        )
                    })
                    .or_else(|_| Ok(HttpResponse::InternalServerError().finish()))
            })
            .or_else(
                |e: actix_web::error::BlockingError<diesel::result::Error>| match e {
                    actix_web::error::BlockingError::Error(diesel::result::Error::NotFound) => {
                        Ok(HttpResponse::Unauthorized().finish())
                    }
                    _ => Ok(HttpResponse::InternalServerError().finish()),
                },
            ),
        )
    } else {
        Either::B(ok(HttpResponse::Unauthorized().finish()))
    }
}

#[derive(Deserialize)]
pub struct ByIdPath {
    pub report_id: i64,
}

pub fn by_id(
    req: HttpRequest,
    path: web::Path<ByIdPath>,
    db: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    if let Some(token) = req.headers().get(header::AUTHORIZATION) {
        let token = token.to_str().unwrap().to_string();

        Either::A(
            web::block(move || {
                let user = models::Token::user_by_token(&db.get().unwrap(), &token)?;

                Ok((db, user))
            })
            .and_then(|(db, user)| {
                web::block(move || {
                    models::Report::by_id_check_user(&db.get().unwrap(), path.report_id, user.id)
                })
                .and_then(|report| Ok(HttpResponse::Ok().json(report)))
                .or_else(
                    |e: actix_web::error::BlockingError<diesel::result::Error>| match e {
                        actix_web::error::BlockingError::Error(diesel::result::Error::NotFound) => {
                            Ok(HttpResponse::NotFound().finish())
                        }
                        _ => Ok(HttpResponse::InternalServerError().finish()),
                    },
                )
            })
            .or_else(
                |e: actix_web::error::BlockingError<diesel::result::Error>| match e {
                    actix_web::error::BlockingError::Error(diesel::result::Error::NotFound) => {
                        Ok(HttpResponse::Unauthorized().finish())
                    }
                    _ => Ok(HttpResponse::InternalServerError().finish()),
                },
            ),
        )
    } else {
        Either::B(ok(HttpResponse::Unauthorized().finish()))
    }
}

pub fn discard_file(
    req: HttpRequest,
    path: web::Path<ByIdPath>,
    db: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    if let Some(token) = req.headers().get(header::AUTHORIZATION) {
        let token = token.to_str().unwrap().to_string();

        Either::A(
            web::block(move || {
                let user = models::Token::user_by_token(&db.get().unwrap(), &token)?;

                Ok((db, user))
            })
            .and_then(|(db, user)| {
                web::block(move || {
                    models::Report::discard_file_check_user(
                        &db.get().unwrap(),
                        path.report_id,
                        user.id,
                    )
                })
                .and_then(|report| Ok(HttpResponse::Ok().finish()))
                .or_else(
                    |e: actix_web::error::BlockingError<diesel::result::Error>| match e {
                        actix_web::error::BlockingError::Error(diesel::result::Error::NotFound) => {
                            Ok(HttpResponse::NotFound().finish())
                        }
                        _ => Ok(HttpResponse::InternalServerError().finish()),
                    },
                )
            })
            .or_else(
                |e: actix_web::error::BlockingError<diesel::result::Error>| match e {
                    actix_web::error::BlockingError::Error(diesel::result::Error::NotFound) => {
                        Ok(HttpResponse::Unauthorized().finish())
                    }
                    _ => Ok(HttpResponse::InternalServerError().finish()),
                },
            ),
        )
    } else {
        Either::B(ok(HttpResponse::Unauthorized().finish()))
    }
}
