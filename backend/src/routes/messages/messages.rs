use axum::{extract::{State, Query}, Json, Extension};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    api::message::{create_dm, find_inboxes, find_messages, send_message, InboxInfo},
    routes::error::Result, database::schema::Message, 
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDmResponse {
    dm: Uuid
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageResponse {
    message: Uuid
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindInboxResponse {
    inboxes: Vec<InboxInfo>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindMessagesResponse {
    messages: Vec<Message>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDmPayload {
    target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessagePayload {
    inbox: Uuid,
    content: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInboxes {

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMessages {
    inbox: Uuid
}


#[debug_handler]
pub async fn create(
    Extension(user): Extension<Uuid>,
    State(pool): State<PgPool>, 
    Json(payload): Json<CreateDmPayload>,
) -> Result<Json<CreateDmResponse>> {
    let conn = &mut pool.acquire().await?;
    let dm = create_dm(conn, user, &payload.target).await?;

    Ok(Json(CreateDmResponse { dm }))
}

#[debug_handler]
pub async fn find_inbox(
    Extension(user): Extension<Uuid>,
    State(pool): State<PgPool>, 
    Query(_params): Query<GetInboxes>,
) -> Result<Json<FindInboxResponse>> {
    let conn = &mut pool.acquire().await?;
    let inboxes = find_inboxes(conn, user).await?;

    Ok(Json(FindInboxResponse {inboxes}))
}

#[debug_handler]
pub async fn find_message(
    State(pool): State<PgPool>, 
    Query(params): Query<GetMessages>,
) -> Result<Json<FindMessagesResponse>> {
    let conn = &mut pool.acquire().await?;
    let messages = find_messages(conn, params.inbox).await?;

    Ok(Json(FindMessagesResponse {messages}))
}

#[debug_handler]
pub async fn send(
    Extension(user): Extension<Uuid>,
    State(pool): State<PgPool>, 
    Json(payload): Json<SendMessagePayload>,
) -> Result<Json<SendMessageResponse>> {
    let conn = &mut pool.acquire().await?;
    let message = send_message(conn, user, payload.inbox, &payload.content).await?;

    Ok(Json(SendMessageResponse {message}))
}