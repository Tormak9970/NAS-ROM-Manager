use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Clone, Debug)]
pub struct StateStore {
  pub library: Library,
  pub parsers: HashMap<String, Parser>,
  pub metadata: HashMap<String, ROMMetadata>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct SystemTagConfig {
  pub backgroundColor: String,
  pub borderColor: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct ParserPattern {
  pub glob: String,
  pub regex: String,
  pub downloadStrategy: Map<String, Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct System {
  pub fullName: String,
  pub abbreviation: String,
  pub igdbPlatformId: String,
  pub tagConfig: SystemTagConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct Parser {
  pub name: String,
  pub abbreviation: String,
  pub igdbPlatformId: String,
  pub folder: String,
  pub tagConfig: SystemTagConfig,
  pub patterns: Vec<ParserPattern>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct ROM {
  pub title: String,
  pub path: String,
  pub size: u64,
  pub addDate: String,
  pub format: String,
  pub system: String,
  pub systemFullName: String,
  pub downloadStrategy: Map<String, Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct ROMMetadata {
  pub title: String,
  pub coverPath: String,
  pub thumbPath: String,
  pub sgdbId: String,
  pub igdbId: String,
  pub metadata: Option<Value>,
  pub isFavorite: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct Library {
  pub libraryPath: String,
  pub romDir: String,
  pub emulatorDir: String,
  pub biosDir: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct Collection {
  pub name: String,
  pub romIds: Vec<String>,
}