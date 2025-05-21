pub mod sgdb;
pub mod igdb;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use std::collections::HashMap;
use std::sync::Arc;

use super::igdb::igdb_client::IGDBClient;
use super::sgdb::SGDBClient;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct HeroUpload {
  pub heroUrl: String,
  pub heroExt: String,
  pub timeout: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct CapsuleUpload {
  pub fullCapsuleUrl: String,
  pub fullCapsuleExt: String,
  pub thumbCapsuleUrl: String,
  pub thumbCapsuleExt: String,
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

#[derive(Clone)]
pub struct SGDBClientStore {
  pub client: Arc<RwLock<SGDBClient>>
}

impl SGDBClientStore {
  pub fn new() -> Self {
    SGDBClientStore {
      client: Arc::new(RwLock::new(SGDBClient::new("".to_string(), 5000))),
    }
  }
}

#[derive(Clone)]
pub struct IGDBClientStore {
  pub client: Arc<RwLock<IGDBClient>>
}

impl IGDBClientStore {
  pub fn new() -> Self {
    IGDBClientStore {
      client: Arc::new(RwLock::new(IGDBClient::new(5000))),
    }
  }
}