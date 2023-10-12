use axum_macros::FromRef;
use sqlx::{Postgres, Pool};

pub mod users;
pub mod error;

#[derive(Clone, FromRef)]
pub struct AppState {
  pub pool: Pool<Postgres>,
}