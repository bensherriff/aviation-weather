use std::future::Future;
use std::pin::Pin;

use actix_web::{FromRequest, Error as ActixError, HttpRequest, dev::Payload, http};
use serde::{Serialize, Deserialize};
use crate::{
  error::ApiError,
  users::{User, UserResponse},
};

use super::{Session, SESSION_COOKIE_NAME};

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
  pub session_id: Option<String>,
  pub user: UserResponse,
}

impl FromRequest for Auth {
  type Error = ActixError;
  type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

  fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
    // Get session ID from request
    let session_id = match req
      .cookie(SESSION_COOKIE_NAME)
      .map(|c| c.value().to_string())
      .or_else(|| {
        req
          .headers()
          .get(http::header::AUTHORIZATION)
          .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
      }) {
      Some(id) => id,
      None => {
        let fut = async {
          Err(
            ApiError {
              status: 401,
              message: "No session ID found in the request".to_string(),
            }
            .into(),
          )
        };
        return Box::pin(fut);
      }
    };

    // Get IP address from request
    let ip_address = req.peer_addr().unwrap().ip().to_string();

    // Verify the session
    let fut = async move {
      match Session::verify(&session_id, &ip_address).await {
        Ok(session) => match User::get_by_email(&session.email) {
          Ok(user) => Ok(Auth {
            session_id: Some(session_id),
            user: user.into(),
          }),
          Err(err) => Err(err.into()),
        },
        Err(err) => Err(err.into()),
      }
    };
    Box::pin(fut)
  }
}
