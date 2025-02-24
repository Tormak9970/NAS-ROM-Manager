use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct SGDBGame {
  pub id: u64,
  pub name: String,
  pub types: Vec<String>,
  pub verified: bool,
  // pub numResultPages: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct SGDBAuthor {
  pub name: String,
  pub steam64: String,
  pub avatar: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct GridResults {
  pub images: Vec<SGDBImage>,
  pub page: u64,
  pub total: u64,
}

fn default_animated() -> bool {
  false
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct SGDBImage {
  pub author: SGDBAuthor,

  pub id: u64,
  pub url: String,
  pub thumb: String,
  pub width: u64,
  pub height: u64,
  pub language: String,

  pub style: String,
  pub mime: String,
  pub humor: bool,
  pub epilepsy: bool,
  pub nsfw: bool,
  pub notes: Option<String>,

  #[serde(default = "default_animated")]
  pub isAnimated: bool,

  pub downvotes: u64,
  pub upvotes: u64,

  pub lock: bool,
  // is_deleted: bool,
}