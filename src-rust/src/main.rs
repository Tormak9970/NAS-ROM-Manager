use websocket::initialize_websocket_api;
use dotenv::dotenv;

mod websocket;
mod restful;

/// The main function
#[tokio::main]
async fn main() {
  dotenv().ok();
  
  pretty_env_logger::init_timed();

  let websocket_route = initialize_websocket_api();
  
  let routes = websocket_route;

  warp::serve(routes)
    .run(([127, 0, 0, 1], 1500))
    .await;
}