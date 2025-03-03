use std::{collections::HashMap, env::var, time::{Duration, SystemTime, UNIX_EPOCH}};

use log::warn;
use reqwest::Client;


#[derive(Clone, Debug)]
pub struct TwitchAuth {
  base_url: String,
  client: Client,
  params: HashMap<String, String>,
  expires_in: u64,
  last_checked: u64,
  token: String,
  timeout: u64,
}

fn get_current_seconds() -> u64 {
  let time = SystemTime::now().duration_since(UNIX_EPOCH);
  return time.expect("Should always be able to get time.").as_secs();
}

impl TwitchAuth {
  /// Creates a new TwitchAuth
  pub fn new(timeout: u64) -> TwitchAuth {
    let mut params = HashMap::new();

    params.insert("client_id".to_string(), "".to_string());
    params.insert("client_secret".to_string(), "".to_string());
    params.insert("grant_type".to_string(), "client_credentials".to_string());

    let http_client_res = Client::builder()
      .timeout(Duration::from_secs(timeout))
      .build();

    let http_client: Client = http_client_res.expect("Failed to make the reqwest client.");

    return TwitchAuth {
      base_url: "https://id.twitch.tv/oauth2/token".to_string(),
      params,
      client: http_client,
      expires_in: 0,
      last_checked: get_current_seconds(),
      token: "".to_string(),
      timeout,
    };
  }

  /// Initializes the TwitchAuth client.
  pub fn init(&mut self) -> Result<String, String> {
    let client_id_res = var("IGDB_CLIENT_ID");
    if client_id_res.is_err() {
      warn!("No environment variable \"IGDB_CLIENT_ID\" was found!");
      return Err("No environment variable \"IGDB_CLIENT_ID\" was found!".to_string());
    }
    let client_id = client_id_res.unwrap();
    self.params.insert("client_id".to_string(), client_id.clone());

    let client_secret_res = var("IGDB_CLIENT_SECRET");
    if client_secret_res.is_err() {
      warn!("No environment variable \"IGDB_CLIENT_SECRET\" was found!");
      return Err("No environment variable \"IGDB_CLIENT_SECRET\" was found!".to_string());
    }
    self.params.insert("client_secret".to_string(), client_secret_res.unwrap());

    return Ok(client_id);
  }

  async fn get_updated_token(&mut self) -> Result<String, String> {
    let entries: Vec<(String, String)> = self.params.clone().into_iter().collect();
    
    let response_res = self.client.get(&self.base_url)
      .query(&entries)
      .send().await;

    if response_res.is_ok() {
      let response = response_res.ok().expect("Failed to get response from ok result.");
      let data: HashMap<String, String> = response.json().await.expect("Data should have been of type");

      let token = data.get("access_token").unwrap().to_owned();
      let expires_in = data.get("expires_in").unwrap().to_owned();

      self.expires_in = expires_in.parse::<u64>().unwrap_or(0);
  
      return Ok(token);
    } else {
      let err = response_res.err().expect("Request failed, error should have existed.");
      return Err(err.to_string());
    }
  }

  /// Gets the twitch oauth token.
  pub async fn get_oauth_token(&mut self) -> Result<String, String> {
    let now = get_current_seconds();
    if now > self.expires_in + self.last_checked {
      let new_token = self.get_updated_token().await;

      if new_token.is_err() {
        return Err(new_token.err().unwrap().to_string());
      }

      self.token = new_token.ok().unwrap();
      self.last_checked = now;
    }

    return Ok(self.token.clone());
  }
}