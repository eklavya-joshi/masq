use axum::{
    routing::{get, post},
    Router,
};

use self::users::{create, find, login, logout};

use super::AppState;

pub mod models;
pub mod users;

pub async fn auth_users_router(app_state: AppState) -> Router {
    Router::new()
        .route("/find", get(find))
        .route("/logout", post(logout))
        .with_state(app_state.clone())
}

pub async fn noauth_users_router(app_state: AppState) -> Router {
    Router::new()
        .route("/create", post(create))
        .route("/login", post(login))
        .with_state(app_state)
}
