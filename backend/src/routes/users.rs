use std::sync::Arc;
use axum::{Router, routing::{get, post}, extract::{State, Query}, response::{IntoResponse, Html}, Json};
use axum_macros::debug_handler;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{
    api::user::{get_users, create_user, verify_user},
    routes::error::Result, 
    middleware::jwt::create_token
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
struct LoginPayload {
    username: String,
    password: String,
}


pub async fn users_router(app_state: Arc<AppState>) -> Router {

    Router::new()
        .route("/find", get(find))
        .route("/create", post(create))
        .route("/login", post(login))
        .with_state(Arc::clone(&app_state))
}

#[debug_handler]
async fn find(State(state): State<Arc<AppState>>, Query(params): Query<GetUsers>) -> impl IntoResponse {
    let conn = &mut state.pool.acquire().await.unwrap();
    let user_list = get_users(conn, params.name).await.unwrap();
    let mut str = String::new();
    for u in user_list {
        str.push_str(&format!("username: {}\ncreated: {}\n", u.name, u.created.format("%Y-%m-%d %H:%M:%S")));
    }
    Html(str)
}

#[debug_handler]
async fn create(State(state): State<Arc<AppState>>, Json(payload): Json<CreatePayload>) -> Result<Json<Value>> {
    let conn = &mut state.pool.acquire().await?;
    let u = create_user(conn, payload.username.clone(), payload.password.clone()).await?;
    let token = create_token()?;

    let body = Json(json!({
		"result": u,
        "token" : token
	}));

    Ok(body)
}

#[debug_handler]
async fn login(State(state): State<Arc<AppState>>, Json(payload): Json<LoginPayload>) -> Result<Json<Value>> {
    let conn = &mut state.pool.acquire().await.unwrap();
    // let u: String = match verify_user(conn, payload.username, payload.password).await {
    //     Ok(_) => "success".to_owned(),
    //     Err(e) => format!("{e:?}"),
    // };
    let u = verify_user(conn, payload.username, payload.password).await?;
    let token = create_token()?;

    let body = Json(json!({
		"result": u,
        "token" : token
	}));

    Ok(body)
}