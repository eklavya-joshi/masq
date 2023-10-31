use std::collections::HashMap;
use std::sync::Arc;

use axum::response::Html;
use axum::routing::get;
use axum::{middleware::from_fn_with_state, Router};
use axum_macros::FromRef;
use sqlx::PgPool;
use tokio::sync::broadcast::Sender;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;
use uuid::Uuid;

pub mod error;
pub mod messages;
pub mod users;
pub mod websocket;

use crate::middleware::auth::require_auth;

pub use self::error::{Error, Result};
use self::messages::messages_router;
use self::users::{auth_users_router, noauth_users_router};
use self::websocket::websocket_router;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub pool: PgPool,
    pub tx_map: HashMap<Uuid, Sender<String>>
}

pub async fn router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/users", auth_users_router(app_state.clone()).await)
        .nest("/messages", messages_router(app_state.clone()).await)
        .nest("/ws", websocket_router(app_state.clone()).await)
        .route_layer(from_fn_with_state(app_state.clone(), require_auth))
        .nest("/users", noauth_users_router(app_state.clone()).await)
        .layer(CookieManagerLayer::new())
        .route("/hello", get(|| async { Html("i love poop") }))
        .layer(CorsLayer::very_permissive()) // ! REMOVE IN PRODUCTION
}
