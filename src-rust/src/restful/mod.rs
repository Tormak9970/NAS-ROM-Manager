mod covers;
mod roms;
mod types;

use covers::{delete_cover, upload_cover};
use roms::{delete_rom, download_rom, upload_rom};
use types::{CoverUpload, ROMDelete, ROMDownload, StreamStore};
use warp::{Filter, http::Method};

fn json_cover_upload() -> impl Filter<Extract = (CoverUpload,), Error = warp::Rejection> + Clone {
  warp::body::content_length_limit(50 * 1024 * 1024).and(warp::body::json())
}

fn json_body_download() -> impl Filter<Extract = (ROMDownload,), Error = warp::Rejection> + Clone {
  warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn json_body_delete() -> impl Filter<Extract = (ROMDelete,), Error = warp::Rejection> + Clone {
  warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

/// Gets the rest api routes.
pub fn initialize_rest_api(cover_cache_dir: String) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
  let cache_dir = cover_cache_dir.clone();
  let cache_dir_filter = warp::any().map(move || cache_dir.clone());

  let cors = warp::cors()
    .allow_any_origin()
    .allow_headers(vec![
      "Access-Control-Allow-Origin",
      "Origin",
      "Accept",
      "X-Requested-With",
      "Content-Type",
      "Cover-Extension"
    ])
    .allow_methods(&[Method::GET, Method::POST, Method::DELETE]);

  // * GET cover (rest/covers/{id}.png)
  let get_cover_route = warp::path!("rest" / "covers" / ..)
    .and(warp::fs::dir(cover_cache_dir))
    .with(&cors);

  // * POST cover (rest/covers/{id})
  let post_cover_route = warp::path!("rest" / "covers" / String)
    .and(warp::post())
    .and(cache_dir_filter.clone())
    .and(json_cover_upload())
    .and_then(upload_cover)
    .with(&cors);
  
  // * DELETE cover (rest/covers/{id}) (might need delete)
  let delete_cover_route = warp::path!("rest" / "covers" / String)
    .and(warp::delete())
    .and(cache_dir_filter.clone())
    .and(warp::filters::header::header("Cover-Extension"))
    .and_then(delete_cover)
    .with(&cors);


  // * GET ROM (rest/roms)
  let get_rom_route = warp::path!("rest" / "roms")
    .and(warp::get())
    .and(json_body_download())
    .and(warp::header::optional::<String>("range"))
    .and_then(download_rom)
    .with(&cors);

  let upload_store = StreamStore::new();
  let upload_store_filter = warp::any().map(move || upload_store.clone());
    
  // * POST ROM (rest/roms)
  let post_rom_route = warp::path!("rest" / "roms")
    .and(warp::post())
    .and(warp::filters::body::stream())
    .and(upload_store_filter)
    .and(warp::filters::header::headers_cloned())
    .and_then(upload_rom)
    .with(&cors);

  // * DELETE ROM (rest/roms)
  let delete_rom_route = warp::path!("rest" / "roms")
    .and(warp::delete())
    .and(json_body_delete())
    .and_then(delete_rom)
    .with(&cors);

  
  let http_routes = get_cover_route
    .or(post_cover_route)
    .or(delete_cover_route);
    // .or(get_rom_route)
    // .or(post_rom_route)
    // .or(delete_rom_route);

  return http_routes;
}