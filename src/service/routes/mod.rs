use actix_web::{http::StatusCode, post, web, HttpResponse, Responder, ResponseError, Scope};
use serde::{Deserialize, Serialize};

pub mod v1;

// Essential enum that can tell actix what to generate for a standard error_response.
// More complicated errors on a route may need to define their own error that implemets ResponseError.
#[derive(derive_more::Display, Debug)]
enum ErrorCode {
    Unauthorised,
    BadRequest,
}

impl ResponseError for ErrorCode {
    fn error_response(&self) -> HttpResponse {
        match &self {
            ErrorCode::Unauthorised => HttpResponse::Unauthorized().into(),
            ErrorCode::BadRequest => HttpResponse::BadRequest().into(),
        }
    }

    fn status_code(&self) -> StatusCode {
        match &self {
            ErrorCode::Unauthorised => StatusCode::UNAUTHORIZED,
            ErrorCode::BadRequest => StatusCode::BAD_REQUEST,
        }
    }
}
