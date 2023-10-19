use axum::{
    extract::{Query, State},
    Extension, Json,
};
use axum_macros::debug_handler;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    api::user::{create_user, find_users, logout_user, verify_user},
    routes::error::{log, Result},
};

use super::models::*;

#[debug_handler]
pub async fn find(
    Extension(user): Extension<Uuid>,
    State(pool): State<PgPool>,
    Query(params): Query<FindUsersQuery>,
) -> Result<Json<FindUsersResponse>> {
    println!("->> {:<18} - {}", "HANDLER", "/users/find");

    let conn = &mut pool.acquire().await?;
    let user_list = find_users(conn, &params.name, user).await?;

    log(Json(FindUsersResponse { users: user_list }), "/users/find")
}

#[debug_handler]
pub async fn create(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserPayload>,
) -> Result<Json<AuthResponse>> {
    println!("->> {:<18} - {}", "HANDLER", "/users/create");

    let conn = &mut pool.acquire().await?;

    let token = create_user(conn, &payload.username, &payload.password).await?;

    log(Json(AuthResponse { token }), "/users/create")
}

#[debug_handler]
pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<AuthResponse>> {
    println!("->> {:<18} - {}", "HANDLER", "/users/login");

    let conn = &mut pool.acquire().await?;

    let token = verify_user(conn, &payload.username, &payload.password).await?;

    log(Json(AuthResponse { token }), "/users/login")
}

#[debug_handler]
pub async fn logout(
    State(pool): State<PgPool>,
    Json(payload): Json<LogoutPayload>,
) -> Result<Json<LogoutResponse>> {
    println!("->> {:<18} - {}", "HANDLER", "/users/logout");

    let conn = &mut pool.acquire().await?;

    logout_user(conn, payload.username).await?;

    log(
        Json(LogoutResponse {
            result: "success".to_string(),
        }),
        "/users/logout",
    )
}
