use axum::{extract::{State, Query}, Json};
use axum_macros::debug_handler;
use tower_cookies::{Cookies, Cookie};
use sqlx::PgPool;

use crate::{
    api::user::{get_users, create_user, logout_user, verify_user},
    routes::error::Result
};

use super::models::*;

#[debug_handler]
pub async fn find(
    State(pool): State<PgPool>, 
    Query(params): Query<FindUsersQuery>,
) -> Result<Json<FindUsersResponse>> {
    let conn = &mut pool.acquire().await?;
    let user_list = get_users(conn, &params.name).await?;

    let body = Json(FindUsersResponse { users: user_list });

    Ok(body)
}

#[debug_handler]
pub async fn create(
    cookies: Cookies,
    State(pool): State<PgPool>, 
    Json(payload): Json<CreateUserPayload>) -> Result<Json<AuthResponse>> {
    let conn = &mut pool.acquire().await?;
    let token = create_user(conn, &payload.username, &payload.password).await?;
    cookies.add(Cookie::new("token", token.to_string()));

    Ok(Json(AuthResponse { token }))
}

#[debug_handler]
pub async fn login(
    cookies: Cookies,
    State(pool): State<PgPool>, 
    Json(payload): Json<LoginPayload>) -> Result<Json<AuthResponse>> {
    let conn = &mut pool.acquire().await?;
    let token = verify_user(conn, &payload.username, &payload.password).await?;
    cookies.add(Cookie::new("token", token.to_string()));

    Ok(Json(AuthResponse { token }))

}

#[debug_handler]
pub async fn logout(
    State(pool): State<PgPool>, 
    Json(payload): Json<LogoutPayload>,
) -> Result<Json<LogoutResponse>> {
    let conn = &mut pool.acquire().await?;

    logout_user(conn, payload.username).await?;

    let body = Json(LogoutResponse { result: "success".to_string() });

    Ok(body)
}