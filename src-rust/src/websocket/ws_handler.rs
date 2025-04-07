use futures_util::{SinkExt, StreamExt};
use warp::filters::ws::{Message, WebSocket};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use sysinfo::{DiskRefreshKind, Disks};

use crate::websocket::{
  auth::authenticate_user,
  library_manager::{parse_library, parse_added_rom},
  settings::{load_settings, set_setting, write_settings},
  types::{
    args::{
      AuthArgs,
      ModifyLibraryArgs,
      SetSettingArgs,
      SimpleArgs,
      ParseRomArgs,
      FilePickerArgs,
      MetadataArgs
    },
    library::StateStore,
    settings::Settings,
    ErrorSender,
    AvailableStorage
  },
  utils::{check_hash, send, get_error_sender},
  watcher::Watcher,
  file_picker::get_entries
};

use super::{metadata::{load_metadata, write_metadata}, parsers::{delete_parser, write_parsers}, types::args::{DeleteParserArgs, ParsersArgs}};


fn handle_message(
  message: &str,
  data: &str,
  tx: broadcast::Sender<String>,
  disks: Arc<Mutex<Disks>>,
  settings: Arc<Mutex<Settings>>,
  watcher: Arc<Mutex<Watcher>>,
  state_store: Arc<Mutex<StateStore>>
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
      let mut state = state_store.lock().expect("Failed to lock State Mutex.");
      let saved_settings = load_settings(send_error);

      // If loading failed, we've already notfied the frontend of that, so we don't need to here.
      if saved_settings.is_ok() {
        let settings = saved_settings.unwrap();
        *state_settings = settings.clone();
        (*state).library = settings.library.clone();

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
    "load_library" => {
      let args: SimpleArgs = serde_json::from_str(data).unwrap();
      let valid = check_hash(args.passwordHash, tx.clone());
      if !valid {
        return;
      }

      let state_settings = settings.lock().expect("Failed to lock Settings Mutex.");
      let state_watcher = watcher.lock().expect("Failed to lock Watcher Mutex.");
      let mut state = state_store.lock().expect("Failed to lock State Mutex.");
      let library_res = parse_library(
        &state_settings.library,
        &state_watcher,
        &mut state,
        send_error
      );

      // If loading failed, we've already notfied the frontend of that, so we don't need to here.
      if library_res.is_ok() {
        send(tx, "load_library", library_res.unwrap());
      }
    }
    "update_library" => {
      let args: ModifyLibraryArgs = serde_json::from_str(data).unwrap();
      let valid = check_hash(args.passwordHash, tx.clone());
      if !valid {
        return;
      }
      
      let state_watcher = watcher.lock().expect("Failed to lock Watcher Mutex.");
      let mut state = state_store.lock().expect("Failed to lock State Mutex.");
      let library_res = parse_library(
        &args.library,
        &state_watcher,
        &mut state,
        send_error
      );

      // If loading failed, we've already notfied the frontend of that, so we don't need to here.
      if library_res.is_ok() {
        (*state).library = args.library.clone();

        send(tx, "update_library", library_res.unwrap());
      }
    }
    "load_metadata" => {
      let args: SimpleArgs = serde_json::from_str(data).unwrap();
      let valid = check_hash(args.passwordHash, tx.clone());
      if !valid {
        return;
      }
      
      let mut state = state_store.lock().expect("Failed to lock State Mutex.");
      let metadata_res = load_metadata(send_error);

      // If loading failed, we've already notfied the frontend of that, so we don't need to here.
      if metadata_res.is_ok() {
        let metadata = metadata_res.unwrap();
        (*state).metadata = metadata.clone();

        send(tx, "load_metadata", &metadata);
      }
    }
    "save_metadata" => {
      let args: MetadataArgs = serde_json::from_str(data).unwrap();
      let valid = check_hash(args.passwordHash, tx.clone());
      if !valid {
        return;
      }
      
      let mut state = state_store.lock().expect("Failed to lock State Mutex.");
      let success = write_metadata(&args.data, send_error);

      if success {
        (*state).metadata = args.data;
        send(tx, "save_metadata", success);
      }
    }
    "save_parsers" => {
      let args: ParsersArgs = serde_json::from_str(data).unwrap();
      let valid = check_hash(args.passwordHash, tx.clone());
      if !valid {
        return;
      }
      
      let mut state = state_store.lock().expect("Failed to lock State Mutex.");
      let success = write_parsers(&args.data, send_error);

      if success {
        (*state).parsers = args.data;
        send(tx, "save_parsers", success);
      }
    }
    "delete_parser" => {
      let args: DeleteParserArgs = serde_json::from_str(data).unwrap();
      let valid = check_hash(args.passwordHash, tx.clone());
      if !valid {
        return;
      }
      
      let mut state = state_store.lock().expect("Failed to lock State Mutex.");
      let parser = state.parsers.get(&args.abbreviation).expect("Failed to get parser with provided abbreviation.");
      let success = delete_parser(parser, send_error);

      if success {
        (*state).parsers.remove(&args.abbreviation);
        send(tx, "delete_parser", success);
      }
    }
    "parse_rom" => {
      let args: ParseRomArgs = serde_json::from_str(data).unwrap();
      let valid = check_hash(args.passwordHash, tx.clone());
      if !valid {
        return;
      }

      let state = state_store.lock().expect("Failed to lock State Mutex.");
      let rom_res = parse_added_rom(
        args.parser,
        &args.romPath,
        &state,
        send_error
      );

      if rom_res.is_ok() {
        send(tx, "parse_rom", rom_res.unwrap());
      }
    }
    "file_picker" => {
      let args: FilePickerArgs = serde_json::from_str(data).unwrap();
      let valid = check_hash(args.passwordHash, tx.clone());
      if !valid {
        return;
      }

      let file_entries_res = get_entries(
        args.path,
        args.config,
        send_error
      );

      // If reading failed, we've already notfied the frontend of that, so we don't need to here.
      if file_entries_res.is_ok() {
        send(tx, "file_picker", file_entries_res.unwrap());
      }
    }
    "available_storage" => {
      let args: SimpleArgs = serde_json::from_str(data).unwrap();
      let valid = check_hash(args.passwordHash, tx.clone());
      if !valid {
        return;
      }

      let mut state_disks = disks.lock().expect("Failed to lock Disks");
      
      let disk_refresh_kind = DiskRefreshKind::nothing().with_storage();
      state_disks.refresh_specifics(true, disk_refresh_kind);

      let mut available_space: u64 = 0;
      let mut total_space: u64 = 0;

      for disk in state_disks.list() {
        available_space += disk.available_space();
        total_space += disk.total_space();
      }

      let info = AvailableStorage {
        usedSpace: total_space - available_space,
        totalSpace: total_space,
      };

      send(tx, "available_storage", info);
    }
    "demo" => {
      let args: SimpleArgs = serde_json::from_str(data).unwrap();
      let valid = check_hash(args.passwordHash, tx.clone());
      if !valid {
        return;
      }
    }
    _ => {}
  }
}

/// Handles WebSocket Connections
pub async fn handle_connection(
  ws: WebSocket,
  tx: Arc<Mutex<broadcast::Sender<String>>>,
  disks: Arc<Mutex<Disks>>,
  settings: Arc<Mutex<Settings>>,
  watcher: Arc<Mutex<Watcher>>,
  state_store: Arc<Mutex<StateStore>>
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
            disks.clone(),
            settings.clone(),
            watcher.clone(),
            state_store.clone()
          );
        }
      },
      Err(_e) => break,
    }
  }
}