pub mod settings;
pub mod args;
pub mod library;

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