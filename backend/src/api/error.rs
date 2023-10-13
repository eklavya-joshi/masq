use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use thiserror::Error;

use crate::middleware;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, Error)]
pub enum Error {
    // -- Database Error
    #[error("User not found")]
    UserNotFound,
    #[error("Messsage not found")]
    MessageNotFound,
    #[error("Database error")]
    SqlxError(#[serde_as(as = "DisplayFromStr")] sqlx::error::Error),
    // -- JWT Error
    #[error("JWT error")]
    JWTError,
    // -- User Input Error
    #[error("Username not available")]
    UsernameNotAvailable,
    #[error("Invalid password")]
    InvalidPassword
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::error::Error) -> Self {
        Error::SqlxError(value)
    }
}

impl From<middleware::error::Error> for Error {
    fn from(value: middleware::error::Error) -> Self {
        match value {
            middleware::error::Error::InvalidToken => Error::JWTError,
            middleware::error::Error::SqlxError(value) => Error::SqlxError(value),
            middleware::error::Error::Unauthorised => Error::JWTError,
        }
    }
}