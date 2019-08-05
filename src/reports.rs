use actix_web::{Error as AWError, http::header, HttpRequest, HttpResponse, web};
use actix_web::error::PayloadError;
use actix_web::web::Payload;
use bytes::{Bytes, BytesMut};
use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use futures::{
    future::{Either, ok},
    Future, Stream,
};
use futures::stream::Fold;
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
    profiles: Vec<i64>,
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
) -> impl Future<Item=HttpResponse, Error=AWError> {
    if let Some(token) = req.headers().get(header::AUTHORIZATION) {
        let token = token.to_str().unwrap().to_string();

        Either::A(
            web::block(move || {
                let user = models::Token::user_by_token(&db.get().unwrap(), &token)?;

                Ok((db, user))
            })
                .and_then(|(db, user)| {
                    payload
                        .fold(bytes::BytesMut::new(), |mut body, chunk| {
                            body.extend_from_slice(&chunk);
                            Ok(body)
                        })
                        .and_then(|body| {
                            web::block(move || {
                                let report_id =
                                    models::Report::create(&db.get().unwrap(), user.id, body.to_vec())?;

                                for profile_id in query.profiles {
                                    models::Task::create(&db.get().unwrap(), report_id, profile_id)?;
                                }

                                Ok(report_id)
                            })
                                .and_then(|report_id| {
                                    Ok(HttpResponse::Ok().json(CreateResponse { report_id }))
                                })
                                .or_else(|_| Ok(HttpResponse::InternalServerError().finish()))
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

/*
pub fn create(
    req: HttpRequest,
    query: web::Query<CreateQuery>,
    payload: web::Payload,
    db: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse, AWError> {
    if let Some(token) = req.headers().get(header::AUTHORIZATION) {
        let user = match models::Token::user_by_token(&db.get().unwrap(), token.to_str().unwrap()) {
            Ok(user) => user,
            Err(diesel::result::Error::NotFound) => {
                return Ok(HttpResponse::Unauthorized().finish())
            }
            Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
        };

        models::Report::create(&db.get().unwrap(), user.id, )

        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::Unauthorized().finish())
    }
}
*/
