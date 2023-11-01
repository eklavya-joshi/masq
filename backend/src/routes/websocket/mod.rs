use std::sync::Arc;

use axum::{Router, routing::get};
use self::websocket::websocket_handler;

use super::AppState;

pub mod websocket;

pub async fn websocket_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/inbox/:id", get(websocket_handler))
        .with_state(app_state.clone())
}