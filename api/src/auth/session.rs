use actix_web::cookie::{time::Duration, Cookie};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use redis::{AsyncCommands, RedisResult};

use crate::{
  db::redis_async_connection,
  error::{ApiError, ApiResult},
};

use super::{csprng_128bit, hash, verify_hash};

pub const DEFAULT_SESSION_TTL: i64 = 86400; // (In seconds) 24 hours
pub const SESSION_COOKIE_NAME: &str = "session";

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
  pub session_id: String,
  pub email: String,
  pub ip_address: String,
  pub expires_at: DateTime<Utc>,
}

impl Session {
  pub fn new(email: &str, ip_address: &str) -> Self {
    let now = chrono::Utc::now();
    Self {
      session_id: csprng_128bit(32),
      email: email.to_string(),
      ip_address: hash(&ip_address).unwrap(),
      expires_at: now + chrono::Duration::seconds(DEFAULT_SESSION_TTL),
    }
  }

  pub async fn store(&self) -> ApiResult<()> {
    let mut conn = redis_async_connection().await?;
    let key = self.session_id.clone();
    let value = serde_json::to_string(self)?;
    let result: RedisResult<()> = conn.set_ex(key, &value, DEFAULT_SESSION_TTL as u64).await;
    match result {
      Ok(_) => Ok(()),
      Err(err) => Err(err.into()),
    }
  }

  pub async fn get(session_id: &str) -> ApiResult<Option<Self>> {
    let mut conn = redis_async_connection().await?;
    let result: RedisResult<Option<String>> = conn.get(session_id).await;
    match result {
      Ok(Some(value)) => Ok(Some(serde_json::from_str(&value)?)),
      Ok(None) => Ok(None),
      Err(err) => Err(err.into()),
    }
  }

  pub async fn delete(session_id: &str) -> ApiResult<()> {
    let mut conn = redis_async_connection().await?;
    let result: RedisResult<()> = conn.del(session_id).await;
    match result {
      Ok(_) => Ok(()),
      Err(err) => Err(err.into()),
    }
  }

  pub async fn verify(session_id: &str, ip_address: &str) -> ApiResult<Self> {
    // Check if the session exists
    let session = match Self::get(session_id).await? {
      Some(session) => session,
      None => return Err(ApiError::new(401, "Session does not exist".to_string())),
    };

    // Check if the IP Address matches the Session's IP Address
    if verify_hash(ip_address, &session.ip_address) {
      return Ok(session);
    } else {
      return Err(ApiError::new(
        401,
        "IP Address does not match".to_string(),
      ));
    }
  }

  pub fn cookie(&self) -> Cookie {
    Cookie::build(SESSION_COOKIE_NAME, self.session_id.clone())
      .path("/")
      .max_age(Duration::seconds(DEFAULT_SESSION_TTL))
      .secure(true)
      .http_only(true)
      .finish()
  }
}
