use warp::Filter;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use dotenv::dotenv;

use jwt_simple::prelude::*;

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
  let jwt_key = Arc::new(Mutex::new(HS256Key::generate()));

  let ws_route = warp::path("ws")
    .and(warp::ws())
    .map(move |ws: warp::ws::Ws| {
      let tx = tx_ws.clone();
      let jwt = jwt_key.clone();

      ws.on_upgrade(move |websocket| ws_handler::handle_connection(websocket, tx, jwt))
    });

  warp::serve(ws_route)
    .run(([127, 0, 0, 1], 1500))
    .await;
}