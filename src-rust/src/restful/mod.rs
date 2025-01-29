mod covers;
mod roms;

use std::{convert::Infallible, env::var};

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

/// Gets the rest api routes.
pub fn initialize_rest_api() -> impl Filter + Clone {
  let cache_dir = var("NRM_COVER_CACHE_DIR").ok().unwrap();

  // ! GET cover (rest/covers/{id})
  let get_cover_route = warp::get()
    .and(warp::path!("rest" / "covers"))
    .and(warp::fs::dir(cache_dir.clone()));

  // TODO: POST cover (rest/covers/{id})
  let post_cover_route = warp::post()
    .and(warp::path!("rest" / "covers" / String))
    .map(|rom_id| {
      format!("Hello, {}!", rom_id)
    });

  // TODO: DELETE cover (rest/covers/{id}) (might need delete)
  let delete_cover_route = warp::delete()
    .and(warp::path!("rest" / "covers" / String))
    .map(|rom_id| {
      format!("Hello, {}!", rom_id)
    });

  // TODO: GET ROM (rest/roms/{id}/download)
  // TODO: PUT ROM (rest/roms/{id}/upload)
  // TODO: delete ROM

  // GET /hello/warp => 200 OK with body "Hello, warp!"
  let hello = warp::path!("hello" / String)
    .map(|name| format!("Hello, {}!", name));
  
  let http_routes = get_cover_route
    .or(post_cover_route)
    .or(delete_cover_route)
    .or(hello)
    .recover(handle_rejection);

  return http_routes;
}