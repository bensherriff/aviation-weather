use std::env;

use argon2::{password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString, Error as HashError}, Argon2, PasswordHash};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, encode, decode, Validation, Algorithm};
use serde::{Deserialize, Serialize};

mod model;
mod routes;

pub use model::*;
pub use routes::init_routes;
use crate::error_handler::ServiceError;

#[derive(Debug, Serialize, Deserialize)]
struct TokenClaims {
    sub: String, // Subject
    token_uuid: String, // Token UUID
    iss: String, // Issuer
    exp: i64, // Expiration time
    iat: i64, // Issued At
    nbf: i64 // Not Before
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenDetails {
    pub token: Option<String>,
    pub token_uuid: uuid::Uuid,
    pub email: String,
    pub expires_in: Option<i64>
}

pub fn verify_token(token: &str, public_key: &str) -> Result<TokenDetails, ServiceError> {
  let key = DecodingKey::from_rsa_pem(public_key.as_bytes())?;
  let validation = Validation::new(Algorithm::RS256);
  let decoded = decode::<TokenClaims>(token, &key, &validation)?;
  let email = decoded.claims.sub;
  let token_uuid = uuid::Uuid::parse_str(decoded.claims.token_uuid.as_str()).unwrap();
  Ok(TokenDetails { token: None, token_uuid, email, expires_in: None })
}

pub fn generate_access_token(email: &str) -> Result<TokenDetails, ServiceError> {
  let access_token_max_age = env::var("ACCESS_TOKEN_MAXAGE")
    .expect("ACCESS_TOKEN_MAXAGE must be set")
    .parse::<i64>()
    .expect("ACCESS_TOKEN_MAXAGE must be an integer");
  let keys_dir = env::var("KEYS_DIR_PATH")?;
  let access_private_key = std::fs::read_to_string(format!("{}/access_private_key.pem", keys_dir))?;
  generate_token(&email, access_token_max_age, &access_private_key)
}

pub fn generate_refresh_token(email: &str) -> Result<TokenDetails, ServiceError> {
  let refresh_token_max_age = env::var("REFRESH_TOKEN_MAXAGE")
    .expect("REFRESH_TOKEN_MAXAGE must be set")
    .parse::<i64>()
    .expect("REFRESH_TOKEN_MAXAGE must be an integer");
  let keys_dir = env::var("KEYS_DIR_PATH")?;
  let refresh_private_key = std::fs::read_to_string(format!("{}/refresh_private_key.pem", keys_dir))?;
  generate_token(&email, refresh_token_max_age, &refresh_private_key)
}

pub fn generate_token(email: &str, ttl: i64, private_key: &str) -> Result<TokenDetails, ServiceError> {
  let now = chrono::Utc::now();
  let mut token_details = TokenDetails {
    token: None,
    token_uuid: uuid::Uuid::new_v4(),
    email: email.to_string(),
    expires_in: Some((now + chrono::Duration::minutes(ttl)).timestamp())
  };
  let claims = TokenClaims {
    sub: token_details.email.clone(),
    iss: "aviation-weather".to_string(),
    token_uuid: token_details.token_uuid.to_string(),
    exp: token_details.expires_in.unwrap(),
    iat: now.timestamp(),
    nbf: now.timestamp()
  };
  let header = Header::new(Algorithm::RS256);
  let key = EncodingKey::from_rsa_pem(private_key.as_bytes())?;
  let token = encode(&header, &claims, &key)?;
  token_details.token = Some(token);
  Ok(token_details)
}

pub fn hash_password(password: &[u8]) -> Result<String, HashError> {
  let salt = SaltString::generate(&mut OsRng);
  Ok(Argon2::default().hash_password(password, &salt)?.to_string())
}

pub fn verify_password(hash: &str, password: &[u8]) -> Result<(), HashError> {
  let parsed_hash = PasswordHash::new(hash)?;
  Ok(Argon2::default().verify_password(password, &parsed_hash)?)
}