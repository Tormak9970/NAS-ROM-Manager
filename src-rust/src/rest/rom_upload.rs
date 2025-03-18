use std::{collections::HashMap, path::PathBuf};

use bytes::Buf;
use futures::{Stream, StreamExt};
use log::{info, warn};
use tokio::{fs::{File, OpenOptions}, io::AsyncWriteExt};
use warp::{http::HeaderMap, reject::Rejection, reply::Reply};
use chrono::Utc;

use crate::rest::zip::unpack_zip;

use super::types::{ROMUploadComplete, StreamProgress, StreamStore};

/// Prepares for the upload of a rom
pub async fn prepare_rom_upload(query_params: HashMap<String, String>) -> Result<impl Reply, Rejection> {
  if !query_params.contains_key("romPath") {
    warn!("Prepare ROM upload: Missing query param romPath");
    return Err(warp::reject::reject());
  }
  let path = PathBuf::from(query_params.get("romPath").unwrap().to_owned());
  let parent_dir = path.parent();

  if parent_dir.is_some() {
    tokio::fs::create_dir_all(parent_dir.unwrap()).await.map_err(|e| {
      warn!("Error creating parent directory: {}", e);
      warp::reject::reject()
    })?;
  }
  
  tokio::fs::File::create(&path).await.map_err(|e| {
    warn!("Error creating file: {}", e);
    warp::reject::reject()
  })?;

  return Ok(warp::reply::with_header("success", "Access-Control-Allow-Origin", "*"));
}

/// Handles uploading a rom.
pub async fn upload_rom(
  mut body: impl Stream<Item = Result<impl Buf, warp::Error>> + Unpin + Send + Sync,
  streams_store: StreamStore,
  query_params: HashMap<String, String>,
  headers: HeaderMap
) -> Result<impl Reply, Rejection> {
  let content_type = headers.get("content-type").unwrap().to_str().unwrap();
  if content_type != "application/octet-stream" {
    return Err(warp::reject::reject());
  }

  let file_size = headers.get("file-size").unwrap().to_str().unwrap().to_string().parse::<u64>().map_err(|e| {
    warn!("error parsing Content-Length: {}", e);
    warp::reject::reject()
  })?;

  let range_header = headers.get("range").unwrap().to_str();
  let (start, end) = match range_header {
    Ok(range) => {
      if range.starts_with("bytes=") {
        let range_parts: Vec<&str> = range[6..].split('-').collect();
        let start = range_parts[0].parse::<u64>().unwrap_or(0);
        let end = range_parts[1].parse::<u64>().unwrap_or(file_size - 1);
        (start, end)
      } else {
        (0, file_size - 1) // Default to the entire file if no valid range is provided
      }
    },
    Err(_) => (0, file_size - 1), // No Range header provided
  };
  let chunk_size = end - start;

  let upload_id = headers.get("upload-id").unwrap().to_str().unwrap().to_string();

  if !query_params.contains_key("romPath") {
    warn!("Upload ROM: Missing query param romPath");
    return Err(warp::reject::reject());
  }
  let file_path = query_params.get("romPath").unwrap().to_owned();
  
  let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .open(&file_path)
    .await.map_err(|e| {
      warn!("Error opening file: {}", e);
      warp::reject::reject()
    })?;

  while let Some(buf) = body.next().await {
    let mut buf = buf.unwrap();
    
    file.write_buf(&mut buf).await.map_err(|e| {
      warn!("Error writing data to file: {}", e);
      warp::reject::reject()
    })?;
  }

  file.flush().await.map_err(|e| {
    warn!("Error flushing data to file: {}", e);
    warp::reject::reject()
  })?;

  
  if !streams_store.streams.read().await.contains_key(&upload_id) {
    streams_store.streams.write().await.insert(upload_id.clone(), StreamProgress {
      id: upload_id.clone(),
      path: file_path.clone(),
      currentSize: 0,
      totalSize: file_size,
      lastChunkTime: Utc::now().timestamp(),
    });
  }
  
  let mut stream_progress = streams_store.streams.read().await.get(&upload_id).unwrap().clone();
  stream_progress.currentSize += chunk_size;
  stream_progress.lastChunkTime = Utc::now().timestamp();

  return Ok(warp::reply::with_header("success", "Access-Control-Allow-Origin", "*"));
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

/// Completes the ROM upload
pub async fn rom_upload_cancel(streams_store: StreamStore, upload_id: String) -> Result<impl Reply, Rejection> {
  let stream_res = streams_store.streams.write().await.remove(&upload_id);

  if stream_res.is_none() {
    warn!("(Cancel) Error getting stream for upload: {}", upload_id);
    return Err(warp::reject::reject());
  }
  let stream = stream_res.unwrap();
  
  tokio::fs::remove_file(&stream.path).await.map_err(|e| {
    warn!("(Cancel) Error deleting canceled upload: {}", e);
    warp::reject::reject()
  })?;
  
  info!("(Cancel) Deleted file: {}", stream.path);

  let response = warp::http::Response::builder()
    .status(200)
    .header("Content-Type", "text/plain")
    .header("Access-Control-Allow-Origin", "*")
    .body("true".to_string())
    .map_err(|_| warp::reject())?;

  return Ok(response);
}