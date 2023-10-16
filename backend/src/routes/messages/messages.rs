use axum::{extract::{State, Query}, Json, Extension};
use axum_macros::debug_handler;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    api::message::{self},
    routes::error::{Result, log}
};

use super::models::*;


#[debug_handler]
pub async fn create_dm(
    Extension(user): Extension<Uuid>,
    State(pool): State<PgPool>, 
    Json(payload): Json<CreateDmPayload>,
) -> Result<Json<CreateDmResponse>> {
    println!("->> {:<18} - {}", "HANDLER", "/create/dm");

    let conn = &mut pool.acquire().await?;
    let dm = message::create_dm(conn, user, &payload.target).await?;

    log(Json(CreateDmResponse { dm }), "/create/dm")
}

#[debug_handler]
pub async fn find_inboxes(
    Extension(user): Extension<Uuid>,
    State(pool): State<PgPool>, 
    Query(_params): Query<GetInboxes>,
) -> Result<Json<FindInboxResponse>> {
    println!("->> {:<18} - {}", "HANDLER", "/find/inbox");

    let conn = &mut pool.acquire().await?;
    let inboxes = message::find_inboxes(conn, user).await?;

    log(Json(FindInboxResponse {inboxes}), "/find/inbox")
}

#[debug_handler]
pub async fn find_messages(
    State(pool): State<PgPool>, 
    Query(params): Query<GetMessages>,
) -> Result<Json<FindMessagesResponse>> {
    println!("->> {:<18} - {}", "HANDLER", "/find/messages");

    let conn = &mut pool.acquire().await?;
    let messages = message::find_messages(conn, params.inbox).await?;

    log(Json(FindMessagesResponse {messages}), "/find/messages")
}

#[debug_handler]
pub async fn send_message(
    Extension(user): Extension<Uuid>,
    State(pool): State<PgPool>, 
    Json(payload): Json<SendMessagePayload>,
) -> Result<Json<SendMessageResponse>> {
    println!("->> {:<18} - {}", "HANDLER", "/send/message");

    let conn = &mut pool.acquire().await?;
    let message = message::send_message(conn, user, payload.inbox, &payload.content).await?;

    log(Json(SendMessageResponse {message}), "/send/message")
}