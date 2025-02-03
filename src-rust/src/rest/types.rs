use serde::{Deserialize, Serialize};
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
  pub path: String,
  pub parent: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct ROMUploadComplete {
  pub uploadId: String,
  pub path: String,
  pub libraryPath: String,
  pub system: String,
  pub unzip: bool,
}


type Streams = HashMap<String, StreamProgress>;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct StreamProgress {
  pub id: String,
  pub path: String,
  pub currentSize: u64,
  pub totalSize: u64,
  pub lastChunkTime: i64,
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
