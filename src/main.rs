use actix_web::{middleware, App, HttpServer};

mod routes;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    let args = clap::App::new("Violetear Web API")
        .arg(
            clap::Arg::with_name("listen-address")
                .short("l")
                .takes_value(true)
                .number_of_values(1),
        )
        .arg(
            clap::Arg::with_name("listen-port")
                .short("p")
                .takes_value(true)
                .number_of_values(1),
        )
        .get_matches();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(routes::index)
    })
    .bind((
        args.value_of("listen-address")
            .expect("listen-address argument missing"),
        args.value_of("listen-port")
            .expect("listen-port argument missing")
            .parse()
            .expect("listen-port argument invalid"),
    ))?
    .run()
}