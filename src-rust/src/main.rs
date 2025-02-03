use std::env::var;

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

  let cover_cache_dir = var("NRM_COVER_CACHE_DIR").unwrap();
  create_dir_all(&cover_cache_dir).await.expect("Failed to create cover cache dir.");
  
  let cleanup_schedule = var("NRM_UPLOAD_CLEAN_SCHEDULE").unwrap();

  let websocket_route = initialize_websocket_api();
  let rest_routes = initialize_rest_api(cover_cache_dir, cleanup_schedule);
  
  let routes = websocket_route.or(rest_routes);

  warp::serve(routes)
    .run(([127, 0, 0, 1], 1500))
    .await;
}