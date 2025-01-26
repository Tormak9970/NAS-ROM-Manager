use log::warn;
use serde::Serialize;
use serde_json::Map;
use tokio::sync::broadcast;

use crate::auth::validate_hash;

/// Wraps a Broadcast Sender to handle json data.
pub fn send<T: Serialize>(tx: broadcast::Sender<String>, message: &str, data: T) {
  let mut map = Map::new();
  map.insert(String::from("data"), serde_json::to_value(data).unwrap());

  tx.send(format!("{} {}", message, serde_json::to_string(&map).unwrap())).expect("Failed to broadcast message");
}

/// Checks the User's hash.
pub fn check_hash(hash: String, tx: broadcast::Sender<String>) -> bool {
  let is_valid = validate_hash(hash, tx.clone());

  if !is_valid {
    warn!("Password hashes do not match!");
    send(tx, "hash_mismatch", String::from(""));
  }

  return is_valid;
}