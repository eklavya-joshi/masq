use std::io::stdin;
use backend::{
    database::establish_connection,
    api::user::verify_user,
    api::Result
};

#[tokio::main]
async fn main() -> Result<()> {
    let connection = &mut establish_connection().await?;

    let mut username = String::new();
    let mut password = String::new();

    println!("Find users:");
    println!("\tusername:");
    stdin().read_line(&mut username).unwrap();
    let username = username.trim_end().to_string();
    println!("\tpassword:");
    stdin().read_line(&mut password).unwrap();
    let password = password.trim_end().to_string();

    println!("{:?}", verify_user(connection, &username, &password).await?);

    Ok(())

}