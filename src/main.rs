use axum::{extract::Query, response::IntoResponse, routing::get, Json, Router};
use axum_valid::Valid;
use reqwest::RequestBuilder;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{env, error::Error, net::SocketAddr};
use validator::Validate;

mod gravatar;
mod serde_visitor;
mod web;

// this project author name
const AUTHOR: &'static str = "Clover You";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  // load env profile
  dotenv::dotenv()?;

  // build our application with a single route.
  let app = Router::new();

  let user_routers =
    Router::new().route("/get_user_info_by_gravatar", get(get_user_info_by_gravatar));

  let app = app.nest("/user", user_routers);

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

#[derive(Debug, Deserialize, Validate)]
struct QueryGravatarParams {
  #[validate(email(message = "email validate fail..."))]
  email: String,
  #[validate(range(max = 2048, min = 1, message = "maxmum size is 2048"))]
  size: Option<i16>,
}

/// send request to gravatar get user info by email
async fn get_user_info_by_gravatar(query: Valid<Query<QueryGravatarParams>>) -> impl IntoResponse {
  let trimmed_email = query.email.trim();

  let mut hashed_email = Sha256::new();
  hashed_email.update(trimmed_email);

  // user email to hash 256 code
  let hash = format!("{:X}", hashed_email.finalize());
  let hash = hash.to_lowercase();

  let response = build_gravatar_info_request(hash.as_str()).send().await;

  if let Err(req_err) = &response {
    let err_msg = req_err.to_string();

    println!("err msg = {}", err_msg);
    return Json(create_api_rep_with_none("system error".to_string(), None));
  }

  let response = response.unwrap();
  let resp_status = response.status();

  // get response text from request result
  let data: reqwest::Result<String> = response.text().await;
  let data = data.as_ref();

  if let Err(err) = data {
    println!("err msg = {}", err.to_string());
    return Json(create_api_rep_with_none("system error".to_string(), None));
  }

  // if the request result in 'NOT_FOUND', the user does not exist in Gravatar.
  if resp_status == reqwest::StatusCode::NOT_FOUND {
    return Json(create_api_rep_with_none("user not found".to_string(), None));
  }

  // unknow request error
  if !resp_status.is_success() {
    return Json(create_api_rep_with_none("system error".to_string(), None));
  }

  let data = data.unwrap();

  let form_str: serde_json::Result<gravatar::GravatarEntry> = serde_json::from_str(data.as_str());

  if let Err(err) = form_str {
    println!("err msg = {}", err.to_string());
    return Json(create_api_rep_with_none("system error".to_string(), None));
  }

  let form_str = form_str.unwrap();

  Json(create_api_rep_with_none(
    "success".to_string(),
    Some(form_str),
  ))
}

/// create a request-builder for fetching gravatar info
fn build_gravatar_info_request(hash: &str) -> RequestBuilder {
  let json_url = format!("{}{}.json", gravatar::GRAVATAR_URL, hash);

  let request = reqwest::Client::builder().build().unwrap();
  let request = request.get(&json_url);

  request.header("User-Agent", "rust api")
}

/// create a quick result
fn create_api_rep_with_none<T>(msg: String, data: Option<T>) -> web::ApiRep<Option<T>> {
  web::ApiRep {
    message: msg,
    data: data,
  }
}
