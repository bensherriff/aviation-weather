use std::env;

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger, web};
use dotenv::from_filename;
use moka::future::Cache;
use crate::auth::hash;
use crate::users::{User, ADMIN_ROLE};

mod airports;
mod auth;
mod db;
mod error;
mod metars;
mod scheduler;
mod users;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  initialize_environment()?;
  db::initialize().await?;
  // scheduler::update_airports();

  let host = "0.0.0.0".to_string();
  let port = env::var("API_PORT").unwrap_or("5000".to_string());

  // Initialize admin user
  let admin_email = env::var("ADMIN_EMAIL");
  let admin_password = env::var("ADMIN_PASSWORD");
  if admin_email.is_ok() && admin_password.is_ok() {
    let email = admin_email.unwrap();
    if User::select(&email).await.is_none() {
      log::debug!("Creating default administrator");
      let password = admin_password.unwrap();
      let password_hash = hash(&password)?;
      if email == "admin@example.com" || password == "CHANGEME" {
        log::warn!(
          "Default admin credentials are in use, update the ADMIN_EMAIL and ADMIN_PASSWORD."
        );
      }
      let admin_user = User {
        email,
        password_hash,
        role: ADMIN_ROLE.to_string(),
        first_name: "Admin".to_string(),
        last_name: "".to_string(),
        updated_at: Default::default(),
        created_at: Default::default(),
      };
      match admin_user.insert().await {
        Ok(_) => log::debug!("Default administrator was successfully created"),
        Err(err) => {
          log::warn!("{}", err);
        }
      };
    }
  }

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
      .service(web::scope("api")
        .configure(airports::init_routes)
        .configure(metars::init_routes)
        .configure(auth::init_routes)
        .configure(users::init_routes))
  })
  .bind(format!("{}:{}", host, port))
  {
    Ok(b) => {
      log::info!("Binding server to {}:{}", host, port);
      b
    }
    Err(err) => {
      log::error!("Could not bind server: {}", err);
      return Err(err.into());
    }
  };

  if let Err(err) = server.run().await {
    return Err(err.into());
  }
  Ok(())
}

fn initialize_environment() -> std::io::Result<()> {
  // Iterate over files in the current directory
  for entry in std::fs::read_dir(".")? {
    let entry = entry?;
    let path = entry.path();

    // Check if the file name starts with ".env" and is a file
    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
      if file_name.starts_with(".env") && path.is_file() {
        // Try to load the file
        if let Err(err) = from_filename(&file_name) {
          eprintln!("Failed to load {}: {}", file_name, err);
        } else {
          println!("Loaded: {}", file_name);
        }
      }
    }
  }

  env_logger::init_from_env(env_logger::Env::default().filter_or("RUST_LOG", "warn,api=info"));
  Ok(())
}
