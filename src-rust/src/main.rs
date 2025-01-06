use warp::Filter;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use dotenv::dotenv;

mod ws_handler;
mod manager;
mod auth;
mod types;

/// The main function
#[tokio::main]
async fn main() {
  dotenv().ok();
  
  pretty_env_logger::init_timed();

  let tx = Arc::new(Mutex::new(broadcast::channel(100).0));
  let tx_ws = tx.clone();

  let ws_route = warp::path("ws")
    .and(warp::ws())
    .map(move |ws: warp::ws::Ws| {
      let tx = tx_ws.clone();

      ws.on_upgrade(move |websocket| ws_handler::handle_connection(websocket, tx))
    });

  warp::serve(ws_route)
    .run(([127, 0, 0, 1], 1500))
    .await;
}