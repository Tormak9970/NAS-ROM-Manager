use std::{collections::HashMap, path::{PathBuf}};

use bytes::Buf;
use futures::{Stream, StreamExt};
use log::{info, warn};
use tokio::{fs::OpenOptions, io::AsyncWriteExt};
use warp::{http::HeaderMap, reject::Rejection, reply::Reply};
use chrono::Utc;

use crate::rest::types::{StreamProgress, StreamStore};

/// Completes a file upload
pub async fn prepare_file_upload(query_params: HashMap<String, String>) -> Result<impl Reply, Rejection> {
  if !query_params.contains_key("filePath") {
    warn!("Prepare Upload: Missing query param filePath");
    return Err(warp::reject::reject());
  }

  let file_path = PathBuf::from(query_params.get("filePath").unwrap().to_owned());

  let exists_res = tokio::fs::try_exists(&file_path).await;

  let exists = exists_res.is_err() || exists_res.ok().unwrap();

  if exists {
    let response = warp::http::Response::builder()
      .status(200)
      .header("Content-Type", "text/plain")
      .header("Access-Control-Allow-Origin", "*")
      .body(format!("{}", exists))
      .map_err(|_| warp::reject())?;

    return Ok(response);
  }
  
  let parent_dir = file_path.parent();

  if parent_dir.is_some() {
    tokio::fs::create_dir_all(parent_dir.unwrap()).await.map_err(|e| {
      warn!("Error creating parent directory: {}", e);
      warp::reject::reject()
    })?;
  }
  
  tokio::fs::File::create(&file_path).await.map_err(|e| {
    warn!("Error creating file: {}", e);
    warp::reject::reject()
  })?;

  let response = warp::http::Response::builder()
    .status(200)
    .header("Content-Type", "text/plain")
    .header("Access-Control-Allow-Origin", "*")
    .body(format!("{}", exists))
    .map_err(|_| warp::reject())?;

  return Ok(response);
}

/// Handles an upload stream
pub async fn upload_file(
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

  if !query_params.contains_key("filePath") {
    warn!("Upload File: Missing query param filePath");
    return Err(warp::reject::reject());
  }
  let file_path = query_params.get("filePath").unwrap().to_owned();
  
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

/// Cancels a file upload
pub async fn upload_cancel(streams_store: StreamStore, upload_id: String) -> Result<impl Reply, Rejection> {
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