use axum::response::Html;
use axum::routing::get;
use axum::{middleware::from_fn_with_state, Router};
use axum_macros::FromRef;
use sqlx::{Pool, Postgres};
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;

pub mod error;
pub mod messages;
pub mod users;

use crate::middleware::auth::require_auth;

pub use self::error::{Error, Result};
use self::messages::messages_router;
use self::users::{auth_users_router, noauth_users_router};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub pool: Pool<Postgres>,
}

pub async fn router(app_state: AppState) -> Router {
    Router::new()
        .nest("/users", auth_users_router(app_state.clone()).await)
        .nest("/messages", messages_router(app_state.clone()).await)
        .route_layer(from_fn_with_state(app_state.clone(), require_auth))
        .nest("/users", noauth_users_router(app_state.clone()).await)
        .layer(CookieManagerLayer::new())
        .route("/hello", get(|| async { Html("i love poop") }))
        .layer(CorsLayer::very_permissive()) // ! REMOVE IN PRODUCTION
}
