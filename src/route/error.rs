use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use axum::response::Response;
use serde::Serialize;

use crate::mpd;

#[derive(Debug)]
pub struct Error {
    code: StatusCode,
    message: String,
}

impl Error {
    pub fn new(code: StatusCode, message: String) -> Self {
        Error { code, message }
    }
}

impl From<mpd::Error> for Error {
    fn from(error: mpd::Error) -> Self {
        let (code, message) = match error {
            mpd::Error::Internal(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            },
            mpd::Error::Forbidden(msg) => {
                (StatusCode::FORBIDDEN, msg)
            },
            mpd::Error::NotFound(msg) => {
                (StatusCode::NOT_FOUND, msg)
            },
            mpd::Error::AlreadyExists(msg) => {
                (StatusCode::CONFLICT, msg)
            },
            mpd::Error::Disconnected(msg) | mpd::Error::Unavailable(msg) => {
                (StatusCode::SERVICE_UNAVAILABLE, msg)
            },
        };

        Error::new(code, message)
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (self.code, Json(ErrorResponse { message: self.message })).into_response()
    }
}
