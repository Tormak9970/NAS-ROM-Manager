use std::time::Duration;

use log::info;
use reqwest::{header::{self, HeaderMap, HeaderValue}, Client, StatusCode};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::rest::types::igdb::{IGDBAgeRating, IGDBCoverResponse, IGDBMetadata, IGDBMetadataPlatform, IGDBNamedResponse, IGDBRelatedGame, IGDBRelatedGameResponse, IGDBRom, IGDBRomResponse, IGDBRomsResponse, IGDBSearchResponse, GAMES_FIELDS, IGDB_AGE_RATINGS, SEARCH_FIELDS};

use super::twitch_auth::TwitchAuth;

fn get_fields(fields: &[&str]) -> String {
  return fields.join(",");
}

fn remove_special_chars(query: &str) -> String {
  return query.replace("\u{2122}", "")        // Remove trademark symbol
    .replace("\u{00a9}", "")                  // Remove copywrite symbol
    .replace("\u{00ae}", "")                  // Remove registered symbol
    .replace("\u{2120}", "")                  // Remove service mark symbol
    .replace("\u{003A}", "")                  // Remove colon symbol
    .trim()                                               
    .to_string();
}

fn map_to_name(list: &Vec<IGDBNamedResponse>) -> Vec<String> {
  return list.iter().filter_map(| entry | entry.name.clone()).collect(); 
}

fn map_to_related_game(games: &Vec<IGDBRelatedGameResponse>, relation_type: &str) -> Vec<IGDBRelatedGame> {
  return games.iter().map(| game | {
    let thumb_url = game.cover.url.clone().unwrap_or("".to_string());
    let cover_url = thumb_url.replace("t_thumb", "t_1080p");
    
    return IGDBRelatedGame {
      id: game.id.clone(),
      slug: game.slug.clone().unwrap_or("".to_string()),
      name: game.name.clone().unwrap_or("".to_string()),
      coverUrl: cover_url,
      thumbUrl: thumb_url,
      r#type: relation_type.to_string(),
    }
  }).collect(); 
}

fn extract_metadata_from_response(rom: IGDBRomResponse) -> IGDBMetadata {
  let default_string: Value = Value::String("".to_string());

  let default_named = IGDBNamedResponse {
    name: Some("".to_string()),
  };

  let mut franchises: Vec<String> = vec![
    rom.franchise.unwrap_or(default_named.clone()).name.unwrap_or("".to_string())
  ];
  let mut other_francises = map_to_name(&rom.franchises.unwrap_or(vec![]));
  franchises.append(&mut other_francises);

  let companies: Vec<String> = rom.involved_companies.unwrap_or(vec![]).iter().filter_map(| company | company.company.name.clone()).collect();

  let platforms: Vec<IGDBMetadataPlatform> = rom.platforms.unwrap_or(vec![]).iter().map(| platform | {
    let id = platform.get("id").unwrap_or(&default_string).as_u64().unwrap();
    let name = platform.get("name").unwrap_or(&default_string).as_str().unwrap().to_string();
    let abbreviation = platform.get("abbreviation").unwrap_or(&default_string).as_str().unwrap().to_string();

    return IGDBMetadataPlatform {
      igdbId: id,
      name,
      abbreviation,
    }
  }).collect();
  
  let age_ratings: Vec<IGDBAgeRating> = rom.age_ratings.unwrap_or(vec![]).iter().filter_map(| rating | {
    let rating_id = rating.get("rating").expect("rating_map should have had rating prop.").as_str().unwrap_or("None");

    return IGDB_AGE_RATINGS.get(rating_id);
  }).map(| age_rating | age_rating.to_owned().into()).collect(); 

  return IGDBMetadata {
    totalRating: format!("{:.2}", rom.total_rating.unwrap_or(0.0)),
    aggregatedRating: format!("{:.2}", rom.aggregated_rating.unwrap_or(0.0)),
    firstReleaseDate: rom.first_release_date.unwrap_or(0),
    genres: map_to_name(&rom.genres),
    franchises,
    alternativeNames: map_to_name(&rom.alternative_names.unwrap_or(vec![])),
    game_modes: map_to_name(&rom.game_modes.unwrap_or(vec![])),
    companies,
    platforms,
    ageRatings: age_ratings,
    similarGames: map_to_related_game(&rom.similar_games.unwrap_or(vec![]), "similar"),
  };
}

#[derive(Clone, Debug)]
pub struct IGDBClient {
  token: String,
  pub client_id: String,
  timeout: u64,
  games_endpoint: String,
  search_endpoint: String,
  twitch_auth: TwitchAuth,
  client: Client,
}

impl IGDBClient {
  /// Creates a new IGDB Client
  pub fn new(timeout: u64) -> IGDBClient {
    let base_url = "https://api.igdb.com/v4".to_string();

    let twitch_auth = TwitchAuth::new(timeout);

    let client_res = Client::builder()
      .timeout(Duration::from_secs(timeout))
      .build();

    return IGDBClient {
      token: "".to_string(),
      client_id: "".to_string(),
      games_endpoint: format!("{}/games", &base_url),
      search_endpoint: format!("{}/search", &base_url),
      twitch_auth,
      client: client_res.expect("Failed to make the reqwest client."),
      timeout,
    };
  }

  /// Initialies the IGDB Client
  pub fn init(&mut self) -> Result<(), String> {
    let client_id_res = self.twitch_auth.init();

    if client_id_res.is_err() {
      return Err(client_id_res.err().unwrap().to_string());
    }

    self.client_id = client_id_res.unwrap();

    return Ok(());
  }

  /// Updates the client's api token.
  pub fn update_token(&mut self, token: String) {
    let mut headers = HeaderMap::new();

    headers.insert(header::AUTHORIZATION, HeaderValue::try_from(format!("Bearer {token}")).unwrap());
    headers.insert("Client-ID", HeaderValue::try_from(self.client_id.clone()).unwrap());
    headers.insert(header::ACCEPT, HeaderValue::from_static("application/json"));

    let client_res = Client::builder()
      .timeout(Duration::from_secs(self.timeout))
      .default_headers(headers)
      .build();

    self.client = client_res.expect("Failed to make the reqwest client.");
    self.token = token;
  }

  /// Checks to make sure the token is still valid
  async fn check_oauth(&mut self) -> Result<(), String> {
    let token_res = self.twitch_auth.get_oauth_token().await;
    if token_res.is_err() {
      return Err(token_res.err().unwrap().to_string());
    }
    let token = token_res.unwrap();

    if token != self.token {
      self.update_token(token);
    }

    return Ok(());
  }

  /// Makes a request.
  async fn handle_request<T: DeserializeOwned>(&mut self, url: String, body: String) -> Result<T, String> {
    let update_res = self.check_oauth().await;
    if update_res.is_err() {
      return Err(update_res.err().unwrap().to_string());
    }
    
    let response_res = self.client.post(&url)
      .body(body.clone()).send().await;

    if response_res.is_err() {
      let err = response_res.err().expect("Request failed, error should have existed.");
      return Err(err.to_string());
    }

    let mut response = response_res.ok().expect("Failed to get response from ok result.");
    

    if response.status() == StatusCode::UNAUTHORIZED {
      let update_res = self.check_oauth().await;
      if update_res.is_err() {
        return Err(update_res.err().unwrap().to_string());
      }

      let response_res = self.client.get(&url)
      .body(body).send().await;

      if response_res.is_err() {
        let err = response_res.err().expect("Request failed, error should have existed.");
        return Err(err.to_string());
      }

      response = response_res.ok().expect("Failed to get response from ok result.");
    }

    
    if response.status() != StatusCode::OK {
      return Err(response.error_for_status().err().unwrap().to_string());
    }

    let data: T = response.json().await.expect("Data should have been of type");

    return Ok(data);
  }

  async fn search_rom_helper(&mut self, query: &str, igdb_platform_id: &str) -> Result<IGDBRomResponse, String> {
    //  & game.category=(0,8,9,10,11)
    let body = format!("fields {}; where game.platforms=[{}] & (name ~ *\"{}\"* | alternative_name ~ *\"{}\"*);", get_fields(&SEARCH_FIELDS), igdb_platform_id, query, query);
    let search_res = self.handle_request::<Vec<IGDBSearchResponse>>(self.search_endpoint.clone(), body).await;

    if search_res.is_err() {
      return Err(search_res.err().unwrap());
    }
    let results = search_res.unwrap();

    if results.len() == 0 {
      return Err(format!("Search for \"{}\" returned 0 results.", query));
    }

    let data_body = format!("fields {}; where id={};", get_fields(&GAMES_FIELDS), results[0].game.id);

    let rom_res = self.handle_request::<Vec<IGDBRomResponse>>(self.games_endpoint.clone(), data_body).await;
    if rom_res.is_err() {
      return Err(rom_res.err().unwrap());
    }
    let roms = rom_res.unwrap();
    
    if roms.len() == 0 {
      return Err(format!("Linking Roms for Search: \"{}\" returned 0 results.", query));
    }

    return Ok(roms[0].clone());
  }

  /// Gets the rom matching the search query from IGDB.
  pub async fn search_game(&mut self, query: &str, igdb_platform_id: String) -> Result<IGDBRom, String> {
    let cleaned_query = remove_special_chars(query);

    let rom_res = self.search_rom_helper(&cleaned_query, &igdb_platform_id).await;
    if rom_res.is_err() {
      return Err(rom_res.err().unwrap().to_string());
    }
    let rom = rom_res.unwrap();
    
    let thumb_url = rom.cover.clone().unwrap_or(IGDBCoverResponse { url: None }).url.clone().unwrap_or("".to_string());
    let cover_url = thumb_url.replace("t_thumb", "t_1080p");

    return Ok(IGDBRom {
      igdbId: rom.id.clone(),
      slug: rom.slug.clone(),
      name: rom.name.clone(),
      summary: rom.summary.clone(),
      coverUrl: Some(cover_url),
      thumbUrl: Some(thumb_url),
      metadata: Some(extract_metadata_from_response(rom)),
    });
  }

  /// Gets a rom by its IGDB id.
  pub async fn get_metadata_by_id(&mut self, igdb_id: String) -> Result<IGDBRom, String> {
    let body = format!("fields {}; where id={};", get_fields(&GAMES_FIELDS), igdb_id);
    let roms_res = self.handle_request::<IGDBRomsResponse>(self.games_endpoint.clone(), body).await;

    if roms_res.is_err() {
      return Err(roms_res.err().unwrap().to_string());
    }
    let roms = roms_res.unwrap();

    if roms.len() == 0 {
      return Ok(IGDBRom {
        igdbId: 0,
        slug: None,
        name: None,
        summary: None,
        coverUrl: None,
        thumbUrl: None,
        metadata: None,
      });
    }
    let rom = roms[0].clone();

    let thumb_url = rom.cover.clone().unwrap_or(IGDBCoverResponse { url: None }).url.clone().unwrap_or("".to_string());
    let cover_url = thumb_url.replace("t_thumb", "t_1080p");

    return Ok(IGDBRom {
      igdbId: rom.id.clone(),
      slug: rom.slug.clone(),
      name: rom.name.clone(),
      summary: rom.summary.clone(),
      coverUrl: Some(cover_url),
      thumbUrl: Some(thumb_url),
      metadata: Some(extract_metadata_from_response(rom)),
    });
  }
}