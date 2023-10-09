use axum::{Router, routing::get, extract::{State, Query}, response::{IntoResponse, Html}};
use database::{get_connection_pool};
use diesel::{PgConnection, r2d2::{ConnectionManager, Pool}};
use serde::Deserialize;

use crate::users::get_user_pass;
pub mod database;
pub mod users;
pub mod schema;
pub mod models;

#[derive(Clone)]
struct AppState {
  pool: Pool<ConnectionManager<PgConnection>>,
}

#[derive(Deserialize)]
struct TestParams {
  name: String
}

#[tokio::main]
async fn main() {

  let app_state = AppState {pool: get_connection_pool()};

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

async fn get_user(State(state): State<AppState>, Query(params): Query<TestParams>) -> impl IntoResponse {
  Html(get_user_pass(&mut state.pool.get().unwrap(), params.name))
}