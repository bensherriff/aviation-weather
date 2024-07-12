use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use log::warn;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct ServiceError {
  pub status: u16,
  pub message: String,
}

impl ServiceError {
  pub fn new(status: u16, message: String) -> ServiceError {
    ServiceError { status, message }
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

impl fmt::Display for ServiceError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(self.message.as_str())
  }
}

impl From<std::io::Error> for ServiceError {
  fn from(error: std::io::Error) -> ServiceError {
    ServiceError::new(500, format!("Unknown IO error: {}", error))
  }
}

impl From<std::env::VarError> for ServiceError {
  fn from(error: std::env::VarError) -> ServiceError {
    ServiceError::new(
      500,
      format!("Unknown environment variable error: {}", error),
    )
  }
}

impl From<DieselError> for ServiceError {
  fn from(error: DieselError) -> ServiceError {
    match error {
      DieselError::DatabaseError(kind, err) => match kind {
        diesel::result::DatabaseErrorKind::UniqueViolation => {
          ServiceError::new(409, err.message().to_string())
        }
        _ => ServiceError::new(500, err.message().to_string()),
      },
      DieselError::NotFound => ServiceError::new(404, "The record was not found".to_string()),
      DieselError::SerializationError(err) => ServiceError::new(422, err.to_string()),
      err => ServiceError::new(500, format!("Unknown Diesel error: {}", err)),
    }
  }
}

impl From<reqwest::Error> for ServiceError {
  fn from(error: reqwest::Error) -> ServiceError {
    ServiceError::new(500, format!("Unknown reqwest error: {}", error))
  }
}

impl From<serde_json::Error> for ServiceError {
  fn from(error: serde_json::Error) -> ServiceError {
    ServiceError::new(500, format!("Unknown serde_json error: {}", error))
  }
}

impl From<argon2::password_hash::Error> for ServiceError {
  fn from(error: argon2::password_hash::Error) -> ServiceError {
    ServiceError::new(500, format!("Unknown argon2 error: {}", error))
  }
}

impl From<jsonwebtoken::errors::Error> for ServiceError {
  fn from(error: jsonwebtoken::errors::Error) -> ServiceError {
    ServiceError::new(500, format!("Unknown jsonwebtoken error: {}", error))
  }
}

impl From<redis::RedisError> for ServiceError {
  fn from(error: redis::RedisError) -> ServiceError {
    ServiceError::new(500, format!("Unknown redis error: {}", error))
  }
}

impl From<s3::error::S3Error> for ServiceError {
  fn from(error: s3::error::S3Error) -> ServiceError {
    match error {
      s3::error::S3Error::Credentials(err) => {
        ServiceError::new(500, format!("Unknown s3 credentials error: {}", err))
      }
      s3::error::S3Error::FromUtf8(err) => {
        ServiceError::new(500, format!("Unknown s3 from utf8 error: {}", err))
      }
      s3::error::S3Error::FmtError(err) => {
        ServiceError::new(500, format!("Unknown s3 fmt error: {}", err))
      }
      s3::error::S3Error::HeaderToStr(err) => {
        ServiceError::new(500, format!("Unknown s3 header to str error: {}", err))
      }
      s3::error::S3Error::HmacInvalidLength(err) => ServiceError::new(
        500,
        format!("Unknown s3 hmac invalid length error: {}", err),
      ),
      s3::error::S3Error::Http(error) => ServiceError::new(error.status_code().as_u16(), error.to_string()),
      _ => ServiceError::new(500, format!("Unknown s3 error: {}", error)),
    }
  }
}

impl ResponseError for ServiceError {
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
