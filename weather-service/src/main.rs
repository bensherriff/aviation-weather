extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger};
use dotenv::dotenv;
use env_logger::Env;
use listenfd::ListenFd;
use log::debug;
use std::env;

mod airports;
mod db;
mod error_handler;
mod metars;
mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info,actix=info,diesel_migrations=warn,reqwest=warn,hyper=warn");
    }
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    db::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        App::new()
        .configure(airports::init_routes)
        .configure(metars::init_routes)
        .wrap(cors)
        .wrap(Logger::default())
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Please set host in .env");
            let port = env::var("PORT").expect("Please set port in .env");
            debug!("Binding server to {}:{}", host, port);
            server.bind(format!("{}:{}", host, port))?
        }
    };
    server.run().await
}
