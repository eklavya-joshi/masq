use std::sync::Arc;

use axum::{Router, routing::get, extract::{State, Query}, response::{IntoResponse, Html}};
use diesel::{PgConnection, r2d2::{ConnectionManager, Pool}};
use serde::Deserialize;

pub mod api;
pub mod database;
pub mod schema;
pub mod models;

use database::{
  database::get_connection_pool,
};
use api::{
  user::get_users
};

#[derive(Clone)]
struct AppState {
  pool: Pool<ConnectionManager<PgConnection>>,
}

#[derive(Deserialize)]
struct GetUsers {
  name: String,
  n: u32
}

#[tokio::main]
async fn main() {

  let app_state = Arc::new(AppState {pool: get_connection_pool()});

  let app = Router::new()
  .route("/", get(|| async { "Hello, World!" }))
  .route("/user", get(get_user))
  .with_state(app_state);

  // run it with hyper on localhost:3000
  axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
      .serve(app.into_make_service())
      .await
      .unwrap();
}

async fn get_user(State(state): State<Arc<AppState>>, Query(params): Query<GetUsers>) -> impl IntoResponse {
  Html(get_users(&mut state.pool.get().unwrap(), params.name, params.n))
}