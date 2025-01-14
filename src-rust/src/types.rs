use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
/// Use "i" when performing matches to ignore case sensitivity
pub struct ParserPattern {
  pub glob: String,
  pub regex: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
/// Use "i" when performing matches to ignore case sensitivity
pub struct Parser {
  pub name: String,
  pub abbreviation: String,
  pub folder: String,
  pub patterns: Vec<ParserPattern>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct ROM {
  pub name: String,
  pub path: String,
  pub size: usize,
  pub addDate: String,
  pub format: String,
  pub system: String,
  pub library: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct ROMCustomization {
  pub name: String,
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
  pub roms: Vec<ROM>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct ThemeSettings {
  pub primaryColor: String,
  pub palette: String,
  pub useOledPalette: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct Settings {
  pub FILE_SIG_DO_NOT_EDIT: String,
  pub version: String,
  pub theme: ThemeSettings,
  pub libraries: Vec<Library>,
  pub collections: Vec<Collection>,
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
  pub key: String,
  pub library: Library,
}