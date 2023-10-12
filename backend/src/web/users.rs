use std::sync::Arc;
use axum::{Router, routing::{get, post}, extract::{State, Query}, response::{IntoResponse, Html}, Json};
use axum_macros::{debug_handler};
use serde::{Deserialize};
use serde_json::{json, Value};
use sqlx::{Postgres, Pool};

use crate::{
    error::Result,
    api::user::{get_users, create_user, verify_user, show_user},
};

use super::AppState;

#[derive(Deserialize)]
struct GetUsers {
    name: String,
    n: u32
}

#[derive(Debug, Deserialize)]
struct CreatePayload {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    tag: i16,
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
    let user_list = get_users(&mut state.pool.acquire().await.unwrap(), params.name, params.n).await;
    let mut str = String::new();
    for u in user_list {
        str.push_str(&format!("username: {}\ncreated: {}\n", show_user(u.name, u.tag), u.created.format("%Y-%m-%d %H:%M:%S")));
    }
    Html(str)
}

async fn create(State(state): State<Arc<AppState>>, Json(payload): Json<CreatePayload>) -> Json<Value> {
    let u = create_user(&mut state.pool.acquire().await.unwrap(), payload.username.clone(), payload.password.clone()).await;

    let created = u.created.clone().format("%Y-%m-%d %H:%M:%S").to_string();

    let body = Json(json!({
		"result": {
			"created": created
		}
	}));

    body
}

async fn login(State(state): State<Arc<AppState>>, Json(payload): Json<LoginPayload>) -> Json<Value> {
    let u = verify_user(
        &mut state.pool.acquire().await.unwrap(), 
        payload.username.clone(), 
        payload.tag, 
        payload.password.clone())
        .await;

    let body = Json(json!({
		"result": {
			"logged in": u
		}
	}));

    body
}