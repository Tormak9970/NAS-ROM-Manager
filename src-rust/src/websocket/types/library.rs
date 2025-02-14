use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

type ParserMap = HashMap<String, Parser>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ParserStore {
  pub libraries: HashMap<String, ParserMap>,
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
  pub tagConfig: SystemTagConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct Parser {
  pub name: String,
  pub abbreviation: String,
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
  pub library: String,
  pub downloadStrategy: Map<String, Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct ROMCustomization {
  pub path: String,
  pub title: String,
  pub gridPath: String,
  pub isFavorite: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct Library {
  pub name: String,
  pub path: String,
  pub useProvidedParsers: bool,
  pub parsersPath: String,
  pub romCustomizations: Vec<ROMCustomization>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct Collection {
  pub name: String,
  pub romIds: Vec<String>,
}