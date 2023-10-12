use std::io::stdin;
use backend::{
    database::establish_connection,
    api::user::get_users
};

#[tokio::main]
async fn main() {
    let connection = &mut establish_connection().await;

    let mut name = String::new();
    let mut n_str = String::new();

    println!("Find users:");
    println!("\tname:");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim_end().to_string();
    println!("\tnumber of users:");
    stdin().read_line(&mut n_str).unwrap();
    let n = n_str
        .trim_end()
        .parse::<u32>()
        .ok()
        .unwrap();

    println!("{:?}", get_users(connection, name, n).await);
}