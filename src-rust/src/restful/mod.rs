mod covers;
mod roms;

use warp::Filter;

/// Gets the rest api routes.
pub fn initialize_rest_api() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {

  // TODO: GET cover (rest/covers/{id})
  // TODO: PUT cover (rest/covers/{id})
  // TODO: DELETE cover (rest/covers/{id}) (might need delete)

  // TODO: GET ROM (rest/roms/{id}/download)
  // TODO: PUT ROM (rest/roms/{id}/upload)
  // TODO: delete ROM

  // GET /hello/warp => 200 OK with body "Hello, warp!"
  let hello = warp::path!("hello" / String)
    .map(|name| format!("Hello, {}!", name));
  
  let http_routes = hello;
  return http_routes;
}