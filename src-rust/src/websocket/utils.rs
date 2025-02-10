use log::warn;
use serde::Serialize;
use serde_json::{Map, Value};
use tokio::sync::broadcast;

use crate::websocket::auth::validate_hash;

use super::types::{BackendErrorType, ErrorSender};

/// Wraps a Broadcast Sender to handle json data.
pub fn send<T: Serialize>(tx: broadcast::Sender<String>, message: &str, data: T) {
  let mut map = Map::new();
  map.insert(String::from("data"), serde_json::to_value(data).unwrap());

  tx.send(format!("{} {}", message, serde_json::to_string(&map).unwrap())).expect("Failed to broadcast message");
}

/// Gets the sender to send errors to the frontend.
pub fn get_error_sender(tx: broadcast::Sender<String>) -> ErrorSender {
  let error_sender = tx.clone();

  return Box::new(move | message: String, fix: String, error_type: BackendErrorType | {
    warn!("{}", &message);

    let mut data = Map::new();
    data.insert(String::from("message"), Value::String(message));
    data.insert(String::from("fix"), Value::String(fix));
    data.insert(String::from("type"), serde_json::to_value(error_type).unwrap());

    let mut map = Map::new();
    map.insert(String::from("data"), serde_json::to_value(data).unwrap());

    error_sender.send(format!("backend_error {}", serde_json::to_string(&map).unwrap())).expect("Failed to send error message");
  });
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