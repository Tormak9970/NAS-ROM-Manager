use std::{collections::HashMap, path::{self, Path}, sync::{mpsc::{Receiver, Sender}, Arc, Mutex}};

use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher as _};
use tokio::sync::broadcast;

use crate::utils::send;

pub enum WatcherEvent {
  Add(String, String),
  Remove(String, String),
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

    // path -> library_name
    let mut map: HashMap<String, String> = HashMap::new();
    let library_path_map = Arc::new(Mutex::new(map));

    let (sender, receiver) = std::sync::mpsc::channel();

    let reciever_mutex = Arc::new(Mutex::new(receiver));
    
    // Create a thread for handling the folder watching.
    std::thread::spawn(move || {
      println!("starting watcher loop...");

      let mut folders_watching: Vec<String> = vec![];

      // Select recommended watcher for debouncer.
      // Using a callback here, could also be a channel.
      let mut watcher = RecommendedWatcher::new(sender, Config::default()).unwrap();

      // * Listen for watcher events from the frontend.
      loop {
        let event = event_receiver.try_lock().unwrap().recv();

        if let Ok(result) = event {
          match result {
            WatcherEvent::Add(path, library_name) => {
              // * Remove watchers from any paths that were removed.
              // let folders_watching_loop = folders_watching.clone();
              // for current_folder in folders_watching_loop {
              //   if !folders.contains(&current_folder) {
              //     let _ = watcher.unwatch(Path::new(&current_folder));
                  
              //     let index = (&folders_watching).iter().position(|f| *f == current_folder).unwrap();
              //     folders_watching.remove(index);
              //   }
              // }

              // // * Add watchers to any paths that were added.
              // for folder in folders {
              //   if !folders_watching.contains(&folder) {
              //     let _ = watcher.watch(Path::new(&folder), RecursiveMode::Recursive);

              //     folders_watching.push(folder);
              //   }
              // }
            },
            WatcherEvent::Remove(path, library_name) => {

            }
          }
        }
      }
    });

    // Create a thread for handling the watcher events
    std::thread::spawn(move || {
      println!("starting watcher event thread...");

      loop {
        let event = reciever_mutex.try_lock().unwrap().recv();

        if let Ok(res) = event {
          match res {
            Ok(event) => match event.kind {
                EventKind::Create(path) => {
                  let paths = event.paths;

                  // send(tx.clone(), "file_added", data);
                },
                EventKind::Remove(path) => {
                  let paths = event.paths;
                  
                  // send(tx.clone(), "file_removed", data);
                },
                _ => {
                  
                }
            },
            Err(e) => {
              println!("Error {:?}", e)
            },
          }
        }
      }
    });
  }

  /// Watches a path.
  pub fn watch_path(&self, path: String, library_name: String) {
    let _ = self.sender.send(WatcherEvent::Add(path, library_name));
  }

  /// Unwatches a path.
  pub fn unwatch_path(&self, path: String, library_name: String) {
    let _ = self.sender.send(WatcherEvent::Remove(path, library_name));
  }
}