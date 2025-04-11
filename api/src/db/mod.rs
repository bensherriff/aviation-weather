use crate::error::ApiResult;
use redis::{Client as RedisClient, aio::MultiplexedConnection as RedisConnection, RedisResult};
use s3::{
  Bucket, Region, creds::Credentials, BucketConfiguration, request::ResponseData,
  bucket_ops::CreateBucketResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use std::time::Duration;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

static POOL: OnceLock<Pool<Postgres>> = OnceLock::new();
static REDIS: OnceLock<RedisClient> = OnceLock::new();
static BUCKET: OnceLock<Bucket> = OnceLock::new();

pub async fn initialize() -> ApiResult<()> {
  let db_user = std::env::var("POSTGRES_USER").unwrap_or("aviation".to_string());
  let db_password = std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
  let db_host: String = std::env::var("POSTGRES_HOST").expect("POSTGRES_HOST must be set");
  let db_port = std::env::var("POSTGRES_PORT").unwrap_or("5432".to_string());
  let db_name = std::env::var("POSTGRES_NAME").unwrap_or("aviation".to_string());

  let db_url = format!(
    "postgres://{}:{}@{}:{}/{}",
    &db_user, &db_password, &db_host, &db_port, &db_name
  );

  log::info!(
    "Connecting to database at postgres://{}:*****@{}:{}/{}...",
    &db_user,
    &db_host,
    &db_port,
    &db_name
  );
  // Setup Postgres pool connection
  let pool = PgPoolOptions::new()
    .max_connections(5)
    .acquire_timeout(Duration::from_secs(30))
    .connect(&db_url)
    .await?;
  match POOL.set(pool) {
    Ok(_) => log::info!("Database connection established"),
    Err(_) => log::warn!("Database pool already initialized"),
  }

  // Setup Redis connection
  let redis = {
    let host = std::env::var("REDIS_HOST").unwrap_or("localhost".to_string());
    let port = std::env::var("REDIS_PORT").unwrap_or("6379".to_string());
    let url = format!("redis://{}:{}", host, port);
    RedisClient::open(url).expect("Failed to create redis client")
  };
  match REDIS.set(redis) {
    Ok(_) => log::info!("Redis connection established"),
    Err(_) => log::warn!("Redis client already initialized"),
  }

  let schema = std::env::var("MINIO_SCHEMA").unwrap_or("http".to_string());
  let url = std::env::var("MINIO_HOST").unwrap_or("localhost".to_string());
  let port = std::env::var("MINIO_PORT").unwrap_or("9000".to_string());
  let user = std::env::var("MINIO_ROOT_USER").expect("MINIO_ROOT_USER is not set");
  let password = std::env::var("MINIO_ROOT_PASSWORD").expect("MINIO_ROOT_PASSWORD is not set");
  let base_url = format!("{}://{}:{}", schema, url, port);

  let region = Region::Custom {
    region: "".to_string(),
    endpoint: base_url,
  };

  let credentials = Credentials {
    access_key: Some(user),
    secret_key: Some(password),
    security_token: None,
    session_token: None,
    expiration: None,
  };

  let bucket = Bucket::new("aviation", region.clone(), credentials.clone())
    .expect("Failed to create S3 Bucket")
    .with_path_style();

  match BUCKET.set(*bucket) {
    Ok(_) => log::info!("Bucket initialized"),
    Err(_) => log::warn!("Bucket client already initialized"),
  }

  // Run migrations
  match run_migrations().await {
    Ok(_) => log::debug!("Successfully ran database migrations"),
    Err(e) => log::error!("Failed to run migrations: {}", e),
  }

  log::info!("Database initialized");

  Ok(())
}

pub fn pool() -> &'static Pool<Postgres> {
  POOL.get().unwrap()
}

fn redis() -> &'static RedisClient {
  REDIS.get().unwrap()
}

pub fn redis_connection() -> RedisResult<redis::Connection> {
  let conn = redis().get_connection()?;
  Ok(conn)
}

pub async fn redis_async_connection() -> RedisResult<RedisConnection> {
  let conn = redis().get_multiplexed_async_connection().await?;
  Ok(conn)
}

async fn run_migrations() -> ApiResult<()> {
  log::debug!("Running migrations");
  let pool = pool();
  sqlx::migrate!().run(pool).await?;
  Ok(())
}

async fn create_bucket() -> ApiResult<CreateBucketResponse> {
  let url = std::env::var("MINIO_URL").unwrap_or("localhost".to_string());
  let port = std::env::var("MINIO_PORT").unwrap_or("9000".to_string());
  let user = std::env::var("MINIO_ROOT_USER").expect("MINIO_ROOT_USER is not set");
  let password = std::env::var("MINIO_ROOT_PASSWORD").expect("MINIO_ROOT_PASSWORD is not set");
  let base_url = format!("http://{}:{}", url, port);

  let region = Region::Custom {
    region: "".to_string(),
    endpoint: base_url,
  };

  let credentials = Credentials {
    access_key: Some(user),
    secret_key: Some(password),
    security_token: None,
    session_token: None,
    expiration: None,
  };
  let bucket_name = "aviation";
  let response = Bucket::create_with_path_style(
    bucket_name,
    region,
    credentials,
    BucketConfiguration::default(),
  )
  .await?;
  Ok(response)
}

pub async fn upload_file(path: &str, content: &[u8]) -> ApiResult<ResponseData> {
  let response = BUCKET.get().unwrap().put_object(path, content).await?;
  Ok(response)
}

pub async fn get_file(path: &str) -> ApiResult<Vec<u8>> {
  let response = BUCKET.get().unwrap().get_object(path).await?;
  let bytes = response.bytes();
  Ok(bytes.to_vec())
}

pub async fn delete_file(path: &str) -> ApiResult<ResponseData> {
  let response = BUCKET.get().unwrap().delete_object(path).await?;
  Ok(response)
}

#[derive(Serialize, Deserialize)]
pub struct Paged<T> {
  pub data: T,
  pub page: u32,
  pub limit: u32,
  pub total: i64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Coordinate {
  pub lon: f64,
  pub lat: f64,
}
