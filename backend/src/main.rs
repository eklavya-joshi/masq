// #[macro_use(bson, doc)]
// extern crate bson;
// extern crate mongodb;
// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(hello)
//             .service(echo)
//             .route("/hey", web::get().to(manual_hello))
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }


use bson::Document;
use mongodb::{bson::doc, options::{ClientOptions, ServerApi, ServerApiVersion}, Client, Database, Collection};
use dotenv;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {

  dotenv::dotenv().ok();

  let mut client_options =
    ClientOptions::parse(dotenv::var("DB_URL").unwrap()).await.ok().unwrap();

  // Set the server_api field of the client_options object to Stable API version 1
  let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
  client_options.server_api = Some(server_api);

  // Get a handle to the cluster
  let client = Client::with_options(client_options)?;

  let db: Database = client.database("masq");
  // Ping the server to see if you can connect to the cluster
  db
    .run_command(doc! {"ping": 1}, None)
    .await?;
  println!("Pinged your deployment. You successfully connected to MongoDB!");

  let users: Collection<Document> = db.collection("users");
  
  let mut userCursor = users.find(None, None).await.ok().unwrap();

  while userCursor.advance().await? {
    println!("{:?}, {:?}", userCursor.current().get("firstName"), userCursor.current().get("lastName"));
  }

  Ok(())
}