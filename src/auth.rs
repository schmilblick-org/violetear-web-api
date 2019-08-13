use actix_web::{http::header, web, Error as AWError, HttpRequest, HttpResponse};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    Connection, PgConnection,
};
use futures::{
    future::{ok, Either},
    Future,
};
use serde_derive::{Deserialize, Serialize};

use crate::models;

#[derive(Serialize, Deserialize)]
pub struct Register {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct RegisterResponse {
    token: Option<String>,
}

pub fn register(
    db: web::Data<Pool<ConnectionManager<PgConnection>>>,
    register: web::Json<Register>,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    web::block(move || {
        let conn = &db.get().unwrap();

        conn.transaction(|| {
            let user_id = models::User::create(conn, &register.username, &register.password, 0)?;
            let token = models::Token::generate(conn, user_id)?;

            Ok(token)
        })
    })
    .and_then(|token| Ok(HttpResponse::Ok().json(RegisterResponse { token: Some(token) })))
    .or_else(
        |_: actix_web::error::BlockingError<diesel::result::Error>| {
            Ok(HttpResponse::Conflict().finish())
        },
    )
}

#[derive(Serialize, Deserialize)]
pub struct Login {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct LoginResponse {
    token: Option<String>,
}

pub fn login(
    db: web::Data<Pool<ConnectionManager<PgConnection>>>,
    login: web::Json<Login>,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    web::block(move || {
        let conn = &db.get().unwrap();

        conn.transaction(|| {
            let is_valid = models::User::verify_password(conn, &login.username, &login.password)?;
            if is_valid {
                let user = models::User::by_username(conn, &login.username)?;

                Ok((models::Token::generate(conn, user.id)?, is_valid))
            } else {
                Ok(("".into(), is_valid))
            }
        })
    })
    .map(|(token, is_valid)| {
        if is_valid {
            HttpResponse::Ok().json(LoginResponse { token: Some(token) })
        } else {
            HttpResponse::Unauthorized().finish()
        }
    })
    .or_else(
        |e: actix_web::error::BlockingError<diesel::result::Error>| match e {
            actix_web::error::BlockingError::Error(diesel::result::Error::NotFound) => {
                Ok(HttpResponse::Unauthorized().finish())
            }
            _ => Ok(HttpResponse::InternalServerError().finish()),
        },
    )
}

pub fn logout(
    req: HttpRequest,
    db: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    if let Some(token) = &req.headers().get(header::AUTHORIZATION) {
        let token = token.to_str().unwrap().to_string();

        Either::A(
            web::block(move || {
                let conn = &db.get().unwrap();

                conn.transaction(|| {
                    if models::Token::user_by_token(conn, &token).is_ok() {
                        models::Token::destroy(conn, &token)?;
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                })
            })
            .and_then(|is_authenticated| {
                if is_authenticated {
                    Ok(HttpResponse::Ok().finish())
                } else {
                    Ok(HttpResponse::Unauthorized().finish())
                }
            })
            .or_else(
                |_: actix_web::error::BlockingError<diesel::result::Error>| {
                    Ok(HttpResponse::InternalServerError().finish())
                },
            ),
        )
    } else {
        Either::B(ok(HttpResponse::Unauthorized().finish()))
    }
}
