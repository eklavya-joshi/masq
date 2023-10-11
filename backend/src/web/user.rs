use std::sync::Arc;

use axum::{Router, routing::get, extract::{State, Query}, response::{IntoResponse, Html}};
use diesel::{PgConnection, r2d2::{ConnectionManager, Pool}};
use serde::Deserialize;

use crate::{
  database::database::get_connection_pool,
  api::user::get_users,
};

use super::{
    state::AppState
};

#[derive(Deserialize)]
struct GetUsers {
  name: String,
  n: u32
}

pub async fn user_router(app_state: Arc<AppState>) -> Router {

    Router::new()
        .route("/user", get(get_user))
        .with_state(app_state)
}

async fn get_user(State(state): State<Arc<AppState>>, Query(params): Query<GetUsers>) -> impl IntoResponse {
    let user_list = get_users(&mut state.pool.get().unwrap(), params.name, params.n);
    let mut str = String::new();
    for u in user_list {
        str.push_str(&format!("{:?}\n", u));
    }
    Html(str)
}