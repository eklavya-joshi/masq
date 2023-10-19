pub mod api;
pub mod database;
pub mod error;
pub mod middleware;
pub mod routes;
pub mod utils;

use crate::{
    database::get_connection_pool,
    error::Result,
    routes::{router, AppState},
};

#[tokio::main]
async fn main() -> Result<()> {
    let pool = get_connection_pool().await?;
    let app_state = AppState { pool };

    let app = router(app_state).await;

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("Couldn't start server");

    Ok(())
}
