use std::{io::SeekFrom, path::PathBuf};

use bytes::Buf;
use futures::{Stream, StreamExt};
use log::{info, warn};
use tokio::{fs::File, io::{AsyncSeekExt, AsyncWriteExt}};
use warp::{http::HeaderMap, reject::Rejection, reply::Reply};

use crate::restful::zip::unpack_zip;

use super::types::{ROMUploadComplete, StreamProgress, StreamStore};

/// Prepares for the upload of a rom
pub async fn prepare_rom_upload(path: String) -> Result<impl Reply, Rejection> {
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
  headers: HeaderMap
) -> Result<impl Reply, Rejection> {
  let content_type = headers.get("content-type").unwrap().to_str().unwrap();
  if content_type != "application/octet-stream" {
    return Err(warp::reject::reject());
  }

  let content_length = headers.get("content-length").unwrap().to_str().unwrap().to_string();
  let file_size = content_length.parse::<u64>().map_err(|e| {
    warn!("error parsing Content-Length: {}", e);
    warp::reject::reject()
  })?;

  let range_header = headers.get("range").unwrap().to_str();
  let (start, _end) = match range_header {
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

  let upload_id = headers.get("upload-id").unwrap().to_str().unwrap().to_string();
  let file_path = headers.get("rom-path").unwrap().to_str().unwrap().to_string();


  let mut collected: Vec<u8> = vec![];

  while let Some(buf) = body.next().await {
    let mut buf = buf.unwrap();

    while buf.remaining() > 0 {
      let chunk = buf.chunk();
      let chunk_len = chunk.len();

      collected.extend_from_slice(chunk);
      buf.advance(chunk_len);
    }
  }


  let chunk_size = collected.len() as u64;

  let mut file = tokio::fs::File::open(&file_path).await.map_err(|e| {
    warn!("Error opening file: {}", e);
    warp::reject::reject()
  })?;

  file.seek(SeekFrom::Start(start)).await.map_err(|e| {
    warn!("Error seeking in file: {}", e);
    warp::reject::reject()
  })?;
  
  file.write_buf(&mut collected.as_slice()).await.map_err(|e| {
    warn!("Error writing data to file: {}", e);
    warp::reject::reject()
  })?;
  
  if !streams_store.streams.read().await.contains_key(&upload_id) {
    streams_store.streams.write().await.insert(upload_id.clone(), StreamProgress {
      currentSize: 0,
      totalSize: file_size
    });
  }
  
  let mut stream_progress = streams_store.streams.read().await.get(&upload_id).unwrap().clone();
  stream_progress.currentSize += chunk_size;

  return Ok(warp::reply::with_header("success", "Access-Control-Allow-Origin", "*"));
}

/// Completes the ROM upload
pub async fn rom_upload_complete(streams_store: StreamStore, data: ROMUploadComplete) -> Result<impl Reply, Rejection> {
  streams_store.streams.write().await.remove(&data.uploadId);
  
  if data.unzip {
    let file = File::open(&data.path)
      .await
      .map_err(|e| {
        warn!("Error opening zip: {}", e);
        warp::reject::reject()
      })?;
    
    let folder_path = format!("{}/{}", data.libraryPath, data.system);
    let output_path = PathBuf::from(folder_path);

    unpack_zip(file, &output_path)
      .await
      .map_err(|e| {
        warn!("Error unpacking zip: {}", e);
        warp::reject::reject()
      })?;
    
    info!("Unzipped file: {}", data.path);

    tokio::fs::remove_file(&data.path).await.map_err(|e| {
      warn!("Error deleting zipped rom folder: {}", e);
      warp::reject::reject()
    })?;
    
    info!("Deleted file: {}", data.path);
  }

  return Ok(warp::reply::with_header("success", "Access-Control-Allow-Origin", "*"));
}