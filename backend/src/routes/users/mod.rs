use axum::{Router, routing::{get, post}};

use self::users::{find, create, logout, login};

use super::AppState;

pub mod users;

pub async fn users_router(app_state: AppState) -> Router {

    Router::new()
        .route("/find", get(find))
        .route("/logout", post(logout))
        .with_state(app_state.clone())

}

pub async fn new_users_router(app_state: AppState) -> Router {

    Router::new()
        .route("/create", post(create))
        .route("/login", post(login))
        .with_state(app_state)
}