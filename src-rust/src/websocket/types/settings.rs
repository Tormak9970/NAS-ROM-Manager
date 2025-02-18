use std::env::var;
use serde::{Deserialize, Serialize};

use super::library::{Library, ROMCustomization};

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
    FILE_SIG_DO_NOT_EDIT: "dev.travislane.nas-rom-manager".to_string(),
    version,
    theme: ThemeSettings {
      primaryColor: "#a74bf2".to_string(),
      palette: "Auto".to_string(),
      useOledPalette: false,
    },
    landingPage: "library".to_string(),
    library: Library {
      libraryPath: "".to_string(),
      romDir: "roms".to_string(),
      emulatorDir: "emulators".to_string(),
      biosDir: "bios".to_string(),
    },
    romCustomizations: Vec::new(),
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
  pub library: Library,
  pub romCustomizations: Vec<ROMCustomization>,
}