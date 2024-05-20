pub mod route_handler;
pub mod router;

use serde::Serialize;

/// common response structure
#[derive(Serialize)]
pub struct ApiRep<T> {
  /// current response message
  pub message: String,
  /// current response data
  pub data: T,
}

/// create a quick result
pub fn create_api_rep_with_none<T>(msg: String, data: Option<T>) -> ApiRep<Option<T>> {
  ApiRep { message: msg, data }
}
