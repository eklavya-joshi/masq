use bson::Document;
use mongodb::{Collection, Database};

use crate::database::{getUserCollection};

pub async fn getUserInfo(database: Database, user: String) -> String {
    let users: Collection<Document> = getUserCollection(database).await;

    let user: Document = users.find_one(doc! {"firstName": user}, None).await.ok().unwrap().unwrap();

    // println!("{:?}", user);

    return user.get("firstName").unwrap().to_string() + " " + &user.get("lastName").unwrap().to_string();
}