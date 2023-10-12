use chrono::Utc;
use sqlx::{PgConnection, query, Error};
use uuid::Uuid;

use crate::{
    models::{Message, MessageRecipient}, 
};

pub async fn create_message(conn: &mut PgConnection, author_id: Uuid, content_str: String) -> Result<Uuid, Error> {

    query!(
        r#"SELECT * FROM Users WHERE id=$1"#, 
        author_id)
        .fetch_one(conn.as_mut())
        .await?;

    let msg_id = Uuid::new_v4();

    let new_message = Message {
        id: msg_id,
        author: author_id,
        content: content_str,
        created: Utc::now().naive_local(),
    };

    query!(
        r#"INSERT INTO Messages(id, author, content, created)
        VALUES ($1, $2, $3, $4)"#,
        new_message.id,
        new_message.author,
        new_message.content,
        new_message.created
    )
    .execute(conn)
    .await?;

    return Ok(msg_id);
}

pub async fn send_message(conn: &mut PgConnection, msg_id: Uuid, receiver_id: Uuid) -> Result<Uuid, Error> {

    query!(
        r#"SELECT * FROM Users WHERE id=$1"#, 
        receiver_id)
        .fetch_one(conn.as_mut())
        .await?;

    query!(
        r#"SELECT * FROM Messages WHERE id=$1"#, 
        msg_id)
        .fetch_one(conn.as_mut())
        .await?;
    
    println!("Sending message");

    let new_recipient = MessageRecipient {
        id: Uuid::new_v4(),
        message_id: msg_id,
        recipient: Some(receiver_id),
        recipient_group: None,
    };

    query!(
        r#"INSERT INTO Message_Recipients(id, message_id, recipient, recipient_group)
        VALUES ($1, $2, $3, $4)"#,
        new_recipient.id,
        new_recipient.message_id,
        new_recipient.recipient,
        new_recipient.recipient_group
    )
    .execute(conn)
    .await?;

    return Ok(new_recipient.id)
    
}