use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub enum FileSelectionType {
  FILE,
  FOLDER,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct FilePickerEntry {
  pub path: String,
  pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct FilePickerConfig {
  pub showFiles: bool,
  pub extensions: Option<Vec<String>>,
  pub showHiddenFiles: bool,
  pub max: u64,
}