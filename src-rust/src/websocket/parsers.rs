use std::{collections::HashMap, env::var, fs::{read_dir, File}};
use log::warn;

use super::types::{
  library::Parser,
  ErrorSender
};

/// Loads a library's parsers.
pub fn load_parsers(send_error: &ErrorSender) -> Result<HashMap<String, Parser>, ()> {
  let mut parsers: HashMap<String, Parser> = HashMap::new();

  let parsers_path = var("NRM_PARSERS_DIR").expect("Failed to get builtin default parsers env variable");

  let entries_res = read_dir(&parsers_path);
  if entries_res.is_err() {
    let err = entries_res.err().unwrap();
    
    send_error(
      format!("Failed to load parsers: {}", err.to_string()),
      format!("Please double check that \"{}\" is readable.", parsers_path),
      crate::websocket::types::BackendErrorType::PANIC
    );
    
    return Err(());
  }

  for parser_entry_res in entries_res.unwrap() {
    if parser_entry_res.is_err() {
      continue;
    }
    let parser_entry = parser_entry_res.unwrap();
    let parser_filename_os = parser_entry.file_name();
    let parser_filename = parser_filename_os.to_str().unwrap();

    let parser_metadata_res = parser_entry.metadata();
    if parser_metadata_res.is_err() {
      let err = parser_metadata_res.err().unwrap();
      warn!("Library: Can't read metadata of parser \"{}\": {}", parser_filename, err.to_string());
      continue;
    }
    let parser_metadata = parser_metadata_res.unwrap();

    if parser_metadata.is_file() {
      let parser_file_res = File::open(&parser_entry.path());
      if parser_file_res.is_err() {
        let err = parser_file_res.err().unwrap();
        warn!("Library: Can't read parser \"{}\": {}", parser_filename, err.to_string());
        continue;
      }

      let parser_res = serde_json::from_reader(parser_file_res.ok().unwrap());
      if parser_res.is_err() {
        let err = parser_res.err().unwrap();
        
        send_error(
          format!("Failed to parse parser \"{}\": {}", parser_filename, err.to_string()),
          "Please ensure your settings.json follows the proper JSON format listed in the docs.".to_string(),
          crate::websocket::types::BackendErrorType::PANIC
        );
        return Err(());
      }
      
      let parser: Parser = parser_res.unwrap();

      parsers.insert(parser.abbreviation.clone(), parser);
    }
  }

  return Ok(parsers);
}

pub fn write_parsers(parsers: &HashMap<String, Parser>, send_error: ErrorSender) -> bool {
  let parsers_path = var("NRM_PARSERS_DIR").expect("Failed to get builtin default parsers env variable");

  // for

  return false;
}