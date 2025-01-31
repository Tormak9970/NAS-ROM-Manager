use std::time::Duration;

use log::{info, warn};
use reqwest::Client;
use warp::{reject::Rejection, reply::Reply};

use super::types::CoverUpload;

/// Downloads a file from a url.
async fn download_url(url: String, dest_path: &str, timeout: u64) -> Result<(), Rejection> {
  let http_client_res = Client::builder().timeout(Duration::from_secs(timeout)).build();
  let http_client: Client = http_client_res.expect("Should have been able to successfully make the reqwest client.");

  let response_res = http_client.get(url.clone()).send().await;
  
  if response_res.is_ok() {
    let response = response_res.ok().expect("Should have been able to get response from ok result.");
    let response_bytes = response.bytes().await.expect("Should have been able to await getting response bytes.");

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
  let url = data.url;

  let file_path = format!("{}/{}.{}", cover_cache_dir, rom_id, data.ext);

  download_url(url, &file_path, data.timeout)
    .await
    .map_err(|_e| {
      warp::reject::reject()
    })?;

  info!("Created file: {}", file_path);

  let response = warp::http::Response::builder()
    .status(200)
    .header("Content-Type", "text/plain")
    .header("Access-Control-Allow-Origin", "*")
    .body(format!("http://127.0.0.1:1500/rest/covers/{}.{}", rom_id, data.ext))
    .map_err(|_| warp::reject())?;

  return Ok(response);
}

/// Handles deleting a rom cover from the cache.
pub async fn delete_cover(rom_id: String, cover_cache_dir: String, cover_ext: String) -> Result<impl Reply, Rejection> {
  let file_path = format!("{}/{}.{}", cover_cache_dir, rom_id, cover_ext);

  tokio::fs::remove_file(&file_path).await.map_err(|e| {
    warn!("Error deleting file: {}", e);
    warp::reject::reject()
  })?;

  info!("Removed file: {}", file_path);

  return Ok(warp::reply::with_header("success", "Access-Control-Allow-Origin", "*"));
}