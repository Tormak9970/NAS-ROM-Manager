use std::path::{Path, PathBuf};

use bytes::Buf;
use futures::{Stream, StreamExt};
use log::{info, warn};
use tokio::{fs::{create_dir_all, File, OpenOptions}, io::{AsyncSeekExt, AsyncWriteExt, BufReader}};
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};
use warp::{http::{HeaderMap, Response}, reject::Rejection, reply::Reply};
use async_zip::{base::read::seek::ZipFileReader, error::ZipError};
use sanitize_filename::sanitize;

use super::types::{ROMDelete, ROMDownload, StreamProgress, StreamStore};

/// Handles downloading a rom.
pub async fn download_rom(data: ROMDownload, streams: StreamStore, headers: HeaderMap) -> Result<impl Reply, Rejection> {
  // let mut parts = form.into_stream();

  // while let Some(Ok(p)) = parts.next().await {
  //   if p.name() == "file" {
  //     let content_type = p.content_type();
  //     let file_ending;

  //     match content_type {
  //       Some(file_type) => match file_type {
  //         "image/png" => {
  //           file_ending = "png";
  //         }
  //         v => {
  //           warn!("invalid file type found: {}", v);
  //           return Err(warp::reject::reject());
  //         }
  //       },
  //       None => {
  //         warn!("file type could not be determined");
  //         return Err(warp::reject::reject());
  //       }
  //     }

  //     let value = p
  //       .stream()
  //       .try_fold(Vec::new(), |mut vec, data| {
  //         vec.put(data);
  //         async move { Ok(vec) }
  //       })
  //       .await
  //       .map_err(|e| {
  //         warn!("reading file error: {}", e);
  //         warp::reject::reject()
  //       })?;

  //     let file_name = format!("{}/{}.{}", cover_cache_dir, rom_id, file_ending);

  //     tokio::fs::write(&file_name, value).await.map_err(|e| {
  //       warn!("error writing file: {}", e);
  //       warp::reject::reject()
  //     })?;

  //     info!("created file: {}", file_name);
  //   }
  // }

  let response = Response::builder()
    .header("content-length", "some-value")
    .header("content-type", "some-value")
    .body("and a custom body");

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
  let content_length = headers.get("content-length").unwrap().to_str().unwrap().to_string();
  let full_size = content_length.parse::<u64>().map_err(|e| {
    warn!("error parsing content-length: {}", e);
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

  file.seek(std::io::SeekFrom::Start(start_pos)).await.map_err(|e| {
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