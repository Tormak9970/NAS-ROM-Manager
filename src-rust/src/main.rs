use types::get_default_settings;
use warp::Filter;
use watcher::Watcher;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use dotenv::dotenv;

mod ws_handler;
mod auth;
mod types;
mod settings;
mod library_manager;
mod watcher;
mod utils;

/// The main function
#[tokio::main]
async fn main() {
  dotenv().ok();
  
  pretty_env_logger::init_timed();

  let tx = Arc::new(Mutex::new(broadcast::channel(100).0));
  let settings = Arc::new(Mutex::new(get_default_settings()));

  let tx_ws = tx.clone();
  let settings_ws = settings.clone();

  let watcher_core = Watcher::new();
  watcher_core.init(tx.lock().unwrap().to_owned());

  let watcher_arc = Arc::new(Mutex::new(watcher_core));
  let watcher_ws = watcher_arc.clone();

  let ws_route = warp::path("api")
    .and(warp::ws())
    .map(move |ws: warp::ws::Ws| {
      let tx = tx_ws.clone();
      let settings = settings_ws.clone();
      let watcher = watcher_ws.clone();

      ws.on_upgrade(move |websocket| ws_handler::handle_connection(websocket, tx, settings, watcher))
    });

  warp::serve(ws_route)
    .run(([127, 0, 0, 1], 1500))
    .await;
}