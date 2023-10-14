use axum::{middleware::from_fn_with_state, Router};
use axum_macros::FromRef;
use serde::{Serialize, Deserialize};
use sqlx::{Postgres, Pool};

pub mod users;
pub mod error;

pub use self::error::{Error, Result};
use self::users::users::users_router;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub pool: Pool<Postgres>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    access_token: String,
    token_type: String,
}

impl AuthResponse {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

pub async fn router(app_state: AppState) -> Router {

    Router::new()
        .nest("/users", users_router(app_state.clone()).await)

}