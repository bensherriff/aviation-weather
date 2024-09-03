use argon2::{
  password_hash::{rand_core::OsRng, SaltString},
  Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

mod model;
mod routes;
mod session;

pub use model::*;
pub use session::*;
pub use routes::init_routes;

use crate::error::{ApiError, ApiResult};

pub const SESSION_COOKIE_NAME: &str = "session";

pub fn csprng_128bit(take: usize) -> String {
  // Generate a CSPRNG 128-bit (16 byte) ID using alphanumeric characters (a-z, A-Z, 0-9)
  let rng = ChaCha20Rng::from_entropy();
  rng
    .sample_iter(rand::distributions::Alphanumeric)
    .take(take)
    .map(char::from)
    .collect()
}

pub fn hash(str: &str) -> ApiResult<String> {
  let salt = SaltString::generate(&mut OsRng);
  let bytes = str.as_bytes();
  let hash = Argon2::default().hash_password(bytes, &salt)?.to_string();
  Ok(hash)
}

pub fn verify_hash(str: &str, hash: &str) -> bool {
  let bytes = str.as_bytes();
  let parsed_hash = match PasswordHash::new(hash) {
    Ok(h) => h,
    Err(_) => return false,
  };
  match Argon2::default().verify_password(bytes, &parsed_hash) {
    Ok(_) => true,
    Err(_) => false,
  }
}

pub fn verify_role(auth: &Auth, role: &str) -> ApiResult<()> {
  if auth.user.role == role {
    Ok(())
  } else {
    Err(ApiError {
      status: 403,
      message: "User does not have permission to perform this action.".to_string(),
    })
  }
}
