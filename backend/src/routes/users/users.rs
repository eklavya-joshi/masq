use axum::{extract::{State, Query}, Json};
use axum_macros::debug_handler;
use tower_cookies::{Cookies, Cookie};
use sqlx::PgPool;

use crate::{
    api::user::{get_users, create_user, logout_user, verify_user},
    routes::error::{Result, log}
};

use super::models::*;

#[debug_handler]
pub async fn find(
    State(pool): State<PgPool>, 
    Query(params): Query<FindUsersQuery>,
) -> Result<Json<FindUsersResponse>> {
    println!("->> {:<18} - {}", "HANDLER", "/users/find");

    let conn = &mut pool.acquire().await?;
    let user_list = get_users(conn, &params.name).await?;
    
    log(Json(FindUsersResponse { users: user_list }), "/users/find")
}

#[debug_handler]
pub async fn create(
    cookies: Cookies,
    State(pool): State<PgPool>, 
    Json(payload): Json<CreateUserPayload>) -> Result<Json<AuthResponse>> {
    println!("->> {:<18} - {}", "HANDLER", "/users/create");

    let conn = &mut pool.acquire().await?;

    let token = create_user(conn, &payload.username, &payload.password).await?;

    let mut cookie = Cookie::new("jwt", token.to_string());
    cookie.set_http_only(Some(false));
    cookie.set_secure(true);
    cookie.set_same_site(None);
    cookies.add(cookie);

    log(Json(AuthResponse { token }), "/users/create")
}

#[debug_handler]
pub async fn login(
    cookies: Cookies,
    State(pool): State<PgPool>, 
    Json(payload): Json<LoginPayload>) -> Result<Json<AuthResponse>> {
    println!("->> {:<18} - {}", "HANDLER", "/users/login");

    let conn = &mut pool.acquire().await?;

    let token = verify_user(conn, &payload.username, &payload.password).await?;

    let mut cookie = Cookie::new("jwt", token.to_string());
    cookie.set_http_only(Some(false));
    cookie.set_secure(true);
    cookie.set_same_site(None);
    cookies.add(cookie);

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

    log(Json(LogoutResponse { result: "success".to_string() }), "/users/logout")
}