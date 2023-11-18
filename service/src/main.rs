extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use std::env;

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger};
use dotenv::dotenv;
use log::{info, error};

mod airports;
mod auth;
mod db;
mod error_handler;
mod metars;
mod users;
mod scheduler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();
  env_logger::init_from_env(env_logger::Env::default().filter_or("RUST_LOG", "warn,service=info"));
  db::init();
  scheduler::update_airports();

  let host = env::var("SERVICE_HOST").unwrap_or("localhost".to_string());
  let port = env::var("SERVICE_PORT").unwrap_or("5000".to_string());

  let server = match HttpServer::new(move || {
    let cors = Cors::default()
      .allow_any_origin()
      .allow_any_method()
      .allow_any_header()
      .supports_credentials()
      .max_age(3600);
    App::new()
      .wrap(cors)
      .wrap(Logger::default())
      .configure(airports::init_routes)
      .configure(metars::init_routes)
      .configure(auth::init_routes)
      .configure(users::init_routes)
  })
  .bind(format!("{}:{}", host, port)) {
    Ok(b) => {
      info!("Binding server to {}:{}", host, port);
      b
    },
    Err(err) => {
      error!("Could not bind server: {}", err);
      return Err(err);
    }
  };

  server.run().await
}
