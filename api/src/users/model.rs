use serde::{Deserialize, Serialize};
use diesel::prelude::*;

use crate::{
  auth::hash,
  db::{connection, schema::users},
  error::ApiResult,
};

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
    let hash = hash(&self.password)?;
    Ok(User {
      email: self.email.to_lowercase(),
      hash,
      role: "user".to_string(),
      first_name: self.first_name,
      last_name: self.last_name,
      updated_at: chrono::Utc::now().naive_utc(),
      created_at: chrono::Utc::now().naive_utc(),
      profile_picture: None,
      favorites: vec![],
      verified: false,
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
  pub profile_picture: Option<String>,
}

impl From<User> for UserResponse {
  fn from(user: User) -> Self {
    UserResponse {
      email: user.email,
      role: user.role,
      first_name: user.first_name,
      last_name: user.last_name,
      profile_picture: user.profile_picture,
    }
  }
}

/**
 * User
 */
#[derive(Debug, Insertable, AsChangeset, Queryable, QueryableByName, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
  pub email: String,
  pub hash: String,
  pub role: String,
  pub first_name: String,
  pub last_name: String,
  pub updated_at: chrono::NaiveDateTime,
  pub created_at: chrono::NaiveDateTime,
  pub profile_picture: Option<String>,
  pub favorites: Vec<String>,
  pub verified: bool,
}

impl User {
  pub fn get_by_email(email: &str) -> ApiResult<User> {
    let mut conn = connection()?;
    // Check if the user exists by email, case insensitive

    let user = users::table
      .filter(users::email.eq(email.to_lowercase()))
      .first(&mut conn)?;
    Ok(user)
  }

  pub fn insert(user: Self) -> ApiResult<User> {
    let mut conn = connection()?;
    let user = diesel::insert_into(users::table)
      .values(user)
      .get_result(&mut conn)?;
    Ok(user)
  }
}
