use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use tokio::sync::RwLock;

use std::collections::HashMap;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct CoverUpload {
  pub url: String,
  pub ext: String,
  pub timeout: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct ROMDownload {
  pub title: String,
  pub path: String,
  pub downloadStrategy: Map<String, Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct ROMDelete {
  pub path: String,
}


type Streams = HashMap<String, StreamProgress>;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct StreamProgress {
  pub currentSize: u64,
  pub totalSize: u64,
}

#[derive(Clone)]
pub struct StreamStore {
  pub streams: Arc<RwLock<Streams>>
}

impl StreamStore {
  pub fn new() -> Self {
    StreamStore {
      streams: Arc::new(RwLock::new(HashMap::new())),
    }
  }
}
