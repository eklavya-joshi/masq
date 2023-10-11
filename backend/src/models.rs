use chrono::NaiveDateTime;
use uuid::Uuid;

use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub tag: i16,
    pub salt: Option<String>,
    pub pass: String,
    pub created: NaiveDateTime,
    pub active: bool,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::groups)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub created: NaiveDateTime,
    pub active: bool,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Message {
    pub id: Uuid,
    pub author: Uuid,
    pub content: String,
    pub created: NaiveDateTime
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::message_recipients)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MessageRecipient {
    pub id: Uuid,
    pub message_id: Uuid,
    pub recipient: Option<Uuid>,
    pub recipient_group: Option<Uuid>
}