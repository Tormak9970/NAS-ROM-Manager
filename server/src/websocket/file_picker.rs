use std::{fs::read_dir, path::PathBuf};

use log::warn;

use super::types::{file_picker::{FilePickerConfig, FilePickerEntry}, ErrorSender};

/// Gets the file picker entries.
pub fn get_entries(path: String, config: FilePickerConfig, send_error: ErrorSender) -> Result<Vec<FilePickerEntry>, ()> {
  let root_path = PathBuf::from(&path);

  let entries_res = read_dir(&path);
  if entries_res.is_err() {
    let err = entries_res.err().unwrap();
    
    send_error(
      format!("File Picker: Failed to read entries of \"{}\": {}", path, err.to_string()),
      format!("Please double check that NRM has access to the folder mounted to \"{}\".", path),
      crate::websocket::types::BackendErrorType::WARN
    );

    return Err(());
  }

  let has_extensions_filter = config.extensions.len() > 0;
  let extensions = config.extensions;

  let mut entries: Vec<FilePickerEntry> = Vec::new();

  for entry_res in entries_res.unwrap() {
    if entry_res.is_err() {
      let err = entry_res.err().unwrap();
      warn!("File Picker: Failed to read entry: {}", err.to_string());
      continue;
    }
    let entry = entry_res.unwrap();

    let entry_name = entry.file_name().to_str().unwrap().to_owned();
    let entry_path = root_path.join(&entry_name);

    let metadata_res = entry.metadata();
    if metadata_res.is_err() {
      let err = metadata_res.err().unwrap();
      warn!("File Picker: Failed to read metadata of \"{}\": {}", entry_name, err.to_string());
      continue;
    }
    let metadata = metadata_res.unwrap();

    if metadata.is_dir() {
      entries.push(FilePickerEntry {
        path: entry_path.to_str().unwrap().to_string(),
        name: entry_name,
        isDir: true
      });
      continue;
    }

    if !config.showFiles {
      continue;
    }
    
    if !config.showHiddenFiles && entry_name.starts_with(".") {
      continue;
    }

    let file_ext_res = entry_path.extension();
    if file_ext_res.is_none() {
      continue;
    }

    let file_ext = file_ext_res.unwrap().to_str().unwrap().to_string();
    if has_extensions_filter && !extensions.contains(&file_ext) {
      continue;
    }

    entries.push(FilePickerEntry {
      path: entry_path.to_str().unwrap().to_string(),
      name: entry_name,
      isDir: false
    });
  }

  return Ok(entries);
}