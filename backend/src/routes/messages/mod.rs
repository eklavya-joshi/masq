use axum::{Router, routing::{post, get}};
use crate::routes::messages::messages::{create_dm, find_inboxes, find_messages, send_message};

use super::AppState;

pub mod messages;
pub mod models;

pub async fn messages_router(app_state: AppState) -> Router {

    Router::new()
        .route("/new", post(create_dm))
        .route("/inbox", get(find_inboxes))
        .route("/find/messages", get(find_messages))
        .route("/send/message", post(send_message))
        .with_state(app_state.clone())

}