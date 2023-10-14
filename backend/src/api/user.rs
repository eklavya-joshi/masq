use chrono::{Utc, NaiveDateTime};
use serde::Serialize;
use sqlx::{PgConnection, query};
use uuid::Uuid;

use crate::{
    database::schema::User,
    api::error::{Error, Result},
    utils::pwd::{encrypt, decrypt}, middleware::jwt::create_token
};

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub name: String,
    pub created: NaiveDateTime
}

pub async fn create_user(conn: &mut PgConnection, name: &str, pass: &str) -> Result<String> {

    let user_id: Uuid = Uuid::new_v4();

    let _ = query!(
        r#"SELECT COUNT(*) FROM Users WHERE name=$1"#,
        name
    )
    .fetch_one(conn.as_mut())
    .await
    .or(Err(Error::UsernameNotAvailable));

    let crypt = encrypt(&pass).await;

    let new_user = User 
    { 
        id: user_id, 
        name: name.to_owned(),
        salt: Some(crypt.salt), 
        pass: crypt.hash, 
        created: Utc::now().naive_local(), 
        active: true,
        token: None
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

    Ok(token)

}

pub async fn get_users(conn: &mut PgConnection, name: &str) -> Result<Vec<UserInfo>> {

    let existing_usernames = query!(
        r#"SELECT name, created FROM Users WHERE name iLIKE $1 LIMIT $2"#,
        format!("%{name}%"),
        25
    )
    .fetch_all(conn.as_mut())
    .await?;

    if existing_usernames.is_empty() {
        return Ok(vec![]);
    }

    let mut user_list: Vec<UserInfo> = vec![];
    for user in existing_usernames {
        user_list.push(UserInfo {name: user.name, created: user.created});
    }

    Ok(user_list)
}

pub async fn remove_user(conn: &mut PgConnection, id: &str) -> Result<bool> {
    query!(
        r#"DELETE FROM Users WHERE id=$1"#,
        Uuid::parse_str(&id).ok().unwrap()
    )
    .execute(conn)
    .await.map(|_| true)
    .map_err(|e| e.into())
}

pub async fn verify_user(conn: &mut PgConnection, name: &str, pass: &str) -> Result<String> {

    let user = query!(
        r#"SELECT * FROM Users WHERE name=$1"#,
        name,
    )
    .fetch_one(conn.as_mut())
    .await
    .or(Err(Error::UserNotFound))?;

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

    Ok(token)
}

pub async fn logout_user(conn: &mut PgConnection, name: String) -> Result<bool> {

    query!(
        r#"UPDATE Users SET token = NULL WHERE name = $1"#,
        name
    )
    .execute(conn)
    .await?;

    Ok(true)
}