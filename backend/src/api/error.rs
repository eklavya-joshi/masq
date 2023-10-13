use serde::Serialize;
use thiserror::Error;

use crate::middleware;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, Error, Serialize)]
pub enum Error {
    // -- Database Error
    #[error("User not found")]
    UserNotFound,
    #[error("Messsage not found")]
    MessageNotFound,
    #[error("Database error")]
    SqlxError,
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
    fn from(_value: sqlx::error::Error) -> Self {
        Error::SqlxError
    }
}

impl From<middleware::error::Error> for Error {
    fn from(value: middleware::error::Error) -> Self {
        match value {
            middleware::error::Error::InvalidToken => Error::JWTError,
            middleware::error::Error::SqlxError => Error::SqlxError,
            middleware::error::Error::Unauthorised => Error::JWTError,
        }
    }
}