use axum::{Router, routing::get, middleware::from_fn_with_state};

pub mod api;
pub mod database;
pub mod routes;
pub mod middleware;
pub mod utils;

use crate::{
  database::get_connection_pool,
  routes::{
    AppState,
    users::users_router,
    users_login::users_login_router
  },
  middleware::auth::require_auth
};

#[tokio::main]
async fn main() {

  let app_state = AppState {pool: get_connection_pool().await};

  let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .nest("/users", users_router(app_state.clone()).await)
    .route_layer(from_fn_with_state(app_state.clone(), require_auth))
    .nest("/users", users_login_router(app_state.clone()).await);
  

  // run it with hyper on localhost:3000
  axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
      .serve(app.into_make_service())
      .await
      .unwrap();
}
