use crate::{error_handler::ServiceError, airports::{QueryAirport, Airport}};
use diesel::{r2d2::ConnectionManager, PgConnection};
use redis::{Client as RedisClient, aio::Connection as RedisConnection};
use serde::{Deserialize, Serialize};
use crate::diesel_migrations::MigrationHarness;
use lazy_static::lazy_static;
use log::{error, debug, info};
use r2d2;
use std::env;

pub mod schema;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub const MIGRATIONS: diesel_migrations::EmbeddedMigrations = embed_migrations!();

lazy_static! {
  static ref POOL: Pool = {
    let username = env::var("DATABASE_USER").expect("Database username is not set");
    let password = env::var("DATABASE_PASSWORD").expect("Database password is not set");
    let host = env::var("DATABASE_HOST").unwrap_or("localhost".to_string());
    let name = env::var("DATABASE_NAME").expect("Database name is not set");
    let port = env::var("DATABASE_PORT").unwrap_or("5432".to_string());
    let url = format!("postgres://{}:{}@{}:{}/{}", username, password, host, port, name);
    let manager = ConnectionManager::<PgConnection>::new(url);
    Pool::builder().test_on_check_out(true).build(manager).expect("Failed to create db pool")
  };
  static ref REDIS: RedisClient = {
    let host = env::var("REDIS_HOST").unwrap_or("localhost".to_string());
    let port = env::var("REDIS_PORT").unwrap_or("6379".to_string());
    let url = format!("redis://{}:{}", host, port);
    RedisClient::open(url).expect("Failed to create redis client")
  };
}

pub fn init() {
  lazy_static::initialize(&POOL);
  lazy_static::initialize(&REDIS);
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

pub fn redis_connection() -> Result<redis::Connection, ServiceError> {
  let conn = REDIS.get_connection()?;
  Ok(conn)
}

pub async fn redis_async_connection() -> Result<RedisConnection, ServiceError> {
  let conn = REDIS.get_async_connection().await?;
  Ok(conn)
}

pub fn import_data() -> i32 {
  let path = "airport-codes.json";
  debug!("Importing data from {}", path);
  let contents: String = std::fs::read_to_string(path).expect("Failed to read file");
  let airports: Vec<Airport> = serde_json::from_str(&contents).expect("JSON was not well formed.");
  let mut count = 0;
  for airport in airports {
    match QueryAirport::insert(airport.into()) {
      Ok(_) => count += 1,
      Err(err) => error!("Error inserting airport; {}", err)
    };
  }
  debug!("Import complete");
  return count;
}

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
  pub data: T,
  pub meta: Option<Metadata>
}

#[derive(Serialize, Deserialize)]
pub struct Metadata {
  pub page: i32,
  pub limit: i32,
  pub pages: i64,
  pub total: i64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Coordinate {
  pub lon: f64,
  pub lat: f64
}
