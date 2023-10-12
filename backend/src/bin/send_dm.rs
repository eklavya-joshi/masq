use std::io::stdin;

use backend::{
    database::database::establish_connection,
    api::message::{create_message, send_message}
};
use uuid::Uuid;


#[tokio::main]
async fn main() {
    
    let connection = &mut establish_connection().await;

    let mut content = String::new();
    let mut sender_id = String::new();
    let mut receiver_id = String::new();

    println!("Send dm:");
    println!("\tcontent:");
    stdin().read_line(&mut content).unwrap();
    let content = content.trim_end().to_string();
    println!("\tyour id:");
    stdin().read_line(&mut sender_id).unwrap();
    let sender_id = sender_id.trim_end().to_string();
    println!("\treceiver id:");
    stdin().read_line(&mut receiver_id).unwrap();
    let receiver_id = receiver_id.trim_end().to_string();

    let message_id = create_message(connection, Uuid::parse_str(&sender_id).ok().unwrap(), content).await;
    send_message(connection, message_id.unwrap(), Uuid::parse_str(&receiver_id).ok().unwrap()).await.ok();
}