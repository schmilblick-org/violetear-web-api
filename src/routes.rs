use actix_web::{get, post, web, Error as AWError, HttpResponse};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::r2d2::{self, ConnectionManager, Pool};
use diesel::PgConnection;
use futures::{future::ok, Future};
use rusqlite::{ToSql, NO_PARAMS};
use serde_derive::{Deserialize, Serialize};

use crate::models;

fn index() -> impl Future<Item = HttpResponse, Error = AWError> {
    ok(HttpResponse::Ok().body("You have reached a Violetear Web API."))
}

#[derive(Serialize, Deserialize)]
struct Register {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct RegisterResponse {
    token: Option<String>,
}

fn register(
    db: web::Data<Pool<ConnectionManager<PgConnection>>>,
    register: web::Json<Register>,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    web::block(move || {
        let conn = &db.get().unwrap();

        let user_id = models::User::create(conn, &register.username, &register.password, 0)?;
        let token = models::Token::generate(conn, user_id)?;

        Ok(token)
    }).map(|token| HttpResponse::Ok().json(RegisterResponse { token: Some(token)})
    ).map_err(|_| HttpResponse::Conflict().json(RegisterResponse { token: None }))
}

#[derive(Serialize, Deserialize)]
struct Login {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct LoginResponse {
    token: Option<String>,
}
/*
fn login(db: web::Data<Pool<SqliteConnectionManager>>, login: web::Json<Login>) -> HttpResponse {
    let conn = db.get().unwrap();

    match conn.query_row(
        "SELECT rowid,password FROM users WHERE username = $1",
        &[&login.username],
        |row| match (row.get::<_, i64>(0), row.get::<_, String>(1)) {
            (Err(x), _) => Err(x),
            (_, Err(x)) => Err(x),
            (Ok(x), Ok(y)) => Ok((x, y)),
        },
    ) {
        Ok((rowid, ref hashed)) if verify(&login.password, &hashed).unwrap() => {
            let token = uuid::Uuid::new_v4().to_simple().to_string().to_lowercase();

            conn.execute(
                "INSERT INTO tokens (user_id, token) VALUES ($1, $2)",
                &[&rowid as &ToSql, &token],
            )
            .unwrap();
            HttpResponse::Ok().json(LoginResponse { token: Some(token) })
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            HttpResponse::Unauthorized().json(LoginResponse { token: None })
        }
        Err(_) | Ok(_) => HttpResponse::InternalServerError().json(LoginResponse { token: None }),
    }
}

#[derive(Serialize, Deserialize)]
struct Logout {
    token: String,
}

#[derive(Serialize, Deserialize)]
struct LogoutResponse {}

#[post("/logout")]
fn logout(db: web::Data<Pool<SqliteConnectionManager>>, logout: web::Json<Logout>) -> HttpResponse {
    let conn = db.get().unwrap();
    conn.execute("DELETE FROM tokens WHERE token = $1", &[&logout.token])
        .unwrap();
    HttpResponse::Ok().json(LogoutResponse {})
}
*/
