use futures_util::{SinkExt, StreamExt};
use warp::filters::ws::{Message, WebSocket};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;


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
          tx.lock().unwrap().send(text.to_string()).expect("Failed to broadcast message");
          
          println!("message: {}", text);
        }
      },
      Err(_e) => break,
    }
  }
}