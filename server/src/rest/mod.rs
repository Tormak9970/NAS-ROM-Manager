mod grids;
mod rom_download;
mod rom_upload;
mod types;
mod zip;
mod sgdb;
mod igdb;
mod utils;
mod bios_files;

use std::{collections::HashMap, fs::remove_file, str::FromStr, thread};

use bios_files::{bios_file_download_get_metadata, bios_file_upload_complete, delete_bios_file, prepare_bios_file_upload};
use chrono::Utc;
use grids::{delete_hero, delete_capsule, upload_hero, upload_capsule};
use cron::Schedule;
use igdb::{igdb_get_metadata_by_id, igdb_search_game, igdb_search_platform, init_igdb_client};
use log::{info, warn};
use rom_download::{delete_rom, rom_download_complete, rom_download_get_metadata};
use rom_upload::{prepare_rom_upload, rom_upload_complete};
use sgdb::{init_sgdb_client, sgdb_get_grids_by_id, sgdb_search_game};
use types::{HeroUpload, CapsuleUpload, IGDBClientStore, ROMDownload, ROMUploadComplete, SGDBClientStore, StreamStore};
use utils::{download::download_file, upload::{upload_cancel, upload_file}};
use warp::{http::Method, Filter};

fn json_capsule_upload() -> impl Filter<Extract = (CapsuleUpload,), Error = warp::Rejection> + Clone {
  warp::body::content_length_limit(50 * 1024 * 1024).and(warp::body::json())
}

fn json_hero_upload() -> impl Filter<Extract = (HeroUpload,), Error = warp::Rejection> + Clone {
  warp::body::content_length_limit(50 * 1024 * 1024).and(warp::body::json())
}

fn json_body_download() -> impl Filter<Extract = (ROMDownload,), Error = warp::Rejection> + Clone {
  warp::body::content_length_limit(50 * 1024 * 1024).and(warp::body::json())
}

fn json_body_upload_complete() -> impl Filter<Extract = (ROMUploadComplete,), Error = warp::Rejection> + Clone {
  warp::body::content_length_limit(50 * 1024 * 1024).and(warp::body::json())
}

/// Gets the rest api routes.
pub fn initialize_rest_api(grids_cache_dir: String, cleanup_schedule: String) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
  let cache_dir = grids_cache_dir.clone();
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
      "Full-Capsule-Extension",
      "Thumb-Capsule-Extension",
      "Hero-Extension",
      "File-Length",
      "Upload-Id",
      "File-Size",
      "SGDB-Game-Id",
      "SGDB-Results-Page",
      "SGDB-Grid-Type",
      "IGDB-Game-Id",
    ])
    .allow_methods(&[
      Method::GET,
      Method::POST,
      Method::DELETE
    ]);

  // * GET grids (rest/grids/{image_file})
  let grids_get_route = warp::path!("rest" / "grids" / ..)
    .and(
      warp::fs::dir(grids_cache_dir).map(|reply| {
        warp::reply::with_header(
          reply,
          "Cache-Control",
          "no-store"
        )
      })
    )
    .with(&cors);

  // * POST capsule (rest/grids/capsules/{id})
  let capsule_upload_route = warp::path!("rest" / "grids" / "capsules" / String)
    .and(warp::post())
    .and(cache_dir_filter.clone())
    .and(json_capsule_upload())
    .and_then(upload_capsule)
    .with(&cors);
  
  // * DELETE capsule (rest/grids/capsules/{id})
  let capsule_delete_route = warp::path!("rest" / "grids" / "capsules" / String)
    .and(warp::delete())
    .and(cache_dir_filter.clone())
    .and(warp::filters::header::header("Full-Capsule-Extension"))
    .and(warp::filters::header::header("Thumb-Capsule-Extension"))
    .and_then(delete_capsule)
    .with(&cors);
  
  // * POST hero (rest/grids/heroes/{id})
  let hero_upload_route = warp::path!("rest" / "grids" / "heroes" / String)
    .and(warp::post())
    .and(cache_dir_filter.clone())
    .and(json_hero_upload())
    .and_then(upload_hero)
    .with(&cors);
  
  // * DELETE hero (rest/grids/heroes/{id})
  let hero_delete_route = warp::path!("rest" / "grids" / "heroes" / String)
    .and(warp::delete())
    .and(cache_dir_filter.clone())
    .and(warp::filters::header::header("Hero-Extension"))
    .and_then(delete_hero)
    .with(&cors);


  // * GET ROM Metadata (rest/roms/download/metadata)
  let rom_download_get_metadata = warp::path!("rest" / "roms" / "download" / "metadata")
    .and(warp::get())
    .and(warp::query::<HashMap<String, String>>())
    .and_then(rom_download_get_metadata)
    .with(&cors);

  // * GET ROM (rest/roms/download)
  let rom_download_route = warp::path!("rest" / "roms" / "download")
    .and(warp::get())
    .and(warp::query::<HashMap<String, String>>())
    .and_then(download_file)
    .with(&cors);
  
  // * POST ROM (rest/roms/download/complete)
  let rom_download_complete_route = warp::path!("rest" / "roms" / "download" / "complete")
    .and(warp::post())
    .and(json_body_download())
    .and_then(rom_download_complete)
    .with(&cors);


  let upload_store = StreamStore::new();
  let filter_upload_store = upload_store.clone();
  let upload_store_filter = warp::any().map(move || filter_upload_store.clone());
  
  // * POST Prepare ROM Upload (rest/roms/upload/prepare)
  let rom_upload_prepare_route = warp::path!("rest" / "roms" / "upload" / "prepare")
    .and(warp::post())
    .and(warp::query::<HashMap<String, String>>())
    .and_then(prepare_rom_upload)
    .with(&cors);
  
  // * POST ROM (rest/roms/upload)
  let rom_upload_route = warp::path!("rest" / "roms" / "upload")
    .and(warp::post())
    .and(warp::filters::body::stream())
    .and(upload_store_filter.clone())
    .and(warp::query::<HashMap<String, String>>())
    .and(warp::filters::header::headers_cloned())
    .and_then(upload_file)
    .with(&cors);

  // * POST ROM (rest/roms/upload/complete)
  let rom_upload_complete_route = warp::path!("rest" / "roms" / "upload" / "complete")
    .and(warp::post())
    .and(upload_store_filter.clone())
    .and(json_body_upload_complete())
    .and_then(rom_upload_complete)
    .with(&cors);

  // * POST ROM (rest/roms/upload/complete)
  let rom_upload_cancel_route = warp::path!("rest" / "roms" / "upload" / "cancel")
    .and(warp::post())
    .and(upload_store_filter.clone())
    .and(warp::filters::header::header("Upload-Id"))
    .and_then(upload_cancel)
    .with(&cors);
  

  // * DELETE ROM (rest/roms)
  let rom_delete_route = warp::path!("rest" / "roms" / "delete")
    .and(warp::delete())
    .and(warp::query::<HashMap<String, String>>())
    .and_then(delete_rom)
    .with(&cors);

  
  // * GET BIOS Metadata (rest/bios-files/download/metadata)
  let bios_download_get_metadata = warp::path!("rest" / "bios-files" / "download" / "metadata")
    .and(warp::get())
    .and(warp::query::<HashMap<String, String>>())
    .and_then(bios_file_download_get_metadata)
    .with(&cors);

  // * GET BIOS (rest/bios-files/download)
  let bios_download_route = warp::path!("rest" / "bios-files" / "download")
    .and(warp::get())
    .and(warp::query::<HashMap<String, String>>())
    .and_then(download_file)
    .with(&cors);


  // * POST Prepare BIOS Upload (rest/bios-files/upload/prepare)
  let bios_upload_prepare_route = warp::path!("rest" / "bios-files" / "upload" / "prepare")
    .and(warp::post())
    .and(warp::query::<HashMap<String, String>>())
    .and_then(prepare_bios_file_upload)
    .with(&cors);
  
  // * POST BIOS (rest/bios-files/upload)
  let bios_upload_route = warp::path!("rest" / "bios-files" / "upload")
    .and(warp::post())
    .and(warp::filters::body::stream())
    .and(upload_store_filter.clone())
    .and(warp::query::<HashMap<String, String>>())
    .and(warp::filters::header::headers_cloned())
    .and_then(upload_file)
    .with(&cors);

  // * POST BIOS (rest/bios-files/upload/complete)
  let bios_upload_complete_route = warp::path!("rest" / "bios-files" / "upload" / "complete")
    .and(warp::post())
    .and(upload_store_filter.clone())
    .and(warp::filters::header::header("Upload-Id"))
    .and_then(bios_file_upload_complete)
    .with(&cors);

  // * POST BIOS (rest/bios-files/upload/complete)
  let bios_upload_cancel_route = warp::path!("rest" / "bios-files" / "upload" / "cancel")
    .and(warp::post())
    .and(upload_store_filter)
    .and(warp::filters::header::header("Upload-Id"))
    .and_then(upload_cancel)
    .with(&cors);


  // * DELETE BIOS (rest/bios-files)
  let bios_delete_route = warp::path!("rest" / "bios-files" / "delete")
    .and(warp::delete())
    .and(warp::query::<HashMap<String, String>>())
    .and_then(delete_bios_file)
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
    .and(warp::filters::header::header("SGDB-Grid-Type"))
    .and_then(sgdb_get_grids_by_id)
    .with(&cors);
  
  let sgdb_search_game_route = warp::path!("rest" / "proxy" / "sgdb" / "search")
    .and(warp::get())
    .and(sgdb_client_store_filter.clone())
    .and(warp::query::<HashMap<String, String>>())
    .and_then(sgdb_search_game)
    .with(&cors);


  
  let igdb_client_store = IGDBClientStore::new();
  let igdb_client_store_filter = warp::any().map(move || igdb_client_store.clone());

  let igdb_init_route = warp::path!("rest" / "proxy" / "igdb" / "init")
    .and(warp::post())
    .and(igdb_client_store_filter.clone())
    .and_then(init_igdb_client)
    .with(&cors);
  
  let igdb_get_metadata_route = warp::path!("rest" / "proxy" / "igdb" / "metadata")
    .and(warp::get())
    .and(igdb_client_store_filter.clone())
    .and(warp::filters::header::header("IGDB-Game-Id"))
    .and_then(igdb_get_metadata_by_id)
    .with(&cors);
  
  let igdb_search_game_route = warp::path!("rest" / "proxy" / "igdb" / "search" / "games")
    .and(warp::get())
    .and(igdb_client_store_filter.clone())
    .and(warp::query::<HashMap<String, String>>())
    .and_then(igdb_search_game)
    .with(&cors);
  
  let igdb_search_platform_route = warp::path!("rest" / "proxy" / "igdb" / "search" / "platforms")
    .and(warp::get())
    .and(igdb_client_store_filter.clone())
    .and(warp::query::<HashMap<String, String>>())
    .and_then(igdb_search_platform)
    .with(&cors);

  let grids_routes = grids_get_route
    .or(capsule_upload_route)
    .or(capsule_delete_route)
    .or(hero_upload_route)
    .or(hero_delete_route);
  
  let rom_routes = rom_download_get_metadata
    .or(rom_download_route)
    .or(rom_download_complete_route)
    .or(rom_upload_prepare_route)
    .or(rom_upload_route)
    .or(rom_upload_complete_route)
    .or(rom_upload_cancel_route)
    .or(rom_delete_route);

  let bios_routes = bios_download_get_metadata
    .or(bios_download_route)
    .or(bios_upload_prepare_route)
    .or(bios_upload_route)
    .or(bios_upload_complete_route)
    .or(bios_upload_cancel_route)
    .or(bios_delete_route);

  let sgdb_routes = sgdb_init_route
    .or(sgdb_get_grids_route)
    .or(sgdb_search_game_route);

  let igdb_routes = igdb_init_route
    .or(igdb_get_metadata_route)
    .or(igdb_search_game_route)
    .or(igdb_search_platform_route);

  let http_routes = grids_routes
    .or(rom_routes)
    .or(bios_routes)
    .or(sgdb_routes)
    .or(igdb_routes);


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