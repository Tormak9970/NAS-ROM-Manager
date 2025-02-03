use std::{collections::HashMap, env::var, fs::{self, read_dir, File}, path::PathBuf};
use chrono::{DateTime, Local};
use log::warn;
use serde_json::Map;
use tokio::sync::broadcast;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::RegexBuilder;
use serde::{Deserialize, Serialize};
use wax::{Glob, Pattern};

use crate::websocket::{
  types::{
    Library,
    Parser,
    ParserPattern,
    Settings,
    ROM,
    ParserStore
  },
  watcher::Watcher
};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct LoadedLibrary {
  pub library: Library,
  pub roms: Vec<ROM>,
}

/// Loads a library's parsers.
fn load_parsers(library: &Library, tx: broadcast::Sender<String>) -> HashMap<String, Parser> {
  let mut parsers: HashMap<String, Parser> = HashMap::new();

  let mut parsers_path = library.parsersPath.clone();
  if library.useProvidedParsers {
    let env_parsers_default_res = var("NRM_DEFAULT_PARSERS_DIR");
    if env_parsers_default_res.is_err() {
      warn!("No default_parsers variable \"NRM_DEFAULT_PARSERS_DIR\" was found!");
      tx.send(format!("missing_env_variable NRM_DEFAULT_PARSERS_DIR")).expect("Failed to broadcast message");
      return parsers;
    }

    parsers_path = env_parsers_default_res.unwrap();
  }

  let entries_res = read_dir(&parsers_path);
  if entries_res.is_err() {
    let err = entries_res.err().unwrap();
    warn!("Library: \"{}\"; Can't load parsers: {}", library.name, err.to_string());
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
      warn!("Library: \"{}\"; Can't read metadata of parser \"{}\": {}", library.name, parser_filename, err.to_string());
      continue;
    }
    let parser_metadata = parser_metadata_res.unwrap();

    if parser_metadata.is_file() {
      let parser_file_res = File::open(&parser_entry.path());
      if parser_file_res.is_err() {
        let err = parser_file_res.err().unwrap();
        warn!("Library: \"{}\"; Can't read parser \"{}\": {}", library.name, parser_filename, err.to_string());
        continue;
      }

      let parser: Parser = serde_json::from_reader(parser_file_res.ok().unwrap()).unwrap();

      parsers.insert(parser.folder.clone(), parser);
    }
  }

  return parsers;
}

fn load_rom(library_name: &str, parser: &Parser, pattern: &ParserPattern, path: PathBuf) -> ROM {
  let path_str = path.to_str().unwrap().to_string();
  let metadata = fs::metadata(&path).expect("Failed to read ROM metadata");

  let create_date: DateTime<Local> = DateTime::<Local>::from(metadata.created().unwrap());
  let extension = path.extension().unwrap().to_str().unwrap();

  let mut title = path_str.clone();

  let mut regex_builder = RegexBuilder::new(&pattern.regex);
  regex_builder.case_insensitive(true);

  let regex_res = regex_builder.build();
  if regex_res.is_err() {
    let err = regex_res.err().unwrap();
    warn!("Library: \"{}\"; Parser: \"{}\"; Failed to parse glob pattern \"{}\": {}", library_name, &parser.abbreviation, &pattern.glob, err.to_string());
  } else {
    let regex = regex_res.unwrap();

    let clean_title = title.replace("\\", "/");

    let captures = regex.captures(&clean_title);

    if captures.is_some() {
      let results = captures.unwrap();
      if results.len() > 0 {
        title = results.name("title").unwrap().as_str().to_owned();
      }
    }
  }

  return ROM {
    title,
    path: path_str,
    size: metadata.len(),
    addDate: format!("{}", create_date.format("%B %e, %Y")),
    format: extension.to_owned(),
    system: parser.abbreviation.clone(),
    systemFullName: parser.name.clone(),
    library: library_name.to_string(),
    downloadStrategy: pattern.downloadStrategy.clone(),
  };
}

fn load_platform(library: &Library, parser: &Parser, path: PathBuf) -> Vec<ROM> {
  let mut roms: Vec<ROM> = Vec::new();

  for pattern in &parser.patterns {
    let glob = Glob::new(&pattern.glob).unwrap();
    for entry in glob.walk(&path) {
        if entry.is_err() {
          let err = entry.err().unwrap();
          warn!("Library: \"{}\"; Parser: \"{}\"; Failed to walk entry \"{}\"", &library.name, &parser.abbreviation, err.path().unwrap().display());
          continue;
        }

        let path = entry.unwrap().into_path();
  
        roms.push(load_rom(
          &library.name,
          parser,
          pattern,
          path
        ));
    }
  }

  return roms;
}

fn load_library(library: &Library, watcher: &Watcher, tx: broadcast::Sender<String>) -> (LoadedLibrary, HashMap<String, Parser>) {
  let parsers = load_parsers(library, tx);
  let mut roms: Vec<ROM> = vec![];

  let entries_res = read_dir(&library.path);
  if entries_res.is_err() {
    let err = entries_res.err().unwrap();
    warn!("Library: \"{}\"; Failed to load: {}", library.name, err.to_string());
    return (
      LoadedLibrary {
        library: library.to_owned(),
        roms,
      },
      parsers
    );
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
      warn!("Library: \"{}\"; Can't read metadata of directory \"{}\": {}", library.name, dir_name, err.to_string());
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

  return (
    LoadedLibrary {
      library: library.to_owned(),
      roms,
    },
    parsers
  );
}

/// Loads the app's libraries.
pub fn load_libraries(state_settings: &Settings, watcher: &Watcher, parser_store: &mut ParserStore, tx: broadcast::Sender<String>) -> Vec<LoadedLibrary> {
  let libraries = &state_settings.libraries;

  let loaded_data: Vec<(LoadedLibrary, HashMap<String, Parser>)> = libraries.par_iter().map_with(watcher, | watcher_par, library | {
    return load_library(&library, &watcher_par, tx.clone());
  }).collect();

  let loaded_libraries: Vec<LoadedLibrary> = loaded_data.iter().map(| (loaded_library, parsers) | {
    (*parser_store).libraries.insert(loaded_library.library.name.clone(), parsers.to_owned());
    return loaded_library.to_owned();
  }).collect();

  return loaded_libraries;
}

/// Adds a library to the app
pub fn add_library(library: &Library, watcher: &Watcher, parser_store: &mut ParserStore, tx: broadcast::Sender<String>) -> LoadedLibrary {
  let (loaded_library, parsers) = load_library(library, watcher, tx);
  (*parser_store).libraries.insert(loaded_library.library.name.clone(), parsers.to_owned());

  return loaded_library;
}

/// Parses a ROM's data from its path.
pub fn parse_added_rom(library_name: String, system: String, rom_path: &str, parser_store: &ParserStore) -> ROM {
  let parsers = parser_store.libraries.get(&library_name).expect("Library name was missing from parser map");
  let parser = parsers.get(&system).expect("System abbreviation was missing from parser map");

  for pattern in &parser.patterns {
    let glob = Glob::new(&pattern.glob).unwrap();

    if glob.is_match(rom_path) {
      return load_rom(
        &library_name,
        parser,
        pattern,
        PathBuf::from(rom_path)
      );
    }
  }

  warn!("ROM should have matched on of the parsers for system \"{}\".", system);

  return ROM {
    title: "ERROR".to_string(),
    path: "ERROR".to_string(),
    size: 0,
    addDate: "ERROR".to_string(),
    format: "ERROR".to_string(),
    system: "ERROR".to_string(),
    systemFullName: "ERROR".to_string(),
    library: "ERROR".to_string(),
    downloadStrategy: Map::new(),
  }
}