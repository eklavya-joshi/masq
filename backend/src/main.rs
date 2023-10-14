use axum::{Router, middleware::from_fn_with_state};

pub mod api;
pub mod database;
pub mod routes;
pub mod middleware;
pub mod utils;
pub mod error;

use crate::{
  database::get_connection_pool,
  routes::{
    AppState,
    users::{users::users_router, users_login::users_login_router},
  },
  middleware::auth::require_auth,
  error::Result
};

#[tokio::main]
async fn main() -> Result<()> {

  let pool = get_connection_pool().await?;
  let app_state = AppState {pool: pool};

  let app = Router::new()
    .nest("/users", users_router(app_state.clone()).await)
    .route_layer(from_fn_with_state(app_state.clone(), require_auth))
    .nest("/users", users_login_router(app_state.clone()).await);
  

  // run it with hyper on localhost:3000
  axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
      .serve(app.into_make_service())
      .await
      .expect("Couldn't start server");

  Ok(())
}
