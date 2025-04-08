use argon2::{
  password_hash::{rand_core::OsRng, SaltString},
  Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use rand::distr::Alphanumeric;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use serde::{Deserialize, Serialize};

mod model;
mod routes;
mod session;

pub use model::*;
pub use session::*;
pub use routes::init_routes;

use crate::error::{Error, ApiResult};

pub fn csprng(take: usize) -> String {
  // Generate a CSPRNG 128-bit (16 byte) ID using alphanumeric characters (a-z, A-Z, 0-9)
  let rng = ChaCha20Rng::from_os_rng();
  rng
    .sample_iter(Alphanumeric)
    .take(take)
    .map(char::from)
    .collect()
}

pub fn hash(string: &str) -> ApiResult<String> {
  let salt = SaltString::generate(&mut OsRng);
  let hash = Argon2::default()
    .hash_password(string.as_bytes(), &salt)?
    .to_string();
  Ok(hash)
}

pub fn verify_hash(string: &str, hashed_string: &str) -> bool {
  let bytes = string.as_bytes();
  let parsed_hash = match PasswordHash::new(hashed_string) {
    Ok(h) => h,
    Err(err) => {
      log::error!(
        "Failed to construct PasswordHash from '{}': {}",
        hashed_string,
        err
      );
      return false;
    }
  };
  Argon2::default()
    .verify_password(bytes, &parsed_hash)
    .is_ok()
}

pub fn verify_role(auth: &Auth, role: &str) -> ApiResult<()> {
  if auth.user.role == role {
    Ok(())
  } else {
    Err(Error {
      status: 403,
      details: "User does not have permission to perform this action.".to_string(),
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_hash() {
    let password = hash("password").unwrap();
    assert!(!verify_hash(&password, "bad_password"));
    assert!(verify_hash("password", &password));
  }
}
