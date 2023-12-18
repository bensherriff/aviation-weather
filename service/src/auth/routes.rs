use std::env;

use actix_web::{get, post, web, HttpResponse, ResponseError, cookie::{Cookie, time::Duration}, HttpRequest};
use log::error;
use redis::AsyncCommands;
use serde::{Serialize, Deserialize};
use crate::{error_handler::ServiceError, db::Response};

use crate::{auth::{LoginRequest, RegisterUser, InsertUser, QueryUser, verify_password, JwtAuth, verify_token, generate_access_token, generate_refresh_token}, db};

#[post("/register")]
async fn register(user: web::Json<RegisterUser>) -> HttpResponse {
  let register_user = user.0;
  let insert_user: InsertUser = match register_user.convert_to_insert() {
    Ok(user) => user,
    Err(err) => return ResponseError::error_response(&err)
  };
  match InsertUser::insert(insert_user) {
    Ok(_) => {
      HttpResponse::Created().finish()
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
async fn login(request: web::Json<LoginRequest>) -> HttpResponse {
  let email = request.email.clone();

  let query_user = match QueryUser::get_by_email(&email) {
    Ok(query_user) => query_user,
    Err(err) => return ResponseError::error_response(&err)
  };
  let hash = &query_user.hash;
  let password = request.password.as_bytes();
  match verify_password(hash, password) {
    Ok(_) => {
      let access_token_details = match generate_access_token(&email) {
        Ok(token_details) => token_details,
        Err(err) => {
          error!("Failed to generate access token: {}", err);
          return ResponseError::error_response(&err)
        }
      };
      
      let refresh_token_details = match generate_refresh_token(&email) {
        Ok(token_details) => token_details,
        Err(err) => {
          error!("Failed to generate refresh token: {}", err);
          return ResponseError::error_response(&err)
        }
      };

      let mut conn = match db::redis_async_connection().await {
        Ok(conn) => conn,
        Err(err) => {
          error!("Failed to get redis connection: {}", err);
          return ResponseError::error_response(&err)
        }
      };

      let access_token_max_age = env::var("ACCESS_TOKEN_MAXAGE")
        .expect("ACCESS_TOKEN_MAXAGE must be set")
        .parse::<i64>()
        .expect("ACCESS_TOKEN_MAXAGE must be an integer");

      let refresh_token_max_age = env::var("REFRESH_TOKEN_MAXAGE")
        .expect("REFRESH_TOKEN_MAXAGE must be set")
        .parse::<i64>()
        .expect("REFRESH_TOKEN_MAXAGE must be an integer");

      let access_result: redis::RedisResult<()> = conn.set_ex(access_token_details.token_uuid.to_string(), &email, (access_token_max_age * 60) as usize).await;
      if let Err(err) = access_result {
        error!("Failed to set access token in redis: {}", err);
        return ResponseError::error_response(&ServiceError {
          status: 500,
          message: format!("Failed to set access token in redis: {}", err)
        })
      };

      let refresh_result: redis::RedisResult<()> = conn.set_ex(refresh_token_details.token_uuid.to_string(), &email, (refresh_token_max_age * 60) as usize).await;
      if let Err(err) = refresh_result {
        error!("Failed to set refresh token in redis: {}", err);
        return ResponseError::error_response(&ServiceError {
          status: 500,
          message: format!("Failed to set refresh token in redis: {}", err)
        })
      };

      let access_cookie = Cookie::build("access_token", access_token_details.token.clone().unwrap())
        .path("/")
        .max_age(Duration::new(access_token_max_age * 60, 0))
        .http_only(true)
        .secure(true)
        .finish();
      let refresh_cookie = Cookie::build("refresh_token", refresh_token_details.token.clone().unwrap())
        .path("/")
        .max_age(Duration::new(refresh_token_max_age * 60, 0))
        .http_only(true)
        .secure(true)
        .finish();
      let logged_in_cookie = Cookie::build("logged_in", "true")
        .path("/")
        .max_age(Duration::new(access_token_max_age * 60, 0))
        .http_only(false)
        .finish();

      let access_token_uuid = uuid::Uuid::parse_str(&access_token_details.token_uuid.to_string()).unwrap();

      HttpResponse::Ok()
        .cookie(access_cookie)
        .cookie(refresh_cookie)
        .cookie(logged_in_cookie)
        .json(JwtAuth { token: access_token_uuid, user: query_user.into() })
    },
    Err(err) => ResponseError::error_response(&ServiceError {
      status: 401,
      message: err.to_string()
    })
  }
}

#[get("/session")]
async fn session(req: HttpRequest) -> HttpResponse {
  let keys_dir = env::var("KEYS_DIR_PATH").expect("KEYS_DIR_PATH must be set");
  // If there is a access_token cookie, check if it is valid
  let has_session = match req.cookie("access_token") {
    Some(cookie) => {
      let access_token = cookie.value().to_string();
      let public_key = std::fs::read_to_string(format!("{}access_public_key.pem", keys_dir))
    .expect("Unable to read refresh public key");
      match verify_token(&access_token, &public_key) {
        Ok(_) => true,
        Err(_) => false
      }
    },
    None => false
  };
  if !has_session {
    // If there is a refresh_token cookie, check if it is valid
    match req.cookie("refresh_token") {
      Some(cookie) => {
        let refresh_token = cookie.value().to_string();
        let public_key = std::fs::read_to_string(format!("{}/refresh_public_key.pem", keys_dir))
    .expect("Unable to read refresh public key");
        match verify_token(&refresh_token, &public_key) {
          Ok(_) => return HttpResponse::Ok().json(true),
          Err(_) => return HttpResponse::Ok().json(false)
        };
      },
      None => return HttpResponse::Ok().json(false)
    };
  } else {
    return HttpResponse::Ok().json(true)  
  }
}

#[derive(Serialize, Deserialize)]
struct RefreshParams {
  refresh_token_rotation: Option<bool>
}

#[get("/refresh")]
async fn refresh(req: HttpRequest) -> HttpResponse {
  let params = match web::Query::<RefreshParams>::from_query(req.query_string()) {
    Ok(params) => params,
    Err(err) => return ResponseError::error_response(&ServiceError {
      status: 422,
      message: err.to_string()
    })
  };

  let refresh_token = match req.cookie("refresh_token") {
    Some(cookie) => cookie.value().to_string(),
    None => return ResponseError::error_response(&ServiceError {
      status: 401,
      message: "Refresh token not found".to_string()
    })
  };

  let keys_dir = env::var("KEYS_DIR_PATH").expect("KEYS_DIR_PATH must be set");
  let public_key = std::fs::read_to_string(format!("{}/refresh_public_key.pem", keys_dir))
    .expect("Unable to read refresh public key");
  let refresh_token_details = match verify_token(&refresh_token, &public_key) {
    Ok(token_details) => token_details,
    Err(err) => return ResponseError::error_response(&err)
  };

  let email = refresh_token_details.email.clone();

  match QueryUser::get_by_email(&email) {
    Ok(query_user) => {
      let access_token_details = match generate_access_token(&email) {
        Ok(token_details) => token_details,
        Err(err) => {
          error!("Failed to generate access token: {}", err);
          return ResponseError::error_response(&err)
        }
      };

      let mut conn = match db::redis_async_connection().await {
        Ok(conn) => conn,
        Err(err) => {
          error!("Failed to get redis connection: {}", err);
          return ResponseError::error_response(&err)
        }
      };

      // Delete old auth token if it exists
      match req.cookie("access_token") {
        Some(cookie) => {
          let access_token = cookie.value().to_string();
          let keys_dir = env::var("KEYS_DIR_PATH").expect("KEYS_DIR_PATH must be set");
          let public_key = std::fs::read_to_string(format!("{}/access_public_key.pem", keys_dir))
            .expect("Unable to read access public key");
          match verify_token(&access_token, &public_key) {
            Ok(token_details) => {
              let _: redis::RedisResult<()> = conn.del(token_details.token_uuid.to_string()).await;  
              
            },
            Err(_) => {}
          };
        },
        None => {}
      };
    
      let access_token_max_age = env::var("ACCESS_TOKEN_MAXAGE")
        .expect("ACCESS_TOKEN_MAXAGE must be set")
        .parse::<i64>()
        .expect("ACCESS_TOKEN_MAXAGE must be an integer");
    
      let access_result: redis::RedisResult<()> = conn.set_ex(access_token_details.token_uuid.to_string(), &email, (access_token_max_age * 60) as usize).await;
      if let Err(err) = access_result {
        error!("Failed to set access token in redis: {}", err);
        return ResponseError::error_response(&ServiceError {
          status: 500,
          message: format!("Failed to set access token in redis: {}", err)
        })
      };
    
      let access_cookie = Cookie::build("access_token", access_token_details.token.clone().unwrap())
        .path("/")
        .max_age(Duration::new(access_token_max_age * 60, 0))
        .http_only(true)
        .secure(true)
        .finish();
      let logged_in_cookie = Cookie::build("logged_in", "true")
        .path("/")
        .max_age(Duration::new(access_token_max_age * 60, 0))
        .http_only(false)
        .finish();
    
      let access_token_uuid = uuid::Uuid::parse_str(&access_token_details.token_uuid.to_string()).unwrap();

      // Refresh the refresh token if requested
      let refresh_token_rotation = match params.refresh_token_rotation {
        Some(refresh_token_rotation) => refresh_token_rotation,
        None => false
      };
      if refresh_token_rotation {
        // Delete the old refresh token
        let _: redis::RedisResult<()> = conn.del(refresh_token_details.token_uuid.to_string()).await;

        let refresh_token_details = match generate_refresh_token(&refresh_token_details.email) {
          Ok(token_details) => token_details,
          Err(err) => {
            error!("Failed to generate refresh token: {}", err);
            return ResponseError::error_response(&err)
          }
        };
    
        let refresh_token_max_age = env::var("REFRESH_TOKEN_MAXAGE")
          .expect("REFRESH_TOKEN_MAXAGE must be set")
          .parse::<i64>()
          .expect("REFRESH_TOKEN_MAXAGE must be an integer");

        let refresh_result: redis::RedisResult<()> = conn.set_ex(refresh_token_details.token_uuid.to_string(), &refresh_token_details.email, (refresh_token_max_age * 60) as usize).await;
        if let Err(err) = refresh_result {
          error!("Failed to set refresh token in redis: {}", err);
          return ResponseError::error_response(&ServiceError {
            status: 500,
            message: format!("Failed to set refresh token in redis: {}", err)
          })
        };
    
        let refresh_cookie = Cookie::build("refresh_token", refresh_token_details.token.clone().unwrap())
          .path("/")
          .max_age(Duration::new(refresh_token_max_age * 60, 0))
          .http_only(true)
          .secure(true)
          .finish();
    
        HttpResponse::Ok()
          .cookie(refresh_cookie)
          .cookie(access_cookie)
          .cookie(logged_in_cookie)
          .json(JwtAuth { token: access_token_uuid, user: query_user.into() })
      } else {
        HttpResponse::Ok()
        .cookie(access_cookie)
        .cookie(logged_in_cookie)
        .json(JwtAuth { token: access_token_uuid, user: query_user.into() })
      }
    },
    Err(err) => return ResponseError::error_response(&err)
  }
}

#[post("/logout")]
async fn logout(req: HttpRequest, auth: JwtAuth) -> HttpResponse {
  let refresh_token = match req.cookie("refresh_token") {
    Some(cookie) => cookie.value().to_string(),
    None => return ResponseError::error_response(&ServiceError {
      status: 401,
      message: "Refresh token not found".to_string()
    })
  };
  let keys_dir = env::var("KEYS_DIR_PATH").expect("KEYS_DIR_PATH must be set");
  let public_key = std::fs::read_to_string(format!("{}/refresh_public_key.pem", keys_dir))
    .expect("Unable to read refresh public key");
  let refresh_token_details = match verify_token(&refresh_token, &public_key) {
    Ok(token_details) => token_details,
    Err(err) => return ResponseError::error_response(&err)
  };

  let mut conn = match db::redis_async_connection().await {
    Ok(conn) => conn,
    Err(err) => {
      error!("Failed to get redis connection: {}", err);
      return ResponseError::error_response(&err)
    }
  };

  let access_result: redis::RedisResult<()> = conn.del(&[
    refresh_token_details.token_uuid.to_string(),
    auth.token.to_string()
  ]).await;
  if let Err(err) = access_result {
    error!("Failed to set access token in redis: {}", err);
    return ResponseError::error_response(&ServiceError {
      status: 500,
      message: format!("Failed to set access token in redis: {}", err)
    })
  };

  let access_cookie = Cookie::build("access_token", "")
    .path("/")
    .max_age(Duration::new(-1, 0))
    .http_only(true)
    .finish();
  let refresh_cookie = Cookie::build("refresh_token", "")
    .path("/")
    .max_age(Duration::new(-1, 0))
    .http_only(true)
    .finish();
  let logged_in_cookie = Cookie::build("logged_in", "")
    .path("/")
    .max_age(Duration::new(-1, 0))
    .http_only(true)
    .finish();
  
  HttpResponse::Ok()
    .cookie(access_cookie)
    .cookie(refresh_cookie)
    .cookie(logged_in_cookie)
    .finish()
}

#[get("/me")]
async fn me(auth: JwtAuth) -> HttpResponse {
  HttpResponse::Ok().json(auth)
}

#[get("/roles")]
async fn roles() -> HttpResponse {
  HttpResponse::Ok().json(Response {
    data: vec!["admin", "user"],
    meta: None
  })
}

pub fn init_routes(config: &mut web::ServiceConfig) {
  let r = RegisterUser {
    email: "admin".to_string(),
    password: "admin".to_string(),
    first_name: "Admin".to_string(),
    last_name: "Admin".to_string(),
  };
  let mut u = r.convert_to_insert().unwrap();
  u.role = "admin".to_string();
  u.verified = true;
  let _ = InsertUser::insert(u);
  config.service(web::scope("auth")
    .service(register)
    .service(login)
    .service(refresh)
    .service(logout)
    .service(me)
    .service(roles)
    );
}