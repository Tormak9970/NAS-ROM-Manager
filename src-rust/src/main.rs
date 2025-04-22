use std::{env::var, path::PathBuf};

use rest::initialize_rest_api;
use tokio::fs::create_dir_all;
use warp::Filter;
use websocket::initialize_websocket_api;
use dotenv::dotenv;

mod websocket;
mod rest;

/// The main function
#[tokio::main]
async fn main() {
  dotenv().ok();
  
  pretty_env_logger::init_timed();

  let grids_cache_dir_str = var("NRM_GRIDS_CACHE_DIR").unwrap();
  let grids_cache_dir = PathBuf::from(&grids_cache_dir_str);
  create_dir_all(&grids_cache_dir.join("full")).await.expect("Failed to create full capsule cache dir.");
  create_dir_all(&grids_cache_dir.join("thumb")).await.expect("Failed to create thumb capsule cache dir.");
  create_dir_all(&grids_cache_dir.join("hero")).await.expect("Failed to create hero cache dir.");
  
  let cleanup_schedule = var("NRM_UPLOAD_CLEAN_SCHEDULE").unwrap();

  let websocket_route = initialize_websocket_api();
  let rest_routes = initialize_rest_api(grids_cache_dir_str, cleanup_schedule);
  
  let routes = websocket_route.or(rest_routes);

  warp::serve(routes)
    .run(([127, 0, 0, 1], 1500))
    .await;
}