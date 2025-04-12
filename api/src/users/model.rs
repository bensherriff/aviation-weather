use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, QueryBuilder};
use crate::{auth::hash, error::ApiResult};
use crate::db;

pub const ADMIN_ROLE: &str = "ADMIN";
pub const USER_ROLE: &str = "USER";
const TABLE_NAME: &str = "users";

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

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
  pub email: String,
  pub password: String,
}

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

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UpdateUser {
  pub email: Option<String>,
  pub password: Option<String>,
  pub role: Option<String>,
  pub first_name: Option<String>,
  pub last_name: Option<String>,
}

impl UpdateUser {
  pub async fn update(&self, email: &str) -> ApiResult<User> {
    let pool = db::pool();

    let mut query_builder: QueryBuilder<Postgres> =
      QueryBuilder::new(&format!("UPDATE {} SET ", TABLE_NAME));

    let mut first_clause = true;

    let mut push_comma = |query_builder: &mut QueryBuilder<Postgres>| {
      if !first_clause {
        query_builder.push(", ");
      } else {
        first_clause = false;
      }
    };

    if let Some(ref email) = self.email {
      push_comma(&mut query_builder);
      query_builder.push("email = ");
      query_builder.push_bind(email);
    }
    if let Some(ref password) = self.password {
      push_comma(&mut query_builder);
      let password_hash = hash(password)?;
      query_builder.push("password_hash = ");
      query_builder.push_bind(password_hash);
    }
    if let Some(ref role) = self.role {
      push_comma(&mut query_builder);
      query_builder.push("role = ");
      query_builder.push_bind(role);
    }
    if let Some(ref first_name) = self.first_name {
      push_comma(&mut query_builder);
      query_builder.push("first_name = ");
      query_builder.push_bind(first_name);
    }
    if let Some(ref last_name) = self.last_name {
      push_comma(&mut query_builder);
      query_builder.push("last_name = ");
      query_builder.push_bind(last_name);
    }
    push_comma(&mut query_builder);
    query_builder.push("updated_at = ");
    query_builder.push_bind(Utc::now());

    query_builder.push(" WHERE email = ");
    query_builder.push_bind(email.to_string());
    query_builder.push(" RETURNING *");

    dbg!(&query_builder.sql());

    let query = query_builder.build_query_as::<User>();
    let user = query.fetch_one(pool).await?;

    Ok(user)
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

  pub async fn count() -> i64 {
    let pool = db::pool();

    sqlx::query_scalar(&format!(
      r#"
      SELECT COUNT(*) FROM {}
      "#,
      TABLE_NAME
    ))
    .fetch_one(pool)
    .await
    .unwrap_or_else(|_| 0)
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
