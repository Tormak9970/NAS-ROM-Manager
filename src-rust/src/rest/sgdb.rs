use std::{collections::HashMap, env::var, time::Duration};
use log::warn;

use reqwest::{header::{self, HeaderMap, HeaderValue}, Client};
use serde::{de::DeserializeOwned, Deserialize};
use warp::{reject::Rejection, reply::Reply};

use super::types::{sgdb::{GridResults, SGDBGame, SGDBImage}, SGDBClientStore};

#[derive(Clone, Deserialize)]
struct ResponseGeneric<T> {
  pub data: T,
}

#[derive(Clone, Deserialize)]
struct Results {
  pub data: Vec<SGDBImage>,
  pub page: u64,
  pub total: u64,
}

#[derive(Clone, Debug)]
pub struct SGDBClient {
  pub key: String,
  base_url: String,
  client: Client,
  timeout: u64,
}

impl SGDBClient {
  /// Creates a new SGDB Client
  pub fn new(key: String, timeout: u64) -> SGDBClient {
    let mut headers = HeaderMap::new();

    headers.insert(header::AUTHORIZATION, HeaderValue::try_from(format!("Bearer {key}")).unwrap());
    headers.insert(header::ACCEPT, HeaderValue::from_static("application/json"));

    let http_client_res = Client::builder()
      .timeout(Duration::from_secs(timeout))
      .default_headers(headers)
      .build();
    let http_client: Client = http_client_res.expect("Failed to make the reqwest client.");

    return SGDBClient {
      key,
      base_url: "https://www.steamgriddb.com/api/".to_string(),
      client: http_client,
      timeout,
    };
  }

  /// Updates the client's api key.
  pub fn update_key(&mut self, key: String) {
    let mut headers = HeaderMap::new();

    headers.insert(header::AUTHORIZATION, HeaderValue::try_from(format!("Bearer {key}")).unwrap());
    headers.insert(header::ACCEPT, HeaderValue::from_static("application/json"));

    let http_client_res = Client::builder()
      .timeout(Duration::from_secs(self.timeout))
      .default_headers(headers)
      .build();
    let http_client: Client = http_client_res.expect("Failed to make the reqwest client.");

    self.client = http_client;
    self.key = key;
  }

  /// Makes a request.
  async fn handle_request<T: DeserializeOwned>(&self, url: String, params: Option<HashMap<String, String>>) -> Result<T, String> {
    let entries: Vec<(String, String)> = params.unwrap_or(HashMap::new()).clone().into_iter().collect();
    
    let response_res = self.client.get(format!("{}v2{}", self.base_url, url))
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


  /// Searches for SGDB games by their id.
  pub async fn search_game(&self, query: &str) -> Result<Vec<SGDBGame>, String> {
    let encoded_query = urlencoding::encode(query).into_owned();

    let res = self.handle_request::<ResponseGeneric<Vec<SGDBGame>>>(format!("/search/autocomplete/{encoded_query}"), None).await;
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    return Ok(res.unwrap().data);
  }

  /// Gets the grids for a game based on its SGDB id.
  pub async fn get_grids_by_id(&self, id: u64, page: u16) -> Result<GridResults, String> {
    let mut params = HashMap::new();

    params.insert("types".to_string(), "static,animated".to_string());
    params.insert("nsfw".to_string(), "any".to_string());
    params.insert("humor".to_string(), "any".to_string());
    params.insert("epilepsy".to_string(), "any".to_string());
    params.insert("page".to_string(), page.to_string());

    let res = self.handle_request::<Results>(format!("/grids/game/{id}"), Some(params)).await;
    if res.is_err() {
      return Err(res.err().unwrap());
    }
    let results = res.unwrap();

    let grids = results.data.into_iter().map(| mut img | {
      img.isAnimated = img.thumb.contains(".webm");
      return img;
    }).collect();

    return Ok(GridResults {
      images: grids,
      page: results.page,
      total: results.total,
    });
  }
}


/// Initializes the SGDB API Client, and returns an error if there was no API key env variable.
pub async fn init_sgdb_client(sgdb_client_store: SGDBClientStore) -> Result<impl Reply, Rejection> {
  if sgdb_client_store.client.read().await.key == "".to_string() {
    let api_key_res = var("SGDB_API_KEY");
    if api_key_res.is_err() {
      warn!("No environment variable \"SGDB_API_KEY\" was found!");
      return Err(warp::reject::reject());
    }

    sgdb_client_store.client.write().await.update_key(api_key_res.unwrap());
  }

  return Ok(warp::reply::with_header("success", "Access-Control-Allow-Origin", "*"));
}

/// Gets the SGDB grids for the provided game.
pub async fn sgdb_get_grids_by_id(sgdb_client_store: SGDBClientStore, sgdb_id: String, results_page: String) -> Result<impl Reply, Rejection> {
  if sgdb_client_store.client.read().await.key == "".to_string() {
    warn!("SGDB Client was not initialized before request (Get Grids)");
    return Err(warp::reject::reject());
  }

  let id = sgdb_id.parse::<u64>().map_err(|e| {
    warn!("error parsing SGDB-Game-Id: {}", e);
    warp::reject::reject()
  })?;
  
  let page = results_page.parse::<u16>().map_err(|e| {
    warn!("error parsing SGDB-Game-Id: {}", e);
    warp::reject::reject()
  })?;

  let res = sgdb_client_store.client.read().await.get_grids_by_id(id, page).await;

  if res.is_err() {
    warn!("SGDB Get Grids Error: {}", res.err().unwrap());
    return Err(warp::reject::reject());
  }

  let results = res.unwrap();

  let response = warp::http::Response::builder()
    .status(200)
    .header("Access-Control-Allow-Origin", "*")
    .body(serde_json::to_string(&results).unwrap())
    .map_err(|_| warp::reject())?;

  return Ok(response);
}

/// Gets the SGDB Games for the provided search query.
pub async fn sgdb_search_game(sgdb_client_store: SGDBClientStore, query_params: HashMap<String, String>) -> Result<impl Reply, Rejection> {
  if !query_params.contains_key("query") {
    warn!("SGDB Search request did not contain query");
    return Err(warp::reject::reject());
  }

  let query = query_params.get("query").unwrap().to_owned();

  if sgdb_client_store.client.read().await.key == "".to_string() {
    warn!("SGDB Client was not initialized before request (Search Game)");
    return Err(warp::reject::reject());
  }

  let res = sgdb_client_store.client.read().await.search_game(&query).await;

  if res.is_err() {
    warn!("SGDB Search Game Error: {}", res.err().unwrap());
    return Err(warp::reject::reject());
  }

  let results = res.unwrap();

  let response = warp::http::Response::builder()
    .status(200)
    .header("Access-Control-Allow-Origin", "*")
    .body(serde_json::to_string(&results).unwrap())
    .map_err(|_| warp::reject())?;

  return Ok(response);
}