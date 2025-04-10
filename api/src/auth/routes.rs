use std::sync::OnceLock;
use actix_web::{
  post, web, HttpResponse, ResponseError,
  cookie::{Cookie, time::Duration},
  HttpRequest, put,
};
use crate::{
  auth::{verify_hash, Session, SESSION_COOKIE_NAME},
  error::Error,
  users::{LoginRequest, RegisterRequest, User, UserResponse},
};

use crate::auth::{hash, Auth, DEFAULT_SESSION_TTL};
use crate::error::ApiResult;
use crate::users::UpdateUser;

#[post("/register")]
async fn register(user: web::Json<RegisterRequest>, req: HttpRequest) -> HttpResponse {
  let register_user = user.into_inner();
  let email = register_user.email.clone();
  let ip_address = req.peer_addr().unwrap().ip().to_string();
  let mut insert_user: User = match register_user.to_user() {
    Ok(user) => user,
    Err(err) => return ResponseError::error_response(&err),
  };

  match insert_user.insert().await {
    Ok(user) => {
      let response: UserResponse = user.into();
      log::info!(
        "Successful user registration [Email: {}] [IP Address: {}]",
        email,
        ip_address
      );
      HttpResponse::Created().json(response)
    }
    Err(err) => {
      // Obfuscate the service error message to prevent leaking database details
      if err.status == 409 {
        log::warn!(
          "Duplicate user registration attempt [Email: {}] [IP Address: {}]",
          email,
          ip_address
        );
        HttpResponse::Conflict().finish()
      } else {
        log::error!("attemptFailed to register user [Email: {}]: {}", email, err);
        ResponseError::error_response(&err)
      }
    }
  }
}

#[post("/login")]
async fn login(request: web::Json<LoginRequest>, req: HttpRequest) -> HttpResponse {
  let email = &request.email;
  let ip_address = req.peer_addr().unwrap().ip().to_string();

  let query_user = match User::select(&email).await {
    Some(query_user) => query_user,
    None => return HttpResponse::Unauthorized().finish(),
  };

  if verify_hash(&request.password, &query_user.password_hash) {
    // Create a session
    let session = Session::new(64, &email, &ip_address, Some(DEFAULT_SESSION_TTL));
    let session_cookie = session.to_cookie();
    // Save the session to the database
    if let Err(err) = session.store().await {
      log::error!(
        "Login attempt failure [Email: {}] [IP Address: {}]: {}",
        email,
        ip_address,
        err
      );
      return ResponseError::error_response(&Error::new(500, err.to_string()));
    }
    log::info!(
      "Successful login attempt [Email: {}] [IP Address: {}]",
      email,
      ip_address
    );
    HttpResponse::Ok().cookie(session_cookie).finish()
  } else {
    log::error!(
      "Invalid login attempt [Email: {}] [IP Address: {}]",
      email,
      ip_address
    );
    HttpResponse::Unauthorized().finish()
  }
}

#[post("/logout")]
async fn logout(req: HttpRequest, auth: Auth) -> HttpResponse {
  let email = auth.user.email;
  let ip_address = req.peer_addr().unwrap().ip().to_string();
  // Delete the session from the store
  match req.cookie(SESSION_COOKIE_NAME) {
    Some(cookie) => {
      let session_id = cookie.value().to_string();
      if let Err(err) = Session::delete(&session_id).await {
        log::error!(
          "Logout attempt failure [Email: {}] [IP Address: {}]: {}",
          email,
          ip_address,
          err
        );
        return ResponseError::error_response(&Error::new(500, err.to_string()));
      }
    }
    None => {
      log::error!(
        "Invalid logout attempt [Email: {}] [IP Address: {}]",
        email,
        ip_address
      );
      return ResponseError::error_response(&Error::new(400, "Invalid session".to_string()));
    }
  }

  let session_cookie = Cookie::build(SESSION_COOKIE_NAME, "")
    .path("/")
    .max_age(Duration::seconds(-1))
    .secure(true)
    .http_only(true)
    .finish();

  log::info!(
    "Successful logout attempt [Email: {}] [IP Address: {}]",
    email,
    ip_address
  );
  HttpResponse::Ok().cookie(session_cookie).finish()
}

#[put("/password")]
async fn change_password(
  password: web::Json<String>,
  req: HttpRequest,
  auth: Auth,
) -> HttpResponse {
  let ip_address = req.peer_addr().unwrap().ip().to_string();
  let email = auth.user.email;

  if let None = User::select(&email).await {
    return HttpResponse::Unauthorized().finish();
  };

  let update_user = UpdateUser {
    email: None,
    password: Some(password.into_inner()),
    role: None,
    first_name: None,
    last_name: None,
  };

  match update_user.update(&email).await {
    Ok(user) => {
      let response: UserResponse = user.into();
      log::info!(
        "Successful password change attempt [Email: {}] [IP Address: {}]",
        &email,
        ip_address
      );
      HttpResponse::Ok().json(response)
    }
    Err(err) => {
      log::error!(
        "Invalid password change attempt [Email: {}] [IP Address: {}]: {}",
        &email,
        ip_address,
        err
      );
      ResponseError::error_response(&Error::new(500, err.to_string()))
    }
  }
}

#[post("/password-reset")]
async fn password_reset(req: HttpRequest, auth: Auth) -> HttpResponse {
  let ip_address = req.peer_addr().unwrap().ip().to_string();
  HttpResponse::Ok().finish()
}

pub fn init_routes(config: &mut web::ServiceConfig) {
  config.service(
    web::scope("account")
      .service(register)
      .service(login)
      .service(logout)
      .service(change_password),
  );
}
