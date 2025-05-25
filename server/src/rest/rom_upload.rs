use std::{collections::HashMap, path::PathBuf};

use log::{info, warn};
use tokio::fs::File;
use warp::{reject::Rejection, reply::Reply};

use crate::rest::zip::unpack_zip;

use super::{types::{ROMUploadComplete, StreamStore}, utils::upload::prepare_file_upload};

/// Prepares for the upload of a rom
pub async fn prepare_rom_upload(query_params: HashMap<String, String>) -> Result<impl Reply, Rejection> {
  if !query_params.contains_key("filePath") {
    warn!("Prepare ROM upload: Missing query param filePath");
    return Err(warp::reject::reject());
  }

  let path = PathBuf::from(query_params.get("filePath").unwrap().to_owned());
  return prepare_file_upload(&path).await;
}

/// Completes the ROM upload
pub async fn rom_upload_complete(streams_store: StreamStore, data: ROMUploadComplete) -> Result<impl Reply, Rejection> {
  streams_store.streams.write().await.remove(&data.uploadId);
  
  let mut final_path = data.path.clone();

  if data.unzip {
    let file = File::open(&data.path)
      .await
      .map_err(|e| {
        warn!("(Complete) Error opening zip: {}", e);
        warp::reject::reject()
      })?;
    
    let folder_path = format!("{}/{}", data.libraryPath, data.system);
    let output_path = PathBuf::from(folder_path);

    let unzipped_path = unpack_zip(file, &output_path)
      .await
      .map_err(|e| {
        warn!("(Complete) Error unpacking zip: {}", e);
        warp::reject::reject()
      })?;
    
    final_path = unzipped_path.to_str().unwrap().to_string();

    info!("Unzipped file: {}", data.path);

    tokio::fs::remove_file(&data.path).await.map_err(|e| {
      warn!("(Complete) Error deleting zipped rom folder: {}", e);
      warp::reject::reject()
    })?;
    
    info!("(Complete) Deleted file: {}", data.path);
  }

  let response = warp::http::Response::builder()
    .status(200)
    .header("Content-Type", "text/plain")
    .header("Access-Control-Allow-Origin", "*")
    .body(final_path)
    .map_err(|_| warp::reject())?;

  return Ok(response);
}