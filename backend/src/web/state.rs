use axum::{Router, routing::get, extract::{State, Query}, response::{IntoResponse, Html}};
use diesel::{PgConnection, r2d2::{ConnectionManager, Pool}};

#[derive(Clone)]
pub struct AppState {
  pub pool: Pool<ConnectionManager<PgConnection>>,
}