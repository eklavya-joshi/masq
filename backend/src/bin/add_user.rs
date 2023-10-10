use std::io::stdin;
use backend::{
    database::database::establish_connection,
    api::user::create_user
};

fn main() {
    let connection = &mut establish_connection();

    let mut name = String::new();
    let mut pass = String::new();

    println!("Create a user:");
    println!("\tname:");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim_end().to_string();

    println!("\tpass:");
    stdin().read_line(&mut pass).unwrap();
    let pass = pass.trim_end().to_string();

    create_user(connection, name, pass);

    println!("user created");
}