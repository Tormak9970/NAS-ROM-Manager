use futures_util::{SinkExt, StreamExt};
use log::warn;
use warp::filters::ws::{Message, WebSocket};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

use jwt_simple::prelude::*;

use crate::auth::authenticate_user;

fn handle_message(message: &str, args: Vec<&str>, tx: broadcast::Sender<String>, jwt_key: Arc<Mutex<HS256Key>>) {
  let key = jwt_key.lock().unwrap();
  match message {
    "user_auth" => {
      let username = args[1].to_owned();
      let password_hash = args[2].to_owned();

      let result = authenticate_user(username.clone(), password_hash);

      let mut token = String::from("AUTH_FAILED");

      if result {
        let claims = Claims::create(Duration::from_hours(48));
        token = key.authenticate(claims).unwrap();
      }

      tx.send(format!("user_auth {} {} {}", username, result.to_string(), token)).expect("Failed to broadcast message");
    }
    "demo" => {
      let token = args[1].to_owned();
      let claims_res = key.verify_token::<NoCustomClaims>(&token, None);

      if claims_res.is_err() {
        warn!("JWT Token has expired!");
        tx.send(String::from("token_expired")).expect("Failed to broadcast message");
        return;
      }

      // TODO: run the logic here
    }
    _ => {}
  }
}

/// Handles WebSocket Connections
pub async fn handle_connection(ws: WebSocket, tx: Arc<Mutex<broadcast::Sender<String>>>, jwt_key: Arc<Mutex<HS256Key>>) {
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

          handle_message(contents[0], contents, tx.lock().unwrap().to_owned(), jwt_key.clone());
        }
      },
      Err(_e) => break,
    }
  }
}