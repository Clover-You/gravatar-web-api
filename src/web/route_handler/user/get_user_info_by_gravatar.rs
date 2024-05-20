use axum::{extract::Query, response::IntoResponse, Json};
use axum_valid::Valid;
use reqwest::RequestBuilder;
use serde::Deserialize;
use sha2::{Digest, Sha256};
pub use validator::Validate;

use crate::{gravatar, web::create_api_rep_with_none};

/// query gravatar user info params model
#[derive(Debug, Deserialize, Validate)]
pub struct QueryGravatarUserInfoParams {
  /// user email
  #[validate(email(message = "email validate fail..."))]
  email: String,
}

/// send request to gravatar get user info by email
pub async fn get_user_info_by_gravatar(
  query: Valid<Query<QueryGravatarUserInfoParams>>,
) -> impl IntoResponse {
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
