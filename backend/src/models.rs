use chrono::{NaiveDateTime};
use uuid::{Uuid};

use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub salt: Option<String>,
    pub pass: String,
    pub created: NaiveDateTime,
    pub active: bool,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::groups)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub created: NaiveDateTime,
    pub active: bool,
}
