use serde::Serialize;
use thiserror::Error;

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
    // -- User Input Error
    #[error("Username not available")]
    UsernameNotAvailable
}

impl From<sqlx::Error> for Error {
    fn from(_value: sqlx::error::Error) -> Self {
        Error::SqlxError
    }
}