use std::{io::SeekFrom, path::{Path, PathBuf}};

use bytes::Buf;
use futures::{Stream, StreamExt};
use log::{info, warn};
use tokio::{fs::{create_dir_all, File, OpenOptions}, io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufReader}};
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};
use warp::{http::HeaderMap, reject::Rejection, reply::Reply};
use async_zip::{base::read::seek::ZipFileReader, error::ZipError};
use sanitize_filename::sanitize;

use super::types::{ROMDelete, ROMDownload, StreamProgress, StreamStore};

/// Handles downloading a rom.
pub async fn download_rom(data: ROMDownload, range_header: Option<String>) -> Result<impl Reply, Rejection> {
  let file_path = Path::new(&data.path);

  // TODO: if the download strategy requires zipping the folder structure, do it here.

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
    .body(buffer)
    .map_err(|_| warp::reject())?;

  return Ok(response);
}

/// Returns a relative path without reserved names, redundant separators, ".", or "..".
fn sanitize_file_path(path: &str) -> PathBuf {
  path.replace('\\', "/")
    .split('/')
    .map(sanitize)
    .collect()
}

/// Extracts everything from the ZIP archive to the output directory
async fn unpack_zip(archive: File, out_dir: &Path) -> Result<bool, ZipError> {
  let archive = BufReader::new(archive).compat();
  let mut reader = ZipFileReader::new(archive).await?;

  for index in 0..reader.file().entries().len() {
    let entry = reader.file().entries().get(index).unwrap();
    let path = out_dir.join(sanitize_file_path(entry.filename().as_str().unwrap()));
    let entry_is_dir = entry.dir().unwrap();

    let mut entry_reader = reader.reader_without_entry(index).await?;

    if entry_is_dir {
      if !path.exists() {
        create_dir_all(&path).await.expect("Failed to create extracted directory");
      }
    } else {
      let parent = path.parent().expect("A file entry should have parent directories");
      if !parent.is_dir() {
        create_dir_all(parent).await.expect("Failed to create parent directories");
      }

      let writer = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .await
        .expect("Failed to create extracted file");

      futures_util::io::copy(&mut entry_reader, &mut writer.compat_write())
        .await
        .expect("Failed to copy to extracted file");
    }
  }

  return Ok(true);
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
    warn!("error parsing needs-unzip: {}", e);
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
    warn!("error opening file: {}", e);
    warp::reject::reject()
  })?;

  let start_pos = content_position.parse::<u64>().map_err(|e| {
    warn!("error parsing start position: {}", e);
    warp::reject::reject()
  })?;

  file.seek(SeekFrom::Start(start_pos)).await.map_err(|e| {
    warn!("error seeking in file: {}", e);
    warp::reject::reject()
  })?;
  
  file.write_buf(&mut collected.as_slice()).await.map_err(|e| {
    warn!("error writing data to file: {}", e);
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
    return Ok("success");
  }

  // * We'll only get here once the upload is done.

  streams_store.streams.write().await.remove(&upload_id);

  if needs_unzip {
    let file = File::open(file_path.clone())
      .await
      .map_err(|e| {
        warn!("error opening zip: {}", e);
        warp::reject::reject()
      })?;
    
    let folder_path = format!("{}/{}", library_path, system);
    let output_path = PathBuf::from(folder_path);

    unpack_zip(file, &output_path)
      .await
      .map_err(|e| {
        warn!("error unpacking zip: {}", e);
        warp::reject::reject()
      })?;
    
    info!("unzipped file: {}", file_path);
  }

  return Ok("success");
}

/// Handles deleting a rom.
pub async fn delete_rom(data: ROMDelete) -> Result<impl Reply, Rejection> {
  tokio::fs::remove_file(&data.path).await.map_err(|e| {
    warn!("error deleting file: {}", e);
    warp::reject::reject()
  })?;

  return Ok("success");
}