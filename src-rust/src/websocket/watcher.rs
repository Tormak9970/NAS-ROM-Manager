use std::{path::PathBuf, sync::{mpsc::{Receiver, Sender}, Arc, Mutex}};

use log::{info, warn};
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher as _};
use tokio::sync::broadcast;

use crate::websocket::utils::send;

pub enum WatcherEvent {
  Add(PathBuf),
  Remove(PathBuf),
}

#[derive(Clone)]
pub struct Watcher {
  receiver: Arc<Mutex<Receiver<WatcherEvent>>>,
  sender: Sender<WatcherEvent>
}

impl Watcher {
  /// Creates a new Watcher.
  pub fn new() -> Watcher {
    let (sender, receiver) = std::sync::mpsc::channel();
    
    return Watcher {
      receiver: Arc::new(Mutex::new(receiver)),
      sender
    };
  }

  /// Initializes the watcher thread.
  pub fn init(&self, tx: broadcast::Sender<String>) {
    let event_receiver = self.receiver.clone();

    let (sender, receiver) = std::sync::mpsc::channel();

    let reciever_mutex = Arc::new(Mutex::new(receiver));
    
    // Create a thread for handling the folder watching.
    std::thread::spawn(move || {
      info!("Thread: Starting watcher file listener...");

      // Select recommended watcher for debouncer.
      // Using a callback here, could also be a channel.
      let mut watcher = RecommendedWatcher::new(sender, Config::default()).unwrap();

      // * Listen for watcher events from the frontend.
      loop {
        let event = event_receiver.try_lock().unwrap().recv();

        if let Ok(result) = event {
          match result {
            WatcherEvent::Add(path) => {
              // * Add watcher to the path.
              let _ = watcher.watch(path.as_path(), RecursiveMode::Recursive);
            },
            WatcherEvent::Remove(path) => {
              // * Remove path from watcher.
              let _ = watcher.unwatch(path.as_path());
            }
          }
        }
      }
    });

    // Create a thread for handling the watcher events
    std::thread::spawn(move || {
      info!("Thread: Starting watcher event handler...");

      loop {
        let event = reciever_mutex.try_lock().unwrap().recv();

        if let Ok(res) = event {
          match res {
            Ok(event) => match event.kind {
                EventKind::Create(_create_kind) => {
                  let path = event.paths[0].clone();

                  send(tx.clone(), "reload_library", path.to_str().to_owned());
                },
                EventKind::Remove(_create_kind) => {
                  let path = event.paths[0].clone();
                  
                  send(tx.clone(), "reload_library", path.to_str().to_owned());
                },
                _ => {
                  
                }
            },
            Err(e) => {
              warn!("Error {:?}", e)
            },
          }
        }
      }
    });
  }

  /// Watches a path.
  pub fn watch_path(&self, path: PathBuf) {
    let _ = self.sender.send(WatcherEvent::Add(path));
  }

  /// Unwatches a path.
  pub fn unwatch_path(&self, path: PathBuf) {
    let _ = self.sender.send(WatcherEvent::Remove(path));
  }
}