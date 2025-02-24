pub mod settings;
pub mod args;
pub mod library;
pub mod file_picker;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub enum BackendErrorType {
  WARN,
  PANIC,
}

impl Into<usize> for BackendErrorType {
  fn into(self) -> usize {
    self as usize
  }
}

pub type ErrorSender = Box<dyn Fn(String, String, BackendErrorType)>;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct AvailableStorage {
  pub usedSpace: u64,
  pub totalSpace: u64,
}