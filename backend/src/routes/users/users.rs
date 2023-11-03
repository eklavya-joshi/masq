use axum::{
    extract::{Query, State},
    Extension, Json,
};
use axum_macros::debug_handler;
use uuid::Uuid;

use crate::{
    api::user::{create_user, find_users, logout_user, verify_user, AuthUserInfo},
    routes::{error::{log, Result}, AppState},
};

use super::models::*;

#[debug_handler]
pub async fn find(
    Extension(user): Extension<Uuid>,
    State(state): State<AppState>,
    Query(params): Query<FindUsersQuery>,
) -> Result<Json<FindUsersResponse>> {
    println!("->> {:<18} - {}", "HANDLER", "/users/find");

    let conn = &mut state.pool.acquire().await?;
    let user_list = find_users(conn, &params.name, user).await?;

    log(Json(FindUsersResponse { users: user_list }), "/users/find")
}

#[debug_handler]
pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserPayload>,
) -> Result<Json<AuthUserInfo>> {
    println!("->> {:<18} - {}", "HANDLER", "/users/create");

    let conn = &mut state.pool.acquire().await?;

    let auth_user_info = create_user(conn, &payload.username, &payload.password).await?;

    log(Json(auth_user_info), "/users/create")
}

#[debug_handler]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<AuthUserInfo>> {
    println!("->> {:<18} - {}", "HANDLER", "/users/login");

    let conn = &mut state.pool.acquire().await?;

    let auth_user_info = verify_user(conn, &payload.username, &payload.password).await?;

    log(Json(auth_user_info), "/users/login")
}

#[debug_handler]
pub async fn logout(
    State(state): State<AppState>,
    Json(payload): Json<LogoutPayload>,
) -> Result<Json<LogoutResponse>> {
    println!("->> {:<18} - {}", "HANDLER", "/users/logout");

    let conn = &mut state.pool.acquire().await?;

    logout_user(conn, payload.username).await?;

    log(
        Json(LogoutResponse {
            result: "success".to_string(),
        }),
        "/users/logout",
    )
}
