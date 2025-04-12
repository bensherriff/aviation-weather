use actix_web::cookie::{time::Duration, Cookie};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use redis::{AsyncCommands, RedisResult};
use tokio::task;
use crate::{
  db::redis_async_connection,
  error::{Error, ApiResult},
};
use super::{csprng, hash, verify_hash};

const DEFAULT_SESSION_TTL: i64 = 86400; // (In seconds) 24 hours
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
  pub fn default(email: &str, ip_address: &str) -> Self {
    Self::new(64, email, ip_address, Some(DEFAULT_SESSION_TTL))
  }

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

  pub async fn get(session_id: &str) -> ApiResult<Self> {
    let mut conn = redis_async_connection().await?;
    let result: RedisResult<Option<String>> = conn.get(session_id).await;
    match result {
      Ok(Some(value)) => Ok(serde_json::from_str(&value)?),
      Ok(None) => Err(Error::new(401, format!("Missing session {}", session_id))),
      Err(err) => Err(err.into()),
    }
  }

  pub async fn replace(session_id: &str, ip_address: &str) -> ApiResult<Self> {
    let mut session = Self::verify(session_id, ip_address).await?;
    let session_id_owned = session_id.to_owned();
    task::spawn(async move {
      if let Err(err) = Self::delete(&session_id_owned).await {
        log::error!(
          "Error deleting old session in replace session call: {}",
          err
        );
      };
    });
    session = Session::default(&session.email, ip_address);
    session.store().await?;
    Ok(session)
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
    let session = Self::get(session_id).await?;

    // Check if the IP Address matches the Session's IP Address
    if verify_hash(ip_address, &session.ip_address) {
      Ok(session)
    } else {
      Err(Error::new(401, "IP Address does not match".to_string()))
    }
  }

  pub fn cookie(&self) -> Cookie {
    let expires_at = match self.expires_at {
      Some(expires_at) => expires_at.timestamp(),
      None => DEFAULT_SESSION_TTL,
    };
    let ttl = expires_at - Utc::now().timestamp();
    let mut cookie = Cookie::build(SESSION_COOKIE_NAME, self.session_id.clone())
      .path("/")
      .max_age(Duration::seconds(ttl))
      .secure(true)
      .http_only(true)
      .finish();

    if let Ok(environment) = std::env::var("ENVIRONMENT") {
      if environment == "development" || environment == "dev" {
        log::trace!(
          "Development cookie [Email: {}]: {}",
          self.email,
          self.session_id
        );
        cookie.set_secure(false);
        cookie.set_http_only(false);
      }
    }

    cookie
  }

  pub fn empty_cookie() -> Cookie<'static> {
    let mut cookie = Cookie::build(SESSION_COOKIE_NAME, "")
      .path("/")
      .max_age(Duration::seconds(-1))
      .secure(true)
      .http_only(true)
      .finish();

    if let Ok(environment) = std::env::var("ENVIRONMENT") {
      if environment == "development" || environment == "dev" {
        cookie.set_secure(false);
        cookie.set_http_only(false);
      }
    }

    cookie
  }
}
