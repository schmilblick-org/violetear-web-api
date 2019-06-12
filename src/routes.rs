use actix_web::{get, post, web, HttpResponse};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use rusqlite::{ToSql, NO_PARAMS};
use serde_derive::{Deserialize, Serialize};
#[get("/")]
fn index() -> &'static str {
    "You have reached a Violetear Web API."
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

#[post("/register")]
fn register(
    db: web::Data<Pool<SqliteConnectionManager>>,
    register: web::Json<Register>,
) -> HttpResponse {
    let conn = db.get().unwrap();

    if conn
        .execute(
            "SELECT username FROM users WHERE username = $1",
            &[&register.username],
        )
        .unwrap()
        > 0
    {
        HttpResponse::Conflict().json(RegisterResponse { token: None })
    } else {

        conn.execute("BEGIN", NO_PARAMS).unwrap();
        conn.execute(
            "INSERT INTO users (username, password) VALUES ($1, $2)",
            &[&register.username, &register.password],
        )
        .unwrap();

        let token = uuid::Uuid::new_v4().to_simple().to_string().to_lowercase();

        conn.execute(
            "INSERT INTO tokens (user_id, token) VALUES ($1, $2)",
            &[&conn.last_insert_rowid() as &ToSql, &token],
        )
        .unwrap();
        conn.execute("COMMIT", NO_PARAMS).unwrap();

        HttpResponse::Ok().json(RegisterResponse { token: Some(token) })
    }
}

