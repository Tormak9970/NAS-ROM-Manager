mod covers;
mod rom_download;
mod rom_upload;
mod types;
mod zip;

use covers::{delete_cover, upload_cover};
use rom_download::{delete_rom, download_rom, download_rom_complete, get_rom_metadata};
use rom_upload::{prepare_rom_upload, rom_upload_complete, upload_rom};
use types::{CoverUpload, ROMDownload, ROMUploadComplete, StreamStore};
use warp::{Filter, http::Method};

fn json_cover_upload() -> impl Filter<Extract = (CoverUpload,), Error = warp::Rejection> + Clone {
  warp::body::content_length_limit(50 * 1024 * 1024).and(warp::body::json())
}

fn json_body_download() -> impl Filter<Extract = (ROMDownload,), Error = warp::Rejection> + Clone {
  warp::body::content_length_limit(50 * 1024 * 1024).and(warp::body::json())
}

fn json_body_upload_complete() -> impl Filter<Extract = (ROMUploadComplete,), Error = warp::Rejection> + Clone {
  warp::body::content_length_limit(50 * 1024 * 1024).and(warp::body::json())
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
      "Content-Range",
      "Range",
      "Content-Type",
      "Cover-Extension",
      "Rom-Path",
      "Rom-Parent",
      "File-Length",
      "Upload-Id",
      "File-Size"
    ])
    .allow_methods(&[
      Method::GET,
      Method::POST,
      Method::DELETE
    ]);

  // * GET cover (rest/covers/{id}.png)
  let get_cover_route = warp::path!("rest" / "covers" / ..)
    .and(warp::fs::dir(cover_cache_dir))
    .with(&cors);

  // * POST cover (rest/covers/{id})
  let upload_cover_route = warp::path!("rest" / "covers" / String)
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


  // * GET ROM Metadata (rest/roms/download/metadata)
  let get_rom_download_metadata = warp::path!("rest" / "roms" / "download" / "metadata")
    .and(warp::get())
    .and(warp::filters::header::header("Rom-Path"))
    .and(warp::filters::header::header("Rom-Parent"))
    .and_then(get_rom_metadata)
    .with(&cors);

  // * GET ROM (rest/roms/download)
  let rom_download_route = warp::path!("rest" / "roms" / "download")
    .and(warp::get())
    .and(warp::filters::header::header("Rom-Path"))
    .and(warp::header::optional::<String>("range"))
    .and_then(download_rom)
    .with(&cors);
  
  // * POST ROM (rest/roms/download/complete)
  let rom_download_complete_route = warp::path!("rest" / "roms" / "download" / "complete")
    .and(warp::post())
    .and(json_body_download())
    .and_then(download_rom_complete)
    .with(&cors);


  let upload_store = StreamStore::new();
  let upload_store_filter = warp::any().map(move || upload_store.clone());
  
  // * POST Prepare ROM Upload (rest/roms/upload/prepare)
  let prepare_upload_rom_route = warp::path!("rest" / "roms" / "upload" / "prepare")
    .and(warp::post())
    .and(warp::filters::header::header("Rom-Path"))
    .and_then(prepare_rom_upload)
    .with(&cors);
  
  // * POST ROM (rest/roms/upload)
  let upload_rom_route = warp::path!("rest" / "roms" / "upload")
    .and(warp::post())
    .and(warp::filters::body::stream())
    .and(upload_store_filter.clone())
    .and(warp::filters::header::headers_cloned())
    .and_then(upload_rom)
    .with(&cors);

  // * POST ROM (rest/roms/upload/complete)
  let upload_rom_complete_route = warp::path!("rest" / "roms" / "upload" / "complete")
    .and(warp::post())
    .and(upload_store_filter)
    .and(json_body_upload_complete())
    .and_then(rom_upload_complete)
    .with(&cors);
  

  // * DELETE ROM (rest/roms)
  let delete_rom_route = warp::path!("rest" / "roms" / "delete")
    .and(warp::delete())
    .and(warp::filters::header::header("Rom-Path"))
    .and_then(delete_rom)
    .with(&cors);

  
  let http_routes = get_cover_route
    .or(upload_cover_route)
    .or(delete_cover_route)
    .or(get_rom_download_metadata)
    .or(rom_download_route)
    .or(rom_download_complete_route)
    .or(prepare_upload_rom_route)
    .or(upload_rom_route)
    .or(upload_rom_complete_route)
    .or(delete_rom_route);

  return http_routes;
}