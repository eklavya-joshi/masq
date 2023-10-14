use chrono::Utc;
use sqlx::{PgConnection, query};
use uuid::Uuid;

use crate::{
    database::schema::{Message, Inbox}, 
    api::error::{Error, Result},
};

pub async fn send_message(conn: &mut PgConnection, author: Uuid, inbox: Uuid, content: &str) -> Result<Uuid> {

    query!(
        r#"SELECT * FROM Users WHERE id=$1"#, 
        author)
        .fetch_one(conn.as_mut())
        .await
        .or(Err(Error::UserNotFound(author.to_string())))?;

    query!(
        r#"SELECT * FROM Inbox WHERE id=$1"#, 
        inbox)
        .fetch_one(conn.as_mut())
        .await
        .or(Err(Error::InboxNotFound(inbox.to_string())))?;

    let msg_id = Uuid::new_v4();

    let new_message = Message {
        id: msg_id,
        author,
        inbox,
        content: content.to_string(),
        created: Utc::now().naive_local(),
    };

    query!(
        r#"INSERT INTO Messages(id, author, inbox, content, created)
        VALUES ($1, $2, $3, $4, $5)"#,
        new_message.id,
        new_message.author,
        new_message.inbox,
        new_message.content,
        new_message.created
    )
    .execute(conn)
    .await?;

    Ok(msg_id)
}

pub async fn create_dm(conn: &mut PgConnection, id_1: Uuid, id_2: Uuid) -> Result<Uuid> {

    query!(
        r#"SELECT * FROM Users WHERE id=$1"#,
        id_1)
        .fetch_one(conn.as_mut())
        .await
        .or(Err(Error::UserNotFound(id_1.to_string())))?;

    query!(
        r#"SELECT * FROM Users WHERE id=$1"#,
        id_2)
        .fetch_one(conn.as_mut())
        .await
        .or(Err(Error::UserNotFound(id_2.to_string())))?;

    let inbox: Inbox = Inbox {
        id: Uuid::new_v4(),
        created: Utc::now().naive_local(),
        active: true
    };

    query!(
        r#"INSERT INTO Inbox(id, created)
        VALUES($1, $2)"#,
        inbox.id,
        inbox.created)
    .execute(conn.as_mut())
    .await?;

    query!(
        r#"INSERT INTO InboxRecipients(inbox, recipient)
        VALUES($1, $2)"#,
        inbox.id,
        id_1)
    .execute(conn.as_mut())
    .await?;

    query!(
        r#"INSERT INTO InboxRecipients(inbox, recipient)
        VALUES($1, $2)"#,
        inbox.id,
        id_2)
    .execute(conn)
    .await?;

    Ok(inbox.id)
}

// pub async fn send_message(conn: &mut PgConnection, message: Uuid, receiver: Uuid) -> Result<Uuid> {

//     query!(
//         r#"SELECT * FROM Users WHERE id=$1"#, 
//         receiver)
//         .fetch_one(conn.as_mut())
//         .await
//         .or(Err(Error::UserNotFound(receiver.to_string())))?;

//     query!(
//         r#"SELECT * FROM Messages WHERE id=$1"#, 
//         message)
//         .fetch_one(conn.as_mut())
//         .await
//         .or(Err(Error::MessageNotFound(message.to_string())))?;
    
//     println!("Sending message");

//     let new_recipient = MessageRecipient {
//         id: Uuid::new_v4(),
//         message_id: message,
//         recipient: Some(receiver),
//         recipient_group: None,
//     };

//     // query!(
//     //     r#"INSERT INTO MessageRecipients(id, message_id, recipient, recipient_group)
//     //     VALUES ($1, $2, $3, $4)"#,
//     //     new_recipient.id,
//     //     new_recipient.message_id,
//     //     new_recipient.recipient,
//     //     new_recipient.recipient_group
//     // )
//     // .execute(conn)
//     // .await?;

//     Ok(new_recipient.id)
    
// }