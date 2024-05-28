use crate::routes::test::calculate::calculate;
use crate::routes::test::hello_world::hello_world;
use axum::{
    routing::{get, post},
    Router,
};

pub fn make_test_router() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/calculate/:operation", post(calculate))
}
