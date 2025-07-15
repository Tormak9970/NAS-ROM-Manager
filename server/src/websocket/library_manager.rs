use std::{collections::HashMap, fs::{self, read_dir}, path::PathBuf};
use chrono::{DateTime, Local};
use log::warn;
use regex::RegexBuilder;
use wax::{Glob, Pattern};

use super::{parsers::load_parsers, types::{
  library::{
    Library, LoadResult, Parser, ParserPattern, StateStore, ROM
  },
  ErrorSender
}};
use super::watcher::Watcher;

fn load_rom(parser: &Parser, pattern: &ParserPattern, path: PathBuf) -> ROM {
  let path_str = path.to_str().unwrap().to_string();
  let metadata = fs::metadata(&path).expect("Failed to read ROM metadata");

  let create_date: DateTime<Local> = DateTime::<Local>::from(metadata.created().unwrap());
  let extension = path.extension().unwrap().to_str().unwrap();

  let mut title = path_str.clone();

  let mut regex_builder = RegexBuilder::new(&pattern.regex);
  regex_builder.case_insensitive(true);

  let regex = regex_builder.build().unwrap();

  let clean_title = title.replace("\\", "/");
  let captures = regex.captures(&clean_title);

  if captures.is_some() {
    let results = captures.unwrap();
    if results.len() > 0 {
      title = results.name("title").unwrap().as_str().to_owned();
    }
  }

  return ROM {
    title,
    path: path_str,
    size: metadata.len(),
    addDate: format!("{}", create_date.format("%b %e, %Y")),
    format: extension.to_owned(),
    system: parser.abbreviation.clone(),
    systemFullName: parser.name.clone(),
    downloadStrategy: pattern.downloadStrategy.clone(),
  };
}

fn load_platform(parser: &Parser, path: PathBuf) -> Vec<ROM> {
  let mut roms: Vec<ROM> = Vec::new();

  for pattern in &parser.patterns {
    let glob = Glob::new(&pattern.glob).unwrap();
    for entry in glob.walk(&path) {
      if entry.is_err() {
        let err = entry.err().unwrap();
        warn!("Library: Parser: \"{}\"; Failed to walk entry \"{}\"", &parser.abbreviation, err.path().unwrap().display());
        continue;
      }

      let path = entry.unwrap().into_path();

      roms.push(load_rom(
        parser,
        pattern,
        path
      ));
    }
  }

  return roms;
}

fn load_roms(library: &Library, watcher: &Watcher, parsers: &HashMap<String, Parser>, send_error: &ErrorSender) -> Result<Vec<ROM>, ()> {
  let mut roms: Vec<ROM> = vec![];

  let mut parsers_folder_map: HashMap<String, Parser> = HashMap::new();
  
  parsers.iter().for_each(| (_, parser)| {
    parsers_folder_map.insert(parser.folder.clone(), parser.to_owned());
  });

  let roms_path = PathBuf::from(&library.libraryPath).join(&library.romDir);

  let entries_res = read_dir(&roms_path);
  if entries_res.is_err() {
    let err = entries_res.err().unwrap();
    
    send_error(
      format!("Failed to load library: {}", err.to_string()),
      format!("Please double check that there is a library mounted to \"{}\".", library.libraryPath),
      crate::websocket::types::BackendErrorType::WARN
    );

    return Err(());
  }

  for dir_entry_res in entries_res.unwrap() {
    if dir_entry_res.is_err() {
      continue;
    }
    let dir_entry = dir_entry_res.unwrap();
    let dir_name_os = dir_entry.file_name();
    let dir_name = dir_name_os.to_str().unwrap().to_owned();

    let dir_metadata_res = dir_entry.metadata();
    if dir_metadata_res.is_err() {
      let err = dir_metadata_res.err().unwrap();
      warn!("Library: Can't read metadata of directory \"{}\": {}", dir_name, err.to_string());
      continue;
    }
    let dir_metadata = dir_metadata_res.unwrap();

    if dir_metadata.is_dir() && parsers_folder_map.contains_key(&dir_name) {
      let parser = parsers_folder_map.get(&dir_name).unwrap();

      let mut platform_roms = load_platform(parser, dir_entry.path());

      roms.append(&mut platform_roms);
      
      watcher.watch_path(dir_entry.path());
    }
  }

  return Ok(roms);
}


fn load_platform_extras(dict: &mut HashMap<String, Vec<String>>, path: PathBuf) {
  let entries_res = read_dir(&path);
  for dir_entry_res in entries_res.unwrap() {
    if dir_entry_res.is_err() {
      continue;
    }
    let dir_entry = dir_entry_res.unwrap();
    let rom_id_os = dir_entry.file_name();
    let rom_id = rom_id_os.to_str().unwrap().to_owned();

    let dir_metadata_res = dir_entry.metadata();
    if dir_metadata_res.is_err() {
      let err = dir_metadata_res.err().unwrap();
      warn!("Load Platform Extras: Can't read metadata of directory \"{}\": {}", rom_id, err.to_string());
      continue;
    }
    let dir_metadata = dir_metadata_res.unwrap();

    if dir_metadata.is_dir() {
      let extra_entries_res = read_dir(&dir_entry.path());
      for extra_entry_res in extra_entries_res.unwrap() {
        if extra_entry_res.is_err() {
          continue;
        }

        let extra_entry = extra_entry_res.unwrap();
        let extra_filename_os = extra_entry.file_name();
        let extra_filename = extra_filename_os.to_str().unwrap();

        let extra_metadata_res = extra_entry.metadata();
        if extra_metadata_res.is_err() {
          continue;
        }
        let extra_metadata = extra_metadata_res.unwrap();

        if extra_metadata.is_file() {
          if dict.contains_key(&rom_id) {
            dict.get_mut(&rom_id).unwrap().push(extra_filename.to_string());
          } else {
            let extras = vec![extra_filename.to_string()];

            dict.insert(rom_id.clone(), extras);
          }
        }
      }
    }
  }
}

fn load_extra(path: PathBuf, send_error: &ErrorSender) -> Result<HashMap<String, Vec<String>>, ()> {
  let mut dict: HashMap<String, Vec<String>> = HashMap::new();

  let exists_res = fs::exists(&path);
  if exists_res.is_err() {
    let err = exists_res.err().unwrap();
    
    send_error(
      format!("Failed to load extras: {}", err.to_string()),
      String::from("Please double check that there your extras directory is readable."),
      crate::websocket::types::BackendErrorType::WARN
    );

    return Err(());
  }

  if !exists_res.ok().unwrap() {
    return Ok(dict);
  }

  let entries_res = read_dir(&path);
  for dir_entry_res in entries_res.unwrap() {
    if dir_entry_res.is_err() {
      continue;
    }
    let dir_entry = dir_entry_res.unwrap();
    let dir_name_os = dir_entry.file_name();
    let dir_name = dir_name_os.to_str().unwrap().to_owned();

    let dir_metadata_res = dir_entry.metadata();
    if dir_metadata_res.is_err() {
      let err = dir_metadata_res.err().unwrap();
      warn!("Load Extras: Can't read metadata of directory \"{}\": {}", dir_name, err.to_string());
      continue;
    }
    let dir_metadata = dir_metadata_res.unwrap();

    if dir_metadata.is_dir() {
      let rom_extras_path = dir_entry.path();
      load_platform_extras(&mut dict, rom_extras_path);
    }
  }

  return Ok(dict);
}

fn load_extras(library: &Library, send_error: &ErrorSender) -> Result<(HashMap<String, Vec<String>>, HashMap<String, Vec<String>>), ()> {
  let dlcs_path = PathBuf::from(&library.libraryPath).join(&library.dlcDir);
  let dlcs_res = load_extra(dlcs_path, send_error);
  if dlcs_res.is_err() {
    return Err(());
  }
  let dlcs = dlcs_res.unwrap();
  
  let updates_path = PathBuf::from(&library.libraryPath).join(&library.updateDir);
  let updates_res = load_extra(updates_path, send_error);
  if updates_res.is_err() {
    return Err(());
  }
  let updates = updates_res.unwrap();

  return Ok((dlcs, updates));
}

fn load_library(library: &Library, watcher: &Watcher, send_error: &ErrorSender) -> Result<(LoadResult, HashMap<String, Parser>), ()> {
  let parsers_res = load_parsers(library, send_error);
  if parsers_res.is_err() {
    return Err(());
  }
  let parsers = parsers_res.unwrap();

  let systems: Vec<Parser> = parsers.clone().into_values().collect();

  let roms_res = load_roms(library, watcher, &parsers, &send_error);
  if roms_res.is_err() {
    return Err(());
  }
  
  let roms_res = load_roms(library, watcher, &parsers, &send_error);
  if roms_res.is_err() {
    return Err(());
  }
  
  let extras_res = load_extras(library, send_error);
  if extras_res.is_err() {
    return Err(());
  }
  let extras = extras_res.unwrap();

  return Ok((
    LoadResult {
      library: library.to_owned(),
      roms: roms_res.unwrap(),
      systems,
      dlcs: extras.0,
      updates: extras.1
    },
    parsers
  ));
}

/// Parses the app's library
pub fn parse_library(library: &Library, watcher: &Watcher, sate_store: &mut StateStore, send_error: ErrorSender) -> Result<LoadResult, ()> {
  let load_res = load_library(library, watcher, &send_error);

  if load_res.is_err() {
    return Err(());
  }

  let (loaded_library, parsers) = load_res.unwrap();
  (*sate_store).parsers = parsers.to_owned();

  return Ok(loaded_library);
}

/// Parses a ROM's data from its path.
pub fn parse_added_rom(parser_name: String, rom_path: &str, state_store: &StateStore, send_error: ErrorSender) -> Result<ROM, ()> {
  let parser = state_store.parsers.get(&parser_name).expect("System abbreviation was missing from parser map");

  let path = PathBuf::from(rom_path);

  if path.is_dir() {
    for pattern in &parser.patterns {
      let glob = Glob::new(&pattern.glob).unwrap();

      for entry in glob.walk(&path) {
        if entry.is_err() {
          let err = entry.err().unwrap();
          warn!("Adding ROM: Parser: \"{}\"; Failed to walk entry \"{}\"", &parser.abbreviation, err.path().unwrap().display());
          continue;
        }
  
        let path = entry.unwrap().into_path();
  
        return Ok(load_rom(
          parser,
          pattern,
          path
        ));
      }
    }
  } else {
    let filename = path.file_name().unwrap().to_str().unwrap();

    for pattern in &parser.patterns {
      let glob = Glob::new(&pattern.glob).unwrap();
  
      if glob.is_match(filename) {
        return Ok(load_rom(
          parser,
          pattern,
          path
        ));
      }
    }
  }

  send_error(
    format!("Adding ROM: ROM should have matched on of the parsers for system \"{}\".", parser_name),
    format!("Please double check that there is a parser for system \"{}\".", parser_name),
    crate::websocket::types::BackendErrorType::WARN
  );

  return Err(());
}