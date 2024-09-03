use actix_web::{
  post, web, HttpResponse, ResponseError,
  cookie::{Cookie, time::Duration},
  HttpRequest,
};
use crate::{
  auth::{verify_hash, Session, SESSION_COOKIE_NAME},
  error::ApiError,
  users::{LoginRequest, RegisterRequest, User, UserResponse},
};

use crate::auth::Auth;

#[post("/register")]
async fn register(user: web::Json<RegisterRequest>) -> HttpResponse {
  let register_user = user.0;
  let insert_user: User = match register_user.to_user() {
    Ok(user) => user,
    Err(err) => return ResponseError::error_response(&err),
  };
  match User::insert(insert_user) {
    Ok(user) => {
      let response: UserResponse = user.into();
      HttpResponse::Created().json(response)
    },
    Err(err) => {
      // Obfuscate the service error message to prevent leaking database details
      if err.status == 409 {
        return HttpResponse::Conflict().finish();
      } else {
        return ResponseError::error_response(&err);
      }
    }
  }
}

#[post("/login")]
async fn login(request: web::Json<LoginRequest>, req: HttpRequest) -> HttpResponse {
  let email = request.email.clone();
  let ip_address = req.peer_addr().unwrap().ip().to_string();

  let query_user = match User::get_by_email(&email) {
    Ok(query_user) => query_user,
    Err(err) => {
      log::error!("{}", err);
      return ResponseError::error_response(&err);
    }
  };
  if verify_hash(&query_user.hash, &request.password) {
    // Create a session
    let session = Session::new(&email, &ip_address);
    let session_cookie = session.cookie();
    // Save the session to the database
    if let Err(err) = session.store().await {
      log::error!("Failed to store session");
      return ResponseError::error_response(&ApiError::new(500, err.to_string()));
    }
    return HttpResponse::Ok().cookie(session_cookie).finish();
  } else {
    log::error!("Invalid login attempt for {}", email);
    return HttpResponse::Unauthorized().finish();
  }
}

#[post("/logout")]
async fn logout(req: HttpRequest, _auth: Auth) -> HttpResponse {
  // Delete the session from the store
  match req.cookie(SESSION_COOKIE_NAME) {
    Some(cookie) => {
      let session_id = cookie.value().to_string();
      if let Err(err) = Session::delete(&session_id).await {
        log::error!("Failed to delete session");
        return ResponseError::error_response(&ApiError::new(500, err.to_string()));
      }
    }
    None => {
      return ResponseError::error_response(&ApiError::new(400, "Invalid session".to_string()));
    }
  }

  let session_cookie = Cookie::build(SESSION_COOKIE_NAME, "")
    .path("/")
    .max_age(Duration::seconds(-1))
    .secure(true)
    .http_only(true)
    .finish();

  HttpResponse::Ok().cookie(session_cookie).finish()
}

pub fn init_routes(config: &mut web::ServiceConfig) {
  config.service(
    web::scope("auth")
      .service(register)
      .service(login)
      .service(logout),
  );
}
