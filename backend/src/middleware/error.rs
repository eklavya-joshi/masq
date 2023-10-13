use axum::response::{Response, IntoResponse};
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use thiserror::Error;

use crate::routes;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, Error)]
pub enum Error {
    // -- Server side
    #[error("Invalid token")]
    InvalidToken,
    #[error("Database error")]
    SqlxError(#[serde_as(as = "DisplayFromStr")] sqlx::error::Error),
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
    fn from(value: sqlx::error::Error) -> Self {
        Error::SqlxError(value)
    }
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		let e: routes::error::Error = self.into();
        e.into_response()
	}
}