use backend::{api::user::remove_user, api::Result, database::establish_connection};
use std::io::stdin;

#[tokio::main]
async fn main() -> Result<()> {
    let connection = &mut establish_connection().await?;

    let mut id_str = String::new();

    println!("Remove user:");
    println!("\tid:");
    stdin().read_line(&mut id_str).unwrap();
    let id_str = id_str.trim_end().to_string();

    println!("{:?}", remove_user(connection, &id_str).await?);

    Ok(())
}
