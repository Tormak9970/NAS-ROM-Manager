mod covers;
mod roms;
mod types;

use std::{convert::Infallible, env::var};
use covers::{delete_cover, upload_cover};
use roms::{delete_rom, upload_rom};
use types::{ROMDelete, ROMDownload, ROMUpload};
use warp::{http::StatusCode, reject::Rejection, reply::Reply, Filter};

async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
  let (code, message) = if err.is_not_found() {
    (StatusCode::NOT_FOUND, "Not Found".to_string())
  } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
    (StatusCode::BAD_REQUEST, "Payload too large".to_string())
  } else {
    eprintln!("unhandled error: {:?}", err);
    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string(),)
  };

  Ok(warp::reply::with_status(message, code))
}

fn json_body_download() -> impl Filter<Extract = (ROMDownload,), Error = warp::Rejection> + Clone {
  warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn json_body_upload() -> impl Filter<Extract = (ROMUpload,), Error = warp::Rejection> + Clone {
  warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn json_body_delete() -> impl Filter<Extract = (ROMDelete,), Error = warp::Rejection> + Clone {
  warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

/// Gets the rest api routes.
pub fn initialize_rest_api() -> impl Filter + Clone {
  let cover_cache_dir = var("NRM_COVER_CACHE_DIR").ok().unwrap();
  let cache_dir = cover_cache_dir.clone();
  let cache_dir_filter = warp::any().map(move || cache_dir.clone());


  // * GET cover (rest/covers/{id}.png)
  let get_cover_route = warp::path!("rest" / "covers")
    .and(warp::get())
    .and(warp::fs::dir(cover_cache_dir.clone()));

  // * POST cover (rest/covers/{id})
  let post_cover_route = warp::path!("rest" / "covers" / String)
    .and(warp::post())
    .and(cache_dir_filter.clone())
    .and(warp::multipart::form().max_length(100_000_000))
    .and_then(upload_cover);
  
  // * DELETE cover (rest/covers/{id}) (might need delete)
  let delete_cover_route = warp::path!("rest" / "covers" / String)
    .and(warp::delete())
    .and(cache_dir_filter.clone())
    .and_then(delete_cover);


  // TODO: GET ROM (rest/roms)

  // * POST ROM (rest/roms)
  let post_rom_route = warp::path!("rest" / "roms")
    .and(warp::post())
    .and(json_body_upload())
    .and(warp::multipart::form().max_length(20_000_000_000))
    .and_then(upload_rom);

  // * DELETE ROM (rest/roms)
  let delete_rom_route = warp::path!("rest" / "roms")
    .and(warp::delete())
    .and(json_body_delete())
    .and_then(delete_rom);

  
  let http_routes = get_cover_route
    .or(post_cover_route)
    .or(delete_cover_route)
    .or(post_rom_route)
    .or(delete_rom_route)
    .recover(handle_rejection);

  return http_routes;
}