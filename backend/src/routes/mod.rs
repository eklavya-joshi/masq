use axum::response::Html;
use axum::routing::get;
use axum::{middleware::from_fn_with_state, Router};
use tower_cookies::CookieManagerLayer;
use axum_macros::FromRef;
use sqlx::{Postgres, Pool};
use tower_http::cors::CorsLayer;

pub mod users;
pub mod messages;
pub mod error;

use crate::middleware::auth::require_auth;

pub use self::error::{Error, Result};
use self::users::{auth_users_router, noauth_users_router};
use self::messages::messages_router;

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
        .route("/hello", get(|| async {Html("i love poop")}))
        .layer(CorsLayer::very_permissive()) // ! REMOVE IN PRODUCTION

}