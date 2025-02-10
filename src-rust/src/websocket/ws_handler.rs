use futures_util::{SinkExt, StreamExt};
use log::warn;
use warp::filters::ws::{Message, WebSocket};
use std::{env::var, sync::{Arc, Mutex}};
use tokio::sync::broadcast;

use crate::websocket::{
  auth::authenticate_user,
  library_manager::{add_library, load_libraries, parse_added_rom},
  settings::{load_settings, set_setting, write_settings},
  types::{
    args::{
      AuthArgs,
      ModifyLibraryArgs,
      SetSettingArgs,
      SimpleArgs,
      ParseRomArgs
    },
    library::ParserStore,
    settings::Settings,
    ErrorSender
  },
  utils::{check_hash, send, get_error_sender},
  watcher::Watcher
};


fn handle_message(
  message: &str,
  data: &str,
  tx: broadcast::Sender<String>,
  settings: Arc<Mutex<Settings>>,
  watcher: Arc<Mutex<Watcher>>,
  parser_store: Arc<Mutex<ParserStore>>
) {
  let send_error: ErrorSender = get_error_sender(tx.clone());

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

      let mut state_settings = settings.lock().expect("Failed to lock Settings Mutex.");
      let saved_settings = load_settings(send_error);

      // If loading failed, we've already notfied the frontend of that, so we don't need to here.
      if saved_settings.is_ok() {
        let settings = saved_settings.unwrap();
        *state_settings = settings.clone();

        send(tx, "load_settings", &settings);
      }
    }
    "write_settings" => {
      let args: SimpleArgs = serde_json::from_str(data).unwrap();
      let valid = check_hash(args.passwordHash, tx.clone());
      if !valid {
        return;
      }

      let state_settings = settings.lock().expect("Failed to lock Settings Mutex.");
      let success = write_settings(state_settings, send_error);

      // If write failed, we've already notfied the frontend of that, so we don't need to here.
      if success {
        send(tx, "write_settings", success);
      }
      
    }
    "set_setting" => {
      let args: SetSettingArgs = serde_json::from_str(data).unwrap();
      let valid = check_hash(args.passwordHash, tx.clone());
      if !valid {
        return;
      }

      let mut state_settings = settings.lock().expect("Failed to lock Settings Mutex.");
      set_setting(&mut state_settings, &args.key, args.value);
      
      let success = write_settings(state_settings, send_error);

      // If write failed, we've already notfied the frontend of that, so we don't need to here.
      if success {
        send(tx, "set_setting", success);
      }
    }
    "load_libraries" => {
      let args: SimpleArgs = serde_json::from_str(data).unwrap();
      let valid = check_hash(args.passwordHash, tx.clone());
      if !valid {
        return;
      }

      let state_settings = settings.lock().expect("Failed to lock Settings Mutex.");
      let state_watcher = watcher.lock().expect("Failed to lock Watcher Mutex.");
      let mut state_parsers = parser_store.lock().expect("Failed to lock ParserStore Mutex.");
      let libraries_res = load_libraries(
        &state_settings,
        &state_watcher,
        &mut state_parsers,
        send_error
      );

      // If loading failed, we've already notfied the frontend of that, so we don't need to here.
      if libraries_res.is_ok() {
        send(tx, "load_libraries", libraries_res.unwrap());
      }
    }
    "add_library" => {
      let args: ModifyLibraryArgs = serde_json::from_str(data).unwrap();
      let valid = check_hash(args.passwordHash, tx.clone());
      if !valid {
        return;
      }
      
      let state_watcher = watcher.lock().expect("Failed to lock Watcher Mutex.");
      let mut state_parsers = parser_store.lock().expect("Failed to lock ParserStore Mutex.");
      let library_res = add_library(
        &args.library,
        &state_watcher,
        &mut state_parsers,
        send_error
      );

      // If loading failed, we've already notfied the frontend of that, so we don't need to here.
      if library_res.is_ok() {
        send(tx, "add_library", library_res.unwrap());
      }
    }
    "remove_library" => {
      let args: ModifyLibraryArgs = serde_json::from_str(data).unwrap();
      let valid = check_hash(args.passwordHash, tx.clone());
      if !valid {
        return;
      }

      let state_watcher = watcher.lock().expect("Failed to lock Watcher Mutex.");
      state_watcher.unwatch_library(args.library.path);

      let mut state_parsers = parser_store.lock().expect("Failed to lock ParserStore Mutex.");
      state_parsers.libraries.remove(&args.library.name);

      send(tx, "remove_library", true);
    }
    "parse_rom" => {
      let args: ParseRomArgs = serde_json::from_str(data).unwrap();
      let valid = check_hash(args.passwordHash, tx.clone());
      if !valid {
        return;
      }

      let state_parsers = parser_store.lock().expect("Failed to lock ParserStore Mutex.");
      let rom_res = parse_added_rom(
        args.libraryName,
        args.parser,
        &args.romPath,
        &state_parsers,
        send_error
      );

      if rom_res.is_ok() {
        send(tx, "parse_rom", rom_res.unwrap());
      }
    }
    "get_sgdb_key" => {
      let args: SimpleArgs = serde_json::from_str(data).unwrap();
      let valid = check_hash(args.passwordHash, tx.clone());
      if !valid {
        return;
      }

      let env_api_key_res = var("SGDB_API_KEY");
      if env_api_key_res.is_err() {
        warn!("No environment variable \"SGDB_API_KEY\" was found!");
        send(tx, "missing_env_variable", "SGDB_API_KEY");
      } else {
        send(tx, "get_sgdb_key", env_api_key_res.unwrap());
      }
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
pub async fn handle_connection(
  ws: WebSocket,
  tx: Arc<Mutex<broadcast::Sender<String>>>,
  settings: Arc<Mutex<Settings>>,
  watcher: Arc<Mutex<Watcher>>,
  parser_store: Arc<Mutex<ParserStore>>
) {
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

          handle_message(
            message,
            data,
            tx.lock().unwrap().to_owned(),
            settings.clone(),
            watcher.clone(),
            parser_store.clone()
          );
        }
      },
      Err(_e) => break,
    }
  }
}