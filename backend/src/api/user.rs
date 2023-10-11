use std::cmp::min;
use chrono::{Utc, NaiveDateTime};
use rand::Rng;
use uuid::Uuid;
use diesel::result::Error;
use bcrypt::hash_with_salt;
use diesel::{PgConnection, SelectableHelper, RunQueryDsl};
use diesel::prelude::*;

use crate::{models::User, schema};

#[derive(Debug)]
pub struct UserInfo {
    pub name: String,
    pub tag: i16,
    pub created: NaiveDateTime
}

pub fn show_user(name: String, tag: i16) -> String {
    format!("{name}#{:04}", tag)
}

pub fn create_user(conn: &mut PgConnection, name_input: String, pass_input: String) -> User {
    use schema::users::dsl::*;

    let user_id: Uuid = Uuid::new_v4();

    let existing_usernames: i64 = users
        .filter(name.eq(&name_input))
        .count()
        .get_result(conn)
        .expect("Couldn't reach database");

    if existing_usernames >= 9998 {
        todo!();
    }

    let user_salt = rand::thread_rng().gen::<[u8; 16]>();
    let pass_hash = hash_with_salt(pass_input, bcrypt::DEFAULT_COST, user_salt).unwrap().to_string();

    let new_user = User 
    { 
        id: user_id, 
        name: name_input.to_owned(),
        tag: (existing_usernames + 1).try_into().unwrap(),
        salt: Some(hex::encode(user_salt)), 
        pass: pass_hash, 
        created: Utc::now().naive_local(), 
        active: true  
    };

    diesel::insert_into(users)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Couldn't insert user")
}

pub fn get_users(pg: &mut PgConnection, user_name: String, n: u32) -> Vec<UserInfo> {
    use schema::users::dsl::*;

    let vec = users
        .filter(name.ilike(format!("%{user_name}%")))
        .limit(min(n.into(), 25))
        .select(User::as_select())
        .load(pg)
        .expect("Error finding users");

    if vec.len() == 0 {
        return vec![];
    }

    let mut userList: Vec<UserInfo> = vec![];
    for user in vec {
        userList.push(UserInfo {name: user.name.clone(), tag: user.tag.clone(), created: user.created.clone()});
    }

    userList
}

pub fn remove_user(pg: &mut PgConnection, user_id: String) -> Result<usize, Error> {
    use schema::users::dsl::*;
    
    diesel::delete(users
        .find(Uuid::parse_str(&user_id).ok().unwrap()))
        .execute(pg)
}

pub fn verify_user(pg: &mut PgConnection, user_name: String, user_tag: i16, user_pass: String) -> bool {
    use schema::users::dsl::*;

    let vec = users
        .filter(name.eq(&user_name).and(tag.eq(user_tag)))
        .limit(1)
        .select(User::as_select())
        .load(pg)
        .expect("Error finding user");

    if vec.len() == 0 {
        println!("No users with this username found");
        return false;
    }

    let user_salt: [u8 ; 16] = hex::decode(&vec[0].salt.clone().unwrap().clone()).unwrap().try_into().unwrap();
    let target_hash = &vec[0].pass.clone();

    let new_hash = hash_with_salt(user_pass, bcrypt::DEFAULT_COST, user_salt).unwrap().to_string();

    new_hash.eq(target_hash)

}