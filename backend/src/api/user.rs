use std::cmp::min;
use chrono::Utc;
use rand::Rng;
use uuid::Uuid;
use diesel::result::Error;
use bcrypt::{hash, hash_with_salt};
use diesel::{PgConnection, SelectableHelper, RunQueryDsl};
use diesel::prelude::*;

use crate::{models::User, schema};

pub fn create_user(conn: &mut PgConnection, name_input: String, pass_input: String) -> User {
    use schema::users::dsl::*;

    let user_id: Uuid = Uuid::new_v4();

    let user_salt = rand::thread_rng().gen::<[u8; 16]>();
    let pass_hash = hash_with_salt(pass_input, bcrypt::DEFAULT_COST, user_salt).unwrap().to_string();

    let new_user = User 
    { 
        id: user_id, 
        name: name_input.to_owned(), 
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

pub fn get_users(pg: &mut PgConnection, user_name: String, n: u32) -> String {
    use schema::users::dsl::*;

    let vec = users
        .filter(name.ilike(format!("%{user_name}%")))
        .limit(min(n.into(), 25))
        .select(User::as_select())
        .load(pg)
        .expect("Error finding users");

    if vec.len() == 0 {
        return "No users found".to_owned();
    }

    let mut str = String::new();
    for user in vec {
        str.push_str(&format!("user: {}\npass: {}\nid: {}\n", user.name.clone(), user.pass.clone(), user.id.clone())) 
    }
    str
}

pub fn remove_user(pg: &mut PgConnection, user_id: String) -> Result<usize, Error> {
    use schema::users::dsl::*;
    
    diesel::delete(users
        .find(Uuid::parse_str(&user_id).ok().unwrap()))
        .execute(pg)
}

pub fn verify_user(pg: &mut PgConnection, user_name: String, user_pass: String) {
    use schema::users::dsl::*;

    let vec = users
        .filter(name.eq(&user_name))
        .limit(1)
        .select(User::as_select())
        .load(pg)
        .expect("Error finding users");

    if vec.len() == 0 {
        println!("No users with this username found");
        return;
    }

    let user_salt: [u8 ; 16] = hex::decode(&vec[0].salt.clone().unwrap().clone()).unwrap().try_into().unwrap();
    let target_hash = &vec[0].pass.clone();

    let new_hash = hash_with_salt(user_pass, bcrypt::DEFAULT_COST, user_salt).unwrap().to_string();

    if new_hash.eq(target_hash) {
        println!("Correct password");
        return;
    }

    println!("Incorrect password");

}