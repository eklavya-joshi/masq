use axum::{response::{IntoResponse, Response}, http::StatusCode};
use serde::Serialize;
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, Serialize, Error)]
pub enum Error {
    // -- Database Error
    #[error("Database error")]
    SqlxError,
}

impl From<sqlx::Error> for Error {
    fn from(_value: sqlx::error::Error) -> Self {
        Error::SqlxError
    }
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		// println!("->> {:<12} - model::Error {self:?}", "INTO_RES");

		// Create a placeholder Axum reponse.
		let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

		// Insert the Error into the reponse.
		response.extensions_mut().insert(self);

		response
	}
}