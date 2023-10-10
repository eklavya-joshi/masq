use std::cmp::min;
use chrono::Utc;
use diesel::result::Error;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;
use uuid::Uuid;
use diesel::{PgConnection, SelectableHelper, RunQueryDsl};
use diesel::prelude::*;

use crate::{models::User, schema};

pub fn create_user(conn: &mut PgConnection, name_input: String, pass_input: String) -> User {
    use schema::users::dsl::*;

    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
    let n_iter = NonZeroU32::new(100_000).unwrap();
    let rng = rand::SystemRandom::new();

    let mut salt_gen = [0u8; CREDENTIAL_LEN];
    rng.fill(&mut salt_gen).ok();

    let mut hash_gen = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt_gen,
        pass_input.as_bytes(),
        &mut hash_gen,
    );


    let salt_hex = hex::encode(&salt_gen);
    let hash_hex = hex::encode(&hash_gen);

    let new_user = User 
    { 
        id: Uuid::new_v4(), 
        name: name_input.to_owned(), 
        salt: Some(salt_hex), 
        pass: hash_hex, 
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

pub fn login(pg: &mut PgConnection, user_name: String, user_pass: String) {
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

    let n_iter = NonZeroU32::new(100_000).unwrap();
    let user_hash: &[u8] = &hex::decode(&vec[0].pass.clone()).unwrap();
    let user_salt: &[u8] = &hex::decode(&vec[0].salt.clone().unwrap()).unwrap();

    let verify_pass = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        user_salt,
        user_pass.as_bytes(),
        &user_hash,
    );

    match verify_pass {
        Ok(_) => println!("Correct password"),
        Err(_) => println!("Wrong password"),
    }
}