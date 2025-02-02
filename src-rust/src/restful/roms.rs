use std::{ffi::OsStr, io::SeekFrom, path::{Path, PathBuf}};

use bytes::Buf;
use futures::{Stream, StreamExt};
use log::{info, warn};
use serde_json::{Map, Value};
use tokio::{fs::File, io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt}};
use warp::{http::HeaderMap, reject::Rejection, reply::Reply};

use crate::restful::zip::unpack_zip;

use super::{types::{ROMDownload, StreamProgress, StreamStore}, zip::pack_zip};

/// Gets the needed metadata for downloading a rom, and zips its folder if necessary.
pub async fn get_rom_metadata(path: String, parent: String) -> Result<impl Reply, Rejection> {
  let mut file_path = PathBuf::from(&path);


  if parent != String::from("") {
    let os_parent_dir = OsStr::new(&parent);

    let mut rom_dir = file_path.clone();

    for ancestor in file_path.ancestors() {
      if ancestor.parent().unwrap().file_name().unwrap() == os_parent_dir {
        rom_dir = ancestor.to_path_buf();
        break;
      }
    }

    if rom_dir == file_path {
      warn!("Error zipping rom folder: The path \"{}\" does not contain \"{}\"", &path, &parent);
      return Err(warp::reject::reject());
    }

    file_path = pack_zip(&rom_dir)
      .await
      .map_err(|e| {
        warn!("Error zipping rom folder: {}", e);
        warp::reject::reject()
      })?;
  }


  let file = File::open(&file_path).await.map_err(|_| warp::reject())?;
  let metadata = file.metadata().await.map_err(|_| warp::reject())?;
  let file_size = metadata.len();

  let mut map = Map::new();
  map.insert("size".to_string(), Value::Number(file_size.into()));
  map.insert("path".to_string(), Value::String(file_path.to_str().unwrap().to_string()));

  let response = warp::http::Response::builder()
    .status(200)
    .header("File-Length", file_size.to_string())
    .header("Content-Type", "text/plain")
    .header("Access-Control-Allow-Origin", "*")
    .body(serde_json::to_string(&map).unwrap())
    .map_err(|_| warp::reject())?;

  return Ok(response);
}

/// Handles downloading a rom.
pub async fn download_rom(path: String, range_header: Option<String>) -> Result<impl Reply, Rejection> {
  let file_path = Path::new(&path);

  let file = File::open(file_path).await.map_err(|_| warp::reject())?;
  let metadata = file.metadata().await.map_err(|_| warp::reject())?;
  let file_size = metadata.len();

  let (start, end) = match range_header {
    Some(range) => {
      if range.starts_with("bytes=") {
        let range_parts: Vec<&str> = range[6..].split('-').collect();
        let start = range_parts[0].parse::<u64>().unwrap_or(0);
        let end = range_parts[1].parse::<u64>().unwrap_or(file_size - 1);
        (start, end)
      } else {
        (0, file_size - 1) // Default to the entire file if no valid range is provided
      }
    },
    None => (0, file_size - 1), // No Range header provided
  };

  let mut file = file;
  file.seek(SeekFrom::Start(start)).await.map_err(|_| warp::reject())?;

  let mut buffer = Vec::new();
  file.take(end - start + 1).read_to_end(&mut buffer).await.map_err(|_| warp::reject())?;

  let response = warp::http::Response::builder()
    .status(206)  // Partial Content
    .header("Content-Range", format!("bytes {}-{}/{}", start, end, file_size))
    .header("Content-Length", (end - start + 1).to_string())
    .header("Content-Type", "application/octet-stream")
    .header("Access-Control-Allow-Origin", "*")
    .body(buffer)
    .map_err(|_| warp::reject())?;

  return Ok(response);
}

/// Handles cleanup after a rom download finished.
pub async fn download_rom_complete(data: ROMDownload) -> Result<impl Reply, Rejection> {
  if data.parent != String::from("") {
    tokio::fs::remove_file(&data.path).await.map_err(|e| {
      warn!("Error deleting zipped rom folder: {}", e);
      warp::reject::reject()
    })?;
  }

  return Ok(warp::reply::with_header("success", "Access-Control-Allow-Origin", "*"));
}

/// Handles uploading a rom.
pub async fn upload_rom(
  mut body: impl Stream<Item = Result<impl Buf, warp::Error>> + Unpin + Send + Sync,
  streams_store: StreamStore,
  headers: HeaderMap
) -> Result<impl Reply, Rejection> {
  // TODO: reject if Content-Type is wrong

  let content_length = headers.get("content-length").unwrap().to_str().unwrap().to_string();
  let full_size = content_length.parse::<u64>().map_err(|e| {
    warn!("error parsing Content-Length: {}", e);
    warp::reject::reject()
  })?;

  let content_position = headers.get("content-position").unwrap().to_str().unwrap().to_string();

  let upload_id = headers.get("upload-id").unwrap().to_str().unwrap().to_string();
  let filename = headers.get("file-name").unwrap().to_str().unwrap().to_string();
  let system = headers.get("game-system").unwrap().to_str().unwrap().to_string();
  let library_path = headers.get("library-path").unwrap().to_str().unwrap().to_string();
  let needs_unzip = headers.get("needs-unzip").unwrap().to_str().unwrap().to_string().parse::<bool>().map_err(|e| {
    warn!("Error parsing needs-unzip: {}", e);
    warp::reject::reject()
  })?;


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
  let file_path = format!("{}/{}/{}", library_path, system, filename);

  let mut file = tokio::fs::File::open(&file_path).await.map_err(|e| {
    warn!("Error opening file: {}", e);
    warp::reject::reject()
  })?;

  let start_pos = content_position.parse::<u64>().map_err(|e| {
    warn!("Error parsing start position: {}", e);
    warp::reject::reject()
  })?;

  file.seek(SeekFrom::Start(start_pos)).await.map_err(|e| {
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
      totalSize: full_size
    });
  }
  
  let mut stream_progress = streams_store.streams.read().await.get(&upload_id).unwrap().clone();
  stream_progress.currentSize += chunk_size;

  if stream_progress.currentSize != stream_progress.totalSize {
    streams_store.streams.write().await.insert(upload_id, stream_progress);
    return Ok(warp::reply::with_header("success", "Access-Control-Allow-Origin", "*"));
  }

  // * We'll only get here once the upload is done.

  streams_store.streams.write().await.remove(&upload_id);

  if needs_unzip {
    let file = File::open(file_path.clone())
      .await
      .map_err(|e| {
        warn!("Error opening zip: {}", e);
        warp::reject::reject()
      })?;
    
    let folder_path = format!("{}/{}", library_path, system);
    let output_path = PathBuf::from(folder_path);

    unpack_zip(file, &output_path)
      .await
      .map_err(|e| {
        warn!("Error unpacking zip: {}", e);
        warp::reject::reject()
      })?;
    
    info!("Unzipped file: {}", file_path);
  }

  return Ok(warp::reply::with_header("success", "Access-Control-Allow-Origin", "*"));
}

/// Handles deleting a rom.
pub async fn delete_rom(rom_path: String) -> Result<impl Reply, Rejection> {
  tokio::fs::remove_file(&rom_path).await.map_err(|e| {
    warn!("Error deleting rom file: {}", e);
    warp::reject::reject()
  })?;

  return Ok(warp::reply::with_header("success", "Access-Control-Allow-Origin", "*"));
}