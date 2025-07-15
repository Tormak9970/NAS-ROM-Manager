use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct LoadResult {
  pub library: Library,
  pub roms: Vec<ROM>,
  pub systems: Vec<Parser>,
  pub dlcs: HashMap<String, Vec<String>>,
  pub updates: HashMap<String, Vec<String>>,
}

#[derive(Clone, Debug)]
pub struct StateStore {
  pub library: Library,
  pub roms: Vec<ROM>,
  pub parsers: HashMap<String, Parser>,
  pub metadata: HashMap<String, ROMMetadata>,
  pub updates: HashMap<String, Vec<String>>,
  pub dlcs: HashMap<String, Vec<String>>,
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
pub struct Parser {
  pub name: String,
  pub abbreviation: String,
  pub igdbPlatformId: String,
  pub folder: String,
  pub sgdbId: String,
  pub fullCapsulePath: String,
  pub thumbCapsulePath: String,
  pub heroPath: String,
  pub tagConfig: SystemTagConfig,
  pub patterns: Vec<ParserPattern>,
  pub biosFiles: Vec<String>,
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
  pub fullCapsulePath: String,
  pub thumbCapsulePath: String,
  pub heroPath: String,
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
  pub dlcDir: String,
  pub updateDir: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct Collection {
  pub name: String,
  pub romIds: Vec<String>,
}