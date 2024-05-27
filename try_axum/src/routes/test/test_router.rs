use crate::routes::test::hello_world::hello_world;
use axum::{routing::get, Router};

pub fn make_test_router() -> Router {
    return Router::new().route("/", get(hello_world));
}
