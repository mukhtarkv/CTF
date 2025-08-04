use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use std::{error::Error as StdError, fmt};

/// Application error type
#[derive(Debug)]
pub enum Error {
    RoomNotFound,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::RoomNotFound => write!(f, "room not found"),
        }
    }
}

impl StdError for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::RoomNotFound => {
                let body = Json(json!({
                    "error": "room_not_found",
                    "message": "the requested room does not exist"
                }));
                (StatusCode::NOT_FOUND, body).into_response()
            }
        }
    }
}

/// Convenient result alias for this crate.
pub type Result<T> = std::result::Result<T, Error>;
