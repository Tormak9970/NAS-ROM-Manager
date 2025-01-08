use std::{env::var, fs::{self, File}, path::PathBuf, sync::MutexGuard};
use log::warn;
use serde_json::Value;

use crate::types::Settings;

pub fn get_default_settings() -> Settings {
  let version = var("ROM_MANAGER_VERSION").ok().unwrap();

  return Settings {
    FILE_SIG_DO_NOT_EDIT: String::from("dev.travislane.nas-rom-manager"),
    version,
    libraries: vec![],
    collections: vec![],
    romOverrides: vec![],
  };
}

fn check_settings(settings: &mut Settings, defaults: &Settings) {
  settings.version = defaults.version.clone();
}

/// Loads the app's settings from the file system.
pub fn load_settings() -> Settings {
  let default_settings = get_default_settings();

  let config_path = PathBuf::from(var("ROM_MANAGER_CONFIG_DIR").ok().unwrap());
  let settings_path = config_path.join("settings.json");

  let _ = fs::create_dir_all(config_path);

  let settings_exists = fs::exists(&settings_path);
  if settings_exists.is_err() {
    warn!("Can't check existence of settings.json (Check permissions)");
    return default_settings;
  }

  if !settings_exists.ok().unwrap() {
    let settings_str = serde_json::to_string_pretty(&default_settings).expect("Settings were malformatted.");
    
    let write_res = fs::write(settings_path, &settings_str);

    if write_res.is_err() {
      warn!("Failed to write default settings (Check permissions)");
    }

    return default_settings;
  }

  let settings_file_res = File::open(&settings_path);
  if settings_file_res.is_err() {
    warn!("Can't read settings.json (Check permissions)");
    return default_settings;
  }

  let mut saved_settings: Settings = serde_json::from_reader(settings_file_res.ok().unwrap()).unwrap();

  check_settings(&mut saved_settings, &default_settings);

  return saved_settings;
}

pub fn write_settings(state_settings: MutexGuard<'_, Settings>) {

}

pub fn set_setting(state_settings: MutexGuard<'_, Settings>, key: &str, value: Value) {

}