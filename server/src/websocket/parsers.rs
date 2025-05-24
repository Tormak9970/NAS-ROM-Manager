use std::{collections::HashMap, env::var, fs::{self, read_dir, File}, path::PathBuf};
use log::{info, warn};
use regex::RegexBuilder;
use wax::Glob;

use super::{types::{
  library::{Library, Parser},
  ErrorSender
}, watcher::Watcher};

/// Validates the structure of a parser.
fn validate_parser(parser: &Parser) -> bool {
  info!("Validate Parser: Validating {}...", &parser.abbreviation);

  // ! Validate required fields
  let empty = "".to_string();

  if parser.name == empty {
    warn!("Validate Parser: {}'s name was empty", &parser.abbreviation);
    return false;
  }
  
  if parser.abbreviation == empty {
    warn!("Validate Parser: {}'s abbreviation was empty", &parser.name);
    return false;
  }
  
  if parser.folder == empty {
    warn!("Validate Parser: {}'s folder was empty", &parser.abbreviation);
    return false;
  }
  

  // ! Validate tagConfig
  if parser.tagConfig.backgroundColor == empty {
    warn!("Validate Parser: {}'s tagConfig.backgroundColor was empty", &parser.abbreviation);
    return false;
  }
  
  let color_parts: Vec<&str> = parser.tagConfig.backgroundColor.split(" ").collect();
  if color_parts.len() != 3 {
    warn!("Validate Parser: {}'s tagConfig.backgroundColor must be formatted as follows: \"R G B\"", &parser.abbreviation);
    return false;
  }

  for part in color_parts {
    let as_u64 = part.parse::<u64>();

    if as_u64.is_err() {
      warn!("Validate Parser: {}'s tagConfig.backgroundColor can only contain numbers", &parser.abbreviation);
      return false;
    }
  }

  if parser.patterns.len() == 0 {
    warn!("Validate Parser: {}'s must have at least one pattern", &parser.abbreviation);
    return false;
  }

  // ! Validate patterns
  for pattern in parser.patterns.clone() {
    let glob_res = Glob::new(&pattern.glob);

    if glob_res.is_err() {
      let err = glob_res.err().unwrap();
      warn!("Validate Parser: {}'s glob pattern \"{}\" is invalid: {}", &parser.abbreviation, &pattern.glob, err);

      return false;
    }

    let mut regex_builder = RegexBuilder::new(&pattern.regex);
    regex_builder.case_insensitive(true);

    let regex_res = regex_builder.build();
    if regex_res.is_err() {
      let err = regex_res.err().unwrap();
      warn!("Validate Parser: {}'s regex \"{}\" is invalid: {}", &parser.abbreviation, &pattern.regex, err);

      return false;
    }
  }
  
  info!("Validate Parser: Successfully validated {}", &parser.abbreviation);

  return true;
}

fn load_bios_files(library: &Library, parser: &Parser) -> Vec<String> {
  let mut bios_files: Vec<String> = vec![];

  let bios_folder = PathBuf::from(&library.libraryPath).join(&library.biosDir);
  let bios_folder_exists = fs::exists(&bios_folder);
  if bios_folder_exists.is_err() || !(bios_folder_exists.unwrap()) {
    return bios_files;
  }

  let parser_bios_folder = bios_folder.join(&parser.folder);
  let parser_bios_folder_exists = fs::exists(&parser_bios_folder);
  if parser_bios_folder_exists.is_err() || !(parser_bios_folder_exists.unwrap()) {
    return bios_files;
  }
  
  let entries_res = read_dir(&parser_bios_folder).expect("Parser Bios Folder should have existed.");

  for bios_entry_res in entries_res {
    if bios_entry_res.is_err() {
      continue;
    }
    let bios_entry = bios_entry_res.unwrap();
    let bios_filename_os = bios_entry.file_name();
    let bios_filename = bios_filename_os.to_str().unwrap();
  
    let bios_metadata_res = bios_entry.metadata();
    if bios_metadata_res.is_err() {
      let err = bios_metadata_res.err().unwrap();
      warn!("Load Bios Files: Can't read metadata of bios file for parser \"{}\": {}", parser.name.clone(), err.to_string());
      continue;
    }
    let bios_metadata = bios_metadata_res.unwrap();

    if bios_metadata.is_file() {
      bios_files.push(bios_filename.to_string());
    }
  }


  return bios_files;
}

/// Loads a library's parsers.
pub fn load_parsers(library: &Library, send_error: &ErrorSender) -> Result<HashMap<String, Parser>, ()> {
  let mut parsers: HashMap<String, Parser> = HashMap::new();

  let parsers_path = var("NRM_PARSERS_DIR").expect("Load Parsers: Failed to get parsers directory env variable");

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
      warn!("Load Parsers: Can't read metadata of parser \"{}\": {}", parser_filename, err.to_string());
      continue;
    }
    let parser_metadata = parser_metadata_res.unwrap();

    if parser_metadata.is_file() {
      let parser_file_res = File::open(&parser_entry.path());
      if parser_file_res.is_err() {
        let err = parser_file_res.err().unwrap();
        warn!("Load Parsers: Can't read parser \"{}\": {}", parser_filename, err.to_string());
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
      
      let mut parser: Parser = parser_res.unwrap();
      parser.biosFiles = load_bios_files(library, &parser);

      if validate_parser(&parser) {
        parsers.insert(parser.abbreviation.clone(), parser);
      } else {
        warn!("Load Parsers: {} failed validation and won't be loaded.", parser.abbreviation);
      }
    }
  }

  return Ok(parsers);
}

/// Writes the parsers to the file system.
pub fn write_parsers(parsers: &HashMap<String, Parser>, library: &Library, watcher: &Watcher, send_error: ErrorSender) -> bool {
  let parsers_path = var("NRM_PARSERS_DIR").expect("Write Parsers: Failed to get parsers directory env variable");

  for parser in parsers.clone().into_values() {
    let file_path = PathBuf::from(&parsers_path).join(format!("{}.json", parser.folder));

    let parser_str = serde_json::to_string_pretty(&parser)
      .expect(format!("Parser {} was malformatted.", parser.abbreviation).as_str());

    let should_watch_path = !fs::exists(&file_path).unwrap_or(false);

    let write_res = fs::write(file_path, &parser_str);
    if write_res.is_err() {
      let err = write_res.err().unwrap();
      
      send_error(
        format!("Failed to write parser {}: {}", parser.abbreviation, err.to_string()),
        "Please ensure NRM has write access to the parsers directory.".to_string(),
        crate::websocket::types::BackendErrorType::PANIC
      );
  
      return false;
    }

    if should_watch_path {
      let system_rom_folder = PathBuf::from(library.romDir.clone()).join(parser.folder);

      let create_res = fs::create_dir_all(&system_rom_folder);
      if create_res.is_err() {
        let err = write_res.err().unwrap();
      
        send_error(
          format!("Failed to create folder for parser {}: {}", parser.abbreviation, err.to_string()),
          "Please ensure NRM has write access to the library directory.".to_string(),
          crate::websocket::types::BackendErrorType::PANIC
        );

        continue;
      }

      watcher.watch_path(system_rom_folder);
    }
  }

  return true;
}

/// Deletes the specified parser.
pub fn delete_parser(parser: &Parser, library: &Library, watcher: &Watcher, send_error: ErrorSender) -> bool {
  let parsers_path = var("NRM_PARSERS_DIR").expect("Delete Parser: Failed to get parsers directory env variable");

  let file_path = PathBuf::from(&parsers_path).join(format!("{}.json", &parser.folder));

  let delete_res = fs::remove_file(file_path);
  if delete_res.is_err() {
    let err = delete_res.err().unwrap();
    
    send_error(
      format!("Failed to delete parser {}: {}", parser.abbreviation, err.to_string()),
      "Please ensure NRM has write access to the parsers directory.".to_string(),
      crate::websocket::types::BackendErrorType::PANIC
    );

    return false;
  }

  info!("Delete Parser: Successfully deleted parser {}.", parser.abbreviation);
  
  let system_rom_folder = PathBuf::from(library.romDir.clone()).join(parser.folder.clone());
  watcher.unwatch_path(system_rom_folder);
  
  info!("Delete Parser: Successfully unwatched system folder {}.", parser.folder);

  return true;
}