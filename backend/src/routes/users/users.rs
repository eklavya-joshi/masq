use axum::{extract::{State, Query}, Json, Extension};
use axum_macros::debug_handler;
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::{
    api::user::{get_users, create_user, logout_user, verify_user},
    routes::{
        AuthResponse,
        error::Result
    }, 
    database::schema::User
};

#[derive(Deserialize)]
pub struct GetUsers {
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreatePayload {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
pub struct LogoutPayload {
    username: String,
}

#[debug_handler]
pub async fn find(
    State(pool): State<PgPool>, 
    Query(params): Query<GetUsers>,
) -> Result<Json<Value>> {
    let conn = &mut pool.acquire().await?;
    let user_list = get_users(conn, &params.name).await?;
    let body = Json(json!({
        "users" : user_list
    }));

    Ok(body)
}

#[debug_handler]
pub async fn create(State(pool): State<PgPool>, Json(payload): Json<CreatePayload>) -> Result<Json<AuthResponse>> {
    let conn = &mut pool.acquire().await?;
    let token = create_user(conn, &payload.username, &payload.password).await?;

    Ok(Json(AuthResponse::new(token)))
}

#[debug_handler]
pub async fn login(State(pool): State<PgPool>, Json(payload): Json<LoginPayload>) -> Result<Json<AuthResponse>> {
    let conn = &mut pool.acquire().await?;
    let token = verify_user(conn, &payload.username, &payload.password).await?;

    Ok(Json(AuthResponse::new(token)))

}

#[debug_handler]
pub async fn logout(
    State(pool): State<PgPool>, 
    Extension(_user): Extension<User>,
    Json(payload): Json<LogoutPayload>,
) -> Result<Json<Value>> {
    let conn = &mut pool.acquire().await?;

    logout_user(conn, payload.username).await?;

    let body = Json(json!({
        "result" : "success"
    }));

    Ok(body)
}