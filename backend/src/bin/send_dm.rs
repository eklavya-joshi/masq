use std::io::stdin;

use backend::{
    database::establish_connection,
    api::message::{send_message, create_dm},
    api::Result
};
use uuid::Uuid;


#[tokio::main]
async fn main() -> Result<()> {
    
    let connection = &mut establish_connection().await?;

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

    let dm_id = create_dm(connection, Uuid::parse_str(&sender_id).unwrap(), Uuid::parse_str(&receiver_id).unwrap()).await?;
    let _message_id = send_message(connection, Uuid::parse_str(&sender_id).unwrap(), dm_id, &content).await?;

    Ok(())
}