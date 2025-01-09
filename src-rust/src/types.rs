use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct ROM {
  pub name: String,
  pub path: String,
  pub heroPath: String,
  pub gridPath: String,
  pub format: String,
  pub system: String,
  pub isFavorite: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct Library {
  pub name: String,
  pub path: String,
  pub roms: Vec<ROM>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct Collection {
  pub name: String,
  pub roms: Vec<ROM>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct Settings {
  pub FILE_SIG_DO_NOT_EDIT: String,
  pub version: String,
  pub libraries: Vec<Library>,
  pub collections: Vec<Collection>,
  pub romOverrides: Vec<ROM>,
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