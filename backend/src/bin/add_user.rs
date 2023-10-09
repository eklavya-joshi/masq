use backend::{*, models::User};
use diesel::{PgConnection, SelectableHelper, RunQueryDsl};
use std::io::{stdin, Read};
use chrono::{Utc};
use uuid::{Uuid};

fn main() {
    let connection = &mut database::establish_connection();

    let mut name = String::new();
    let mut pass = String::new();
    let mut salt = String::new();

    println!("Create a user:");
    println!("\tname:");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim_end();

    println!("\tpass:");
    stdin().read_line(&mut pass).unwrap();
    let pass = pass.trim_end();

    println!("\tsalt:");
    stdin().read_line(&mut salt).unwrap();
    let salt = salt.trim_end();

    create_user(connection, name, pass, salt);
}

pub fn create_user(conn: &mut PgConnection, name: &str, pass: &str, salt: &str) -> User {
    use crate::schema::users;

    let new_user = User 
    { 
        id: Uuid::new_v4(), 
        name: name.to_owned(), 
        salt: Some(salt.to_owned()), 
        pass: pass.to_owned(), 
        created: Utc::now().naive_local(), 
        active: true  
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}