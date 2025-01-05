use std::env::var;

use crypto::{digest::Digest, sha2::Sha256};
use log::warn;

/// Hashes a string using SHA256.
fn hash(text: String) -> String {
  let mut hasher = Sha256::new();

  hasher.input_str(&text);

  let result = hasher.result_str();

  return result;
}

/// password is already hashed with SHA2
pub fn authenticate_user(username: String, password_hash: String) -> bool {
  let env_username_res = var("ROM_MANAGER_USERNAME");
  if env_username_res.is_err() {
    warn!("No environment variable \"ROM_MANAGER_USERNAME\" was found!");
    return false;
  }

  let env_username = env_username_res.unwrap();
  
  let env_password_res = var("ROM_MANAGER_PASSWORD");
  if env_password_res.is_err() {
    warn!("No password variable \"ROM_MANAGER_PASSWORD\" was found!");
    return false;
  }

  let env_password = env_password_res.unwrap();
  let env_password_hash = hash(env_password);

  return username == env_username && password_hash == env_password_hash;
}