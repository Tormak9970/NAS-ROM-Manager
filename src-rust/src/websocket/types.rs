use std::{collections::HashMap, env::var};

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};


type ParserMap = HashMap<String, Parser>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ParserStore {
  pub libraries: HashMap<String, ParserMap>,
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
  pub folder: String,
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
  pub heroPath: String,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct ThemeSettings {
  pub primaryColor: String,
  pub palette: String,
  pub useOledPalette: bool,
}

/// Gets the default values of the settings object.
pub fn get_default_settings() -> Settings {
  let version = var("NRM_VERSION").ok().unwrap();

  return Settings {
    FILE_SIG_DO_NOT_EDIT: String::from("dev.travislane.nas-rom-manager"),
    version,
    theme: ThemeSettings {
      primaryColor: String::from("#a74bf2"),
      palette: String::from("Auto"),
      useOledPalette: false,
    },
    landingPage: String::from("library"),
    libraries: Vec::new(),
  };
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
#[serde(default = "get_default_settings")]
pub struct Settings {
  pub FILE_SIG_DO_NOT_EDIT: String,
  pub version: String,
  pub theme: ThemeSettings,
  pub landingPage: String,
  pub libraries: Vec<Library>,
}


// * Interop types
#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct SimpleArgs {
  pub passwordHash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct AuthArgs {
  pub passwordHash: String,
  pub user: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct SetSettingArgs {
  pub passwordHash: String,
  pub key: String,
  pub value: Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct ModifyLibraryArgs {
  pub passwordHash: String,
  pub library: Library,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct ParseRomArgs {
  pub passwordHash: String,
  pub libraryName: String,
  pub system: String,
  pub romPath: String,
}