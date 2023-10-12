
use dotenvy::dotenv;
use sqlx::{Postgres, PgConnection, Connection, Pool};
use std::env;

pub async fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::connect(&database_url).await.unwrap()
}

pub async fn get_connection_pool() -> Pool<Postgres> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Pool::<Postgres>::connect(&database_url).await.unwrap()
}