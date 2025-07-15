use std::{collections::HashMap, path::PathBuf};

use log::warn;
use warp::{reject::Rejection, reply::Reply};

use super::{types::StreamStore, utils::{download::get_file_metadata}};

/// Completes the Rom Extra upload.
pub async fn rom_extra_upload_complete(streams_store: StreamStore, upload_id: String) -> Result<impl Reply, Rejection> {
  streams_store.streams.write().await.remove(&upload_id);

  return Ok(warp::reply::with_header("success", "Access-Control-Allow-Origin", "*"));
}

/// Gets the download metadata for a rom extras file.
pub async fn rom_extra_download_get_metadata(query_params: HashMap<String, String>) -> Result<impl Reply, Rejection> {
  if !query_params.contains_key("filePath") {
    warn!("Get ROM Extras Metadata: Missing query param filePath");
    return Err(warp::reject::reject());
  }

  let path = query_params.get("filePath").unwrap().to_owned();
  let file_path = PathBuf::from(&path);

  return get_file_metadata(&file_path).await;
}

/// Handles deleting a rom extra file.
pub async fn delete_rom_extra(query_params: HashMap<String, String>) -> Result<impl Reply, Rejection> {
  if !query_params.contains_key("filePath") {
    warn!("Delete ROM Extras File: Missing query param filePath");
    return Err(warp::reject::reject());
  }
  let path = query_params.get("filePath").unwrap().to_owned();
  
  tokio::fs::remove_file(&path).await.map_err(|e| {
    warn!("Error deleting ROM Extras file: {}", e);
    warp::reject::reject()
  })?;

  return Ok(warp::reply::with_header("success", "Access-Control-Allow-Origin", "*"));
}