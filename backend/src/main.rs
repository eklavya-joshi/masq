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
    router
  },
  error::Result
};

#[tokio::main]
async fn main() -> Result<()> {

  let pool = get_connection_pool().await?;
  let app_state = AppState {pool: pool};

  let app = router(app_state).await;
  

  // run it with hyper on localhost:3000
  axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
      .serve(app.into_make_service())
      .await
      .expect("Couldn't start server");

  Ok(())
}
