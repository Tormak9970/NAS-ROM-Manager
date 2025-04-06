use std::{fs, time::Duration};

use log::{info, warn};
use reqwest::Client;
use warp::{reject::Rejection, reply::Reply};

use super::types::CoverUpload;

/// Downloads a file from a url.
async fn download_url(url: String, dest_path: &str, timeout: u64) -> Result<(), Rejection> {
  let http_client_res = Client::builder().timeout(Duration::from_secs(timeout)).build();
  let http_client: Client = http_client_res.expect("Failed to make the reqwest client.");

  let response_res = http_client.get(url.clone()).send().await;
  
  if response_res.is_ok() {
    let response = response_res.ok().expect("Failed to get response from ok result.");
    let response_bytes = response.bytes().await.expect("Failed to await getting response bytes.");

    let write_res = tokio::fs::write(&dest_path, &response_bytes).await;

    if write_res.is_ok() {
      info!("Download of {} finished.", url.clone());
      return Ok(());
    } else {
      let err = write_res.err().expect("Request failed, error should have existed.");
      warn!("Download of {} failed with {}.", url.clone(), err.to_string());
      return Err(warp::reject::reject());
    }
  } else {
    let err = response_res.err().expect("Request failed, error should have existed.");
    warn!("Download of {} failed with {}.", url.clone(), err.to_string());
    return Ok(());
  }
}

/// Handles uploading a rom cover to the cache.
pub async fn upload_cover(rom_id: String, cover_cache_dir: String, data: CoverUpload) -> Result<impl Reply, Rejection> {
  let covers_path = format!("{}/full", cover_cache_dir);
  let cover_file_path = format!("{}/full/{}.{}", cover_cache_dir, rom_id, data.coverExt);

  for entry in fs::read_dir(covers_path).expect("Covers directory should have existed") {
    if entry.is_err() {
      continue;
    }
    let dir_entry = entry.unwrap();

    if dir_entry.file_name().eq_ignore_ascii_case(&rom_id) {
      let path = dir_entry.path();
      let path_str = path.to_str().expect("Failed to convert existing cover path to string.");
      let delete_res = fs::remove_file(&path);

      if delete_res.is_err() {
        let err = delete_res.err().unwrap();
        warn!("Error deleting existing cover file \"{}\": {}", path_str, err);
      }
    }
  }

  download_url(data.coverUrl, &cover_file_path, data.timeout)
    .await
    .map_err(|_e| {
      warp::reject::reject()
    })?;

  info!("Created full cover file: {}", cover_file_path);

  let thumb_file_path = format!("{}/thumb/{}.{}", cover_cache_dir, rom_id, data.thumbExt);

  download_url(data.thumbUrl, &thumb_file_path, data.timeout)
    .await
    .map_err(|_e| {
      warp::reject::reject()
    })?;

  info!("Created thumb cover file: {}", thumb_file_path);

  let response = warp::http::Response::builder()
    .status(200)
    .header("Content-Type", "text/plain")
    .header("Access-Control-Allow-Origin", "*")
    .body(format!("{}.{},{}.{}", rom_id, data.thumbExt, rom_id, data.coverExt))
    .map_err(|_| warp::reject())?;

  return Ok(response);
}

/// Handles deleting a rom cover from the cache.
pub async fn delete_cover(rom_id: String, cover_cache_dir: String, cover_ext: String, thumb_ext: String) -> Result<impl Reply, Rejection> {
  let cover_file_path = format!("{}/full/{}.{}", cover_cache_dir, rom_id, cover_ext);

  tokio::fs::remove_file(&cover_file_path).await.map_err(|e| {
    warn!("Error deleting full cover file: {}", e);
    warp::reject::reject()
  })?;

  info!("Removed full cover file: {}", cover_file_path);


  let thumb_file_path = format!("{}/thumb/{}.{}", cover_cache_dir, rom_id, thumb_ext);

  tokio::fs::remove_file(&thumb_file_path).await.map_err(|e| {
    warn!("Error deleting thumb cover file: {}", e);
    warp::reject::reject()
  })?;

  info!("Removed thumb cover file: {}", thumb_file_path);


  return Ok(warp::reply::with_header("success", "Access-Control-Allow-Origin", "*"));
}