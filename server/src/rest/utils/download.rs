use std::{collections::HashMap, path::Path};

use log::warn;
use serde_json::{Map, Value};
use tokio::{fs::File, io::BufReader};
use tokio_util::codec::{BytesCodec, FramedRead};
use warp::{
  reject::Rejection,
  reply::Reply,
  http::{StatusCode, Response},
  hyper::Body
};

/// Gets the needed metadata for downloading a file.
pub async fn get_file_metadata(file_path: &Path) -> Result<impl Reply, Rejection> {
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

/// Handles downloading a file.
pub async fn download_file(query_params: HashMap<String, String>) -> Result<impl Reply, Rejection> {
  if !query_params.contains_key("filePath") {
    warn!("Download File: Missing query param filePath");
    return Ok(Response::builder().status(StatusCode::NOT_FOUND).body(Body::empty()).unwrap());
  }
  let path = query_params.get("filePath").unwrap().to_owned();


  let file_path = Path::new(&path);

  let filename = file_path.file_name().unwrap().to_str().unwrap();

  let file_res = File::open(file_path).await;
  if file_res.is_err() {
    return Ok(Response::builder().status(StatusCode::NOT_FOUND).body(Body::empty()).unwrap());
  }
  let file = file_res.unwrap();

  let metadata_res = file.metadata().await;
  if metadata_res.is_err() {
    return Ok(Response::builder().status(StatusCode::NOT_FOUND).body(Body::empty()).unwrap());
  }
  let metadata = metadata_res.unwrap();
  let file_size = metadata.len();

  let reader = BufReader::new(file);
  let framed_reader = FramedRead::new(reader, BytesCodec::new());

  let response = Response::builder()
    .status(StatusCode::OK)
    .header("Content-Length", file_size.to_string())
    .header("Content-Type", "application/octet-stream")
    .header("Content-Disposition", format!("attachement; filename = \"{}\"", filename))
    .header("Access-Control-Allow-Origin", "*")
    .body(Body::wrap_stream(framed_reader))
    .map_err(|_| warp::reject())?;

  return Ok(response);
}