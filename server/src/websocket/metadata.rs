use std::{collections::HashMap, env::var, fs::{self, File}, path::PathBuf};

use crate::websocket::types::ErrorSender;

use super::types::library::ROMMetadata;

/// Checks if the app's metadata.json file exist and writes the default if they don't.
fn write_default_if_missing(config_path: &PathBuf, metadata_path: &PathBuf, default_metadata: &HashMap<String, ROMMetadata>, send_error: &ErrorSender) -> bool {
  let _ = fs::create_dir_all(config_path);

  let metadata_exists = fs::exists(metadata_path);
  if metadata_exists.is_err() {
    let err = metadata_exists.err().unwrap();

    send_error(
      format!("Failed to read metadata.json: {}", err.to_string()),
      "Please ensure NRM has write access to the mounted \"/config\" directory.".to_string(),
      crate::websocket::types::BackendErrorType::PANIC
    );

    return false;
  }

  if !metadata_exists.ok().unwrap() {
    let metadata_str = serde_json::to_string_pretty(default_metadata).expect("Default metadata was malformatted.");
    
    let write_res = fs::write(metadata_path, &metadata_str);

    if write_res.is_err() {
      let err = write_res.err().unwrap();

      send_error(
        format!("Failed to write default metadata: {}", err.to_string()),
        "Please ensure NRM has write access to the mounted \"/config\" directory.".to_string(),
        crate::websocket::types::BackendErrorType::PANIC
      );
    }

    return false;
  }

  return true;
}

/// Loads the app's metadata from the file system.
pub fn load_metadata(send_error: ErrorSender) -> Result<HashMap<String, ROMMetadata>, ()> {
  let default_metadata = HashMap::new();
  
  let config_path = PathBuf::from(var("NRM_CONFIG_DIR").ok().unwrap());
  let metadata_path = config_path.join("metadata.json");

  let metadata_exist = write_default_if_missing(&config_path, &metadata_path, &default_metadata, &send_error);
  if !metadata_exist {
    return Ok(default_metadata);
  }

  let metadata_file_res = File::open(&metadata_path);
  if metadata_file_res.is_err() {
    let err = metadata_file_res.err().unwrap();
    
    send_error(
      format!("Failed to read metadata.json: {}", err.to_string()),
      "Please ensure NRM has write access to the mounted \"/config\" directory.".to_string(),
      crate::websocket::types::BackendErrorType::PANIC
    );
    return Err(());
  }

  let saved_metadata_res = serde_json::from_reader(metadata_file_res.ok().unwrap());
  if saved_metadata_res.is_err() {
    let err = saved_metadata_res.err().unwrap();
    
    send_error(
      format!("Failed to parse metadata.json: {}", err.to_string()),
      "Please ensure your metadata.json follows the proper JSON format listed in the docs.".to_string(),
      crate::websocket::types::BackendErrorType::PANIC
    );
    return Err(());
  }

  let saved_metadata = saved_metadata_res.unwrap();

  // check_metadata(&mut saved_metadata, &default_metadata);

  return Ok(saved_metadata);
}

/// Writes all the provided metadata to the file system.
pub fn write_metadata(metadata: &HashMap<String, ROMMetadata>, send_error: ErrorSender) -> bool {
  let default_metadata = HashMap::new();
  
  let config_path = PathBuf::from(var("NRM_CONFIG_DIR").ok().unwrap());
  let metadata_path = config_path.join("metadata.json");

  let _ = write_default_if_missing(&config_path, &metadata_path, &default_metadata, &send_error);

  let metadata_str = serde_json::to_string_pretty(&metadata).expect("metadata were malformatted.");
    
  let write_res = fs::write(metadata_path, &metadata_str);
  if write_res.is_err() {
    let err = write_res.err().unwrap();
    
    send_error(
      format!("Failed to write metadata: {}", err.to_string()),
      "Please ensure NRM has write access to the mounted \"/config\" directory.".to_string(),
      crate::websocket::types::BackendErrorType::PANIC
    );

    return false;
  }

  return true;
}

