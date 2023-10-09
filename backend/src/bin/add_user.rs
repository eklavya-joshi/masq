use backend::{*, models::User};
use diesel::{PgConnection, SelectableHelper, RunQueryDsl};
use std::io::{stdin, Read};
use chrono::Utc;

fn main() {
    let connection = &mut database::establish_connection();

    // let mut name = String::new();
    // let mut salt = String::new();
    // let mut hash = String::new();

    create_user(connection);
}

pub fn create_user(conn: &mut PgConnection) -> User {
    use crate::schema::users;

    let new_user = User 
    { 
        id: 1, 
        name: "BoatyMcBoatFace".to_owned(), 
        salt: Some("salty".to_owned()), 
        pass: "password".to_owned(), 
        created: Utc::now().naive_local(), 
        active: true  
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}