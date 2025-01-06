use futures_util::{SinkExt, StreamExt};
use log::{info, warn};
use warp::filters::ws::{Message, WebSocket};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

use crate::auth::{authenticate_user, validate_hash};

fn handle_message(message: &str, args: Vec<&str>, tx: broadcast::Sender<String>) {
  match message {
    "user_auth" => {
      let username = args[1].to_owned();
      let password_hash = args[2].to_owned();

      let result = authenticate_user(username.clone(), password_hash, tx.clone());

      tx.send(format!("user_auth {} {}", username, result.to_string())).expect("Failed to broadcast message");
    }
    "demo" => {
      let hash = args[1].to_owned();
      let is_valid = validate_hash(hash, tx.clone());

      if !is_valid {
        warn!("Password hashes do not match!");
        tx.send(String::from("hash_mismatch")).expect("Failed to broadcast message");
        return;
      }

      // TODO: run the logic here
    }
    _ => {}
  }
}

/// Handles WebSocket Connections
pub async fn handle_connection(ws: WebSocket, tx: Arc<Mutex<broadcast::Sender<String>>>) {
  let (mut ws_sender, mut ws_receiver) = ws.split();
  let mut rx = tx.lock().unwrap().subscribe();

  // ! Spawn the Message Propegation Thread.
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
        print!("recieved message!");
        
        if let Ok(text) = message.to_str() {
          let contents = text.split(" ").collect::<Vec<&str>>();

          handle_message(contents[0], contents, tx.lock().unwrap().to_owned());
        }
      },
      Err(_e) => break,
    }
  }
}