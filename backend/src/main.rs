pub mod api;
pub mod database;
pub mod error;
pub mod middleware;
pub mod routes;
pub mod utils;

use std::{sync::Arc, collections::HashMap};

use crate::{
    database::get_connection_pool,
    error::Result,
    routes::{router, AppState},
};

#[tokio::main]
async fn main() -> Result<()> {
    let pool = get_connection_pool().await?;
    let tx_map = HashMap::new();

    let app_state = Arc::new(AppState { pool, tx_map });

    let app = router(app_state).await;

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("Couldn't start server");

    Ok(())
}
