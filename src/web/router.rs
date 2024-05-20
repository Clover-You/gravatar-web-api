use axum::routing::get;

use super::route_handler;

pub fn use_user_router<S>(app: axum::Router<S>) -> axum::Router<S>
where
  S: Clone + Send + Sync + 'static,
{
  let user_router_group = axum::Router::new().route(
    "/get_user_info_by_gravatar",
    get(route_handler::user::get_user_info_by_gravatar),
  );

  app.nest("/user", user_router_group)
}
