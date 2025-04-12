use std::future::Future;
use std::pin::Pin;

use actix_web::{FromRequest, Error as ActixError, HttpRequest, dev::Payload, http};
use serde::{Serialize, Deserialize};
use crate::{error::Error, users::User};
use super::{Session, SESSION_COOKIE_NAME};

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
  pub session_id: Option<String>,
  pub api_key: Option<String>,
  pub user: User,
}

impl FromRequest for Auth {
  type Error = ActixError;
  type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

  fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
    // Check for API key
    match req
      .headers()
      .get(http::header::AUTHORIZATION)
      .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
    {
      Some(key_id) => {
        let fut = async move {
          // Check if the Session API key exists
          let api_key = match Session::get(&key_id).await {
            Ok(session) => session,
            Err(err) => {
              log::error!("Invalid session auth attempt: {}", err);
              return Err(Error::new(401, "API Key does not exist".to_string()).into());
            }
          };
          match User::select(&api_key.email).await {
            Some(user) => Ok(Auth {
              session_id: None,
              api_key: Some(key_id),
              user,
            }),
            None => Err(Error::new(404, format!("User {} not found", api_key.email)).into()),
          }
        };
        return Box::pin(fut);
      }
      None => {}
    };

    // Check for session
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
            Error {
              status: 401,
              details: "No session ID found in the request".to_string(),
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
        Ok(session) => match User::select(&session.email).await {
          Some(user) => Ok(Auth {
            session_id: Some(session_id),
            api_key: None,
            user,
          }),
          None => Err(Error::new(404, format!("User {} not found", session.email)).into()),
        },
        Err(err) => Err(err.into()),
      }
    };
    Box::pin(fut)
  }
}
