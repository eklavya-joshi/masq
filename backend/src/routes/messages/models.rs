use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::message::{InboxInfo, FilteredMessage};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDmResponse {
    pub dm: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageResponse {
    pub message: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FindInboxResponse {
    pub inboxes: Vec<InboxInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FindMessagesResponse {
    pub messages: Vec<FilteredMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDmPayload {
    pub target: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessagePayload {
    pub inbox: Uuid,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetInboxesQuery {}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetMessagesQuery {
    pub inbox: Uuid,
}
