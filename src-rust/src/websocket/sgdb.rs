use std::{collections::HashMap, time::Duration};

use reqwest::{header::{self, HeaderMap, HeaderValue}, Client};
use serde::{de::DeserializeOwned, Deserialize};

use super::types::sgdb::{GridResults, SGDBGame, SGDBImage};

#[derive(Clone, Deserialize)]
struct ResponseGeneric<T> {
  pub data: T,
}

#[derive(Clone, Deserialize)]
struct Images {
  images: Vec<SGDBImage>,
}

#[derive(Clone, Deserialize)]
struct Results {
  pub data: Images,
  pub page: u64,
  pub total: u64,
}

#[derive(Clone)]
pub struct SGDBClient {
  key: String,
  base_url: String,
  client: Client,
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
    };
  }

  /// Makes a request.
  async fn handle_request<T: DeserializeOwned>(self, url: String, params: Option<HashMap<String, String>>) -> Result<T, String> {
    let mut stringified_params: String = "".to_string();

    if params.is_some() {
      params.unwrap().into_iter().for_each(| (param, val) | {
        let entry = format!("&{param}={val}");
        stringified_params.push_str(&entry);
      });
      let without_mark = (&stringified_params[1..]).to_string();
      
      stringified_params = "?".to_string();
      stringified_params.push_str(&without_mark);
    }
    
    let response_res = self.client.get(format!("{}v2{}{}", self.base_url, url, stringified_params)).send().await;

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
  pub async fn search_game(self, query: String) -> Result<Vec<SGDBGame>, String> {
    let encoded_query = urlencoding::encode(&query).into_owned();

    let res = self.handle_request::<ResponseGeneric<Vec<SGDBGame>>>(format!("/search/autocomplete/{encoded_query}"), None).await;
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    return Ok(res.unwrap().data);
  }

  /// Gets the grids for a game based on its SGDB id.
  async fn get_grids_by_id(self, id: u64, page: u64) -> Result<GridResults, String> {
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

    let grids = results.data.images.into_iter().map(| mut img | {
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