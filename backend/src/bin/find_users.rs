use std::io::stdin;
use backend::{
    database::establish_connection,
    api::user::find_unfiltered,
    api::Result
};

#[tokio::main]
async fn main() -> Result<()> {
    let connection = &mut establish_connection().await?;

    let mut name = String::new();

    println!("Find users:");
    println!("\tname:");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim_end().to_string();

    println!("{:?}", find_unfiltered(connection, &name).await?);

    Ok(())

}