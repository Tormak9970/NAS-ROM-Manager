use std::{collections::HashMap, path::PathBuf};

use log::warn;
use warp::{reject::Rejection, reply::Reply};

use super::{types::StreamStore, utils::{download::get_file_metadata}};

/// Completes the Bios File upload.
pub async fn bios_file_upload_complete(streams_store: StreamStore, upload_id: String) -> Result<impl Reply, Rejection> {
  streams_store.streams.write().await.remove(&upload_id);

  return Ok(warp::reply::with_header("success", "Access-Control-Allow-Origin", "*"));
}

/// Gets the download metadata for a bios file.
pub async fn bios_file_download_get_metadata(query_params: HashMap<String, String>) -> Result<impl Reply, Rejection> {
  if !query_params.contains_key("filePath") {
    warn!("Get BIOS Metadata: Missing query param filePath");
    return Err(warp::reject::reject());
  }

  let path = query_params.get("filePath").unwrap().to_owned();
  let file_path = PathBuf::from(&path);

  return get_file_metadata(&file_path).await;
}

/// Handles deleting a bios file.
pub async fn delete_bios_file(query_params: HashMap<String, String>) -> Result<impl Reply, Rejection> {
  if !query_params.contains_key("filePath") {
    warn!("Delete BIOS: Missing query param filePath");
    return Err(warp::reject::reject());
  }
  let path = query_params.get("filePath").unwrap().to_owned();
  
  tokio::fs::remove_file(&path).await.map_err(|e| {
    warn!("Error deleting BIOS file: {}", e);
    warp::reject::reject()
  })?;

  return Ok(warp::reply::with_header("success", "Access-Control-Allow-Origin", "*"));
}