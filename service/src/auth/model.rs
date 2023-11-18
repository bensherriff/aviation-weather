use std::{future::{ready, Ready}, env};
use actix_web::{FromRequest, Error as ActixError, HttpRequest, dev::Payload, http};
use diesel::prelude::*;
use log::error;
use redis::Commands;
use serde::{Serialize, Deserialize};
use crate::error_handler::ServiceError;

use crate::db::{schema::users, connection};

use super::{hash_password, verify_token};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUser {
  pub email: String,
  pub password: String,
  pub first_name: String,
  pub last_name: String,
}

impl RegisterUser {
  pub fn convert_to_insert(self) -> Result<InsertUser, ServiceError> {
    let hash = hash_password(self.password.as_bytes())?;
    Ok(InsertUser {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
  pub email: String,
  pub password: String,
}

#[derive(Debug, Queryable, QueryableByName, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct QueryUser {
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

impl QueryUser {
  pub fn get_by_email(email: &str) -> Result<QueryUser, ServiceError> {
    let mut conn = connection()?;
    // Check if the user exists by email, case insensitive

    let user = users::table
      .filter(users::email.eq(email.to_lowercase()))
      .first(&mut conn)?;
    Ok(user)
  }
}

#[derive(Debug, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct InsertUser {
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

impl InsertUser {
  pub fn insert(user: Self) -> Result<QueryUser, ServiceError> {
    let mut conn = connection()?;
    let user = diesel::insert_into(users::table)
      .values(user)
      .get_result(&mut conn)?;
    Ok(user)
  }

  pub fn update_profile_picture(email: &str, profile_picture: Option<&str>) -> Result<QueryUser, ServiceError> {
    let mut conn = connection()?;
    let user = diesel::update(users::table)
      .filter(users::email.eq(&email))
      .set(users::profile_picture.eq(profile_picture))
      .get_result(&mut conn)?;
    Ok(user)
  }

  pub fn update_favorites(email: &str, favorites: Vec<String>) -> Result<QueryUser, ServiceError> {
    let mut conn = connection()?;
    let user = diesel::update(users::table)
      .filter(users::email.eq(&email))
      .set(users::favorites.eq(favorites))
      .get_result(&mut conn)?;
    Ok(user)
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseUser {
  pub email: String,
  pub role: String,
  pub first_name: String,
  pub last_name: String,
  pub profile_picture: Option<String>,
}

impl From<QueryUser> for ResponseUser {
  fn from(user: QueryUser) -> Self {
    ResponseUser {
      email: user.email,
      role: user.role,
      first_name: user.first_name,
      last_name: user.last_name,
      profile_picture: user.profile_picture,
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtAuth {
  pub token: uuid::Uuid,
  pub user: ResponseUser
}

impl FromRequest for JwtAuth {
  type Error = ActixError;
  type Future = Ready<Result<Self, Self::Error>>;
  fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
    let access_token = match req
      .cookie("access_token")
      .map(|c| c.value().to_string())
      .or_else(|| {
        req.headers().get(http::header::AUTHORIZATION)
        .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
      }) {
        Some(token) => token,
        None => return ready(Err(ActixError::from(ServiceError {
          status: 401,
          message: "Unauthorized".to_string()
        })))
      };

    let public_key = env::var("ACCESS_TOKEN_PUBLIC_KEY")
      .expect("ACCESS_TOKEN_PUBLIC_KEY must be set");

    let access_token_details = match verify_token(&access_token, &public_key) {
      Ok(token_details) => token_details,
      Err(err) => {
        error!("Failed to verify access token: {}", err);
        return ready(Err(ActixError::from(ServiceError {
          status: 401,
          message: format!("Failed to verify access token: {}", err)
        })))
      }
    };

    let access_token_uuid = uuid::Uuid::parse_str(&access_token_details.token_uuid.to_string()).unwrap();
    
    let mut conn = match crate::db::redis_connection() {
      Ok(conn) => conn,
      Err(err) => {
        error!("Failed to get redis connection: {}", err);
        return ready(Err(ActixError::from(ServiceError {
          status: 500,
          message: format!("Failed to get redis connection: {}", err)
        })))
      }
    };
    let user_email = match conn.get::<_, String>(access_token_uuid.clone().to_string()) {
      Ok(result) => result,
      Err(_) => {
        return ready(Err(ActixError::from(ServiceError {
          status: 401,
          message: format!("Access token was not found")
        })))
      }
    };

    match QueryUser::get_by_email(&user_email) {
      Ok(user) => {
        ready(Ok(JwtAuth { token: access_token_uuid, user: user.into() }))
      }
      Err(_) => return ready(Err(ActixError::from(ServiceError {
        status: 401,
        message: format!("User was not found")
      })))
    }
  }
}

pub fn verify_role(auth: &JwtAuth, role: &str) -> Result<(), ServiceError> {
  if auth.user.role == role {
    Ok(())
  } else {
    Err(ServiceError {
      status: 403,
      message: "Forbidden".to_string()
    })
  }
}
