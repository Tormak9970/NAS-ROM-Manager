use std::{collections::HashMap, fs::{read_dir, File}, path::PathBuf, sync::MutexGuard};
use log::warn;

use serde::{Deserialize, Serialize};

use crate::{types::{Library, Parser, Settings, ROM}, watcher::Watcher};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct LoadedLibrary {
  pub library: Library,
  pub ROMs: Vec<ROM>,
}

/// Loads a library's parsers.
fn load_parsers(library: &Library) -> HashMap<String, Parser> {
  let mut parsers: HashMap<String, Parser> = HashMap::new();

  let entries_res = read_dir(&library.parsersPath);
  if entries_res.is_err() {
    let err = entries_res.err().unwrap();
    warn!("Can't load parsers for library \"{}\": {}", library.name, err.to_string());
    return parsers;
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
      warn!("Can't read metadata of parser \"{}\" for library \"{}\": {}", parser_filename, library.name, err.to_string());
      continue;
    }
    let parser_metadata = parser_metadata_res.unwrap();

    if parser_metadata.is_file() {
      let parser_file_res = File::open(&parser_entry.path());
      if parser_file_res.is_err() {
        let err = parser_file_res.err().unwrap();
        warn!("Can't read parser \"{}\" for library \"{}\": {}", parser_filename, library.name, err.to_string());
        continue;
      }

      let parser: Parser = serde_json::from_reader(parser_file_res.ok().unwrap()).unwrap();

      parsers.insert(parser.folder, parser);
    }
  }

  return parsers;
}

fn load_platform(library: &Library, parser: &Parser, path: PathBuf) -> Vec<ROM> {

}

fn load_library(library: &Library, watcher: &Watcher) -> LoadedLibrary {
  let parsers = load_parsers(library);
  let mut roms: Vec<ROM> = vec![];

  let entries_res = read_dir(&library.path);
  if entries_res.is_err() {
    let err = entries_res.err().unwrap();
    warn!("Can't load library \"{}\": {}", library.name, err.to_string());
    return LoadedLibrary {
      library: library.to_owned(),
      ROMs: roms,
    };
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
      warn!("Can't read metadata of directory \"{}\" in library \"{}\": {}", dir_name, library.name, err.to_string());
      continue;
    }
    let dir_metadata = dir_metadata_res.unwrap();

    if dir_metadata.is_dir() && parsers.contains_key(&dir_name) {
      let parser = parsers.get(&dir_name).unwrap();

      let mut platform_roms = load_platform(library, parser, dir_entry.path());

      roms.append(&mut platform_roms);
      
      watcher.watch_path(dir_entry.path(), library.path.to_owned());
    }
  }

  return LoadedLibrary {
    library: library.to_owned(),
    ROMs: roms,
  };
}

/// Loads the app's libraries.
pub fn load_libraries(state_settings: &Settings, watcher: &Watcher) -> Vec<LoadedLibrary> {
  let libraries = &state_settings.libraries;

  let mut loaded_libraries: Vec<LoadedLibrary> = vec![];

  for library in libraries {
    let loaded_library = load_library(&library, &watcher);
    loaded_libraries.push(loaded_library);
  }

  return loaded_libraries;
}

/// Adds a library to the app
pub fn add_library(library: &Library, watcher: &Watcher) -> LoadedLibrary {
  return load_library(library, watcher);
}