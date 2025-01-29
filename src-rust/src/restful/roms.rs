use std::path::{Path, PathBuf};

use bytes::BufMut;
use futures::{StreamExt, TryStreamExt};
use log::{info, warn};
use tokio::{fs::{create_dir_all, File, OpenOptions}, io::BufReader};
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};
use warp::{filters::multipart::FormData, reject::Rejection, reply::Reply};
use async_zip::{base::read::seek::ZipFileReader, error::ZipError};
use sanitize_filename::sanitize;

use super::types::{ROMDelete, ROMDownload, ROMUpload};

/// Handles downloading a rom.
pub async fn download_rom(data: ROMDownload) -> Result<impl Reply, Rejection> {
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

  return Ok("success");
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
pub async fn upload_rom(data: ROMUpload, form: FormData) -> Result<impl Reply, Rejection> {
  let mut parts = form.into_stream();

  while let Some(Ok(p)) = parts.next().await {
    if p.name() == "file" {
      let filename = p.filename().unwrap().to_string();

      let value = p
        .stream()
        .try_fold(Vec::new(), |mut vec, data| {
          vec.put(data);
          async move { Ok(vec) }
        })
        .await
        .map_err(|e| {
          warn!("error reading file: {}", e);
          warp::reject::reject()
        })?;

      let file_name = format!("{}/{}/{}", data.library, data.system, filename);
      
      tokio::fs::write(&file_name, value).await.map_err(|e| {
        warn!("error writing file: {}", e);
        warp::reject::reject()
      })?;
      
      info!("created file: {}", file_name);

      if data.needsUnzip {
        let file = File::open(file_name.clone())
          .await
          .map_err(|e| {
            warn!("error opening zip: {}", e);
            warp::reject::reject()
          })?;
        
        let folder_path = format!("{}/{}", data.library, data.system);
        let output_path = PathBuf::from(folder_path);

        unpack_zip(file, &output_path)
          .await
          .map_err(|e| {
            warn!("error unpacking zip: {}", e);
            warp::reject::reject()
          })?;
        
        info!("unzipped file: {}", file_name);
      }
    }
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