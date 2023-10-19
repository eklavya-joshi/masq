use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use thiserror::Error;

use crate::{database, middleware};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, Error)]
pub enum Error {
    // -- Database Error
    #[error("User not found")]
    UserNotFound(String),
    #[error("Messsage not found")]
    MessageNotFound(String),
    #[error("Inbox not found")]
    InboxNotFound(String),
    #[error("Database error")]
    SqlxError(#[serde_as(as = "DisplayFromStr")] sqlx::error::Error),
    // -- User Input Error
    #[error("Username not available")]
    UsernameNotAvailable(String),
    #[error("Invalid password")]
    InvalidPassword,
    #[error("No Self DM")]
    NoSelfDm,
    #[error("DM already exists")]
    DMAlreadyExists(String),
    // -- Module Error
    #[error("Middleware error")]
    Middleware(middleware::Error),
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::error::Error) -> Self {
        Error::SqlxError(value)
    }
}

impl From<middleware::error::Error> for Error {
    fn from(value: middleware::error::Error) -> Self {
        Error::Middleware(value)
    }
}

impl From<database::error::Error> for Error {
    fn from(value: database::error::Error) -> Self {
        match value {
            database::error::Error::SqlxError(x) => Error::SqlxError(x),
        }
    }
}
