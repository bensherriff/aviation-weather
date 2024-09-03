use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use log::warn;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiError {
  pub status: u16,
  pub message: String,
}

impl ApiError {
  pub fn new(status: u16, message: String) -> Self {
    Self { status, message }
  }

  pub fn to_http_response(&self) -> HttpResponse {
    let status = match StatusCode::from_u16(self.status) {
      Ok(s) => s,
      Err(err) => {
        warn!("{}", err);
        StatusCode::INTERNAL_SERVER_ERROR
      }
    };
    HttpResponse::build(status).body(self.message.to_string())
  }
}

impl fmt::Display for ApiError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(self.message.as_str())
  }
}

impl From<std::io::Error> for ApiError {
  fn from(error: std::io::Error) -> Self {
    Self::new(500, format!("Unknown IO error: {}", error))
  }
}

impl From<std::env::VarError> for ApiError {
  fn from(error: std::env::VarError) -> Self {
    Self::new(
      500,
      format!("Unknown environment variable error: {}", error),
    )
  }
}

impl From<DieselError> for ApiError {
  fn from(error: DieselError) -> Self {
    match error {
      DieselError::DatabaseError(kind, err) => match kind {
        diesel::result::DatabaseErrorKind::UniqueViolation => {
          Self::new(409, err.message().to_string())
        }
        _ => Self::new(500, err.message().to_string()),
      },
      DieselError::NotFound => Self::new(404, "The record was not found".to_string()),
      DieselError::SerializationError(err) => Self::new(422, err.to_string()),
      err => Self::new(500, format!("Unknown Diesel error: {}", err)),
    }
  }
}

impl From<reqwest::Error> for ApiError {
  fn from(error: reqwest::Error) -> Self {
    Self::new(500, format!("Unknown reqwest error: {}", error))
  }
}

impl From<serde_json::Error> for ApiError {
  fn from(error: serde_json::Error) -> Self {
    Self::new(500, format!("Unknown serde_json error: {}", error))
  }
}

impl From<argon2::password_hash::Error> for ApiError {
  fn from(error: argon2::password_hash::Error) -> Self {
    Self::new(500, format!("Unknown argon2 error: {}", error))
  }
}

impl From<redis::RedisError> for ApiError {
  fn from(error: redis::RedisError) -> Self {
    Self::new(500, format!("Unknown redis error: {}", error))
  }
}

impl From<s3::error::S3Error> for ApiError {
  fn from(error: s3::error::S3Error) -> Self {
    match error {
      s3::error::S3Error::Credentials(err) => {
        Self::new(500, format!("Unknown s3 credentials error: {}", err))
      }
      s3::error::S3Error::FromUtf8(err) => {
        Self::new(500, format!("Unknown s3 from utf8 error: {}", err))
      }
      s3::error::S3Error::FmtError(err) => {
        Self::new(500, format!("Unknown s3 fmt error: {}", err))
      }
      s3::error::S3Error::HeaderToStr(err) => {
        Self::new(500, format!("Unknown s3 header to str error: {}", err))
      }
      s3::error::S3Error::HmacInvalidLength(err) => Self::new(
        500,
        format!("Unknown s3 hmac invalid length error: {}", err),
      ),
      s3::error::S3Error::Http(error) => {
        Self::new(error.status_code().as_u16(), error.to_string())
      }
      _ => {
        let re = Regex::new(r"HTTP (\d{3})").unwrap();
        // Apply the regex to the input string
        if let Some(captures) = re.captures(&error.to_string()) {
          if let Some(http_code_str) = captures.get(1) {
            if let Ok(http_code) = http_code_str.as_str().parse::<u16>() {
              return Self::new(http_code, error.to_string());
            }
          }
        }
        Self::new(500, format!("Unknown s3 error: {}", error))
      }
    }
  }
}

impl ResponseError for ApiError {
  fn error_response(&self) -> HttpResponse {
    let status = match StatusCode::from_u16(self.status) {
      Ok(status) => status,
      Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };

    let message = match status.as_u16() < 500 {
      true => self.message.clone(),
      false => "Internal server error".to_string(),
    };

    HttpResponse::build(status).json(json!({ "status": status.as_u16(), "message": message }))
  }
}
