use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use log::warn;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct ServiceError {
    pub error_status_code: u16,
    pub error_message: String,
}

impl ServiceError {
    pub fn new(error_status_code: u16, error_message: String) -> ServiceError {
        ServiceError {
            error_status_code,
            error_message,
        }
    }

    pub fn to_http_response(&self) -> HttpResponse {
        let status_code = match StatusCode::from_u16(self.error_status_code) {
            Ok(s) => s,
            Err(err) => {
                warn!("{}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        HttpResponse::build(status_code).body(self.error_message.to_string())
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.error_message.as_str())
    }
}

impl From<DieselError> for ServiceError {
    fn from(error: DieselError) -> ServiceError {
        match error {
            DieselError::DatabaseError(_, err) => ServiceError::new(409, err.message().to_string()),
            DieselError::NotFound => {
                ServiceError::new(404, "The record was not found".to_string())
            }
            err => ServiceError::new(500, format!("Unknown Diesel error: {}", err)),
        }
    }
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match StatusCode::from_u16(self.error_status_code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let error_message = match status_code.as_u16() < 500 {
            true => self.error_message.clone(),
            false => "Internal server error".to_string(),
        };

        HttpResponse::build(status_code).json(json!({ "message": error_message }))
    }
}