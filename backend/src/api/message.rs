use chrono::Utc;
use diesel::result::Error;
use diesel::{PgConnection, SelectableHelper, RunQueryDsl};
use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    models::{User, Message, MessageRecipient}, 
    schema
};

pub fn create_message(conn: &mut PgConnection, author_id: Uuid, content_str: String) -> Result<Uuid, Error> {
    use schema::users::dsl::*;
    use schema::messages::dsl::*;

    match users
        .find(author_id)
        .select(User::as_select())
        .load(conn) {
            Ok(_) => {},
            Err(e) => {
                println!("Error: {:?}", e);
                e;
            },
        };

    let msg_id = Uuid::new_v4();

    let new_message = Message {
        id: msg_id,
        author: author_id,
        content: content_str,
        created: Utc::now().naive_local(),
    };

    diesel::insert_into(messages)
        .values(&new_message)
        .returning(Message::as_returning())
        .get_result(conn)
        .expect("Couldn't create message");

    return Ok(msg_id);
}

pub fn send_message(conn: &mut PgConnection, msg_id: Uuid, receiver_id: Uuid) {
    use schema::users::dsl::*;
    use schema::messages::dsl::*;
    use schema::message_recipients::dsl::*;

    match users
        .find(receiver_id)
        .select(User::as_select())
        .load(conn) {
            Ok(_) => {},
            Err(e) => {
                println!("Error: {:?}", e);
                return;
            },
        };

    let msg = match messages
        .find(msg_id)
        .select(Message::as_select())
        .load(conn) {
            Ok(msg) => msg,
            Err(e) => {
                println!("Error: {:?}", e);
                return;
            },
        };
    
    println!("Sending message");

    let new_recipient = MessageRecipient {
        id: Uuid::new_v4(),
        message_id: msg_id,
        recipient: Some(receiver_id),
        recipient_group: None,
    };

    diesel::insert_into(message_recipients)
        .values(&new_recipient)
        .returning(MessageRecipient::as_returning())
        .get_result(conn)
        .expect("Couldn't send message");
    
}