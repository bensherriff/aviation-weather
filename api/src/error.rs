use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use log::warn;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt;

pub type ApiResult<T> = Result<T, Error>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Error {
  pub status: u16,
  pub details: String,
}

impl Error {
  pub fn new(status: u16, message: String) -> Self {
    Self {
      status,
      details: message,
    }
  }

  pub fn to_http_response(&self) -> HttpResponse {
    let status = StatusCode::from_u16(self.status).unwrap_or_else(|err| {
      warn!("{}", err);
      StatusCode::INTERNAL_SERVER_ERROR
    });
    HttpResponse::build(status).body(self.details.to_string())
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(self.details.as_str())
  }
}

impl std::error::Error for Error {
  fn description(&self) -> &str {
    &self.details
  }
}

impl ResponseError for Error {
  fn error_response(&self) -> HttpResponse {
    let status =
      StatusCode::from_u16(self.status).unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR);

    let status_code = status.as_u16();
    let details = match status_code {
      401 => String::from("Unauthorized"),
      code if code < 500 => self.details.clone(),
      _ => {
        log::error!("Internal server error: {}", self.details);
        String::from("Internal Server Error")
      }
    };

    HttpResponse::build(status).json(json!({ "status": status_code, "details": details }))
  }
}

impl From<std::io::Error> for Error {
  fn from(error: std::io::Error) -> Self {
    Self::new(500, format!("Unknown IO error: {}", error))
  }
}

impl From<chrono::ParseError> for Error {
  fn from(error: chrono::ParseError) -> Self {
    Self::new(500, format!("Parse error: {}", error))
  }
}

impl From<core::num::ParseIntError> for Error {
  fn from(error: core::num::ParseIntError) -> Self {
    Self::new(500, format!("Parse error: {}", error))
  }
}

impl From<std::env::VarError> for Error {
  fn from(error: std::env::VarError) -> Self {
    Self::new(
      500,
      format!("Unknown environment variable error: {}", error),
    )
  }
}

impl From<reqwest::Error> for Error {
  fn from(error: reqwest::Error) -> Self {
    Self::new(500, format!("Unknown reqwest error: {}", error))
  }
}

impl From<serde_json::Error> for Error {
  fn from(error: serde_json::Error) -> Self {
    Self::new(500, format!("Unknown serde_json error: {}", error))
  }
}

impl From<argon2::password_hash::Error> for Error {
  fn from(error: argon2::password_hash::Error) -> Self {
    Self::new(500, format!("Unknown argon2 error: {}", error))
  }
}

impl From<redis::RedisError> for Error {
  fn from(error: redis::RedisError) -> Self {
    Self::new(500, format!("Unknown redis error: {}", error))
  }
}

impl From<s3::error::S3Error> for Error {
  fn from(error: s3::error::S3Error) -> Self {
    match error {
      s3::error::S3Error::Credentials(err) => {
        Self::new(500, format!("Unknown s3 credentials error: {}", err))
      }
      s3::error::S3Error::FromUtf8(err) => {
        Self::new(500, format!("Unknown s3 from utf8 error: {}", err))
      }
      s3::error::S3Error::FmtError(err) => Self::new(500, format!("Unknown s3 fmt error: {}", err)),
      s3::error::S3Error::HeaderToStr(err) => {
        Self::new(500, format!("Unknown s3 header to str error: {}", err))
      }
      s3::error::S3Error::HmacInvalidLength(err) => Self::new(
        500,
        format!("Unknown s3 hmac invalid length error: {}", err),
      ),
      s3::error::S3Error::Http(error) => Self::new(error.status_code().as_u16(), error.to_string()),
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

impl From<sqlx::Error> for Error {
  fn from(error: sqlx::Error) -> Self {
    match error {
      sqlx::Error::RowNotFound => Error::new(404, "Not found".to_string()),
      sqlx::Error::ColumnIndexOutOfBounds { .. } => Error::new(422, error.to_string()),
      sqlx::Error::ColumnNotFound { .. } => Error::new(422, error.to_string()),
      sqlx::Error::ColumnDecode { .. } => Error::new(422, error.to_string()),
      sqlx::Error::Decode(_) => Error::new(422, error.to_string()),
      sqlx::Error::PoolTimedOut => Error::new(503, error.to_string()),
      sqlx::Error::PoolClosed => Error::new(503, error.to_string()),
      sqlx::Error::Tls(_) => Error::new(500, error.to_string()),
      sqlx::Error::Io(_) => Error::new(500, error.to_string()),
      sqlx::Error::Protocol(_) => Error::new(500, error.to_string()),
      sqlx::Error::Configuration(_) => Error::new(500, error.to_string()),
      sqlx::Error::AnyDriverError(_) => Error::new(500, error.to_string()),
      sqlx::Error::Database(err) => {
        if let Some(code) = err.code() {
          match code.trim() {
            // Unique violation
            "23505" => return Error::new(409, err.to_string()),
            _ => (),
          }
        }
        Error::new(500, err.to_string())
      }
      sqlx::Error::Migrate(_) => Error::new(500, error.to_string()),
      sqlx::Error::TypeNotFound { type_name } => {
        Error::new(500, format!("Type not found: {}", type_name))
      }
      sqlx::Error::WorkerCrashed => Error::new(500, error.to_string()),
      _ => Error::new(500, error.to_string()),
    }
  }
}

impl From<sqlx::migrate::MigrateError> for Error {
  fn from(error: sqlx::migrate::MigrateError) -> Self {
    Error::new(500, error.to_string())
  }
}
