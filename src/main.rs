#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::{http::header, middleware, web, App, HttpServer};

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

mod routes;

mod models;
mod schema;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    dotenv().ok();

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

    let manager = ConnectionManager::<PgConnection>::new(
        env::var("DATABASE_URL").expect("incomplete database configuration"),
    );

    let pool = r2d2::Pool::new(manager).unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_origin(
                        &env::var("CORS_ORIGIN").unwrap_or_else(|_| "http://[::1]:8000".into()),
                    )
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
            .service(web::resource("/").route(web::get().to_async(routes::index)))
            .service(
                web::scope("/v1")
                    .service(
                        web::resource("/login")
                            .data(web::JsonConfig::default().limit(4096))
                            .route(web::post().to_async(routes::login)),
                    )
                    .service(
                        web::resource("/register")
                            .data(web::JsonConfig::default().limit(4096))
                            .route(web::post().to_async(routes::register)),
                    )
                    .service(
                        web::resource("/logout")
                            .data(web::JsonConfig::default().limit(4096))
                            .route(web::post().to_async(routes::logout)),
                    ),
            )
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
