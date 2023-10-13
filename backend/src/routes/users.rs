use axum::{Router, routing::{get, post}, extract::{State, Query}, response::{IntoResponse, Html}, Json, Extension};
use axum_macros::debug_handler;
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::{
    api::user::{get_users, create_user, logout_user},
    routes::{
        AuthResponse,
        error::Result
    }, 
    database::schema::User
};

use super::AppState;

#[derive(Deserialize)]
struct GetUsers {
    name: String,
}

#[derive(Debug, Deserialize)]
struct CreatePayload {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct LogoutPayload {
    username: String,
}


pub async fn users_router(app_state: AppState) -> Router {

    Router::new()
        .route("/find", get(find))
        .route("/create", post(create))
        .route("/logout", post(logout))
        .with_state(app_state)
}

#[debug_handler]
async fn find(
    State(pool): State<PgPool>, 
    Query(params): Query<GetUsers>,
) -> impl IntoResponse {
    let conn = &mut pool.acquire().await.unwrap();
    let user_list = get_users(conn, params.name).await.unwrap();
    let mut str = String::new();
    for u in user_list {
        str.push_str(&format!("username: {}\ncreated: {}\n", u.name, u.created.format("%d-%m-%Y %H:%M:%S")));
    }
    Html(str)
}

#[debug_handler]
async fn create(State(pool): State<PgPool>, Json(payload): Json<CreatePayload>) -> Result<Json<AuthResponse>> {
    let conn = &mut pool.acquire().await?;
    let token = create_user(conn, payload.username.clone(), payload.password.clone()).await?;

    Ok(Json(AuthResponse::new(token)))
}

#[debug_handler]
async fn logout(
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