use axum::{http::StatusCode, BoxError};
use tower::timeout;

pub async fn handle_timeout_error(err: BoxError) -> (StatusCode, String) {
    if err.is::<timeout::error::Elapsed>() {
        (
            StatusCode::REQUEST_TIMEOUT,
            "Request took too long".to_string(),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", err),
        )
    }
}
