use axum::{response::{IntoResponse, Response}, http::StatusCode};
use serde::Serialize;
use thiserror::Error;

use crate::{api, middleware};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, Serialize, Error)]
pub enum Error {
    // -- Database Error
    #[error("Database error")]
    SqlxError,
	// -- Request Error
	#[error("Bad request")]
	BadRequest,
	#[error("Unauthorised")]
	Unauthorised
}

impl From<sqlx::Error> for Error {
    fn from(_value: sqlx::error::Error) -> Self {
        Error::SqlxError
    }
}

impl From<api::error::Error> for Error {
    fn from(value: api::error::Error) -> Self {
        match value {
            api::error::Error::SqlxError => Error::SqlxError,
			_ => Error::BadRequest
        }
    }
}

impl From<middleware::error::Error> for Error {
    fn from(value: middleware::error::Error) -> Self {
        match value {
			middleware::error::Error::Unauthorised => Error::Unauthorised,
            middleware::error::Error::InvalidToken => Error::BadRequest,
			_ => Error::BadRequest,
        }
    }
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		// println!("->> {:<12} - model::Error {self:?}", "INTO_RES");

		// Create a placeholder Axum reponse.
		let mut response = match self {
			Error::Unauthorised => StatusCode::UNAUTHORIZED.into_response(),
			Error::BadRequest => StatusCode::BAD_REQUEST.into_response(),
			_ => StatusCode::INTERNAL_SERVER_ERROR.into_response()
		};

		// Insert the Error into the reponse.
		response.extensions_mut().insert(self);

		response
	}
}
