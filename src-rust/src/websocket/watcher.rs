use std::{collections::HashMap, path::PathBuf, sync::{mpsc::{Receiver, Sender}, Arc, Mutex}};

use log::{info, warn};
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher as _};
use tokio::sync::broadcast;

use crate::websocket::utils::send;

pub enum WatcherEvent {
  Add(PathBuf, String),
  RemoveLibrary(String),
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

      // path -> library_path
      let mut dir_path_map: HashMap<PathBuf, String> = HashMap::new();

      // library_path -> path[]
      let mut library_map: HashMap<String, Vec<PathBuf>> = HashMap::new();

      // Select recommended watcher for debouncer.
      // Using a callback here, could also be a channel.
      let mut watcher = RecommendedWatcher::new(sender, Config::default()).unwrap();

      // * Listen for watcher events from the frontend.
      loop {
        let event = event_receiver.try_lock().unwrap().recv();

        if let Ok(result) = event {
          match result {
            WatcherEvent::Add(path, library_path) => {
              // * Add watcher to the path.
              if !library_map.contains_key(library_path.as_str()) {
                library_map.insert(library_path.clone(), Vec::new());
              }

              let folders_watching = library_map.get_mut(&library_path).unwrap();
              if !folders_watching.contains(&path) {
                let _ = watcher.watch(path.as_path(), RecursiveMode::Recursive);

                folders_watching.push(path.clone());
              }

              dir_path_map.insert(path, library_path);
            },
            WatcherEvent::RemoveLibrary(library_path) => {
              // * Remove watcher from the library.
              let paths = library_map.get(&library_path).unwrap();

              for path in paths {
                let _ = watcher.unwatch(path.as_path());

                dir_path_map.remove(path);
              }

              library_map.remove(&library_path);
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
  pub fn watch_path(&self, path: PathBuf, library_path: String) {
    let _ = self.sender.send(WatcherEvent::Add(path, library_path));
  }

  /// Unwatches all paths associated with a given library.
  pub fn unwatch_library(&self, library_path: String) {
    let _ = self.sender.send(WatcherEvent::RemoveLibrary(library_path));
  }
}