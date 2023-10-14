use axum::{Router, routing::{get, post}, middleware::from_fn_with_state};

use crate::middleware::auth::require_auth;

use self::users::{find, create, logout, login};

use super::{AppState, Result};

pub mod users;
pub mod users_login;

async fn users_router(app_state: AppState) -> Router {

    Router::new()
        .route("/find", get(find))
        .route("/create", post(create))
        .route("/logout", post(logout))
        .route_layer(from_fn_with_state(app_state.clone(), require_auth))
        .route("/login", post(login))
        .with_state(app_state.clone())

}