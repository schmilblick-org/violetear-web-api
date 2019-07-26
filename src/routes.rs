use actix_web::{get, post, web, Error as AWError, HttpResponse};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::r2d2::{self, ConnectionManager, Pool};
use diesel::PgConnection;
use futures::{future::ok, Future};
use serde_derive::{Deserialize, Serialize};

use crate::models;

pub fn index() -> impl Future<Item = HttpResponse, Error = AWError> {
    ok(HttpResponse::Ok().body("You have reached a Violetear Web API."))
}

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

        let user_id = models::User::create(conn, &register.username, &register.password, 0)?;
        let token = models::Token::generate(conn, user_id)?;

        Ok(token)
    })
    .map(|token| HttpResponse::Ok().json(RegisterResponse { token: Some(token) }))
    .or_else(
        |_: actix_web::error::BlockingError<diesel::result::Error>| {
            HttpResponse::Conflict().json(RegisterResponse { token: None })
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

        let is_valid = models::User::verify_password(conn, &login.username, &login.password)?;
        let user = models::User::by_username(conn, &login.username)?;

        models::Token::generate(conn, user.id)
    })
    .map(|token| HttpResponse::Ok().json(LoginResponse { token: Some(token) }))
    .or_else(|_| HttpResponse::Unauthorized().json(LoginResponse { token: None }))
}

#[derive(Serialize, Deserialize)]
pub struct Logout {
    token: String,
}

#[derive(Serialize, Deserialize)]
struct LogoutResponse {}

pub fn logout(
    db: web::Data<Pool<ConnectionManager<PgConnection>>>,
    logout: web::Json<Logout>,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    web::block(move || models::Token::destroy(&db.get().unwrap(), &logout.token))
        .map(|_| HttpResponse::Ok().json(LogoutResponse {}))
        .or_else(|_| HttpResponse::Ok().json(LogoutResponse {}))
}
