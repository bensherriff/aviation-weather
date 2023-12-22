use crate::error_handler::ServiceError;
use diesel::{r2d2::ConnectionManager, PgConnection};
use redis::{Client as RedisClient, aio::Connection as RedisConnection};
use s3::{Bucket, Region, creds::Credentials, BucketConfiguration, request::ResponseData, bucket_ops::CreateBucketResponse};
use serde::{Deserialize, Serialize};
use crate::diesel_migrations::MigrationHarness;
use lazy_static::lazy_static;
use log::{error, info, warn};
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
  static ref BUCKET: Bucket = {
    let url = env::var("MINIO_HOST").unwrap_or("localhost".to_string());
    let port = env::var("MINIO_PORT").unwrap_or("9000".to_string());
    let user = env::var("MINIO_ROOT_USER").expect("MINIO_ROOT_USER is not set");
    let password = env::var("MINIO_ROOT_PASSWORD").expect("MINIO_ROOT_PASSWORD is not set");
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
      expiration: None
    };
  
    Bucket::new("aviation", region.clone(), credentials.clone()).expect("Failed to create S3 Bucket").with_path_style()
  };
}

pub async fn init() {
  lazy_static::initialize(&POOL);
  lazy_static::initialize(&REDIS);
  lazy_static::initialize(&BUCKET);
  match create_bucket().await {
    Ok(_) => info!("Bucket initialized"),
    Err(err) => {
      match err.status {
        409 => warn!("Bucket already exists"),
        _ => error!("Failed to initialize bucket; {}", err.message)
      }
    }
  };
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

async fn create_bucket() -> Result<CreateBucketResponse, ServiceError> {
  let url = env::var("MINIO_URL").unwrap_or("localhost".to_string());
  let port = env::var("MINIO_PORT").unwrap_or("9000".to_string());
  let user = env::var("MINIO_ROOT_USER").expect("MINIO_ROOT_USER is not set");
  let password = env::var("MINIO_ROOT_PASSWORD").expect("MINIO_ROOT_PASSWORD is not set");
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
    expiration: None
  };
  let bucket_name = "aviation";
  let response = Bucket::create_with_path_style(bucket_name, region, credentials, BucketConfiguration::default()).await?;
  Ok(response)
}

pub async fn upload_file(path: &str, content: &[u8]) -> Result<ResponseData, ServiceError> {
  let response = BUCKET.put_object(path, content).await?;
  Ok(response)
}

pub async fn get_file(path: &str) -> Result<Vec<u8>, ServiceError> {
  let response = BUCKET.get_object(path).await?;
  let bytes = response.bytes();
  Ok(bytes.to_vec())
}

pub async fn delete_file(path: &str) -> Result<ResponseData, ServiceError> {
  let response = BUCKET.delete_object(path).await?;
  Ok(response)
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
