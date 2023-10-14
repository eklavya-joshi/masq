use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use thiserror::Error;

use crate::database;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, Error)]
pub enum Error {
    // -- Server Error
    #[error("Cannot start server")]
    CannotStartServer,
    #[error("Database error")]
    SqlxError(#[serde_as(as = "DisplayFromStr")] sqlx::error::Error)
}

impl From<database::error::Error> for Error {
    fn from(value: database::error::Error) -> Self {
        match value {
            database::error::Error::SqlxError(x) => Error::SqlxError(x),
        }
    }
}
