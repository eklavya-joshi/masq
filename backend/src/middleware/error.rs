use axum::response::{Response, IntoResponse};
use serde::Serialize;
use thiserror::Error;

use crate::routes;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, Error, Serialize)]
pub enum Error {
    // -- Server side
    #[error("Invalid token")]
    InvalidToken,
    #[error("Database error")]
    SqlxError,
    // -- Client side
    #[error("Unauthorised")]
    Unauthorised
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        match value.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => Error::Unauthorised,
            _ => Error::InvalidToken,
        }
    }
}

impl From<sqlx::error::Error> for Error {
    fn from(_value: sqlx::error::Error) -> Self {
        Error::SqlxError
    }
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		let e: routes::error::Error = self.into();
        e.into_response()
	}
}