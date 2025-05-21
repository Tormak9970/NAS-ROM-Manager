use std::{fs, time::Duration};

use log::{info, warn};
use reqwest::Client;
use warp::{reject::Rejection, reply::Reply};

use super::types::{HeroUpload, CapsuleUpload};

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

/// Handles uploading a rom capsule to the cache.
pub async fn upload_capsule(rom_id: String, capsule_cache_dir: String, data: CapsuleUpload) -> Result<impl Reply, Rejection> {
  let capsules_path = format!("{}/full", capsule_cache_dir);
  let capsule_file_path = format!("{}/full/{}.{}", capsule_cache_dir, rom_id, data.fullCapsuleExt);

  for entry in fs::read_dir(capsules_path).expect("capsules directory should have existed") {
    if entry.is_err() {
      continue;
    }
    let dir_entry = entry.unwrap();

    if dir_entry.file_name().eq_ignore_ascii_case(&rom_id) {
      let path = dir_entry.path();
      let path_str = path.to_str().expect("Failed to convert existing capsule path to string.");
      let delete_res = fs::remove_file(&path);

      if delete_res.is_err() {
        let err = delete_res.err().unwrap();
        warn!("Error deleting existing capsule file \"{}\": {}", path_str, err);
      }
    }
  }

  download_url(data.fullCapsuleUrl, &capsule_file_path, data.timeout)
    .await
    .map_err(|_e| {
      warp::reject::reject()
    })?;

  info!("Created full capsule file: {}", capsule_file_path);

  let thumb_file_path = format!("{}/thumb/{}.{}", capsule_cache_dir, rom_id, data.thumbCapsuleExt);

  download_url(data.thumbCapsuleUrl, &thumb_file_path, data.timeout)
    .await
    .map_err(|_e| {
      warp::reject::reject()
    })?;

  info!("Created thumb capsule file: {}", thumb_file_path);

  let response = warp::http::Response::builder()
    .status(200)
    .header("Content-Type", "text/plain")
    .header("Access-Control-Allow-Origin", "*")
    .body(format!("{}.{},{}.{}", rom_id, data.thumbCapsuleExt, rom_id, data.fullCapsuleExt))
    .map_err(|_| warp::reject())?;

  return Ok(response);
}

/// Handles deleting a rom capsule from the cache.
pub async fn delete_capsule(rom_id: String, capsule_cache_dir: String, capsule_ext: String, thumb_ext: String) -> Result<impl Reply, Rejection> {
  let capsule_file_path = format!("{}/full/{}.{}", capsule_cache_dir, rom_id, capsule_ext);

  tokio::fs::remove_file(&capsule_file_path).await.map_err(|e| {
    warn!("Error deleting full capsule file: {}", e);
    warp::reject::reject()
  })?;

  info!("Removed full capsule file: {}", capsule_file_path);


  let thumb_file_path = format!("{}/thumb/{}.{}", capsule_cache_dir, rom_id, thumb_ext);

  tokio::fs::remove_file(&thumb_file_path).await.map_err(|e| {
    warn!("Error deleting thumb capsule file: {}", e);
    warp::reject::reject()
  })?;

  info!("Removed thumb capsule file: {}", thumb_file_path);


  return Ok(warp::reply::with_header("success", "Access-Control-Allow-Origin", "*"));
}

/// Handles uploading a rom hero to the cache.
pub async fn upload_hero(rom_id: String, capsule_cache_dir: String, data: HeroUpload) -> Result<impl Reply, Rejection> {
  let capsules_path = format!("{}/hero", capsule_cache_dir);
  let hero_file_path = format!("{}/hero/{}.{}", capsule_cache_dir, rom_id, data.heroExt);

  for entry in fs::read_dir(capsules_path).expect("capsules directory should have existed") {
    if entry.is_err() {
      continue;
    }
    let dir_entry = entry.unwrap();

    if dir_entry.file_name().eq_ignore_ascii_case(&rom_id) {
      let path = dir_entry.path();
      let path_str = path.to_str().expect("Failed to convert existing capsule path to string.");
      let delete_res = fs::remove_file(&path);

      if delete_res.is_err() {
        let err = delete_res.err().unwrap();
        warn!("Error deleting existing hero file \"{}\": {}", path_str, err);
      }
    }
  }

  download_url(data.heroUrl, &hero_file_path, data.timeout)
    .await
    .map_err(|_e| {
      warp::reject::reject()
    })?;

  info!("Created hero file: {}", hero_file_path);

  let response = warp::http::Response::builder()
    .status(200)
    .header("Content-Type", "text/plain")
    .header("Access-Control-Allow-Origin", "*")
    .body(format!("{}.{}", rom_id, data.heroExt))
    .map_err(|_| warp::reject())?;

  return Ok(response);
}

/// Handles deleting a rom hero from the cache.
pub async fn delete_hero(rom_id: String, capsule_cache_dir: String, hero_ext: String) -> Result<impl Reply, Rejection> {
  let hero_file_path = format!("{}/hero/{}.{}", capsule_cache_dir, rom_id, hero_ext);

  tokio::fs::remove_file(&hero_file_path).await.map_err(|e| {
    warn!("Error deleting hero file: {}", e);
    warp::reject::reject()
  })?;

  info!("Removed hero file: {}", hero_file_path);


  return Ok(warp::reply::with_header("success", "Access-Control-Allow-Origin", "*"));
}