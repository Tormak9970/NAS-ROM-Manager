mod auth;
mod library_manager;
mod settings;
mod types;
mod utils;
mod watcher;
mod ws_handler;
mod file_picker;

use types::{
  settings::get_default_settings,
  library::ParserStore
};
use warp::Filter;
use watcher::Watcher;
use std::{collections::HashMap, sync::{Arc, Mutex}};
use tokio::sync::broadcast;

/// Initializes the websocket api.
pub fn initialize_websocket_api() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
  let tx = Arc::new(Mutex::new(broadcast::channel(100).0));
  let settings = Arc::new(Mutex::new(get_default_settings()));
  let parser_store = Arc::new(Mutex::new(ParserStore {
    libraries: HashMap::new()
  }));

  let tx_ws = tx.clone();
  let settings_ws = settings.clone();
  let parser_store_ws = parser_store.clone();

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
      let parser_store = parser_store_ws.clone();

      ws.on_upgrade(move |websocket| ws_handler::handle_connection(
        websocket,
        tx,
        settings,
        watcher,
        parser_store
      ))
    });

  return ws_route;
}