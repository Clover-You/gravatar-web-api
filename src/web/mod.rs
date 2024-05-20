use serde::Serialize;

/// common response structure
#[derive(Serialize)]
pub struct ApiRep<T> {
  /// current response message
  pub message: String,
  /// current response data
  pub data: T,
}
