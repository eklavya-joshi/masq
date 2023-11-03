use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, PgConnection};
use uuid::Uuid;

use crate::{
    api::{Error, Result},
    database::schema::User,
    middleware::jwt::create_token,
    utils::pwd::{decrypt, encrypt},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub name: String,
    pub created: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUserInfo {
    pub id: Uuid,
    pub token: String
}

pub async fn create_user(conn: &mut PgConnection, name: &str, pass: &str) -> Result<AuthUserInfo> {
    let user_id: Uuid = Uuid::new_v4();

    let u = query!(r#"SELECT * FROM Users WHERE name=$1"#, name)
        .fetch_optional(conn.as_mut())
        .await?;

    if u.is_some() {
        return Err(Error::UsernameNotAvailable(name.to_owned()));
    }

    let crypt = encrypt(&pass).await;

    let new_user = User {
        id: user_id,
        name: name.to_owned(),
        salt: Some(crypt.salt),
        pass: crypt.hash,
        created: Utc::now().naive_local(),
        active: true,
        token: None,
    };

    let token = create_token(&new_user.name)?;

    query!(
        r#"INSERT INTO Users(id, name, salt, pass, created, token)
        VALUES ($1, $2, $3, $4, $5, $6)"#,
        new_user.id,
        new_user.name,
        new_user.salt,
        new_user.pass,
        new_user.created,
        token
    )
    .execute(conn)
    .await?;

    Ok(AuthUserInfo { id: new_user.id, token })
}

pub async fn find_users(
    conn: &mut PgConnection,
    name: &str,
    query_user_id: Uuid,
) -> Result<Vec<UserInfo>> {
    let existing_usernames = find_filtered(conn, name, query_user_id).await?;

    if existing_usernames.is_empty() {
        return Ok(vec![]);
    }

    let mut user_list: Vec<UserInfo> = vec![];
    for user in existing_usernames {
        user_list.push(UserInfo {
            name: user.name,
            created: user.created,
        });
    }

    Ok(user_list)
}

pub async fn remove_user(conn: &mut PgConnection, id: &str) -> Result<bool> {
    query!(
        r#"DELETE FROM Users WHERE id=$1"#,
        Uuid::parse_str(&id).ok().unwrap()
    )
    .execute(conn)
    .await
    .map(|_| true)
    .map_err(|e| e.into())
}

pub async fn verify_user(conn: &mut PgConnection, name: &str, pass: &str) -> Result<AuthUserInfo> {
    let user = query!(r#"SELECT * FROM Users WHERE name=$1"#, name,)
        .fetch_one(conn.as_mut())
        .await
        .or(Err(Error::UserNotFound(name.to_string())))?;

    if !decrypt(&user.salt.unwrap(), &user.pass, &pass).await {
        return Err(Error::InvalidPassword);
    }

    let token = create_token(name)?;

    query!(
        r#"UPDATE Users SET token = $1 WHERE name = $2"#,
        token,
        user.name
    )
    .execute(conn)
    .await?;

    Ok(AuthUserInfo { id: user.id, token })
}

pub async fn logout_user(conn: &mut PgConnection, name: String) -> Result<bool> {
    let user = query!(r#"SELECT id FROM Users WHERE name=$1"#, name,)
        .fetch_one(conn.as_mut())
        .await
        .or(Err(Error::UserNotFound(name.to_string())))?;

    query!(r#"UPDATE Users SET token = NULL WHERE id = $1"#, user.id)
        .execute(conn)
        .await?;

    Ok(true)
}

pub async fn find_unfiltered(conn: &mut PgConnection, name: &str) -> Result<Vec<UserInfo>> {
    query_as!(
        UserInfo,
        r#"SELECT name, created FROM Users WHERE name iLIKE $1 LIMIT $2"#,
        format!("%{name}%"),
        25
    )
    .fetch_all(conn.as_mut())
    .await
    .map_err(|e| e.into())
}

pub async fn find_filtered(
    conn: &mut PgConnection,
    name: &str,
    query_user_id: Uuid,
) -> Result<Vec<UserInfo>> {
    query_as!(
        UserInfo,
        r#"SELECT name, created FROM Users WHERE name iLIKE $1 AND id <> $2 LIMIT $3"#,
        format!("%{name}%"),
        query_user_id,
        25
    )
    .fetch_all(conn.as_mut())
    .await
    .map_err(|e| e.into())
}
