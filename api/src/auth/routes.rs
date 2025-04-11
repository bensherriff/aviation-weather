use actix_web::{post, web, HttpResponse, ResponseError, HttpRequest, put, get};
use crate::{
  auth::{verify_hash, Session, SESSION_COOKIE_NAME},
  error::Error,
  users::{LoginRequest, RegisterRequest, User, UserResponse},
};

use crate::auth::Auth;
use crate::users::UpdateUser;

#[post("/register")]
async fn register(user: web::Json<RegisterRequest>, req: HttpRequest) -> HttpResponse {
  let register_user = user.into_inner();
  let email = register_user.email.clone();
  let ip_address = req.peer_addr().unwrap().ip().to_string();
  let insert_user: User = match register_user.to_user() {
    Ok(user) => user,
    Err(err) => return ResponseError::error_response(&err),
  };

  match insert_user.insert().await {
    Ok(user) => {
      let user_response: UserResponse = user.into();
      log::info!(
        "Successful user registration [Email: {}] [IP Address: {}]",
        email,
        ip_address
      );
      HttpResponse::Created().json(user_response)
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
    let session = Session::default(&email, &ip_address);
    let session_cookie = session.cookie();
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
    let user_response: UserResponse = query_user.into();
    HttpResponse::Ok()
      .cookie(session_cookie)
      .json(user_response)
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

  log::info!(
    "Successful logout attempt [Email: {}] [IP Address: {}]",
    email,
    ip_address
  );
  HttpResponse::Ok().cookie(Session::empty_cookie()).finish()
}

#[get("/session")]
async fn validate_session(req: HttpRequest) -> HttpResponse {
  let ip_address = req.peer_addr().unwrap().ip().to_string();
  // Verify a session cookie exists
  match req.cookie(SESSION_COOKIE_NAME) {
    // Validate the session
    Some(cookie) => {
      let session_id = cookie.value().to_string();
      let session = match Session::replace(&session_id, &ip_address).await {
        Ok(session) => session,
        Err(err) => {
          log::error!(
            "Invalid session validate attempt [Session: {}] [IP Address: {}]",
            session_id,
            ip_address
          );
          return ResponseError::error_response(&Error::new(500, err.to_string()));
        }
      };
      let email = &session.email;
      let query_user = match User::select(&email).await {
        Some(query_user) => query_user,
        None => {
          return HttpResponse::Unauthorized()
            .cookie(Session::empty_cookie())
            .finish()
        }
      };

      let user_response: UserResponse = query_user.into();
      let session_cookie = session.cookie();

      log::info!(
        "Successful session validate attempt [Email: {}] [IP Address: {}]",
        email,
        ip_address
      );
      HttpResponse::Ok()
        .cookie(session_cookie)
        .json(user_response)
    }
    None => HttpResponse::Unauthorized()
      .cookie(Session::empty_cookie())
      .finish(),
  }
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
async fn password_reset(req: HttpRequest, _auth: Auth) -> HttpResponse {
  let _ip_address = req.peer_addr().unwrap().ip().to_string();
  HttpResponse::Ok().finish()
}

pub fn init_routes(config: &mut web::ServiceConfig) {
  config.service(
    web::scope("account")
      .service(register)
      .service(login)
      .service(logout)
      .service(change_password)
      .service(validate_session),
  );
}
