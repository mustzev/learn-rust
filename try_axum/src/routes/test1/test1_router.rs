use axum::{routing::get, Router};

pub fn make_test1_router() -> Router {
    Router::new().route("/", get(|| async { "test1" }))
}
