use std::io::stdin;
use backend::{
    database::database::establish_connection,
    api::user::remove_user
};


#[tokio::main]
async fn main() {
    let connection = &mut establish_connection().await;

    let mut id_str = String::new();

    println!("Remove user:");
    println!("\tid:");
    stdin().read_line(&mut id_str).unwrap();
    let id_str = id_str.trim_end().to_string();

    remove_user(connection, id_str).await;// {
    //     Ok(_) => println!("Removed user"),
    //     Err(e) => println!("{:?}", e),
    // }

}