use crate::{error_handler::ServiceError, airports::{InsertAirport, QueryAirport}};
use diesel::{r2d2::ConnectionManager, PgConnection};
use serde::{Deserialize, Serialize};
use crate::diesel_migrations::MigrationHarness;
use lazy_static::lazy_static;
use log::{error, debug, info};
use r2d2;
use std::env;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub const MIGRATIONS: diesel_migrations::EmbeddedMigrations = embed_migrations!();

lazy_static! {
  static ref POOL: Pool = {
    let username = env::var("DATABASE_USER").expect("Database username is not set");
    let password = env::var("DATABASE_PASSWORD").expect("Database password is not set");
    let host = env::var("DATABASE_HOST").expect("Database host is not set");
    let name = env::var("DATABASE_NAME").expect("Database name is not set");
    // let port = env::var("DATABASE_PORT").expect("Database port is not set");
    let port = 5432;
    let url = format!("postgres://{}:{}@{}:{}/{}", username, password, host, port, name);
    let manager = ConnectionManager::<PgConnection>::new(url);
    Pool::builder().test_on_check_out(true).build(manager).expect("Failed to create db pool")
  };
}

pub fn init() {
  lazy_static::initialize(&POOL);
  let mut pool: DbConnection = connection().expect("Failed to get db connection");
  match pool.run_pending_migrations(MIGRATIONS) {
    Ok(_) => info!("Database initialized"),
    Err(err) => error!("Failed to initialize database; {}", err)
  };
}

pub fn connection() -> Result<DbConnection, ServiceError> {
  POOL.get()
    .map_err(|e| ServiceError::new(500, format!("Failed getting db connection: {}", e)))
}

pub fn import_data() {
  let path = "airport-codes.json";
  debug!("Importing data from {}", path);
  let contents: String = std::fs::read_to_string(path).expect("Failed to read file");
  let airports: Vec<InsertAirport> = serde_json::from_str(&contents).expect("JSON was not well formed.");
  for airport in airports {
    match QueryAirport::create(airport) {
      Ok(_) => {},
      Err(err) => error!("Error inserting airport; {}", err)
    };
  }
  debug!("Import complete");
}

#[derive(Serialize, Deserialize)]
pub struct Metadata {
  pub page: i32,
  pub limit: i32,
  pub pages: i32,
  pub total: i32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Coordinate {
  pub lon: f64,
  pub lat: f64
}
