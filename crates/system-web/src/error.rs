use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

/// Unified error type for API handlers, replacing `(StatusCode, String)` tuples.
pub enum AppError {
    Internal(anyhow::Error),
    NotFound(String),
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    PaymentRequired(String),
    Conflict(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::Internal(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            Self::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            Self::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
            Self::PaymentRequired(msg) => (StatusCode::PAYMENT_REQUIRED, msg),
            Self::Conflict(msg) => (StatusCode::CONFLICT, msg),
        };
        (status, message).into_response()
    }
}

impl From<anyhow::Error> for AppError {
    fn from(e: anyhow::Error) -> Self {
        Self::Internal(e)
    }
}

/// Allow converting legacy `(StatusCode, String)` tuples into AppError.
impl From<(StatusCode, String)> for AppError {
    fn from((status, msg): (StatusCode, String)) -> Self {
        match status {
            StatusCode::NOT_FOUND => Self::NotFound(msg),
            StatusCode::BAD_REQUEST => Self::BadRequest(msg),
            StatusCode::UNAUTHORIZED => Self::Unauthorized(msg),
            StatusCode::FORBIDDEN => Self::Forbidden(msg),
            StatusCode::PAYMENT_REQUIRED => Self::PaymentRequired(msg),
            StatusCode::CONFLICT => Self::Conflict(msg),
            _ => Self::Internal(anyhow::anyhow!("{msg}")),
        }
    }
}
