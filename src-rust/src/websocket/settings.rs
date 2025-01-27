use std::{env::var, fs::{self, File}, path::PathBuf, sync::MutexGuard};
use log::warn;
use serde_json::{Map, Value};

use crate::websocket::types::{get_default_settings, Settings};

fn check_settings(settings: &mut Settings, defaults: &Settings) {
  settings.version = defaults.version.clone();

  // ! any changes to settings structure need to be checked and addressed here.
  // ! remember that missing values will be the defaults, so check for existence of old field vs lack of new field.
}

/// Checks if the app's settings exist and writes the defaults if they don't.
fn write_default_if_missing(config_path: &PathBuf, settings_path: &PathBuf, default_settings: &Settings) -> bool {
  let _ = fs::create_dir_all(config_path);

  let settings_exists = fs::exists(settings_path);
  if settings_exists.is_err() {
    let err = settings_exists.err().unwrap();
    warn!("Can't check existence of settings.json: {}", err.to_string());
    return false;
  }

  if !settings_exists.ok().unwrap() {
    let settings_str = serde_json::to_string_pretty(default_settings).expect("Settings were malformatted.");
    
    let write_res = fs::write(settings_path, &settings_str);

    if write_res.is_err() {
      let err = write_res.err().unwrap();
      warn!("Failed to write default settings: {}", err.to_string());
    }

    return false;
  }

  return true;
}

/// Loads the app's settings from the file system.
pub fn load_settings() -> Settings {
  let default_settings = get_default_settings();
  
  let config_path = PathBuf::from(var("NRM_CONFIG_DIR").ok().unwrap());
  let settings_path = config_path.join("settings.json");

  let settings_exist = write_default_if_missing(&config_path, &settings_path, &default_settings);
  if !settings_exist {
    return default_settings;
  }

  let settings_file_res = File::open(&settings_path);
  if settings_file_res.is_err() {
    let err = settings_file_res.err().unwrap();
    warn!("Can't read settings.json: {}", err.to_string());
    return default_settings;
  }

  let mut saved_settings: Settings = serde_json::from_reader(settings_file_res.ok().unwrap()).unwrap();

  check_settings(&mut saved_settings, &default_settings);

  return saved_settings;
}

/// Writes all settings in state to the file system.
pub fn write_settings(state_settings: MutexGuard<'_, Settings>) -> bool {
  let default_settings = get_default_settings();
  
  let config_path = PathBuf::from(var("NRM_CONFIG_DIR").ok().unwrap());
  let settings_path = config_path.join("settings.json");

  let _ = write_default_if_missing(&config_path, &settings_path, &default_settings);

  let settings = state_settings.clone();
  let settings_str = serde_json::to_string_pretty(&settings).expect("Settings were malformatted.");
    
  let write_res = fs::write(settings_path, &settings_str);
  if write_res.is_err() {
    let err = write_res.err().unwrap();
    warn!("Failed to write default settings: {}", err.to_string());
    return false;
  }

  return true;
}

fn set_property_recursive(object: &mut Map<String, Value>, properties: &[&str], index: usize, value: &Value) -> Value {
  let mut new_value = value.to_owned();

  if index != properties.len() - 1 {
    let child = object.get_mut(properties[index]).unwrap().as_object_mut().unwrap();
    new_value = set_property_recursive(child, properties, index + 1, value);
  }

  object.insert(properties[index].to_string(), new_value);
  
  return Value::Object(object.to_owned());
}

/// Sets the provided setting.
pub fn set_setting(state_settings: &mut MutexGuard<'_, Settings>, key: &str, value: Value) {
  let mut map_value = serde_json::to_value(state_settings.clone()).unwrap();
  let map = map_value.as_object_mut().unwrap();

  let keys: Vec<&str> = key.split(".").collect();
  
  let updated_value = set_property_recursive(map, &keys, 0, &value);
  let updated_settings: Settings = serde_json::from_value(updated_value).unwrap();

  **state_settings = updated_settings.clone();
}