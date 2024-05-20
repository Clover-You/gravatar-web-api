use axum::{extract::Query, response::IntoResponse, Json};
use axum_valid::Valid;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use validator::Validate;

use crate::{gravatar, web::create_api_rep_with_none};

/// get avatar url request params
#[derive(Debug, Deserialize, Validate)]
pub struct GetAvatarInGravatarParamsModel {
  /// user email
  #[validate(email(message = "email validate fail..."))]
  email: String,
  /// url image size param
  #[validate(range(max = 2048, min = 1, message = "maxmum size is 2048"))]
  size: Option<i16>,
}

/// get user avatar url
pub async fn get_avatar_in_gravatar(
  query: Valid<Query<GetAvatarInGravatarParamsModel>>,
) -> impl IntoResponse {
  let trimmed_email = query.email.trim();

  let mut hashed_email = Sha256::new();
  hashed_email.update(trimmed_email);

  // user email to hash 256 code
  let hash = format!("{:X}", hashed_email.finalize());
  let hash = hash.to_lowercase();

  let img_size = query.size.unwrap_or(200);

  let avatar_url = format!("{}avatar/{}?s={}", gravatar::GRAVATAR_URL, hash, img_size);

  Json(create_api_rep_with_none(
    "success".to_string(),
    Some(avatar_url),
  ))
}
