pub mod igdb_client;
mod twitch_auth;

use std::collections::HashMap;
use log::warn;

use warp::{reject::Rejection, reply::Reply};

use super::types::IGDBClientStore;


/// Initializes the IGDB API Client, and returns an error if there were missing env variables.
pub async fn init_igdb_client(igdb_client_store: IGDBClientStore) -> Result<impl Reply, Rejection> {
  if igdb_client_store.client.read().await.client_id == "".to_string() {
    let init_res = igdb_client_store.client.write().await.init();

    if init_res.is_err() {
      return Err(warp::reject::reject());
    }
  }

  return Ok(warp::reply::with_header("success", "Access-Control-Allow-Origin", "*"));
}

/// Gets the IGDB metadata for the provided game.
pub async fn igdb_get_metadata_by_id(igdb_client_store: IGDBClientStore, igdb_id: String) -> Result<impl Reply, Rejection> {
  if igdb_client_store.client.read().await.client_id == "".to_string() {
    warn!("IGDB Client was not initialized before request (Get Metadata)");
    return Err(warp::reject::reject());
  }

  let res = igdb_client_store.client.write().await.get_metadata_by_id(igdb_id).await;

  if res.is_err() {
    warn!("IGDB Get Metadata Error: {}", res.err().unwrap());
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

/// Gets the IGDB Games for the provided search query.
pub async fn igdb_search_game(igdb_client_store: IGDBClientStore, query_params: HashMap<String, String>) -> Result<impl Reply, Rejection> {
  if !query_params.contains_key("query") {
    warn!("IGDB Search request did not contain query");
    return Err(warp::reject::reject());
  }

  if !query_params.contains_key("platform-id") {
    warn!("IGDB Search request did not contain platform-id");
    return Err(warp::reject::reject());
  }

  let query = query_params.get("query").unwrap().to_owned();
  let igdb_platform_id = query_params.get("platform-id").unwrap().to_owned();

  if igdb_client_store.client.read().await.client_id == "".to_string() {
    warn!("IGDB Client was not initialized before request (Search Game)");
    return Err(warp::reject::reject());
  }

  let res = igdb_client_store.client.write().await.search_game(&query, igdb_platform_id).await;

  if res.is_err() {
    warn!("IGDB Search Game Error: {}", res.clone().err().unwrap());
  }

  let results = res.unwrap_or(vec![]);

  let response = warp::http::Response::builder()
    .status(200)
    .header("Access-Control-Allow-Origin", "*")
    .body(serde_json::to_string(&results).unwrap())
    .map_err(|_| warp::reject())?;

  return Ok(response);
}