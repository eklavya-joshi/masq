use dotenvy::dotenv;
use sqlx::{PgConnection, Connection, PgPool};
use std::env;

pub mod schema;

pub async fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::connect(&database_url).await.unwrap()
}

pub async fn get_connection_pool() -> PgPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&database_url).await.unwrap()
}