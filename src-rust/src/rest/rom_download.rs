use std::{collections::HashMap, ffi::OsStr, io::SeekFrom, path::{Path, PathBuf}};

use log::warn;
use serde_json::{Map, Value};
use tokio::{fs::File, io::{AsyncReadExt, AsyncSeekExt, BufReader}};
use tokio_util::codec::{BytesCodec, FramedRead};
use warp::{
  reject::Rejection,
  reply::Reply,
  http::{StatusCode, Response},
  hyper::Body
};

use super::{types::ROMDownload, zip::pack_zip};

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
pub async fn rom_download(query_params: HashMap<String, String>, range_header: Option<String>) -> Result<impl Reply, Rejection> {
  if !query_params.contains_key("romPath") {
    warn!("Download ROM: Missing query param romPath");
    // return Ok(Response::builder().status(404).body("File not found").unwrap());
  }
  let path = query_params.get("romPath").unwrap().to_owned();


  let file_path = Path::new(&path);

  let filename = file_path.file_name().unwrap().to_str().unwrap();

  let file_res = File::open(file_path).await;
  if file_res.is_err() {
    // return Ok(Response::builder().status(404).body("File not found").unwrap());
  }
  let file = file_res.unwrap();

  let metadata_res = file.metadata().await;
  if metadata_res.is_err() {
    // return Ok(Response::builder().status(404).body("File not found").unwrap());
  }
  let metadata = metadata_res.unwrap();
  let file_size = metadata.len();

  // let (start, end) = match range_header {
  //   Some(range) => {
  //     if range.starts_with("bytes=") {
  //       let range_parts: Vec<&str> = range[6..].split('-').collect();
  //       let start = range_parts[0].parse::<u64>().unwrap_or(0);
  //       let end = range_parts[1].parse::<u64>().unwrap_or(file_size - 1);
  //       (start, end)
  //     } else {
  //       (0, file_size - 1) // Default to the entire file if no valid range is provided
  //     }
  //   },
  //   None => (0, file_size - 1), // No Range header provided
  // };

  // let mut file = file;
  // file.seek(SeekFrom::Start(start)).await.map_err(|_| warp::reject())?;

  // let mut buffer = Vec::new();
  // file.take(end - start + 1).read_to_end(&mut buffer).await.map_err(|_| warp::reject())?;

  let reader = BufReader::new(file);
    
  // Wrap the reader in FramedRead to convert it to a stream
  let framed_reader = FramedRead::new(reader, BytesCodec::new());

  // Return the response with the body as the file stream
  let response: Response<_> = Response::builder()
    .header("Content-Length", file_size.to_string())
    .header("Content-Type", "application/octet-stream")
    .header("Content-Disposition", format!("attachement; filename = \"{}\"", filename))
    .header("Access-Control-Allow-Origin", "*")
    .body(Body::wrap_stream(framed_reader))
    .map_err(|_| warp::reject())?;

  // let response = warp::http::Response::builder()
  //   .status(206)  // Partial Content
  //   .header("Content-Range", format!("bytes {}-{}/{}", start, end, file_size))
  //   .header("Content-Length", (end - start + 1).to_string())
  //   .header("Content-Type", "application/octet-stream")
  //   .header("Content-Disposition", format!("attachement; filename = \"{}\"", filename))
  //   .header("Access-Control-Allow-Origin", "*")
  //   .body(buffer)
  //   .map_err(|_| warp::reject())?;

  return Ok(response);
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