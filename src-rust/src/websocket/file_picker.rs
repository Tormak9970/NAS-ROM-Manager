// check https://github.com/SteamDeckHomebrew/decky-loader/blob/543ee3d19ed41a951eabd90ccbf81e221f54ae3d/backend/decky_loader/utilities.py#L312 for ref

use serde_json::{Map, Value};

// TODO: actually type config, this is just a placeholder.
pub async fn get_entries(path: String, config: Map<String, Value>) -> Vec<String> {
  return vec![];
} 