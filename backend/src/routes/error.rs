use axum::{response::{IntoResponse, Response}, http::StatusCode};
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use thiserror::Error;

use crate::{api, middleware};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, Error)]
pub enum Error {
    // -- Server Error
    #[error("Database error")]
    SqlxError(#[serde_as(as = "DisplayFromStr")] sqlx::error::Error),
	#[error("UUID error")]
	UuidError(#[serde_as(as = "DisplayFromStr")] uuid::Error),
	// -- Request Error
	#[error("Bad request")]
	BadRequest,
	#[error("Unauthorised")]
	Unauthorised,
	// -- Module Error
	#[error("API error")]
	Api(#[serde_as(as = "DisplayFromStr")] api::Error),
	#[error("Middleware error")]
	Middleware(#[serde_as(as = "DisplayFromStr")] middleware::Error),
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::error::Error) -> Self {
        Error::SqlxError(value)
    }
}

impl From<uuid::Error> for Error {
	fn from(value: uuid::Error) -> Self {
		Error::UuidError(value)
	}
}

impl From<api::error::Error> for Error {
    fn from(value: api::error::Error) -> Self {
		Error::Api(value)
    }
}

impl From<middleware::error::Error> for Error {
    fn from(value: middleware::error::Error) -> Self {
		Error::Middleware(value)
    }
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {

		let mut response = match self {
			Error::Unauthorised => StatusCode::UNAUTHORIZED.into_response(),
			Error::BadRequest => StatusCode::BAD_REQUEST.into_response(),
			Error::Middleware(middleware::Error::Unauthorised) => StatusCode::UNAUTHORIZED.into_response(),
			Error::Middleware(middleware::Error::InvalidToken) => StatusCode::BAD_REQUEST.into_response(),
			_ => StatusCode::INTERNAL_SERVER_ERROR.into_response()
		};

		println!("->> {:<18} - {self:?} | STATUS CODE: {:?}", "ERR_INTO_RES", response.status());

		response.extensions_mut().insert(self);

		response
	}
}

pub fn log<T: core::fmt::Debug>(res: T, route: &str) -> Result<T> {
	println!("->> {:<18} - {route}", "SUCCESS");

    Ok(res)
}

pub fn log_with_res<T: core::fmt::Debug>(res: T, route: &str) -> Result<T> {
	println!("->> {:<18} - {route}", "SUCCESS");

    Ok(res)
}