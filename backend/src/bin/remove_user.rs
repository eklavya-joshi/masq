use std::io::stdin;
use backend::{
    database::database::establish_connection,
    api::user::remove_user
};



fn main() {
    let connection = &mut establish_connection();

    let mut id_str = String::new();

    println!("Remove user:");
    println!("\tid:");
    stdin().read_line(&mut id_str).unwrap();
    let id_str = id_str.trim_end().to_string();

    match remove_user(connection, id_str) {
        Ok(_) => println!("Removed user"),
        Err(e) => println!("{:?}", e),
    }

}