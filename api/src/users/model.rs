use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{auth::hash, error::ApiResult};
use crate::db;

pub const ADMIN_ROLE: &str = "ADMIN";
pub const USER_ROLE: &str = "USER";
const TABLE_NAME: &str = "users";

/**
 * RegisterRequest
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
  pub email: String,
  pub password: String,
  pub first_name: String,
  pub last_name: String,
}

impl RegisterRequest {
  pub fn to_user(self) -> ApiResult<User> {
    let password_hash = hash(&self.password)?;
    Ok(User {
      email: self.email.to_lowercase(),
      password_hash,
      role: USER_ROLE.to_string(),
      first_name: self.first_name,
      last_name: self.last_name,
      updated_at: Utc::now(),
      created_at: Utc::now(),
    })
  }
}

/**
 * LoginRequest
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
  pub email: String,
  pub password: String,
}

/**
 * UserResponse
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
  pub email: String,
  pub role: String,
  pub first_name: String,
  pub last_name: String,
}

impl From<User> for UserResponse {
  fn from(user: User) -> Self {
    UserResponse {
      email: user.email,
      role: user.role,
      first_name: user.first_name,
      last_name: user.last_name,
    }
  }
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
pub struct User {
  pub email: String,
  pub password_hash: String,
  pub role: String,
  pub first_name: String,
  pub last_name: String,
  pub updated_at: DateTime<Utc>,
  pub created_at: DateTime<Utc>,
}

impl User {
  pub async fn select(email: &str) -> Option<Self> {
    let pool = db::pool();
    let user: Option<Self> = sqlx::query_as::<_, Self>(&format!(
      r#"
      SELECT * FROM {} WHERE email = LOWER($1)
      "#,
      TABLE_NAME
    ))
    .bind(email)
    .fetch_optional(pool)
    .await
    .unwrap_or_else(|err| {
      log::error!("Unable to find user '{}': {}", email, err);
      None
    });

    user
  }

  pub async fn insert(&self) -> ApiResult<User> {
    let pool = db::pool();
    let user: User = sqlx::query_as::<_, Self>(&format!(
      r#"
      INSERT INTO {} (
        email,
        password_hash,
        role,
        first_name,
        last_name,
        created_at,
        updated_at
      )
      VALUES ($1, $2, $3, $4, $5, $6, $7)
      RETURNING *
      "#,
      TABLE_NAME,
    ))
    .bind(&self.email)
    .bind(&self.password_hash)
    .bind(&self.role)
    .bind(&self.first_name)
    .bind(&self.last_name)
    .bind(self.created_at)
    .bind(self.updated_at)
    .fetch_one(pool)
    .await?;

    Ok(user)
  }
}
