use chrono::{Utc, NaiveDateTime};
use sqlx::{PgConnection, query};
use uuid::Uuid;

use crate::{
    database::schema::User,
    api::error::{Error, Result},
    utils::crypt::{encrypt, decrypt}
};

#[derive(Debug)]
pub struct UserInfo {
    pub name: String,
    pub created: NaiveDateTime
}

pub async fn create_user(conn: &mut PgConnection, name: String, pass: String) -> Result<User> {

    let user_id: Uuid = Uuid::new_v4();

    let existing_usernames = query!(
        r#"SELECT COUNT(*) FROM Users WHERE name=$1"#,
        name
    )
    .fetch_one(conn.as_mut())
    .await?;

    if existing_usernames.count.unwrap() > 0 {
        return Err(Error::UsernameNotAvailable);
    }

    let crypt = encrypt(pass);

    let new_user = User 
    { 
        id: user_id, 
        name: name.to_owned(),
        salt: Some(crypt.salt), 
        pass: crypt.hash, 
        created: Utc::now().naive_local(), 
        active: true  
    };

    query!(
        r#"INSERT INTO Users(id, name, salt, pass, created)
        VALUES ($1, $2, $3, $4, $5)"#,
        new_user.id,
        new_user.name,
        new_user.salt,
        new_user.pass,
        new_user.created
    )
    .execute(conn)
    .await?;

    return Ok(new_user);

}

pub async fn get_users(conn: &mut PgConnection, name: String) -> Result<Vec<UserInfo>> {

    let existing_usernames = query!(
        r#"SELECT name, created FROM Users WHERE name iLIKE $1 LIMIT $2"#,
        format!("%{name}%"),
        25
    )
    .fetch_all(conn.as_mut())
    .await?;

    if existing_usernames.len() == 0 {
        return Ok(vec![]);
    }

    let mut user_list: Vec<UserInfo> = vec![];
    for user in existing_usernames {
        user_list.push(UserInfo {name: user.name, created: user.created});
    }

    Ok(user_list)
}

pub async fn remove_user(conn: &mut PgConnection, id: String) -> Result<bool> {
    query!(
        r#"DELETE FROM Users WHERE id=$1"#,
        Uuid::parse_str(&id).ok().unwrap()
    )
    .execute(conn)
    .await
    .and_then(|_| Ok(true))
    .map_err(|e| e.into())
}

pub async fn verify_user(conn: &mut PgConnection, name: String, pass: String) -> Result<bool> {

    let user = query!(
        r#"SELECT * FROM Users WHERE name=$1"#,
        name,
    )
    .fetch_one(conn.as_mut())
    .await
    .or(Err(Error::UserNotFound))?;

    return Ok(decrypt(user.salt.unwrap(), user.pass, pass));

}