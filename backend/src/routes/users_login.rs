use axum::{Router, routing::{post}, extract::{State}, Json};
use axum_macros::debug_handler;
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    api::user::verify_user,
    routes::{
        AuthResponse,
        error::Result
    }, 
};

use super::AppState;

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

pub async fn users_login_router(app_state: AppState) -> Router {

    Router::new()
        .route("/login", post(login))
        .with_state(app_state)
}

#[debug_handler]
async fn login(State(pool): State<PgPool>, Json(payload): Json<LoginPayload>) -> Result<Json<AuthResponse>> {
    let conn = &mut pool.acquire().await?;
    let token = verify_user(conn, payload.username.clone(), payload.password).await?;
    // println!("2: {:?}", token);

    Ok(Json(AuthResponse::new(token)))

}