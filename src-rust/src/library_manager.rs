use std::sync::MutexGuard;

use serde::{Deserialize, Serialize};

use crate::types::{Library, Settings, ROM};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct LoadedLibrary {
  pub library: Library,
  pub ROMs: Vec<ROM>,
}

fn load_library(library: &Library) -> LoadedLibrary {
  
}

pub fn load_libraries(state_settings: MutexGuard<'_, Settings>) -> Vec<LoadedLibrary> {

}

pub fn add_library(library: &Library) -> LoadedLibrary {
  
}

pub fn remove_library(library: &Library) -> bool {

}