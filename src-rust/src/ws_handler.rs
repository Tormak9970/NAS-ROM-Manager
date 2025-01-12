use futures_util::{SinkExt, StreamExt};
use log::warn;
use serde::Serialize;
use serde_json::Map;
use warp::filters::ws::{Message, WebSocket};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

use crate::{auth::{authenticate_user, validate_hash}, library_manager::{add_library, load_libraries, remove_library}, settings::{load_settings, set_setting, write_settings}, types::{AuthArgs, ModifyLibraryArgs, SetSettingArgs, Settings, SimpleArgs}};

fn send<T: Serialize>(tx: broadcast::Sender<String>, message: &str, data: T) {
  let mut map = Map::new();
  map.insert(String::from("data"), serde_json::to_value(data).unwrap());

  tx.send(format!("{} {}", message, serde_json::to_string(&map).unwrap())).expect("Failed to broadcast message");
}

fn check_hash(hash: String, tx: broadcast::Sender<String>) -> bool {
  let is_valid = validate_hash(hash, tx.clone());

  if !is_valid {
    warn!("Password hashes do not match!");
    send(tx, "hash_mismatch", String::from(""));
    return false;
  }

  return true;
}


fn handle_message(message: &str, data: &str, tx: broadcast::Sender<String>, settings: Arc<Mutex<Settings>>) {
  match message {
    "user_auth" => {
      let args: AuthArgs = serde_json::from_str(data).unwrap();
      let username = args.user;
      let password_hash = args.passwordHash;

      let result = authenticate_user(username.clone(), password_hash, tx.clone());

      send(tx, "user_auth", result);
    }
    "load_settings" => {
      let args: SimpleArgs = serde_json::from_str(data).unwrap();
      let valid = check_hash(args.passwordHash, tx.clone());
      if !valid {
        return;
      }

      let mut state_settings = settings.lock().expect("Should have been able to lock Settings Mutex.");
      let saved_settings = load_settings();

      *state_settings = saved_settings.clone();

      send(tx, "load_settings", &saved_settings);
    }
    "write_settings" => {
      let args: SimpleArgs = serde_json::from_str(data).unwrap();
      let valid = check_hash(args.passwordHash, tx.clone());
      if !valid {
        return;
      }

      let state_settings = settings.lock().expect("Should have been able to lock Settings Mutex.");
      let success = write_settings(state_settings);
      
      send(tx, "write_settings", success);
    }
    "set_setting" => {
      let args: SetSettingArgs = serde_json::from_str(data).unwrap();
      let valid = check_hash(args.passwordHash, tx.clone());
      if !valid {
        return;
      }

      let mut state_settings = settings.lock().expect("Should have been able to lock Settings Mutex.");
      set_setting(&mut state_settings, &args.key, args.value);
      
      let success = write_settings(state_settings);

      send(tx, "set_setting", success);
    }
    "load_libraries" => {
      let args: SimpleArgs = serde_json::from_str(data).unwrap();
      let valid = check_hash(args.passwordHash, tx.clone());
      if !valid {
        return;
      }

      let state_settings = settings.lock().expect("Should have been able to lock Settings Mutex.");
      let libraries = load_libraries(state_settings);

      send(tx, "load_libraries", libraries);
    }
    "add_library" => {
      let args: ModifyLibraryArgs = serde_json::from_str(data).unwrap();
      let valid = check_hash(args.passwordHash, tx.clone());
      if !valid {
        return;
      }

      let library = add_library(&args.library);

      send(tx, "add_library", library);
    }
    "remove_library" => {
      let args: ModifyLibraryArgs = serde_json::from_str(data).unwrap();
      let valid = check_hash(args.passwordHash, tx.clone());
      if !valid {
        return;
      }

      let success = remove_library(&args.library);

      send(tx, "remove_library", success);
    }
    "demo" => {
      let args: SimpleArgs = serde_json::from_str(data).unwrap();
      let valid = check_hash(args.passwordHash, tx.clone());
      if !valid {
        return;
      }

      // TODO: run the logic here
    }
    _ => {}
  }
}

/// Handles WebSocket Connections
pub async fn handle_connection(ws: WebSocket, tx: Arc<Mutex<broadcast::Sender<String>>>, settings: Arc<Mutex<Settings>>) {
  let (mut ws_sender, mut ws_receiver) = ws.split();
  let mut rx = tx.lock().unwrap().subscribe();

  // * Spawn the Message Propegation Thread.
  tokio::spawn(async move {
    while let Ok(msg) = rx.recv().await {
      if ws_sender.send(Message::text(msg)).await.is_err() {
        break;
      }
    }
  });

  while let Some(result) = ws_receiver.next().await {
    match result {
      Ok(message) => {
        if let Ok(text) = message.to_str() {
          let (message, data) = text.split_once(" ").unwrap();

          handle_message(message, data, tx.lock().unwrap().to_owned(), settings.clone());
        }
      },
      Err(_e) => break,
    }
  }
}