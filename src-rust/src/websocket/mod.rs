mod auth;
mod library_manager;
mod settings;
mod types;
mod utils;
mod watcher;
mod ws_handler;
mod file_picker;

use sysinfo::Disks;
use types::{
  settings::get_default_settings,
  library::StateStore
};
use warp::Filter;
use watcher::Watcher;
use std::{collections::HashMap, sync::{Arc, Mutex}};
use tokio::sync::broadcast;

/// Initializes the websocket api.
pub fn initialize_websocket_api() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
  let tx = Arc::new(Mutex::new(broadcast::channel(100).0));
  let settings = Arc::new(Mutex::new(get_default_settings()));
  
  sysinfo::set_open_files_limit(0);
  let disks = Arc::new(Mutex::new(Disks::new()));

  let state_store = Arc::new(Mutex::new(StateStore {
    library: get_default_settings().library,
    parsers: HashMap::new()
  }));

  let tx_ws = tx.clone();
  let settings_ws = settings.clone();
  let disks_ws = disks.clone();
  let state_store_ws = state_store.clone();

  let watcher_core = Watcher::new();
  watcher_core.init(tx.lock().unwrap().to_owned());

  let watcher_arc = Arc::new(Mutex::new(watcher_core));
  let watcher_ws = watcher_arc.clone();

  let ws_route = warp::path("ws")
    .and(warp::ws())
    .map(move |ws: warp::ws::Ws| {
      let tx = tx_ws.clone();
      let settings = settings_ws.clone();
      let watcher = watcher_ws.clone();
      let disks = disks_ws.clone();
      let state_store = state_store_ws.clone();

      ws.on_upgrade(move |websocket| ws_handler::handle_connection(
        websocket,
        tx,
        disks,
        settings,
        watcher,
        state_store
      ))
    });

  return ws_route;
}