use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, Error)]
pub enum Error {
    // -- Server Error
    #[error("Database error")]
    SqlxError(#[serde_as(as = "DisplayFromStr")] sqlx::error::Error)
}

impl From<sqlx::error::Error> for Error {
    fn from(value: sqlx::error::Error) -> Self {
        Error::SqlxError(value)
    }
}