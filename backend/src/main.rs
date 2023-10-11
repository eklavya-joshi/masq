use std::sync::Arc;

use axum::{Router, routing::{get, post}, extract::{State, Query}, response::{IntoResponse, Html}, Json};
use diesel::{PgConnection, r2d2::{ConnectionManager, Pool}};
use serde::Deserialize;
use serde_json::{Value, json};

pub mod error;
pub mod api;
pub mod database;
pub mod schema;
pub mod models;
pub mod web;

use crate::{
  database::{
    database::get_connection_pool,
  },
  web::{
    state::AppState,
    users::users_router
  }
};

#[derive(Deserialize)]
struct GetUsers {
  name: String,
  n: u32
}

struct TestPayload {
    username: String,
    password: String,
}

#[tokio::main]
async fn main() {

  let app_state = Arc::new(AppState {pool: get_connection_pool()});

  let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .merge(users_router(app_state).await);
  

  // run it with hyper on localhost:3000
  axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
      .serve(app.into_make_service())
      .await
      .unwrap();
}
