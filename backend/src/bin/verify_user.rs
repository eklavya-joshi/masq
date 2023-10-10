use std::io::stdin;
use backend::{
    database::database::establish_connection,
    api::user::verify_user
};

fn main() {
    let connection = &mut establish_connection();

    let mut username = String::new();
    let mut password = String::new();

    println!("Find users:");
    println!("\tusername:");
    stdin().read_line(&mut username).unwrap();
    let username = username.trim_end().to_string();
    println!("\tpassword:");
    stdin().read_line(&mut password).unwrap();
    let password = password.trim_end().to_string();

    verify_user(connection, username, password);

}