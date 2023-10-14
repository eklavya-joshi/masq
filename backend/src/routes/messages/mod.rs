use axum::{Router, routing::post};
use crate::routes::messages::messages::create;

use super::AppState;

pub mod messages;

pub async fn messages_router(app_state: AppState) -> Router {

    Router::new()
        .route("/create/dm", post(create))
        .with_state(app_state.clone())

}