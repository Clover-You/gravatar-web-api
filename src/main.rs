mod gravatar;
mod serde_visitor;
mod web;

use axum::Router;
use std::{env, error::Error, net::SocketAddr};

// this project author name
const AUTHOR: &'static str = "Clover You";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  // load env profile
  dotenv::dotenv()?;

  // build our application with a single route.
  let app = Router::new();

  let app = web::router::use_user_router(app);

  // read the start port from the .env profile, if it is undefined, use 3000 as the default port.
  let srv_port = env::var("port").unwrap_or("3000".to_string());
  let srv_port: u16 = srv_port.parse().unwrap_or(3000);

  // run our app with hyper, listener globally on poer 3000
  let addr = SocketAddr::from(([127, 0, 0, 1], srv_port));
  let listener = tokio::net::TcpListener::bind(addr).await?;

  println!("{} ===>> server start in the port {}", AUTHOR, addr.port());
  axum::serve(listener, app).await?;

  Ok(())
}
