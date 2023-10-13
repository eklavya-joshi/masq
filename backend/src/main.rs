use std::sync::Arc;

use axum::{Router, routing::get};

pub mod api;
pub mod database;
pub mod routes;
pub mod middleware;
pub mod utils;

use crate::{
  database::get_connection_pool,
  routes::{
    AppState,
    users::users_router
  }
};

#[tokio::main]
async fn main() {

  let app_state = Arc::new(AppState {pool: get_connection_pool().await});

  let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .nest("/users", users_router(app_state).await);
  

  // run it with hyper on localhost:3000
  axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
      .serve(app.into_make_service())
      .await
      .unwrap();
}
