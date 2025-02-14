use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{file_picker::FilePickerConfig, library::Library};

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
  pub parser: String,
  pub romPath: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct FilePickerArgs {
  pub passwordHash: String,
  pub path: String,
  pub config: FilePickerConfig,
}