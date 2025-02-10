use std::env::var;
use serde::{Deserialize, Serialize};

use super::library::Library;

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