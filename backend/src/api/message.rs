use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, PgConnection};
use uuid::Uuid;

use crate::database::schema::{Inbox, Message};

use super::{Error, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboxInfo {
    pub inbox: Uuid,
}

pub async fn send_message(
    conn: &mut PgConnection,
    author: Uuid,
    inbox: Uuid,
    content: &str,
) -> Result<Uuid> {
    let author_name = query!(r#"SELECT name FROM Users WHERE id=$1"#, author)
        .fetch_one(conn.as_mut())
        .await
        .or(Err(Error::UserNotFound(author.to_string())))?;

    query!(r#"SELECT * FROM Inbox WHERE id=$1"#, inbox)
        .fetch_one(conn.as_mut())
        .await
        .or(Err(Error::InboxNotFound(inbox.to_string())))?;

    let msg_id = Uuid::new_v4();

    let new_message = Message {
        id: msg_id,
        author_id: author,
        author_name: author_name.name,
        inbox,
        content: content.to_string(),
        created: Utc::now().naive_local(),
    };

    query!(
        r#"INSERT INTO Messages(id, author_id, author_name, inbox, content, created)
        VALUES ($1, $2, $3, $4, $5, $6)"#,
        new_message.id,
        new_message.author_id,
        new_message.author_name,
        new_message.inbox,
        new_message.content,
        new_message.created
    )
    .execute(conn)
    .await?;

    Ok(msg_id)
}

pub async fn create_dm(conn: &mut PgConnection, id: Uuid, target: &str) -> Result<Uuid> {
    // TODO: Can't create existing DMs

    query!(r#"SELECT * FROM Users WHERE id=$1"#, id)
        .fetch_one(conn.as_mut())
        .await
        .or(Err(Error::UserNotFound(id.to_string())))?;

    let target_id = query!(r#"SELECT * FROM Users WHERE name=$1"#, target)
        .fetch_one(conn.as_mut())
        .await
        .or(Err(Error::UserNotFound(target.to_string())))?
        .id;

    if id == target_id {
        return Err(Error::NoSelfDm);
    }

    let inbox: Inbox = Inbox {
        id: Uuid::new_v4(),
        created: Utc::now().naive_local(),
        active: true,
    };

    query!(
        r#"INSERT INTO Inbox(id, created)
        VALUES($1, $2)"#,
        inbox.id,
        inbox.created
    )
    .execute(conn.as_mut())
    .await?;

    match query!(
        r#"SELECT inbox FROM InboxDmRecipients WHERE 
        (recipient1=$1 AND recipient2=$2) OR
        (recipient1=$2 AND recipient2=$1)"#,
        id,
        target_id
    )
    .fetch_optional(conn.as_mut())
    .await?
    {
        Some(x) => return Err(Error::DMAlreadyExists(x.inbox.to_string())),
        None => {}
    };

    query!(
        r#"INSERT INTO InboxDmRecipients(inbox, recipient1, recipient2)
        VALUES($1, $2, $3)"#,
        inbox.id,
        id,
        target_id
    )
    .execute(conn.as_mut())
    .await?;

    Ok(inbox.id)
}

pub async fn find_inboxes(conn: &mut PgConnection, id: Uuid) -> Result<Vec<InboxInfo>> {
    let mut inboxes: Vec<InboxInfo> = vec![];

    let mut group_inboxes = query_as!(
        InboxInfo,
        r#"SELECT inbox FROM InboxRecipients WHERE recipient=$1"#,
        id
    )
    .fetch_all(conn.as_mut())
    .await?;

    inboxes.append(&mut group_inboxes);

    let mut dm_inboxes = query_as!(
        InboxInfo,
        r#"SELECT inbox FROM InboxDmRecipients WHERE recipient1=$1 OR recipient2=$1"#,
        id
    )
    .fetch_all(conn.as_mut())
    .await?;

    inboxes.append(&mut dm_inboxes);

    Ok(inboxes)
}

pub async fn find_messages(conn: &mut PgConnection, id: Uuid) -> Result<Vec<Message>> {
    let messages = query_as!(Message, r#"SELECT * from Messages WHERE inbox=$1"#, id)
        .fetch_all(conn.as_mut())
        .await?;

    Ok(messages)
}
