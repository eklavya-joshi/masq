use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

// use diesel::prelude::*;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub tag: i16,
    pub salt: Option<String>,
    pub pass: String,
    pub created: NaiveDateTime,
    pub active: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub created: NaiveDateTime,
    pub active: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub id: Uuid,
    pub author: Uuid,
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