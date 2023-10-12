use std::cmp::min;
use chrono::{Utc, NaiveDateTime};
use rand::Rng;
use sqlx::{PgConnection, query};
use uuid::Uuid;
use bcrypt::hash_with_salt;

use crate::{
    database::schema::User,
    api::error::{Error, Result},
};

#[derive(Debug)]
pub struct UserInfo {
    pub name: String,
    pub tag: i16,
    pub created: NaiveDateTime
}

pub fn show_user(name: String, tag: i16) -> String {
    format!("{name}#{:04}", tag)
}

pub async fn create_user(conn: &mut PgConnection, name_input: String, pass_input: String) -> Result<User> {

    let user_id: Uuid = Uuid::new_v4();

    let existing_usernames = query!(
        r#"SELECT COUNT(*) FROM Users WHERE name=$1"#,
        name_input
    )
    .fetch_one(conn.as_mut())
    .await?;

    if existing_usernames.count.unwrap() >= 9998 {
        return Err(Error::UsernameNotAvailable);
    }

    let user_salt = rand::thread_rng().gen::<[u8; 16]>();
    let pass_hash = hash_with_salt(pass_input, bcrypt::DEFAULT_COST, user_salt).unwrap().to_string();

    let new_user = User 
    { 
        id: user_id, 
        name: name_input.to_owned(),
        tag: (existing_usernames.count.unwrap() + 1).try_into().unwrap(),
        salt: Some(hex::encode(user_salt)), 
        pass: pass_hash, 
        created: Utc::now().naive_local(), 
        active: true  
    };

    query!(
        r#"INSERT INTO Users(id, name, tag, salt, pass, created)
        VALUES ($1, $2, $3, $4, $5, $6)"#,
        new_user.id,
        new_user.name,
        new_user.tag,
        new_user.salt,
        new_user.pass,
        new_user.created
    )
    .execute(conn)
    .await?;

    return Ok(new_user);

}

pub async fn get_users(conn: &mut PgConnection, user_name: String, n: u32) -> Result<Vec<UserInfo>> {

    let existing_usernames = query!(
        r#"SELECT name, tag, created FROM Users WHERE name iLIKE $1"#,
        format!("%{user_name}%")
    )
    .fetch_all(conn.as_mut())
    .await?;

    if existing_usernames.len() == 0 {
        return Ok(vec![]);
    }

    let mut user_list: Vec<UserInfo> = vec![];
    for user in existing_usernames {
        user_list.push(UserInfo {name: user.name, tag: user.tag, created: user.created});
    }

    Ok(user_list)
}

pub async fn remove_user(conn: &mut PgConnection, user_id: String) -> Result<bool> {
    query!(
        r#"DELETE FROM Users WHERE id=$1"#,
        Uuid::parse_str(&user_id).ok().unwrap()
    )
    .execute(conn)
    .await
    .and_then(|_| Ok(true))
    .map_err(|e| e.into())
}

pub async fn verify_user(conn: &mut PgConnection, user_name: String, user_tag: i16, user_pass: String) -> Result<bool> {

    let user = query!(
        r#"SELECT * FROM Users WHERE name=$1 AND tag=$2"#,
        user_name,
        user_tag
    )
    .fetch_one(conn.as_mut())
    .await
    .or(Err(Error::UserNotFound))?;

    let user_salt: [u8 ; 16] = hex::decode(&user.salt.unwrap()).unwrap().try_into().unwrap();
            let target_hash = user.pass;

            let new_hash = hash_with_salt(user_pass, bcrypt::DEFAULT_COST, user_salt).unwrap().to_string();

            return Ok(new_hash.eq(&target_hash));

}