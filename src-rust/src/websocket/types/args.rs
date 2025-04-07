use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{file_picker::FilePickerConfig, library::{Library, Parser, ROMMetadata}};

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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct MetadataArgs {
  pub passwordHash: String,
  pub data: HashMap<String, ROMMetadata>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct ParsersArgs {
  pub passwordHash: String,
  pub data: HashMap<String, Parser>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct DeleteParserArgs {
  pub passwordHash: String,
  pub abbreviation: String,
}