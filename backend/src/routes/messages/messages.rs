use axum::{extract::{State, Query}, Json, Extension};
use axum_macros::debug_handler;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    api::message::{self},
    routes::error::Result
};

use super::models::*;


#[debug_handler]
pub async fn create_dm(
    Extension(user): Extension<Uuid>,
    State(pool): State<PgPool>, 
    Json(payload): Json<CreateDmPayload>,
) -> Result<Json<CreateDmResponse>> {
    let conn = &mut pool.acquire().await?;
    let dm = message::create_dm(conn, user, &payload.target).await?;

    Ok(Json(CreateDmResponse { dm }))
}

#[debug_handler]
pub async fn find_inboxes(
    Extension(user): Extension<Uuid>,
    State(pool): State<PgPool>, 
    Query(_params): Query<GetInboxes>,
) -> Result<Json<FindInboxResponse>> {
    let conn = &mut pool.acquire().await?;
    let inboxes = message::find_inboxes(conn, user).await?;

    Ok(Json(FindInboxResponse {inboxes}))
}

#[debug_handler]
pub async fn find_messages(
    State(pool): State<PgPool>, 
    Query(params): Query<GetMessages>,
) -> Result<Json<FindMessagesResponse>> {
    let conn = &mut pool.acquire().await?;
    let messages = message::find_messages(conn, params.inbox).await?;

    Ok(Json(FindMessagesResponse {messages}))
}

#[debug_handler]
pub async fn send_message(
    Extension(user): Extension<Uuid>,
    State(pool): State<PgPool>, 
    Json(payload): Json<SendMessagePayload>,
) -> Result<Json<SendMessageResponse>> {
    let conn = &mut pool.acquire().await?;
    let message = message::send_message(conn, user, payload.inbox, &payload.content).await?;

    Ok(Json(SendMessageResponse {message}))
}