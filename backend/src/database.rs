use bson::Document;
use mongodb::{bson::doc, options::{ClientOptions, ServerApi, ServerApiVersion}, Client, Database, Collection};
use dotenv;

// #[tokio::main]
// async fn main() -> mongodb::error::Result<()> {

//   dotenv::dotenv().ok();

//   let mut client_options =
//     ClientOptions::parse(dotenv::var("DB_URL").unwrap()).await.ok().unwrap();

//   // Set the server_api field of the client_options object to Stable API version 1
//   let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
//   client_options.server_api = Some(server_api);

//   // Get a handle to the cluster
//   let client = Client::with_options(client_options)?;

//   let db: Database = client.database("masq");
//   // Ping the server to see if you can connect to the cluster
//   db
//     .run_command(doc! {"ping": 1}, None)
//     .await?;
//   println!("Pinged your deployment. You successfully connected to MongoDB!");

//   let users: Collection<Document> = db.collection("users");

//   Ok(())
// }

pub async fn getDatabase() -> Database {
    
    dotenv::dotenv().ok();

    let mut client_options = ClientOptions::parse(dotenv::var("DB_URL").unwrap()).await.ok().unwrap();

    // Set the server_api field of the client_options object to Stable API version 1
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    // Get a handle to the cluster
    let client = Client::with_options(client_options).ok().unwrap();

    client.database("masq")
    // Ping the server to see if you can connect to the cluster
    // db
    //   .run_command(doc! {"ping": 1}, None)
    //   .await?;
    // println!("Pinged your deployment. You successfully connected to MongoDB!");

    // let users: Collection<Document> = db.collection("users");

    // Ok(())
}


pub async fn getUserCollection(database: Database) -> Collection<Document> {
    database.collection("users")
}