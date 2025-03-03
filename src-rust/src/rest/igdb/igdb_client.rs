use std::{collections::HashMap, time::Duration};

use reqwest::{header::{self, HeaderMap, HeaderValue}, Client};
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

use crate::rest::types::igdb::{IGDBAgeRating, IGDBMetadata, IGDBMetadataPlatform, IGDBRelatedGame, IGDB_AGE_RATINGS};

use super::twitch_auth::{self, TwitchAuth};

fn remove_special_chars(query: &str) -> String {
  return query.replace("\u{2122}", "")        // Remove trademark symbol
    .replace("\u{00a9}", "")                  // Remove copywrite symbol
    .replace("\u{00ae}", "")                  // Remove registered symbol
    .replace("\u{2120}", "")                  // Remove service mark symbol
    .trim()                                               
    .to_string();
}

fn map_to_name(rom: &Map<String, Value>, field: &str, default: &Value) -> Vec<String> {
  return rom.get(field).unwrap_or(default).as_array().unwrap().iter().map(| entry | {
    entry.as_object().expect(format!("{} should have been object", field).as_str()).get("name").unwrap().as_str().unwrap().to_owned()
  }).collect(); 
}

fn map_to_related_game(rom: &Map<String, Value>, field: &str, relation_type: &str, default: &Value) -> Vec<IGDBRelatedGame> {
  let default_string: Value = Value::String("".to_string());
  
  return rom.get(field).unwrap_or(default).as_array().unwrap().iter().map(| entry | {
    let entry_map = entry.as_object().expect("entry should have been a map.");

    let id = entry_map.get("id").unwrap_or(&default_string).as_str().unwrap().to_string();
    let slug = entry_map.get("slug").unwrap_or(&default_string).as_str().unwrap().to_string();
    let name = entry_map.get("name").unwrap_or(&default_string).as_str().unwrap().to_string();

    let cover_map = entry_map.get("cover").unwrap_or(&default_string).as_object().expect("cover should have been a map.");
    let thumb_url = cover_map.get("url").unwrap_or(&default_string).as_str().unwrap().to_string();
    let cover_url = thumb_url.replace("t_thumb", "t_1080p");
    
    return IGDBRelatedGame {
      id,
      slug,
      name,
      cover_url,
      thumb_url,
      r#type: relation_type.to_string(),
    }
  }).collect(); 
}

fn extract_metadata_from_igdb_rom(rom: Map<String, Value>) -> IGDBMetadata {
  let default_rating: Value = 0.0f64.into();
  let default_string: Value = Value::String("".to_string());
  let default_map: Value = Value::Object(Map::new());
  let default_array: Value = Value::Array(vec![]);

  let mut franchises: Vec<String> = vec![
    rom.get("franchise").unwrap_or(&default_map).as_object().unwrap().get("name").unwrap_or(&default_string).as_str().unwrap().to_string()
  ];
  let mut other_francises = map_to_name(&rom, "franchises", &default_array);
  franchises.append(&mut other_francises);

  let companies: Vec<String> = rom.get("involved_companies").unwrap_or(&default_array).as_array().unwrap().iter().map(| company | {
    company.as_object().expect("involved_companies should have been object")
    .get("company").unwrap().as_object().unwrap()
    .get("name").unwrap().as_str().unwrap().to_owned()
  }).collect();

  let platforms: Vec<IGDBMetadataPlatform> = rom.get("platforms").unwrap_or(&default_array).as_array().unwrap().iter().map(| platform | {
    let platform_map = platform.as_object().expect("platform should have been a map.");
    let id = platform_map.get("id").unwrap_or(&default_string).as_str().unwrap().to_string();
    let name = platform_map.get("name").unwrap_or(&default_string).as_str().unwrap().to_string();

    return IGDBMetadataPlatform {
      igdb_id: id,
      name,
    }
  }).collect();
  
  let age_ratings: Vec<IGDBAgeRating> = rom.get("age_ratings").unwrap_or(&default_array).as_array().unwrap().iter().filter_map(| rating | {
    let rating_map = rating.as_object().expect("rating should have been a map.");
    let rating_id = rating_map.get("rating").expect("rating_map should have had rating prop.").as_str().unwrap_or("None");

    return IGDB_AGE_RATINGS.get(rating_id);
  }).map(| age_rating | age_rating.to_owned().into()).collect(); 

  return IGDBMetadata {
    total_rating: format!("{:.2}", rom.get("total_rating").unwrap_or(&default_rating).as_f64().unwrap_or(0.0)),
    aggregated_rating: format!("{:.2}", rom.get("aggregated_rating").unwrap_or(&default_rating).as_f64().unwrap_or(0.0)),
    first_release_date: match rom.get("first_release_date") {
      Some(first_release_date) => {
        first_release_date.as_str().unwrap_or("0").to_string().parse::<u64>().expect("Should have been able to parse release date")
      },
      None => {
        0u64
      }
    },
    genres: map_to_name(&rom, "genres", &default_array),
    franchises,
    alternative_names: map_to_name(&rom, "alternative_names", &default_array),
    collections: map_to_name(&rom, "collections", &default_array),
    game_modes: map_to_name(&rom, "game_modes", &default_array),
    companies,
    platforms,
    age_ratings,
    expansions: map_to_related_game(&rom, "expansions", "expansion", &default_array),
    dlcs: map_to_related_game(&rom, "dlcs", "dlc", &default_array),
    similar_games: map_to_related_game(&rom, "similar_games", "similar", &default_array),
  };
}

#[derive(Clone, Debug)]
pub struct IGDBClient {
  token: String,
  client_id: String,
  timeout: u64,
  base_url: String,
  platform_endpoint: String,
  platform_version_endpoint: String,
  games_endpoint: String,
  search_endpoint: String,
  twitch_auth: TwitchAuth,
  client: Client,
}

impl IGDBClient {
  /// Creates a new IGDB Client
  pub fn new(timeout: u64) -> Result<IGDBClient, String> {
    let base_url = "https://api.igdb.com/v4".to_string();

    let mut twitch_auth = TwitchAuth::new(timeout);
    let client_id_res = twitch_auth.init();

    if client_id_res.is_err() {
      return Err(client_id_res.err().unwrap().to_string());
    }

    let http_client_res = Client::builder()
      .timeout(Duration::from_secs(timeout))
      .build();
    let http_client: Client = http_client_res.expect("Failed to make the reqwest client.");

    return Ok(IGDBClient {
      token: "".to_string(),
      client_id: client_id_res.unwrap(),
      base_url: base_url.clone(),
      platform_endpoint: format!("{}/platforms", &base_url),
      platform_version_endpoint: format!("{}/platform_versions", &base_url),
      games_endpoint: format!("{}/games", &base_url),
      search_endpoint: format!("{}/search", &base_url),
      twitch_auth,
      client: http_client,
      timeout,
    });
  }

  /// Updates the client's api key.
  pub fn update_token(&mut self) {
    let mut headers = HeaderMap::new();

    // TODO: token set here
    let token = "".to_string();

    headers.insert(header::AUTHORIZATION, HeaderValue::try_from(format!("Bearer {token}")).unwrap());
    headers.insert("Client-ID", HeaderValue::try_from(self.client_id.clone()).unwrap());
    headers.insert(header::ACCEPT, HeaderValue::from_static("application/json"));

    let http_client_res = Client::builder()
      .timeout(Duration::from_secs(self.timeout))
      .default_headers(headers)
      .build();
    let http_client: Client = http_client_res.expect("Failed to make the reqwest client.");

    self.client = http_client;
    self.token = token;
  }

  /// Makes a request.
  async fn handle_request<T: DeserializeOwned>(&self, url: String, params: HashMap<String, String>) -> Result<T, String> {
    let entries: Vec<(String, String)> = params.clone().into_iter().collect();
    
    let response_res = self.client.get(url)
      .query(&entries).send().await;

    if response_res.is_ok() {
      let response = response_res.ok().expect("Failed to get response from ok result.");
      let data: T = response.json().await.expect("Data should have been of type");
  
      return Ok(data);
    } else {
      let err = response_res.err().expect("Request failed, error should have existed.");
      return Err(err.to_string());
    }
  }


}