use serde::Serialize;
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, Error, Serialize)]
pub enum Error {
    // -- Server side
    #[error("Invalid Token")]
    InvalidToken,
    // -- Client side
    #[error("Unauthorised")]
    Unauthorised
}

// impl From<sqlx::Error> for Error {
//     fn from(_value: sqlx::error::Error) -> Self {
//         Error::Default
//     }
// }

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        match value.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => Error::Unauthorised,
            _ => Error::InvalidToken,
        }
    }
}