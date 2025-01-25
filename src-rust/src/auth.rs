use std::env::var;

use crypto::{digest::Digest, sha2::Sha256};
use log::warn;
use tokio::sync::broadcast;

/// Hashes a string using SHA256.
fn hash(text: String) -> String {
  let mut hasher = Sha256::new();

  hasher.input_str(&text);

  let result = hasher.result_str();

  return result;
}

pub fn validate_hash(password_hash: String, tx: broadcast::Sender<String>) -> bool {
  let env_password_res = var("NRM_PASSWORD");
  if env_password_res.is_err() {
    warn!("No password variable \"NRM_PASSWORD\" was found!");
    tx.send(format!("missing_env_variable NRM_PASSWORD")).expect("Failed to broadcast message");
    return false;
  }

  let env_password = env_password_res.unwrap();
  let env_password_hash = hash(env_password);

  return password_hash == env_password_hash;
}

/// password is already hashed with SHA2
pub fn authenticate_user(username: String, password_hash: String, tx: broadcast::Sender<String>) -> bool {
  let env_username_res = var("NRM_USERNAME");
  if env_username_res.is_err() {
    warn!("No environment variable \"NRM_USERNAME\" was found!");
    tx.send(format!("missing_env_variable NRM_USERNAME")).expect("Failed to broadcast message");
    return false;
  }

  let env_username = env_username_res.unwrap();
  
  let env_password_res = var("NRM_PASSWORD");
  if env_password_res.is_err() {
    warn!("No password variable \"NRM_PASSWORD\" was found!");
    tx.send(format!("missing_env_variable NRM_PASSWORD")).expect("Failed to broadcast message");
    return false;
  }

  let env_password = env_password_res.unwrap();
  let env_password_hash = hash(env_password);

  return username == env_username && password_hash == env_password_hash;
}