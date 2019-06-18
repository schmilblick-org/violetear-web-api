use actix_web::{http::header, middleware, App, HttpServer};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::NO_PARAMS;
use std::env;
use actix_cors::Cors;

mod routes;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    let args = clap::App::new("Violetear Web API")
        .arg(
            clap::Arg::with_name("listen-address")
                .short("l")
                .takes_value(true)
                .number_of_values(1)
                .default_value("0.0.0.0"),
        )
        .arg(
            clap::Arg::with_name("listen-port")
                .short("p")
                .takes_value(true)
                .number_of_values(1),
        )
        .get_matches();

    let manager = SqliteConnectionManager::file("web-api.db");
    let pool = r2d2::Pool::new(manager).unwrap();

    let conn = pool.get().unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (username TEXT, password TEXT, rank INTEGER)",
        NO_PARAMS,
    )
    .unwrap();

    conn.execute("CREATE TABLE IF NOT EXISTS tokens (user_id INTEGER, token TEXT, created_when DATETIME DEFAULT CURRENT_TIMESTAMP)", NO_PARAMS).unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_origin(&env::var("CORS_ORIGIN").unwrap_or_else(|_| "http://[::1]:8000".into()))
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![
                        header::AUTHORIZATION,
                        header::ACCEPT,
                        header::CONTENT_TYPE,
                        header::ACCEPT_ENCODING,
                        header::ACCEPT_LANGUAGE,
                    ])
                    .max_age(3600),
            )
            .data(pool.clone())
            .wrap(middleware::DefaultHeaders::new())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(routes::index)
            .service(routes::register)
            .service(routes::login)
            .service(routes::logout)
    })
    .bind((
        args.value_of("listen-address")
            .expect("listen-address argument missing"),
        (&env::var("PORT").unwrap_or_else(|_| {
            args.value_of("listen-port")
                .expect("listen-port argument missing")
                .to_owned()
        }))
            .parse()
            .expect("listen-port argument invalid"),
    ))?
    .run()
}
