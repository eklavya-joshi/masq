use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub salt: Option<String>,
    pub pass: String,
    pub created: NaiveDateTime,
    pub active: bool,
    pub token: Option<String>
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Inbox {
    pub id: Uuid,
    pub created: NaiveDateTime,
    pub active: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InboxRecipients {
    pub inbox: Uuid,
    pub recipient: Uuid
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub id: Uuid,
    pub author: Uuid,
    pub inbox: Uuid,
    pub content: String,
    pub created: NaiveDateTime
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageRecipient {
    pub id: Uuid,
    pub message_id: Uuid,
    pub recipient: Option<Uuid>,
    pub recipient_group: Option<Uuid>
}