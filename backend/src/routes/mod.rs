use axum_macros::FromRef;
use serde::{Serialize, Deserialize};
use sqlx::{Postgres, Pool};

pub mod users;
pub mod users_login;
pub mod error;

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