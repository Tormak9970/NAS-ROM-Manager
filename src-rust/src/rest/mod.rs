mod covers;
mod rom_download;
mod rom_upload;
mod types;
mod zip;
mod sgdb;

use std::{collections::HashMap, fs::remove_file, str::FromStr, thread};

use chrono::Utc;
use covers::{delete_cover, upload_cover};
use cron::Schedule;
use log::{info, warn};
use rom_download::{delete_rom, download_rom, download_rom_complete, get_rom_metadata};
use rom_upload::{prepare_rom_upload, rom_upload_complete, upload_rom};
use sgdb::{init_sgdb_client, sgdb_get_grids_by_id, sgdb_search_game};
use types::{CoverUpload, ROMDownload, ROMUploadComplete, SGDBClientStore, StreamStore};
use warp::{http::Method, Filter};

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
pub fn initialize_rest_api(cover_cache_dir: String, cleanup_schedule: String) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
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
      "Thumb-Extension",
      "File-Length",
      "Upload-Id",
      "File-Size",
      "SGDB-Game-Id",
      "SGDB-Results-Page",
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
    .and(warp::filters::header::header("Thumb-Extension"))
    .and_then(delete_cover)
    .with(&cors);


  // * GET ROM Metadata (rest/roms/download/metadata)
  let get_rom_download_metadata = warp::path!("rest" / "roms" / "download" / "metadata")
    .and(warp::get())
    .and(warp::query::<HashMap<String, String>>())
    .and_then(get_rom_metadata)
    .with(&cors);

  // * GET ROM (rest/roms/download)
  let rom_download_route = warp::path!("rest" / "roms" / "download")
    .and(warp::get())
    .and(warp::query::<HashMap<String, String>>())
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
  let filter_upload_store = upload_store.clone();
  let upload_store_filter = warp::any().map(move || filter_upload_store.clone());
  
  // * POST Prepare ROM Upload (rest/roms/upload/prepare)
  let prepare_upload_rom_route = warp::path!("rest" / "roms" / "upload" / "prepare")
    .and(warp::post())
    .and(warp::query::<HashMap<String, String>>())
    .and_then(prepare_rom_upload)
    .with(&cors);
  
  // * POST ROM (rest/roms/upload)
  let upload_rom_route = warp::path!("rest" / "roms" / "upload")
    .and(warp::post())
    .and(warp::filters::body::stream())
    .and(upload_store_filter.clone())
    .and(warp::query::<HashMap<String, String>>())
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
    .and(warp::query::<HashMap<String, String>>())
    .and_then(delete_rom)
    .with(&cors);


  let sgdb_client_store = SGDBClientStore::new();
  let sgdb_client_store_filter = warp::any().map(move || sgdb_client_store.clone());

  let sgdb_init_route = warp::path!("rest" / "proxy" / "sgdb" / "init")
    .and(warp::post())
    .and(sgdb_client_store_filter.clone())
    .and_then(init_sgdb_client)
    .with(&cors);
  
  let sgdb_get_grids_route = warp::path!("rest" / "proxy" / "sgdb" / "grids")
    .and(warp::get())
    .and(sgdb_client_store_filter.clone())
    .and(warp::filters::header::header("SGDB-Game-Id"))
    .and(warp::filters::header::header("SGDB-Results-Page"))
    .and_then(sgdb_get_grids_by_id)
    .with(&cors);
  
  let sgdb_search_game_route = warp::path!("rest" / "proxy" / "sgdb" / "search")
    .and(warp::get())
    .and(sgdb_client_store_filter.clone())
    .and(warp::query::<HashMap<String, String>>())
    .and_then(sgdb_search_game)
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
    .or(delete_rom_route)
    .or(sgdb_init_route)
    .or(sgdb_get_grids_route)
    .or(sgdb_search_game_route);


  let cleanup_upload_store = upload_store.clone();
  std::thread::spawn(move || {
    info!("Thread: Starting failed download cleanup check...");

    const DEAD_TIME_LENGTH: i64 = 60 * 60;

    let schedule = Schedule::from_str(&cleanup_schedule).expect("Failed to parse CRON expression");

    for datetime in schedule.upcoming(Utc).take(1) {
      let now = Utc::now();
      let until = datetime - now;
      thread::sleep(until.to_std().unwrap());


      let streams = cleanup_upload_store.streams.blocking_read().clone();
      for stream in streams.into_values() {
        if now.timestamp() - stream.lastChunkTime > DEAD_TIME_LENGTH {
          info!("Removing failed upload for path \"{}\"", stream.path);

          let remove_res = remove_file(&stream.path);
          if remove_res.is_err() {
            warn!("Error deleting zipped rom folder: {}", remove_res.err().unwrap());
          }

          cleanup_upload_store.streams.blocking_write().remove(&stream.id);
        }
      }
    }
  });


  return http_routes;
}