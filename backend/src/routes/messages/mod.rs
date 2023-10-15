use axum::{Router, routing::{post, get}};
use crate::routes::messages::messages::{create, find_inbox, find_message, send};

use super::AppState;

pub mod messages;

pub async fn messages_router(app_state: AppState) -> Router {

    Router::new()
        .route("/create/dm", post(create))
        .route("/find/inbox", get(find_inbox))
        .route("/find/messages", get(find_message))
        .route("/send/message", post(send))
        .with_state(app_state.clone())

}