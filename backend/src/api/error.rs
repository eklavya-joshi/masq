use thiserror::Error;

use crate::{api};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, Error)]
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
    fn from(value: sqlx::error::Error) -> Self {
        Error::SqlxError
    }
}