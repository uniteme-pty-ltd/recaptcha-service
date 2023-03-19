use actix_web::{
    delete, dev::ServiceFactory, get, http::StatusCode, post, put, web, HttpResponse, Responder,
    ResponseError, Scope,
};
use serde::{Deserialize, Serialize};

pub mod v1;

// Essential enum that can tell actix what to generate for a standard error_response.
// More complicated errors on a route may need to define their own error that implemets ResponseError.
#[derive(derive_more::Display, Debug)]
enum ErrorCode {
    NotFound,
    Conflict,
    Unauthorised,
}

impl ResponseError for ErrorCode {
    fn error_response(&self) -> HttpResponse {
        match &self {
            ErrorCode::NotFound => HttpResponse::NotFound().into(),
            ErrorCode::Conflict => HttpResponse::Conflict().into(),
            ErrorCode::Unauthorised => HttpResponse::Unauthorized().into(),
        }
    }

    fn status_code(&self) -> StatusCode {
        match &self {
            ErrorCode::NotFound => StatusCode::NOT_FOUND,
            ErrorCode::Conflict => StatusCode::CONFLICT,
            ErrorCode::Unauthorised => StatusCode::UNAUTHORIZED,
        }
    }
}
