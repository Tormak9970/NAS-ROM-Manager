use std::{collections::HashMap, ffi::OsStr, path::PathBuf};

use log::warn;
use warp::{
  reject::Rejection,
  reply::Reply
};

use super::{types::ROMDownload, utils::download::get_file_metadata, zip::pack_zip};

/// Gets the needed metadata for downloading a rom, and zips its folder if necessary.
pub async fn rom_download_get_metadata(query_params: HashMap<String, String>) -> Result<impl Reply, Rejection> {
  if !query_params.contains_key("romPath") {
    warn!("Get ROM Metadata: Missing query param romPath");
    return Err(warp::reject::reject());
  }
  let path = query_params.get("romPath").unwrap().to_owned();

  if !query_params.contains_key("romParent") {
    warn!("Get ROM Metadata: Missing query param romParent");
    return Err(warp::reject::reject());
  }
  let parent = query_params.get("romParent").unwrap().to_owned();
  
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


  return get_file_metadata(&file_path).await;
}

/// Handles cleanup after a rom download finished.
pub async fn rom_download_complete(data: ROMDownload) -> Result<impl Reply, Rejection> {
  if data.parent != String::from("") {
    tokio::fs::remove_file(&data.path).await.map_err(|e| {
      warn!("(Complete) Error deleting zipped rom folder: {}", e);
      warp::reject::reject()
    })?;
  }

  return Ok(warp::reply::with_header("success", "Access-Control-Allow-Origin", "*"));
}

/// Handles deleting a rom.
pub async fn delete_rom(query_params: HashMap<String, String>) -> Result<impl Reply, Rejection> {
  if !query_params.contains_key("romPath") {
    warn!("Delete ROM: Missing query param romPath");
    return Err(warp::reject::reject());
  }
  let path = query_params.get("romPath").unwrap().to_owned();
  
  tokio::fs::remove_file(&path).await.map_err(|e| {
    warn!("Error deleting rom file: {}", e);
    warp::reject::reject()
  })?;

  return Ok(warp::reply::with_header("success", "Access-Control-Allow-Origin", "*"));
}