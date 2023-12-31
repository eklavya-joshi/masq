use dotenvy::dotenv;
use sqlx::{Connection, PgConnection, PgPool};
use std::env;

pub mod error;
pub mod schema;

use self::error::{Error, Result};

pub async fn establish_connection() -> Result<PgConnection> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::connect(&database_url)
        .await
        .map_err(|e| Error::SqlxError(e))
}

pub async fn get_connection_pool() -> Result<PgPool> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&database_url)
        .await
        .map_err(|e| Error::SqlxError(e))
}
