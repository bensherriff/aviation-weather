use actix_web::cookie::{time::Duration, Cookie};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use redis::{AsyncCommands, RedisResult};

use crate::{
  db::redis_async_connection,
  error::{Error, ApiResult},
};

use super::{csprng, hash, verify_hash};

pub const DEFAULT_SESSION_TTL: i64 = 86400; // (In seconds) 24 hours
pub const SESSION_COOKIE_NAME: &str = "session";

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
  pub session_id: String,
  pub email: String,
  pub ip_address: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub expires_at: Option<DateTime<Utc>>,
}

impl Session {
  pub fn new(take: usize, email: &str, ip_address: &str, ttl: Option<i64>) -> Self {
    let now = Utc::now();
    Self {
      session_id: csprng(take),
      email: email.to_string(),
      ip_address: hash(&ip_address).unwrap(),
      expires_at: match ttl {
        Some(ttl) => Some(now + chrono::Duration::seconds(ttl)),
        None => None,
      },
    }
  }

  pub async fn store(&self) -> ApiResult<()> {
    let mut conn = redis_async_connection().await?;
    let key = self.session_id.clone();
    let value = serde_json::to_string(self)?;
    let result: RedisResult<()> = match self.expires_at {
      Some(expires_at) => {
        let ttl = expires_at.timestamp() - Utc::now().timestamp();
        conn.set_ex(key, &value, ttl as u64).await
      }
      None => conn.set(key, value).await,
    };
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
      None => return Err(Error::new(401, "Session does not exist".to_string())),
    };

    // Check if the IP Address matches the Session's IP Address
    if verify_hash(ip_address, &session.ip_address) {
      Ok(session)
    } else {
      Err(Error::new(401, "IP Address does not match".to_string()))
    }
  }

  pub fn to_cookie(&self) -> Cookie {
    let expires_at = match self.expires_at {
      Some(expires_at) => expires_at.timestamp(),
      None => DEFAULT_SESSION_TTL,
    };
    let ttl = expires_at - Utc::now().timestamp();
    Cookie::build(SESSION_COOKIE_NAME, self.session_id.clone())
      .path("/")
      .max_age(Duration::seconds(ttl))
      // TODO: enable secure and http_only
      // .secure(true)
      // .http_only(true)
      .finish()
  }
}
