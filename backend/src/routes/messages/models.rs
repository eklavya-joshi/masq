use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{api::message::InboxInfo, database::schema::Message};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDmResponse {
    pub dm: Uuid
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageResponse {
    pub message: Uuid
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindInboxResponse {
    pub inboxes: Vec<InboxInfo>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindMessagesResponse {
    pub messages: Vec<Message>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDmPayload {
    pub target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessagePayload {
    pub inbox: Uuid,
    pub content: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInboxesQuery {

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMessagesQuery {
    pub inbox: Uuid
}