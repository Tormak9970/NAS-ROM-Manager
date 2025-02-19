use std::env::var;
use serde::{Deserialize, Serialize};

use super::library::{Library, ROMCustomization};

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
    navigation: NavigationSettings {
      landingPage: "library".to_string(),
      landscapeViews: vec!["Dashboard".to_string(), "Library".to_string(), "Systems".to_string(), "Emulators".to_string(), "Settings".to_string()],
      portraitViews: vec!["Dashboard".to_string(), "Library".to_string(), "Search".to_string(), "System".to_string(), "Settings".to_string()],
    },
    metadata: MetadataSettings {
      saveAlongsideROMs: false,
    },
    accessibility: AccessibilitySettings {
      reducedMotion: false,
    },
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
pub struct ThemeSettings {
  pub primaryColor: String,
  pub palette: String,
  pub useOledPalette: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct NavigationSettings {
  pub landingPage: String,
  pub landscapeViews: Vec<String>,
  pub portraitViews: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct MetadataSettings {
  pub saveAlongsideROMs: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct AccessibilitySettings {
  pub reducedMotion: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
#[serde(default = "get_default_settings")]
pub struct Settings {
  pub FILE_SIG_DO_NOT_EDIT: String,
  pub version: String,
  pub theme: ThemeSettings,
  pub navigation: NavigationSettings,
  pub metadata: MetadataSettings,
  pub accessibility: AccessibilitySettings,
  pub library: Library,
  pub romCustomizations: Vec<ROMCustomization>,
}